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
use crate::wechat::mp::WechatMpClient;

/// OpenAPI管理模块
#[derive(Debug, Clone)]
pub struct WechatMpApiManage<'a> {
    client: &'a WechatMpClient,
}

#[allow(unused)]
impl<'a> WechatMpApiManage<'a> {
    #[inline]
    pub fn new(client: &'a WechatMpClient) -> Self {
        Self { client }
    }

    /// 获取API调用配额
    ///
    /// 本接口用于查询API调用次数和频率限制。
    pub async fn get_api_quota(&self, cgi_path: &str) -> LabradorResult<ApiQuotaResponse> {
        let request = json!({ "cgi_path": cgi_path });
        let response: WechatApiResponse<ApiQuotaResponse> = self.client.wechat_client()
            .post("/cgi-bin/openapi/quota/get", request)
            .await?;
        response.into_result()
    }

    /// 清除API调用配额
    ///
    /// 本接口用于清空API调用次数（仅限2019年之前的旧接口）。
    #[deprecated = "建议使用 clear_quota_by_appsecret"]
    pub async fn clear_quota(&self) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self.client.wechat_client()
            .get("/cgi-bin/clear_quota")
            .await?;
        Ok(response)
    }

    /// 使用AppSecret清除API调用配额
    ///
    /// 本接口用于清空API调用次数（使用AppSecret鉴权）。
    pub async fn clear_quota_by_appsecret(&self, appid: &str, appsecret: &str) -> LabradorResult<WechatApiResponse> {
        let request = json!({
            "appid": appid,
            "appsecret": appsecret
        });
        let response: WechatApiResponse = self.client.wechat_client()
            .post("/cgi-bin/clear_quota/v2", request)
            .await?;
        Ok(response)
    }

    /// 清除API调用配额（新）
    ///
    /// 本接口用于清空指定的API调用次数。
    pub async fn clear_api_quota(&self, cgi_path: &str) -> LabradorResult<WechatApiResponse> {
        let request = json!({ "cgi_path": cgi_path });
        let response: WechatApiResponse = self.client.wechat_client()
            .post("/cgi-bin/openapi/quota/clear", request)
            .await?;
        Ok(response)
    }

    /// 获取RID信息
    ///
    /// 本接口用于根据调用接口返回的rid（request id）查询详细信息。
    pub async fn get_rid_info(&self, rid: &str) -> LabradorResult<RidInfoResponse> {
        let request = json!({ "rid": rid });
        let response: WechatApiResponse<RidInfoResponse> = self.client.wechat_client()
            .post("/cgi-bin/openapi/rid/get", request)
            .await?;
        response.into_result()
    }
}

/// API配额响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiQuotaResponse {
    pub quota: ApiQuota,
    pub rate_limit: ApiRateLimit,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiQuota {
    /// 当天该接口可访问次数
    pub daily_limit: i32,
    /// 当天剩余次数
    pub remaining: i32,
    /// 当天已用次数
    pub used: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiRateLimit {
    /// 每分钟可访问次数
    pub per_minute: i32,
    /// 每分钟剩余次数
    pub remaining: i32,
}

/// RID信息响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RidInfoResponse {
    pub request: RidRequestInfo,
    pub response: RidResponseInfo,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RidRequestInfo {
    pub url: String,
    pub method: String,
    pub headers: Vec<RidHeader>,
    pub body: String,
    pub time: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RidHeader {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RidResponseInfo {
    pub headers: Vec<RidHeader>,
    pub body: String,
    pub status: i32,
    pub time: String,
}
