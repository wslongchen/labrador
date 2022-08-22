use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

use crate::{session::SessionStore, request::{RequestType}, WechatCommonResponse, WeChatMpClient, LabradorResult};
use crate::wechat::mp::method::{MpSubscribeMessageMethod, WechatMpMethod};

/// 订阅消息服务接口
#[derive(Debug, Clone)]
pub struct WeChatMpSubscribeMessage<'a, T: SessionStore> {
    client: &'a WeChatMpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WeChatMpSubscribeMessage<'a, T> {

    #[inline]
    pub fn new(client: &WeChatMpClient<T>) -> WeChatMpSubscribeMessage<T> {
        WeChatMpSubscribeMessage {
            client,
        }
    }

    /// <pre>
    /// 构造用户订阅一条模板消息授权的url连接
    /// 详情请见: https://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1500374289_66bvB
    /// </pre>
    pub async fn subscribe_message_authorization_url(&self, redirect_uri: &str, scene: i32, reserved: &str) -> String {
        format!("{}?action=get_confirm&appid={}&scene={}&template_id={}&redirect_url={}&reserved={}#wechat_redirect", MpSubscribeMessageMethod::SubscribeAuthorizeUrl.get_method(),
                          self.client.appid, scene, self.client.template_id.to_owned().unwrap_or_default(), urlencoding::encode(redirect_uri), reserved)
    }

    /// <pre>
    /// 发送一次性订阅消息
    /// 详情请见: https://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1500374289_66bvB
    /// </pre>
    pub async fn send_subscribe_message_once(&self, msg: &MpSendSubscribeOnceRequest) -> LabradorResult<WechatCommonResponse> {
        self.client.post(WechatMpMethod::SubscribeMessage(MpSubscribeMessageMethod::SubscribeMessageOnce), vec![], msg, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 获取帐号所属类目下的公共模板标题
    ///
    /// 详情请见: <a href="https://developers.weixin.qq.com/miniprogram/dev/api-backend/open-api/subscribe-message/subscribeMessage.getPubTemplateTitleList.html">获取帐号所属类目下的公共模板标题</a>
    /// 接口url格式: https://api.weixin.qq.com/wxaapi/newtmpl/getpubtemplatetitles?access_token=ACCESS_TOKEN
    /// </pre>
    pub async fn get_pub_template_title_list(&self, ids: Vec<&str>, start: i32, limit: i32) -> LabradorResult<WechatMpPubTemplateTitleListResponse> {
        let ids = ids.join(",").to_string();
        let start = start.to_string();
        let limit = limit.to_string();
        let params = vec![( "ids", ids.as_str()),
          ("start", start.as_str()),
          ("limit", limit.as_str())];
        let v = self.client.get(WechatMpMethod::SubscribeMessage(MpSubscribeMessageMethod::GetPubTemplateTitles), params, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatMpPubTemplateTitleListResponse>(v)
    }

    /// <pre>
    /// 获取模板库某个模板标题下关键词库
    ///
    /// 详情请见: https://developers.weixin.qq.com/doc/offiaccount/Subscription_Messages/api.html
    /// 接口url格式: GET https://api.weixin.qq.com/wxaapi/newtmpl/getpubtemplatekeywords?access_token=ACCESS_TOKEN
    /// </pre>
    pub async fn get_pub_template_keywords_byid(&self, ids: &str) -> LabradorResult<Vec<WechatMpPubTemplateKeywordResponse>> {
        let v = self.client.get(WechatMpMethod::SubscribeMessage(MpSubscribeMessageMethod::GetPubTemplateKeywords), vec![( "tid", ids)], RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse_with_key::<Vec<WechatMpPubTemplateKeywordResponse>>(v, "data")
    }

    /// <pre>
    /// 组合模板并添加至帐号下的个人模板库
    ///
    /// 详情请见: https://developers.weixin.qq.com/doc/offiaccount/Subscription_Messages/api.html
    /// 接口url格式: POST https://api.weixin.qq.com/wxaapi/newtmpl/addtemplate?access_token=ACCESS_TOKEN
    /// </pre>
    pub async fn add_template(&self, id: &str, keywords: Vec<i32>, scene_desc: &str) -> LabradorResult<String> {
        let req = json!({
            "tid": id,
            "kidList": keywords,
            "sceneDesc": scene_desc,
        });
        let v = self.client.post(WechatMpMethod::SubscribeMessage(MpSubscribeMessageMethod::AddTemplate), vec![], req, RequestType::Json).await?.json::<Value>()?;
        let resp= WechatCommonResponse::parse::<Value>(v)?;
        let pri_tmpl_id = resp["priTmplId"].as_str().to_owned().unwrap_or_default().to_string();
        Ok(pri_tmpl_id)
    }

    /// <pre>
    /// 获取当前帐号下的个人模板列表
    ///
    /// 详情请见: https://developers.weixin.qq.com/doc/offiaccount/Subscription_Messages/api.html
    /// 接口url格式: GET https://api.weixin.qq.com/wxaapi/newtmpl/gettemplate?access_token=ACCESS_TOKEN
    /// </pre>
    pub async fn get_template_list(&self) -> LabradorResult<Vec<WechatMpTemplateInfoResponse>> {
        let v = self.client.get(WechatMpMethod::SubscribeMessage(MpSubscribeMessageMethod::GetTemplate), vec![], RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse_with_key::<Vec<WechatMpTemplateInfoResponse>>(v, "data")
    }

    /// <pre>
    /// 删除帐号下的某个模板
    ///
    /// 详情请见: https://developers.weixin.qq.com/doc/offiaccount/Subscription_Messages/api.html
    /// 接口url格式: POST https://api.weixin.qq.com/wxaapi/newtmpl/deltemplate?access_token=ACCESS_TOKEN
    /// </pre>
    pub async fn delete_template(&self, template_id: &str) -> LabradorResult<bool> {
        let _ = self.client.post(WechatMpMethod::SubscribeMessage(MpSubscribeMessageMethod::DeleteTemplate), vec![], json!({ "priTmplId": template_id }), RequestType::Json).await?.json::<WechatCommonResponse>()?;
        Ok(true)
    }

    /// <pre>
    /// 获取公众号类目
    /// https://developers.weixin.qq.com/doc/offiaccount/Subscription_Messages/api.html
    /// GET https://api.weixin.qq.com/wxaapi/newtmpl/getcategory?access_token=ACCESS_TOKEN
    /// </pre>
    pub async fn get_category(&self, template_id: &str) -> LabradorResult<Vec<WechatMpCategoryDataResponse>> {
        let v = self.client.get(WechatMpMethod::SubscribeMessage(MpSubscribeMessageMethod::GetCategory), vec![], RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse_with_key::<Vec<WechatMpCategoryDataResponse>>(v, "data")
    }

    /// <pre>
    /// 发送订阅消息
    /// https://developers.weixin.qq.com/doc/offiaccount/Subscription_Messages/api.html
    /// </pre>
    pub async fn send_subscribe_message(&self, msg: &MpSendSubscribeMessageRequest) -> LabradorResult<WechatCommonResponse> {
        self.client.post(WechatMpMethod::SubscribeMessage(MpSubscribeMessageMethod::SendSubscribeMessage), vec![], msg, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

}


//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MpSendSubscribeOnceRequest {
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

#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct MiniprogramMsg {
    /// 所需跳转到的小程序appid（该小程序 appid 必须与发模板消息的公众号是绑定关联关系，并且小程序要求是已发布的）
    app_id: String,
    /// 所需跳转到小程序的具体页面路径，支持带参数,（示例index?foo=bar）
    pagepath: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpPubTemplateKeywordResponse {
    pub kid: Option<i32>,
    pub name: Option<String>,
    pub example: Option<String>,
    pub rule: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpPubTemplateTitleListResponse {
    pub count: Option<i32>,
    pub data: Option<Vec<WechatMpTemplateItem>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpTemplateItem {
    #[serde(rename = "type")]
    pub r#type: Option<i32>,
    pub tid: Option<i32>,
    pub categoryId: Option<String>,
    pub title: Option<String>,
}


#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpTemplateInfoResponse {
    pub priTmplId: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub example: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<i32>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpCategoryDataResponse {
    pub id: Option<i32>,
    pub name: Option<String>,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MpSendSubscribeMessageRequest {
    /// 填接收消息的用户openid
    pub touser: String,
    /// 订阅消息模板ID
    pub template_id: String,
    /// 模板跳转链接.
    /// <pre>
    /// url和miniprogram都是非必填字段，若都不传则模板无跳转；若都传，会优先跳转至小程序。
    /// 开发者可根据实际需要选择其中一种跳转方式即可。当用户的微信客户端版本不支持跳小程序时，将会跳转至url。
    /// </pre>
    pub url: Option<String>,
    /// 跳小程序所需数据，不需跳小程序可不用传该数据
    pub miniprogram	: Option<MiniprogramMsg>,
    /// 订阅场景值
    pub scene: Option<String>,
    /// 消息正文，value为消息内容文本（200字以内），没有固定格式，可用\n换行，color为整段消息内容的字体颜色（目前仅支持整段消息为一种颜色）
    pub data: Value,
}
