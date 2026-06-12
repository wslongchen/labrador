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

use crate::errors::LabradorResult;
use crate::wechat::client::WechatApiResponse;
use crate::wechat::miniapp::WechatMiniAppClient;

/// 物流助手模块
///
/// 提供电子面单、运单管理、打印员配置、轨迹查询等接口。
/// 包含小程序端和运力方（快递公司）使用的接口。
#[derive(Debug, Clone)]
pub struct WechatMxaExpress<'a> {
    client: &'a WechatMiniAppClient,
}

#[allow(unused)]
impl<'a> WechatMxaExpress<'a> {
    #[inline]
    pub fn new(client: &'a WechatMiniAppClient) -> Self {
        Self { client }
    }

    // ========================= 小程序使用接口 =========================

    /// 绑定/解绑物流账号
    ///
    /// 该接口用于商家绑定或解绑其在快递公司侧的商户账号。
    ///
    /// # 参数
    /// * `request` - 绑定/解绑请求参数，包含快递公司ID、绑定状态、账号密码等。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/express/express-by-business/bindAccount.html>
    pub async fn bind_account(
        &self,
        request: &BindAccountRequest,
    ) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/cgi-bin/express/business/account/bind", request)
            .await?;
        Ok(response)
    }

    /// 获取所有绑定的物流账号
    ///
    /// 该接口用于获取商家在所有快递公司已绑定的账号列表。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/express/express-by-business/getAllBindAccount.html>
    pub async fn get_all_bound_account(&self) -> LabradorResult<Vec<BoundAccount>> {
        let response: WechatApiResponse<GetAllBoundAccountResponse> = self
            .client
            .wechat_client()
            .get("/cgi-bin/express/business/account/getall")
            .await?;
        response.into_result().map(|r| r.list)
    }

    /// 获取支持的快递公司列表
    ///
    /// 该接口用于获取微信物流助手当前支持的所有快递公司列表。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/express/express-by-business/getAllDelivery.html>
    pub async fn get_all_delivery(&self) -> LabradorResult<Vec<ExpressDeliveryCompany>> {
        let response: WechatApiResponse<GetAllDeliveryResponse> = self
            .client
            .wechat_client()
            .get("/cgi-bin/express/business/delivery/getall")
            .await?;
        response.into_result().map(|r| r.list)
    }

    /// 取消运单
    ///
    /// 该接口用于在生成运单后，发货前取消运单。
    ///
    /// # 参数
    /// * `request` - 取消运单请求参数，包含运单ID、商户订单号等。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/express/express-by-business/cancelOrder.html>
    pub async fn cancel_order(
        &self,
        request: &CancelExpressOrderRequest,
    ) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/cgi-bin/express/business/order/cancel", request)
            .await?;
        Ok(response)
    }

    /// 配置面单打印员
    ///
    /// 该接口用于配置面单打印员，可以设置多个。
    /// 若需要使用微信打单 PC 软件，才需要调用此接口。
    ///
    /// # 参数
    /// * `request` - 配置打印员请求参数，包含打印员openid或微信号、绑定/解绑标记。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/express/express-by-business/updatePrinter.html>
    pub async fn update_printer(
        &self,
        request: &UpdatePrinterRequest,
    ) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/cgi-bin/express/business/printer/update", request)
            .await?;
        Ok(response)
    }

    /// 获取电子面单余额
    ///
    /// 该接口用于获取商家在指定快递公司的电子面单账户余额。
    ///
    /// # 参数
    /// * `delivery_id` - 快递公司ID
    /// * `biz_id` - 商家侧在快递公司绑定的业务ID（即 account）
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/express/express-by-business/getQuota.html>
    pub async fn get_quota(&self, delivery_id: &str, biz_id: &str) -> LabradorResult<QuotaInfo> {
        let response: WechatApiResponse<QuotaInfo> = self
            .client
            .wechat_client()
            .get(&format!(
                "/cgi-bin/express/business/quota/get?delivery_id={}&biz_id={}",
                delivery_id, biz_id
            ))
            .await?;
        response.into_result()
    }

    /// 获取运单数据
    ///
    /// 该接口用于根据运单ID或商户订单号获取运单的详细信息。
    ///
    /// # 参数
    /// * `request` - 查询运单请求参数。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/express/express-by-business/getOrder.html>
    pub async fn get_order(
        &self,
        request: &GetExpressOrderRequest,
    ) -> LabradorResult<ExpressOrderDetail> {
        let response: WechatApiResponse<ExpressOrderDetail> = self
            .client
            .wechat_client()
            .post("/cgi-bin/express/business/order/get", request)
            .await?;
        response.into_result()
    }

    /// 模拟更新订单状态
    ///
    /// 该接口用于模拟快递公司更新订单状态，仅限测试环境使用。
    ///
    /// # 参数
    /// * `request` - 模拟更新请求参数。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/express/express-by-business/testUpdateOrder.html>
    pub async fn test_update_order(
        &self,
        request: &TestUpdateOrderRequest,
    ) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/cgi-bin/express/business/test_update_order", request)
            .await?;
        Ok(response)
    }

    /// 获取打印员列表
    ///
    /// 该接口用于获取当前已配置的所有面单打印员信息。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/express/express-by-business/getAllPrinter.html>
    pub async fn get_all_printer(&self) -> LabradorResult<Vec<PrinterInfo>> {
        let response: WechatApiResponse<GetAllPrinterResponse> = self
            .client
            .wechat_client()
            .get("/cgi-bin/express/business/printer/getall")
            .await?;
        response.into_result().map(|r| r.list)
    }

    /// 查询运单轨迹
    ///
    /// 该接口用于查询运单的实时轨迹信息。
    ///
    /// # 参数
    /// * `request` - 查询轨迹请求参数。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/express/express-by-business/getPath.html>
    pub async fn get_path(&self, request: &GetPathRequest) -> LabradorResult<PathInfo> {
        let response: WechatApiResponse<PathInfo> = self
            .client
            .wechat_client()
            .post("/cgi-bin/express/business/path/get", request)
            .await?;
        response.into_result()
    }

    /// 批量获取运单数据
    ///
    /// 该接口用于根据多个商户订单号批量获取运单信息。
    ///
    /// # 参数
    /// * `request` - 批量查询请求参数，包含订单号列表（最多50个）。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/express/express-by-business/batchGetOrder.html>
    pub async fn batch_get_order(
        &self,
        request: &BatchGetOrderRequest,
    ) -> LabradorResult<Vec<ExpressOrderDetail>> {
        let response: WechatApiResponse<BatchGetOrderResponse> = self
            .client
            .wechat_client()
            .post("/cgi-bin/express/business/order/batchget", request)
            .await?;
        response.into_result().map(|r| r.order_list)
    }

    /// 生成运单
    ///
    /// 该接口用于生成物流运单，获取电子面单。
    ///
    /// # 参数
    /// * `request` - 生成运单请求参数，包含收发件人信息、货物信息、快递公司ID等。
    ///
    /// # 返回
    /// 包含运单号、面单数据等信息的响应。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/express/express-by-business/addOrder.html>
    pub async fn add_order(
        &self,
        request: &AddExpressOrderRequest,
    ) -> LabradorResult<AddExpressOrderResponse> {
        let response: WechatApiResponse<AddExpressOrderResponse> = self
            .client
            .wechat_client()
            .post("/cgi-bin/express/business/order/add", request)
            .await?;
        response.into_result()
    }

    // ========================= 运力方使用接口 =========================
    // 以下接口通常由已接入的快递公司（运力方）调用

    /// 更新商户审核结果
    ///
    /// 该接口用于快递公司更新商家的账号审核结果。
    ///
    /// # 参数
    /// * `request` - 更新审核结果请求。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/express/express-by-delivery/updateBusiness.html>
    pub async fn update_business(
        &self,
        request: &UpdateBusinessRequest,
    ) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/cgi-bin/express/delivery/service/business/update", request)
            .await?;
        Ok(response)
    }

    /// 更新运单轨迹
    ///
    /// 该接口用于快递公司更新运单的物流轨迹信息。
    ///
    /// # 参数
    /// * `request` - 更新轨迹请求。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/express/express-by-delivery/updatePath.html>
    pub async fn update_path(
        &self,
        request: &UpdatePathRequest,
    ) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/cgi-bin/express/delivery/path/update", request)
            .await?;
        Ok(response)
    }

    /// 预览面单模板
    ///
    /// 该接口用于快递公司预览面单模板样式。
    ///
    /// # 参数
    /// * `request` - 预览模板请求。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/express/express-by-delivery/previewTemplate.html>
    pub async fn preview_template(
        &self,
        request: &PreviewTemplateRequest,
    ) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/cgi-bin/express/delivery/template/preview", request)
            .await?;
        Ok(response)
    }

    /// 获取面单联系人信息
    ///
    /// 该接口用于快递公司获取运单中联系人的脱敏信息。
    ///
    /// # 参数
    /// * `request` - 获取联系人信息请求。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/express/express-by-delivery/getContact.html>
    pub async fn get_contact(&self, request: &GetContactRequest) -> LabradorResult<ContactInfo> {
        let response: WechatApiResponse<ContactInfo> = self
            .client
            .wechat_client()
            .post("/cgi-bin/express/delivery/contact/get", request)
            .await?;
        response.into_result()
    }
}

// ============================================================================
// 请求与响应结构体
// ============================================================================

// -------------------- 小程序使用 结构体 --------------------

/// 绑定/解绑物流账号请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BindAccountRequest {
    /// 快递公司ID，通过 get_all_delivery 接口获得
    pub delivery_id: String,
    /// 绑定状态：bind-绑定，unbind-解绑
    pub r#type: String,
    /// 商家侧在快递公司注册的账号
    pub biz_id: Option<String>,
    /// 商家侧在快递公司注册的密码
    pub password: Option<String>,
    /// 备注内容，选填
    pub remark_content: Option<String>,
}

/// 已绑定的物流账号信息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoundAccount {
    /// 快递公司ID
    pub delivery_id: String,
    /// 快递公司名称
    pub delivery_name: String,
    /// 商家侧在快递公司绑定的业务ID（即 account）
    pub biz_id: String,
    /// 绑定状态：1-已绑定，2-绑定中，其他-异常
    pub status: i32,
    /// 更新时间
    pub update_time: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetAllBoundAccountResponse {
    list: Vec<BoundAccount>,
}

/// 支持的快递公司信息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpressDeliveryCompany {
    /// 快递公司ID
    pub delivery_id: String,
    /// 快递公司名称
    pub delivery_name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetAllDeliveryResponse {
    #[serde(default)]
    list: Vec<ExpressDeliveryCompany>,
}

/// 取消运单请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelExpressOrderRequest {
    /// 商家侧订单ID，与运单号二选一
    pub order_id: Option<String>,
    /// 微信订单号，与商户订单号二选一
    pub waybill_id: Option<String>,
    /// 取消原因
    pub cancel_reason: Option<String>,
}

/// 配置打印员请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePrinterRequest {
    /// 打印员openid（与userid二选一）
    pub openid: Option<String>,
    /// 打印员微信号（与openid二选一）
    pub userid: Option<String>,
    /// 标记操作：bind-绑定，unbind-解绑
    pub r#type: String,
    /// 打印员别名
    pub tag_name: Option<String>,
}

/// 打印员信息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrinterInfo {
    /// 打印员openid
    pub openid: String,
    /// 打印员微信号
    pub userid: String,
    /// 打印员别名
    pub tag_name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetAllPrinterResponse {
    list: Vec<PrinterInfo>,
}

/// 电子面单余额信息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotaInfo {
    /// 余额数量
    pub quota_num: i32,
}

/// 查询运单请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExpressOrderRequest {
    /// 商家侧订单ID（与运单号二选一）
    pub order_id: Option<String>,
    /// 微信运单号（与商户订单号二选一）
    pub waybill_id: Option<String>,
    /// 快递公司ID，当使用waybill_id查询时必填
    pub delivery_id: Option<String>,
    /// 商家侧在快递公司绑定的业务ID，当使用waybill_id查询时必填
    pub biz_id: Option<String>,
    /// 打印面单类型：默认1，使用waybill_id查询时生效
    pub print_type: Option<i32>,
}

/// 运单详情
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpressOrderDetail {
    /// 微信运单号
    pub waybill_id: String,
    /// 商家侧订单ID
    pub order_id: String,
    /// 快递公司ID
    pub delivery_id: String,
    /// 商家侧在快递公司绑定的业务ID
    pub biz_id: String,
    /// 运单状态
    pub order_status: i32,
    /// 运单状态描述
    pub order_status_msg: String,
    /// 生成运单时返回的面单数据
    pub waybill_data: Option<String>,
    /// 运单轨迹列表
    pub path_item_list: Option<Vec<PathItem>>,
    /// 运费金额（分）
    pub fee: i64,
    /// 下单时间
    pub add_time: i64,
    /// 收件人信息
    pub receiver: ExpressReceiver,
    /// 发件人信息
    pub sender: ExpressSender,
    /// 货物信息
    pub cargo: Cargo,
    /// 保价信息
    pub insured: Insured,
}

/// 轨迹点信息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathItem {
    /// 轨迹节点时间戳
    pub action_time: i64,
    /// 轨迹节点类型
    pub action_type: i32,
    /// 轨迹节点描述
    pub action_msg: String,
}

/// 模拟更新订单状态请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestUpdateOrderRequest {
    /// 快递公司ID
    pub delivery_id: String,
    /// 商家侧在快递公司绑定的业务ID
    pub biz_id: String,
    /// 微信运单号
    pub waybill_id: String,
    /// 轨迹节点时间戳
    pub action_time: i64,
    /// 轨迹节点类型
    pub action_type: i32,
    /// 轨迹节点描述
    pub action_msg: Option<String>,
}

/// 查询轨迹请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPathRequest {
    /// 微信运单号
    pub waybill_id: String,
    /// 快递公司ID
    pub delivery_id: String,
    /// 商家侧在快递公司绑定的业务ID
    pub biz_id: String,
    /// 是否需要返回完整的轨迹列表
    pub need_all_path: Option<bool>,
}

/// 轨迹信息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathInfo {
    /// 微信运单号
    pub waybill_id: String,
    /// 轨迹列表
    pub path_item_list: Vec<PathItem>,
}

/// 批量查询请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchGetOrderRequest {
    /// 商家侧订单ID列表，最多50个
    pub order_id_list: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BatchGetOrderResponse {
    order_list: Vec<ExpressOrderDetail>,
}

/// 生成运单请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddExpressOrderRequest {
    /// 快递公司ID
    pub delivery_id: String,
    /// 商家侧在快递公司绑定的业务ID
    pub biz_id: String,
    /// 商家侧订单ID
    pub order_id: String,
    /// 收件人信息
    pub receiver: ExpressReceiver,
    /// 发件人信息
    pub sender: ExpressSender,
    /// 货物信息
    pub cargo: Cargo,
    /// 保价信息
    pub insured: Insured,
    /// 下单时间戳
    pub order_time: i64,
    /// 操作人openid
    pub shop_openid: Option<String>,
    /// 备注
    pub remark: Option<String>,
    /// 贴尾面单标识
    pub is_end: Option<bool>,
    /// 发货方式
    pub send_type: Option<i32>,
    /// 自定义字段
    pub custom_remark: Option<String>,
    /// 面单打印类型
    pub print_type: Option<i32>,
    /// 商户侧订单来源
    pub order_source: Option<String>,
}

/// 生成运单响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddExpressOrderResponse {
    /// 微信运单号
    pub waybill_id: String,
    /// 面单渲染后的数据，用于打印
    pub waybill_data: String,
    /// 运费金额（分）
    pub fee: i64,
    /// 下单结果码
    pub result_code: i32,
    /// 下单结果信息
    pub result_msg: String,
}

// -------------------- 公共子结构体 --------------------
// 被多个请求复用的收/发件人、货物、保价信息

/// 收/发件人信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpressSender {
    /// 姓名
    pub name: String,
    /// 联系电话
    pub phone: String,
    /// 省份
    pub province: String,
    /// 城市
    pub city: String,
    /// 区县
    pub district: String,
    /// 详细地址
    pub address: String,
}

pub use ExpressSender as ExpressReceiver;
// 收件人结构相同，复用

/// 货物信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cargo {
    /// 货物数量
    pub count: i32,
    /// 货物重量（千克）
    pub weight: f64,
    /// 货物价值（分）
    pub space_x: i32,
    /// 货物名称
    pub cargo: String,
}

/// 保价信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Insured {
    /// 是否保价：0-否，1-是
    pub use_insured: i32,
    /// 保价金额（分）
    pub insured_value: i64,
}

// -------------------- 运力方使用 结构体 --------------------

/// 更新商户审核结果请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBusinessRequest {
    /// 快递公司ID
    pub delivery_id: String,
    /// 商家侧在快递公司绑定的业务ID
    pub biz_id: String,
    /// 审核结果码
    pub result_code: i32,
    /// 审核结果信息
    pub result_msg: Option<String>,
}

/// 更新运单轨迹请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePathRequest {
    /// 快递公司ID
    pub delivery_id: String,
    /// 商家侧在快递公司绑定的业务ID
    pub biz_id: String,
    /// 微信运单号
    pub waybill_id: String,
    /// 轨迹节点时间戳
    pub action_time: i64,
    /// 轨迹节点类型
    pub action_type: i32,
    /// 轨迹节点描述
    pub action_msg: Option<String>,
    /// 该轨迹对应的面单数据（可选）
    pub waybill_data: Option<String>,
}

/// 预览面单模板请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewTemplateRequest {
    /// 快递公司ID
    pub delivery_id: String,
    /// 商家侧在快递公司绑定的业务ID
    pub biz_id: String,
    /// 订单ID
    pub order_id: String,
    /// 面单版本号
    pub template_version: Option<String>,
}

/// 获取联系人信息请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactRequest {
    /// 微信运单号
    pub token: String,
    /// 运单所属的商户openid
    pub openid: String,
    /// 快递公司ID
    pub delivery_id: String,
    /// 商家侧在快递公司绑定的业务ID
    pub biz_id: String,
    /// 运单号
    pub waybill_id: String,
}

/// 联系人信息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactInfo {
    /// 发件人姓名（脱敏）
    pub sender_name: String,
    /// 发件人电话（脱敏）
    pub sender_phone: String,
    /// 收件人姓名（脱敏）
    pub receiver_name: String,
    /// 收件人电话（脱敏）
    pub receiver_phone: String,
}
