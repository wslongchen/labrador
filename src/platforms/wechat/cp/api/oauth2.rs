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
use serde::Deserialize;
use serde_json::json;

use crate::errors::LabradorResult;
use crate::wechat::client::WechatApiResponse;
use crate::wechat::cp::types::{SNSAPI_BASE, SNSAPI_PRIVATEINFO, SNSAPI_USERINFO};
use crate::wechat::cp::WechatCpClient;

/// 企业微信OAuth2授权模块
///
/// 包含网页授权登录、获取用户身份、获取用户详细信息等功能。
#[derive(Debug, Clone)]
pub struct WechatCpOauth2<'a> {
    client: &'a WechatCpClient,
}

#[allow(unused)]
impl<'a> WechatCpOauth2<'a> {
    #[inline]
    pub fn new(client: &'a WechatCpClient) -> Self {
        Self { client }
    }

    /// 构造OAuth2授权链接
    ///
    /// 详情请见：<https://work.weixin.qq.com/api/doc/90000/90135/91022>
    ///
    /// # 参数说明
    /// * `redirect_uri` - 授权后重定向的回调链接地址，请使用urlencode对链接进行处理
    /// * `scope` - 应用授权作用域：
    ///   - `snsapi_base`：静默授权，可获取成员的基础信息（UserId与DeviceId）
    ///   - `snsapi_userinfo`：静默授权，可获取成员的详细信息，但不包含手机、邮箱等敏感信息
    ///   - `snsapi_privateinfo`：手动授权，可获取成员的详细信息，包含手机、邮箱等敏感信息
    /// * `state` - 重定向后会带上state参数，企业可以填写a-zA-Z0-9的参数值，最多128字节
    pub fn build_authorization_url(
        &self,
        redirect_uri: &str,
        scope: &str,
        state: Option<&str>,
    ) -> String {
        let mut url = format!(
            "https://open.weixin.qq.com/connect/oauth2/authorize?appid={}&redirect_uri={}&response_type=code&scope={}",
            self.client.corp_id(),
            urlencoding::encode(redirect_uri),
            scope
        );

        // 对于需要agentid的scope，添加agentid参数
        if scope == SNSAPI_PRIVATEINFO || scope == SNSAPI_USERINFO {
            if let Some(agent_id) = self.client.agent_id() {
                url.push_str("&agentid=");
                url.push_str(&agent_id.to_string());
            }
        }

        if let Some(s) = state {
            url.push_str("&state=");
            url.push_str(s);
        }

        url.push_str("#wechat_redirect");
        url
    }

    /// 构造OAuth2授权链接（使用snsapi_base scope）
    ///
    /// 静默授权，可获取成员的基础信息（UserId与DeviceId）
    pub fn build_authorization_url_base(&self, redirect_uri: &str, state: Option<&str>) -> String {
        self.build_authorization_url(redirect_uri, SNSAPI_BASE, state)
    }

    /// 构造OAuth2授权链接（使用snsapi_userinfo scope）
    ///
    /// 静默授权，可获取成员的详细信息，但不包含手机、邮箱等敏感信息
    pub fn build_authorization_url_userinfo(
        &self,
        redirect_uri: &str,
        state: Option<&str>,
    ) -> String {
        self.build_authorization_url(redirect_uri, SNSAPI_USERINFO, state)
    }

    /// 构造OAuth2授权链接（使用snsapi_privateinfo scope）
    ///
    /// 手动授权，可获取成员的详细信息，包含手机、邮箱等敏感信息
    pub fn build_authorization_url_privateinfo(
        &self,
        redirect_uri: &str,
        state: Option<&str>,
    ) -> String {
        self.build_authorization_url(redirect_uri, SNSAPI_PRIVATEINFO, state)
    }

    /// 获取访问用户身份（新版）
    ///
    /// 该接口用于根据code获取成员信息，适用于自建应用与代开发应用。
    /// 详情请见：<https://work.weixin.qq.com/api/doc/90000/90135/91023>
    ///
    /// # 参数说明
    /// * `code` - 通过成员授权获取到的code，每次成员授权带上的code将不一样，code只能使用一次，5分钟未被使用自动过期
    pub async fn get_user_info(&self, code: &str) -> LabradorResult<UserInfoResponse> {
        let response: WechatApiResponse<UserInfoResponse> = self
            .client
            .get(&format!("/cgi-bin/auth/getuserinfo?code={}", code))
            .await?;
        response.into_result()
    }

    /// 获取访问用户身份（旧版）
    ///
    /// 根据code获取成员信息，适用于自建应用与代开发应用。
    /// 注意：需要指定agent_id，该方法可能已过时，建议使用 `get_user_info`。
    pub async fn get_user_info_with_agent(
        &self,
        code: &str,
        agent_id: i32,
    ) -> LabradorResult<UserInfoResponse> {
        let response: WechatApiResponse<UserInfoResponse> = self
            .client
            .get(&format!(
                "/cgi-bin/user/getuserinfo?code={}&agentid={}",
                code, agent_id
            ))
            .await?;
        response.into_result()
    }

    /// 使用user_ticket获取成员详情
    ///
    /// 详情请见：<https://work.weixin.qq.com/api/doc/90000/90135/91023#获取成员详情>
    ///
    /// # 参数说明
    /// * `user_ticket` - 成员票据，通过get_user_info获取
    pub async fn get_user_detail(&self, user_ticket: &str) -> LabradorResult<UserDetail> {
        let response: WechatApiResponse<UserDetail> = self
            .client
            .post(
                "/cgi-bin/auth/getuserdetail",
                json!({ "user_ticket": user_ticket }),
            )
            .await?;
        response.into_result()
    }

    /// 获取访问用户敏感信息
    ///
    /// 自建应用与代开发应用可通过该接口获取成员授权的敏感字段。
    /// 详情请见：<https://developer.work.weixin.qq.com/document/path/95833>
    ///
    /// # 参数说明
    /// * `user_ticket` - 成员票据，通过get_user_info获取
    pub async fn get_user_sensitive_info(
        &self,
        user_ticket: &str,
    ) -> LabradorResult<UserSensitiveInfo> {
        let response: WechatApiResponse<UserSensitiveInfo> = self
            .client
            .post(
                "/cgi-bin/auth/getuser_sensitive_info",
                json!({ "user_ticket": user_ticket }),
            )
            .await?;
        response.into_result()
    }
}

// ============================================================================
// 响应结构体
// ============================================================================

/// 用户信息响应（get_user_info接口返回）
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfoResponse {
    /// 成员UserID。若需要获得用户详情信息，可调用通讯录接口：<https://work.weixin.qq.com/api/doc/90000/90135/90196>
    pub userid: Option<String>,
    /// 非企业成员的openid
    pub openid: Option<String>,
    /// 外部联系人id，当用户是微信用户时返回
    pub external_userid: Option<String>,
    /// 成员票据，最大有效期为1800秒
    pub user_ticket: Option<String>,
    /// user_ticket的有效时间（秒）
    pub expires_in: Option<i64>,
}

/// 用户详情（get_user_detail接口返回）
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDetail {
    /// 成员UserID
    pub userid: String,
    /// 成员姓名
    pub name: String,
    /// 成员手机号，仅在用户同意snsapi_privateinfo授权时返回
    pub mobile: Option<String>,
    /// 性别。0表示未定义，1表示男性，2表示女性
    pub gender: Option<i32>,
    /// 成员邮箱，仅在用户同意snsapi_privateinfo授权时返回
    pub email: Option<String>,
    /// 头像url。注：如果要获取小图将url最后的”/0”改成”/100”即可。
    /// 仅在用户同意snsapi_privateinfo授权时返回
    pub avatar: Option<String>,
    /// 员工个人二维码（扫描可添加为外部联系人），仅在用户同意snsapi_privateinfo授权时返回
    pub qr_code: Option<String>,
    /// 地址
    pub address: Option<String>,
}

/// 用户敏感信息（get_user_sensitive_info接口返回）
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSensitiveInfo {
    /// 成员UserID
    pub userid: String,
    /// 成员姓名
    pub name: String,
    /// 性别。0表示未定义，1表示男性，2表示女性
    pub gender: Option<i32>,
    /// 头像url
    pub avatar: Option<String>,
    /// 员工个人二维码
    pub qr_code: Option<String>,
    /// 手机号码
    pub mobile: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 企业邮箱
    pub biz_mail: Option<String>,
    /// 地址
    pub address: Option<String>,
}
