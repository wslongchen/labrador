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

use serde::Deserialize;
use serde_json::Value;

use crate::errors::LabradorResult;
use crate::wechat::client::WechatApiResponse;
use crate::wechat::miniapp::WechatMiniAppClient;

/// 图像处理与文字识别模块
///
/// 提供图像智能裁剪、二维码识别、OCR识别（身份证、银行卡、营业执照等）功能。
/// 注意：`图片高清化`接口因系统维护已下架，此处保留方法但标记为不可用。
#[derive(Debug, Clone)]
pub struct WechatMxaImgOcr<'a> {
    client: &'a WechatMiniAppClient,
}

#[allow(unused)]
impl<'a> WechatMxaImgOcr<'a> {
    #[inline]
    pub fn new(client: &'a WechatMiniAppClient) -> Self {
        Self { client }
    }

    // ========================= 图像处理 =========================

    /// 图片智能裁剪
    ///
    /// 本接口用于对图片主体区域进行智能识别和裁剪，返回裁剪后的图片URL。
    ///
    /// # 参数
    /// * `img_url` - 待裁剪图片的URL地址（需为微信小程序后台配置的合法域名）
    ///
    /// # 返回
    /// 返回 `LabradorResult<AiCropResponse>`，包含裁剪后的图片URL列表及裁剪框坐标。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/img-ocr/img-identify/imgAICrop.html>
    pub async fn img_aicrop(&self, img_url: &str) -> LabradorResult<AiCropResponse> {
        let request = serde_json::json!({
            "img_url": img_url,
        });
        let response: WechatApiResponse<AiCropResponse> = self
            .client
            .wechat_client()
            .post("/cv/img/aicrop", request)
            .await?;
        response.into_result()
    }

    /// 二维码/条码识别
    ///
    /// 识别图片中的二维码、条码、DataMatrix和PDF417，返回识别结果列表。
    ///
    /// # 参数
    /// * `img_url` - 包含条码/二维码的图片URL地址
    ///
    /// # 返回
    /// 返回 `LabradorResult<QrCodeResponse>`，包含识别出的所有码信息（类型、数据、位置）和图片大小。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/img-ocr/img-identify/QRCodeIdentify.html>
    pub async fn img_qrcode(&self, img_url: &str) -> LabradorResult<QrCodeResponse> {
        let request = serde_json::json!({
            "img_url": img_url,
        });
        let response: WechatApiResponse<QrCodeResponse> = self
            .client
            .wechat_client()
            .post("/cv/img/qrcode", request)
            .await?;
        response.into_result()
    }

    /// 图片高清化（已下架）
    ///
    /// **注意：由于系统维护原因，该接口已下架，调用将返回错误。**
    /// 如有需要使用，可前往微信开放社区发帖或联系微信服务市场客服。
    /// 此处保留方法仅为历史兼容，不建议使用。
    #[deprecated = "该接口已下架，无法使用"]
    pub async fn img_superresolution(&self, _img_url: &str) -> LabradorResult<Value> {
        // 直接返回错误或调用时由微信服务器返回错误
        // 此处我们简单调用，期望服务器返回明确错误
        let request = serde_json::json!({
            "img_url": _img_url,
        });
        let response: WechatApiResponse<Value> = self
            .client
            .wechat_client()
            .post("/cv/img/superresolution", request)
            .await?;
        response.into_result()
    }

    // ========================= OCR 识别 =========================

    /// 通用印刷体识别
    ///
    /// 本接口用于识别图片中的通用印刷体文字。
    ///
    /// # 参数
    /// * `img_url` - 待识别图片的URL地址
    ///
    /// # 返回
    /// 返回 `LabradorResult<OcrCommResponse>`，包含识别出的文字项列表及每个文字的位置信息。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/img-ocr/ocr/printedTextOCR.html>
    pub async fn ocr_comm(&self, img_url: &str) -> LabradorResult<OcrCommResponse> {
        let request = serde_json::json!({
            "img_url": img_url,
        });
        let response: WechatApiResponse<OcrCommResponse> = self
            .client
            .wechat_client()
            .post("/cv/ocr/comm", request)
            .await?;
        response.into_result()
    }

    /// 行驶证识别
    ///
    /// 提供机动车行驶证信息OCR识别，可识别正副页的车辆信息。
    ///
    /// # 参数
    /// * `img_url` - 行驶证图片的URL地址
    ///
    /// # 返回
    /// 返回 `LabradorResult<OcrDrivingResponse>`，包含车牌号、车辆类型、所有人、品牌型号等行驶证信息。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/img-ocr/ocr/drivingLicenseOCR.html>
    pub async fn ocr_driving(&self, img_url: &str) -> LabradorResult<OcrDrivingResponse> {
        let request = serde_json::json!({
            "img_url": img_url,
        });
        let response: WechatApiResponse<OcrDrivingResponse> = self
            .client
            .wechat_client()
            .post("/cv/ocr/driving", request)
            .await?;
        response.into_result()
    }

    /// 银行卡识别
    ///
    /// 本接口提供银行卡卡面信息OCR识别，可识别银行卡号。
    ///
    /// # 参数
    /// * `img_url` - 银行卡图片的URL地址
    ///
    /// # 返回
    /// 返回 `LabradorResult<OcrBankcardResponse>`，包含识别出的银行卡号。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/img-ocr/ocr/bankCardOCR.html>
    pub async fn ocr_bankcard(&self, img_url: &str) -> LabradorResult<OcrBankcardResponse> {
        let request = serde_json::json!({
            "img_url": img_url,
        });
        let response: WechatApiResponse<OcrBankcardResponse> = self
            .client
            .wechat_client()
            .post("/cv/ocr/bankcard", request)
            .await?;
        response.into_result()
    }

    /// 营业执照识别
    ///
    /// 本接口提供营业执照 OCR 识别能力，可识别企业名称、注册号、经营范围等信息。
    ///
    /// # 参数
    /// * `img_url` - 营业执照图片的URL地址
    ///
    /// # 返回
    /// 返回 `LabradorResult<OcrBizLicenseResponse>`，包含注册号、企业名称、法定代表人、
    /// 经营范围、注册资本等营业执照信息。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/img-ocr/ocr/businessLicenseOCR.html>
    pub async fn ocr_bizlicense(&self, img_url: &str) -> LabradorResult<OcrBizLicenseResponse> {
        let request = serde_json::json!({
            "img_url": img_url,
        });
        let response: WechatApiResponse<OcrBizLicenseResponse> = self
            .client
            .wechat_client()
            .post("/cv/ocr/bizlicense", request)
            .await?;
        response.into_result()
    }

    /// 驾驶证识别
    ///
    /// 本接口用于驾驶证信息OCR识别，可识别证号、姓名、准驾车型等信息。
    ///
    /// # 参数
    /// * `img_url` - 驾驶证图片的URL地址
    ///
    /// # 返回
    /// 返回 `LabradorResult<OcrDrivingLicenseResponse>`，包含证号、姓名、性别、国籍、
    /// 出生日期、准驾车型、有效期限等驾驶证信息。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/img-ocr/ocr/driverLicenseOCR.html>
    pub async fn ocr_drivinglicense(
        &self,
        img_url: &str,
    ) -> LabradorResult<OcrDrivingLicenseResponse> {
        let request = serde_json::json!({
            "img_url": img_url,
        });
        let response: WechatApiResponse<OcrDrivingLicenseResponse> = self
            .client
            .wechat_client()
            .post("/cv/ocr/drivinglicense", request)
            .await?;
        response.into_result()
    }

    /// 身份证识别
    ///
    /// 本接口提供身份证正反面OCR识别功能，可识别姓名、身份证号、有效日期等信息。
    ///
    /// # 参数
    /// * `img_url` - 身份证图片的URL地址
    ///
    /// # 返回
    /// 返回 `LabradorResult<OcrIdCardResponse>`，包含类型（正面/背面）、姓名、身份证号和有效日期。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/img-ocr/ocr/idCardOCR.html>
    pub async fn ocr_idcard(&self, img_url: &str) -> LabradorResult<OcrIdCardResponse> {
        let request = serde_json::json!({
            "img_url": img_url,
        });
        let response: WechatApiResponse<OcrIdCardResponse> = self
            .client
            .wechat_client()
            .post("/cv/ocr/idcard", request)
            .await?;
        response.into_result()
    }
}

// ============================================================================
// 响应结构体
// 注意：使用 #[serde(rename_all = "camelCase")] 处理驼峰字段。
// ============================================================================

// -------------------- 图像处理 响应结构体 --------------------

/// 图片智能裁剪响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiCropResponse {
    /// 裁剪后图片的URL地址
    pub results: Vec<CropResult>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CropResult {
    /// 裁剪后的图片URL
    pub crop_url: String,
    /// 裁剪框的宽度
    pub width: f64,
    /// 裁剪框的高度
    pub height: f64,
    /// 裁剪框左上角x坐标（相对原图）
    pub x: f64,
    /// 裁剪框左上角y坐标（相对原图）
    pub y: f64,
}

/// 二维码/条码识别响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QrCodeResponse {
    /// 识别出的码信息列表
    pub code_results: Vec<CodeResult>,
    /// 图片大小信息
    pub img_size: ImageSize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeResult {
    /// 识别出的码类型：QR_CODE, BARCODE, DATAMATRIX, PDF417
    pub type_name: String,
    /// 识别出的码数据
    pub data: String,
    /// 码在图片中的位置（四个角坐标）
    pub pos: CodePosition,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodePosition {
    /// 左上角坐标
    pub left_top: Coordinate,
    /// 右上角坐标
    pub right_top: Coordinate,
    /// 右下角坐标
    pub right_bottom: Coordinate,
    /// 左下角坐标
    pub left_bottom: Coordinate,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coordinate {
    /// x坐标
    pub x: i32,
    /// y坐标
    pub y: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageSize {
    /// 图片宽度
    pub w: i32,
    /// 图片高度
    pub h: i32,
}

// -------------------- OCR 响应结构体 --------------------

/// 通用印刷体识别响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OcrCommResponse {
    /// 图片大小信息
    pub img_size: ImageSize,
    /// 识别出的文字项列表
    pub items: Vec<OcrCommItem>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OcrCommItem {
    /// 识别出的文本
    pub text: String,
    /// 文本在图片中的位置
    pub pos: OcrItemPosition,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OcrItemPosition {
    /// 左上角坐标
    pub left_top: Coordinate,
    /// 右上角坐标
    pub right_top: Coordinate,
    /// 右下角坐标
    pub right_bottom: Coordinate,
    /// 左下角坐标
    pub left_bottom: Coordinate,
}

/// 行驶证识别响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OcrDrivingResponse {
    /// 车牌号码
    pub plate_num: Option<String>,
    /// 车辆类型
    pub vehicle_type: Option<String>,
    /// 所有人
    pub owner: Option<String>,
    /// 住址
    pub addr: Option<String>,
    /// 使用性质
    pub use_character: Option<String>,
    /// 品牌型号
    pub model: Option<String>,
    /// 车辆识别代号
    pub vin: Option<String>,
    /// 发动机号码
    pub engine_num: Option<String>,
    /// 注册日期
    pub register_date: Option<String>,
    /// 发证日期
    pub issue_date: Option<String>,
    /// 车牌号码（背面）
    pub plate_num_b: Option<String>,
    /// 号牌
    pub record: Option<String>,
    /// 核定载人数
    pub passengers_num: Option<String>,
    /// 总质量
    pub total_quality: Option<String>,
    /// 整备质量
    pub prepare_quality: Option<String>,
    /// 外廓尺寸
    pub overall_size: Option<String>,
    /// 卡片正面位置
    pub card_position_front: Option<CardPosition>,
    /// 卡片反面位置
    pub card_position_back: Option<CardPosition>,
    /// 图片大小
    pub img_size: Option<ImageSize>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardPosition {
    /// 位置坐标
    pub pos: OcrItemPosition,
}

/// 银行卡识别响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OcrBankcardResponse {
    /// 银行卡号
    pub number: String,
}

/// 营业执照识别响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OcrBizLicenseResponse {
    /// 注册号
    pub reg_num: Option<String>,
    /// 编号
    pub serial: Option<String>,
    /// 法定代表人姓名
    pub legal_representative: Option<String>,
    /// 企业名称
    pub enterprise_name: Option<String>,
    /// 组成形式
    pub type_of_organization: Option<String>,
    /// 经营场所/企业住所
    pub address: Option<String>,
    /// 公司类型
    pub type_of_enterprise: Option<String>,
    /// 经营范围
    pub business_scope: Option<String>,
    /// 注册资本
    pub registered_capital: Option<String>,
    /// 实收资本
    pub paid_in_capital: Option<String>,
    /// 营业期限
    pub valid_period: Option<String>,
    /// 注册日期/成立日期
    pub registered_date: Option<String>,
    /// 营业执照位置
    pub cert_position: Option<CertPosition>,
    /// 图片大小
    pub img_size: Option<ImageSize>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertPosition {
    /// 位置坐标
    pub pos: OcrItemPosition,
}

/// 驾驶证识别响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OcrDrivingLicenseResponse {
    /// 证号
    pub id_num: Option<String>,
    /// 姓名
    pub name: Option<String>,
    /// 性别
    pub sex: Option<String>,
    /// 国籍
    pub nationality: Option<String>,
    /// 住址
    pub address: Option<String>,
    /// 出生日期
    pub birth_date: Option<String>,
    /// 初次领证日期
    pub issue_date: Option<String>,
    /// 准驾车型
    pub car_class: Option<String>,
    /// 有效期限起始日
    pub valid_from: Option<String>,
    /// 有效期限终止日
    pub valid_to: Option<String>,
    /// 印章文字
    pub official_seal: Option<String>,
}

/// 身份证识别响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OcrIdCardResponse {
    /// 类型：Front-正面，Back-背面
    pub r#type: String,
    /// 姓名（正面）
    pub name: Option<String>,
    /// 身份证号（正面）
    pub id: Option<String>,
    /// 有效日期（背面）
    pub valid_date: Option<String>,
}
