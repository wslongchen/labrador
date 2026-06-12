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

//! 微信小程序实现

pub mod api;
pub mod builder;
pub mod config;
pub mod types;

use super::client::{WechatApiResponse, WechatClient, WechatClientConfig};
use crate::errors::{LabraError, LabradorResult};
use crate::platforms::wechat::miniapp::config::WechatMiniAppConfig;
use crate::platforms::wechat::miniapp::types::Code2SessionResponse;
use crate::wechat::client::AccessToken;
use crate::wechat::miniapp::api::{
    WechatMxaAuth, WechatMxaCloudBase, WechatMxaExpress, WechatMxaImgOcr,
    WechatMxaImmediateDelivery, WechatMxaLiveBroadcast, WechatMxaMessage, WechatMxaQrcode,
    WechatMxaUser,
};
use crate::wechat::miniapp::types::WechatResetUserSessionKeyResponse;

/// 微信小程序客户端
pub struct WechatMiniAppClient {
    /// 微信客户端
    wechat_client: WechatClient,
    /// 配置
    config: WechatMiniAppConfig,
}

impl std::fmt::Debug for WechatMiniAppClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WechatMiniAppClient")
            .field("config", &self.config)
            .field("wechat_client", &self.wechat_client)
            .finish()
    }
}

impl WechatMiniAppClient {
    /// 创建新的微信小程序客户端
    pub fn new(config: WechatMiniAppConfig) -> LabradorResult<Self> {
        let wechat_client_config = WechatClientConfig {
            app_id: config.app_id.clone(),
            app_secret: config.app_secret.clone(),
            sandbox: config.sandbox,
            ..WechatClientConfig::default()
        };

        let wechat_client = WechatClient::new(wechat_client_config)?;

        Ok(Self {
            wechat_client,
            config,
        })
    }

    /// 获取微信客户端
    pub fn wechat_client(&self) -> &WechatClient {
        &self.wechat_client
    }

    /// 获取订阅消息
    pub fn message(&self) -> WechatMxaMessage<'_> {
        WechatMxaMessage::new(self)
    }

    /// 获取二维码
    pub fn qrcode(&self) -> WechatMxaQrcode<'_> {
        WechatMxaQrcode::new(self)
    }

    /// 获取用户信息
    pub fn user(&self) -> WechatMxaUser<'_> {
        WechatMxaUser::new(self)
    }

    /// 即时配送
    pub fn delivery(&self) -> WechatMxaImmediateDelivery<'_> {
        WechatMxaImmediateDelivery::new(self)
    }

    /// 直播
    pub fn broadcast(&self) -> WechatMxaLiveBroadcast<'_> {
        WechatMxaLiveBroadcast::new(self)
    }

    /// 认证
    pub fn auth(&self) -> WechatMxaAuth<'_> {
        WechatMxaAuth::new(self)
    }

    /// 物流
    pub fn express(&self) -> WechatMxaExpress<'_> {
        WechatMxaExpress::new(self)
    }

    /// 图片识别
    pub fn ocr(&self) -> WechatMxaImgOcr<'_> {
        WechatMxaImgOcr::new(self)
    }

    /// 云开发
    pub fn cloud(&self) -> WechatMxaCloudBase<'_> {
        WechatMxaCloudBase::new(self)
    }

    /// 登录凭证校验，获取openid和session_key
    /// # code换取session
    /// [文档](https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/user-login/code2Session.html)
    ///
    pub async fn code2session(&self, js_code: &str) -> LabradorResult<Code2SessionResponse> {
        let response: Code2SessionResponse = self
            .wechat_client
            .get(&format!(
                "/sns/jscode2session?appid={}&secret={}&js_code={}&grant_type=authorization_code",
                self.config.app_id, self.config.app_secret, js_code
            ))
            .await?;

        if response.errcode != 0 {
            return Err(LabraError::business(
                response.errcode.to_string(),
                response.errmsg.clone(),
            ));
        }

        Ok(response)
    }

    /// 检测session_key
    /// 校验服务器所保存的登录态 session_key 是否有效。为了保持 session_key 私密性，接口不明文传输 session_key，而是通过校验登录态签名完成。
    ///
    /// session_key 具有唯一性，在使用小程序时，同一用户在同一时刻仅有一个有效的 session_key。 通过 code2Session 接口获得的用户 session_key 拥有一定的时效性。 除了过期失效外，触发获取临时登录凭证 code 的操作（小程序登录 和 数据预拉取）也可能会生成新的登录态 session_key，从而使旧的 session_key 被顶替而失效。
    ///
    /// 为了处理以上失效情况，可以通过本接口校验用户 session_key 的有效性
    pub async fn check_session_key(
        &self,
        openid: &str,
        signature: &str,
        sig_method: &str,
    ) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .wechat_client
            .get(&format!(
                "/wxa/checksession?openid={}&signature={}&sig_method={}",
                openid, signature, sig_method
            ))
            .await?;
        Ok(response)
    }

    /// # 重置登录态
    /// 重置指定的登录态 session_key。为了保持 session_key 私密性，接口不明文传入 session_key，而是通过校验登录态签名完成。
    pub async fn reset_user_session_key(
        &self,
        openid: &str,
        signature: &str,
        sig_method: &str,
    ) -> LabradorResult<WechatResetUserSessionKeyResponse> {
        let response: WechatApiResponse<WechatResetUserSessionKeyResponse> = self
            .wechat_client
            .get(&format!(
                "/wxa/resetsession?openid={}&signature={}&sig_method={}",
                openid, signature, sig_method
            ))
            .await?;
        response.into_result()
    }

    /// 获取小程序全局唯一后台接口调用凭据
    pub async fn get_access_token(&self) -> LabradorResult<AccessToken> {
        self.wechat_client.get_access_token().await?;

        // 调用微信客户端的获取access_token方法
        let response: WechatApiResponse<AccessToken> = self
            .wechat_client
            .get(&format!(
                "/cgi-bin/token?grant_type=client_credential&appid={}&secret={}",
                self.config.app_id, self.config.app_secret
            ))
            .await?;
        response.into_result()
    }

    /// 获取配置
    pub fn config(&self) -> &WechatMiniAppConfig {
        &self.config
    }
}
