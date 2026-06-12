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

//! 微信开放平台类型定义

use serde::{Deserialize, Serialize};

/// component_verify_ticket 推送内容（解密后的结构）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentVerifyTicket {
    /// 第三方平台 AppID
    #[serde(rename = "AppId")]
    pub app_id: String,
    /// 消息创建时间
    #[serde(rename = "CreateTime")]
    pub create_time: i64,
    /// 信息类型：component_verify_ticket
    #[serde(rename = "InfoType")]
    pub info_type: String,
    /// 验证票据
    #[serde(rename = "ComponentVerifyTicket")]
    pub component_verify_ticket: String,
}

/// 授权事件通知
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationEvent {
    /// 第三方平台 AppID
    #[serde(rename = "AppId")]
    pub app_id: String,
    /// 消息创建时间
    #[serde(rename = "CreateTime")]
    pub create_time: i64,
    /// 信息类型：authorized / updateauthorized / unauthorized
    #[serde(rename = "InfoType")]
    pub info_type: String,
    /// 授权方 AppID
    #[serde(rename = "AuthorizerAppid")]
    pub authorizer_appid: Option<String>,
    /// 预授权码（用于换取授权码）
    #[serde(rename = "PreAuthCode")]
    pub pre_auth_code: Option<String>,
    /// 授权码（在授权成功事件中）
    #[serde(rename = "AuthorizationCode")]
    pub authorization_code: Option<String>,
    /// 授权码过期时间
    #[serde(rename = "AuthorizationCodeExpiredTime")]
    pub authorization_code_expired_time: Option<i64>,
}

/// 获取 component_access_token 的请求
#[derive(Debug, Clone, Serialize)]
pub struct ComponentAccessTokenRequest {
    /// 第三方平台 appid
    pub component_appid: String,
    /// 第三方平台 appsecret
    pub component_appsecret: String,
    /// 微信后台推送的 ticket
    pub component_verify_ticket: String,
}

/// component_access_token 响应
#[derive(Debug, Clone, Deserialize)]
pub struct ComponentAccessTokenResponse {
    /// 第三方平台令牌
    pub component_access_token: String,
    /// 有效时间（秒）
    pub expires_in: i64,
}

/// 获取预授权码请求
#[derive(Debug, Clone, Serialize)]
pub struct PreAuthCodeRequest {
    /// 第三方平台 appid
    pub component_appid: String,
}

/// 预授权码响应
#[derive(Debug, Clone, Deserialize)]
pub struct PreAuthCodeResponse {
    /// 预授权码
    pub pre_auth_code: String,
    /// 有效时间（秒）
    pub expires_in: i64,
}

/// 使用授权码换取 authorizer_access_token 的请求
#[derive(Debug, Clone, Serialize)]
pub struct ApiAuthorizerTokenRequest {
    /// 第三方平台 appid
    pub component_appid: String,
    /// 授权码
    pub authorization_code: String,
    /// 授权方 appid
    pub authorizer_appid: String,
}

/// authorizer_access_token 响应
#[derive(Debug, Clone, Deserialize)]
pub struct ApiAuthorizerTokenResponse {
    /// 授权方令牌
    pub authorizer_access_token: String,
    /// 有效时间（秒）
    pub expires_in: i64,
    /// 刷新令牌
    pub authorizer_refresh_token: String,
}

/// 刷新 authorizer_access_token 的请求
#[derive(Debug, Clone, Serialize)]
pub struct ApiAuthorizerRefreshRequest {
    /// 第三方平台 appid
    pub component_appid: String,
    /// 授权方 appid
    pub authorizer_appid: String,
    /// 刷新令牌
    pub authorizer_refresh_token: String,
}

/// 获取授权方账号信息请求
#[derive(Debug, Clone, Serialize)]
pub struct ApiGetAuthorizerInfoRequest {
    /// 第三方平台 appid
    pub component_appid: String,
    /// 授权方 appid
    pub authorizer_appid: String,
}

/// 授权方基础信息
#[derive(Debug, Clone, Deserialize)]
pub struct AuthorizerInfo {
    /// 昵称
    pub nick_name: String,
    /// 头像
    pub head_img: String,
    /// 服务类型
    pub service_type_info: Option<ServiceTypeInfo>,
    /// 认证类型
    pub verify_type_info: Option<VerifyTypeInfo>,
    /// 用户名（原始ID）
    pub user_name: String,
    /// 主体名称
    pub principal_name: Option<String>,
    /// 二维码地址
    pub qrcode_url: Option<String>,
    /// 账号状态
    pub account_status: Option<i32>,
    /// 签名
    pub signature: Option<String>,
}

/// 服务类型信息
#[derive(Debug, Clone, Deserialize)]
pub struct ServiceTypeInfo {
    /// 服务类型ID
    pub id: i32,
}

/// 认证类型信息
#[derive(Debug, Clone, Deserialize)]
pub struct VerifyTypeInfo {
    /// 认证类型ID
    pub id: i32,
}

/// 获取授权方账号信息响应
#[derive(Debug, Clone, Deserialize)]
pub struct ApiGetAuthorizerInfoResponse {
    /// 授权方基础信息
    pub authorizer_info: AuthorizerInfo,
    /// 授权信息
    pub authorization_info: Option<AuthorizationInfo>,
}

/// 授权信息
#[derive(Debug, Clone, Deserialize)]
pub struct AuthorizationInfo {
    /// 授权方 appid
    pub authorizer_appid: String,
    /// 授权方接口权限集
    pub func_info: Option<Vec<FuncInfo>>,
}

/// 权限集信息
#[derive(Debug, Clone, Deserialize)]
pub struct FuncInfo {
    /// 权限集 scope
    pub funcscope_category: Option<FuncscopeCategory>,
}

/// 权限集分类
#[derive(Debug, Clone, Deserialize)]
pub struct FuncscopeCategory {
    /// 权限集ID
    pub id: i32,
}

/// 获取授权方选项信息请求
#[derive(Debug, Clone, Serialize)]
pub struct ApiGetAuthorizerOptionRequest {
    /// 第三方平台 appid
    pub component_appid: String,
    /// 授权方 appid
    pub authorizer_appid: String,
    /// 选项名称
    pub option_name: String,
}

/// 设置授权方选项请求
#[derive(Debug, Clone, Serialize)]
pub struct ApiSetAuthorizerOptionRequest {
    /// 第三方平台 appid
    pub component_appid: String,
    /// 授权方 appid
    pub authorizer_appid: String,
    /// 选项名称
    pub option_name: String,
    /// 选项值
    pub option_value: String,
}

/// 网站应用 OAuth2 授权码换取 access_token 响应
#[derive(Debug, Clone, Deserialize)]
pub struct OpenOauth2AccessTokenResponse {
    /// 访问令牌
    pub access_token: String,
    /// 刷新令牌
    pub refresh_token: Option<String>,
    /// 用户唯一标识
    pub openid: String,
    /// 授权范围
    pub scope: String,
    /// 过期时间（秒）
    pub expires_in: i64,
    /// 用户 unionid
    pub unionid: Option<String>,
}

/// 网站应用 OAuth2 用户信息响应
#[derive(Debug, Clone, Deserialize)]
pub struct OpenOauth2UserInfo {
    /// 用户唯一标识
    pub openid: String,
    /// 昵称
    pub nickname: Option<String>,
    /// 性别
    pub sex: Option<u8>,
    /// 城市
    pub city: Option<String>,
    /// 省份
    pub province: Option<String>,
    /// 国家
    pub country: Option<String>,
    /// 头像地址
    pub headimgurl: Option<String>,
    /// 用户 unionid
    pub unionid: Option<String>,
}

/// 校验授权凭证响应
#[derive(Debug, Clone, Deserialize)]
pub struct OpenOauth2AuthResponse {
    /// 是否有效
    pub errcode: i32,
    /// 错误信息
    pub errmsg: String,
}

/// 授权事件推送类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorizationEventType {
    /// 推送 component_verify_ticket
    ComponentVerifyTicket,
    /// 授权成功
    Authorized,
    /// 授权更新
    UpdateAuthorized,
    /// 取消授权
    Unauthorized,
    /// 未知事件
    Unknown(String),
}

impl From<&str> for AuthorizationEventType {
    fn from(s: &str) -> Self {
        match s {
            "component_verify_ticket" => AuthorizationEventType::ComponentVerifyTicket,
            "authorized" => AuthorizationEventType::Authorized,
            "updateauthorized" => AuthorizationEventType::UpdateAuthorized,
            "unauthorized" => AuthorizationEventType::Unauthorized,
            other => AuthorizationEventType::Unknown(other.to_string()),
        }
    }
}

/// 授权作用域（OAuth2）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenAuthScope {
    /// 静默授权（仅获取 openid）
    SnsapiBase,
    /// 用户信息授权（获取昵称头像等）
    SnsapiUserInfo,
    /// 网站应用扫码登录
    SnsapiLogin,
}

impl OpenAuthScope {
    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            OpenAuthScope::SnsapiBase => "snsapi_base",
            OpenAuthScope::SnsapiUserInfo => "snsapi_userinfo",
            OpenAuthScope::SnsapiLogin => "snsapi_login",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_verify_ticket_deserialize() {
        let xml = r#"<xml>
            <AppId>wx_test_appid</AppId>
            <CreateTime>1413192605</CreateTime>
            <InfoType>component_verify_ticket</InfoType>
            <ComponentVerifyTicket>test_ticket_content</ComponentVerifyTicket>
        </xml>"#;

        // XML 反序列化测试
        let result = serde_xml_rs::from_str::<ComponentVerifyTicket>(xml);
        assert!(result.is_ok());
        let ticket = result.unwrap();
        assert_eq!(ticket.app_id, "wx_test_appid");
        assert_eq!(ticket.info_type, "component_verify_ticket");
        assert_eq!(ticket.component_verify_ticket, "test_ticket_content");
    }

    #[test]
    fn test_component_access_token_request_serialization() {
        let req = ComponentAccessTokenRequest {
            component_appid: "wx_appid".to_string(),
            component_appsecret: "wx_secret".to_string(),
            component_verify_ticket: "ticket_123".to_string(),
        };
        let json = serde_json::to_string(&req).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["component_appid"], "wx_appid");
        assert_eq!(parsed["component_verify_ticket"], "ticket_123");
    }

    #[test]
    fn test_component_access_token_response_deserialize() {
        let json = r#"{"component_access_token":"token_abc","expires_in":7200}"#;
        let resp: ComponentAccessTokenResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.component_access_token, "token_abc");
        assert_eq!(resp.expires_in, 7200);
    }

    #[test]
    fn test_pre_auth_code_response_deserialize() {
        let json = r#"{"pre_auth_code":"pre_code_xyz","expires_in":1800}"#;
        let resp: PreAuthCodeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.pre_auth_code, "pre_code_xyz");
        assert_eq!(resp.expires_in, 1800);
    }

    #[test]
    fn test_api_authorizer_token_response_deserialize() {
        let json = r#"{
            "authorizer_access_token": "auth_token_123",
            "expires_in": 7200,
            "authorizer_refresh_token": "refresh_token_456"
        }"#;
        let resp: ApiAuthorizerTokenResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.authorizer_access_token, "auth_token_123");
        assert_eq!(resp.expires_in, 7200);
        assert_eq!(resp.authorizer_refresh_token, "refresh_token_456");
    }

    #[test]
    fn test_open_oauth2_token_response_deserialize() {
        let json = r#"{
            "access_token": "open_access_token",
            "refresh_token": "open_refresh_token",
            "openid": "openid_abc",
            "scope": "snsapi_login",
            "expires_in": 7200,
            "unionid": "unionid_xyz"
        }"#;
        let resp: OpenOauth2AccessTokenResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.access_token, "open_access_token");
        assert_eq!(resp.openid, "openid_abc");
        assert_eq!(resp.scope, "snsapi_login");
        assert_eq!(resp.unionid, Some("unionid_xyz".to_string()));
    }

    #[test]
    fn test_authorization_event_type_from_str() {
        assert_eq!(
            AuthorizationEventType::from("component_verify_ticket"),
            AuthorizationEventType::ComponentVerifyTicket
        );
        assert_eq!(
            AuthorizationEventType::from("authorized"),
            AuthorizationEventType::Authorized
        );
        assert_eq!(
            AuthorizationEventType::from("unauthorized"),
            AuthorizationEventType::Unauthorized
        );
        assert!(matches!(
            AuthorizationEventType::from("unknown_event"),
            AuthorizationEventType::Unknown(_)
        ));
    }
}
