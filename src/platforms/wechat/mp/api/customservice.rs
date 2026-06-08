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
use crate::wechat::client::WechatApiResponse;
use crate::wechat::mp::WechatMpClient;
use crate::HashAlgorithm;

/// 客服接口.
#[derive(Debug, Clone)]
pub struct WechatMpCustomService<'a> {
    client: &'a WechatMpClient,
}

#[allow(unused)]
impl<'a> WechatMpCustomService<'a> {
    #[inline]
    pub fn new(client: &'a WechatMpClient) -> WechatMpCustomService<'a> {
        WechatMpCustomService { client }
    }

    /// <pre>
    /// 发送客服消息
    /// 详情请见: <a href="https://developers.weixin.qq.com/doc/offiaccount/Message_Management/Service_Center_messages.html">发送客服消息</a>
    /// 接口url格式：https://api.weixin.qq.com/cgi-bin/message/custom/send?access_token=ACCESS_TOKEN
    /// </pre>
    pub async fn send_custom_message(
        &self,
        request: &CustomMessageRequest,
    ) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post(
                "/cgi-bin/message/custom/send",
                serde_json::to_value(request).unwrap(),
            )
            .await?;
        Ok(response)
    }

    /// 客服接口 - 发送文字消息
    pub async fn send_text(
        &self,
        openid: &str,
        content: &str,
    ) -> LabradorResult<WechatApiResponse> {
        self.send_custom_message(&CustomMessageRequest::Text(TextMessage::new(
            openid, content,
        )))
        .await
    }

    /// 客服接口 - 发送图片消息
    pub async fn send_image(
        &self,
        openid: &str,
        media_id: &str,
    ) -> LabradorResult<WechatApiResponse> {
        let req = SendImageRequest::new(openid, media_id);
        self.send_custom_message(&CustomMessageRequest::Image(ImageMessage::new(
            openid, media_id,
        )))
        .await
    }

    /// 客服接口 - 发送声音消息
    pub async fn send_voice(
        &self,
        openid: &str,
        media_id: &str,
    ) -> LabradorResult<WechatApiResponse> {
        let req = SendVoiceRequest::new(openid, media_id);
        self.send_custom_message(&CustomMessageRequest::Voice(VoiceMessage::new(
            openid, media_id,
        )))
        .await
    }

    //*******************客服管理接口***********************//

    /// <pre>
    /// 添加客服账号
    /// 详情请见：<a href="http://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1458044813&token=&lang=zh_CN">客服管理</a>
    /// 接口url格式：https://api.weixin.qq.com/customservice/kfaccount/add?access_token=ACCESS_TOKEN
    /// </pre>
    pub async fn add_account(
        &self,
        account: &str,
        nickname: &str,
        password: &str,
    ) -> LabradorResult<WechatApiResponse> {
        let encrypted_password = HashAlgorithm::Md5.hash_hex(password.as_bytes());
        let data = json!({
            "kf_account": account.to_owned(),
            "nickname": nickname.to_owned(),
            "password": encrypted_password
        });
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/customservice/kfaccount/add", data)
            .await?;
        Ok(response)
    }

    /// <pre>
    /// 设置客服信息（即更新客服信息）
    /// 详情请见：<a href="http://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1458044813&token=&lang=zh_CN">客服管理</a>
    /// 接口url格式：https://api.weixin.qq.com/customservice/kfaccount/update?access_token=ACCESS_TOKEN
    /// </pre>
    pub async fn update_account(
        &self,
        account: &str,
        nickname: &str,
        password: &str,
    ) -> LabradorResult<WechatApiResponse> {
        let encrypted_password = HashAlgorithm::Md5.hash_hex(password.as_bytes());
        let data = json!({
            "kf_account": account.to_owned(),
            "nickname": nickname.to_owned(),
            "password": encrypted_password
        });
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/customservice/kfaccount/update", data)
            .await?;
        Ok(response)
    }

    /// <pre>
    /// 删除客服账号
    /// 详情请见：<a href="http://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1458044813&token=&lang=zh_CN">客服管理</a>
    /// 接口url格式：https://api.weixin.qq.com/customservice/kfaccount/del?access_token=ACCESS_TOKEN&kf_account=KFACCOUNT
    /// </pre>
    pub async fn delete_account(&self, account: &str) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .get(&format!(
                "/customservice/kfaccount/del?kf_account={}",
                account
            ))
            .await?;
        Ok(response)
    }

    /// 获取账号列表
    /// <pre>
    /// 获取客服基本信息
    /// 详情请见：<a href="http://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1458044813&token=&lang=zh_CN">客服管理</a>
    /// 接口url格式：https://api.weixin.qq.com/cgi-bin/customservice/getkflist?access_token=ACCESS_TOKEN
    /// </pre>
    pub async fn get_accounts(&self) -> LabradorResult<Vec<KFAccount>> {
        let response: WechatApiResponse<GetKFListResponse> = self
            .client
            .wechat_client()
            .get("/cgi-bin/customservice/getkflist")
            .await?;
        let data = response.into_result()?;
        Ok(data.kf_list)
    }

    /// <pre>
    /// 获取在线客服接待信息
    /// 详情请见：<a href="http://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1458044813&token=&lang=zh_CN">客服管理</a>
    /// 接口url格式：https://api.weixin.qq.com/cgi-bin/customservice/getonlinekflist?access_token=ACCESS_TOKEN
    /// </pre>
    pub async fn get_online_accounts(&self) -> LabradorResult<Vec<OnlineKFAccount>> {
        let response: WechatApiResponse<GetKFOnlineListResponse> = self
            .client
            .wechat_client()
            .get("/cgi-bin/customservice/getonlinekflist")
            .await?;
        let data = response.into_result()?;
        Ok(data.kf_online_list)
    }
}

//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetKFListResponse {
    pub kf_list: Vec<KFAccount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KFAccount {
    #[serde(rename = "kf_id")]
    pub id: String,
    #[serde(rename = "kf_nick")]
    pub nick: String,
    #[serde(rename = "kf_account")]
    pub account: String,
    #[serde(rename = "kf_headimgurl")]
    pub avatar: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetKFOnlineListResponse {
    pub kf_online_list: Vec<OnlineKFAccount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnlineKFAccount {
    #[serde(rename = "kf_id")]
    pub id: String,
    #[serde(rename = "kf_account")]
    pub account: String,
    pub status: u64,
    #[serde(rename = "auto_accept")]
    pub auto_accept: u64,
    #[serde(rename = "accepted_case")]
    pub accepted_case: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendVoiceRequest {
    openid: String,
    account: Option<String>,
    media_id: String,
}

#[allow(unused)]
impl SendVoiceRequest {
    pub fn new<S: Into<String>>(openid: S, media_id: S) -> SendVoiceRequest {
        SendVoiceRequest {
            openid: openid.into(),
            account: None,
            media_id: media_id.into(),
        }
    }

    pub fn with_account<S: Into<String>>(openid: S, media_id: S, account: S) -> SendVoiceRequest {
        SendVoiceRequest {
            openid: openid.into(),
            account: Some(account.into()),
            media_id: media_id.into(),
        }
    }

    fn to_json(&self) -> Value {
        let mut data = json!({
            "msgtype": "voice",
            "touser": self.openid,
            "voice": {
                "media_id": self.media_id
            }
        });
        if let Some(ref account) = self.account {
            data.as_object_mut().unwrap().insert(
                "customservice".to_string(),
                json!({
                    "kf_account": account
                }),
            );
        }
        data
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendImageRequest {
    openid: String,
    account: Option<String>,
    media_id: String,
}

#[allow(unused)]
impl SendImageRequest {
    pub fn new<S: Into<String>>(openid: S, media_id: S) -> SendImageRequest {
        SendImageRequest {
            openid: openid.into(),
            account: None,
            media_id: media_id.into(),
        }
    }

    pub fn with_account<S: Into<String>>(openid: S, media_id: S, account: S) -> SendImageRequest {
        SendImageRequest {
            openid: openid.into(),
            account: Some(account.into()),
            media_id: media_id.into(),
        }
    }

    fn to_json(&self) -> Value {
        let mut data = json!({
            "msgtype": "image",
            "touser": self.openid,
            "image": {
                "media_id": self.media_id
            }
        });
        if let Some(ref account) = self.account {
            data.as_object_mut().unwrap().insert(
                "customservice".to_string(),
                json!({
                    "kf_account": account
                }),
            );
        }
        data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTextRequest {
    openid: String,
    account: Option<String>,
    content: String,
}

#[allow(unused)]
impl SendTextRequest {
    pub fn new<S: Into<String>>(openid: S, content: S) -> SendTextRequest {
        SendTextRequest {
            openid: openid.into(),
            content: content.into(),
            account: None,
        }
    }

    pub fn with_account<S: Into<String>>(openid: S, content: S, account: S) -> SendTextRequest {
        SendTextRequest {
            openid: openid.into(),
            content: content.into(),
            account: Some(account.into()),
        }
    }

    fn to_json(&self) -> Value {
        let mut data = json!({
            "msgtype": "text",
            "touser": self.openid,
            "text": {
                "content": self.content
            }
        });
        if let Some(ref account) = self.account {
            data.as_object_mut().unwrap().insert(
                "customservice".to_string(),
                json!({
                    "kf_account": account
                }),
            );
        }
        data
    }
}

/// 客服消息请求
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CustomMessageRequest {
    /// 文本消息
    Text(TextMessage),
    /// 图片消息
    Image(ImageMessage),
    /// 语音消息
    Voice(VoiceMessage),
    /// 视频消息
    Video(VideoMessage),
    /// 音乐消息
    Music(MusicMessage),
    /// 图文消息
    News(NewsMessage),
    /// 菜单消息
    Msgmenu(MsgMenuMessage),
    /// 卡券消息
    Wxcard(WxCardMessage),
    /// 小程序卡片消息
    Miniprogrampage(MiniProgramPageMessage),
}

/// 客服消息类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomMessageType {
    /// 文本消息
    Text,
    /// 图片消息
    Image,
    /// 语音消息
    Voice,
    /// 视频消息
    Video,
    /// 音乐消息
    Music,
    /// 图文消息
    News,
    /// 菜单消息
    Msgmenu,
    /// 卡券消息
    Wxcard,
    /// 小程序卡片消息
    Miniprogrampage,
}

/// 文本消息
#[derive(Debug, Clone, Serialize)]
pub struct TextMessage {
    /// 接收者openid
    pub touser: String,
    /// 消息类型
    pub msgtype: CustomMessageType,
    /// 文本消息内容
    pub text: TextContent,
}

/// 文本消息内容
#[derive(Debug, Clone, Serialize)]
pub struct TextContent {
    /// 文本消息内容
    pub content: String,
}

impl TextMessage {
    /// 创建新的文本消息
    pub fn new(touser: &str, content: &str) -> Self {
        Self {
            touser: touser.to_string(),
            msgtype: CustomMessageType::Text,
            text: TextContent {
                content: content.to_string(),
            },
        }
    }
}

/// 图片消息
#[derive(Debug, Clone, Serialize)]
pub struct ImageMessage {
    /// 接收者openid
    pub touser: String,
    /// 消息类型
    pub msgtype: CustomMessageType,
    /// 图片消息内容
    pub image: ImageContent,
}

/// 图片消息内容
#[derive(Debug, Clone, Serialize)]
pub struct ImageContent {
    /// 发送的图片的媒体ID
    pub media_id: String,
}

impl ImageMessage {
    /// 创建新的图片消息
    pub fn new(touser: &str, media_id: &str) -> Self {
        Self {
            touser: touser.to_string(),
            msgtype: CustomMessageType::Image,
            image: ImageContent {
                media_id: media_id.to_string(),
            },
        }
    }
}

/// 语音消息
#[derive(Debug, Clone, Serialize)]
pub struct VoiceMessage {
    /// 接收者openid
    pub touser: String,
    /// 消息类型
    pub msgtype: CustomMessageType,
    /// 语音消息内容
    pub voice: VoiceContent,
}

/// 语音消息内容
#[derive(Debug, Clone, Serialize)]
pub struct VoiceContent {
    /// 发送的语音的媒体ID
    pub media_id: String,
}

impl VoiceMessage {
    /// 创建新的语音消息
    pub fn new(touser: &str, media_id: &str) -> Self {
        Self {
            touser: touser.to_string(),
            msgtype: CustomMessageType::Voice,
            voice: VoiceContent {
                media_id: media_id.to_string(),
            },
        }
    }
}

/// 视频消息
#[derive(Debug, Clone, Serialize)]
pub struct VideoMessage {
    /// 接收者openid
    pub touser: String,
    /// 消息类型
    pub msgtype: CustomMessageType,
    /// 视频消息内容
    pub video: VideoContent,
}

/// 视频消息内容
#[derive(Debug, Clone, Serialize)]
pub struct VideoContent {
    /// 发送的视频的媒体ID
    pub media_id: String,
    /// 视频消息的标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 视频消息的描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl VideoMessage {
    /// 创建新的视频消息
    pub fn new(touser: &str, media_id: &str) -> Self {
        Self {
            touser: touser.to_string(),
            msgtype: CustomMessageType::Video,
            video: VideoContent {
                media_id: media_id.to_string(),
                title: None,
                description: None,
            },
        }
    }

    /// 设置标题
    pub fn with_title(mut self, title: &str) -> Self {
        self.video.title = Some(title.to_string());
        self
    }

    /// 设置描述
    pub fn with_description(mut self, description: &str) -> Self {
        self.video.description = Some(description.to_string());
        self
    }
}

/// 音乐消息
#[derive(Debug, Clone, Serialize)]
pub struct MusicMessage {
    /// 接收者openid
    pub touser: String,
    /// 消息类型
    pub msgtype: CustomMessageType,
    /// 音乐消息内容
    pub music: MusicContent,
}

/// 音乐消息内容
#[derive(Debug, Clone, Serialize)]
pub struct MusicContent {
    /// 音乐标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 音乐描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 音乐链接
    pub musicurl: String,
    /// 高品质音乐链接，wifi环境优先使用该链接播放音乐
    pub hqmusicurl: String,
    /// 缩略图的媒体ID
    pub thumb_media_id: String,
}

impl MusicMessage {
    /// 创建新的音乐消息
    pub fn new(touser: &str, musicurl: &str, hqmusicurl: &str, thumb_media_id: &str) -> Self {
        Self {
            touser: touser.to_string(),
            msgtype: CustomMessageType::Music,
            music: MusicContent {
                title: None,
                description: None,
                musicurl: musicurl.to_string(),
                hqmusicurl: hqmusicurl.to_string(),
                thumb_media_id: thumb_media_id.to_string(),
            },
        }
    }

    /// 设置标题
    pub fn with_title(mut self, title: &str) -> Self {
        self.music.title = Some(title.to_string());
        self
    }

    /// 设置描述
    pub fn with_description(mut self, description: &str) -> Self {
        self.music.description = Some(description.to_string());
        self
    }
}

/// 图文消息
#[derive(Debug, Clone, Serialize)]
pub struct NewsMessage {
    /// 接收者openid
    pub touser: String,
    /// 消息类型
    pub msgtype: CustomMessageType,
    /// 图文消息内容
    pub news: NewsContent,
}

/// 图文消息内容
#[derive(Debug, Clone, Serialize)]
pub struct NewsContent {
    /// 图文消息列表
    pub articles: Vec<Article>,
}

/// 文章
#[derive(Debug, Clone, Serialize)]
pub struct Article {
    /// 标题
    pub title: String,
    /// 描述
    pub description: String,
    /// 点击后跳转的链接
    pub url: String,
    /// 图文消息的图片链接，支持JPG、PNG格式，较好的效果为大图640*320，小图80*80
    pub picurl: String,
}

impl NewsMessage {
    /// 创建新的图文消息
    pub fn new(touser: &str, articles: Vec<Article>) -> Self {
        Self {
            touser: touser.to_string(),
            msgtype: CustomMessageType::News,
            news: NewsContent { articles },
        }
    }
}

/// 菜单消息
#[derive(Debug, Clone, Serialize)]
pub struct MsgMenuMessage {
    /// 接收者openid
    pub touser: String,
    /// 消息类型
    pub msgtype: CustomMessageType,
    /// 菜单消息内容
    pub msgmenu: MsgMenuContent,
}

/// 菜单消息内容
#[derive(Debug, Clone, Serialize)]
pub struct MsgMenuContent {
    /// 菜单标题
    pub head_content: String,
    /// 菜单列表
    pub list: Vec<MsgMenuItem>,
    /// 菜单尾部文本
    pub tail_content: String,
}

/// 菜单项
#[derive(Debug, Clone, Serialize)]
pub struct MsgMenuItem {
    /// 菜单ID
    pub id: String,
    /// 菜单内容
    pub content: String,
}

impl MsgMenuMessage {
    /// 创建新的菜单消息
    pub fn new(
        touser: &str,
        head_content: &str,
        list: Vec<MsgMenuItem>,
        tail_content: &str,
    ) -> Self {
        Self {
            touser: touser.to_string(),
            msgtype: CustomMessageType::Msgmenu,
            msgmenu: MsgMenuContent {
                head_content: head_content.to_string(),
                list,
                tail_content: tail_content.to_string(),
            },
        }
    }
}

/// 卡券消息
#[derive(Debug, Clone, Serialize)]
pub struct WxCardMessage {
    /// 接收者openid
    pub touser: String,
    /// 消息类型
    pub msgtype: CustomMessageType,
    /// 卡券消息内容
    pub wxcard: WxCardContent,
}

/// 卡券消息内容
#[derive(Debug, Clone, Serialize)]
pub struct WxCardContent {
    /// 卡券ID
    pub card_id: String,
    /// 卡券扩展信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_ext: Option<String>,
}

impl WxCardMessage {
    /// 创建新的卡券消息
    pub fn new(touser: &str, card_id: &str) -> Self {
        Self {
            touser: touser.to_string(),
            msgtype: CustomMessageType::Wxcard,
            wxcard: WxCardContent {
                card_id: card_id.to_string(),
                card_ext: None,
            },
        }
    }

    /// 设置卡券扩展信息
    pub fn with_card_ext(mut self, card_ext: &str) -> Self {
        self.wxcard.card_ext = Some(card_ext.to_string());
        self
    }
}

/// 小程序卡片消息
#[derive(Debug, Clone, Serialize)]
pub struct MiniProgramPageMessage {
    /// 接收者openid
    pub touser: String,
    /// 消息类型
    pub msgtype: CustomMessageType,
    /// 小程序卡片消息内容
    pub miniprogrampage: MiniProgramPageContent,
}

/// 小程序卡片消息内容
#[derive(Debug, Clone, Serialize)]
pub struct MiniProgramPageContent {
    /// 小程序卡片的标题
    pub title: String,
    /// 小程序的appid
    pub appid: String,
    /// 小程序的页面路径
    pub pagepath: String,
    /// 小程序卡片图片的媒体ID
    pub thumb_media_id: String,
}

impl MiniProgramPageMessage {
    /// 创建新的小程序卡片消息
    pub fn new(
        touser: &str,
        title: &str,
        appid: &str,
        pagepath: &str,
        thumb_media_id: &str,
    ) -> Self {
        Self {
            touser: touser.to_string(),
            msgtype: CustomMessageType::Miniprogrampage,
            miniprogrampage: MiniProgramPageContent {
                title: title.to_string(),
                appid: appid.to_string(),
                pagepath: pagepath.to_string(),
                thumb_media_id: thumb_media_id.to_string(),
            },
        }
    }
}
