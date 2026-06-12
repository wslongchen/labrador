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
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::LabradorResult;
use crate::wechat::client::WechatApiResponse;
use crate::wechat::miniapp::WechatMiniAppClient;

/// 即时配送模块
#[derive(Debug, Clone)]
pub struct WechatMxaImmediateDelivery<'a> {
    client: &'a WechatMiniAppClient,
}

#[allow(unused)]
impl<'a> WechatMxaImmediateDelivery<'a> {
    #[inline]
    pub fn new(client: &'a WechatMiniAppClient) -> Self {
        Self { client }
    }

    // --- 小程序使用接口 ---

    /// 获取已支持的配送公司列表
    ///
    /// 本接口用于获取当前城市已支持的即时配送公司列表。
    ///
    /// # 返回
    /// 返回 `LabradorResult<Vec<DeliveryCompany>>`，包含配送公司ID和名称的列表。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/immediate-delivery/delivery-by-business/getAllImmeDelivery.html>
    pub async fn get_all_delivery_company(&self) -> LabradorResult<Vec<DeliveryCompany>> {
        let response: WechatApiResponse<GetAllDeliveryCompanyResponse> = self
            .client
            .wechat_client()
            .get("/cgi-bin/express/local/business/delivery/getall")
            .await?;
        response.into_result().map(|r| r.list)
    }

    /// 预下配送单
    ///
    /// 本接口用于在正式下单前进行预下单，用于查询运费、预计送达时间等信息。
    /// 预下单不产生实际配送订单。
    ///
    /// # 参数
    /// * `request` - 预下单请求参数，包含配送公司ID（delivery_id）、门店编号（shop_no）、
    ///   发货人（sender）、收货人（receiver）、货物信息（cargo）、订单信息（order_info）等
    ///
    /// # 返回
    /// 返回 `LabradorResult<PreAddOrderResponse>`，包含运费（fee）、预计送达时间
    /// （expected_delivery_time）、距离（distance）等信息。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/immediate-delivery/delivery-by-business/preAddOrder.html>
    pub async fn pre_add_order(
        &self,
        request: &PreAddOrderRequest,
    ) -> LabradorResult<PreAddOrderResponse> {
        let response: WechatApiResponse<PreAddOrderResponse> = self
            .client
            .wechat_client()
            .post("/cgi-bin/express/local/business/order/pre_add", request)
            .await?;
        response.into_result()
    }

    /// 拉取已绑定门店
    ///
    /// 本接口用于获取商家在配送公司侧已绑定的门店信息列表。
    ///
    /// # 返回
    /// 返回 `LabradorResult<BoundShopInfo>`，包含已绑定门店的列表信息。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/immediate-delivery/delivery-by-business/getBindShop.html>
    pub async fn get_bound_shop(&self) -> LabradorResult<BoundShopInfo> {
        let response: WechatApiResponse<BoundShopInfo> = self
            .client
            .wechat_client()
            .get("/cgi-bin/express/local/business/shop/get")
            .await?;
        response.into_result()
    }

    /// 预取消配送单
    ///
    /// 本接口用于在正式取消前预查询取消配送单的费用信息。
    ///
    /// # 参数
    /// * `request` - 预取消请求参数，包含门店编号（shop_no）、订单ID（order_id）、
    ///   运单号（waybill_id）、取消原因ID（cancel_reason_id）和取消原因（cancel_reason）
    ///
    /// # 返回
    /// 返回 `LabradorResult<PreCancelOrderResponse>`，包含扣除费用（deduct_fee）和描述信息。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/immediate-delivery/delivery-by-business/preCancelOrder.html>
    pub async fn pre_cancel_order(
        &self,
        request: &PreCancelOrderRequest,
    ) -> LabradorResult<PreCancelOrderResponse> {
        let response: WechatApiResponse<PreCancelOrderResponse> = self
            .client
            .wechat_client()
            .post("/cgi-bin/express/local/business/order/precancel", request)
            .await?;
        response.into_result()
    }

    /// 申请开通即时配送
    ///
    /// 本接口用于申请开通小程序的即时配送服务能力。
    ///
    /// # 返回
    /// 返回 `LabradorResult<WechatApiResponse>`，成功时返回空响应。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/immediate-delivery/delivery-by-business/openImmediateDelivery.html>
    pub async fn open_immediate_delivery(&self) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/cgi-bin/express/local/business/open", Value::Null)
            .await?;
        Ok(response)
    }

    /// 发起门店绑定请求
    ///
    /// 本接口用于向配送公司发起门店绑定申请，绑定成功后可使用该配送公司的即时配送服务。
    ///
    /// # 参数
    /// * `request` - 绑定门店请求参数，包含配送公司ID（delivery_id）、门店名称（shop_name）、
    ///   门店地址（shop_address）、联系电话（shop_phone）、联系人等信息
    ///
    /// # 返回
    /// 返回 `LabradorResult<WechatApiResponse>`，成功时返回空响应。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/immediate-delivery/delivery-by-business/bindShop.html>
    pub async fn add_shop(&self, request: &AddShopRequest) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/cgi-bin/express/local/business/shop/add", request)
            .await?;
        Ok(response)
    }

    /// 重新下单
    ///
    /// 本接口用于在配送异常或取消后重新下单，使用原订单信息创建新的配送订单。
    ///
    /// # 参数
    /// * `request` - 重新下单请求参数，包含门店编号（shop_no）、原订单ID（order_id）、
    ///   原运单号（waybill_id）和配送公司ID（delivery_id）
    ///
    /// # 返回
    /// 返回 `LabradorResult<ReAddOrderResponse>`，包含新订单ID、运单号、运费等信息。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/immediate-delivery/delivery-by-business/reAddOrder.html>
    pub async fn re_add_order(
        &self,
        request: &ReAddOrderRequest,
    ) -> LabradorResult<ReAddOrderResponse> {
        let response: WechatApiResponse<ReAddOrderResponse> = self
            .client
            .wechat_client()
            .post("/cgi-bin/express/local/business/order/readd", request)
            .await?;
        response.into_result()
    }

    /// 模拟更新配送单状态（用于正式环境测试）
    ///
    /// 本接口用于在正式环境中模拟配送公司更新配送单状态，仅限测试使用。
    ///
    /// # 参数
    /// * `request` - 模拟更新请求参数，包含门店编号（shop_no）、订单ID（order_id）、
    ///   运单号（waybill_id）、操作时间（action_time）、订单状态（order_status）等
    ///
    /// # 返回
    /// 返回 `LabradorResult<WechatApiResponse>`，成功时返回空响应。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/immediate-delivery/delivery-by-business/mockUpdateOrder.html>
    pub async fn realmock_update_order(
        &self,
        request: &MockUpdateOrderRequest,
    ) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post(
                "/cgi-bin/express/local/business/realmock_update_order",
                request,
            )
            .await?;
        Ok(response)
    }

    /// 模拟配送公司更新配送单状态（用于沙盒环境）
    ///
    /// 本接口用于在沙盒环境中模拟配送公司更新配送单状态，用于开发调试。
    ///
    /// # 参数
    /// * `request` - 模拟更新请求参数，包含门店编号（shop_no）、订单ID（order_id）、
    ///   运单号（waybill_id）、操作时间（action_time）、订单状态（order_status）等
    ///
    /// # 返回
    /// 返回 `LabradorResult<WechatApiResponse>`，成功时返回空响应。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/immediate-delivery/delivery-by-business/testUpdateOrder.html>
    pub async fn test_update_order(
        &self,
        request: &MockUpdateOrderRequest,
    ) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/cgi-bin/express/local/business/test_update_order", request)
            .await?;
        Ok(response)
    }

    /// 拉取配送单信息
    ///
    /// 本接口用于查询指定配送单的详细信息，包括订单状态、骑手信息、费用等。
    ///
    /// # 参数
    /// * `request` - 查询请求参数，包含门店编号（shop_no）、订单ID（order_id）
    ///   或运单号（waybill_id）
    ///
    /// # 返回
    /// 返回 `LabradorResult<DeliveryOrderDetail>`，包含订单ID、运单号、订单状态、
    /// 骑手信息、费用、收发件人信息等详细信息。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/immediate-delivery/delivery-by-business/getOrder.html>
    pub async fn get_order(
        &self,
        request: &GetDeliveryOrderRequest,
    ) -> LabradorResult<DeliveryOrderDetail> {
        let response: WechatApiResponse<DeliveryOrderDetail> = self
            .client
            .wechat_client()
            .post("/cgi-bin/express/local/business/order/get", request)
            .await?;
        response.into_result()
    }

    /// 异常件退回商家确认
    ///
    /// 本接口用于商家确认异常件退回，确认后配送单将进入退回流程。
    ///
    /// # 参数
    /// * `request` - 确认退回请求参数，包含门店编号（shop_no）、订单ID（order_id）
    ///   或运单号（waybill_id）
    ///
    /// # 返回
    /// 返回 `LabradorResult<WechatApiResponse>`，成功时返回空响应。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/immediate-delivery/delivery-by-business/confirmReturn.html>
    pub async fn confirm_return(
        &self,
        request: &ConfirmReturnRequest,
    ) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post(
                "/cgi-bin/express/local/business/order/confirm_return",
                request,
            )
            .await?;
        Ok(response)
    }

    /// 取消配送单
    ///
    /// 本接口用于正式取消配送单。建议先调用预取消接口查询取消费用。
    ///
    /// # 参数
    /// * `request` - 取消请求参数，包含门店编号（shop_no）、订单ID（order_id）
    ///   或运单号（waybill_id）、取消原因ID（cancel_reason_id）和取消原因（cancel_reason）
    ///
    /// # 返回
    /// 返回 `LabradorResult<CancelOrderResponse>`，包含扣除费用（deduct_fee）和描述信息。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/immediate-delivery/delivery-by-business/cancelOrder.html>
    pub async fn cancel_order(
        &self,
        request: &CancelDeliveryOrderRequest,
    ) -> LabradorResult<CancelOrderResponse> {
        let response: WechatApiResponse<CancelOrderResponse> = self
            .client
            .wechat_client()
            .post("/cgi-bin/express/local/business/order/cancel", request)
            .await?;
        response.into_result()
    }

    /// 添加小费
    ///
    /// 本接口用于在配送过程中向骑手添加小费，以激励骑手更快送达。
    ///
    /// # 参数
    /// * `request` - 添加小费请求参数，包含门店编号（shop_no）、订单ID（order_id）
    ///   或运单号（waybill_id）、小费金额（tips，单位分）、备注（remark）等
    ///
    /// # 返回
    /// 返回 `LabradorResult<WechatApiResponse>`，成功时返回空响应。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/immediate-delivery/delivery-by-business/addTips.html>
    pub async fn add_tips(&self, request: &AddTipsRequest) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/cgi-bin/express/local/business/order/addtips", request)
            .await?;
        Ok(response)
    }

    /// 正式下单
    ///
    /// 本接口用于正式创建即时配送订单。建议先调用预下单接口确认费用等信息后再正式下单。
    ///
    /// # 参数
    /// * `request` - 下单请求参数，包含配送公司ID（delivery_id）、门店编号（shop_no）、
    ///   发货人（sender）、收货人（receiver）、货物信息（cargo）、订单信息（order_info）等
    ///
    /// # 返回
    /// 返回 `LabradorResult<AddDeliveryOrderResponse>`，包含订单ID、运单号、运费、
    /// 预计送达时间等信息。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/immediate-delivery/delivery-by-business/addOrder.html>
    pub async fn add_order(
        &self,
        request: &AddDeliveryOrderRequest,
    ) -> LabradorResult<AddDeliveryOrderResponse> {
        let response: WechatApiResponse<AddDeliveryOrderResponse> = self
            .client
            .wechat_client()
            .post("/cgi-bin/express/local/business/order/add", request)
            .await?;
        response.into_result()
    }

    // --- 运力方使用接口 ---

    /// 更新配送单状态（供配送公司调用）
    ///
    /// 本接口供配送公司（运力方）调用，用于更新配送单的实时状态。
    ///
    /// # 参数
    /// * `request` - 更新请求参数，包含运单号（waybill_id）、订单状态（order_status）、
    ///   操作时间（action_time）和操作描述（action_msg）
    ///
    /// # 返回
    /// 返回 `LabradorResult<WechatApiResponse>`，成功时返回空响应。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/immediate-delivery/delivery-by-delivery/updateOrder.html>
    pub async fn delivery_update_order(
        &self,
        request: &DeliveryUpdateOrderRequest,
    ) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/cgi-bin/express/local/delivery/update_order", request)
            .await?;
        Ok(response)
    }
}

//----------------------------------------------------------------------------------------------------------------------------
// 请求与响应结构体

/// 配送公司信息
#[derive(Debug, Clone, Deserialize)]
pub struct DeliveryCompany {
    /// 配送公司ID
    pub id: String,
    /// 配送公司名称
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
struct GetAllDeliveryCompanyResponse {
    list: Vec<DeliveryCompany>,
}

/// 预下单请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreAddOrderRequest {
    pub delivery_id: String,
    pub shop_no: String,
    pub sender: Sender,
    pub receiver: DeliveryReceiver,
    pub cargo: DeliveryGoodsInfo,
    pub order_info: OrderInfo,
    pub shop: Shop,
    pub sub_biz_id: Option<String>,
}

/// 预下单响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreAddOrderResponse {
    pub result_code: i32,
    pub result_msg: String,
    pub order_id: Option<String>,
    pub waybill_id: Option<String>,
    pub fee: Option<i64>,
    pub delivery_id: Option<String>,
    pub expected_delivery_time: Option<i64>,
    pub distance: Option<i64>,
    pub coupon_money: Option<i64>,
    pub tips: Option<i64>,
}

/// 预取消请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreCancelOrderRequest {
    pub shop_no: String,
    pub shop_id: Option<String>,
    pub order_id: Option<String>,
    pub waybill_id: Option<String>,
    pub cancel_reason_id: i32,
    pub cancel_reason: Option<String>,
}

/// 预取消响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreCancelOrderResponse {
    pub deduct_fee: i64,
    pub desc: String,
}

/// 绑定门店请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddShopRequest {
    pub delivery_id: String,
    pub business_name: Option<String>,
    pub shop_name: String,
    pub shop_address: String,
    pub shop_phone: String,
    pub shop_no: String,
    pub contact_name: String,
    pub contact_phone: String,
    pub city: String,
    pub business_license: Option<String>,
    pub id_card: Option<String>,
    pub id_card_name: Option<String>,
    pub door_photo: Option<String>,
    pub id_card_photo: Option<String>,
    pub business_license_photo: Option<String>,
}

/// 已绑定门店信息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoundShopInfo {
    pub shop_list: Vec<BoundShop>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoundShop {
    pub shop_no: String,
    pub shop_name: String,
    pub shop_phone: String,
    pub shop_address: String,
    pub shop_status: i32,
    pub delivery_id: String,
    pub delivery_name: String,
    pub business_name: Option<String>,
    pub contact_name: String,
    pub contact_phone: String,
}

/// 重新下单请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReAddOrderRequest {
    pub shop_no: String,
    pub shop_id: Option<String>,
    pub order_id: String,
    pub waybill_id: String,
    pub delivery_id: String,
}

/// 重新下单响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReAddOrderResponse {
    pub result_code: i32,
    pub result_msg: String,
    pub order_id: String,
    pub waybill_id: String,
    pub fee: i64,
    pub expected_delivery_time: i64,
    pub tips: i64,
    pub coupon_money: i64,
}

/// 模拟更新状态请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MockUpdateOrderRequest {
    pub shop_no: String,
    pub shop_id: Option<String>,
    pub order_id: Option<String>,
    pub waybill_id: Option<String>,
    pub action_time: i64,
    pub order_status: i32,
    pub action_msg: Option<String>,
}

/// 查询订单请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDeliveryOrderRequest {
    pub shop_no: String,
    pub shop_id: Option<String>,
    pub order_id: Option<String>,
    pub waybill_id: Option<String>,
}

/// 订单详情
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryOrderDetail {
    pub order_id: String,
    pub waybill_id: String,
    pub order_status: i32,
    pub order_status_msg: String,
    pub fee: i64,
    pub distance: i64,
    pub receiver: DeliveryReceiver,
    pub sender: Sender,
    pub cargo: DeliveryGoodsInfo,
    pub expected_finish_time: i64,
    pub rider_name: Option<String>,
    pub rider_phone: Option<String>,
    pub shop_no: String,
    pub shop_id: Option<String>,
}

/// 确认退回请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmReturnRequest {
    pub shop_no: String,
    pub shop_id: Option<String>,
    pub order_id: Option<String>,
    pub waybill_id: Option<String>,
}

/// 取消订单请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelDeliveryOrderRequest {
    pub shop_no: String,
    pub shop_id: Option<String>,
    pub order_id: Option<String>,
    pub waybill_id: Option<String>,
    pub cancel_reason_id: i32,
    pub cancel_reason: Option<String>,
}

/// 取消订单响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    pub deduct_fee: i64,
    pub desc: String,
}

/// 添加小费请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddTipsRequest {
    pub shop_no: String,
    pub shop_id: Option<String>,
    pub order_id: Option<String>,
    pub waybill_id: Option<String>,
    pub tips: i64,
    pub remark: Option<String>,
    pub reason: Option<String>,
}

/// 添加订单请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddDeliveryOrderRequest {
    pub delivery_id: String,
    pub shop_no: String,
    pub sender: Sender,
    pub receiver: DeliveryReceiver,
    pub cargo: DeliveryGoodsInfo,
    pub order_info: OrderInfo,
    pub shop: Shop,
    pub sub_biz_id: Option<String>,
}

/// 添加订单响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddDeliveryOrderResponse {
    pub result_code: i32,
    pub result_msg: String,
    pub order_id: String,
    pub waybill_id: String,
    pub fee: i64,
    pub expected_delivery_time: i64,
    pub distance: i64,
    pub coupon_money: i64,
    pub tips: i64,
    pub delivery_id: String,
}

/// 配送公司更新订单状态请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryUpdateOrderRequest {
    pub waybill_id: String,
    pub order_status: i32,
    pub action_time: i64,
    pub action_msg: Option<String>,
}

// --- 公共子结构体 ---
// 这些结构体被多个请求复用，也应用驼峰重命名

/// 发货人信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sender {
    pub name: String,
    pub phone: String,
    pub address: String,
    pub city: String,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
}

/// 收货人信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryReceiver {
    pub name: String,
    pub phone: String,
    pub address: String,
    pub city: String,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
}

/// 货物信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryGoodsInfo {
    pub goods_value: i64,
    pub goods_height: Option<i64>,
    pub goods_length: Option<i64>,
    pub goods_width: Option<i64>,
    pub goods_weight: Option<i64>,
    pub goods_name: String,
    pub goods_detail: Option<GoodsDetail>,
}

/// 货物详情
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodsDetail {
    pub goods: Vec<GoodsItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodsItem {
    pub good_count: i32,
    pub good_name: String,
    pub good_price: i64,
    pub good_units: String,
}

/// 订单信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderInfo {
    pub order_id: String,
    pub order_time: i64,
    pub is_insured: i32,
    pub declared_value: Option<i64>,
    pub tips: Option<i64>,
    pub insured_value: Option<i64>,
    pub cash_on_delivery: Option<i64>,
    pub order_source: Option<String>,
    pub delivery_service_code: Option<String>,
    pub expected_delivery_time: Option<i64>,
}

/// 门店信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shop {
    pub wxa_path: Option<String>,
    pub wxa_appid: Option<String>,
    pub shop_name: String,
    pub shop_no: String,
    pub shop_phone: String,
    pub shop_address: String,
    pub shop_img: Option<String>,
}
