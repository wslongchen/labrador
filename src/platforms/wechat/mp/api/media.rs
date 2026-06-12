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

//! 公众号经常有需要用到一些临时性的多媒体素材的场景，例如在使用接口特别是发送消息时，对多媒体文件、多媒体消息的获取和调用等操作，是通过media_id来进行的。素材管理接口对所有认证的订阅号和服务号开放。通过本接口，公众号可以新增临时素材（即上传临时多媒体文件）。使用接口过程中有任何问题，可以前往微信开放社区 #公众号 专区发帖交流
//!
//! 注意点：
//!
//! 1、临时素材media_id是可复用的。
//!
//! 2、媒体文件在微信后台保存时间为3天，即3天后media_id失效。
//!
//! 3、上传临时素材的格式、大小限制与公众平台官网一致。
//!
//! 图片（image）: 10M，支持PNG\JPEG\JPG\GIF格式
//!
//! 语音（voice）：2M，播放长度不超过60s，支持AMR\MP3格式
//!
//! 视频（video）：10MB，支持MP4格式
//!
//! 缩略图（thumb）：64KB，支持 JPG 格式
//!
//! 4、需使用 https 调用本接口。
use std::fs::File;
use std::io::Read;
use std::path::Path;

use bytes::Bytes;
use serde::Deserialize;
use serde_json::json;

use crate::errors::LabradorResult;
use crate::request::RequestBody;
use crate::utils::string::random_string;
use crate::wechat::client::WechatApiResponse;
use crate::wechat::mp::WechatMpClient;

/// 素材管理模块
#[derive(Clone)]
pub struct WechatMpMedia<'a> {
    client: &'a WechatMpClient,
}

#[allow(unused)]
impl<'a> WechatMpMedia<'a> {
    #[inline]
    pub fn new(client: &'a WechatMpClient) -> WechatMpMedia<'a> {
        WechatMpMedia { client }
    }

    // ==================== 临时素材接口 ====================

    /// 新增临时素材
    ///
    /// 公众号经常有需要用到一些临时性的多媒体素材的场景，例如在使用接口特别是发送消息时，对多媒体文件、多媒体消息的获取和调用等操作，是通过media_id来进行的。
    /// 素材管理接口对所有认证的订阅号和服务号开放。通过本接口，公众号可以新增临时素材（即上传临时多媒体文件）。
    ///
    /// 注意事项：
    /// - 图片（image）: 2M，支持PNG\JPEG\JPG\GIF格式
    /// - 语音（voice）：2M，播放长度不超过60s，支持AMR\MP3格式
    /// - 视频（video）：10MB，支持MP4格式
    /// - 缩略图（thumb）：64KB，支持JPG格式
    ///   媒体文件在后台保存时间为3天，即3天后media_id失效。
    pub async fn upload_media(
        &self,
        media_type: &str,
        file_name: Option<&str>,
        data: Vec<u8>,
    ) -> LabradorResult<TempMediaResponse> {
        let default_file_name = format!("{}.png", random_string(16));
        let file_name = file_name.unwrap_or(&default_file_name);

        let form = reqwest::multipart::Form::new().part(
            "media",
            reqwest::multipart::Part::bytes(data).file_name(file_name.to_string()),
        );

        let response: WechatApiResponse<TempMediaResponse> = self
            .client
            .wechat_client()
            .post(
                &format!("/cgi-bin/media/upload?type={}", media_type),
                RequestBody::Multipart(form),
            )
            .await?;
        response.into_result()
    }

    /// 新增临时素材（通过文件路径）
    ///
    /// 本接口用于通过本地文件路径上传临时素材。便捷方法，内部读取文件后调用 `upload_media`。
    ///
    /// # 参数
    /// * `media_type` - 媒体文件类型：image、voice、video、thumb
    /// * `file_path` - 本地文件的路径
    ///
    /// # 返回
    /// 返回 `LabradorResult<TempMediaResponse>`，包含media_id和素材类型等信息。
    pub async fn upload_media_with_file(
        &self,
        media_type: &str,
        file_path: &str,
    ) -> LabradorResult<TempMediaResponse> {
        let path = Path::new(file_path);
        let file_name = path
            .file_name()
            .and_then(|v| v.to_str())
            .unwrap_or_default();
        let mut file = File::open(path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;
        self.upload_media(media_type, Some(file_name), content)
            .await
    }

    /// 新增临时素材（通过URL）
    ///
    /// 本接口用于通过远程URL下载后上传临时素材。便捷方法，内部下载后调用 `upload_media`。
    ///
    /// # 参数
    /// * `media_type` - 媒体文件类型：image、voice、video、thumb
    /// * `url` - 远程文件的URL地址
    ///
    /// # 返回
    /// 返回 `LabradorResult<TempMediaResponse>`，包含media_id和素材类型等信息。
    pub async fn upload_media_with_url(
        &self,
        media_type: &str,
        url: &str,
    ) -> LabradorResult<TempMediaResponse> {
        let response = reqwest::get(url).await?;
        let content = response.bytes().await?.to_vec();
        self.upload_media(media_type, None, content).await
    }

    /// 获取临时素材
    ///
    /// 公众号可以使用本接口获取临时素材（即下载临时的多媒体文件）。请注意，视频文件不支持https下载，调用该接口需http协议。
    pub async fn get_media(&self, media_id: &str) -> LabradorResult<Bytes> {
        self.client
            .wechat_client()
            .get_bytes(&format!("/cgi-bin/media/get?media_id={}", media_id))
            .await
    }

    /// 获取高清语音素材
    ///
    /// 公众号可以使用本接口获取从JSSDK的uploadVoice接口上传的临时语音素材，格式为speex，16K采样率。
    /// 该音频比上文的临时素材获取接口（格式为amr，8K采样率）更加清晰，适合用作语音识别等对音质要求较高的业务。
    pub async fn get_hd_voice(&self, media_id: &str) -> LabradorResult<Bytes> {
        self.client
            .wechat_client()
            .get_bytes(&format!("/cgi-bin/media/get/jssdk?media_id={}", media_id))
            .await
    }

    // ==================== 永久素材接口 ====================

    /// 新增永久素材（非图文）
    ///
    /// 除了3天就会失效的临时素材外，开发者有时需要永久保存一些素材，届时就可以通过本接口新增永久素材。
    /// 永久图片素材新增后，将带有URL返回给开发者，开发者可以在腾讯系域名内使用。
    ///
    /// 注意事项：
    /// 1、新增的永久素材也可以在公众平台官网素材管理模块中看到
    /// 2、永久素材的数量是有上限的，请谨慎新增。图文消息素材和图片素材的上限为5000，其他类型为1000
    /// 3、素材的格式大小等要求与公众平台官网一致。
    /// 4、新增永久视频素材需特别注意：需要传入video_title和video_introduction
    pub async fn add_material(
        &self,
        media_type: &str,
        filename: &str,
        data: Vec<u8>,
        video_title: Option<&str>,
        video_introduction: Option<&str>,
    ) -> LabradorResult<PermanentMediaResponse> {
        let mut form = reqwest::multipart::Form::new().part(
            "media",
            reqwest::multipart::Part::bytes(data).file_name(filename.to_string()),
        );

        if let (Some(title), Some(intro)) = (video_title, video_introduction) {
            let description = json!({
                "title": title,
                "introduction": intro
            });
            form = form.text("description", description.to_string());
        }

        let response: WechatApiResponse<PermanentMediaResponse> = self
            .client
            .wechat_client()
            .post(
                &format!("/cgi-bin/material/add_material?type={}", media_type),
                RequestBody::Multipart(form),
            )
            .await?;
        response.into_result()
    }

    /// 上传图文消息内的图片获取URL
    ///
    /// 该接口所上传的图片，不占用公众号的素材库中图片数量的100000个的限制，图片仅支持jpg/png格式，大小必须在1MB以下。
    /// 图文消息支持正文中插入自己账号和其他公众号已群发文章链接的能力。
    pub async fn upload_img(
        &self,
        file_name: &str,
        data: Vec<u8>,
    ) -> LabradorResult<UploadImgResponse> {
        let form = reqwest::multipart::Form::new().part(
            "media",
            reqwest::multipart::Part::bytes(data).file_name(file_name.to_string()),
        );

        let response: WechatApiResponse<UploadImgResponse> = self
            .client
            .wechat_client()
            .post("/cgi-bin/media/uploadimg", RequestBody::Multipart(form))
            .await?;
        response.into_result()
    }

    /// 获取永久素材
    ///
    /// 本接口用于根据media_id获取永久素材的详细信息。
    /// 除图文、视频之外，其他类型的素材消息，则响应的直接为素材的内容，开发者可以自行保存为文件。
    pub async fn get_material(&self, media_id: &str) -> LabradorResult<Bytes> {
        self.client
            .wechat_client()
            .post_bytes(
                "/cgi-bin/material/get_material",
                json!({ "media_id": media_id }),
            )
            .await
    }

    /// 获取永久素材（图文信息）
    ///
    /// 专门用于获取图文素材的详细信息。
    pub async fn get_material_news(&self, media_id: &str) -> LabradorResult<NewsMaterialDetail> {
        let response: WechatApiResponse<NewsMaterialDetail> = self
            .client
            .wechat_client()
            .post(
                "/cgi-bin/material/get_material",
                json!({ "media_id": media_id }),
            )
            .await?;
        response.into_result()
    }

    /// 获取永久素材（视频信息）
    ///
    /// 专门用于获取视频素材的标题、描述和下载地址。
    pub async fn get_material_video(&self, media_id: &str) -> LabradorResult<VideoMaterialDetail> {
        let response: WechatApiResponse<VideoMaterialDetail> = self
            .client
            .wechat_client()
            .post(
                "/cgi-bin/material/get_material",
                json!({ "media_id": media_id }),
            )
            .await?;
        response.into_result()
    }

    /// 删除永久素材
    ///
    /// 在新增了永久素材后，开发者可以根据本接口来删除不再需要的永久素材，节省空间。
    /// 请注意：
    /// 1、请谨慎操作本接口，它可以删除公众号在公众平台官网素材管理模块中新建的素材
    /// 2、临时素材无法通过本接口删除
    pub async fn delete_material(&self, media_id: &str) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post(
                "/cgi-bin/material/del_material",
                json!({ "media_id": media_id }),
            )
            .await?;
        Ok(response)
    }

    /// 获取永久素材总数
    ///
    /// 本接口用于获取公众号永久素材的总数信息。
    /// 注意：永久素材的总数包含公众平台官网素材管理中的素材。
    pub async fn get_material_count(&self) -> LabradorResult<MaterialCountResponse> {
        let response: WechatApiResponse<MaterialCountResponse> = self
            .client
            .wechat_client()
            .get("/cgi-bin/material/get_materialcount")
            .await?;
        response.into_result()
    }

    /// 获取永久素材列表
    ///
    /// 分类型获取永久素材列表，包含公众号在官网素材管理模块新建的素材。
    ///
    /// # 参数说明
    /// * `media_type` - 素材的类型，图片（image）、视频（video）、语音（voice）、图文（news）
    /// * `offset` - 从全部素材的该偏移位置开始返回，0表示从第一个素材返回
    /// * `count` - 返回素材的数量，取值在1到20之间
    pub async fn get_material_list(
        &self,
        media_type: MaterialType,
        offset: u32,
        count: u32,
    ) -> LabradorResult<MaterialListResponse> {
        let request = json!({
            "type": media_type.as_str(),
            "offset": offset,
            "count": count,
        });

        let response: WechatApiResponse<MaterialListResponse> = self
            .client
            .wechat_client()
            .post("/cgi-bin/material/batchget_material", request)
            .await?;
        response.into_result()
    }
}

// ============================================================================
// 响应结构体
// ============================================================================

/// 临时素材上传响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TempMediaResponse {
    /// 媒体文件类型，分别有图片（image）、语音（voice）、视频（video）和缩略图（thumb）
    #[serde(rename = "type")]
    pub media_type: String,
    /// 媒体文件上传后，获取时的唯一标识
    pub media_id: String,
    /// 媒体文件上传时间戳
    pub created_at: i64,
    /// 视频素材的缩略图媒体ID（仅视频素材返回）
    pub thumb_media_id: Option<String>,
}

/// 永久素材上传响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermanentMediaResponse {
    /// 媒体文件/图文消息上传后获取的唯一标识
    pub media_id: Option<String>,
    /// 图片、语音、缩略图等素材的URL（仅图片素材返回）
    pub url: Option<String>,
    /// 视频素材的缩略图媒体ID（仅视频素材返回）
    pub thumb_media_id: Option<String>,
}

/// 上传图文图片响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadImgResponse {
    /// 图片URL
    pub url: String,
}

/// 图文素材详情（用于get_material_news）
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewsMaterialDetail {
    /// 图文素材文章列表
    pub news_item: Vec<NewsArticleDetail>,
}

/// 图文文章详情
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewsArticleDetail {
    /// 图文消息的标题
    pub title: String,
    /// 图文消息的封面图片素材id（必须是永久mediaID）
    pub thumb_media_id: String,
    /// 是否显示封面，0为不显示，1为显示
    pub show_cover_pic: i32,
    /// 作者
    pub author: String,
    /// 图文消息的摘要，仅有单图文消息才有摘要，多图文此处为空
    pub digest: String,
    /// 图文消息的具体内容，支持HTML标签
    pub content: String,
    /// 图文页的URL
    pub url: String,
    /// 图文消息的原文地址，即点击“阅读原文”后的URL
    pub content_source_url: String,
}

/// 视频素材详情（用于get_material_video）
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoMaterialDetail {
    /// 视频标题
    pub title: String,
    /// 视频描述
    pub description: String,
    /// 视频下载地址
    pub down_url: String,
}

/// 素材总数响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialCountResponse {
    /// 语音总数量
    pub voice_count: i32,
    /// 视频总数量
    pub video_count: i32,
    /// 图片总数量
    pub image_count: i32,
    /// 图文总数量
    pub news_count: i32,
}

/// 素材列表响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialListResponse {
    /// 该类型的素材的总数
    pub total_count: i32,
    /// 本次调用获取的素材的数量
    pub item_count: i32,
    /// 素材列表
    pub item: Vec<MaterialItem>,
}

/// 素材项
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialItem {
    /// 素材ID
    pub media_id: String,
    /// 素材最后更新时间
    pub update_time: i64,
    /// 图片、语音、视频素材的名字
    pub name: Option<String>,
    /// 图片、语音、视频素材的URL
    pub url: Option<String>,
    /// 图文素材内容
    pub content: Option<NewsMaterialContent>,
}

/// 图文素材内容
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewsMaterialContent {
    /// 图文素材文章列表
    pub news_item: Vec<NewsMaterialItem>,
}

/// 图文素材文章
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewsMaterialItem {
    /// 图文消息的标题
    pub title: String,
    /// 图文消息的封面图片素材id
    pub thumb_media_id: String,
    /// 图文消息的封面图片的地址
    pub thumb_url: String,
    /// 是否显示封面，0为false，1为true
    pub show_cover_pic: i32,
    /// 作者
    pub author: String,
    /// 图文消息的摘要
    pub digest: String,
    /// 图文消息的具体内容
    pub content: String,
    /// 图文页的URL
    pub url: String,
    /// 图文消息的原文地址
    pub content_source_url: String,
}

/// 素材类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialType {
    /// 图片
    Image,
    /// 视频
    Video,
    /// 语音
    Voice,
    /// 图文
    News,
}

impl MaterialType {
    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            MaterialType::Image => "image",
            MaterialType::Video => "video",
            MaterialType::Voice => "voice",
            MaterialType::News => "news",
        }
    }
}
