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
use serde_json::{json};
use bytes::Bytes;
use crate::errors::LabradorResult;
use crate::request::RequestBody;
use crate::wechat::client::WechatApiResponse;
use crate::wechat::mp::WechatMpClient;

/// 微信发票模块（含商户开票、开票平台、发票报销、非税票据）
#[derive(Debug, Clone)]
pub struct WechatMpInvoice<'a> {
    client: &'a WechatMpClient,
}

#[allow(unused)]
impl<'a> WechatMpInvoice<'a> {
    #[inline]
    pub fn new(client: &'a WechatMpClient) -> Self {
        Self { client }
    }

    // ==================== 商户开票接口 ====================

    /// 查询与设置授权页与商户信息
    ///
    /// 本接口提供了一系列用于管理授权页和商户信息的功能：
    /// - set_auth_field：设置授权页上用户需要填写的信息
    /// - get_auth_field：查询授权页的字段设置情况
    /// - set_pay_mch：将商户号与开票平台的识别号进行关联
    /// - get_pay_mch：查询商户与开票平台的绑定情况
    /// - set_contact：设置商户的联系方式
    /// - get_contact：获取商户的联系方式
    pub async fn set_biz_attr(&self, action: &str, request: &BizAttrRequest) -> LabradorResult<BizAttrResponse> {
        let response: WechatApiResponse<BizAttrResponse> = self.client.wechat_client()
            .post(&format!("/card/invoice/setbizattr?action={}", action), request)
            .await?;
        response.into_result()
    }

    /// 查询授权信息
    ///
    /// 执收单位可以调用该接口查询订单是否有被用户授权。
    pub async fn get_auth_data(&self, s_pappid: &str, order_id: &str) -> LabradorResult<AuthDataResponse> {
        let request = json!({
            "s_pappid": s_pappid,
            "order_id": order_id
        });
        let response: WechatApiResponse<AuthDataResponse> = self.client.wechat_client()
            .post("/card/invoice/getauthdata", request)
            .await?;
        response.into_result()
    }

    /// 获取授权页链接
    ///
    /// 商户通过本接口传入订单号、开票平台标识等参数，获取授权页的链接。
    /// type参数控制授权页样式：
    /// - 0：开票授权（申请开票类型）
    /// - 1：填写字段开票授权（填写抬头申请开票类型）
    /// - 2：领票授权（领取发票类型）
    pub async fn get_auth_url(&self, request: &AuthUrlRequest) -> LabradorResult<AuthUrlResponse> {
        let response: WechatApiResponse<AuthUrlResponse> = self.client.wechat_client()
            .post("/card/invoice/getauthurl", request)
            .await?;
        response.into_result()
    }

    /// 拒绝领受发票
    ///
    /// 商户收到用户的发票之后，可以选择是否接收。若不接收，则可以调用该接口拒绝领受发票。
    pub async fn reject_insert(&self, s_pappid: &str, order_id: &str, reason: &str) -> LabradorResult<WechatApiResponse> {
        let request = json!({
            "s_pappid": s_pappid,
            "order_id": order_id,
            "reason": reason
        });
        let response: WechatApiResponse = self.client.wechat_client()
            .post("/card/invoice/rejectinsert", request)
            .await?;
        Ok(response)
    }

    // ==================== 开票平台接口 ====================

    /// 设置商户联系信息
    ///
    /// 开票平台可以设置商户的联系方式等信息。
    pub async fn set_invoice_url(&self, request: &SetInvoiceUrlRequest) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self.client.wechat_client()
            .post("/card/invoice/platform/seturl", request)
            .await?;
        Ok(response)
    }

    /// 获取发票PDF
    ///
    /// 开票平台可以通过该接口获取发票PDF文件。
    pub async fn get_invoice_pdf(&self, s_media_id: &str) -> LabradorResult<Bytes> {
        let request = json!({ "s_media_id": s_media_id });
        self.client.wechat_client()
            .post_bytes("/card/invoice/platform/getpdf", request)
            .await
    }

    /// 更新发票状态
    ///
    /// 开票平台可以通过该接口更新发票的状态。
    pub async fn update_invoice_status(&self, request: &UpdateInvoiceStatusRequest) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self.client.wechat_client()
            .post("/card/invoice/platform/updatestatus", request)
            .await?;
        Ok(response)
    }

    /// 上传发票PDF
    ///
    /// 商户或开票平台可以通过该接口上传PDF。PDF上传成功后将获得发票文件的标识s_media_id。
    /// 注意：s_media_id有效期3天，3天内未关联到发票卡券将自动销毁。
    pub async fn set_invoice_pdf(&self, pdf_data: Vec<u8>, file_name: &str) -> LabradorResult<SetPdfResponse> {
        let form = reqwest::multipart::Form::new()
            .part("pdf", reqwest::multipart::Part::bytes(pdf_data).file_name(file_name.to_string()));
        let response: WechatApiResponse<SetPdfResponse> = self.client.wechat_client()
            .post("/card/invoice/platform/setpdf", RequestBody::Multipart(form))
            .await?;
        response.into_result()
    }

    /// 创建发票卡券模板
    ///
    /// 开票平台可以通过该接口创建发票卡券模板。
    pub async fn create_invoice_card(&self, request: &CreateInvoiceCardRequest) -> LabradorResult<CreateInvoiceCardResponse> {
        let response: WechatApiResponse<CreateInvoiceCardResponse> = self.client.wechat_client()
            .post("/card/invoice/platform/createcard", request)
            .await?;
        response.into_result()
    }

    /// 插入发票到用户卡包
    ///
    /// 开票平台可以通过该接口将发票插入到用户的微信卡包中。
    pub async fn insert_invoice(&self, request: &InsertInvoiceRequest) -> LabradorResult<InsertInvoiceResponse> {
        let response: WechatApiResponse<InsertInvoiceResponse> = self.client.wechat_client()
            .post("/card/invoice/insert", request)
            .await?;
        response.into_result()
    }

    // ==================== 发票报销接口 ====================

    /// 获取报销发票信息
    ///
    /// 报销方可以通过该接口获取用户的发票信息。
    pub async fn get_invoice_info(&self, card_id: &str, encrypt_code: &str) -> LabradorResult<InvoiceInfoResponse> {
        let request = json!({
            "card_id": card_id,
            "encrypt_code": encrypt_code
        });
        let response: WechatApiResponse<InvoiceInfoResponse> = self.client.wechat_client()
            .post("/card/invoice/reimburse/getinvoiceinfo", request)
            .await?;
        response.into_result()
    }

    /// 更新报销发票状态
    ///
    /// 报销方可以通过该接口更新发票的报销状态。
    pub async fn update_invoice_status_reimburse(&self, request: &ReimburseStatusRequest) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self.client.wechat_client()
            .post("/card/invoice/reimburse/updatestatus", request)
            .await?;
        Ok(response)
    }

    /// 批量更新报销发票状态
    ///
    /// 报销方可以通过该接口批量更新发票的报销状态。
    pub async fn batch_update_invoice_status(&self, request: &BatchReimburseStatusRequest) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self.client.wechat_client()
            .post("/card/invoice/reimburse/updatestatusbatch", request)
            .await?;
        Ok(response)
    }

    /// 批量获取报销发票信息
    ///
    /// 报销方可以通过该接口批量获取用户的发票信息。
    pub async fn batch_get_invoice_info(&self, item_list: Vec<InvoiceItem>) -> LabradorResult<Vec<InvoiceInfoResponse>> {
        let request = json!({ "item_list": item_list });
        let response: WechatApiResponse<BatchInvoiceInfoResponse> = self.client.wechat_client()
            .post("/card/invoice/reimburse/getinvoiceinfobatch", request)
            .await?;
        Ok(response.into_result()?.item_list)
    }

    // ==================== 极速开发票 ====================

    /// 获取用户抬头（获取用户填写抬头链接）
    ///
    /// 商户可以通过该接口获取让用户填写抬头的链接。
    pub async fn get_user_title_url(&self, request: &UserTitleUrlRequest) -> LabradorResult<UserTitleUrlResponse> {
        let response: WechatApiResponse<UserTitleUrlResponse> = self.client.wechat_client()
            .post("/card/invoice/biz/getusertitleurl", request)
            .await?;
        response.into_result()
    }

    /// 获取选择抬头链接
    ///
    /// 商户可以通过该接口获取让用户选择抬头的链接。
    pub async fn get_select_title_url(&self, request: &SelectTitleUrlRequest) -> LabradorResult<SelectTitleUrlResponse> {
        let response: WechatApiResponse<SelectTitleUrlResponse> = self.client.wechat_client()
            .post("/card/invoice/biz/getselecttitleurl", request)
            .await?;
        response.into_result()
    }

    /// 扫描抬头
    ///
    /// 商户可以通过该接口扫描用户的抬头二维码。
    pub async fn scan_title(&self, scan_text: &str) -> LabradorResult<TitleInfo> {
        let request = json!({ "scan_text": scan_text });
        let response: WechatApiResponse<TitleInfo> = self.client.wechat_client()
            .post("/card/invoice/scantitle", request)
            .await?;
        response.into_result()
    }

    // ==================== 非税票据 ====================

    /// 获取非税票据授权页链接
    ///
    /// 此接口用于获取非税票据授权页链接，让用户跳转到授权页。
    pub async fn get_notax_auth_url(&self, request: &NotaxAuthUrlRequest) -> LabradorResult<NotaxAuthUrlResponse> {
        let response: WechatApiResponse<NotaxAuthUrlResponse> = self.client.wechat_client()
            .post("/nontax/getbillauthurl", request)
            .await?;
        response.into_result()
    }

    /// 创建财政电子票据模板
    ///
    /// 财政局可以通过这个接口帮助执收单位创建一张财政电子票据模板。
    pub async fn create_notax_card(&self, request: &CreateNotaxCardRequest) -> LabradorResult<CreateNotaxCardResponse> {
        let response: WechatApiResponse<CreateNotaxCardResponse> = self.client.wechat_client()
            .post("/nontax/createbillcard", request)
            .await?;
        response.into_result()
    }

    /// 票据插入用户卡包
    ///
    /// 执收单位完成用户插卡授权后，向财政局请求给某一个订单号进行领取财政电子票据。
    pub async fn insert_notax_invoice(&self, request: &InsertNotaxInvoiceRequest) -> LabradorResult<InsertNotaxInvoiceResponse> {
        let response: WechatApiResponse<InsertNotaxInvoiceResponse> = self.client.wechat_client()
            .post("/nontax/insertbill", request)
            .await?;
        response.into_result()
    }

    // ==================== 通用工具 ====================

    /// 获取api_ticket
    ///
    /// Api_ticket是用于调用js-sdk的临时票据，有效期为7200秒。
    /// type: jsapi 为 js-sdk凭证；wx_card 为微信卡券凭证
    pub async fn get_ticket(&self, ticket_type: &str) -> LabradorResult<TicketResponse> {
        let response: WechatApiResponse<TicketResponse> = self.client.wechat_client()
            .get(&format!("/cgi-bin/ticket/getticket?type={}", ticket_type))
            .await?;
        response.into_result()
    }
}

// ============================================================================
// 请求与响应结构体
// ============================================================================

// -------------------- 商户开票接口 --------------------

/// 授权页与商户信息请求（用于set_biz_attr）
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BizAttrRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_field: Option<AuthField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paymch_info: Option<PayMchInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<ContactInfo>,
}

/// 授权页字段
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthField {
    pub user_field: UserField,
    pub biz_field: BizField,
}

/// 个人发票字段
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserField {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_title: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_phone: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_email: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_phone: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_email: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field: Option<Vec<InvoiceCustomField>>,
}

/// 单位发票字段
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BizField {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_title: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_tax_no: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_addr: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_phone: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_bank_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_bank_no: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_tax_no: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_addr: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_phone: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_bank_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_bank_no: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field: Option<Vec<InvoiceCustomField>>,
}

/// 自定义字段
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceCustomField {
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_require: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notice: Option<String>,
}

/// 微信商户号与开票平台关系信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayMchInfo {
    pub mchid: String,
    pub s_pappid: String,
}

/// 联系方式信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactInfo {
    pub time_out: i64,
    pub phone: String,
}

/// 授权页与商户信息响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BizAttrResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_field: Option<AuthField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paymch_info: Option<PayMchInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<ContactInfo>,
}

/// 查询授权信息响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthDataResponse {
    pub invoice_status: String,
    pub auth_time: i64,
}

/// 获取授权页链接请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthUrlRequest {
    pub s_pappid: String,
    pub order_id: String,
    pub money: i32,
    pub timestamp: i64,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    pub ticket: String,
    pub r#type: i32,
}

/// 获取授权页链接响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthUrlResponse {
    pub auth_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,
}

// -------------------- 开票平台接口 --------------------

/// 设置商户联系信息请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetInvoiceUrlRequest {
    pub s_pappid: String,
    pub url: String,
}

/// 更新发票状态请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInvoiceStatusRequest {
    pub card_id: String,
    pub code: String,
    pub reimburse_status: String,
}

/// 上传发票PDF响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetPdfResponse {
    pub s_media_id: String,
}

/// 创建发票卡券模板请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInvoiceCardRequest {
    pub base_info: InvoiceBaseInfo,
    pub payee: String,
}

/// 发票卡券基础信息
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceBaseInfo {
    pub logo_url: String,
}

/// 创建发票卡券模板响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInvoiceCardResponse {
    pub card_id: String,
}

/// 插入发票请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertInvoiceRequest {
    pub order_id: String,
    pub card_id: String,
    pub appid: String,
    pub card_ext: InvoiceCardExt,
}

/// 发票卡券扩展信息
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceCardExt {
    pub user_card: UserCardInfo,
}

/// 用户卡信息
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserCardInfo {
    pub fee: i32,
    pub title: String,
    pub billing_time: i64,
    pub billing_no: String,
    pub billing_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_pdf_media_id: Option<String>,
}

/// 插入发票响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertInvoiceResponse {
    pub code: String,
    pub openid: String,
}

// -------------------- 发票报销接口 --------------------

/// 发票信息响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceInfoResponse {
    pub card_id: String,
    pub begin_time: i64,
    pub end_time: i64,
    pub openid: String,
    pub fee: i32,
    pub title: String,
    pub billing_time: i64,
    pub billing_no: String,
    pub billing_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_pdf_media_id: Option<String>,
}

/// 更新报销状态请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReimburseStatusRequest {
    pub card_id: String,
    pub encrypt_code: String,
    pub reimburse_status: String,
}

/// 批量更新报销状态请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchReimburseStatusRequest {
    pub invoice_list: Vec<InvoiceStatusItem>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceStatusItem {
    pub card_id: String,
    pub encrypt_code: String,
    pub reimburse_status: String,
}

/// 批量获取发票信息请求项
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceItem {
    pub card_id: String,
    pub encrypt_code: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BatchInvoiceInfoResponse {
    item_list: Vec<InvoiceInfoResponse>,
}

// -------------------- 极速开发票 --------------------

/// 获取用户填写抬头链接请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserTitleUrlRequest {
    pub s_pappid: String,
    pub order_id: String,
    pub money: i32,
    pub timestamp: i64,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    pub ticket: String,
}

/// 获取用户填写抬头链接响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserTitleUrlResponse {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,
}

/// 获取选择抬头链接请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectTitleUrlRequest {
    pub s_pappid: String,
    pub order_id: String,
    pub money: i32,
    pub timestamp: i64,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    pub ticket: String,
}

/// 获取选择抬头链接响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectTitleUrlResponse {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,
}

/// 抬头信息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TitleInfo {
    pub title: String,
    pub tax_no: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_no: Option<String>,
}

// -------------------- 非税票据 --------------------

/// 获取非税票据授权页链接请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotaxAuthUrlRequest {
    pub s_pappid: String,
    pub order_id: String,
    pub money: i32,
    pub timestamp: i64,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    pub ticket: String,
}

/// 获取非税票据授权页链接响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotaxAuthUrlResponse {
    pub auth_url: String,
    pub expire_time: i32,
}

/// 创建财政电子票据模板请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNotaxCardRequest {
    pub invoice_info: NotaxInvoiceInfo,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotaxInvoiceInfo {
    pub payee: String,
    pub base_info: NotaxBaseInfo,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotaxBaseInfo {
    pub logo_url: String,
}

/// 创建财政电子票据模板响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNotaxCardResponse {
    pub card_id: String,
}

/// 票据插入用户卡包请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertNotaxInvoiceRequest {
    pub order_id: String,
    pub card_id: String,
    pub appid: String,
    pub card_ext: NotaxCardExt,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotaxCardExt {
    pub user_card: NotaxUserCard,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotaxUserCard {
    pub fee: i32,
    pub title: String,
    pub billing_time: i64,
    pub billing_no: String,
    pub billing_code: String,
    pub s_pdf_media_id: String,
}

/// 票据插入用户卡包响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertNotaxInvoiceResponse {
    pub code: String,
    pub openid: String,
}

// -------------------- 通用工具 --------------------

/// 获取ticket响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketResponse {
    pub ticket: String,
    pub expires_in: i32,
}