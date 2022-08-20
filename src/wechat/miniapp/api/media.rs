use serde_json::{Value};
use serde::{Serialize, Deserialize};

use crate::{session::SessionStore, request::{RequestType}, WechatCommonResponse, LabradorResult, RequestMethod};
use crate::wechat::miniapp::method::{MaMediaMethod, WechatMaMethod};
use crate::wechat::miniapp::{WeChatMaClient, WechatRequest};


#[derive(Debug, Clone)]
pub struct WechatMaMedia<'a, T: SessionStore> {
    client: &'a WeChatMaClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WechatMaMedia<'a, T> {

    #[inline]
    pub fn new(client: &WeChatMaClient<T>) -> WechatMaMedia<T> {
        WechatMaMedia {
            client,
        }
    }

    // /// <pre>
    // /// 新增临时素材
    // /// 小程序可以使用本接口把媒体文件（目前仅支持图片）上传到微信服务器，用户发送客服消息或被动回复用户消息。
    // /// 详情请见: <a href="https://mp.weixin.qq.com/debug/wxadoc/dev/api/custommsg/material.html#新增临时素材">新增临时素材</a>
    // /// 接口url格式：https://api.weixin.qq.com/cgi-bin/media/upload?access_token=ACCESS_TOKEN&type=TYPE
    // /// </pre>
    // pub async fn upload_media(&self, code: &str) -> LabradorResult<JsCodeSession> {
    //     let v = self.client.get(WechatMaMethod::Media(), vec![
    //         ("grant_type", "authorization_code"),
    //         ("js_code", code),
    //         ("appid", &self.client.appid),
    //         ("secret", &self.client.secret),
    //     ], RequestType::Json).await?.json::<serde_json::Value>()?;
    //     WechatCommonResponse::parse::<JsCodeSession>(v)
    // }
    //
    // /// <pre>
    // /// 新增临时素材
    // /// 小程序可以使用本接口把媒体文件（目前仅支持图片）上传到微信服务器，用户发送客服消息或被动回复用户消息。
    // ///
    // /// 详情请见: <a href="https://mp.weixin.qq.com/debug/wxadoc/dev/api/custommsg/material.html#新增临时素材">新增临时素材</a>
    // /// 接口url格式：https://api.weixin.qq.com/cgi-bin/media/upload?access_token=ACCESS_TOKEN&type=TYPE
    // /// </pre>
    // pub async fn upload_media_with_type(&self, media_type: &str) -> LabradorResult<JsCodeSession> {
    //     let v = self.client.post(WechatMaMethod::Media(MaMediaMethod::UploadMedia(media_type.to_string())), vec![
    //         ("grant_type", "authorization_code"),
    //         ("js_code", code),
    //         ("appid", &self.client.appid),
    //         ("secret", &self.client.secret),
    //     ], RequestType::Json).await?.json::<serde_json::Value>()?;
    //     WechatCommonResponse::parse::<JsCodeSession>(v)
    // }
    //
    // /// <pre>
    // /// 获取临时素材
    // /// 小程序可以使用本接口获取客服消息内的临时素材（即下载临时的多媒体文件）。目前小程序仅支持下载图片文件。
    // ///
    // /// 详情请见: <a href="https://mp.weixin.qq.com/debug/wxadoc/dev/api/custommsg/material.html#获取临时素材">获取临时素材</a>
    // /// 接口url格式：https://api.weixin.qq.com/cgi-bin/media/get?access_token=ACCESS_TOKEN&media_id=MEDIA_ID
    // /// </pre>
    // pub async fn get_media(&self, code: &str) -> LabradorResult<JsCodeSession> {
    //     let v = self.client.get(WechatMaMethod::CodeSession, vec![
    //         ("grant_type", "authorization_code"),
    //         ("js_code", code),
    //         ("appid", &self.client.appid),
    //         ("secret", &self.client.secret),
    //     ], RequestType::Json).await?.json::<serde_json::Value>()?;
    //     WechatCommonResponse::parse::<JsCodeSession>(v)
    // }
}

//----------------------------------------------------------------------------------------------------------------------------



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMaMediaRequest {
    pub media_type: Option<String>,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMaMediaResponse {
    pub url: Option<String>,
    pub media_id: Option<String>,
    #[serde(rename="type")]
    pub r#type: Option<String>,
    pub thumb_media_id: Option<String>,
    pub created_at: Option<i64>,
}
