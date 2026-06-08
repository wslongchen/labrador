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
use serde::{Deserialize, Serialize};

use crate::errors::LabradorResult;
use crate::wechat::client::WechatApiResponse;
use crate::wechat::cp::WechatCpClient;

/// 企业微信应用管理模块
#[derive(Debug, Clone)]
pub struct WechatCpAgent<'a> {
    client: &'a WechatCpClient,
}

#[allow(unused)]
impl<'a> WechatCpAgent<'a> {
    #[inline]
    pub fn new(client: &'a WechatCpClient) -> Self {
        Self { client }
    }

    /// 获取应用信息
    ///
    /// 该API用于获取企业号某个应用的基本信息，包括头像、昵称、帐号类型、认证类型、可见范围等信息。
    /// 详情请见：<https://work.weixin.qq.com/api/doc/10087>
    pub async fn get(&self, agent_id: i32) -> LabradorResult<AgentInfo> {
        let response: WechatApiResponse<AgentInfo> = self
            .client
            .get(&format!("/cgi-bin/agent/get?agentid={}", agent_id))
            .await?;
        response.into_result()
    }

    /// 设置应用
    ///
    /// 仅企业可调用，可设置当前凭证对应的应用；第三方不可调用。
    /// 详情请见：<https://work.weixin.qq.com/api/doc/10088>
    pub async fn set(&self, agent_info: &SetAgentRequest) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse =
            self.client.post("/cgi-bin/agent/set", agent_info).await?;
        Ok(response)
    }

    /// 获取应用列表
    ///
    /// 企业仅可获取当前凭证对应的应用；第三方仅可获取被授权的应用。
    /// 详情请见：<https://work.weixin.qq.com/api/doc/11214>
    pub async fn list(&self) -> LabradorResult<Vec<AgentInfo>> {
        let response: WechatApiResponse<AgentListResponse> =
            self.client.get("/cgi-bin/agent/list").await?;
        Ok(response.into_result()?.agentlist)
    }
}

// ============================================================================
// 请求与响应结构体
// ============================================================================

/// 应用信息（响应）
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentInfo {
    /// 应用id
    pub agentid: i32,
    /// 应用名称
    pub name: String,
    /// 应用方形头像url
    #[serde(skip_serializing_if = "Option::is_none")]
    pub square_logo_url: Option<String>,
    /// 应用圆形头像url
    #[serde(skip_serializing_if = "Option::is_none")]
    pub round_logo_url: Option<String>,
    /// 应用详情
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 可见用户范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_userinfos: Option<Users>,
    /// 可见部门范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_partys: Option<Parties>,
    /// 可见标签范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_tags: Option<Tags>,
    /// 是否关闭应用：1-关闭，0-开启
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close: Option<i32>,
    /// 可信域名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_domain: Option<String>,
    /// 是否上报用户地理位置：1-上报，0-不上报
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_location_flag: Option<i32>,
    /// 是否上报用户进入应用事件：1-上报，0-不上报
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isreportenter: Option<i32>,
    /// 应用主页url
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_url: Option<String>,
}

/// 设置应用请求
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAgentRequest {
    /// 应用id
    pub agentid: i32,
    /// 应用名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 应用方形头像media_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_mediaid: Option<String>,
    /// 应用详情
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 可信域名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_domain: Option<String>,
    /// 是否上报用户地理位置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_location_flag: Option<i32>,
    /// 是否上报用户进入应用事件
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isreportenter: Option<i32>,
    /// 应用主页url
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_url: Option<String>,
}

/// 可见用户范围
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Users {
    /// 用户列表
    pub user: Vec<UserItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserItem {
    /// 用户id
    pub userid: String,
}

/// 可见部门范围
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parties {
    /// 部门id列表
    pub partyid: Vec<i64>,
}

/// 可见标签范围
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tags {
    /// 标签id列表
    pub tagid: Vec<i64>,
}

/// 应用列表响应
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AgentListResponse {
    /// 应用列表
    pub agentlist: Vec<AgentInfo>,
}
