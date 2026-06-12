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

//! 微信分账模块
//!
//! 提供微信支付分账功能：请求分账、查询分账结果、解冻剩余资金、查询剩余待分金额、
//! 添加分账接收方、删除分账接收方等。

use crate::errors::{LabraError, LabradorResult};
use crate::platforms::wechat::pay::types::WechatPayCommonResponse;
use crate::request::{HttpMethod, Request};
use crate::ApiClient;
use serde::{Deserialize, Serialize};

// ==================== 分账接收方 ====================

/// 分账接收方类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReceiverType {
    /// 商户号
    MerchantId,
    /// 个人openid
    PersonalOpenid,
}

impl ReceiverType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReceiverType::MerchantId => "MERCHANT_ID",
            ReceiverType::PersonalOpenid => "PERSONAL_OPENID",
        }
    }
}

/// 分账关系类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationType {
    /// 服务商
    ServiceProvider,
    /// 门店
    Store,
    /// 员工
    Staff,
    /// 店主
    StoreOwner,
    /// 合作伙伴
    Partner,
    /// 总部
    Headquarter,
    /// 品牌方
    Brand,
    /// 分销商
    Distributor,
    /// 用户
    User,
    /// 供应商
    Supplier,
    /// 自定义
    Custom,
}

impl RelationType {
    pub fn as_str(&self) -> &'static str {
        match self {
            RelationType::ServiceProvider => "SERVICE_PROVIDER",
            RelationType::Store => "STORE",
            RelationType::Staff => "STAFF",
            RelationType::StoreOwner => "STORE_OWNER",
            RelationType::Partner => "PARTNER",
            RelationType::Headquarter => "HEADQUARTER",
            RelationType::Brand => "BRAND",
            RelationType::Distributor => "DISTRIBUTOR",
            RelationType::User => "USER",
            RelationType::Supplier => "SUPPLIER",
            RelationType::Custom => "CUSTOM",
        }
    }
}

/// 分账接收方
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitSharingReceiver {
    /// 分账接收方类型
    #[serde(rename = "type")]
    pub receiver_type: String,
    /// 分账接收方账号
    pub account: String,
    /// 分账金额（分）
    pub amount: i64,
    /// 分账描述
    pub description: String,
    /// 分账接收方名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ProfitSharingReceiver {
    pub fn new(receiver_type: ReceiverType, account: &str, amount: i64, description: &str) -> Self {
        Self {
            receiver_type: receiver_type.as_str().to_string(),
            account: account.to_string(),
            amount,
            description: description.to_string(),
            name: None,
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
}

// ==================== 添加分账接收方 ====================

/// 添加分账接收方请求
#[derive(Debug, Clone, Serialize)]
pub struct AddReceiverRequest {
    /// 应用ID
    pub appid: String,
    /// 分账接收方类型
    #[serde(rename = "type")]
    pub receiver_type: String,
    /// 分账接收方账号
    pub account: String,
    /// 分账接收方名称（个人openid必填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 与分账方的关系类型
    pub relation_type: String,
    /// 自定义关系说明（relation_type=CUSTOM时必填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_relation: Option<String>,
}

impl AddReceiverRequest {
    pub fn new(
        appid: &str,
        receiver_type: ReceiverType,
        account: &str,
        relation_type: RelationType,
    ) -> Self {
        Self {
            appid: appid.to_string(),
            receiver_type: receiver_type.as_str().to_string(),
            account: account.to_string(),
            name: None,
            relation_type: relation_type.as_str().to_string(),
            custom_relation: None,
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn custom_relation(mut self, relation: &str) -> Self {
        self.custom_relation = Some(relation.to_string());
        self
    }
}

/// 添加分账接收方响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddReceiverResponse {
    /// 分账接收方类型
    #[serde(rename = "type")]
    pub receiver_type: String,
    /// 分账接收方账号
    pub account: String,
}

// ==================== 请求分账 ====================

/// 请求分账请求
#[derive(Debug, Clone, Serialize)]
pub struct CreateProfitSharingOrderRequest {
    /// 应用ID
    pub appid: String,
    /// 微信支付订单号（与out_order_no二选一）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// 商户订单号（与transaction_id二选一）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_order_no: Option<String>,
    /// 商户分账单号
    pub out_order_no_profitsharing: String,
    /// 分账接收方列表
    pub receivers: Vec<ProfitSharingReceiver>,
    /// 是否解冻剩余未分账资金
    pub unfreeze_unsplit: bool,
}

impl CreateProfitSharingOrderRequest {
    pub fn new(
        appid: &str,
        out_order_no_profitsharing: &str,
        receivers: Vec<ProfitSharingReceiver>,
        unfreeze_unsplit: bool,
    ) -> Self {
        Self {
            appid: appid.to_string(),
            transaction_id: None,
            out_order_no: None,
            out_order_no_profitsharing: out_order_no_profitsharing.to_string(),
            receivers,
            unfreeze_unsplit,
        }
    }

    pub fn transaction_id(mut self, id: &str) -> Self {
        self.transaction_id = Some(id.to_string());
        self
    }

    pub fn out_order_no(mut self, no: &str) -> Self {
        self.out_order_no = Some(no.to_string());
        self
    }
}

/// 请求分账响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitSharingOrderResponse {
    /// 微信支付订单号
    pub transaction_id: String,
    /// 商户分账单号
    pub out_order_no_profitsharing: String,
    /// 微信分账单号
    pub order_id: String,
    /// 分账单状态：PROCESSING, FINISHED
    pub state: String,
    /// 分账接收方列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receivers: Option<Vec<ProfitSharingReceiver>>,
}

// ==================== 查询分账 ====================

/// 查询分账结果请求
#[derive(Debug, Clone)]
pub struct QueryProfitSharingOrderRequest {
    /// 商户分账单号
    pub out_order_no_profitsharing: String,
    /// 微信支付订单号
    pub transaction_id: String,
}

impl QueryProfitSharingOrderRequest {
    pub fn new(out_order_no_profitsharing: &str, transaction_id: &str) -> Self {
        Self {
            out_order_no_profitsharing: out_order_no_profitsharing.to_string(),
            transaction_id: transaction_id.to_string(),
        }
    }
}

// ==================== 解冻剩余资金 ====================

/// 解冻剩余资金请求
#[derive(Debug, Clone, Serialize)]
pub struct UnfreezeOrderRequest {
    /// 微信支付订单号
    pub transaction_id: String,
    /// 商户分账单号
    pub out_order_no_profitsharing: String,
    /// 分账描述
    pub description: String,
}

impl UnfreezeOrderRequest {
    pub fn new(transaction_id: &str, out_order_no_profitsharing: &str, description: &str) -> Self {
        Self {
            transaction_id: transaction_id.to_string(),
            out_order_no_profitsharing: out_order_no_profitsharing.to_string(),
            description: description.to_string(),
        }
    }
}

// ==================== 查询剩余待分金额 ====================

/// 查询剩余待分金额请求
#[derive(Debug, Clone)]
pub struct QueryUnsplitAmountRequest {
    /// 微信支付订单号
    pub transaction_id: String,
}

impl QueryUnsplitAmountRequest {
    pub fn new(transaction_id: &str) -> Self {
        Self {
            transaction_id: transaction_id.to_string(),
        }
    }
}

/// 查询剩余待分金额响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsplitAmountResponse {
    /// 微信支付订单号
    pub transaction_id: String,
    /// 订单剩余未分金额（分）
    pub unsplit_amount: i64,
}

// ==================== 分账回退 ====================

/// 分账回退请求
#[derive(Debug, Clone, Serialize)]
pub struct ProfitSharingReturnRequest {
    /// 微信分账单号（与out_order_no_profitsharing二选一）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 商户分账单号（与order_id二选一）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_order_no_profitsharing: Option<String>,
    /// 商户回退单号
    pub out_return_no: String,
    /// 回退商户号
    pub return_mchid: String,
    /// 回退金额（分）
    pub amount: i64,
    /// 回退描述
    pub description: String,
}

impl ProfitSharingReturnRequest {
    pub fn new(out_return_no: &str, return_mchid: &str, amount: i64, description: &str) -> Self {
        Self {
            order_id: None,
            out_order_no_profitsharing: None,
            out_return_no: out_return_no.to_string(),
            return_mchid: return_mchid.to_string(),
            amount,
            description: description.to_string(),
        }
    }

    pub fn order_id(mut self, id: &str) -> Self {
        self.order_id = Some(id.to_string());
        self
    }

    pub fn out_order_no_profitsharing(mut self, no: &str) -> Self {
        self.out_order_no_profitsharing = Some(no.to_string());
        self
    }
}

/// 分账回退响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitSharingReturnResponse {
    /// 微信分账单号
    pub order_id: String,
    /// 商户分账单号
    pub out_order_no_profitsharing: String,
    /// 商户回退单号
    pub out_return_no: String,
    /// 微信回退单号
    pub return_id: String,
    /// 回退商户号
    pub return_mchid: String,
    /// 回退金额
    pub amount: i64,
    /// 回退结果：PROCESSING, SUCCESS, FAILED
    pub result: String,
}

// ==================== 分账客户端 ====================

/// 微信分账客户端
pub struct ProfitSharingClient {
    http_client: ApiClient,
}

impl ProfitSharingClient {
    /// 创建分账客户端
    pub fn new(http_client: ApiClient) -> Self {
        Self { http_client }
    }

    /// 添加分账接收方
    ///
    /// 向微信支付平台添加一个分账接收方。接收方可以是商户（MERCHANT_ID）或个人（PERSONAL_OPENID），
    /// 需要指定与分账方的关系类型。添加成功后才能在后续分账请求中向该接收方分配资金。
    ///
    /// # 参数
    /// * `request` - 添加接收方请求，包含应用ID、接收方类型、账号、关系类型等字段。
    ///   当 `relation_type` 为 `CUSTOM` 时需设置 `custom_relation`
    ///
    /// # 返回
    /// 返回 `LabradorResult<AddReceiverResponse>`，成功时包含接收方类型和账号信息
    ///
    /// # 官方文档
    /// <https://pay.weixin.qq.com/docs/merchant/apis/profitsharing/receiver/add-receiver.html>
    pub async fn add_receiver(
        &self,
        request: AddReceiverRequest,
    ) -> LabradorResult<AddReceiverResponse> {
        let http_request = Request::builder()
            .method(HttpMethod::Post)
            .path("/v3/profitsharing/receivers/add")
            .body(request)
            .build();

        let response = self.http_client.request(http_request).await?;
        if let Some(signer) = self.http_client.signer() {
            if !signer.verify_signature(&response)? {
                return Err(LabraError::Sign("响应签名验证失败".to_string()));
            }
        }
        let result = response.json::<WechatPayCommonResponse<AddReceiverResponse>>()?;
        if !result.is_success() {
            return Err(LabraError::business(
                result.code.unwrap_or_default(),
                result.message.unwrap_or_default(),
            ));
        }
        Ok(result.data)
    }

    /// 删除分账接收方
    ///
    /// 从微信支付平台删除一个已添加的分账接收方。删除后将无法再向该接收方发起分账。
    /// 请求参数与添加接收方接口一致。
    ///
    /// # 参数
    /// * `request` - 删除接收方请求，与添加接收方使用相同的 `AddReceiverRequest` 结构
    ///
    /// # 返回
    /// 返回 `LabradorResult<AddReceiverResponse>`，成功时包含被删除的接收方信息
    ///
    /// # 官方文档
    /// <https://pay.weixin.qq.com/docs/merchant/apis/profitsharing/receiver/delete-receiver.html>
    pub async fn delete_receiver(
        &self,
        request: AddReceiverRequest,
    ) -> LabradorResult<AddReceiverResponse> {
        let http_request = Request::builder()
            .method(HttpMethod::Post)
            .path("/v3/profitsharing/receivers/delete")
            .body(request)
            .build();

        let response = self.http_client.request(http_request).await?;
        if let Some(signer) = self.http_client.signer() {
            if !signer.verify_signature(&response)? {
                return Err(LabraError::Sign("响应签名验证失败".to_string()));
            }
        }
        let result = response.json::<WechatPayCommonResponse<AddReceiverResponse>>()?;
        if !result.is_success() {
            return Err(LabraError::business(
                result.code.unwrap_or_default(),
                result.message.unwrap_or_default(),
            ));
        }
        Ok(result.data)
    }

    /// 请求分账
    ///
    /// 对一笔已支付的订单发起分账请求，将资金分配到指定的分账接收方。支持同时向多个接收方分账，
    /// 并可选择是否在分账完成后自动解冻剩余未分账资金。
    ///
    /// # 参数
    /// * `request` - 分账请求，包含应用ID、支付订单号（transaction_id 或 out_order_no）、
    ///   商户分账单号、接收方列表及是否解冻剩余资金标志
    ///
    /// # 返回
    /// 返回 `LabradorResult<ProfitSharingOrderResponse>`，成功时包含微信分账单号及分账状态（PROCESSING/FINISHED）
    ///
    /// # 官方文档
    /// <https://pay.weixin.qq.com/docs/merchant/apis/profitsharing/orders/create-order.html>
    pub async fn create_order(
        &self,
        request: CreateProfitSharingOrderRequest,
    ) -> LabradorResult<ProfitSharingOrderResponse> {
        let http_request = Request::builder()
            .method(HttpMethod::Post)
            .path("/v3/profitsharing/orders")
            .body(request)
            .build();

        let response = self.http_client.request(http_request).await?;
        if let Some(signer) = self.http_client.signer() {
            if !signer.verify_signature(&response)? {
                return Err(LabraError::Sign("响应签名验证失败".to_string()));
            }
        }
        let result = response.json::<WechatPayCommonResponse<ProfitSharingOrderResponse>>()?;
        if !result.is_success() {
            return Err(LabraError::business(
                result.code.unwrap_or_default(),
                result.message.unwrap_or_default(),
            ));
        }
        Ok(result.data)
    }

    /// 查询分账结果
    ///
    /// 查询指定分账单的当前状态和分账详情。通过商户分账单号和微信支付订单号定位分账记录。
    ///
    /// # 参数
    /// * `out_order_no_profitsharing` - 商户分账单号，请求分账时传入的唯一编号
    /// * `transaction_id` - 微信支付订单号，原支付订单的交易号
    ///
    /// # 返回
    /// 返回 `LabradorResult<ProfitSharingOrderResponse>`，成功时包含分账状态及接收方列表
    ///
    /// # 官方文档
    /// <https://pay.weixin.qq.com/docs/merchant/apis/profitsharing/orders/query-order.html>
    pub async fn query_order(
        &self,
        out_order_no_profitsharing: &str,
        transaction_id: &str,
    ) -> LabradorResult<ProfitSharingOrderResponse> {
        let path = format!(
            "/v3/profitsharing/orders/{}?transaction_id={}",
            out_order_no_profitsharing, transaction_id
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
        let result = response.json::<WechatPayCommonResponse<ProfitSharingOrderResponse>>()?;
        if !result.is_success() {
            return Err(LabraError::business(
                result.code.unwrap_or_default(),
                result.message.unwrap_or_default(),
            ));
        }
        Ok(result.data)
    }

    /// 解冻剩余资金
    ///
    /// 解冻分账订单中尚未分配的剩余资金，将剩余金额返还给商户。
    /// 适用于分账时 `unfreeze_unsplit` 设为 `false` 的场景，后续手动解冻。
    ///
    /// # 参数
    /// * `request` - 解冻请求，包含微信支付订单号、商户分账单号及解冻描述
    ///
    /// # 返回
    /// 返回 `LabradorResult<ProfitSharingOrderResponse>`，成功时包含解冻后的分账单状态
    ///
    /// # 官方文档
    /// <https://pay.weixin.qq.com/docs/merchant/apis/profitsharing/orders/unfreeze-order.html>
    pub async fn unfreeze_order(
        &self,
        request: UnfreezeOrderRequest,
    ) -> LabradorResult<ProfitSharingOrderResponse> {
        let http_request = Request::builder()
            .method(HttpMethod::Post)
            .path("/v3/profitsharing/orders/unfreeze")
            .body(request)
            .build();

        let response = self.http_client.request(http_request).await?;
        if let Some(signer) = self.http_client.signer() {
            if !signer.verify_signature(&response)? {
                return Err(LabraError::Sign("响应签名验证失败".to_string()));
            }
        }
        let result = response.json::<WechatPayCommonResponse<ProfitSharingOrderResponse>>()?;
        if !result.is_success() {
            return Err(LabraError::business(
                result.code.unwrap_or_default(),
                result.message.unwrap_or_default(),
            ));
        }
        Ok(result.data)
    }

    /// 查询剩余待分金额
    ///
    /// 查询指定支付订单中尚未分配的剩余可分账金额。商户可根据此金额决定后续分账策略。
    ///
    /// # 参数
    /// * `transaction_id` - 微信支付订单号
    ///
    /// # 返回
    /// 返回 `LabradorResult<UnsplitAmountResponse>`，成功时包含剩余未分金额（单位：分）
    ///
    /// # 官方文档
    /// <https://pay.weixin.qq.com/docs/merchant/apis/profitsharing/orders/query-unsplit-amount.html>
    pub async fn query_unsplit_amount(
        &self,
        transaction_id: &str,
    ) -> LabradorResult<UnsplitAmountResponse> {
        let path = format!("/v3/profitsharing/transactions/{}/amounts", transaction_id);

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
        let result = response.json::<WechatPayCommonResponse<UnsplitAmountResponse>>()?;
        if !result.is_success() {
            return Err(LabraError::business(
                result.code.unwrap_or_default(),
                result.message.unwrap_or_default(),
            ));
        }
        Ok(result.data)
    }

    /// 分账回退
    ///
    /// 对已完成的分账发起回退操作，将已分给接收方的资金退回。支持通过微信分账单号或商户分账单号
    /// 指定回退目标，需提供回退金额和回退描述。
    ///
    /// # 参数
    /// * `request` - 分账回退请求，包含分账单号（order_id 或 out_order_no_profitsharing）、
    ///   商户回退单号、回退商户号、回退金额及描述
    ///
    /// # 返回
    /// 返回 `LabradorResult<ProfitSharingReturnResponse>`，成功时包含回退单号及回退结果（PROCESSING/SUCCESS/FAILED）
    ///
    /// # 官方文档
    /// <https://pay.weixin.qq.com/docs/merchant/apis/profitsharing/return-orders/create-return-order.html>
    pub async fn create_return_order(
        &self,
        request: ProfitSharingReturnRequest,
    ) -> LabradorResult<ProfitSharingReturnResponse> {
        let http_request = Request::builder()
            .method(HttpMethod::Post)
            .path("/v3/profitsharing/return-orders")
            .body(request)
            .build();

        let response = self.http_client.request(http_request).await?;
        if let Some(signer) = self.http_client.signer() {
            if !signer.verify_signature(&response)? {
                return Err(LabraError::Sign("响应签名验证失败".to_string()));
            }
        }
        let result = response.json::<WechatPayCommonResponse<ProfitSharingReturnResponse>>()?;
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
    fn test_receiver_serialization() {
        let receiver =
            ProfitSharingReceiver::new(ReceiverType::MerchantId, "1900000109", 100, "分账给商户");

        let json = serde_json::to_string(&receiver).unwrap();
        assert!(json.contains("MERCHANT_ID"));
        assert!(json.contains("1900000109"));
        assert!(json.contains("100"));
    }

    #[test]
    fn test_create_order_request_serialization() {
        let receivers = vec![ProfitSharingReceiver::new(
            ReceiverType::MerchantId,
            "1900000109",
            100,
            "分账给商户",
        )];
        let request =
            CreateProfitSharingOrderRequest::new("wx123456", "PS_20240101_001", receivers, false)
                .transaction_id("4200001234567890");

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("PS_20240101_001"));
        assert!(json.contains("4200001234567890"));
    }

    #[test]
    fn test_receiver_type_as_str() {
        assert_eq!(ReceiverType::MerchantId.as_str(), "MERCHANT_ID");
        assert_eq!(ReceiverType::PersonalOpenid.as_str(), "PERSONAL_OPENID");
    }

    #[test]
    fn test_relation_type_as_str() {
        assert_eq!(RelationType::ServiceProvider.as_str(), "SERVICE_PROVIDER");
        assert_eq!(RelationType::Store.as_str(), "STORE");
        assert_eq!(RelationType::Partner.as_str(), "PARTNER");
    }
}
