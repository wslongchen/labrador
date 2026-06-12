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

// ============================================================================
// 模块：接收消息与事件 (callback.rs)
// 说明：定义微信服务器推送的各类消息和事件的XML结构体。
//       所有结构体均实现了 from_xml 方法，用于反序列化。
// ============================================================================

use crate::errors::LabradorResult;
use serde::{Deserialize, Serialize};

// ==================== 公共基础部分 ====================
// 所有接收消息和事件的XML都包含以下基础字段

/// 接收消息的基础结构，包含所有消息共有的字段
#[derive(Debug, Clone, Deserialize)]
pub struct BaseIncomingMessage {
    /// 开发者微信号
    #[serde(rename = "ToUserName")]
    pub to_user_name: String,
    /// 发送方账号（一个OpenID）
    #[serde(rename = "FromUserName")]
    pub from_user_name: String,
    /// 消息创建时间（整型）
    #[serde(rename = "CreateTime")]
    pub create_time: i64,
    /// 消息类型
    #[serde(rename = "MsgType")]
    pub msg_type: String,
}

// ==================== 1. 普通消息 ====================

/// 文本消息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TextMessage {
    /// 开发者微信号
    pub to_user_name: String,
    /// 发送方账号（一个OpenID）
    pub from_user_name: String,
    /// 消息创建时间（整型）
    pub create_time: i64,
    /// 消息类型，文本为text
    pub msg_type: String,
    /// 文本消息内容
    pub content: String,
    /// 消息id，64位整型
    pub msg_id: i64,
    /// 消息的数据ID（消息如果来自文章时才有）
    pub msg_data_id: Option<String>,
    /// 多图文时第几篇文章，从1开始（消息如果来自文章时才有）
    pub idx: Option<i32>,
}

impl TextMessage {
    /// 从XML字符串解析为TextMessage
    pub fn from_xml(xml: &str) -> LabradorResult<Self> {
        let message: Self = quick_xml::de::from_str(xml)?;
        Ok(message)
    }
}

/// 图片消息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageMessage {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub msg_type: String,
    /// 图片链接（由系统生成）
    pub pic_url: String,
    /// 图片消息媒体id，可以调用获取临时素材接口拉取数据
    pub media_id: String,
    pub msg_id: i64,
    pub msg_data_id: Option<String>,
    pub idx: Option<i32>,
}

impl ImageMessage {
    pub fn from_xml(xml: &str) -> LabradorResult<Self> {
        let message: Self = quick_xml::de::from_str(xml)?;
        Ok(message)
    }
}

/// 语音消息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VoiceMessage {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub msg_type: String,
    /// 语音消息媒体id，Format为amr时返回8K采样率amr语音
    pub media_id: String,
    /// 语音格式，如amr，speex等
    pub format: String,
    pub msg_id: i64,
    pub msg_data_id: Option<String>,
    pub idx: Option<i32>,
    /// 16K采样率语音消息媒体id
    pub media_id_16k: Option<String>,
}

impl VoiceMessage {
    pub fn from_xml(xml: &str) -> LabradorResult<Self> {
        let message: Self = quick_xml::de::from_str(xml)?;
        Ok(message)
    }
}

/// 视频消息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VideoMessage {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub msg_type: String,
    /// 视频消息媒体id
    pub media_id: String,
    /// 视频消息缩略图的媒体id
    pub thumb_media_id: String,
    pub msg_id: i64,
    pub msg_data_id: Option<String>,
    pub idx: Option<i32>,
}

impl VideoMessage {
    pub fn from_xml(xml: &str) -> LabradorResult<Self> {
        let message: Self = quick_xml::de::from_str(xml)?;
        Ok(message)
    }
}

/// 小视频消息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShortVideoMessage {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub msg_type: String,
    pub media_id: String,
    pub thumb_media_id: String,
    pub msg_id: i64,
    pub msg_data_id: Option<String>,
    pub idx: Option<i32>,
}

impl ShortVideoMessage {
    pub fn from_xml(xml: &str) -> LabradorResult<Self> {
        let message: Self = quick_xml::de::from_str(xml)?;
        Ok(message)
    }
}

/// 地理位置消息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LocationMessage {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub msg_type: String,
    /// 地理位置纬度
    pub location_x: f64,
    /// 地理位置经度
    pub location_y: f64,
    /// 地图缩放大小
    pub scale: i32,
    /// 地理位置信息
    pub label: String,
    pub msg_id: i64,
    pub msg_data_id: Option<String>,
    pub idx: Option<i32>,
}

impl LocationMessage {
    pub fn from_xml(xml: &str) -> LabradorResult<Self> {
        let message: Self = quick_xml::de::from_str(xml)?;
        Ok(message)
    }
}

/// 链接消息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LinkMessage {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub msg_type: String,
    /// 消息标题
    pub title: String,
    /// 消息描述
    pub description: String,
    /// 消息链接
    pub url: String,
    pub msg_id: i64,
    pub msg_data_id: Option<String>,
    pub idx: Option<i32>,
}

impl LinkMessage {
    pub fn from_xml(xml: &str) -> LabradorResult<Self> {
        let message: Self = quick_xml::de::from_str(xml)?;
        Ok(message)
    }
}

// ==================== 2. 事件推送 ====================
// 文档: https://developers.weixin.qq.com/doc/service/guide/product/message/Receiving_event_pushes.html

/// 关注/取消关注事件
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubscribeEvent {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub msg_type: String, // 值为 "event"
    /// 事件类型，subscribe(订阅)、unsubscribe(取消订阅)
    pub event: String,
    /// 事件KEY值，qrscene_为前缀，后面为二维码的参数值（用户未关注时，扫描带参数二维码事件会附带）
    pub event_key: Option<String>,
    /// 二维码的ticket，可用来换取二维码图片（用户未关注时，扫描带参数二维码事件会附带）
    pub ticket: Option<String>,
}

impl SubscribeEvent {
    pub fn from_xml(xml: &str) -> LabradorResult<Self> {
        let message: Self = quick_xml::de::from_str(xml)?;
        Ok(message)
    }
}

/// 扫描带参数二维码事件
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScanEvent {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub msg_type: String,
    /// 事件类型，SCAN
    pub event: String,
    /// 事件KEY值，是一个32位无符号整数，即创建二维码时的二维码scene_id
    pub event_key: String,
    /// 二维码的ticket，可用来换取二维码图片
    pub ticket: String,
}

impl ScanEvent {
    pub fn from_xml(xml: &str) -> LabradorResult<Self> {
        let message: Self = quick_xml::de::from_str(xml)?;
        Ok(message)
    }
}

/// 上报地理位置事件
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LocationEvent {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub msg_type: String,
    /// 事件类型，LOCATION
    pub event: String,
    /// 地理位置纬度
    pub latitude: f64,
    /// 地理位置经度
    pub longitude: f64,
    /// 地理位置精度
    pub precision: f64,
}

impl LocationEvent {
    pub fn from_xml(xml: &str) -> LabradorResult<Self> {
        let message: Self = quick_xml::de::from_str(xml)?;
        Ok(message)
    }
}

/// 自定义菜单事件 - 点击菜单拉取消息时的事件推送
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MenuClickEvent {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub msg_type: String,
    /// 事件类型，CLICK
    pub event: String,
    /// 事件KEY值，与自定义菜单接口中KEY值对应
    pub event_key: String,
}

impl MenuClickEvent {
    pub fn from_xml(xml: &str) -> LabradorResult<Self> {
        let message: Self = quick_xml::de::from_str(xml)?;
        Ok(message)
    }
}

/// 自定义菜单事件 - 点击菜单跳转链接时的事件推送
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MenuViewEvent {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub msg_type: String,
    /// 事件类型，VIEW
    pub event: String,
    /// 事件KEY值，设置的跳转URL
    pub event_key: String,
    /// 指菜单ID，如果是个性化菜单，则可以通过这个字段，知道是哪个规则的菜单被点击了
    pub menu_id: Option<String>,
}

impl MenuViewEvent {
    pub fn from_xml(xml: &str) -> LabradorResult<Self> {
        let message: Self = quick_xml::de::from_str(xml)?;
        Ok(message)
    }
}

/// 模板消息发送结果事件推送
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TemplateSendJobFinishEvent {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub msg_type: String,
    /// 事件类型，TEMPLATESENDJOBFINISH
    pub event: String,
    /// 群发的消息ID
    pub msg_id: i64,
    /// 发送状态：success（成功），failed:user block（用户拒收），failed: system failed（系统失败）
    pub status: String,
}

impl TemplateSendJobFinishEvent {
    pub fn from_xml(xml: &str) -> LabradorResult<Self> {
        let message: Self = quick_xml::de::from_str(xml)?;
        Ok(message)
    }
}

// ==================== 3. 群发消息结果事件推送 ====================
// 文档: https://developers.weixin.qq.com/doc/service/guide/product/message/Batch_Sends.html

/// 群发结果事件推送
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MassSendJobFinishEvent {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub msg_type: String,
    /// 事件类型，MASSSENDJOBFINISH
    pub event: String,
    /// 群发的消息ID
    pub msg_id: i64,
    /// 群发的结构，为“send success”或“send fail”或“err(num)”。但send success时，也有可能因用户拒收公众号的消息、系统错误等原因造成少量用户接收失败。err( num)表示审核失败/主动撤回等原因，不同num对应不同原因。
    pub status: String,
    /// 发送的总粉丝数
    pub total_count: i32,
    /// 过滤（过滤是指，有些用户在微信端设置拒收公众号的推送）后，准备发送的粉丝数，原则上等于 sent_count + error_count
    pub filter_count: i32,
    /// 发送成功的粉丝数
    pub sent_count: i32,
    /// 发送失败的粉丝数
    pub error_count: i32,
}

impl MassSendJobFinishEvent {
    pub fn from_xml(xml: &str) -> LabradorResult<Self> {
        let message: Self = quick_xml::de::from_str(xml)?;
        Ok(message)
    }
}

// ==================== 4. 订阅通知事件推送 ====================
// 文档: https://developers.weixin.qq.com/doc/service/guide/product/subscription_messages/push.html

/// 订阅通知发送结果事件推送
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubscribeMsgSendResultEvent {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub msg_type: String,
    /// 事件类型，subscribe_msg_send_result
    pub event: String,
    /// 订阅消息模板ID
    pub template_id: String,
    /// 消息ID
    pub msg_id: i64,
    /// 发送状态：success（成功），failed:user block（用户拒收），failed: system failed（系统失败）
    pub status: String,
}

impl SubscribeMsgSendResultEvent {
    pub fn from_xml(xml: &str) -> LabradorResult<Self> {
        let message: Self = quick_xml::de::from_str(xml)?;
        Ok(message)
    }
}

/// 用户管理订阅状态变更事件推送
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubscribeMsgPopupEvent {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub msg_type: String,
    /// 事件类型，subscribe_msg_popup_event
    pub event: String,
    /// 订阅管理列表
    pub list: Vec<SubscribeItem>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubscribeItem {
    /// 订阅消息模板ID
    pub template_id: String,
    /// 订阅状态：accept（接受），reject（拒绝）
    pub subscribe_status_string: String,
    /// 弹出订阅框的场景值，从1000开始
    pub popup_scene: i32,
}

impl SubscribeMsgPopupEvent {
    pub fn from_xml(xml: &str) -> LabradorResult<Self> {
        let message: Self = quick_xml::de::from_str(xml)?;
        Ok(message)
    }
}

// ==================== 5. 被动回复消息 ====================
// 文档: https://developers.weixin.qq.com/doc/service/guide/product/message/Passive_user_reply_message.html
// 注意：这些是开发者需要回复给微信服务器的XML结构体，需要实现 to_xml 方法

/// 被动回复文本消息
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReplyTextMessage {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub msg_type: String,
    pub content: String,
}

impl ReplyTextMessage {
    pub fn new(to_user: &str, from_user: &str, content: &str) -> Self {
        Self {
            to_user_name: to_user.to_string(),
            from_user_name: from_user.to_string(),
            create_time: chrono::Utc::now().timestamp(),
            msg_type: "text".to_string(),
            content: content.to_string(),
        }
    }

    pub fn to_xml(&self) -> LabradorResult<String> {
        let xml = quick_xml::se::to_string(self)?;
        Ok(xml)
    }
}

/// 被动回复图片消息
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReplyImageMessage {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub msg_type: String,
    pub image: ReplyImage,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReplyImage {
    pub media_id: String,
}

impl ReplyImageMessage {
    pub fn new(to_user: &str, from_user: &str, media_id: &str) -> Self {
        Self {
            to_user_name: to_user.to_string(),
            from_user_name: from_user.to_string(),
            create_time: chrono::Utc::now().timestamp(),
            msg_type: "image".to_string(),
            image: ReplyImage {
                media_id: media_id.to_string(),
            },
        }
    }

    pub fn to_xml(&self) -> LabradorResult<String> {
        let xml = quick_xml::se::to_string(self)?;
        Ok(xml)
    }
}

/// 被动回复语音消息
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReplyVoiceMessage {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub msg_type: String,
    pub voice: ReplyVoice,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReplyVoice {
    pub media_id: String,
}

impl ReplyVoiceMessage {
    pub fn new(to_user: &str, from_user: &str, media_id: &str) -> Self {
        Self {
            to_user_name: to_user.to_string(),
            from_user_name: from_user.to_string(),
            create_time: chrono::Utc::now().timestamp(),
            msg_type: "voice".to_string(),
            voice: ReplyVoice {
                media_id: media_id.to_string(),
            },
        }
    }

    pub fn to_xml(&self) -> LabradorResult<String> {
        let xml = quick_xml::se::to_string(self)?;
        Ok(xml)
    }
}

/// 被动回复视频消息
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReplyVideoMessage {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub msg_type: String,
    pub video: ReplyVideo,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReplyVideo {
    pub media_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
}

impl ReplyVideoMessage {
    pub fn new(
        to_user: &str,
        from_user: &str,
        media_id: &str,
        title: Option<String>,
        description: Option<String>,
    ) -> Self {
        Self {
            to_user_name: to_user.to_string(),
            from_user_name: from_user.to_string(),
            create_time: chrono::Utc::now().timestamp(),
            msg_type: "video".to_string(),
            video: ReplyVideo {
                media_id: media_id.to_string(),
                title,
                description,
            },
        }
    }

    pub fn to_xml(&self) -> LabradorResult<String> {
        let xml = quick_xml::se::to_string(self)?;
        Ok(xml)
    }
}

/// 被动回复音乐消息
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReplyMusicMessage {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub msg_type: String,
    pub music: ReplyMusic,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReplyMusic {
    pub title: Option<String>,
    pub description: Option<String>,
    pub music_url: Option<String>,
    pub hq_music_url: Option<String>,
    pub thumb_media_id: String,
}

impl ReplyMusicMessage {
    pub fn new(to_user: &str, from_user: &str, thumb_media_id: &str) -> Self {
        Self {
            to_user_name: to_user.to_string(),
            from_user_name: from_user.to_string(),
            create_time: chrono::Utc::now().timestamp(),
            msg_type: "music".to_string(),
            music: ReplyMusic {
                title: None,
                description: None,
                music_url: None,
                hq_music_url: None,
                thumb_media_id: thumb_media_id.to_string(),
            },
        }
    }

    pub fn to_xml(&self) -> LabradorResult<String> {
        let xml = quick_xml::se::to_string(self)?;
        Ok(xml)
    }
}

/// 被动回复图文消息
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReplyNewsMessage {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub msg_type: String,
    pub article_count: i32,
    pub articles: ReplyArticles,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReplyArticles {
    pub item: Vec<ReplyNewsItem>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReplyNewsItem {
    pub title: String,
    pub description: String,
    pub pic_url: String,
    pub url: String,
}

impl ReplyNewsMessage {
    pub fn new(to_user: &str, from_user: &str, items: Vec<ReplyNewsItem>) -> Self {
        Self {
            to_user_name: to_user.to_string(),
            from_user_name: from_user.to_string(),
            create_time: chrono::Utc::now().timestamp(),
            msg_type: "news".to_string(),
            article_count: items.len() as i32,
            articles: ReplyArticles { item: items },
        }
    }

    pub fn to_xml(&self) -> LabradorResult<String> {
        let xml = quick_xml::se::to_string(self)?;
        Ok(xml)
    }
}

/// 被动回复消息枚举（便于统一处理）
#[derive(Debug, Clone)]
pub enum ReplyMessage {
    Text(ReplyTextMessage),
    Image(ReplyImageMessage),
    Voice(ReplyVoiceMessage),
    Video(ReplyVideoMessage),
    Music(ReplyMusicMessage),
    News(ReplyNewsMessage),
}

impl ReplyMessage {
    pub fn to_xml(&self) -> LabradorResult<String> {
        match self {
            ReplyMessage::Text(msg) => msg.to_xml(),
            ReplyMessage::Image(msg) => msg.to_xml(),
            ReplyMessage::Voice(msg) => msg.to_xml(),
            ReplyMessage::Video(msg) => msg.to_xml(),
            ReplyMessage::Music(msg) => msg.to_xml(),
            ReplyMessage::News(msg) => msg.to_xml(),
        }
    }
}
