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

//! 企业微信实现

pub mod api;
pub mod builder;
pub mod config;
pub mod types;

use super::client::WechatApiResponse;
use crate::errors::{LabraError, LabradorResult};
use crate::request::{HttpMethod, Request, RequestBody};
use crate::response::Response;
use crate::wechat::client::{AccessToken, JsSdkConfig};
use crate::wechat::cp::api::{
    WechatCpAgent, WechatCpDepartment, WechatCpExternalContact, WechatCpGroupRobot, WechatCpMedia,
    WechatCpMenu, WechatCpMessage, WechatCpOauth2, WechatCpTag, WechatCpUser,
};
use crate::wechat::cp::config::WechatCpConfig;
use crate::wechat::cp::types::{WechatCpJsCodeSession, WechatCpProviderToken};
use crate::wechat::{constants, WechatErrorCode};
use crate::{ApiClient, ClientBuilder, CryptoUtils};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;
use tokio::sync::RwLock;

/// 企业微信客户端
pub struct WechatCpClient {
    /// HTTP客户端
    http_client: ApiClient,
    /// 配置
    config: WechatCpConfig,
    /// 访问令牌缓存
    access_token: RwLock<Option<AccessToken>>,
}

impl std::fmt::Debug for WechatCpClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WechatCpClient")
            .field("config", &self.config)
            .field("http_client", &self.http_client)
            .finish()
    }
}

impl WechatCpClient {
    /// 创建新的微信小程序客户端
    pub fn new(config: WechatCpConfig) -> LabradorResult<Self> {
        let base_url = constants::CP_API_BASE_URL;
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

    /// 获取微信客户端
    pub fn http_client(&self) -> &ApiClient {
        &self.http_client
    }

    /// 媒体操作接口
    pub fn media(&self) -> WechatCpMedia<'_> {
        WechatCpMedia::new(self)
    }

    /// 自建应用
    pub fn agent(&self) -> WechatCpAgent<'_> {
        WechatCpAgent::new(self)
    }

    /// 部门
    pub fn department(&self) -> WechatCpDepartment<'_> {
        WechatCpDepartment::new(self)
    }

    /// 外部联系人
    pub fn external_contact(&self) -> WechatCpExternalContact<'_> {
        WechatCpExternalContact::new(self)
    }

    /// 群机器人
    pub fn group_robot(&self) -> WechatCpGroupRobot<'_> {
        WechatCpGroupRobot::new(self)
    }

    /// 菜单
    pub fn menu(&self) -> WechatCpMenu<'_> {
        WechatCpMenu::new(self)
    }

    /// 消息
    pub fn message(&self) -> WechatCpMessage<'_> {
        WechatCpMessage::new(self)
    }

    /// 认证
    pub fn oauth2(&self) -> WechatCpOauth2<'_> {
        WechatCpOauth2::new(self)
    }

    /// 标签
    pub fn tag(&self) -> WechatCpTag<'_> {
        WechatCpTag::new(self)
    }

    /// 用户
    pub fn user(&self) -> WechatCpUser<'_> {
        WechatCpUser::new(self)
    }

    /// 登录凭证校验，获取openid和session_key
    /// # code换取session
    /// [文档](https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/user-login/code2Session.html)
    ///
    pub async fn code2session(&self, js_code: &str) -> LabradorResult<WechatCpJsCodeSession> {
        let response: WechatCpJsCodeSession = self
            .get(&format!("/cgi-bin/miniprogram/jscode2session?appid={}&secret={}&js_code={}&grant_type=authorization_code",
                          self.config.corp_id, self.config.corp_secret, js_code))
            .await?;

        Ok(response)
    }

    /// <pre>
    /// 获取服务商凭证
    /// 文档地址：<a href="https://work.weixin.qq.com/api/doc#90001/90143/91200">地址</a>
    /// 请求方式：POST（HTTPS）
    /// 请求地址： <a href="https://qyapi.weixin.qq.com/cgi-bin/service/get_provider_token">地址</a>
    /// </pre>
    #[inline]
    pub async fn get_provider_token(
        &self,
        corp_id: &str,
        provider_secret: &str,
    ) -> LabradorResult<WechatCpProviderToken> {
        let req = json!({
            "corpid": corp_id,
            "provider_secret": provider_secret,
        });
        let res = self
            .post("/cgi-bin/service/get_provider_token", req)
            .await?;
        Ok(res)
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

    /// 获取全局唯一后台接口调用凭据
    pub async fn fetch_access_token(&self) -> LabradorResult<AccessToken> {
        // 调用微信客户端的获取access_token方法
        let request = Request::builder()
            .method(HttpMethod::Get)
            .path(&format!(
                "/cgi-bin/gettoken?grant_type=client_credential&corpid={}&corpsecret={}",
                self.config.corp_id, self.config.corp_secret
            ))
            .build();
        let response = self.http_client.request(request).await?;
        let response: WechatApiResponse<AccessToken> = response.json()?;
        response.into_result()
    }

    ///
    /// <pre>
    /// 获取微信服务器的ip段
    /// [文档](http://qydev.weixin.qq.com/wiki/index.php?title=回调模式#.E8.8E.B7.E5.8F.96.E5.BE.AE.E4.BF.A1.E6.9C.8D.E5.8A.A1.E5.99.A8.E7.9A.84ip.E6.AE.B5)
    /// </pre>
    pub async fn get_callback_ip(&self) -> LabradorResult<Vec<String>> {
        let res: serde_json::Value = self.get("/cgi-bin/getcallbackip").await?;
        let ip_list = res["ip_list"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .map(|v| v.as_str().unwrap_or_default().to_string())
            .collect::<Vec<String>>();
        Ok(ip_list)
    }

    ///
    /// <pre>
    /// 获取微信服务器的接口ip段
    /// [文档](https://developer.work.weixin.qq.com/document/path/92520)
    /// </pre>
    pub async fn get_api_domain_ip(&self) -> LabradorResult<Vec<String>> {
        let res: serde_json::Value = self.get("/cgi-bin/get_api_domain_ip").await?;
        let ip_list = res["ip_list"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .map(|v| v.as_str().unwrap_or_default().to_string())
            .collect::<Vec<String>>();
        Ok(ip_list)
    }

    /// 发送API请求（自动添加AccessToken）
    pub async fn request<T>(
        &self,
        method: HttpMethod,
        path: &str,
        body: Option<RequestBody>,
    ) -> LabradorResult<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let response = self
            .request_internal(method.clone(), path, body.clone())
            .await?;
        let mut result: WechatApiResponse<T> = response.json()?;

        // 检查是否需要刷新访问令牌
        if let WechatErrorCode::InvalidAccessToken | WechatErrorCode::AccessTokenExpired =
            WechatErrorCode::from(result.errcode().unwrap_or(-1))
        {
            // 清除缓存的访问令牌并重试
            self.clear_access_token_cache().await;
            let response = self.request_internal(method, path, body).await?;
            result = response.json()?;
        }
        result.into_result()
    }

    pub async fn request_internal(
        &self,
        method: HttpMethod,
        path: &str,
        body: Option<RequestBody>,
    ) -> LabradorResult<Response> {
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
            _ => {
                return Err(LabraError::Validation(
                    "不支持的HTTP方法或缺少请求体".to_string(),
                ))
            }
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
        self.request(HttpMethod::Post, path, Some(body.into()))
            .await
    }

    /// POST请求获取字节数据
    pub async fn post_bytes<B>(&self, path: &str, body: B) -> LabradorResult<Bytes>
    where
        B: Into<RequestBody>,
    {
        let response = self
            .request_internal(HttpMethod::Post, path, Some(body.into()))
            .await?;
        Ok(response.bytes())
    }

    /// 清理访问令牌缓存
    pub async fn clear_access_token_cache(&self) {
        let mut token_guard = self.access_token.write().await;
        *token_guard = None;
    }

    /// 获取配置
    pub fn config(&self) -> &WechatCpConfig {
        &self.config
    }

    pub fn agent_id(&self) -> Option<i32> {
        self.config.agent_id
    }

    pub fn webhook_url(&self) -> Option<&String> {
        self.config.webhook_url.as_ref()
    }

    pub fn corp_id(&self) -> &str {
        self.config.corp_id.as_str()
    }
}

/// 微信JS-SDK签名器
#[allow(unused)]
pub struct WechatCpJsSdkSigner {
    crop_id: String,
    /// 应用密钥
    crop_secret: String,
}

impl WechatCpJsSdkSigner {
    /// 创建新的JS-SDK签名器
    pub fn new(crop_id: &str, crop_secret: &str) -> Self {
        Self {
            crop_id: crop_id.to_string(),
            crop_secret: crop_secret.to_string(),
        }
    }

    /// 获取JS-SDK票据
    pub async fn get_jsapi_ticket(&self, client: &WechatCpClient) -> LabradorResult<String> {
        let response: WechatApiResponse<JsapiTicketResponse> =
            client.get("/cgi-bin/get_jsapi_ticket").await?;

        let ticket_response = response
            .data()
            .ok_or_else(|| LabraError::Other("No ticket in response".to_string()))?;
        Ok(ticket_response.ticket)
    }

    /// 获取应用 jsapi_ticket
    pub async fn get_agent_jsapi_ticket(&self, client: &WechatCpClient) -> LabradorResult<String> {
        let response: WechatApiResponse<JsapiTicketResponse> =
            client.get("/cgi-bin/ticket/get?type=agent_config").await?;

        let ticket_response = response
            .data()
            .ok_or_else(|| LabraError::Other("No ticket in response".to_string()))?;
        Ok(ticket_response.ticket)
    }

    /// 生成JS-SDK签名
    pub fn generate_signature(
        &self,
        ticket: &str,
        nonce_str: &str,
        timestamp: i64,
        url: &str,
    ) -> LabradorResult<String> {
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
        client: &WechatCpClient,
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
        let mut config = JsSdkConfig::new(&self.crop_id, js_api_list)
            .with_timestamp(timestamp)
            .with_nonce_str(&nonce_str)
            .with_signature(&signature);
        if debug {
            config = config.enable_debug()
        }

        Ok(config)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsapiTicketResponse {
    pub ticket: String,
    pub expires_in: i64,
}
