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

//! 微信客户端实现

use super::{constants, WechatErrorCode};
use crate::client::builder::ClientBuilder;
use crate::client::ApiClient;
use crate::request::RequestBody;
use crate::{errors::{LabraError, LabradorResult}, request::{HttpMethod, Request}, response::Response, CryptoUtils};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;

/// 微信客户端配置
#[derive(Debug, Clone)]
pub struct WechatClientConfig {
    /// 应用ID
    pub app_id: String,
    /// 应用密钥
    pub app_secret: String,
    /// 是否启用沙箱模式
    pub sandbox: bool,
    /// 访问令牌缓存时间（秒）
    pub access_token_ttl: u64,
    /// 请求超时时间
    pub timeout: Option<Duration>,
}

impl Default for WechatClientConfig {
    fn default() -> Self {
        Self {
            app_id: String::new(),
            app_secret: String::new(),
            sandbox: false,
            access_token_ttl: 7000, // 微信access_token有效期为7200秒，这里提前200秒刷新
            timeout: Some(Duration::from_secs(30)),
        }
    }
}

/// 微信客户端
pub struct WechatClient {
    /// HTTP客户端
    http_client: ApiClient,
    /// 配置
    config: WechatClientConfig,
    /// 访问令牌缓存
    access_token: RwLock<Option<AccessToken>>,
}

impl std::fmt::Debug for WechatClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WechatClient")
            .field("config", &self.config)
            .field("http_client", &self.http_client)
            .finish()
    }
}

/// 访问令牌
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccessToken {
    /// 访问令牌
    pub access_token: String,
    /// 过期时间（秒）
    pub expires_in: i64,
    /// 获取时间
    #[serde(skip, default = "SystemTime::now")]
    pub fetched_at: SystemTime,
}

impl AccessToken {
    /// 创建新的访问令牌
    pub fn new(access_token: String, expires_in: i64) -> Self {
        Self {
            access_token,
            expires_in,
            fetched_at: SystemTime::now(),
        }
    }

    /// 检查是否过期
    pub fn is_expired(&self) -> bool {
        match self.fetched_at.elapsed() {
            Ok(elapsed) => elapsed.as_secs() >= (self.expires_in as u64).saturating_sub(200), // 提前200秒过期
            Err(_) => true,
        }
    }

    /// 获取剩余有效时间
    pub fn remaining_seconds(&self) -> i64 {
        match self.fetched_at.elapsed() {
            Ok(elapsed) => {
                let elapsed_secs = elapsed.as_secs() as i64;
                (self.expires_in - elapsed_secs).max(0)
            }
            Err(_) => 0,
        }
    }
}
/// 微信API响应基础结构
#[derive(Debug, Clone)]
pub enum WechatApiResponse<T = serde_json::Value> {
    Success(T),
    Error {
        errcode: i32,
        errmsg: String,
    },
}

impl<T> WechatApiResponse<T> {
    pub fn is_success(&self) -> bool {
        match self {
            WechatApiResponse::Success(_) => true,
            WechatApiResponse::Error { .. } => false,
        }
    }

    pub fn errcode(&self) -> Option<i32> {
        match self {
            WechatApiResponse::Success(_) => None,
            WechatApiResponse::Error { errcode, .. } => Some(*errcode),
        }
    }

    pub fn errmsg(&self) -> Option<String> {
        match self {
            WechatApiResponse::Success(_) => None,
            WechatApiResponse::Error { errmsg, .. } => Some(errmsg.clone()),
        }
    }

    pub fn data(self) -> Option<T> {
        match self {
            WechatApiResponse::Success(data) => Some(data),
            WechatApiResponse::Error { .. } => None,
        }
    }

    pub fn into_result(self) -> Result<T, crate::errors::LabraError> {
        match self {
            WechatApiResponse::Success(data) => Ok(data),
            WechatApiResponse::Error { errcode, errmsg } => {
                Err(crate::errors::LabraError::business(
                    errcode.to_string(),
                    errmsg,
                ))
            }
        }
    }
}

impl<'de, T> Deserialize<'de> for WechatApiResponse<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        // 检查是否有 errcode 字段
        if let Some(errcode) = value.get("errcode").and_then(|v| v.as_i64()) {
            let errcode = errcode as i32;

            // 如果有 errmsg 字段
            let errmsg = value.get("errmsg")
                .and_then(|v| v.as_str())
                .map(String::from)
                .unwrap_or_else(|| "Unknown error".to_string());

            // 如果 errcode != 0，认为是错误响应
            if errcode != 0 {
                return Ok(WechatApiResponse::Error { errcode, errmsg });
            }

            // errcode == 0，尝试解析业务数据
            // 注意：这里需要移除 errcode 和 errmsg 字段，避免干扰业务数据解析
            let mut data_value = value;
            if let Some(obj) = data_value.as_object_mut() {
                obj.remove("errcode");
                obj.remove("errmsg");
            }

            match T::deserialize(data_value) {
                Ok(data) => Ok(WechatApiResponse::Success(data)),
                Err(e) => {
                    // 如果解析业务数据失败，但 errcode=0，这种情况应该怎么处理？
                    // 可以选择返回 Success(()) 或者返回错误
                    Err(serde::de::Error::custom(format!(
                        "Failed to parse success data: {}", e
                    )))
                }
            }
        } else {
            // 没有 errcode 字段，直接解析为业务数据
            match T::deserialize(value) {
                Ok(data) => Ok(WechatApiResponse::Success(data)),
                Err(e) => Err(serde::de::Error::custom(format!(
                    "Failed to parse response: {}", e
                ))),
            }
        }
    }
}

/// 微信客户端实现
impl WechatClient {
    /// 创建新的微信客户端
    pub fn new(config: WechatClientConfig) -> LabradorResult<Self> {
        let base_url = if config.sandbox {
            constants::API_SANDBOX_BASE_URL
        } else {
            constants::API_BASE_URL
        };

        let http_client = ClientBuilder::new()
            .api_base_url(base_url)
            .timeout(config.timeout.unwrap_or(Duration::from_secs(30)))
            .connect_timeout(Duration::from_secs(10))
            .enable_auto_retry()
            .max_retries(3)
            .build()?;

        Ok(Self {
            http_client,
            config,
            access_token: RwLock::new(None),
        })
    }

    /// 获取访问令牌（带缓存）
    pub async fn get_access_token(&self) -> LabradorResult<String> {
        {
            let access_token = self.access_token.read().await;
            if let Some(token) = access_token.as_ref() {
                if !token.is_expired() {
                    return Ok(token.access_token.clone());
                }
            }
        }

        let mut stored_access_token = self.access_token.write().await;

        if let Some(token) = stored_access_token.as_ref() {
            if !token.is_expired() {
                return Ok(token.access_token.clone());
            }
        }

        // 获取新的访问令牌
        let token = self.fetch_access_token().await?;
        let access_token = token.access_token.clone();
        *stored_access_token = Some(token);

        Ok(access_token)
    }

    /// 获取新的访问令牌
    async fn fetch_access_token(&self) -> LabradorResult<AccessToken> {
        let url = format!(
            "/cgi-bin/token?grant_type=client_credential&appid={}&secret={}",
            self.config.app_id, self.config.app_secret
        );

        let response = self.http_client
            .get(&url, vec![])
            .await?
            .text()?;
        println!("{}", response);
        let response: WechatApiResponse<AccessToken> = serde_json::from_str(&response)?;
        println!("{:?}", response);
        response.into_result()
    }

    /// 发送API请求（自动添加AccessToken）
    pub async fn request<T>(&self, method: HttpMethod, path: &str, body: Option<RequestBody>) -> LabradorResult<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let response = self.request_internal(method.clone(), path, body.clone()).await?;
        let mut result: WechatApiResponse<T> = response.json()?;

        // 检查是否需要刷新访问令牌
        if let WechatErrorCode::InvalidAccessToken | WechatErrorCode::AccessTokenExpired =
            WechatErrorCode::from(result.errcode().unwrap_or(-1)) {
            // 清除缓存的访问令牌并重试
            self.clear_access_token_cache().await;
            let response = self.request_internal(method, path, body).await?;
            result = response.json()?;
        }
        result.into_result()
    }
    
    pub async fn request_internal(&self, method: HttpMethod, path: &str, body: Option<RequestBody>) -> LabradorResult<Response>
    {
        let access_token = self.get_access_token().await?;
        // 判断 path 是否已经包含查询参数
        let url = if path.contains('?') {
            format!("{}&access_token={}", path, access_token)
        } else {
            format!("{}?access_token={}", path, access_token)
        };

        let request = match (method, body) {
            (HttpMethod::Get, _) => Request::builder()
                .method(HttpMethod::Get)
                .path(&url)
                .build(),
            (HttpMethod::Post, Some(body_data)) => Request::builder()
                .method(HttpMethod::Post)
                .path(&url)
                .body(body_data)
                .build(),
            _ => return Err(LabraError::Validation("不支持的HTTP方法或缺少请求体".to_string())),
        };
        let response = self.http_client.request(request).await?;
        Ok(response)
    }

    /// 发送GET请求
    pub async fn get<T>(&self, path: &str) -> LabradorResult<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        self.request::<T>(HttpMethod::Get, path, None).await
    }

    /// GET请求获取字节数据
    pub async fn get_bytes(&self, path: &str) -> LabradorResult<Bytes> {
        let response = self.request_internal(HttpMethod::Get, path, None).await?;
        Ok(response.bytes())
    }

    /// 发送POST请求
    pub async fn post<T, B>(&self, path: &str, body: B) -> LabradorResult<T>
    where
        T: for<'de> Deserialize<'de>,
        B: Into<RequestBody>,
    {
        self.request(HttpMethod::Post, path, Some(body.into())).await
    }

    /// POST请求获取字节数据
    pub async fn post_bytes<B>(&self, path: &str, body: B) -> LabradorResult<Bytes>
    where
        B: Into<RequestBody>,
    {
        let response = self.request_internal(HttpMethod::Post, path, Some(body.into())).await?;
        Ok(response.bytes())
    }

    /// 获取微信服务器IP地址
    pub async fn get_wechat_ips(&self) -> LabradorResult<Vec<String>> {
        #[derive(Debug, Deserialize)]
        struct IpListResponse {
            ip_list: Vec<String>,
        }

        let response: WechatApiResponse<IpListResponse> = self.get("/cgi-bin/get_api_domain_ip").await?;
        Ok(response.data().ok_or_else(|| LabraError::Other("No data in response".to_string()))?.ip_list)
    }

    /// 清理访问令牌缓存
    pub async fn clear_access_token_cache(&self) {
        let mut token_guard = self.access_token.write().await;
        *token_guard = None;
    }

    /// 获取配置
    pub fn config(&self) -> &WechatClientConfig {
        &self.config
    }

    /// 获取HTTP客户端
    pub fn http_client(&self) -> &ApiClient {
        &self.http_client
    }
}

/// 微信JS-SDK配置
#[derive(Debug, Clone, Serialize)]
pub struct JsSdkConfig {
    /// 是否调试模式
    pub debug: bool,
    /// 应用ID
    pub app_id: String,
    /// 时间戳
    pub timestamp: i64,
    /// 随机字符串
    pub nonce_str: String,
    /// 签名
    pub signature: String,
    /// JS接口列表
    pub js_api_list: Vec<String>,
}

impl JsSdkConfig {
    /// 创建新的JS-SDK配置
    pub fn new(app_id: &str, js_api_list: Vec<String>) -> Self {
        Self {
            debug: false,
            app_id: app_id.to_string(),
            timestamp: 0,
            nonce_str: String::new(),
            signature: String::new(),
            js_api_list,
        }
    }

    /// 启用调试模式
    pub fn enable_debug(mut self) -> Self {
        self.debug = true;
        self
    }

    /// 设置时间戳
    pub fn with_timestamp(mut self, timestamp: i64) -> Self {
        self.timestamp = timestamp;
        self
    }

    /// 设置随机字符串
    pub fn with_nonce_str(mut self, nonce_str: &str) -> Self {
        self.nonce_str = nonce_str.to_string();
        self
    }

    /// 设置签名
    pub fn with_signature(mut self, signature: &str) -> Self {
        self.signature = signature.to_string();
        self
    }
}

/// 微信客户端构建器
pub struct WechatClientBuilder {
    config: WechatClientConfig,
}

impl WechatClientBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            config: WechatClientConfig::default(),
        }
    }

    /// 设置应用ID
    pub fn app_id<S: Into<String>>(mut self, app_id: S) -> Self {
        self.config.app_id = app_id.into();
        self
    }

    /// 设置应用密钥
    pub fn app_secret<S: Into<String>>(mut self, app_secret: S) -> Self {
        self.config.app_secret = app_secret.into();
        self
    }

    /// 启用沙箱模式
    pub fn sandbox(mut self, sandbox: bool) -> Self {
        self.config.sandbox = sandbox;
        self
    }

    /// 设置访问令牌缓存时间
    pub fn access_token_ttl(mut self, ttl_seconds: u64) -> Self {
        self.config.access_token_ttl = ttl_seconds;
        self
    }

    /// 设置请求超时时间
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = Some(timeout);
        self
    }

    /// 构建微信客户端
    pub fn build(self) -> LabradorResult<WechatClient> {
        WechatClient::new(self.config)
    }
}

impl Default for WechatClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 微信JS-SDK签名器
#[allow(unused)]
pub struct JsSdkSigner {
    /// 应用ID
    app_id: String,
    /// 应用密钥
    app_secret: String,
}

impl JsSdkSigner {
    /// 创建新的JS-SDK签名器
    pub fn new(app_id: &str, app_secret: &str) -> Self {
        Self {
            app_id: app_id.to_string(),
            app_secret: app_secret.to_string(),
        }
    }

    /// 获取JS-SDK票据
    pub async fn get_jsapi_ticket(&self, client: &WechatClient) -> LabradorResult<String> {

        let response: WechatApiResponse<JsapiTicketResponse> = client
            .get("/cgi-bin/ticket/getticket?type=jsapi")
            .await?;

        let ticket_response = response.data().ok_or_else(|| LabraError::Other("No ticket in response".to_string()))?;
        Ok(ticket_response.ticket)
    }

    /// 生成JS-SDK签名
    pub fn generate_signature(&self, ticket: &str, nonce_str: &str, timestamp: i64, url: &str) -> LabradorResult<String> {
        let sign_string = format!(
            "jsapi_ticket={}&noncestr={}&timestamp={}&url={}",
            ticket, nonce_str, timestamp, url
        );

        let signature = CryptoUtils::sha1_hex(sign_string.as_bytes());
        Ok(signature)
    }

    /// 生成完整的JS-SDK配置
    pub async fn generate_config(
        &self,
        client: &WechatClient,
        url: &str,
        js_api_list: Vec<String>,
        debug: bool,
    ) -> LabradorResult<JsSdkConfig> {
        use rand::Rng;

        // 获取JSAPI票据
        let ticket = self.get_jsapi_ticket(client).await?;

        // 生成随机字符串和时间戳
        let mut rng = rand::thread_rng();
        let nonce_str: String = (0..16)
            .map(|_| {
                let idx = rng.gen_range(0..crate::utils::string::CHARSET.len());
                crate::utils::string::CHARSET[idx] as char
            })
            .collect();

        let timestamp = chrono::Utc::now().timestamp();

        // 生成签名
        let signature = self.generate_signature(&ticket, &nonce_str, timestamp, url)?;

        // 构建配置
        let mut config = JsSdkConfig::new(&self.app_id, js_api_list)
            .with_timestamp(timestamp)
            .with_nonce_str(&nonce_str)
            .with_signature(&signature);
        if debug {
            config = config.enable_debug()
        }

        Ok(config)
    }
}

#[derive(Debug, Serialize,Deserialize)]
pub struct JsapiTicketResponse {
    pub ticket: String,
    pub expires_in: i64,
}