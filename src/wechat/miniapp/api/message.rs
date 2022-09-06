use serde::{Serialize, Deserialize};
use serde_json::{ Value};

use crate::{session::SessionStore, request::{RequestType}, WechatCommonResponse, LabradorResult};
use crate::wechat::constants::{KEFU_MSGTYPE_IMAGE, KEFU_MSGTYPE_MA_PAGE, KEFU_MSGTYPE_TEXT};
use crate::wechat::miniapp::method::{MaMessageMethod, WechatMaMethod};
use crate::wechat::miniapp::WechatMaClient;


/// 消息发送接口.
#[derive(Debug, Clone)]
pub struct WechatMaMessage<'a, T: SessionStore> {
    client: &'a WechatMaClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WechatMaMessage<'a, T> {

    #[inline]
    pub fn new(client: &WechatMaClient<T>) -> WechatMaMessage<T> {
        WechatMaMessage {
            client,
        }
    }


    /// <pre>
    /// 发送客服消息
    /// 详情请见: <a href="https://developers.weixin.qq.com/miniprogram/dev/api-backend/customerServiceMessage.send.html">发送客服消息</a>
    /// 接口url格式：https://api.weixin.qq.com/cgi-bin/message/custom/send?access_token=ACCESS_TOKEN
    /// </pre>
    pub async fn send_kefu_msg(&self, message: WxMaKefuMsgRequest) -> LabradorResult<WechatCommonResponse> {
        self.client.post(WechatMaMethod::Message(MaMessageMethod::SendCustomMsg), vec![], &message, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 发送订阅消息
    /// https://developers.weixin.qq.com/miniprogram/dev/api-backend/open-api/subscribe-message/subscribeMessage.send.html
    /// </pre>
    pub async fn send_subscribe_msg(&self, data: WxMaSubscribeMsgRequest) -> LabradorResult<WechatCommonResponse> {
        self.client.post(WechatMaMethod::Message(MaMessageMethod::SendSubscribeMsg), vec![], data, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 下发小程序和公众号统一的服务消息
    /// 详情请见: <a href="https://developers.weixin.qq.com/miniprogram/dev/api/open-api/uniform-message/sendUniformMessage.html">下发小程序和公众号统一的服务消息</a>
    /// 接口url格式：https://api.weixin.qq.com/cgi-bin/message/wxopen/template/uniform_send?access_token=ACCESS_TOKEN
    /// </pre>
    pub async fn send_uniform_msg(&self, data: WxMaUniformMsgRequest) -> LabradorResult<WechatCommonResponse> {
        self.client.post(WechatMaMethod::Message(MaMessageMethod::SendUniformTemplate), vec![], &data, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    ///  创建被分享动态消息的 activity_id.
    ///  动态消息: https://developers.weixin.qq.com/miniprogram/dev/framework/open-ability/share/updatable-message.html
    ///
    ///  文档地址：https://developers.weixin.qq.com/miniprogram/dev/api-backend/open-api/updatable-message/updatableMessage.createActivityId.html
    ///  接口地址：GET https://api.weixin.qq.com/cgi-bin/message/wxopen/activityid/create?access_token=ACCESS_TOKEN
    /// </pre>
    pub async fn create_updatable_message_activity_id<D: Serialize>(&self, data: D) -> LabradorResult<Value> {
        self.client.get(WechatMaMethod::Message(MaMessageMethod::CreateActivityId), vec![], RequestType::Json).await?.json::<serde_json::Value>()
    }

    /// <pre>
    ///  修改被分享的动态消息.
    ///  动态消息: https://developers.weixin.qq.com/miniprogram/dev/framework/open-ability/share/updatable-message.html
    ///
    ///  文档地址：https://developers.weixin.qq.com/miniprogram/dev/api-backend/open-api/updatable-message/updatableMessage.setUpdatableMsg.html
    ///  接口地址：POST https://api.weixin.qq.com/cgi-bin/message/wxopen/activityid/create?access_token=ACCESS_TOKEN
    /// </pre>
    pub async fn create_updatable_message<D: Serialize>(&self, data: D) -> LabradorResult<()> {
        self.client.post(WechatMaMethod::Message(MaMessageMethod::SendUpdatableMsg), vec![], data, RequestType::Json).await?.json::<serde_json::Value>()?;
        Ok(())
    }
}


//----------------------------------------------------------------------------------------------------------------------------

/// 客服消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxMaKefuMsgRequest {
    pub touser: String,
    pub msgtype: String,
    pub text: Option<KfText>,
    pub image: Option<KfImage>,
    pub link: Option<KfLink>,
    pub miniprogrampage: Option<KfMaPage>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KfText {
    pub content: Option<String>,
}

#[allow(unused)]
impl KfText {

    pub fn new() -> Self {
        Self {
            content: None,
        }
    }

    fn content(mut self, content: &str) -> Self {
        self.content = content.to_string().into();
        self
    }

    pub fn build_msg(self) -> WxMaKefuMsgRequest {
        WxMaKefuMsgRequest {
            touser: "".to_string(),
            msgtype: KEFU_MSGTYPE_TEXT.to_string(),
            text: self.into(),
            image: None,
            link: None,
            miniprogrampage: None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KfImage {
    pub media_id: Option<String>,
}

#[allow(unused)]
impl KfImage {

    pub fn new() -> Self {
        Self {
            media_id: None,
        }
    }

    fn media_id(mut self, media_id: &str) -> Self {
        self.media_id = media_id.to_string().into();
        self
    }

    pub fn build_msg(self) -> WxMaKefuMsgRequest {
        WxMaKefuMsgRequest {
            touser: "".to_string(),
            msgtype: KEFU_MSGTYPE_IMAGE.to_string(),
            text: None,
            image: self.into(),
            link: None,
            miniprogrampage: None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KfLink {
    pub title: Option<String>,
    pub description: Option<String>,
    pub thumb_url: Option<String>,
    pub url: Option<String>,
}

#[allow(unused)]
impl KfLink {

    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            thumb_url: None,
            url: None
        }
    }

    fn url(mut self, url: &str) -> Self {
        self.url = url.to_string().into();
        self
    }
    fn title(mut self, title: &str) -> Self {
        self.title = title.to_string().into();
        self
    }
    fn thumb_url(mut self, thumb_url: &str) -> Self {
        self.thumb_url = thumb_url.to_string().into();
        self
    }
    fn description(mut self, description: &str) -> Self {
        self.description = description.to_string().into();
        self
    }

    pub fn build_msg(self) -> WxMaKefuMsgRequest {
        WxMaKefuMsgRequest {
            touser: "".to_string(),
            msgtype: KEFU_MSGTYPE_IMAGE.to_string(),
            text: None,
            image: None,
            link: self.into(),
            miniprogrampage: None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KfMaPage {
    pub title: Option<String>,
    pub pagepath: Option<String>,
    pub thumb_media_id: Option<String>,
}

#[allow(unused)]
impl KfMaPage {

    pub fn new() -> Self {
        Self {
            title: None,
            pagepath: None,
            thumb_media_id: None
        }
    }
    
    fn thumb_media_id(mut self, thumb_media_id: &str) -> Self {
        self.thumb_media_id = thumb_media_id.to_string().into();
        self
    }
    fn pagepath(mut self, pagepath: &str) -> Self {
        self.pagepath = pagepath.to_string().into();
        self
    }
    fn title(mut self, title: &str) -> Self {
        self.title = title.to_string().into();
        self
    }

    pub fn build_msg(self) -> WxMaKefuMsgRequest {
        WxMaKefuMsgRequest {
            touser: "".to_string(),
            msgtype: KEFU_MSGTYPE_MA_PAGE.to_string(),
            text: None,
            image: None,
            link: None,
            miniprogrampage: self.into()
        }
    }
}

#[allow(unused)]
impl WxMaKefuMsgRequest {

    fn touser(mut self, touser: &str) -> Self {
        self.touser = touser.to_string().into();
        self
    }

    fn text() -> KfText {
        KfText::new()
    }

    fn image() -> KfImage {
        KfImage::new()
    }

    fn link() -> KfLink {
        KfLink::new()
    }

    fn miniprogram() -> KfMaPage {
        KfMaPage::new()
    }
}




#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxMaSubscribeMsgRequest {
    /// 接收者（用户）的 openid.
    /// <pre>
    /// 参数：touser
    /// 是否必填： 是
    /// 描述： 接收者（用户）的 openid
    /// </pre>
    pub touser: String,
    /// 所需下发的模板消息的id.
    /// <pre>
    /// 参数：template_id
    /// 是否必填： 是
    /// 描述： 所需下发的模板消息的id
    /// </pre>
    pub template_id: String,
    /// 点击模板卡片后的跳转页面，仅限本小程序内的页面.
    /// <pre>
    /// 参数：page
    /// 是否必填： 否
    /// 描述： 点击模板卡片后的跳转页面，仅限本小程序内的页面。支持带参数,（示例index?foo=bar）。该字段不填则模板无跳转。
    /// </pre>
    pub page: Option<String>,
    /// 模板内容，不填则下发空模板.
    /// <pre>
    /// 参数：data
    /// 是否必填： 是
    /// 描述： 模板内容，不填则下发空模板
    /// </pre>
    pub data: Option<Value>,
    /// 跳转小程序类型：developer为开发版；trial为体验版；formal为正式版；默认为正式版
    pub miniprogram_state: Option<String>,
    /// 进入小程序查看的语言类型，支持zh_CN(简体中文)、en_US(英文)、zh_HK(繁体中文)、zh_TW(繁体中文)，默认为zh_CN
    pub lang: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxMaUniformMsgRequest {
    touser: String,
    /// 小程序模板消息相关的信息，可以参考小程序模板消息接口; 有此节点则优先发送小程序模板消息；（小程序模板消息已下线，不用传此节点）
    weapp_template_msg: Option<WeappTemplateMsg>,
    /// 公众号模板消息相关的信息，可以参考公众号模板消息接口；有此节点并且没有weapp_template_msg节点时，发送公众号模板消息
    mp_template_msg: MpTemplateMsg,
}

#[derive(Debug, Clone,Deserialize, Serialize)]
pub struct MsgData {
    name: String,
    value: String,
}


#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct WeappTemplateMsg {
    template_id: String,
    page: String,
    form_id: String,
    data: Value,
    /// 小程序模板放大关键词
    emphasis_keyword: String,
}


#[allow(unused)]
impl WxMaUniformMsgRequest {
    pub fn new<S: Into<String>>(touser: S, weapp_template_msg:  Option<WeappTemplateMsg>, mp_template_msg:MpTemplateMsg) -> WxMaUniformMsgRequest {
        WxMaUniformMsgRequest {
            touser: touser.into(),
            weapp_template_msg,
            mp_template_msg,
            
        }
    }
}


#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct MpTemplateMsg {
    appid: String,
    template_id: String,
    url: String,
    miniprogram: Value,
    data: Value,
}