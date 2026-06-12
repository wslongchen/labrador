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

//! 微信开放平台 - 网站应用(web) API
//!
//! 提供网站应用扫码登录相关接口，包括：
//! - OAuth2 授权登录（扫码登录）
//! - 获取用户信息
//! - 刷新 access_token
//! - 校验授权凭证

use crate::errors::LabradorResult;
use crate::wechat::client::WechatApiResponse;
use crate::wechat::open::types::{
    OpenOauth2AccessTokenResponse, OpenOauth2AuthResponse, OpenOauth2UserInfo,
};
use crate::wechat::open::WechatOpenClient;

/// 微信开放平台网站应用服务
#[derive(Debug, Clone)]
pub struct WechatOpenWeb<'a> {
    client: &'a WechatOpenClient,
}

impl<'a> WechatOpenWeb<'a> {
    #[inline]
    pub fn new(client: &'a WechatOpenClient) -> Self {
        WechatOpenWeb { client }
    }

    /// 通过 code 换取网站应用 access_token
    ///
    /// 网站应用扫码登录的第二步：通过 authorization_code 换取 access_token。
    ///
    /// # 参数
    /// * `code` - 微信回调返回的 authorization_code
    ///
    /// # 返回
    /// 返回 `LabradorResult<OpenOauth2AccessTokenResponse>`，包含 access_token、refresh_token 和 openid 等信息
    ///
    /// # 官方文档
    /// <https://developers.weixin.qq.com/doc/oplatform/developers/dev/auth/web.html>
    pub async fn oauth2_access_token(
        &self,
        code: &str,
    ) -> LabradorResult<OpenOauth2AccessTokenResponse> {
        let config = self.client.config();
        let response: WechatApiResponse<OpenOauth2AccessTokenResponse> = self
            .client
            .wechat_client()
            .get(&format!(
                "/sns/oauth2/access_token?appid={}&secret={}&code={}&grant_type=authorization_code",
                config.app_id, config.app_secret, code
            ))
            .await?;
        response.into_result()
    }

    /// 刷新网站应用 access_token
    ///
    /// 由于 access_token 有效期较短，可以使用 refresh_token 刷新。
    /// refresh_token 有效期为 30 天。
    ///
    /// # 参数
    /// * `refresh_token` - 通过 oauth2_access_token 获取的 refresh_token
    ///
    /// # 返回
    /// 返回 `LabradorResult<OpenOauth2AccessTokenResponse>`，包含新的 access_token 和 refresh_token
    ///
    /// # 官方文档
    /// <https://developers.weixin.qq.com/doc/oplatform/developers/dev/auth/web.html>
    pub async fn refresh_access_token(
        &self,
        refresh_token: &str,
    ) -> LabradorResult<OpenOauth2AccessTokenResponse> {
        let config = self.client.config();
        let response: WechatApiResponse<OpenOauth2AccessTokenResponse> = self
            .client
            .wechat_client()
            .get(&format!(
                "/sns/oauth2/refresh_token?appid={}&grant_type=refresh_token&refresh_token={}",
                config.app_id, refresh_token
            ))
            .await?;
        response.into_result()
    }

    /// 获取网站应用用户信息
    ///
    /// 通过 access_token 和 openid 获取已授权用户的个人信息。
    /// 需要 scope 为 snsapi_userinfo（snsapi_login 已包含此权限）。
    ///
    /// # 参数
    /// * `access_token` - 网页授权接口调用凭证
    /// * `openid` - 用户的唯一标识
    ///
    /// # 返回
    /// 返回 `LabradorResult<OpenOauth2UserInfo>`，包含用户昵称、头像、性别等信息
    ///
    /// # 官方文档
    /// <https://developers.weixin.qq.com/doc/oplatform/developers/dev/auth/web.html>
    pub async fn get_user_info(
        &self,
        access_token: &str,
        openid: &str,
    ) -> LabradorResult<OpenOauth2UserInfo> {
        let response: WechatApiResponse<OpenOauth2UserInfo> = self
            .client
            .wechat_client()
            .get(&format!(
                "/sns/userinfo?access_token={}&openid={}&lang=zh_CN",
                access_token, openid
            ))
            .await?;
        response.into_result()
    }

    /// 校验网站应用授权凭证是否有效
    ///
    /// 检验 access_token 和 openid 的授权凭证是否有效。
    ///
    /// # 参数
    /// * `access_token` - 网页授权接口调用凭证
    /// * `openid` - 用户的唯一标识
    ///
    /// # 返回
    /// 返回 `LabradorResult<OpenOauth2AuthResponse>`，errmsg 为 "ok" 表示有效
    ///
    /// # 官方文档
    /// <https://developers.weixin.qq.com/doc/oplatform/developers/dev/auth/web.html>
    pub async fn auth(
        &self,
        access_token: &str,
        openid: &str,
    ) -> LabradorResult<OpenOauth2AuthResponse> {
        let response: WechatApiResponse<OpenOauth2AuthResponse> = self
            .client
            .wechat_client()
            .get(&format!(
                "/sns/auth?access_token={}&openid={}",
                access_token, openid
            ))
            .await?;
        response.into_result()
    }

    /// # 生成网站应用扫码登录 URL
    ///
    /// 生成微信扫码登录页面 URL，用户扫码确认后微信会重定向到 redirect_uri 并带上 code 参数。
    ///
    /// 参数：
    /// - redirect_uri: 回调地址（需要 URL 编码）
    /// - state: 防 CSRF 攻击的状态参数（建议随机生成并和 session 关联）
    ///
    /// [官方文档](https://developers.weixin.qq.com/doc/oplatform/developers/dev/auth/web.html)
    pub fn generate_login_url(&self, redirect_uri: &str, state: &str) -> String {
        let config = self.client.config();
        format!(
            "https://open.weixin.qq.com/connect/qrconnect?appid={}&redirect_uri={}&response_type=code&scope=snsapi_login&state={}#wechat_redirect",
            config.app_id,
            urlencoding::encode(redirect_uri),
            state
        )
    }
}
