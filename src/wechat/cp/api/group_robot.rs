use serde::{Serialize, Deserialize};

use crate::{session::SessionStore, request::{RequestType}, WechatCommonResponse, LabradorResult, WechatCpClient, LabraError};
use crate::wechat::cp::constants::{ GROUP_ROBOT_MSG_IMAGE, GROUP_ROBOT_MSG_MARKDOWN, GROUP_ROBOT_MSG_NEWS, GROUP_ROBOT_MSG_TEXT};
use crate::wechat::cp::method::{WechatCpMethod};

/// 微信群机器人消息发送api
/// 文档地址：<a href="https://work.weixin.qq.com/help?doc_id=13376">文档</a>
#[derive(Debug, Clone)]
pub struct WechatCpGroupRobot<'a, T: SessionStore> {
    client: &'a WechatCpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WechatCpGroupRobot<'a, T> {

    #[inline]
    pub fn new(client: &WechatCpClient<T>) -> WechatCpGroupRobot<T> {
        WechatCpGroupRobot {
            client,
        }
    }

    fn get_webhook_url(&self) -> LabradorResult<String> {
        if let Some(webhook_url) = &self.client.webhook_url {
            Ok(webhook_url.to_string())
        } else {
            return Err(LabraError::ApiError("请先设置WebhookKey".to_string()))
        }
    }

    /// <pre>
    /// 发送text类型的消息
    /// </pre>
    pub async fn send_text(&self, content: &str, mentioneds: Vec<String>, mobiles: Vec<String>) -> LabradorResult<WechatCommonResponse> {
        self.send_text_with_url(&self.get_webhook_url()?, content, mentioneds, mobiles).await
    }


    /// <pre>
    /// 发送text类型的消息
    /// </pre>
    pub async fn send_text_with_url(&self, webhook_url: &str, content: &str, mentioneds: Vec<String>, mobiles: Vec<String>) -> LabradorResult<WechatCommonResponse> {
        let req = WechatCpGroupRobotMessage {
            msg_type: GROUP_ROBOT_MSG_TEXT.to_string(),
            content: content.to_string().into(),
            mentioned_list: mentioneds.into(),
            mentioned_mobile_list: mobiles.into(),
            base64: None,
            md5: None,
            articles: None,
            media_id: None
        };
        self.client.post(WechatCpMethod::Custom {need_token: false, method_url: webhook_url.to_string()}, vec![], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 发送markdown类型的消息
    /// </pre>
    pub async fn send_markdown(&self, content: &str) -> LabradorResult<WechatCommonResponse> {
        self.send_markdown_with_url(&self.get_webhook_url()?, content).await
    }


    /// <pre>
    /// 发送markdown类型的消息
    /// </pre>
    pub async fn send_markdown_with_url(&self, webhook_url: &str, content: &str) -> LabradorResult<WechatCommonResponse> {
        let req = WechatCpGroupRobotMessage {
            msg_type: GROUP_ROBOT_MSG_MARKDOWN.to_string(),
            content: content.to_string().into(),
            mentioned_list: None,
            mentioned_mobile_list: None,
            base64: None,
            md5: None,
            articles: None,
            media_id: None
        };
        self.client.post(WechatCpMethod::Custom {need_token: false, method_url: webhook_url.to_string()}, vec![], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 发送image类型的消息
    /// </pre>
    pub async fn send_image(&self, base64: &str, md5: &str) -> LabradorResult<WechatCommonResponse> {
        self.send_image_with_url(&self.get_webhook_url()?, base64, md5).await
    }


    /// <pre>
    /// 发送image类型的消息
    /// </pre>
    pub async fn send_image_with_url(&self, webhook_url: &str, base64: &str, md5: &str) -> LabradorResult<WechatCommonResponse> {
        let req = WechatCpGroupRobotMessage {
            msg_type: GROUP_ROBOT_MSG_IMAGE.to_string(),
            content: None,
            mentioned_list: None,
            mentioned_mobile_list: None,
            base64: base64.to_string().into(),
            md5: md5.to_string().into(),
            articles: None,
            media_id: None
        };
        self.client.post(WechatCpMethod::Custom {need_token: false, method_url: webhook_url.to_string()}, vec![], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 发送news类型的消息
    /// </pre>
    pub async fn send_news(&self, articles: Vec<WechatCpNewArticle>) -> LabradorResult<WechatCommonResponse> {
        self.send_news_with_url(&self.get_webhook_url()?, articles).await
    }


    /// <pre>
    /// 发送news类型的消息
    /// </pre>
    pub async fn send_news_with_url(&self, webhook_url: &str, articles: Vec<WechatCpNewArticle>) -> LabradorResult<WechatCommonResponse> {
        let req = WechatCpGroupRobotMessage {
            msg_type: GROUP_ROBOT_MSG_NEWS.to_string(),
            content: None,
            mentioned_list: None,
            mentioned_mobile_list: None,
            base64: None,
            md5: None,
            articles: articles.into(),
            media_id: None
        };
        self.client.post(WechatCpMethod::Custom {need_token: false, method_url: webhook_url.to_string()}, vec![], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }
}

//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpMenuInfo {
    /// 企业编号
    pub buttons: Vec<WechatCpMenuButton>,
    pub match_rule: Option<WechatCpMenuRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpMenuButton {
    /// <pre>
    /// 菜单的响应动作类型.
    /// view表示网页类型，
    /// click表示点击类型，
    /// miniprogram表示小程序类型
    /// </pre>
    #[serde(rename="type")]
    pub r#type: String,
    /// 菜单标题，不超过16个字节，子菜单不超过60个字节.
    pub name: String,
    /// <pre>
    /// 菜单KEY值，用于消息接口推送，不超过128字节.
    /// click等点击类型必须
    /// </pre>
    pub key: Option<String>,
    /// <pre>
    /// 网页链接.
    /// 用户点击菜单可打开链接，不超过1024字节。type为miniprogram时，不支持小程序的老版本客户端将打开本url。
    /// view、miniprogram类型必须
    /// </pre>
    pub url: Option<String>,
    /// <pre>
    /// 调用新增永久素材接口返回的合法media_id.
    /// media_id类型和view_limited类型必须
    /// </pre>
    pub media_id: Option<String>,
    /// <pre>
    /// 调用发布图文接口获得的article_id.
    /// article_id类型和article_view_limited类型必须
    /// </pre>
    pub article_id: Option<String>,
    /// <pre>
    /// 小程序的appid.
    /// miniprogram类型必须
    /// </pre>
    pub appid: Option<String>,
    /// <pre>
    /// 小程序的页面路径.
    /// miniprogram类型必须
    /// </pre>
    pub pagepath: Option<String>,
    pub sub_button: Vec<WechatCpMenuButton>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpMenuRule {
    pub tag_id: Option<String>,
    pub sex: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    pub client_platform_type: Option<String>,
    pub language: Option<String>,
}


/// 微信群机器人消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpGroupRobotMessage {
    /// 消息类型
    pub msg_type: String,
    /// 文本内容，最长不超过2048个字节，markdown内容，最长不超过4096个字节，必须是utf8编码
    pub content: Option<String>,
    /// userid的列表，提醒群中的指定成员(@某个成员)，@all表示提醒所有人，如果开发者获取不到userid，可以使用mentioned_mobile_list
    pub mentioned_list: Option<Vec<String>>,
    /// 手机号列表，提醒手机号对应的群成员(@某个成员)，@all表示提醒所有人
    pub mentioned_mobile_list: Option<Vec<String>>,
    /// 图片内容的base64编码
    pub base64: Option<String>,
    /// 图片内容（base64编码前）的md5值
    pub md5: Option<String>,
    /// 图文消息，一个图文消息支持1到8条图文
    pub articles: Option<Vec<WechatCpNewArticle>>,
    /// 文件id
    pub media_id: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpNewArticle {
    /// 标题，不超过128个字节，超过会自动截断
    pub title: String,
    /// 描述，不超过512个字节，超过会自动截断
    pub description: String,
    /// 点击后跳转的链接
    pub url: Option<String>,
    /// 图文消息的图片链接，支持JPG、PNG格式，较好的效果为大图1068*455，小图150*150。
    pub pic_url: Option<String>,
    /// 按钮文字，仅在图文数为1条时才生效。 默认为“阅读全文”， 不超过4个文字，超过自动截断。该设置只在企业微信上生效，微工作台（原企业号）上不生效。
    pub btn_text: Option<String>,
    /// 小程序appid，必须是与当前应用关联的小程序，appid和pagepath必须同时填写，填写后会忽略url字段
    pub appid: Option<String>,
    /// 点击消息卡片后的小程序页面，仅限本小程序内的页面。appid和pagepath必须同时填写，填写后会忽略url字段
    pub pagepath: Option<String>,
}
