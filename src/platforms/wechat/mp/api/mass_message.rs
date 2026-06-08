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
use serde_json::{json, Value};

use crate::errors::LabradorResult;
use crate::request::RequestBody;
use crate::wechat::client::WechatApiResponse;
use crate::wechat::mp::WechatMpClient;

/// 群发消息服务接口
#[derive(Debug, Clone)]
pub struct WechatMpMassMessage<'a> {
    client: &'a WechatMpClient,
}

#[allow(unused)]
impl<'a> WechatMpMassMessage<'a> {
    #[inline]
    pub fn new(client: &'a WechatMpClient) -> Self {
        Self { client }
    }

    /// 上传图文消息内的图片获取URL
    ///
    /// 本接口用于上传图文消息内所需的图片。该接口所上传的图片，不占用公众号的素材库中图片数量的限制。
    /// 图片仅支持jpg/png格式，大小必须在1MB以下。
    ///
    /// # 参数说明
    /// * `file_data` - 图片文件的二进制数据
    /// * `file_name` - 文件名，用于标识
    pub async fn upload_image(
        &self,
        file_data: Vec<u8>,
        file_name: &str,
    ) -> LabradorResult<UploadImageResponse> {
        let form = reqwest::multipart::Form::new().part(
            "media",
            reqwest::multipart::Part::bytes(file_data).file_name(file_name.to_string()),
        );
        let response: WechatApiResponse<UploadImageResponse> = self
            .client
            .wechat_client()
            .post("/cgi-bin/media/uploadimg", RequestBody::Multipart(form))
            .await?;
        response.into_result()
    }

    /// 根据标签进行群发
    ///
    /// 该接口用于向指定标签下的用户群发消息。
    pub async fn send_mass_by_tag(
        &self,
        request: &MassByTagRequest,
    ) -> LabradorResult<MassMessageResponse> {
        let response: WechatApiResponse<MassMessageResponse> = self
            .client
            .wechat_client()
            .post("/cgi-bin/message/mass/sendall", request)
            .await?;
        response.into_result()
    }

    /// 根据OpenID列表进行群发
    ///
    /// 该接口用于向指定的OpenID列表用户群发消息。
    pub async fn send_mass_by_openid(
        &self,
        request: &MassByOpenIdRequest,
    ) -> LabradorResult<MassMessageResponse> {
        let response: WechatApiResponse<MassMessageResponse> = self
            .client
            .wechat_client()
            .post("/cgi-bin/message/mass/send", request)
            .await?;
        response.into_result()
    }

    /// 删除群发消息
    ///
    /// 群发之后，随时可以通过该接口删除群发。只能删除图文消息和视频消息，其他类型的消息一经发送，无法删除。
    ///
    /// # 参数说明
    /// * `msg_id` - 发送出去的消息ID
    /// * `article_idx` - 要删除的文章在图文消息中的位置，第一篇编号为1，不填或填0会删除全部文章
    pub async fn delete_mass_message(
        &self,
        msg_id: i64,
        article_idx: Option<u32>,
    ) -> LabradorResult<WechatApiResponse> {
        let mut req = json!({ "msg_id": msg_id });
        if let Some(idx) = article_idx {
            req["article_idx"] = json!(idx);
        }
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/cgi-bin/message/mass/delete", req)
            .await?;
        Ok(response)
    }

    /// 预览群发消息
    ///
    /// 该接口用于预览群发消息，可通过指定OpenID或微信号进行预览。
    pub async fn preview_mass_message(
        &self,
        request: &MassPreviewRequest,
    ) -> LabradorResult<MassMessageResponse> {
        let response: WechatApiResponse<MassMessageResponse> = self
            .client
            .wechat_client()
            .post("/cgi-bin/message/mass/preview", request)
            .await?;
        response.into_result()
    }

    /// 查询群发消息发送状态
    ///
    /// 该接口用于查询群发消息的发送状态。
    pub async fn get_mass_message_status(&self, msg_id: &str) -> LabradorResult<MassMessageStatus> {
        let response: WechatApiResponse<MassMessageStatus> = self
            .client
            .wechat_client()
            .post("/cgi-bin/message/mass/get", json!({ "msg_id": msg_id }))
            .await?;
        response.into_result()
    }

    /// 设置群发速度
    ///
    /// 该接口用于设置消息的群发速度。
    /// speed 级别：0-4，分别对应 80w/分钟、60w/分钟、45w/分钟、30w/分钟、10w/分钟。
    pub async fn set_mass_speed(&self, speed: i32) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/cgi-bin/message/mass/speed/set", json!({ "speed": speed }))
            .await?;
        Ok(response)
    }

    /// 获取群发速度
    ///
    /// 该接口用于获取当前消息的群发速度。
    pub async fn get_mass_speed(&self) -> LabradorResult<MassSpeedResponse> {
        let response: WechatApiResponse<MassSpeedResponse> = self
            .client
            .wechat_client()
            .post("/cgi-bin/message/mass/speed/get", Value::Null)
            .await?;
        response.into_result()
    }

    /// 上传图文消息素材
    ///
    /// 该接口用于上传图文消息素材，供后续群发使用。
    pub async fn upload_news(
        &self,
        articles: Vec<MpNewsArticle>,
    ) -> LabradorResult<UploadNewsResponse> {
        let response: WechatApiResponse<UploadNewsResponse> = self
            .client
            .wechat_client()
            .post("/cgi-bin/media/uploadnews", json!({ "articles": articles }))
            .await?;
        response.into_result()
    }
}

// ============================================================================
// 请求与响应结构体
// ============================================================================

/// 上传图片响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadImageResponse {
    /// 图片URL
    pub url: String,
}

/// 根据标签群发请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MassByTagRequest {
    /// 用于群发的过滤器
    pub filter: MassFilter,
    /// 消息类型
    pub msgtype: String,
    /// 文本消息内容（当msgtype=text时必填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<MassText>,
    /// 图片/图文/语音/视频/卡券等媒体消息内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<MassMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpnews: Option<MassMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<MassMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<MassMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wxcard: Option<MassWxCard>,
    /// 开发者侧群发任务的唯一标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clientmsgid: Option<String>,
    /// 不经过公众号平台草稿箱，直接发文，默认为false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_ignore_reprint: Option<bool>,
}

/// 群发过滤器
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MassFilter {
    /// 是否向全部用户发送
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_to_all: Option<bool>,
    /// 群发到的标签ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<i32>,
}

/// 群发文本消息
#[derive(Debug, Clone, Serialize)]
pub struct MassText {
    /// 文本内容
    pub content: String,
}

/// 群发媒体消息
#[derive(Debug, Clone, Serialize)]
pub struct MassMedia {
    /// 媒体ID
    pub media_id: String,
}

/// 群发卡券消息
#[derive(Debug, Clone, Serialize)]
pub struct MassWxCard {
    /// 卡券ID
    pub card_id: String,
}

/// 根据OpenID群发请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MassByOpenIdRequest {
    /// 接收消息的用户OpenID列表
    pub touser: Vec<String>,
    /// 消息类型
    pub msgtype: String,
    /// 文本消息内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<MassText>,
    /// 媒体消息内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<MassMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpnews: Option<MassMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<MassMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<MassMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wxcard: Option<MassWxCard>,
    /// 开发者侧群发任务的唯一标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clientmsgid: Option<String>,
}

/// 预览群发请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MassPreviewRequest {
    /// 接收消息的用户OpenID（与towxname二选一）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub touser: Option<String>,
    /// 接收消息的用户微信号（与touser二选一）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub towxname: Option<String>,
    /// 消息类型
    pub msgtype: String,
    /// 文本消息内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<MassText>,
    /// 媒体消息内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<MassMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpnews: Option<MassMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<MassMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<MassMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wxcard: Option<MassWxCard>,
}

/// 群发消息响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MassMessageResponse {
    /// 错误码
    pub errcode: i32,
    /// 错误信息
    pub errmsg: String,
    /// 消息ID
    pub msg_id: i64,
    /// 消息数据ID（当发送消息超过5万/分钟时返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_data_id: Option<String>,
}

/// 群发消息状态响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MassMessageStatus {
    /// 消息ID
    pub msg_id: String,
    /// 消息发送后的状态：SEND_SUCCESS表示发送成功，SENDING表示发送中，SEND_FAIL表示发送失败，DELETE表示已删除
    pub msg_status: String,
}

/// 群发速度响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MassSpeedResponse {
    /// 群发速度的级别，0-4
    pub speed: i32,
    /// 群发速度的真实值，单位：万/分钟
    pub realspeed: i32,
}

/// 图文消息素材
#[derive(Debug, Clone, Serialize)]
pub struct MpNewsArticle {
    /// 图文消息缩略图的media_id
    pub thumb_media_id: String,
    /// 图文消息的作者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// 图文消息的标题
    pub title: String,
    /// 在图文消息页面点击“阅读原文”后的页面链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_source_url: Option<String>,
    /// 图文消息页面的内容，支持HTML标签
    pub content: String,
    /// 图文消息的描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// 是否显示封面，1为显示，0为不显示
    pub show_cover_pic: i32,
    /// 是否打开评论，0不打开，1打开
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_open_comment: Option<i32>,
    /// 是否粉丝才可评论，0所有人可评论，1粉丝才可评论
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_fans_can_comment: Option<i32>,
}

/// 上传图文消息素材响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadNewsResponse {
    /// 媒体文件类型，分别有图片（image）、语音（voice）、视频（video）和缩略图（thumb），图文消息（news）
    pub r#type: String,
    /// 媒体文件/图文消息上传后获取的唯一标识
    pub media_id: String,
    /// 媒体文件上传时间戳
    pub created_at: i64,
}
