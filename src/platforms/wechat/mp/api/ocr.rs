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
use crate::errors::LabradorResult;
use crate::request::RequestBody;
use crate::utils::file::read_file_with_name;
use crate::wechat::client::WechatApiResponse;
use crate::wechat::mp::WechatMpClient;
use serde::Deserialize;
use serde_json::Value;
/// 微信OCR识别接口
#[derive(Debug, Clone)]
pub struct WechatMpOcr<'a> {
    client: &'a WechatMpClient,
}

#[allow(unused)]
impl<'a> WechatMpOcr<'a> {
    #[inline]
    pub fn new(client: &'a WechatMpClient) -> Self {
        Self { client }
    }

    // ==================== 菜单识别（新增）====================

    /// 菜单识别（通过图片URL）
    ///
    /// 本接口用于识别纸质菜单。
    pub async fn menu(&self, img_url: &str) -> LabradorResult<MenuOcrResponse> {
        let response: WechatApiResponse<MenuOcrResponse> = self
            .client
            .wechat_client()
            .get(&format!("/cv/ocr/menu?img_url={}", img_url))
            .await?;
        response.into_result()
    }

    /// 菜单识别（通过文件上传）
    pub async fn menu_file(&self, file_path: &str) -> LabradorResult<MenuOcrResponse> {
        let (file_name, content) = read_file_with_name(file_path)?;
        let form = reqwest::multipart::Form::new().part(
            "img",
            reqwest::multipart::Part::bytes(content).file_name(file_name),
        );
        let response: WechatApiResponse<MenuOcrResponse> = self
            .client
            .wechat_client()
            .post("/cv/ocr/menu", RequestBody::Multipart(form))
            .await?;
        response.into_result()
    }

    // ==================== 通用印刷体识别 ====================

    /// 通用印刷体识别（通过图片URL）
    pub async fn comm(&self, img_url: &str) -> LabradorResult<OcrCommResponse> {
        let response: WechatApiResponse<OcrCommResponse> = self
            .client
            .wechat_client()
            .get(&format!("/cv/ocr/comm?img_url={}", img_url))
            .await?;
        response.into_result()
    }

    /// 通用印刷体识别（通过文件上传）
    pub async fn comm_file(&self, file_path: &str) -> LabradorResult<OcrCommResponse> {
        let (file_name, content) = read_file_with_name(file_path)?;
        let form = reqwest::multipart::Form::new().part(
            "img",
            reqwest::multipart::Part::bytes(content).file_name(file_name),
        );
        let response: WechatApiResponse<OcrCommResponse> = self
            .client
            .wechat_client()
            .post("/cv/ocr/comm", RequestBody::Multipart(form))
            .await?;
        response.into_result()
    }

    // ==================== 行驶证识别 ====================

    /// 行驶证识别（通过图片URL）
    pub async fn driving(&self, img_url: &str) -> LabradorResult<OcrDrivingResponse> {
        let response: WechatApiResponse<OcrDrivingResponse> = self
            .client
            .wechat_client()
            .get(&format!("/cv/ocr/driving?img_url={}", img_url))
            .await?;
        response.into_result()
    }

    /// 行驶证识别（通过文件上传）
    pub async fn driving_file(&self, file_path: &str) -> LabradorResult<OcrDrivingResponse> {
        let (file_name, content) = read_file_with_name(file_path)?;
        let form = reqwest::multipart::Form::new().part(
            "img",
            reqwest::multipart::Part::bytes(content).file_name(file_name),
        );
        let response: WechatApiResponse<OcrDrivingResponse> = self
            .client
            .wechat_client()
            .post("/cv/ocr/driving", RequestBody::Multipart(form))
            .await?;
        response.into_result()
    }

    // ==================== 银行卡识别 ====================

    /// 银行卡识别（通过图片URL）
    pub async fn bank_card(&self, img_url: &str) -> LabradorResult<OcrBankCardResponse> {
        let response: WechatApiResponse<OcrBankCardResponse> = self
            .client
            .wechat_client()
            .get(&format!("/cv/ocr/bankcard?img_url={}", img_url))
            .await?;
        response.into_result()
    }

    /// 银行卡识别（通过文件上传）
    pub async fn bank_card_file(&self, file_path: &str) -> LabradorResult<OcrBankCardResponse> {
        let (file_name, content) = read_file_with_name(file_path)?;
        let form = reqwest::multipart::Form::new().part(
            "img",
            reqwest::multipart::Part::bytes(content).file_name(file_name),
        );
        let response: WechatApiResponse<OcrBankCardResponse> = self
            .client
            .wechat_client()
            .post("/cv/ocr/bankcard", RequestBody::Multipart(form))
            .await?;
        response.into_result()
    }

    // ==================== 营业执照识别 ====================

    /// 营业执照识别（通过图片URL）
    pub async fn biz_license(&self, img_url: &str) -> LabradorResult<OcrBizLicenseResponse> {
        let response: WechatApiResponse<OcrBizLicenseResponse> = self
            .client
            .wechat_client()
            .get(&format!("/cv/ocr/bizlicense?img_url={}", img_url))
            .await?;
        response.into_result()
    }

    /// 营业执照识别（通过文件上传）
    pub async fn biz_license_file(&self, file_path: &str) -> LabradorResult<OcrBizLicenseResponse> {
        let (file_name, content) = read_file_with_name(file_path)?;
        let form = reqwest::multipart::Form::new().part(
            "img",
            reqwest::multipart::Part::bytes(content).file_name(file_name),
        );
        let response: WechatApiResponse<OcrBizLicenseResponse> = self
            .client
            .wechat_client()
            .post("/cv/ocr/bizlicense", RequestBody::Multipart(form))
            .await?;
        response.into_result()
    }

    // ==================== 驾驶证识别 ====================

    /// 驾驶证识别（通过图片URL）
    pub async fn driving_license(
        &self,
        img_url: &str,
    ) -> LabradorResult<OcrDrivingLicenseResponse> {
        let response: WechatApiResponse<OcrDrivingLicenseResponse> = self
            .client
            .wechat_client()
            .get(&format!("/cv/ocr/drivinglicense?img_url={}", img_url))
            .await?;
        response.into_result()
    }

    /// 驾驶证识别（通过文件上传）
    pub async fn driving_license_file(
        &self,
        file_path: &str,
    ) -> LabradorResult<OcrDrivingLicenseResponse> {
        let (file_name, content) = read_file_with_name(file_path)?;
        let form = reqwest::multipart::Form::new().part(
            "img",
            reqwest::multipart::Part::bytes(content).file_name(file_name),
        );
        let response: WechatApiResponse<OcrDrivingLicenseResponse> = self
            .client
            .wechat_client()
            .post("/cv/ocr/drivinglicense", RequestBody::Multipart(form))
            .await?;
        response.into_result()
    }

    // ==================== 身份证识别（原有）====================

    /// 身份证OCR识别接口（通过图片URL）
    pub async fn id_card(&self, img_url: &str) -> LabradorResult<OcrIdCardResponse> {
        let response: WechatApiResponse<OcrIdCardResponse> = self
            .client
            .wechat_client()
            .get(&format!("/cv/ocr/idcard?img_url={}", img_url))
            .await?;
        response.into_result()
    }

    /// 身份证OCR识别接口（通过文件上传）
    pub async fn id_card_file(&self, file_path: &str) -> LabradorResult<OcrIdCardResponse> {
        let (file_name, content) = read_file_with_name(file_path)?;
        let form = reqwest::multipart::Form::new().part(
            "img",
            reqwest::multipart::Part::bytes(content).file_name(file_name),
        );
        let response: WechatApiResponse<OcrIdCardResponse> = self
            .client
            .wechat_client()
            .post("/cv/ocr/idcard", RequestBody::Multipart(form))
            .await?;
        response.into_result()
    }
}

// ==================== OCR响应结构体（完善版）====================

/// 菜单识别响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuOcrResponse {
    /// 识别内容（JSON字符串）
    pub content: String,
}

/// 菜单项
#[derive(Debug, Clone, Deserialize)]
pub struct MenuItem {
    /// 菜单名
    pub name: String,
    /// 价格
    pub price: f64,
}

impl MenuOcrResponse {
    /// 解析菜单项
    pub fn parse_menu_items(&self) -> Result<Vec<MenuItem>, serde_json::Error> {
        let parsed: Value = serde_json::from_str(&self.content)?;
        let items = parsed["menu_items"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| {
                        Some(MenuItem {
                            name: v["name"].as_str()?.to_string(),
                            price: v["price"].as_f64()?,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();
        Ok(items)
    }
}

/// 通用印刷体识别响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OcrCommResponse {
    pub img_size: Option<OcrImgSize>,
    pub items: Option<Vec<OcrItem>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OcrItem {
    pub text: String,
    pub pos: OcrPos,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OcrPos {
    pub left_top: Coordinate,
    pub right_top: Coordinate,
    pub right_bottom: Coordinate,
    pub left_bottom: Coordinate,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coordinate {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OcrImgSize {
    pub w: i64,
    pub h: i64,
}

/// 银行卡识别响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OcrBankCardResponse {
    pub number: String,
}

/// 驾驶证识别响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OcrDrivingLicenseResponse {
    pub id_num: Option<String>,
    pub name: Option<String>,
    pub sex: Option<String>,
    pub nationality: Option<String>,
    pub address: Option<String>,
    pub birth_date: Option<String>,
    pub issue_date: Option<String>,
    pub car_class: Option<String>,
    pub valid_from: Option<String>,
    pub valid_to: Option<String>,
    pub official_seal: Option<String>,
}

/// 营业执照识别响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OcrBizLicenseResponse {
    pub reg_num: Option<String>,
    pub serial: Option<String>,
    pub legal_representative: Option<String>,
    pub enterprise_name: Option<String>,
    pub type_of_organization: Option<String>,
    pub address: Option<String>,
    pub type_of_enterprise: Option<String>,
    pub business_scope: Option<String>,
    pub registered_capital: Option<String>,
    pub paid_in_capital: Option<String>,
    pub valid_period: Option<String>,
    pub registered_date: Option<String>,
    pub cert_position: Option<CertPosition>,
    pub img_size: Option<OcrImgSize>,
}

/// 行驶证识别响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OcrDrivingResponse {
    pub plate_num: Option<String>,
    pub vehicle_type: Option<String>,
    pub owner: Option<String>,
    pub addr: Option<String>,
    pub use_character: Option<String>,
    pub model: Option<String>,
    pub vin: Option<String>,
    pub engine_num: Option<String>,
    pub register_date: Option<String>,
    pub issue_date: Option<String>,
    pub plate_num_b: Option<String>,
    pub record: Option<String>,
    pub passengers_num: Option<String>,
    pub total_quality: Option<String>,
    pub prepare_quality: Option<String>,
    pub overall_size: Option<String>,
    pub card_position_front: Option<CardPosition>,
    pub card_position_back: Option<CardPosition>,
    pub img_size: Option<OcrImgSize>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardPosition {
    pub pos: OcrPos,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertPosition {
    pub pos: OcrPos,
}

/// 身份证识别响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OcrIdCardResponse {
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub name: Option<String>,
    pub id: Option<String>,
    pub valid_date: Option<String>,
}
