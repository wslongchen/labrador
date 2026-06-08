/*
 *
 *  *
 *  *      Copyright (c) 2018-2025, SnackCloud All rights reserved.
 *  *
 *  *   Redistribution and use in source and binary forms, with or without
 *  *   modification, are permitted provided that the following conditions are met:
 *  *
 *  *   Redistributions of source code must retain the above copyright notice,
 *  *   this list of conditions and the following disclaimer.
 *  *   Redistributions in binary form must reproduce the above copyright
 *  *   notice, this list of conditions and the following disclaimer in the
 *  *   documentation and/or other materials provided with the distribution.
 *  *   Neither the name of the www.snackcloud.cn developer nor the names of its
 *  *   contributors may be used to endorse or promote products derived from
 *  *   this software without specific prior written permission.
 *  *   Author: SnackCloud
 *  *
 *
 */
//! HTTP客户端实现
//!
//! 提供核心的HTTP客户端功能，包括：
//! - 客户端配置和构建
//! - 请求拦截器
//! - 请求签名器
//! - 智能重试机制
//! - 会话管理

use crate::request::{HttpMethod, Request, RequestBody};
use crate::response::{Response, StreamResponse};
use http::header;
use moka::future::Cache;
use reqwest::Client as ReqwestClient;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{debug, error, info, warn};

pub mod builder;
pub mod certificate;
/// 客户端配置
pub mod config;
pub mod identity;
/// 请求拦截器
#[allow(unused)]
pub mod interceptors;
/// 重试机制
#[allow(unused)]
pub mod retry;

use crate::errors::{LabraError, LabradorResult};
pub use crate::platforms::signer::RequestSigner;
pub use config::{AuthHeader, ClientConfig, RetryConfig, TlsConfig};
pub use interceptors::RequestInterceptor;

pub(crate) const DEFAULT_USER_AGENT: &str = concat!(
    "Labrador/",
    env!("CARGO_PKG_VERSION"),
    " (+https://github.com/wslongchen/labrador)"
);

/// API客户端
pub struct ApiClient {
    config: ClientConfig,
    session: Cache<String, String>,
    http_client: ReqwestClient,
    interceptors: Vec<Arc<dyn RequestInterceptor>>,
    signer: Option<Arc<dyn RequestSigner>>,
}

impl std::fmt::Debug for ApiClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApiClient")
            .field("config", &self.config)
            .field("has_interceptors", &self.interceptors.len())
            .field("has_signer", &self.signer.is_some())
            .finish()
    }
}

impl ApiClient {
    /// 创建新的API客户端
    pub fn new(config: ClientConfig) -> LabradorResult<Self> {
        let mut client_builder = reqwest::Client::builder()
            .user_agent(config.user_agent.as_deref().unwrap_or(DEFAULT_USER_AGENT));

        // 配置超时
        if let Some(timeout) = config.timeout {
            client_builder = client_builder.timeout(timeout);
        }

        if let Some(connect_timeout) = config.connect_timeout {
            client_builder = client_builder.connect_timeout(connect_timeout);
        }

        if let Some(pool_idle_timeout) = config.pool_idle_timeout {
            client_builder = client_builder.pool_idle_timeout(pool_idle_timeout);
        }

        if let Some(pool_max_idle_per_host) = config.pool_max_idle_per_host {
            client_builder = client_builder.pool_max_idle_per_host(pool_max_idle_per_host);
        }

        // 配置代理
        if let Some(proxy) = &config.proxy {
            client_builder = client_builder.proxy(proxy.clone());
        }

        // 配置TLS
        if let Some(tls_config) = &config.tls_config {
            if let Some(min_protocol_version) = tls_config.min_protocol_version.as_ref() {
                client_builder = client_builder.min_tls_version(min_protocol_version.clone());
            }
            if let Some(max_protocol_version) = tls_config.max_protocol_version.as_ref() {
                client_builder = client_builder.max_tls_version(max_protocol_version.clone());
            }
            for cert in &tls_config.root_certificates {
                client_builder = client_builder.add_root_certificate(cert.clone());
            }
        }

        // 配置身份认证
        if let Some(identity) = &config.identity {
            client_builder = client_builder.identity(identity.to_reqwest_identity()?);
        }

        // 配置根证书
        for cert in &config.root_certificates {
            client_builder = client_builder.add_root_certificate(cert.to_reqwest_certificate()?);
        }

        let http_client = client_builder.build().map_err(LabraError::Network)?;

        // 创建会话缓存，5分钟过期，最大1000个条目
        let session = Cache::builder()
            .time_to_live(Duration::from_secs(300))
            .max_capacity(1000)
            .build();

        Ok(Self {
            config,
            session,
            http_client,
            interceptors: Vec::new(),
            signer: None,
        })
    }

    /// 获取会话缓存
    pub fn session(&self) -> &Cache<String, String> {
        &self.session
    }

    /// 获取配置
    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    /// 添加请求拦截器
    pub fn add_interceptor<I: RequestInterceptor + 'static>(&mut self, interceptor: I) {
        self.interceptors.push(Arc::new(interceptor));
    }

    /// 设置请求签名器
    pub fn set_signer<S: RequestSigner + 'static>(&mut self, signer: S) {
        self.signer = Some(Arc::new(signer));
    }

    pub fn signer(&self) -> Option<&Arc<dyn RequestSigner>> {
        self.signer.as_ref()
    }

    /// 将签名器向下转型为具体类型（用于平台特定操作如验签）
    pub fn downcast_signer<T: 'static>(&self) -> Option<&T> {
        self.signer.as_ref().and_then(|arc| {
            let trait_obj: &dyn RequestSigner = arc.as_ref();
            trait_obj.as_any().downcast_ref::<T>()
        })
    }

    /// 发送HTTP请求
    pub async fn request<R: Into<Request>>(&self, request: R) -> LabradorResult<Response> {
        let mut request = request.into();
        if self.config.auto_retry {
            self.execute_request_with_retry(&mut request).await
        } else {
            self.execute_request_once(&mut request).await
        }
    }

    /// 发送GET请求
    pub async fn get<I, P>(&self, path: I, params: P) -> LabradorResult<Response>
    where
        I: AsRef<str>,
        P: IntoIterator<Item = (String, String)>,
    {
        let request = Request::builder()
            .method(HttpMethod::Get)
            .path(path)
            .query_params(params)
            .build();

        self.request(request).await
    }

    /// 发送POST请求
    pub async fn post<I, P, B>(&self, path: I, params: P, body: B) -> LabradorResult<Response>
    where
        I: AsRef<str>,
        P: IntoIterator<Item = (String, String)>,
        B: Into<RequestBody>,
    {
        let request = Request::builder()
            .method(HttpMethod::Post)
            .path(path)
            .query_params(params)
            .body(body)
            .build();

        self.request(request).await
    }

    /// 发送PUT请求
    pub async fn put<I, P, B>(&self, path: I, params: P, body: B) -> LabradorResult<Response>
    where
        I: AsRef<str>,
        P: IntoIterator<Item = (String, String)>,
        B: Into<RequestBody>,
    {
        let request = Request::builder()
            .method(HttpMethod::Put)
            .path(path)
            .query_params(params)
            .body(body)
            .build();

        self.request(request).await
    }

    /// 发送DELETE请求
    pub async fn delete<I, P>(&self, path: I, params: P) -> LabradorResult<Response>
    where
        I: AsRef<str>,
        P: IntoIterator<Item = (String, String)>,
    {
        let request = Request::builder()
            .method(HttpMethod::Delete)
            .path(path)
            .query_params(params)
            .build();

        self.request(request).await
    }

    /// 发送流式请求
    pub async fn stream_request<R: Into<Request>>(
        &self,
        request: R,
    ) -> LabradorResult<StreamResponse> {
        let mut request = request.into();

        // 执行拦截器
        for interceptor in &self.interceptors {
            interceptor.before_request(&mut request).await?;
        }

        // 构建URL
        let url = self.build_url(&request.path)?;

        // 创建请求构建器
        let mut request_builder = self.http_client.request(request.method.into(), url.clone());

        // 添加查询参数
        if !request.query_params.is_empty() {
            request_builder = request_builder.query(&request.query_params);
        }

        // 添加请求头
        if !request.headers.is_empty() {
            request_builder = request_builder.headers(request.headers.clone());
        }

        // 添加认证头
        if let Some(auth_header) = &self.config.auth_header {
            request_builder = request_builder.header(&auth_header.name, &auth_header.value);
        }

        // 处理请求体
        request_builder = self.apply_body_to_builder(request_builder, request.body.clone())?;

        // 执行签名
        if let Some(signer) = &self.signer {
            signer.sign_request(&mut request)?;
            // 重新应用签名后的请求头
            for (key, value) in request.headers.iter() {
                request_builder = request_builder.header(key, value);
            }
        }

        // 记录请求日志
        debug!("Sending request: {} {}", request.method, request.path);

        // 发送请求并获取流式响应
        let response = request_builder.send().await.map_err(LabraError::Network)?;

        let status = response.status();
        let headers = response.headers().clone();
        let url = response.url().clone();

        // 创建流式响应
        let stream_response = StreamResponse::new(url, status, headers, response);

        // 执行后置拦截器
        for interceptor in &self.interceptors {
            interceptor.after_stream_response(&stream_response).await?;
        }

        Ok(stream_response)
    }

    /// 执行请求（带重试机制）
    #[allow(unused_assignments)]
    async fn execute_request_with_retry(&self, request: &mut Request) -> LabradorResult<Response> {
        let retry_config = &self.config.retry_config;
        let mut attempts = 0;
        let mut last_error = None;

        loop {
            attempts += 1;

            match self.execute_request_once(request).await {
                Ok(response) => {
                    // 检查是否需要重试
                    if retry_config.should_retry(&response) {
                        if attempts < retry_config.max_retries {
                            let delay = retry_config.delay_for_attempt(attempts);
                            warn!(
                                "Request failed with status {}, retrying in {:?} (attempt {}/{})",
                                response.status(),
                                delay,
                                attempts,
                                retry_config.max_retries
                            );

                            tokio::time::sleep(delay).await;
                            continue;
                        } else {
                            return Err(LabraError::RetryExhausted {
                                attempts,
                                last_error: Box::new(LabraError::request_failed(
                                    request.method.to_string(),
                                    request.path.clone(),
                                    response.status().as_u16(),
                                    response.text().unwrap_or_default(),
                                )),
                            });
                        }
                    }

                    return Ok(response);
                }
                Err(err) => {
                    last_error = Some(err);

                    // 检查错误是否应该重试
                    if let Some(err) = last_error.as_ref() {
                        if err.should_retry() && attempts < retry_config.max_retries {
                            let delay = retry_config.delay_for_attempt(attempts);
                            warn!(
                                "Request failed with error: {}, retrying in {:?} (attempt {}/{})",
                                err, delay, attempts, retry_config.max_retries
                            );

                            tokio::time::sleep(delay).await;
                            continue;
                        }
                    }

                    break;
                }
            }
        }

        Err(last_error.unwrap_or(LabraError::Unknown))
    }

    /// 执行单个请求
    async fn execute_request_once(&self, request: &mut Request) -> LabradorResult<Response> {
        let start_time = Instant::now();

        // 执行前置拦截器
        for interceptor in &self.interceptors {
            interceptor.before_request(request).await?;
        }

        // 构建URL
        let url = self.build_url(&request.path)?;

        // 创建请求构建器
        let mut request_builder = self.http_client.request(request.method.into(), url.clone());

        // 添加查询参数
        if !request.query_params.is_empty() {
            request_builder = request_builder.query(&request.query_params);
        }

        // 添加请求头
        if !request.headers.is_empty() {
            request_builder = request_builder.headers(request.headers.clone());
        }

        // 添加认证头
        if let Some(auth_header) = &self.config.auth_header {
            request_builder = request_builder.header(&auth_header.name, &auth_header.value);
        }

        // 处理请求体
        request_builder = self.apply_body_to_builder(request_builder, request.body.clone())?;

        // 执行签名
        if let Some(signer) = &self.signer {
            signer.sign_request(request)?;
            // 重新应用签名后的请求头
            for (key, value) in request.headers.iter() {
                request_builder = request_builder.header(key, value);
            }
        }

        // 记录请求日志
        debug!(
            "Sending request: {} {:?} {} {:?}",
            request.method, request.headers, request.path, request.body
        );

        // 发送请求
        let response = request_builder.send().await.map_err(LabraError::Network)?;

        let status = response.status();
        let remote_addr = response.remote_addr();
        let headers = response.headers().clone();
        let url = response.url().clone();
        let body = response.bytes().await.map_err(LabraError::Network)?;

        let http_response = Response::new(url, status, headers, remote_addr, body);
        // 记录响应日志
        let duration = start_time.elapsed();
        if http_response.is_success() {
            info!(
                "Request succeeded in {:?} with status: {}",
                duration, status
            );
        } else {
            error!("Request failed in {:?} with status: {}", duration, status);
        }

        // 执行后置拦截器
        for interceptor in &self.interceptors {
            interceptor.after_response(&http_response).await?;
        }

        if http_response.is_success() {
            Ok(http_response)
        } else {
            Err(LabraError::request_failed(
                request.method.to_string(),
                request.path.clone(),
                status.as_u16(),
                http_response.text().unwrap_or_default(),
            ))
        }
    }

    /// 构建完整URL
    fn build_url(&self, path: &str) -> LabradorResult<reqwest::Url> {
        if path.starts_with("http://") || path.starts_with("https://") {
            reqwest::Url::parse(path).map_err(LabraError::Url)
        } else {
            let base_url = self.config.api_base_url.trim_end_matches('/');
            let path = path.trim_start_matches('/');
            let full_url = format!("{}/{}", base_url, path);
            reqwest::Url::parse(&full_url).map_err(LabraError::Url)
        }
    }

    /// 应用请求体到请求构建器
    fn apply_body_to_builder(
        &self,
        mut builder: reqwest::RequestBuilder,
        body: RequestBody,
    ) -> LabradorResult<reqwest::RequestBuilder> {
        match body {
            RequestBody::Json(json) => {
                builder = builder.header(header::CONTENT_TYPE, "application/json; charset=UTF-8");
                builder = builder.body(serde_json::to_string(&json)?);
            }
            RequestBody::Form(form_data) => {
                builder = builder.header(
                    header::CONTENT_TYPE,
                    "application/x-www-form-urlencoded; charset=UTF-8",
                );
                builder = builder.form(&form_data);
            }
            RequestBody::Multipart(form) => {
                builder = builder.multipart(form);
            }
            RequestBody::Xml(xml) => {
                builder = builder.header(header::CONTENT_TYPE, "application/xml; charset=UTF-8");
                builder = builder.body(xml);
            }
            RequestBody::Text(text) => {
                builder = builder.header(header::CONTENT_TYPE, "text/plain; charset=UTF-8");
                builder = builder.body(text);
            }
            RequestBody::Binary(bytes) => {
                builder = builder.header(header::CONTENT_TYPE, "application/octet-stream");
                builder = builder.body(bytes);
            }
            RequestBody::Empty => {}
        }

        Ok(builder)
    }
}
