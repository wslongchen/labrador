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
use serde_json::{json, Value};
use base64::{Engine as _, engine::general_purpose::STANDARD};

use crate::errors::{LabraError, LabradorResult};
use crate::wechat::client::WechatApiResponse;
use crate::wechat::mp::WechatMpClient;

/// 微信“一物一码”模块
///
/// 提供二维码申请、激活、下载等一物一码相关功能。
#[derive(Debug, Clone)]
pub struct WechatMpOneCode<'a> {
    client: &'a WechatMpClient,
}

#[allow(unused)]
impl<'a> WechatMpOneCode<'a> {
    #[inline]
    pub fn new(client: &'a WechatMpClient) -> Self {
        Self { client }
    }

    /// 申请二维码
    ///
    /// 用于申请生成一批二维码。
    pub async fn apply_code(&self, request: &ApplyCodeRequest) -> LabradorResult<ApplyCodeResponse> {
        let response: WechatApiResponse<ApplyCodeResponse> = self.client.wechat_client()
            .post("/intp/marketcode/applycode", request)
            .await?;
        response.into_result()
    }

    /// 查询二维码申请单
    ///
    /// 查询二维码申请单状态及详细信息。
    pub async fn query_apply(&self, application_id: Option<u64>, isv_application_id: Option<&str>) -> LabradorResult<ApplyQueryResponse> {
        let mut request = serde_json::Map::new();
        if let Some(id) = application_id {
            request.insert("application_id".to_string(), json!(id));
        }
        if let Some(isv_id) = isv_application_id {
            request.insert("isv_application_id".to_string(), json!(isv_id));
        }
        let response: WechatApiResponse<ApplyQueryResponse> = self.client.wechat_client()
            .post("/intp/marketcode/applycodequery", Value::Object(request))
            .await?;
        response.into_result()
    }

    /// 下载二维码包
    ///
    /// 下载生成的二维码数据包。需先对返回的buffer做base64解码，再按文档解密。
    pub async fn download_code_package(&self, application_id: u64, code_start: u64, code_end: u64) -> LabradorResult<Vec<u8>> {
        let request = json!({
            "application_id": application_id,
            "code_start": code_start,
            "code_end": code_end,
        });
        let response: WechatApiResponse<DownloadCodeResponse> = self.client.wechat_client()
            .post("/intp/marketcode/applycodedownload", request)
            .await?;
        let data = response.into_result()?;
        // base64解码
        let decoded = STANDARD.decode(data.buffer)
            .map_err(|e| LabraError::Crypto(format!("base64解码失败: {}", e)))?;
        Ok(decoded)
    }

    /// 激活二维码
    ///
    /// 激活指定范围的二维码用于实际营销活动。
    pub async fn activate_code(&self, request: &ActivateCodeRequest) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self.client.wechat_client()
            .post("/intp/marketcode/codeactive", request)
            .await?;
        Ok(response)
    }

    /// 查询二维码激活状态
    ///
    /// 查询指定范围的二维码的激活状态。
    pub async fn query_activate_status(&self, application_id: u64, code_start: u64, code_end: u64) -> LabradorResult<Vec<CodeActivateStatus>> {
        let request = json!({
            "application_id": application_id,
            "code_start": code_start,
            "code_end": code_end,
        });
        let response: WechatApiResponse<QueryActivateResponse> = self.client.wechat_client()
            .post("/intp/marketcode/codeactivequery", request)
            .await?;
        Ok(response.into_result()?.code_list)
    }

    /// 二维码ticket换code
    ///
    /// 将用户扫码后获得的ticket转换为对应的code。
    pub async fn ticket_to_code(&self, ticket: &str) -> LabradorResult<String> {
        let request = json!({ "ticket": ticket });
        let response: WechatApiResponse<TicketToCodeResponse> = self.client.wechat_client()
            .post("/intp/marketcode/tickettocode", request)
            .await?;
        Ok(response.into_result()?.code)
    }
}

// ============================================================================
// 请求与响应结构体
// ============================================================================

/// 申请二维码请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyCodeRequest {
    /// 外部单号，相同isv_application_id视为同一申请单
    pub isv_application_id: String,
    /// 申请数量
    pub apply_count: u32,
    /// 二维码类型：1-普通二维码，2-小程序码
    pub code_type: i32,
    /// 小程序appid
    pub wxa_appid: String,
    /// 小程序path
    pub wxa_path: Option<String>,
    /// 小程序版本：0-正式版，1-开发版，2-体验版
    pub wxa_type: Option<i32>,
    /// 备注
    pub remark: Option<String>,
}

/// 申请二维码响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyCodeResponse {
    /// 申请单号
    pub application_id: u64,
    /// 外部单号
    pub isv_application_id: String,
}

/// 查询申请单响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyQueryResponse {
    /// 申请单状态：PROCESSING-处理中，FINISH-完成，FAIL-失败
    pub status: String,
    /// 申请单号
    pub application_id: u64,
    /// 外部单号
    pub isv_application_id: String,
    /// 二维码信息列表
    pub code_generate_list: Vec<CodeGenerateRange>,
    /// 创建时间戳
    pub create_time: i64,
    /// 更新时间戳
    pub update_time: i64,
}

/// 二维码生成范围
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeGenerateRange {
    /// 开始位置
    pub code_start: u64,
    /// 结束位置
    pub code_end: u64,
}

/// 下载二维码响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DownloadCodeResponse {
    /// base64编码的文件buffer
    pub buffer: String,
}

/// 激活二维码请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivateCodeRequest {
    /// 申请单号
    pub application_id: u64,
    /// 活动名称
    pub activity_name: String,
    /// 商品品牌
    pub product_brand: String,
    /// 商品标题
    pub product_title: String,
    /// 商品条码
    pub product_code: String,
    /// 小程序的appid
    pub wxa_appid: String,
    /// 小程序的path
    pub wxa_path: String,
    /// 小程序版本：0-正式版，1-开发版，2-体验版
    pub wxa_type: Option<i32>,
    /// 激活码段的起始位
    pub code_start: u64,
    /// 激活码段的结束位
    pub code_end: u64,
}

/// 查询激活状态响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct QueryActivateResponse {
    code_list: Vec<CodeActivateStatus>,
}

/// 二维码激活状态
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeActivateStatus {
    /// 二维码code
    pub code: String,
    /// 激活状态：0-未激活，1-已激活
    pub active_status: i32,
    /// 激活时间戳
    pub active_time: Option<i64>,
}

/// ticket转code响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TicketToCodeResponse {
    /// 二维码code
    pub code: String,
}