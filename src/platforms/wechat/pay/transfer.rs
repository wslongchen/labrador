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

//! 微信商家转账到零钱模块
//!
//! 提供商家向用户零钱转账的能力，包括发起批量转账、查询转账批次、查询转账明细等。

use crate::errors::{LabraError, LabradorResult};
use crate::platforms::wechat::pay::types::WechatPayCommonResponse;
use crate::request::{HttpMethod, Request};
use crate::ApiClient;
use serde::{Deserialize, Serialize};

// ==================== 转账请求类型 ====================

/// 转账明细
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferDetail {
    /// 商家明细单号
    pub out_detail_no: String,
    /// 转账金额（分）
    pub transfer_amount: i64,
    /// 转账备注
    pub transfer_remark: String,
    /// 用户在直连商户应用下的唯一标识
    pub openid: String,
    /// 用户姓名（需加密）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl TransferDetail {
    pub fn new(
        out_detail_no: &str,
        transfer_amount: i64,
        transfer_remark: &str,
        openid: &str,
    ) -> Self {
        Self {
            out_detail_no: out_detail_no.to_string(),
            transfer_amount,
            transfer_remark: transfer_remark.to_string(),
            openid: openid.to_string(),
            user_name: None,
        }
    }

    pub fn user_name(mut self, name: &str) -> Self {
        self.user_name = Some(name.to_string());
        self
    }
}

/// 批量转账请求
#[derive(Debug, Clone, Serialize)]
pub struct TransferBatchesRequest {
    /// 应用ID
    pub appid: String,
    /// 商家批次单号
    pub out_batch_no: String,
    /// 批次名称
    pub batch_name: String,
    /// 批次备注
    pub batch_remark: String,
    /// 转账总金额（分）
    pub total_amount: i64,
    /// 转账总笔数
    pub total_num: i32,
    /// 转账明细列表（最多1000笔）
    pub transfer_detail_list: Vec<TransferDetail>,
    /// 转账场景ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_scene_id: Option<String>,
    /// 回调通知地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_url: Option<String>,
}

impl TransferBatchesRequest {
    pub fn new(
        appid: &str,
        out_batch_no: &str,
        batch_name: &str,
        batch_remark: &str,
        total_amount: i64,
        total_num: i32,
        transfer_detail_list: Vec<TransferDetail>,
    ) -> Self {
        Self {
            appid: appid.to_string(),
            out_batch_no: out_batch_no.to_string(),
            batch_name: batch_name.to_string(),
            batch_remark: batch_remark.to_string(),
            total_amount,
            total_num,
            transfer_detail_list,
            transfer_scene_id: None,
            notify_url: None,
        }
    }

    pub fn transfer_scene_id(mut self, scene_id: &str) -> Self {
        self.transfer_scene_id = Some(scene_id.to_string());
        self
    }

    pub fn notify_url(mut self, url: &str) -> Self {
        self.notify_url = Some(url.to_string());
        self
    }
}

// ==================== 转账响应类型 ====================

/// 批量转账响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferBatchesResponse {
    /// 商家批次单号
    pub out_batch_no: String,
    /// 微信支付批次单号
    pub batch_id: String,
    /// 批次创建时间
    pub create_time: String,
    /// 批次状态：ACCEPTED, PROCESSING, FINISHED, CLOSED
    pub batch_status: String,
    /// 批次名称
    pub batch_name: String,
    /// 批次备注
    pub batch_remark: String,
    /// 转账总金额
    pub total_amount: i64,
    /// 转账总笔数
    pub total_num: i32,
    /// 批次关闭原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_reason: Option<String>,
}

/// 查询批量转账请求
#[derive(Debug, Clone)]
pub struct TransferBatchQueryRequest {
    /// 商家批次单号
    pub out_batch_no: String,
    /// 是否查询转账明细
    pub need_query_detail: bool,
    /// 分页起始位置
    pub offset: Option<i32>,
    /// 分页大小（最大100）
    pub limit: Option<i32>,
}

impl TransferBatchQueryRequest {
    pub fn new(out_batch_no: &str, need_query_detail: bool) -> Self {
        Self {
            out_batch_no: out_batch_no.to_string(),
            need_query_detail,
            offset: None,
            limit: None,
        }
    }

    pub fn offset(mut self, offset: i32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// 批量转账查询响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferBatchQueryResponse {
    /// 转账批次单
    pub transfer_batch: TransferBatchesResponse,
    /// 转账明细列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_detail_list: Option<Vec<TransferDetailResponse>>,
}

/// 转账明细响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferDetailResponse {
    /// 明细单号
    pub detail_id: String,
    /// 商家明细单号
    pub out_detail_no: String,
    /// 明细状态：PROCESSING, SUCCESS, FAIL
    pub detail_status: String,
}

// ==================== 转账客户端 ====================

/// 商家转账客户端
pub struct TransferClient {
    http_client: ApiClient,
}

impl TransferClient {
    /// 创建转账客户端
    pub fn new(http_client: ApiClient) -> Self {
        Self { http_client }
    }

    /// 发起批量转账
    ///
    /// 发起一笔批量转账到零钱的操作。支持单批次最多 1000 笔转账明细，
    /// 每笔明细需指定收款人 openid、转账金额和备注。
    ///
    /// # 参数
    /// * `request` - 批量转账请求，包含应用ID、商家批次单号、批次名称、总金额、总笔数及转账明细列表。
    ///   可通过 `transfer_scene_id` 指定转账场景，通过 `notify_url` 设置回调地址
    ///
    /// # 返回
    /// 返回 `LabradorResult<TransferBatchesResponse>`，成功时包含微信支付批次单号及批次状态
    ///   （ACCEPTED/PROCESSING/FINISHED/CLOSED）
    ///
    /// # 官方文档
    /// <https://pay.weixin.qq.com/docs/merchant/apis/batch-transfer-to-balance/transfer-batch/initiate-batch-transfer.html>
    pub async fn create_batch(
        &self,
        request: TransferBatchesRequest,
    ) -> LabradorResult<TransferBatchesResponse> {
        let http_request = Request::builder()
            .method(HttpMethod::Post)
            .path("/v3/transfer/batches")
            .body(request)
            .build();

        let response = self.http_client.request(http_request).await?;
        if let Some(signer) = self.http_client.signer() {
            if !signer.verify_signature(&response)? {
                return Err(LabraError::Sign("响应签名验证失败".to_string()));
            }
        }
        let result = response.json::<WechatPayCommonResponse<TransferBatchesResponse>>()?;
        if !result.is_success() {
            return Err(LabraError::business(
                result.code.unwrap_or_default(),
                result.message.unwrap_or_default(),
            ));
        }
        Ok(result.data)
    }

    /// 查询批量转账
    ///
    /// 通过商家批次单号查询批量转账的当前状态。可选择是否同时查询转账明细列表，
    /// 支持分页查询明细。
    ///
    /// # 参数
    /// * `out_batch_no` - 商家批次单号，发起转账时传入的唯一编号
    /// * `need_query_detail` - 是否同时返回转账明细列表
    /// * `offset` - 分页起始位置（可选，从 0 开始）
    /// * `limit` - 分页大小（可选，最大 100）
    ///
    /// # 返回
    /// 返回 `LabradorResult<TransferBatchQueryResponse>`，成功时包含批次信息和可选的明细列表
    ///
    /// # 官方文档
    /// <https://pay.weixin.qq.com/docs/merchant/apis/batch-transfer-to-balance/transfer-batch/get-transfer-batch-by-out-no.html>
    pub async fn query_batch(
        &self,
        out_batch_no: &str,
        need_query_detail: bool,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> LabradorResult<TransferBatchQueryResponse> {
        let mut path = format!(
            "/v3/transfer/batches/out-batch-no/{}?need_query_detail={}",
            out_batch_no, need_query_detail
        );
        if let Some(o) = offset {
            path = format!("{}&offset={}", path, o);
        }
        if let Some(l) = limit {
            path = format!("{}&limit={}", path, l);
        }

        let http_request = Request::builder()
            .method(HttpMethod::Get)
            .path(path)
            .build();

        let response = self.http_client.request(http_request).await?;
        if let Some(signer) = self.http_client.signer() {
            if !signer.verify_signature(&response)? {
                return Err(LabraError::Sign("响应签名验证失败".to_string()));
            }
        }
        let result = response.json::<WechatPayCommonResponse<TransferBatchQueryResponse>>()?;
        if !result.is_success() {
            return Err(LabraError::business(
                result.code.unwrap_or_default(),
                result.message.unwrap_or_default(),
            ));
        }
        Ok(result.data)
    }

    /// 查询转账明细
    ///
    /// 通过商家批次单号和商家明细单号查询单笔转账明细的执行状态。
    ///
    /// # 参数
    /// * `out_batch_no` - 商家批次单号
    /// * `out_detail_no` - 商家明细单号，发起转账时每笔明细的唯一编号
    ///
    /// # 返回
    /// 返回 `LabradorResult<TransferDetailResponse>`，成功时包含明细状态（PROCESSING/SUCCESS/FAIL）
    ///
    /// # 官方文档
    /// <https://pay.weixin.qq.com/docs/merchant/apis/batch-transfer-to-balance/transfer-detail/get-transfer-detail-by-out-no.html>
    pub async fn query_detail(
        &self,
        out_batch_no: &str,
        out_detail_no: &str,
    ) -> LabradorResult<TransferDetailResponse> {
        let path = format!(
            "/v3/transfer/batches/out-batch-no/{}/details/out-detail-no/{}",
            out_batch_no, out_detail_no
        );

        let http_request = Request::builder()
            .method(HttpMethod::Get)
            .path(path)
            .build();

        let response = self.http_client.request(http_request).await?;
        if let Some(signer) = self.http_client.signer() {
            if !signer.verify_signature(&response)? {
                return Err(LabraError::Sign("响应签名验证失败".to_string()));
            }
        }
        let result = response.json::<WechatPayCommonResponse<TransferDetailResponse>>()?;
        if !result.is_success() {
            return Err(LabraError::business(
                result.code.unwrap_or_default(),
                result.message.unwrap_or_default(),
            ));
        }
        Ok(result.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_detail_serialization() {
        let detail = TransferDetail::new(
            "DETAIL_001",
            100,
            "测试转账",
            "oUpF8uMuAJO_M2pxb1Q9zNjWeS6o",
        );

        let json = serde_json::to_string(&detail).unwrap();
        assert!(json.contains("DETAIL_001"));
        assert!(json.contains("100"));
    }

    #[test]
    fn test_transfer_batch_request_serialization() {
        let details = vec![
            TransferDetail::new("DETAIL_001", 100, "测试1", "openid1"),
            TransferDetail::new("DETAIL_002", 200, "测试2", "openid2"),
        ];
        let request = TransferBatchesRequest::new(
            "wx123456",
            "BATCH_001",
            "测试批次",
            "测试批次备注",
            300,
            2,
            details,
        );

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("BATCH_001"));
        assert!(json.contains("DETAIL_001"));
        assert!(json.contains("DETAIL_002"));
    }
}
