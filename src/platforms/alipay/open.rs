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
//! 支付宝开放平台模块
//!
//! 提供支付宝开放平台的授权、用户信息等接口实现

use super::client::AlipayClient;
use crate::alipay::method::AlipayMethod;
use crate::alipay::AlipayBizRequest;
use crate::errors::LabradorResult;
use crate::platforms::alipay::AlipayResponse;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// 开放平台服务
pub struct AlipayOpenService<'a> {
    client: &'a AlipayClient,
}

impl<'a> AlipayOpenService<'a> {
    /// 创建新的开放平台服务
    pub fn new(client: &'a AlipayClient) -> Self {
        Self { client }
    }

    /// 换取授权访问令牌 (alipay.system.oauth.token)
    ///
    /// 通过授权码或刷新令牌换取用户的访问令牌（access_token）。
    ///
    /// # 参数
    /// * `grant_type` - 授权类型：authorization_code（用授权码换令牌）或 refresh_token（用刷新令牌换新令牌）
    /// * `code` - 授权码，grant_type 为 authorization_code 时必传
    /// * `refresh_token` - 刷新令牌，grant_type 为 refresh_token 时必传
    ///
    /// # 返回
    /// 返回 `SystemOauthTokenResponse`，包含 user_id、access_token、refresh_token、expires_in 等
    ///
    /// # 官方文档
    /// <https://opendocs.alipay.com/apis/api_9/alipay.system.oauth.token>
    pub async fn system_oauth_token(
        &self,
        grant_type: String,
        code: Option<String>,
        refresh_token: Option<String>,
    ) -> LabradorResult<SystemOauthTokenResponse> {
        let mut request = AlipayBizRequest::new();
        let mut biz_content = BTreeMap::new();
        biz_content.insert("grant_type".to_string(), grant_type);

        if let Some(auth_code) = code {
            biz_content.insert("code".to_string(), auth_code);
        }

        if let Some(refresh) = refresh_token {
            biz_content.insert("refresh_token".to_string(), refresh);
        }
        request.set_biz_model(biz_content);
        request.set_method(AlipayMethod::SystemOauthToken);
        let response: AlipayResponse<SystemOauthTokenResponse> =
            self.client.request(request, None, None, None).await?;

        response.into_result()
    }

    /// 换取应用授权令牌 (alipay.open.auth.token.app)
    ///
    /// 第三方应用通过授权码或刷新令牌换取商户的应用授权令牌（app_auth_token）。
    ///
    /// # 参数
    /// * `grant_type` - 授权类型：authorization_code 或 refresh_token
    /// * `code` - 授权码，grant_type 为 authorization_code 时必传
    /// * `refresh_token` - 刷新令牌，grant_type 为 refresh_token 时必传
    ///
    /// # 返回
    /// 返回 `OpenAuthTokenAppResponse`，包含 app_auth_token、app_refresh_token、
    ///   auth_app_id（授权商户的AppId）、user_id 等
    ///
    /// # 官方文档
    /// <https://opendocs.alipay.com/apis/api_9/alipay.open.auth.token.app>
    pub async fn open_auth_token_app(
        &self,
        grant_type: String,
        code: Option<String>,
        refresh_token: Option<String>,
    ) -> LabradorResult<OpenAuthTokenAppResponse> {
        let mut request = AlipayBizRequest::new();
        let mut biz_content = BTreeMap::new();
        biz_content.insert("grant_type".to_string(), grant_type);

        if let Some(auth_code) = code {
            biz_content.insert("code".to_string(), auth_code);
        }

        if let Some(refresh) = refresh_token {
            biz_content.insert("refresh_token".to_string(), refresh);
        }
        request.set_biz_model(biz_content);
        request.set_method(AlipayMethod::OpenAuthTokenApp);
        let response: AlipayResponse<OpenAuthTokenAppResponse> =
            self.client.request(request, None, None, None).await?;

        response.into_result()
    }

    /// 获取用户信息 (alipay.user.info.share)
    ///
    /// 通过用户授权令牌获取支付宝用户的公开信息。
    ///
    /// # 参数
    /// * `auth_token` - 用户授权令牌（通过 alipay.system.oauth.token 获取的 access_token）
    ///
    /// # 返回
    /// 返回 `UserInfoShareResponse`，包含 user_id、昵称、头像、性别、省份城市、实名状态等
    ///
    /// # 官方文档
    /// <https://opendocs.alipay.com/apis/api_2/alipay.user.info.share>
    pub async fn user_info_share(
        &self,
        auth_token: String,
    ) -> LabradorResult<UserInfoShareResponse> {
        let mut request = AlipayBizRequest::new();
        let mut biz_content = BTreeMap::new();
        // user_info_share 需要特殊处理，因为它没有biz_content
        biz_content.insert("auth_token".to_string(), auth_token.to_string());
        request.set_biz_model(biz_content);
        request.set_method(AlipayMethod::Custom("alipay.user.info.share".to_string()));
        let response: AlipayResponse<UserInfoShareResponse> = self
            .client
            .request(request, Some(auth_token), None, None)
            .await?;

        response.into_result()
    }

    /// 查询应用授权关系 (alipay.open.auth.token.app.query)
    ///
    /// 查询某个应用授权令牌（app_auth_token）对应的授权信息。
    ///
    /// # 参数
    /// * `app_auth_token` - 应用授权令牌
    ///
    /// # 返回
    /// 返回 `OpenAuthTokenAppQueryResponse`，包含授权商户 AppId、授权时间范围、状态等
    ///
    /// # 官方文档
    /// <https://opendocs.alipay.com/apis/api_9/alipay.open.auth.token.app.query>
    pub async fn open_auth_token_app_query(
        &self,
        app_auth_token: String,
    ) -> LabradorResult<OpenAuthTokenAppQueryResponse> {
        let mut request = AlipayBizRequest::new();
        let mut biz_content = BTreeMap::new();
        biz_content.insert("app_auth_token".to_string(), app_auth_token);
        request.set_biz_model(biz_content);
        request.set_method(AlipayMethod::Custom(
            "alipay.open.auth.token.app.query".to_string(),
        ));
        let response: AlipayResponse<OpenAuthTokenAppQueryResponse> =
            self.client.request(request, None, None, None).await?;

        response.into_result()
    }

    /// 查询授权权限列表 (alipay.open.auth.appauth.query)
    ///
    /// 查询某个第三方应用对商户的授权权限范围。
    ///
    /// # 参数
    /// * `auth_app_id` - 授权应用的 AppId
    /// * `scopes` - 需要查询的权限列表
    ///
    /// # 返回
    /// 返回 `OpenAuthAppAuthQueryResponse`，包含授权范围和状态
    ///
    /// # 官方文档
    /// <https://opendocs.alipay.com/apis/api_9/alipay.open.auth.appauth.query>
    pub async fn open_auth_app_auth_query(
        &self,
        auth_app_id: String,
        scopes: Vec<String>,
    ) -> LabradorResult<OpenAuthAppAuthQueryResponse> {
        let mut request = AlipayBizRequest::new();
        let mut biz_content = BTreeMap::new();
        biz_content.insert("auth_app_id".to_string(), auth_app_id);

        let scopes_json = serde_json::to_string(&scopes).unwrap_or_default();
        biz_content.insert("scopes".to_string(), scopes_json);
        request.set_biz_model(biz_content);
        request.set_method(AlipayMethod::Custom(
            "alipay.open.auth.appauth.query".to_string(),
        ));
        let response: AlipayResponse<OpenAuthAppAuthQueryResponse> =
            self.client.request(request, None, None, None).await?;

        response.into_result()
    }

    /// 取消授权 (alipay.open.auth.appauth.cancel)
    ///
    /// 取消第三方应用对商户的授权。
    ///
    /// # 参数
    /// * `app_auth_token` - 应用授权令牌
    ///
    /// # 返回
    /// 返回 `OpenAuthAppAuthCancelResponse`，包含取消结果
    ///
    /// # 官方文档
    /// <https://opendocs.alipay.com/apis/api_9/alipay.open.auth.appauth.cancel>
    pub async fn open_auth_app_auth_cancel(
        &self,
        app_auth_token: String,
    ) -> LabradorResult<OpenAuthAppAuthCancelResponse> {
        let mut request = AlipayBizRequest::new();
        let mut biz_content = BTreeMap::new();
        biz_content.insert("app_auth_token".to_string(), app_auth_token);
        request.set_biz_model(biz_content);
        request.set_method(AlipayMethod::Custom(
            "alipay.open.auth.appauth.cancel".to_string(),
        ));
        let response: AlipayResponse<OpenAuthAppAuthCancelResponse> =
            self.client.request(request, None, None, None).await?;

        response.into_result()
    }

    /// 生成授权 URL
    ///
    /// 生成支付宝网页授权的 URL，引导用户跳转到支付宝进行授权。
    ///
    /// # 参数
    /// * `redirect_uri` - 授权回调地址
    /// * `scopes` - 授权范围列表，如 auth_base（静默授权）、auth_user（用户信息授权）
    /// * `state` - 状态参数，用于防止 CSRF，可选
    ///
    /// # 返回
    /// 返回支付宝授权页面的完整 URL 字符串
    pub fn generate_auth_url(
        &self,
        redirect_uri: &str,
        scopes: Vec<&str>,
        state: Option<&str>,
    ) -> String {
        let app_id = self.client.config().app_id.clone();
        let scope = scopes.join(",");
        let state_param = state.unwrap_or("");

        format!(
            "https://openauth.alipay.com/oauth2/publicAppAuthorize.htm?\
            app_id={}&scope={}&redirect_uri={}&state={}",
            app_id, scope, redirect_uri, state_param
        )
    }

    /// 生成小程序授权 URL
    ///
    /// 生成支付宝小程序内授权的 URL（带 mode=miniapp 参数）。
    ///
    /// # 参数
    /// * `redirect_uri` - 授权回调地址
    /// * `scopes` - 授权范围列表
    /// * `state` - 状态参数，用于防止 CSRF，可选
    ///
    /// # 返回
    /// 返回支付宝小程序授权页面的完整 URL 字符串
    pub fn generate_miniapp_auth_url(
        &self,
        redirect_uri: &str,
        scopes: Vec<&str>,
        state: Option<&str>,
    ) -> String {
        let app_id = self.client.config().app_id.clone();
        let scope = scopes.join(",");
        let state_param = state.unwrap_or("");

        format!(
            "https://openauth.alipay.com/oauth2/publicAppAuthorize.htm?\
            app_id={}&scope={}&redirect_uri={}&state={}&mode=miniapp",
            app_id, scope, redirect_uri, state_param
        )
    }
}

/// 系统授权令牌响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemOauthTokenResponse {
    /// 支付宝用户ID
    pub user_id: String,
    /// 访问令牌
    pub access_token: String,
    /// 过期时间（秒）
    pub expires_in: i64,
    /// 刷新令牌
    pub refresh_token: String,
    /// 刷新令牌过期时间（秒）
    pub re_expires_in: i64,
    /// 授权开始时间
    pub auth_start: Option<String>,
}

/// 应用授权令牌响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAuthTokenAppResponse {
    /// 应用授权令牌
    pub app_auth_token: String,
    /// 刷新令牌
    pub app_refresh_token: String,
    /// 授权商户的AppId
    pub auth_app_id: String,
    /// 过期时间（秒）
    pub expires_in: String,
    /// 刷新令牌过期时间（秒）
    pub re_expires_in: String,
    /// 授权商户的UserId
    pub user_id: String,
}

/// 用户信息共享响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfoShareResponse {
    /// 支付宝用户ID
    pub user_id: String,
    /// 用户头像地址
    pub avatar: Option<String>,
    /// 省份名称
    pub province: Option<String>,
    /// 城市名称
    pub city: Option<String>,
    /// 用户昵称
    pub nick_name: Option<String>,
    /// 用户性别
    pub gender: Option<String>,
    /// 用户类型
    pub user_type: Option<String>,
    /// 用户状态
    pub user_status: Option<String>,
    /// 是否通过实名认证
    pub is_certified: Option<String>,
    /// 是否是学生
    pub is_student_certified: Option<String>,
}

/// 应用授权令牌查询响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAuthTokenAppQueryResponse {
    /// 授权商户的AppId
    pub auth_app_id: String,
    /// 授权生效时间
    pub auth_start: String,
    /// 授权失效时间
    pub auth_end: String,
    /// 授权状态
    pub status: String,
    /// 授权范围
    pub scopes: Option<Vec<String>>,
}

/// 应用授权查询响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAuthAppAuthQueryResponse {
    /// 授权商户的AppId
    pub auth_app_id: String,
    /// 授权范围
    pub scopes: Vec<String>,
    /// 授权状态
    pub status: String,
}

/// 取消授权响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAuthAppAuthCancelResponse {
    /// 结果码
    pub result_code: String,
    /// 结果描述
    pub result_msg: String,
}

/// 授权范围
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthScope {
    /// 静默授权，获取用户UserId
    Base,
    /// 用户信息授权，获取用户信息
    UserInfo,
}

impl AuthScope {
    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            AuthScope::Base => "auth_base",
            AuthScope::UserInfo => "auth_user",
        }
    }
}

/// 授权结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResult {
    /// 应用ID
    pub app_id: String,
    /// 授权码
    pub auth_code: Option<String>,
    /// 授权范围
    pub scope: Option<String>,
    /// 状态参数
    pub state: Option<String>,
}
