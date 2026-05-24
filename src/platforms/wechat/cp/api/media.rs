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
use bytes::Bytes;
use serde::Deserialize;

use crate::errors::LabradorResult;
use crate::request::RequestBody;
use crate::utils::file::read_file_with_name;
use crate::utils::string::random_string;
use crate::wechat::client::WechatApiResponse;
use crate::wechat::cp::WechatCpClient;

/// 企业微信素材管理模块
#[derive(Debug, Clone)]
pub struct WechatCpMedia<'a> {
    client: &'a WechatCpClient,
}

#[allow(unused)]
impl<'a> WechatCpMedia<'a> {
    #[inline]
    pub fn new(client: &'a WechatCpClient) -> Self {
        Self { client }
    }

    /// 上传临时素材
    ///
    /// 上传的多媒体文件有格式和大小限制，如下：
    /// - 图片（image）: 1M，支持JPG格式
    /// - 语音（voice）：2M，播放长度不超过60s，支持AMR\MP3格式
    /// - 视频（video）：10MB，支持MP4格式
    /// - 缩略图（thumb）：64KB，支持JPG格式
    ///
    /// 详情请见: https://developer.work.weixin.qq.com/document/path/90253
    pub async fn upload_media(
        &self,
        media_type: &str,
        file_name: Option<&str>,
        data: Vec<u8>,
    ) -> LabradorResult<MediaUploadResponse> {
        let default_file_name = format!("{}.png", random_string(16));
        let file_name = file_name.unwrap_or(&default_file_name);

        let form = reqwest::multipart::Form::new()
            .part("media", reqwest::multipart::Part::bytes(data).file_name(file_name.to_string()));

        let response: WechatApiResponse<MediaUploadResponse> = self.client
            .post(&format!("/cgi-bin/media/upload?type={}", media_type), RequestBody::Multipart(form))
            .await?;
        response.into_result()
    }

    /// 上传临时素材（通过文件路径）
    pub async fn upload_media_with_file(&self, media_type: &str, file_path: &str) -> LabradorResult<MediaUploadResponse> {
        let (file_name, data) = read_file_with_name(file_path)?;
        self.upload_media(media_type, Some(&file_name), data).await
    }

    /// 上传临时素材（通过URL）
    pub async fn upload_media_with_url(&self, media_type: &str, url: &str) -> LabradorResult<MediaUploadResponse> {
        let response = reqwest::get(url).await?;
        let data = response.bytes().await?.to_vec();
        self.upload_media(media_type, None, data).await
    }

    /// 上传图片
    ///
    /// 上传图片得到图片URL，该URL永久有效。
    /// 返回的图片URL，仅能用于图文消息（mpnews）正文中的图片展示；
    /// 若用于非企业微信域名下的页面，图片将被屏蔽。
    /// 每个企业每天最多可上传100张图片。
    pub async fn upload_img(&self, file_name: &str, data: Vec<u8>) -> LabradorResult<ImageUploadResponse> {
        let form = reqwest::multipart::Form::new()
            .part("media", reqwest::multipart::Part::bytes(data).file_name(file_name.to_string()));

        let response: WechatApiResponse<ImageUploadResponse> = self.client
            .post("/cgi-bin/media/uploadimg", RequestBody::Multipart(form))
            .await?;
        response.into_result()
    }

    /// 上传图片（通过文件路径）
    pub async fn upload_img_with_file(&self, file_path: &str) -> LabradorResult<ImageUploadResponse> {
        let (file_name, data) = read_file_with_name(file_path)?;
        self.upload_img(&file_name, data).await
    }

    /// 上传附件资源
    ///
    /// 用于发送消息时的附件上传，支持图片、文件、视频等。
    /// media_type: 媒体类型，可取值 image、file、video
    /// attachment_type: 附件类型，对应不同的消息类型
    pub async fn upload_attachment(
        &self,
        media_type: &str,
        attachment_type: &str,
        file_name: Option<&str>,
        data: Vec<u8>,
    ) -> LabradorResult<AttachmentUploadResponse> {
        let default_file_name = format!("{}.png", random_string(16));
        let file_name = file_name.unwrap_or(&default_file_name);

        let form = reqwest::multipart::Form::new()
            .part("media", reqwest::multipart::Part::bytes(data).file_name(file_name.to_string()));

        let response: WechatApiResponse<AttachmentUploadResponse> = self.client
            .post(&format!("/cgi-bin/media/upload_attachment?media_type={}&attachment_type={}", media_type, attachment_type), RequestBody::Multipart(form))
            .await?;
        response.into_result()
    }

    /// 上传附件资源（通过文件路径）
    pub async fn upload_attachment_with_file(
        &self,
        media_type: &str,
        attachment_type: &str,
        file_path: &str,
    ) -> LabradorResult<AttachmentUploadResponse> {
        let (file_name, data) = read_file_with_name(file_path)?;
        self.upload_attachment(media_type, attachment_type, Some(&file_name), data).await
    }

    /// 上传附件资源（通过URL）
    pub async fn upload_attachment_with_url(
        &self,
        media_type: &str,
        attachment_type: &str,
        url: &str,
    ) -> LabradorResult<AttachmentUploadResponse> {
        let response = reqwest::get(url).await?;
        let data = response.bytes().await?.to_vec();
        self.upload_attachment(media_type, attachment_type, None, data).await
    }

    /// 获取临时素材
    ///
    /// 用于获取上传后的媒体文件。注意：视频文件不支持下载。
    pub async fn get_media(&self, media_id: &str) -> LabradorResult<Bytes> {
        self.client
            .get_bytes(&format!("/cgi-bin/media/get?media_id={}", media_id))
            .await
    }

    /// 获取高清语音素材
    ///
    /// 获取从JSSDK的uploadVoice接口上传的临时语音素材，格式为speex，16K采样率。
    pub async fn get_hd_voice(&self, media_id: &str) -> LabradorResult<Bytes> {
        self.client
            .get_bytes(&format!("/cgi-bin/media/get/jssdk?media_id={}", media_id))
            .await
    }
}

// ============================================================================
// 请求与响应结构体
// ============================================================================

/// 临时素材上传响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaUploadResponse {
    /// 媒体文件类型，分别有图片（image）、语音（voice）、视频（video）和缩略图（thumb）
    #[serde(rename = "type")]
    pub media_type: String,
    /// 媒体文件上传后获取的唯一标识，3天内有效
    pub media_id: String,
    /// 媒体文件上传时间戳
    pub created_at: String,
}

/// 图片上传响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageUploadResponse {
    /// 图片URL，永久有效
    pub url: String,
}

/// 附件上传响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentUploadResponse {
    /// 媒体文件类型
    #[serde(rename = "type")]
    pub media_type: String,
    /// 媒体文件上传后获取的唯一标识，3天内有效
    pub media_id: String,
    /// 媒体文件上传时间戳
    pub created_at: String,
}