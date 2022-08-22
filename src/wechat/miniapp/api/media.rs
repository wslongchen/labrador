use std::fs::File;
use std::io::Read;
use bytes::Bytes;
use serde::{Serialize, Deserialize};

use crate::{session::SessionStore, LabradorResult, RequestBody, RequestType};
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

    /// <pre>
    /// 新增临时素材
    /// 小程序可以使用本接口把媒体文件（目前仅支持图片）上传到微信服务器，用户发送客服消息或被动回复用户消息。
    /// 详情请见: <a href="https://mp.weixin.qq.com/debug/wxadoc/dev/api/custommsg/material.html#新增临时素材">新增临时素材</a>
    /// 接口url格式：https://api.weixin.qq.com/cgi-bin/media/upload?access_token=ACCESS_TOKEN&type=TYPE
    /// </pre>
    pub async fn upload_media(&self, media_type: &str, data: Vec<u8>) -> LabradorResult<WechatMaMediaResponse> {
        let req = WechatMaMediaRequest {
            media_type: media_type.to_string(),
            media_data: data
        };
        self.client.execute::<WechatMaMediaRequest, String>(req).await?.json::<WechatMaMediaResponse>()
    }

    /// <pre>
    /// 新增临时素材
    /// 小程序可以使用本接口把媒体文件（目前仅支持图片）上传到微信服务器，用户发送客服消息或被动回复用户消息。
    ///
    /// 详情请见: <a href="https://mp.weixin.qq.com/debug/wxadoc/dev/api/custommsg/material.html#新增临时素材">新增临时素材</a>
    /// 接口url格式：https://api.weixin.qq.com/cgi-bin/media/upload?access_token=ACCESS_TOKEN&type=TYPE
    /// </pre>
    pub async fn upload_media_with_type(&self, media_type: &str, mut f: File) -> LabradorResult<WechatMaMediaResponse> {
        let mut contents: Vec<u8> = Vec::new();
        let _ = f.read_to_end(&mut contents)?;
        self.upload_media(media_type, contents).await
    }

    /// <pre>
    /// 获取临时素材
    /// 小程序可以使用本接口获取客服消息内的临时素材（即下载临时的多媒体文件）。目前小程序仅支持下载图片文件。
    ///
    /// 详情请见: <a href="https://mp.weixin.qq.com/debug/wxadoc/dev/api/custommsg/material.html#获取临时素材">获取临时素材</a>
    /// 接口url格式：https://api.weixin.qq.com/cgi-bin/media/get?access_token=ACCESS_TOKEN&media_id=MEDIA_ID
    /// </pre>
    pub async fn get_media(&self, media_id: &str) -> LabradorResult<Bytes> {
        let response = self.client.post(WechatMaMethod::Media(MaMediaMethod::GetMedia), vec![("media_id".to_string(), media_id.to_string())], serde_json::Value::Null, RequestType::Json).await?;
        response.bytes()
    }
}

//----------------------------------------------------------------------------------------------------------------------------



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMaMediaRequest {
    pub media_type: String,
    pub media_data: Vec<u8>
}

impl WechatRequest for WechatMaMediaRequest {
    fn get_api_method_name(&self) -> String {
        MaMediaMethod::UploadMedia(self.media_type.to_string()).get_method()
    }

    fn get_request_body<T: Serialize>(&self) -> RequestBody<T> {
        let form = reqwest::multipart::Form::new().part("file", reqwest::multipart::Part::stream(self.media_data.to_vec()));
        form.into()
    }
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
