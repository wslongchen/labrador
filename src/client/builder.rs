/*
 *
 *  *
 *  *      Copyright (c) 2018-2025, WoofCloud All rights reserved.
 *  *
 *  *   Redistribution and use in source and binary forms, with or without
 *  *   modification, are permitted provided that the following conditions are met:
 *  *
 *  *   Redistributions of source code must retain the above copyright notice,
 *  *   this list of conditions and the following disclaimer.
 *  *   Redistributions in binary form must reproduce the above copyright
 *  *   notice, this list of conditions and the following disclaimer in the
 *  *   documentation and/or other materials provided with the distribution.
 *  *   Neither the name of the www.woofcloud.com developer nor the names of its
 *  *   contributors may be used to endorse or promote products derived from
 *  *   this software without specific prior written permission.
 *  *   Author: WoofCloud
 *  *
 *
 */
//! 客户端构建器实现

use super::{AuthHeader, ClientConfig, RetryConfig, TlsConfig};
use crate::client::certificate::Certificate;
use crate::client::identity::Identity;
use crate::errors::LabradorResult;
use reqwest::Proxy;
use std::time::Duration;

/// 客户端构建器
#[derive(Default)]
pub struct ClientBuilder {
    pub(crate) config: ClientConfig,
}

impl ClientBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置应用密钥
    pub fn app_key<A: Into<String>>(mut self, app_key: A) -> Self {
        self.config.app_key = app_key.into();
        self
    }

    /// 设置应用密钥
    pub fn secret<S: Into<String>>(mut self, secret: S) -> Self {
        self.config.secret = secret.into();
        self
    }

    /// 设置API基础URL
    pub fn api_base_url<U: Into<String>>(mut self, api_base_url: U) -> Self {
        self.config.api_base_url = api_base_url.into();
        self
    }

    /// 设置用户代理
    pub fn user_agent<U: Into<String>>(mut self, user_agent: U) -> Self {
        self.config.user_agent = Some(user_agent.into());
        self
    }

    /// 设置请求超时
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = Some(timeout);
        self
    }

    /// 设置连接超时
    pub fn connect_timeout(mut self, connect_timeout: Duration) -> Self {
        self.config.connect_timeout = Some(connect_timeout);
        self
    }

    /// 设置连接池空闲超时
    pub fn pool_idle_timeout(mut self, pool_idle_timeout: Duration) -> Self {
        self.config.pool_idle_timeout = Some(pool_idle_timeout);
        self
    }

    /// 设置每个主机的最大空闲连接数
    pub fn pool_max_idle_per_host(mut self, pool_max_idle_per_host: usize) -> Self {
        self.config.pool_max_idle_per_host = Some(pool_max_idle_per_host);
        self
    }

    /// 设置HTTP代理
    pub fn proxy(mut self, proxy: Proxy) -> Self {
        self.config.proxy = Some(proxy);
        self
    }

    /// 设置HTTPS代理
    pub fn https_proxy<U: Into<String>>(mut self, proxy_url: U) -> LabradorResult<Self> {
        let proxy = Proxy::https(proxy_url.into())
            .map_err(|e| crate::errors::LabraError::Config(e.to_string()))?;
        self.config.proxy = Some(proxy);
        Ok(self)
    }

    /// 设置HTTP代理
    pub fn http_proxy<U: Into<String>>(mut self, proxy_url: U) -> LabradorResult<Self> {
        let proxy = Proxy::http(proxy_url.into())
            .map_err(|e| crate::errors::LabraError::Config(e.to_string()))?;
        self.config.proxy = Some(proxy);
        Ok(self)
    }

    /// 设置SOCKS代理
    pub fn socks_proxy<U: Into<String>>(mut self, proxy_url: U) -> LabradorResult<Self> {
        let proxy = Proxy::all(proxy_url.into())
            .map_err(|e| crate::errors::LabraError::Config(e.to_string()))?;
        self.config.proxy = Some(proxy);
        Ok(self)
    }

    /// 设置TLS配置
    pub fn tls_config(mut self, tls_config: TlsConfig) -> Self {
        self.config.tls_config = Some(tls_config);
        self
    }

    /// 启用原生TLS
    pub fn enable_native_tls(mut self) -> Self {
        self.config
            .tls_config
            .get_or_insert_with(TlsConfig::default);
        self
    }

    /// 启用Rustls
    pub fn enable_rustls(mut self) -> Self {
        self.config
            .tls_config
            .get_or_insert_with(TlsConfig::default);
        self
    }

    /// 设置客户端身份
    pub fn identity(mut self, identity: Identity) -> Self {
        self.config.identity = Some(identity);
        self
    }

    /// 添加根证书
    pub fn add_root_certificate(mut self, cert: Certificate) -> Self {
        self.config.root_certificates.push(cert);
        self
    }

    /// 添加PEM格式的根证书
    pub fn add_pem_certificate(mut self, pem: &[u8]) -> LabradorResult<Self> {
        let cert = Certificate::from_pem(pem)
            .map_err(|e| crate::errors::LabraError::Certificate(e.to_string()))?;
        self.config.root_certificates.push(cert);
        Ok(self)
    }

    /// 添加DER格式的根证书
    pub fn add_der_certificate(mut self, der: &[u8]) -> LabradorResult<Self> {
        let cert = Certificate::from_der(der)
            .map_err(|e| crate::errors::LabraError::Certificate(e.to_string()))?;
        self.config.root_certificates.push(cert);
        Ok(self)
    }

    /// 设置认证头
    pub fn auth_header(mut self, auth_header: AuthHeader) -> Self {
        self.config.auth_header = Some(auth_header);
        self
    }

    /// 设置Bearer Token认证
    pub fn bearer_token<T: Into<String>>(mut self, token: T) -> Self {
        self.config.auth_header = Some(AuthHeader::bearer_token(token));
        self
    }

    /// 设置Basic认证
    pub fn basic_auth<U, P>(mut self, username: U, password: P) -> Self
    where
        U: Into<String>,
        P: Into<String>,
    {
        self.config.auth_header = Some(AuthHeader::basic_auth(username, password));
        self
    }

    /// 设置API密钥认证
    pub fn api_key_auth<U, P>(mut self, header_name: U, api_key: P) -> Self
    where
        U: Into<String>,
        P: Into<String>,
    {
        self.config.auth_header = Some(AuthHeader::new(header_name, api_key));
        self
    }

    /// 设置重试配置
    pub fn retry_config(mut self, retry_config: RetryConfig) -> Self {
        self.config.retry_config = retry_config;
        self
    }

    /// 启用自动重试
    pub fn enable_auto_retry(mut self) -> Self {
        self.config.auto_retry = true;
        self
    }

    /// 禁用自动重试
    pub fn disable_auto_retry(mut self) -> Self {
        self.config.auto_retry = false;
        self
    }

    /// 设置最大重试次数
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.config.retry_config.max_retries = max_retries;
        self
    }

    /// 设置重试间隔
    pub fn retry_delay(mut self, delay: Duration) -> Self {
        self.config.retry_config.initial_delay = delay;
        self
    }

    /// 设置最大重试延迟
    pub fn max_retry_delay(mut self, max_delay: Duration) -> Self {
        self.config.retry_config.max_delay = max_delay;
        self
    }

    /// 设置退避乘数
    pub fn backoff_multiplier(mut self, multiplier: f32) -> Self {
        self.config.retry_config.backoff_multiplier = multiplier;
        self
    }

    /// 添加可重试的状态码
    pub fn add_retryable_status(mut self, status: reqwest::StatusCode) -> Self {
        self.config.retry_config.retryable_statuses.push(status);
        self
    }

    /// 启用所有重试
    pub fn enable_all_retries(mut self) -> Self {
        self.config.auto_retry = true;
        self.config.retry_config.retryable_statuses.extend(vec![
            reqwest::StatusCode::TOO_MANY_REQUESTS,
            reqwest::StatusCode::INTERNAL_SERVER_ERROR,
            reqwest::StatusCode::BAD_GATEWAY,
            reqwest::StatusCode::SERVICE_UNAVAILABLE,
            reqwest::StatusCode::GATEWAY_TIMEOUT,
            reqwest::StatusCode::REQUEST_TIMEOUT,
            reqwest::StatusCode::CONFLICT,
            reqwest::StatusCode::LOCKED,
            reqwest::StatusCode::FAILED_DEPENDENCY,
            reqwest::StatusCode::TOO_EARLY,
            reqwest::StatusCode::INSUFFICIENT_STORAGE,
        ]);
        self
    }

    /// 启用调试模式
    pub fn debug_mode(mut self) -> Self {
        self.config.user_agent = Some(format!("Labrador-Debug/{}", "1.0"));
        self
    }

    /// 设置沙箱模式
    pub fn sandbox_mode(mut self, sandbox_url: impl Into<String>) -> Self {
        self.config.api_base_url = sandbox_url.into();
        self
    }

    /// 禁用SSL验证（仅用于测试）
    pub fn disable_ssl_verification(mut self) -> LabradorResult<Self> {
        use reqwest::tls::Version;

        let tls_config = TlsConfig {
            min_protocol_version: Some(Version::TLS_1_0),
            max_protocol_version: Some(Version::TLS_1_3),
            ..Default::default()
        };

        self.config.tls_config = Some(tls_config);
        Ok(self)
    }

    /// 构建配置并创建客户端
    pub fn build(self) -> LabradorResult<super::ApiClient> {
        let client = super::ApiClient::new(self.config)?;
        Ok(client)
    }

    /// 仅构建配置
    pub fn build_config(self) -> ClientConfig {
        self.config
    }

    /// 从环境变量构建
    pub fn from_env() -> Self {
        let mut builder = Self::new();

        if let Ok(api_base_url) = std::env::var("LABRADOR_API_BASE_URL") {
            builder = builder.api_base_url(api_base_url);
        }

        if let Ok(app_key) = std::env::var("LABRADOR_APP_KEY") {
            builder = builder.app_key(app_key);
        }

        if let Ok(secret) = std::env::var("LABRADOR_SECRET") {
            builder = builder.secret(secret);
        }

        if let Ok(timeout) = std::env::var("LABRADOR_TIMEOUT") {
            if let Ok(timeout_secs) = timeout.parse::<u64>() {
                builder = builder.timeout(Duration::from_secs(timeout_secs));
            }
        }

        if let Ok(bearer_token) = std::env::var("LABRADOR_BEARER_TOKEN") {
            builder = builder.bearer_token(bearer_token);
        }

        if let Ok(proxy_url) = std::env::var("LABRADOR_PROXY_URL") {
            match builder.https_proxy(proxy_url) {
                Ok(_builder) => {
                    builder = _builder;
                }
                Err(err) => panic!("Failed to set proxy: {}", err),
            }
        }

        builder
    }
}

/// 简化构建器宏
#[macro_export]
macro_rules! client_builder {
    () => {
        $crate::client::ClientBuilder::new()
    };
    ($($field:ident: $value:expr),* $(,)?) => {{
        let mut builder = $crate::client::ClientBuilder::new();
        $(
            builder = builder.$field($value);
        )*
        builder
    }};
}
