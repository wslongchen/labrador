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
use crate::wechat::mp::api::Coordinate;
use crate::wechat::mp::WechatMpClient;
use serde::Deserialize;

/// 图像处理模块
#[derive(Debug, Clone)]
pub struct WechatMpImage<'a> {
    client: &'a WechatMpClient,
}

#[allow(unused)]
impl<'a> WechatMpImage<'a> {
    #[inline]
    pub fn new(client: &'a WechatMpClient) -> Self {
        Self { client }
    }

    /// 图片智能裁剪
    ///
    /// 本接口用于对图片主体区域进行智能识别和裁剪。
    /// # 参数说明
    /// * `img_url` - 图片URL
    /// * `ratio` - 裁剪比例，可选值：1:1, 3:4, 4:3, 16:9, 9:16
    pub async fn img_aicrop(
        &self,
        img_url: &str,
        ratio: Option<&str>,
    ) -> LabradorResult<AiCropResponse> {
        let mut url = format!("/cv/img/aicrop?img_url={}", img_url);
        if let Some(r) = ratio {
            url.push_str(&format!("&ratio={}", r));
        }
        let response: WechatApiResponse<AiCropResponse> =
            self.client.wechat_client().get(&url).await?;
        response.into_result()
    }

    /// 图片智能裁剪（通过文件上传）
    pub async fn img_aicrop_file(
        &self,
        file_path: &str,
        ratio: Option<&str>,
    ) -> LabradorResult<AiCropResponse> {
        let (file_name, content) = read_file_with_name(file_path)?;
        let mut url = "/cv/img/aicrop".to_string();
        if let Some(r) = ratio {
            url.push_str(&format!("?ratio={}", r));
        }
        let form = reqwest::multipart::Form::new().part(
            "img",
            reqwest::multipart::Part::bytes(content).file_name(file_name),
        );
        let response: WechatApiResponse<AiCropResponse> = self
            .client
            .wechat_client()
            .post(&url, RequestBody::Multipart(form))
            .await?;
        response.into_result()
    }

    /// 二维码/条码识别
    ///
    /// 识别图片中的二维码、条码、DataMatrix和PDF417。
    pub async fn img_qrcode(&self, img_url: &str) -> LabradorResult<QrCodeResponse> {
        let response: WechatApiResponse<QrCodeResponse> = self
            .client
            .wechat_client()
            .get(&format!("/cv/img/qrcode?img_url={}", img_url))
            .await?;
        response.into_result()
    }

    /// 二维码/条码识别（通过文件上传）
    pub async fn img_qrcode_file(&self, file_path: &str) -> LabradorResult<QrCodeResponse> {
        let (file_name, content) = read_file_with_name(file_path)?;
        let form = reqwest::multipart::Form::new().part(
            "img",
            reqwest::multipart::Part::bytes(content).file_name(file_name),
        );
        let response: WechatApiResponse<QrCodeResponse> = self
            .client
            .wechat_client()
            .post("/cv/img/qrcode", RequestBody::Multipart(form))
            .await?;
        response.into_result()
    }
}

/// 智能裁剪响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiCropResponse {
    pub results: Vec<CropResult>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CropResult {
    pub crop_url: String,
    pub width: f64,
    pub height: f64,
    pub x: f64,
    pub y: f64,
}

/// 二维码识别响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QrCodeResponse {
    pub code_results: Vec<CodeResult>,
    pub img_size: ImageSize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeResult {
    pub type_name: String,
    pub data: String,
    pub pos: CodePosition,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodePosition {
    pub left_top: Coordinate,
    pub right_top: Coordinate,
    pub right_bottom: Coordinate,
    pub left_bottom: Coordinate,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageSize {
    pub w: i32,
    pub h: i32,
}
