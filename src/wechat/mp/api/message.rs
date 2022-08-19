use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

use crate::{session::SessionStore, request::{RequestType}, errors::LabraError, util::xmlutil, WechatCommonResponse, WeChatMpClient, LabradorResult};
use crate::wechat::mp::method::{MessageMethod, WechatMpMethod};


#[derive(Debug, Clone)]
pub struct WeChatMessage<'a, T: SessionStore> {
    client: &'a WeChatMpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WeChatMessage<'a, T> {

    #[inline]
    pub fn new(client: &WeChatMpClient<T>) -> WeChatMessage<T> {
        WeChatMessage {
            client,
        }
    }

    /// 发送消息
    pub async fn send<D: Serialize>(&self, data: D) -> LabradorResult<WechatCommonResponse<String>> {
        let v = self.client.post(WechatMpMethod::Message(MessageMethod::Send), data, RequestType::Json).await?.json::<serde_json::Value>()?;
        serde_json::from_value::<WechatCommonResponse<_>>(v).map_err(LabraError::from)
    }

    /// 发送公众号信息
    pub async fn send_mp_message<D: Serialize>(&self, data: D) -> LabradorResult<WechatCommonResponse<String>> {
        let v = self.client.post(WechatMpMethod::Message(MessageMethod::SendTemplate), data, RequestType::Json).await?.json::<serde_json::Value>()?;
        serde_json::from_value::<WechatCommonResponse<_>>(v).map_err(LabraError::from)
    }

    /// 发送服务消息
    pub async fn send_service_message<D: Serialize>(&self, data: D) -> LabradorResult<WechatCommonResponse<String>> {
        let v = self.client.post(WechatMpMethod::Message(MessageMethod::SendUniform), data, RequestType::Json).await?.json::<serde_json::Value>()?;
        serde_json::from_value::<WechatCommonResponse<_>>(v).map_err(LabraError::from)
    }

    /// 发送订阅消息
    pub async fn send_subscribe_message(&self, msg: &SendSubscribeRequest) -> LabradorResult<WechatCommonResponse<String>> {
        let v = self.client.post(WechatMpMethod::Message(MessageMethod::SendSubscribe), msg, RequestType::Json).await?.json::<serde_json::Value>()?;
        serde_json::from_value::<WechatCommonResponse<_>>(v).map_err(LabraError::from)
    }

    /// 发送模板消息
    pub async fn send_template_message(&self, touser: &str, msg: &TemplateMessage) -> LabradorResult<WechatCommonResponse<String>> {
        self.send_mp_message(msg).await
    }

    /// 发送消息
    pub async fn send_message(&self, msg: &TemplateMsg) -> LabradorResult<WechatCommonResponse<String>> {
        self.send_service_message(msg).await
    }


    /// 发送文字消息
    pub async fn send_text(&self, openid: &str, content: &str) -> LabradorResult<WechatCommonResponse<String>> {
        let req = SendTextRequest::new(openid, content);
        self.send(req.to_json()).await
    }

    /// 发送图片消息
    pub async fn send_image(&self, openid: &str, media_id: &str) -> LabradorResult<WechatCommonResponse<String>> {
        let req = SendImageRequest::new(openid, media_id);
        self.send(req.to_json()).await
    }

    /// 发送声音消息
    pub async fn send_voice(&self, openid: &str, media_id: &str) -> LabradorResult<WechatCommonResponse<String>> {
        let req = SendVoiceRequest::new(openid, media_id);
        self.send(req.to_json()).await
    }
}


//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SendSubscribeRequest {
    pub touser: String,
    pub template_id: String,
    pub page: Option<String>,
    pub data: Value,
    pub miniprogram_state: Option<String>,
    pub lang: Option<String>,
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
            "msgtype": "voice".to_owned(),
            "touser": self.openid.to_owned(),
            "voice": {
                "media_id": self.media_id.to_owned()
            }
        });
        if let Some(ref account) = self.account {
            data.as_object_mut().unwrap().insert("customservice".to_string(), json!({
                "kf_account": account.to_owned()
            }));
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
            "msgtype": "image".to_owned(),
            "touser": self.openid.to_owned(),
            "image": {
                "media_id": self.media_id.to_owned()
            }
        });
        if let Some(ref account) = self.account {
            data.as_object_mut().unwrap().insert("customservice".to_string(), json!({
                "kf_account": account.to_owned()
            }));
        }

        data
    }
}


#[derive(Debug, Clone, PartialEq, Eq,Serialize, Deserialize)]
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
            "msgtype": "text".to_owned(),
            "touser": self.openid.to_owned(),
            "text": {
                "content": self.content.to_owned()
            }
        });
        if let Some(ref account) = self.account {
            data.as_object_mut().unwrap().insert("customservice".to_string(), json!({
                "kf_account": account.to_owned()
            }));
        }

        data
    }
}


#[derive(Debug, Clone, PartialEq, Eq,Serialize, Deserialize)]
pub struct WeappTemplateMsg {
    template_id: String,
    page: String,
    form_id: String,
    data: Value,
    /// 小程序模板放大关键词
    emphasis_keyword: String,
}


#[derive(Debug, Clone, PartialEq, Eq,Serialize, Deserialize)]
pub struct TemplateMsg {
    touser: String,
    weapp_template_msg: Option<WeappTemplateMsg>,
    mp_template_msg: MpTemplateMsg,
}

#[allow(unused)]
impl TemplateMsg {
    pub fn new<S: Into<String>>(touser: S, weapp_template_msg:  Option<WeappTemplateMsg>, mp_template_msg:MpTemplateMsg) -> TemplateMsg {
        TemplateMsg {
            touser: touser.into(),
            weapp_template_msg,
            mp_template_msg,
            
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq,Serialize, Deserialize)]
pub struct MpTemplateMsg {
    appid: String,
    template_id: String,
    url: String,
    miniprogram: Value,
    data: Value,
}

#[derive(Debug, Clone, PartialEq, Eq,Serialize, Deserialize)]
pub struct TemplateMessage {
    touser: Option<String>,
    template_id: String,
    url: String,
    miniprogram: Value,
    data: Value,
}


#[derive(Debug, Clone, PartialEq, Eq,Serialize, Deserialize)]
pub struct SubscribeMsg {
    to_user: Option<String>,
    from_user: Option<String>,
    create_time: Option<String>,
    msg_type: Option<String>,
    event: Option<String>,
}

#[allow(unused)]
impl SubscribeMsg {
    fn parse_xml(xml: String) -> Self {
        let package = xmlutil::parse(xml.to_owned());
        let doc = package.as_document();
        let to_user = xmlutil::evaluate(&doc, "//xml/ToUserName/text()").string();
        let from_user = xmlutil::evaluate(&doc, "//xml/FromUserName/text()").string();
        let create_time = xmlutil::evaluate(&doc, "//xml/CreateTime/text()").string();
        let msg_type = xmlutil::evaluate(&doc, "//xml/MsgType/text()").string();
        let event = xmlutil::evaluate(&doc, "//xml/Event/text()").string();
        SubscribeMsg {
            to_user: to_user.into(),
            from_user: from_user.into(),
            create_time: create_time.into(),
            msg_type: msg_type.into(),
            event: event.into(),
        }
    }
}