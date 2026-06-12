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

//! 微信开放平台 - 第三方平台(component) API
//!
//! 提供第三方平台代开发相关接口，包括：
//! - component_access_token 管理
//! - 预授权码获取
//! - 授权方令牌管理
//! - 授权方信息查询

use crate::errors::LabradorResult;
use crate::wechat::client::WechatApiResponse;
use crate::wechat::open::types::{
    ApiAuthorizerRefreshRequest, ApiAuthorizerTokenRequest, ApiAuthorizerTokenResponse,
    ApiGetAuthorizerInfoRequest, ApiGetAuthorizerInfoResponse, ApiGetAuthorizerOptionRequest,
    ApiSetAuthorizerOptionRequest, ComponentAccessTokenRequest, ComponentAccessTokenResponse,
    PreAuthCodeRequest, PreAuthCodeResponse,
};
use crate::wechat::open::WechatOpenClient;

/// 微信开放平台第三方平台服务
#[derive(Debug, Clone)]
pub struct WechatOpenComponent<'a> {
    client: &'a WechatOpenClient,
}

impl<'a> WechatOpenComponent<'a> {
    #[inline]
    pub fn new(client: &'a WechatOpenClient) -> Self {
        WechatOpenComponent { client }
    }

    /// 获取第三方平台 component_access_token
    ///
    /// 第三方平台 access_token 是第三方平台接口的调用凭据。
    /// component_access_token 有效期为 2 小时，需要定时刷新。
    ///
    /// # 参数
    /// * `component_verify_ticket` - 微信服务器推送的 ticket，用于验证第三方平台身份
    ///
    /// # 返回
    /// 返回 `LabradorResult<ComponentAccessTokenResponse>`，包含 component_access_token 及有效期
    ///
    /// # 官方文档
    /// <https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/ThirdParty/token/component_access_token.html>
    pub async fn get_component_access_token(
        &self,
        component_verify_ticket: &str,
    ) -> LabradorResult<ComponentAccessTokenResponse> {
        let config = self.client.config();
        let request = ComponentAccessTokenRequest {
            component_appid: config.app_id.clone(),
            component_appsecret: config.app_secret.clone(),
            component_verify_ticket: component_verify_ticket.to_string(),
        };

        let response: WechatApiResponse<ComponentAccessTokenResponse> = self
            .client
            .wechat_client()
            .post("/cgi-bin/component/api_component_token", request)
            .await?;
        response.into_result()
    }

    /// 获取预授权码 pre_auth_code
    ///
    /// 预授权码用于引导用户进入授权页。
    /// 每个预授权码有效期为 30 分钟。
    ///
    /// 注意：调用此接口需要有效的 component_access_token。
    ///
    /// # 参数
    /// * `component_access_token` - 第三方平台的 component_access_token
    ///
    /// # 返回
    /// 返回 `LabradorResult<PreAuthCodeResponse>`，包含预授权码及有效期
    ///
    /// # 官方文档
    /// <https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/ThirdParty/token/pre_auth_code.html>
    pub async fn create_pre_auth_code(
        &self,
        component_access_token: &str,
    ) -> LabradorResult<PreAuthCodeResponse> {
        let config = self.client.config();
        let request = PreAuthCodeRequest {
            component_appid: config.app_id.clone(),
        };

        let response: WechatApiResponse<PreAuthCodeResponse> = self
            .client
            .wechat_client()
            .post(
                &format!(
                    "/cgi-bin/component/api_create_preauthcode?component_access_token={}",
                    component_access_token
                ),
                request,
            )
            .await?;
        response.into_result()
    }

    /// 使用授权码换取 authorizer_access_token
    ///
    /// 当用户在授权页上确认授权后，微信会重定向到回调 URI 并带上 authorization_code，
    /// 第三方平台使用此接口换取 authorizer_access_token 和 authorizer_refresh_token。
    ///
    /// authorizer_access_token 有效期为 2 小时，authorizer_refresh_token 有效期较长。
    ///
    /// # 参数
    /// * `component_access_token` - 第三方平台的 component_access_token
    /// * `authorization_code` - 授权码，用户在授权页确认授权后由微信回调返回
    /// * `authorizer_appid` - 授权方的 appid
    ///
    /// # 返回
    /// 返回 `LabradorResult<ApiAuthorizerTokenResponse>`，包含 authorizer_access_token 和 authorizer_refresh_token
    ///
    /// # 官方文档
    /// <https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/ThirdParty/authorization/api_authorization_token.html>
    pub async fn get_authorizer_access_token(
        &self,
        component_access_token: &str,
        authorization_code: &str,
        authorizer_appid: &str,
    ) -> LabradorResult<ApiAuthorizerTokenResponse> {
        let config = self.client.config();
        let request = ApiAuthorizerTokenRequest {
            component_appid: config.app_id.clone(),
            authorization_code: authorization_code.to_string(),
            authorizer_appid: authorizer_appid.to_string(),
        };

        let response: WechatApiResponse<ApiAuthorizerTokenResponse> = self
            .client
            .wechat_client()
            .post(
                &format!(
                    "/cgi-bin/component/api_authorizer_token?component_access_token={}",
                    component_access_token
                ),
                request,
            )
            .await?;
        response.into_result()
    }

    /// 刷新 authorizer_access_token
    ///
    /// authorizer_access_token 过期后，使用 authorizer_refresh_token 刷新获取新的令牌。
    ///
    /// # 参数
    /// * `component_access_token` - 第三方平台的 component_access_token
    /// * `authorizer_appid` - 授权方的 appid
    /// * `authorizer_refresh_token` - 授权方的刷新令牌
    ///
    /// # 返回
    /// 返回 `LabradorResult<ApiAuthorizerTokenResponse>`，包含新的 authorizer_access_token 和 authorizer_refresh_token
    ///
    /// # 官方文档
    /// <https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/ThirdParty/authorization/api_authorization_token.html>
    pub async fn refresh_authorizer_access_token(
        &self,
        component_access_token: &str,
        authorizer_appid: &str,
        authorizer_refresh_token: &str,
    ) -> LabradorResult<ApiAuthorizerTokenResponse> {
        let config = self.client.config();
        let request = ApiAuthorizerRefreshRequest {
            component_appid: config.app_id.clone(),
            authorizer_appid: authorizer_appid.to_string(),
            authorizer_refresh_token: authorizer_refresh_token.to_string(),
        };

        let response: WechatApiResponse<ApiAuthorizerTokenResponse> = self
            .client
            .wechat_client()
            .post(
                &format!(
                    "/cgi-bin/component/api_authorizer_token?component_access_token={}",
                    component_access_token
                ),
                request,
            )
            .await?;
        response.into_result()
    }

    /// 获取授权方账号信息
    ///
    /// 获取授权方的账号基本信息，包括昵称、头像、服务类型、认证类型等。
    ///
    /// # 参数
    /// * `component_access_token` - 第三方平台的 component_access_token
    /// * `authorizer_appid` - 授权方的 appid
    ///
    /// # 返回
    /// 返回 `LabradorResult<ApiGetAuthorizerInfoResponse>`，包含授权方账号基本信息
    ///
    /// # 官方文档
    /// <https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/ThirdParty/authorization/api_get_authorizer_info.html>
    pub async fn get_authorizer_info(
        &self,
        component_access_token: &str,
        authorizer_appid: &str,
    ) -> LabradorResult<ApiGetAuthorizerInfoResponse> {
        let config = self.client.config();
        let request = ApiGetAuthorizerInfoRequest {
            component_appid: config.app_id.clone(),
            authorizer_appid: authorizer_appid.to_string(),
        };

        let response: WechatApiResponse<ApiGetAuthorizerInfoResponse> = self
            .client
            .wechat_client()
            .post(
                &format!(
                    "/cgi-bin/component/api_get_authorizer_info?component_access_token={}",
                    component_access_token
                ),
                request,
            )
            .await?;
        response.into_result()
    }

    /// 获取授权方选项信息
    ///
    /// 获取授权方的选项设置信息，如地理位置上报、语音识别开关等。
    ///
    /// # 参数
    /// * `component_access_token` - 第三方平台的 component_access_token
    /// * `authorizer_appid` - 授权方的 appid
    /// * `option_name` - 选项名称，如 `location_report`、`voice_recognize`、`customer_service` 等
    ///
    /// # 返回
    /// 返回 `LabradorResult<serde_json::Value>`，包含选项的当前值
    ///
    /// # 官方文档
    /// <https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/ThirdParty/authorization/api_get_authorizer_option.html>
    pub async fn get_authorizer_option(
        &self,
        component_access_token: &str,
        authorizer_appid: &str,
        option_name: &str,
    ) -> LabradorResult<serde_json::Value> {
        let config = self.client.config();
        let request = ApiGetAuthorizerOptionRequest {
            component_appid: config.app_id.clone(),
            authorizer_appid: authorizer_appid.to_string(),
            option_name: option_name.to_string(),
        };

        let response: WechatApiResponse<serde_json::Value> = self
            .client
            .wechat_client()
            .post(
                &format!(
                    "/cgi-bin/component/api_get_authorizer_option?component_access_token={}",
                    component_access_token
                ),
                request,
            )
            .await?;
        response.into_result()
    }

    /// 设置授权方选项信息
    ///
    /// 设置授权方的选项信息，如地理位置上报、语音识别开关等。
    ///
    /// # 参数
    /// * `component_access_token` - 第三方平台的 component_access_token
    /// * `authorizer_appid` - 授权方的 appid
    /// * `option_name` - 选项名称，如 `location_report`、`voice_recognize`、`customer_service` 等
    /// * `option_value` - 选项值
    ///
    /// # 返回
    /// 返回 `LabradorResult<serde_json::Value>`，设置成功时为空 JSON
    ///
    /// # 官方文档
    /// <https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/ThirdParty/authorization/api_set_authorizer_option.html>
    pub async fn set_authorizer_option(
        &self,
        component_access_token: &str,
        authorizer_appid: &str,
        option_name: &str,
        option_value: &str,
    ) -> LabradorResult<serde_json::Value> {
        let config = self.client.config();
        let request = ApiSetAuthorizerOptionRequest {
            component_appid: config.app_id.clone(),
            authorizer_appid: authorizer_appid.to_string(),
            option_name: option_name.to_string(),
            option_value: option_value.to_string(),
        };

        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post(
                &format!(
                    "/cgi-bin/component/api_set_authorizer_option?component_access_token={}",
                    component_access_token
                ),
                request,
            )
            .await?;
        response.into_result()
    }

    /// 生成授权页面 URL
    ///
    /// 生成引导用户进入授权页面的 URL。
    /// 用户确认授权后，微信会重定向到回调 URI 并带上 authorization_code。
    ///
    /// # 参数
    /// * `pre_auth_code` - 预授权码，通过 `create_pre_auth_code` 获取
    /// * `redirect_uri` - 授权后的回调地址
    /// * `auth_type` - 授权类型：1 表示仅展示公众号，2 表示仅展示小程序，3 表示两者都展示
    ///
    /// # 返回
    /// 返回授权页面的完整 URL
    ///
    /// # 官方文档
    /// <https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/ThirdParty/authorization/api_before_auth.html>
    pub fn generate_auth_url(
        &self,
        pre_auth_code: &str,
        redirect_uri: &str,
        auth_type: Option<i32>,
    ) -> String {
        let config = self.client.config();
        let mut url = format!(
            "https://mp.weixin.qq.com/cgi-bin/componentloginpage?component_appid={}&pre_auth_code={}&redirect_uri={}",
            config.app_id, pre_auth_code, redirect_uri
        );

        if let Some(auth_type) = auth_type {
            url.push_str(&format!("&auth_type={}", auth_type));
        }

        url
    }
}
