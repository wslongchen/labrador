//!
//! 模板消息仅用于公众号向用户发送重要的服务通知，只能用于符合其要求的服务场景中，如信用卡刷卡通知，商品购买成功通知等。不支持广告等营销类消息以及其它所有可能对用户造成骚扰的消息。
//!
//! 关于使用规则，请注意：
//!
//! 所有服务号都可以在功能->添加功能插件处看到申请模板消息功能的入口，但只有认证后的服务号才可以申请模板消息的使用权限并获得该权限；
//! 需要选择公众账号服务所处的2个行业，每月可更改1次所选行业；
//! 在所选择行业的模板库中选用已有的模板进行调用；
//! 每个账号可以同时使用25个模板。
//! 当前每个账号的模板消息的日调用上限为10万次，单个模板没有特殊限制。【2014年11月18日将接口调用频率从默认的日1万次提升为日10万次，可在 MP 登录后的开发者中心查看】。当账号粉丝数超过10W/100W/1000W时，模板消息的日调用上限会相应提升，以公众号 MP 后台开发者中心页面中标明的数字为准。
//! 关于接口文档，请注意：
//!
//! 模板消息调用时主要需要模板 ID 和模板中各参数的赋值内容；
//! 模板中参数内容必须以".DATA"结尾，否则视为保留字；
//! 模板保留符号""。
//!
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

use crate::{session::SessionStore, request::{RequestType}, WechatCommonResponse, WeChatMpClient, LabradorResult};
use crate::wechat::mp::method::{MpMessageMethod, WechatMpMethod};


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

    /// 设置所属行业
    /// 设置行业可在微信公众平台后台完成，每月可修改行业1次，帐号仅可使用所属行业中相关的模板，为方便第三方开发者，提供通过接口调用的方式来修改账号所属行业
    /// [`industry_id1`] 公众号模板消息所属行业编号
    /// [`industry_id2`] 公众号模板消息所属行业编号
    ///
    /// [地址](https://developers.weixin.qq.com/doc/offiaccount/Message_Management/Template_Message_Interface.html)
    pub async fn set_industry<D: Serialize>(&self, industry_id1: &str, industry_id2: &str) -> LabradorResult<WechatCommonResponse> {
        self.client.post(WechatMpMethod::Message(MpMessageMethod::SetIndustry), vec![], json!({
            "industry_id1": industry_id1,
            "industry_id2": industry_id2,
        }), RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// 获取设置的行业信息
    /// 获取帐号设置的行业信息。可登录微信公众平台，在公众号后台中查看行业信息。为方便第三方开发者，提供通过接口调用的方式来获取帐号所设置的行业信息
    ///
    /// [地址](https://developers.weixin.qq.com/doc/offiaccount/Message_Management/Template_Message_Interface.html)
    pub async fn get_industry<D: Serialize>(&self) -> LabradorResult<IndustryResponse> {
        let response = self.client.post(WechatMpMethod::Message(MpMessageMethod::GetIndustry), vec![], Value::Null, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<IndustryResponse>(response)
    }

    /// 获得模板ID
    /// 从行业模板库选择模板到帐号后台，获得模板 ID 的过程可在微信公众平台后台完成。为方便第三方开发者，提供通过接口调用的方式来获取模板ID
    /// [`template_id_short`] 模板库中模板的编号，有“TM**”和“OPENTMTM**”等形式
    ///
    /// [地址](https://developers.weixin.qq.com/doc/offiaccount/Message_Management/Template_Message_Interface.html)
    pub async fn get_template_id<D: Serialize>(&self, template_id_short: &str) -> LabradorResult<String> {
        let response = self.client.post(WechatMpMethod::Message(MpMessageMethod::GetTemplateId), vec![], json!({ "template_id_short": template_id_short }), RequestType::Json).await?.json::<Value>()?;
        let v = WechatCommonResponse::parse::<Value>(response)?;
        let template_id = v["template_id"].as_str().unwrap_or_default();
        Ok(template_id.to_string())
    }

    /// 获得模板列表
    /// 获取已添加至帐号下所有模板列表，可在微信公众平台后台中查看模板列表信息。为方便第三方开发者，提供通过接口调用的方式来获取帐号下所有模板信息
    ///
    /// [地址](https://developers.weixin.qq.com/doc/offiaccount/Message_Management/Template_Message_Interface.html)
    pub async fn get_template_list<D: Serialize>(&self) -> LabradorResult<Vec<TemplateMessageInfo>> {
        let response = self.client.post(WechatMpMethod::Message(MpMessageMethod::GetTemplateList), vec![], Value::Null, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse_with_key::<Vec<TemplateMessageInfo>>(response, "template_list")
    }

    /// 删除模板
    /// 删除模板可在微信公众平台后台完成，为方便第三方开发者，提供通过接口调用的方式来删除某帐号下的模板
    /// [`template_id`]
    ///
    /// [地址](https://developers.weixin.qq.com/doc/offiaccount/Message_Management/Template_Message_Interface.html)
    pub async fn delete_template<D: Serialize>(&self, template_id: &str) -> LabradorResult<WechatCommonResponse> {
        self.client.post(WechatMpMethod::Message(MpMessageMethod::DeleteTemplate), vec![], json!({ "template_id": template_id }), RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// 发送公众号信息(发送模板消息)
    ///
    /// [地址](https://developers.weixin.qq.com/doc/offiaccount/Message_Management/Template_Message_Interface.html)
    pub async fn send_mp_message<D: Serialize>(&self, data: TemplateMessage) -> LabradorResult<WechatCommonResponse> {
        self.client.post(WechatMpMethod::Message(MpMessageMethod::SendTemplate), vec![], data, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// 通过 API 推送订阅模板消息给到授权微信用户
    /// [文档](https://developers.weixin.qq.com/doc/offiaccount/Message_Management/One-time_subscription_info.html)
    pub async fn send_subscribe_message(&self, msg: &MpSendSubscribeRequest) -> LabradorResult<WechatCommonResponse> {
        self.client.post(WechatMpMethod::Message(MpMessageMethod::SubscribeMessage), vec![], msg, RequestType::Json).await?.json::<WechatCommonResponse>()
    }


    /// 客服接口 - 发送文字消息
    pub async fn send_text(&self, openid: &str, content: &str) -> LabradorResult<WechatCommonResponse> {
        let req = SendTextRequest::new(openid, content);
        self.send(req.to_json()).await
    }

    /// 客服接口 - 发送图片消息
    pub async fn send_image(&self, openid: &str, media_id: &str) -> LabradorResult<WechatCommonResponse> {
        let req = SendImageRequest::new(openid, media_id);
        self.send(req.to_json()).await
    }

    /// 客服接口 - 发送声音消息
    pub async fn send_voice(&self, openid: &str, media_id: &str) -> LabradorResult<WechatCommonResponse> {
        let req = SendVoiceRequest::new(openid, media_id);
        self.send(req.to_json()).await
    }


    /// 客服接口 - 发消息
    /// <pre>
    /// 当用户和公众号产生特定动作的交互时（具体动作列表请见下方说明），微信将会把消息数据推送给开发者，开发者可以在一段时间内（目前为48小时，2021年6月1日后启用新规则，查看公告）调用客服接口，通过 POST 一个 JSON 数据包来发送消息给普通用户。此接口主要用于客服等有人工消息处理环节的功能，方便开发者为用户提供更加优质的服务。
    ///
    /// 目前允许的动作列表如下（公众平台会根据运营情况更新该列表，不同动作触发后，允许的客服接口下发消息条数不同，下发条数达到上限后，会遇到错误返回码，具体请见返回码说明页）：
    ///
    /// 用户发送信息
    /// 点击自定义菜单（仅有点击推事件、扫码推事件、扫码推事件且弹出“消息接收中”提示框这3种菜单类型是会触发客服接口的）
    /// 关注公众号
    /// 扫描二维码
    /// 支付成功
    /// </pre>
    pub async fn send<D: Serialize>(&self, data: D) -> LabradorResult<WechatCommonResponse> {
        self.client.post(WechatMpMethod::Message(MpMessageMethod::CustomSend), vec![], data, RequestType::Json).await?.json::<WechatCommonResponse>()
    }
}


//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MpSendSubscribeRequest {
    /// 填接收消息的用户openid
    pub touser: String,
    /// 订阅消息模板ID
    pub template_id: String,
    /// 点击消息跳转的链接，需要有 ICP 备案
    pub url: Option<String>,
    /// 跳小程序所需数据，不需跳小程序可不用传该数据
    pub miniprogram	: Option<MiniprogramMsg>,
    /// 消息正文，value为消息内容文本（200字以内），没有固定格式，可用\n换行，color为整段消息内容的字体颜色（目前仅支持整段消息为一种颜色）
    pub data: Value,
    /// 订阅场景值
    pub scene: Option<String>,
    /// 消息标题，15字以内
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq,Serialize, Deserialize)]
pub struct MiniprogramMsg {
    /// 所需跳转到的小程序appid（该小程序 appid 必须与发模板消息的公众号是绑定关联关系，并且小程序要求是已发布的）
    app_id: String,
    /// 所需跳转到小程序的具体页面路径，支持带参数,（示例index?foo=bar）
    pagepath: String,
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
    pub touser: Option<String>,
    pub template_id: String,
    pub url: Option<String>,
    pub miniprogram: Option<Value>,
    pub data: Value,
}


/// 行业信息
#[derive(Debug, Clone, PartialEq, Eq,Serialize, Deserialize)]
pub struct IndustryResponse {
    primary_industry: Option<IndustryClass>,
    secondary_industry: Option<IndustryClass>,
}



#[derive(Debug, Clone, PartialEq, Eq,Serialize, Deserialize)]
pub struct IndustryClass {
    first_class: Option<String>,
    second_class: Option<String>,
}


/// 模版信息
#[derive(Debug, Clone, PartialEq, Eq,Serialize, Deserialize)]
pub struct TemplateMessageInfo {
    /// 模板ID
    template_id: Option<String>,
    /// 模板标题
    title: Option<String>,
    /// 模板所属行业的一级行业
    primary_industry: Option<String>,
    /// 模板所属行业的二级行业
    deputy_industry: Option<String>,
    /// 模板内容
    content: Option<String>,
    /// 模板示例
    example: Option<String>,
}

