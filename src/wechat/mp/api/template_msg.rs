use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

use crate::{session::SessionStore, request::{RequestType}, WechatCommonResponse, WeChatMpClient, LabradorResult};
use crate::wechat::mp::method::{MpTemplateMessageMethod, WechatMpMethod};


#[derive(Debug, Clone)]
pub struct WeChatMpTemplateMessage<'a, T: SessionStore> {
    client: &'a WeChatMpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WeChatMpTemplateMessage<'a, T> {

    #[inline]
    pub fn new(client: &WeChatMpClient<T>) -> WeChatMpTemplateMessage<T> {
        WeChatMpTemplateMessage {
            client,
        }
    }

    /// 设置所属行业
    /// 设置行业可在微信公众平台后台完成，每月可修改行业1次，帐号仅可使用所属行业中相关的模板，为方便第三方开发者，提供通过接口调用的方式来修改账号所属行业
    /// `industry_id1` 公众号模板消息所属行业编号
    /// `industry_id2` 公众号模板消息所属行业编号
    ///
    /// [地址](https://developers.weixin.qq.com/doc/offiaccount/Message_Management/Template_Message_Interface.html)
    pub async fn set_industry(&self, industry_id1: &str, industry_id2: &str) -> LabradorResult<WechatCommonResponse> {
        self.client.post(WechatMpMethod::TemplateMessage(MpTemplateMessageMethod::SetIndustry), vec![], json!({
            "industry_id1": industry_id1,
            "industry_id2": industry_id2,
        }), RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// 获取设置的行业信息
    /// 获取帐号设置的行业信息。可登录微信公众平台，在公众号后台中查看行业信息。为方便第三方开发者，提供通过接口调用的方式来获取帐号所设置的行业信息
    ///
    /// [地址](https://developers.weixin.qq.com/doc/offiaccount/Message_Management/Template_Message_Interface.html)
    pub async fn get_industry(&self) -> LabradorResult<IndustryResponse> {
        let response = self.client.post(WechatMpMethod::TemplateMessage(MpTemplateMessageMethod::GetIndustry), vec![], Value::Null, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<IndustryResponse>(response)
    }

    /// 发送公众号信息(发送模板消息)
    ///
    /// [地址](https://developers.weixin.qq.com/doc/offiaccount/Message_Management/Template_Message_Interface.html)
    pub async fn send_mp_message(&self, data: TemplateMessage) -> LabradorResult<WechatCommonResponse> {
        self.client.post(WechatMpMethod::TemplateMessage(MpTemplateMessageMethod::SendTemplate), vec![], data, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// 获得模板ID
    /// 从行业模板库选择模板到帐号后台，获得模板 ID 的过程可在微信公众平台后台完成。为方便第三方开发者，提供通过接口调用的方式来获取模板ID
    /// `template_id_short` 模板库中模板的编号，有“TM**”和“OPENTMTM**”等形式
    ///
    /// [地址](https://developers.weixin.qq.com/doc/offiaccount/Message_Management/Template_Message_Interface.html)
    pub async fn get_template_id(&self, template_id_short: &str) -> LabradorResult<String> {
        let response = self.client.post(WechatMpMethod::TemplateMessage(MpTemplateMessageMethod::GetTemplateId), vec![], json!({ "template_id_short": template_id_short }), RequestType::Json).await?.json::<Value>()?;
        let v = WechatCommonResponse::parse::<Value>(response)?;
        let template_id = v["template_id"].as_str().unwrap_or_default();
        Ok(template_id.to_string())
    }

    /// 获得模板列表
    /// 获取已添加至帐号下所有模板列表，可在微信公众平台后台中查看模板列表信息。为方便第三方开发者，提供通过接口调用的方式来获取帐号下所有模板信息
    ///
    /// [地址](https://developers.weixin.qq.com/doc/offiaccount/Message_Management/Template_Message_Interface.html)
    pub async fn get_template_list(&self) -> LabradorResult<Vec<TemplateMessageInfo>> {
        let response = self.client.post(WechatMpMethod::TemplateMessage(MpTemplateMessageMethod::GetTemplateList), vec![], Value::Null, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse_with_key::<Vec<TemplateMessageInfo>>(response, "template_list")
    }

    /// 删除模板
    /// 删除模板可在微信公众平台后台完成，为方便第三方开发者，提供通过接口调用的方式来删除某帐号下的模板
    /// `template_id` 模板编号
    ///
    /// [地址](https://developers.weixin.qq.com/doc/offiaccount/Message_Management/Template_Message_Interface.html)
    pub async fn delete_template(&self, template_id: &str) -> LabradorResult<WechatCommonResponse> {
        self.client.post(WechatMpMethod::TemplateMessage(MpTemplateMessageMethod::DeleteTemplate), vec![], json!({ "template_id": template_id }), RequestType::Json).await?.json::<WechatCommonResponse>()
    }
}


//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMessage {
    pub touser: Option<String>,
    pub template_id: String,
    pub url: Option<String>,
    pub miniprogram: Option<Value>,
    pub data: Value,
}


/// 行业信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryResponse {
    primary_industry: Option<IndustryClass>,
    secondary_industry: Option<IndustryClass>,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryClass {
    first_class: Option<String>,
    second_class: Option<String>,
}


/// 模版信息
#[derive(Debug, Clone, Serialize, Deserialize)]
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

