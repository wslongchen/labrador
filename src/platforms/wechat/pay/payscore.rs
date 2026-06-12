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

//! 微信支付分（先享后付）模块
//!
//! 提供微信支付分相关的接口：创建/查询/取消/修改/完结支付分订单、查询用户授权状态、解除授权等。

use crate::errors::{LabraError, LabradorResult};
use crate::platforms::wechat::pay::types::WechatPayCommonResponse;
use crate::request::{HttpMethod, Request};
use crate::ApiClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// ==================== 支付分订单 ====================

/// 支付分风险金
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFund {
    /// 风险金名称
    pub name: String,
    /// 风险金额（分）
    pub amount: i64,
    /// 风险说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl RiskFund {
    /// 创建风险金
    /// name 枚举值：ESTIMATE_ORDER_COST（预估订单费用）
    pub fn new(name: &str, amount: i64) -> Self {
        Self {
            name: name.to_string(),
            amount,
            description: None,
        }
    }
}

/// 支付分时间范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    /// 服务开始时间，rfc3339格式
    pub start_time: String,
    /// 服务结束时间，rfc3339格式（创建订单时可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 服务开始时间备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time_remark: Option<String>,
    /// 服务结束时间备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time_remark: Option<String>,
}

impl TimeRange {
    pub fn new(start_time: &str) -> Self {
        Self {
            start_time: start_time.to_string(),
            end_time: None,
            start_time_remark: None,
            end_time_remark: None,
        }
    }

    pub fn end_time(mut self, end_time: &str) -> Self {
        self.end_time = Some(end_time.to_string());
        self
    }
}

/// 支付分位置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    /// 服务开始地点
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_location: Option<String>,
    /// 服务结束地点
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_location: Option<String>,
}

/// 支付分后付费项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostPayment {
    /// 付费项目名称
    pub name: String,
    /// 付费金额（分）
    pub amount: i64,
    /// 计费说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 付费数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

impl PostPayment {
    pub fn new(name: &str, amount: i64) -> Self {
        Self {
            name: name.to_string(),
            amount,
            description: None,
            count: None,
        }
    }
}

/// 支付分后付费优惠
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostDiscount {
    /// 优惠名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 优惠金额（分）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// 优惠说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 支付分订单创建请求
#[derive(Debug, Clone, Serialize)]
pub struct PayscoreCreateOrderRequest {
    /// 商户服务订单号
    pub out_order_no: String,
    /// 应用ID
    pub appid: String,
    /// 服务ID
    pub service_id: String,
    /// 服务介绍
    pub service_introduction: String,
    /// 风险金
    pub risk_fund: RiskFund,
    /// 服务时间范围
    pub time_range: TimeRange,
    /// 是否需要用户确认
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_user_confirm: Option<bool>,
    /// 商户回调地址
    pub notify_url: String,
    /// 附加数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
    /// 后付费项目（可选，完结时需要）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_payments: Option<Vec<PostPayment>>,
    /// 后付费优惠（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_discounts: Option<Vec<PostDiscount>>,
    /// 总金额（可选，创建时可不传）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<i64>,
    /// 位置信息（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

impl PayscoreCreateOrderRequest {
    pub fn new(
        out_order_no: &str,
        appid: &str,
        service_id: &str,
        service_introduction: &str,
        risk_fund: RiskFund,
        time_range: TimeRange,
        notify_url: &str,
    ) -> Self {
        Self {
            out_order_no: out_order_no.to_string(),
            appid: appid.to_string(),
            service_id: service_id.to_string(),
            service_introduction: service_introduction.to_string(),
            risk_fund,
            time_range,
            need_user_confirm: None,
            notify_url: notify_url.to_string(),
            attach: None,
            post_payments: None,
            post_discounts: None,
            total_amount: None,
            location: None,
        }
    }

    pub fn need_user_confirm(mut self, confirm: bool) -> Self {
        self.need_user_confirm = Some(confirm);
        self
    }

    pub fn attach(mut self, attach: &str) -> Self {
        self.attach = Some(attach.to_string());
        self
    }

    pub fn post_payments(mut self, payments: Vec<PostPayment>) -> Self {
        self.post_payments = Some(payments);
        self
    }

    pub fn post_discounts(mut self, discounts: Vec<PostDiscount>) -> Self {
        self.post_discounts = Some(discounts);
        self
    }

    pub fn total_amount(mut self, amount: i64) -> Self {
        self.total_amount = Some(amount);
        self
    }
}

/// 支付分订单查询请求
#[derive(Debug, Clone)]
pub struct PayscoreQueryOrderRequest {
    /// 商户服务订单号
    pub out_order_no: Option<String>,
    /// 微信支付服务订单号
    pub order_id: Option<String>,
}

impl PayscoreQueryOrderRequest {
    pub fn by_out_order_no(no: &str) -> Self {
        Self {
            out_order_no: Some(no.to_string()),
            order_id: None,
        }
    }

    pub fn by_order_id(id: &str) -> Self {
        Self {
            out_order_no: None,
            order_id: Some(id.to_string()),
        }
    }
}

/// 支付分取消订单请求
#[derive(Debug, Clone, Serialize)]
pub struct PayscoreCancelOrderRequest {
    /// 商户服务订单号
    pub out_order_no: String,
    /// 服务ID
    pub service_id: String,
    /// 取消原因
    pub reason: String,
}

impl PayscoreCancelOrderRequest {
    pub fn new(out_order_no: &str, service_id: &str, reason: &str) -> Self {
        Self {
            out_order_no: out_order_no.to_string(),
            service_id: service_id.to_string(),
            reason: reason.to_string(),
        }
    }
}

/// 支付分修改订单请求
#[derive(Debug, Clone, Serialize)]
pub struct PayscoreModifyOrderRequest {
    /// 商户服务订单号
    pub out_order_no: String,
    /// 服务ID
    pub service_id: String,
    /// 后付费项目
    pub post_payments: Vec<PostPayment>,
    /// 后付费优惠
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_discounts: Option<Vec<PostDiscount>>,
    /// 总金额
    pub total_amount: i64,
    /// 修改原因
    pub reason: String,
}

impl PayscoreModifyOrderRequest {
    pub fn new(
        out_order_no: &str,
        service_id: &str,
        post_payments: Vec<PostPayment>,
        total_amount: i64,
        reason: &str,
    ) -> Self {
        Self {
            out_order_no: out_order_no.to_string(),
            service_id: service_id.to_string(),
            post_payments,
            post_discounts: None,
            total_amount,
            reason: reason.to_string(),
        }
    }
}

/// 支付分完结订单请求
#[derive(Debug, Clone, Serialize)]
pub struct PayscoreCompleteOrderRequest {
    /// 商户服务订单号
    pub out_order_no: String,
    /// 服务ID
    pub service_id: String,
    /// 后付费项目
    pub post_payments: Vec<PostPayment>,
    /// 后付费优惠
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_discounts: Option<Vec<PostDiscount>>,
    /// 总金额
    pub total_amount: i64,
}

impl PayscoreCompleteOrderRequest {
    pub fn new(
        out_order_no: &str,
        service_id: &str,
        post_payments: Vec<PostPayment>,
        total_amount: i64,
    ) -> Self {
        Self {
            out_order_no: out_order_no.to_string(),
            service_id: service_id.to_string(),
            post_payments,
            post_discounts: None,
            total_amount,
        }
    }
}

/// 支付分订单状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayscoreOrderResponse {
    /// 应用ID
    pub appid: String,
    /// 商户号
    pub mchid: String,
    /// 商户服务订单号
    pub out_order_no: String,
    /// 服务ID
    pub service_id: String,
    /// 服务介绍
    pub service_introduction: String,
    /// 订单状态：CREATED, DOING, DONE, REVOKED, EXPIRED
    pub state: String,
    /// 订单状态说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_description: Option<String>,
    /// 风险金
    pub risk_fund: RiskFund,
    /// 时间范围
    pub time_range: TimeRange,
    /// 是否需要用户确认
    #[serde(default)]
    pub need_user_confirm: bool,
    /// 后付费项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_payments: Option<Vec<PostPayment>>,
    /// 后付费优惠
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_discounts: Option<Vec<PostDiscount>>,
    /// 总金额
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<i64>,
    /// 附加数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
    /// 商户回调地址
    pub notify_url: String,
    /// 订单创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 微信支付服务订单号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openid: Option<String>,
}

// ==================== 支付分授权 ====================

/// 查询用户授权状态请求
#[derive(Debug, Clone)]
pub struct PayscoreQueryAuthRequest {
    /// 服务ID
    pub service_id: String,
    /// 应用ID
    pub appid: String,
    /// 用户标识
    pub openid: String,
}

impl PayscoreQueryAuthRequest {
    pub fn new(service_id: &str, appid: &str, openid: &str) -> Self {
        Self {
            service_id: service_id.to_string(),
            appid: appid.to_string(),
            openid: openid.to_string(),
        }
    }
}

/// 用户授权状态响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayscoreAuthResponse {
    /// 应用ID
    pub appid: String,
    /// 商户号
    pub mchid: String,
    /// 服务ID
    pub service_id: String,
    /// 用户标识
    pub openid: String,
    /// 授权状态：AVAILABLE, UNAVAILABLE
    pub authorization_state: String,
    /// 授权协议号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<String>,
    /// 取消授权时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_authorization_time: Option<String>,
}

/// 解除用户授权请求
#[derive(Debug, Clone, Serialize)]
pub struct PayscoreTerminateAuthRequest {
    /// 服务ID
    pub service_id: String,
    /// 应用ID
    pub appid: String,
    /// 解除授权原因
    pub reason: String,
}

impl PayscoreTerminateAuthRequest {
    pub fn new(service_id: &str, appid: &str, reason: &str) -> Self {
        Self {
            service_id: service_id.to_string(),
            appid: appid.to_string(),
            reason: reason.to_string(),
        }
    }
}

// ==================== 支付分客户端 ====================

/// 微信支付分客户端
pub struct PayscoreClient {
    http_client: ApiClient,
}

impl PayscoreClient {
    /// 创建支付分客户端
    pub fn new(http_client: ApiClient) -> Self {
        Self { http_client }
    }

    /// 创建支付分订单
    ///
    /// 创建微信支付分服务订单。用户需先在商户侧下单并授权，商户通过此接口创建支付分订单，
    /// 传入服务时间、风险金、后付费项目等信息。创建成功后返回订单状态。
    ///
    /// # 参数
    /// * `request` - 支付分创建订单请求，包含商户订单号、服务介绍、风险金、时间范围、回调地址等字段
    ///
    /// # 返回
    /// 返回 `LabradorResult<PayscoreOrderResponse>`，成功时包含订单状态（CREATED/DOING/DONE 等）及详情
    ///
    /// # 官方文档
    /// <https://pay.weixin.qq.com/docs/merchant/apis/payscore/service-order/create-service-order.html>
    pub async fn create_order(
        &self,
        request: PayscoreCreateOrderRequest,
    ) -> LabradorResult<PayscoreOrderResponse> {
        let http_request = Request::builder()
            .method(HttpMethod::Post)
            .path("/v3/payscore/serviceorder")
            .body(request)
            .build();

        let response = self.http_client.request(http_request).await?;
        if let Some(signer) = self.http_client.signer() {
            if !signer.verify_signature(&response)? {
                return Err(LabraError::Sign("响应签名验证失败".to_string()));
            }
        }
        let result = response.json::<WechatPayCommonResponse<PayscoreOrderResponse>>()?;
        if !result.is_success() {
            return Err(LabraError::business(
                result.code.unwrap_or_default(),
                result.message.unwrap_or_default(),
            ));
        }
        Ok(result.data)
    }

    /// 查询支付分订单
    ///
    /// 查询指定支付分服务订单的当前状态和详细信息。支持通过商户订单号 (`out_order_no`)
    /// 或微信支付服务订单号 (`order_id`) 进行查询，二者至少提供一个。
    ///
    /// # 参数
    /// * `request` - 查询请求，可通过 `PayscoreQueryOrderRequest::by_out_order_no()` 或
    ///   `by_order_id()` 构造
    ///
    /// # 返回
    /// 返回 `LabradorResult<PayscoreOrderResponse>`，成功时包含订单完整状态信息
    ///
    /// # 官方文档
    /// <https://pay.weixin.qq.com/docs/merchant/apis/payscore/service-order/get-service-order.html>
    pub async fn query_order(
        &self,
        request: &PayscoreQueryOrderRequest,
    ) -> LabradorResult<PayscoreOrderResponse> {
        let path = if let Some(no) = &request.out_order_no {
            format!("/v3/payscore/serviceorder?out_order_no={}", no)
        } else if let Some(id) = &request.order_id {
            format!("/v3/payscore/serviceorder?order_id={}", id)
        } else {
            return Err(LabraError::Validation(
                "out_order_no和order_id至少需要一个".to_string(),
            ));
        };

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
        let result = response.json::<WechatPayCommonResponse<PayscoreOrderResponse>>()?;
        if !result.is_success() {
            return Err(LabraError::business(
                result.code.unwrap_or_default(),
                result.message.unwrap_or_default(),
            ));
        }
        Ok(result.data)
    }

    /// 取消支付分订单
    ///
    /// 取消已创建的支付分服务订单。只有状态为 `CREATED` 或 `DOING` 的订单可被取消，
    /// 取消后订单状态变为 `REVOKED`。需要提供取消原因。
    ///
    /// # 参数
    /// * `out_order_no` - 商户服务订单号，创建订单时传入的唯一编号
    /// * `service_id` - 支付分服务ID
    /// * `reason` - 取消原因说明
    ///
    /// # 返回
    /// 返回 `LabradorResult<()>`，成功时返回空元组
    ///
    /// # 官方文档
    /// <https://pay.weixin.qq.com/docs/merchant/apis/payscore/service-order/cancel-service-order.html>
    pub async fn cancel_order(
        &self,
        out_order_no: &str,
        service_id: &str,
        reason: &str,
    ) -> LabradorResult<()> {
        let path = format!("/v3/payscore/serviceorder/{}/cancel", out_order_no);
        let http_request = Request::builder()
            .method(HttpMethod::Post)
            .path(path)
            .body(serde_json::json!({
                "service_id": service_id,
                "reason": reason,
            }))
            .build();

        let response = self.http_client.request(http_request).await?;
        if let Some(signer) = self.http_client.signer() {
            if !signer.verify_signature(&response)? {
                return Err(LabraError::Sign("响应签名验证失败".to_string()));
            }
        }
        if !response.is_success() {
            let result = response.json::<WechatPayCommonResponse<Value>>()?;
            return Err(LabraError::business(
                result.code.unwrap_or_default(),
                result.message.unwrap_or_default(),
            ));
        }
        Ok(())
    }

    /// 修改支付分订单金额
    ///
    /// 修改支付分订单的后付费项目及总金额。在服务完成后、完结订单前，
    /// 若实际费用与创建时预估的不同，可调用此接口调整后付费项目和金额。
    ///
    /// # 参数
    /// * `request` - 修改订单请求，包含商户订单号、服务ID、新的后付费项目列表、总金额及修改原因
    ///
    /// # 返回
    /// 返回 `LabradorResult<()>`，成功时返回空元组
    ///
    /// # 官方文档
    /// <https://pay.weixin.qq.com/docs/merchant/apis/payscore/service-order/modify-service-order.html>
    pub async fn modify_order(&self, request: PayscoreModifyOrderRequest) -> LabradorResult<()> {
        let path = format!("/v3/payscore/serviceorder/{}/modify", request.out_order_no);
        let http_request = Request::builder()
            .method(HttpMethod::Post)
            .path(path)
            .body(request)
            .build();

        let response = self.http_client.request(http_request).await?;
        if let Some(signer) = self.http_client.signer() {
            if !signer.verify_signature(&response)? {
                return Err(LabraError::Sign("响应签名验证失败".to_string()));
            }
        }
        if !response.is_success() {
            let result = response.json::<WechatPayCommonResponse<Value>>()?;
            return Err(LabraError::business(
                result.code.unwrap_or_default(),
                result.message.unwrap_or_default(),
            ));
        }
        Ok(())
    }

    /// 完结支付分订单
    ///
    /// 完结支付分服务订单，触发用户扣款。订单完结后状态变为 `DONE`，
    /// 微信支付将按后付费项目金额从用户账户中扣款。完结前需确保后付费项目已确认。
    ///
    /// # 参数
    /// * `request` - 完结订单请求，包含商户订单号、服务ID、后付费项目列表及总金额
    ///
    /// # 返回
    /// 返回 `LabradorResult<()>`，成功时返回空元组
    ///
    /// # 官方文档
    /// <https://pay.weixin.qq.com/docs/merchant/apis/payscore/service-order/complete-service-order.html>
    pub async fn complete_order(
        &self,
        request: PayscoreCompleteOrderRequest,
    ) -> LabradorResult<()> {
        let path = format!(
            "/v3/payscore/serviceorder/{}/complete",
            request.out_order_no
        );
        let http_request = Request::builder()
            .method(HttpMethod::Post)
            .path(path)
            .body(request)
            .build();

        let response = self.http_client.request(http_request).await?;
        if let Some(signer) = self.http_client.signer() {
            if !signer.verify_signature(&response)? {
                return Err(LabraError::Sign("响应签名验证失败".to_string()));
            }
        }
        if !response.is_success() {
            let result = response.json::<WechatPayCommonResponse<Value>>()?;
            return Err(LabraError::business(
                result.code.unwrap_or_default(),
                result.message.unwrap_or_default(),
            ));
        }
        Ok(())
    }

    /// 查询用户授权状态
    ///
    /// 查询用户对指定支付分服务的授权状态。返回授权状态（AVAILABLE/UNAVAILABLE）
    /// 及授权协议号等信息。
    ///
    /// # 参数
    /// * `openid` - 用户的微信 openid
    /// * `service_id` - 支付分服务ID
    /// * `appid` - 商户应用ID
    ///
    /// # 返回
    /// 返回 `LabradorResult<PayscoreAuthResponse>`，成功时包含授权状态（AVAILABLE/UNAVAILABLE）及授权码
    ///
    /// # 官方文档
    /// <https://pay.weixin.qq.com/docs/merchant/apis/payscore/user-authorization/get-user-authorization.html>
    pub async fn query_user_authorization(
        &self,
        openid: &str,
        service_id: &str,
        appid: &str,
    ) -> LabradorResult<PayscoreAuthResponse> {
        let path = format!(
            "/v3/payscore/permissions/openid/{}?service_id={}&appid={}",
            openid, service_id, appid
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
        let result = response.json::<WechatPayCommonResponse<PayscoreAuthResponse>>()?;
        if !result.is_success() {
            return Err(LabraError::business(
                result.code.unwrap_or_default(),
                result.message.unwrap_or_default(),
            ));
        }
        Ok(result.data)
    }

    /// 解除用户授权
    ///
    /// 解除用户对指定支付分服务的授权。解除后用户需重新授权才能使用该服务。
    ///
    /// # 参数
    /// * `openid` - 用户的微信 openid
    /// * `service_id` - 支付分服务ID
    /// * `appid` - 商户应用ID
    /// * `reason` - 解除授权原因说明
    ///
    /// # 返回
    /// 返回 `LabradorResult<()>`，成功时返回空元组
    ///
    /// # 官方文档
    /// <https://pay.weixin.qq.com/docs/merchant/apis/payscore/user-authorization/terminate-user-authorization.html>
    pub async fn terminate_user_authorization(
        &self,
        openid: &str,
        service_id: &str,
        appid: &str,
        reason: &str,
    ) -> LabradorResult<()> {
        let path = format!("/v3/payscore/permissions/openid/{}/terminate", openid);
        let http_request = Request::builder()
            .method(HttpMethod::Post)
            .path(path)
            .body(serde_json::json!({
                "service_id": service_id,
                "appid": appid,
                "reason": reason,
            }))
            .build();

        let response = self.http_client.request(http_request).await?;
        if let Some(signer) = self.http_client.signer() {
            if !signer.verify_signature(&response)? {
                return Err(LabraError::Sign("响应签名验证失败".to_string()));
            }
        }
        if !response.is_success() {
            let result = response.json::<WechatPayCommonResponse<Value>>()?;
            return Err(LabraError::business(
                result.code.unwrap_or_default(),
                result.message.unwrap_or_default(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_order_serialization() {
        let request = PayscoreCreateOrderRequest::new(
            "ORDER_001",
            "wx123456",
            "2002000000001558129851361901536",
            "租借充电宝",
            RiskFund::new("ESTIMATE_ORDER_COST", 10000),
            TimeRange::new("2024-01-01T10:00:00+08:00"),
            "https://example.com/notify",
        )
        .need_user_confirm(true);

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("ORDER_001"));
        assert!(json.contains("ESTIMATE_ORDER_COST"));
        assert!(json.contains("租借充电宝"));
    }

    #[test]
    fn test_risk_fund_creation() {
        let fund = RiskFund::new("ESTIMATE_ORDER_COST", 10000);
        assert_eq!(fund.name, "ESTIMATE_ORDER_COST");
        assert_eq!(fund.amount, 10000);
    }

    #[test]
    fn test_time_range_creation() {
        let range =
            TimeRange::new("2024-01-01T10:00:00+08:00").end_time("2024-01-02T10:00:00+08:00");
        assert_eq!(range.start_time, "2024-01-01T10:00:00+08:00");
        assert_eq!(
            range.end_time,
            Some("2024-01-02T10:00:00+08:00".to_string())
        );
    }

    #[test]
    fn test_complete_order_serialization() {
        let payments = vec![PostPayment::new("充电宝租金", 5000)];
        let request = PayscoreCompleteOrderRequest::new(
            "ORDER_001",
            "2002000000001558129851361901536",
            payments,
            5000,
        );

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("ORDER_001"));
        assert!(json.contains("充电宝租金"));
    }
}
