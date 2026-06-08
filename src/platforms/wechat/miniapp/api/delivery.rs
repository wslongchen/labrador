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
    pub async fn get_all_delivery_company(&self) -> LabradorResult<Vec<DeliveryCompany>> {
        let response: WechatApiResponse<GetAllDeliveryCompanyResponse> = self
            .client
            .wechat_client()
            .get("/cgi-bin/express/local/business/delivery/getall")
            .await?;
        response.into_result().map(|r| r.list)
    }

    /// 预下配送单
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

    /// 拉取已绑定账号
    pub async fn get_bound_shop(&self) -> LabradorResult<BoundShopInfo> {
        let response: WechatApiResponse<BoundShopInfo> = self
            .client
            .wechat_client()
            .get("/cgi-bin/express/local/business/shop/get")
            .await?;
        response.into_result()
    }

    /// 预取消配送单
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
    pub async fn open_immediate_delivery(&self) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/cgi-bin/express/local/business/open", Value::Null)
            .await?;
        Ok(response)
    }

    /// 发起绑定请求
    pub async fn add_shop(&self, request: &AddShopRequest) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/cgi-bin/express/local/business/shop/add", request)
            .await?;
        Ok(response)
    }

    /// 重新下单
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

    /// 模拟更新配送单状态（用于测试）
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
    pub async fn add_tips(&self, request: &AddTipsRequest) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/cgi-bin/express/local/business/order/addtips", request)
            .await?;
        Ok(response)
    }

    /// 添加配送单
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
