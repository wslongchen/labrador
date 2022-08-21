//! 公众号经常有需要用到一些临时性的多媒体素材的场景，例如在使用接口特别是发送消息时，对多媒体文件、多媒体消息的获取和调用等操作，是通过media_id来进行的。素材管理接口对所有认证的订阅号和服务号开放。通过本接口，公众号可以新增临时素材（即上传临时多媒体文件）。使用接口过程中有任何问题，可以前往微信开放社区 #公众号 专区发帖交流
//!
//! 注意点：
//!
//! 1、临时素材media_id是可复用的。
//!
//! 2、媒体文件在微信后台保存时间为3天，即3天后media_id失效。
//!
//! 3、上传临时素材的格式、大小限制与公众平台官网一致。
//!
//! 图片（image）: 10M，支持PNG\JPEG\JPG\GIF格式
//!
//! 语音（voice）：2M，播放长度不超过60s，支持AMR\MP3格式
//!
//! 视频（video）：10MB，支持MP4格式
//!
//! 缩略图（thumb）：64KB，支持 JPG 格式
//!
//! 4、需使用 https 调用本接口。
use std::fs::File;
use std::io::Read;
use bytes::Bytes;
use serde::{Serialize, Deserialize};

use crate::{session::SessionStore, LabradorResult, RequestBody, RequestType, WeChatMpClient};
use crate::wechat::miniapp::WechatRequest;
use crate::wechat::mp::method::{MpMediaMethod, WechatMpMethod};


#[derive(Debug, Clone)]
pub struct WechatMpMedia<'a, T: SessionStore> {
    client: &'a WeChatMpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WechatMpMedia<'a, T> {

    #[inline]
    pub fn new(client: &WeChatMpClient<T>) -> WechatMpMedia<T> {
        WechatMpMedia {
            client,
        }
    }

    /// <pre>
    /// 新增临时素材
    /// 公众号可以使用本接口把媒体文件（目前仅支持图片）上传到微信服务器，用户发送客服消息或被动回复用户消息。
    /// 详情请见: <a href="https://developers.weixin.qq.com/doc/offiaccount/Asset_Management/New_temporary_materials.html">新增临时素材</a>
    /// </pre>
    pub async fn upload_media(&self, media_type: &str, data: Vec<u8>) -> LabradorResult<WechatMaMediaResponse> {
        let req = WechatMpMediaRequest {
            media_type: media_type.to_string(),
            media_data: data
        };
        self.client.execute::<WechatMpMediaRequest, String>(req).await?.json::<WechatMaMediaResponse>()
    }

    /// <pre>
    /// 新增临时素材
    /// 公众号可以使用本接口把媒体文件（目前仅支持图片）上传到微信服务器，用户发送客服消息或被动回复用户消息。
    /// 详情请见: <a href="https://developers.weixin.qq.com/doc/offiaccount/Asset_Management/New_temporary_materials.html">新增临时素材</a>
    /// </pre>
    pub async fn upload_media_with_type(&self, media_type: &str, mut f: File) -> LabradorResult<WechatMaMediaResponse> {
        let mut contents: Vec<u8> = Vec::new();
        let _ = f.read_to_end(&mut contents)?;
        self.upload_media(media_type, contents).await
    }

    /// <pre>
    /// 公众号可以使用本接口获取临时素材（即下载临时的多媒体文件）。
    ///
    /// 本接口即为原“下载多媒体文件”接口。
    /// 详情请见: <a href="https://developers.weixin.qq.com/doc/offiaccount/Asset_Management/Get_temporary_materials.html">获取临时素材</a>
    /// </pre>
    pub async fn get_media(&self, media_id: &str) -> LabradorResult<Bytes> {
        let response = self.client.post(WechatMpMethod::Media(MpMediaMethod::GetMedia), vec![("media_id".to_string(), media_id.to_string())], serde_json::Value::Null, RequestType::Json).await?;
        response.bytes()
    }

    /// 上传图文消息内的图片获取URL
    /// <pre>
    /// 对于常用的素材，开发者可通过本接口上传到微信服务器，永久使用。新增的永久素材也可以在公众平台官网素材管理模块中查询管理。
    /// 请注意：
    /// 1、最近更新：永久图片素材新增后，将带有 URL 返回给开发者，开发者可以在腾讯系域名内使用（腾讯系域名外使用，图片将被屏蔽）。
    /// 2、公众号的素材库保存总数量有上限：图文消息素材、图片素材上限为100000，其他类型为1000。
    /// 3、素材的格式大小等要求与公众平台官网一致：
    /// 图片（image）: 10M，支持bmp/png/jpeg/jpg/gif格式
    /// 语音（voice）：2M，播放长度不超过60s，mp3/wma/wav/amr格式
    /// 视频（video）：10MB，支持MP4格式
    /// 缩略图（thumb）：64KB，支持 JPG 格式
    /// 4、图文消息的具体内容中，微信后台将过滤外部的图片链接，图片 url 需通过"上传图文消息内的图片获取URL"接口上传图片获取。
    /// 5、"上传图文消息内的图片获取URL"接口所上传的图片，不占用公众号的素材库中图片数量的100000个的限制，图片仅支持jpg/png格式，大小必须在1MB以下。
    /// 6、图文消息支持正文中插入自己帐号和其他公众号已群发文章链接的能力。
    /// </pre>
    pub async fn upload_img(&self, media_type: &str, data: Vec<u8>) -> LabradorResult<WechatMaMediaResponse> {
        let req = WechatMpMediaRequest {
            media_type: media_type.to_string(),
            media_data: data
        };
        self.client.execute::<WechatMpMediaRequest, String>(req).await?.json::<WechatMaMediaResponse>()
    }

    /// <pre>
    /// 新增临时素材
    /// 公众号可以使用本接口把媒体文件（目前仅支持图片）上传到微信服务器，用户发送客服消息或被动回复用户消息。
    /// 详情请见: <a href="https://developers.weixin.qq.com/doc/offiaccount/Asset_Management/New_temporary_materials.html">新增临时素材</a>
    /// </pre>
    pub async fn upload_media_with_type(&self, media_type: &str, mut f: File) -> LabradorResult<WechatMaMediaResponse> {
        let mut contents: Vec<u8> = Vec::new();
        let _ = f.read_to_end(&mut contents)?;
        self.upload_media(media_type, contents).await
    }

    /// <pre>
    /// 公众号可以使用本接口获取临时素材（即下载临时的多媒体文件）。
    ///
    /// 本接口即为原“下载多媒体文件”接口。
    /// 详情请见: <a href="https://developers.weixin.qq.com/doc/offiaccount/Asset_Management/Get_temporary_materials.html">获取临时素材</a>
    /// </pre>
    pub async fn get_media(&self, media_id: &str) -> LabradorResult<Bytes> {
        let response = self.client.post(WechatMpMethod::Media(MpMediaMethod::GetMedia), vec![("media_id".to_string(), media_id.to_string())], serde_json::Value::Null, RequestType::Json).await?;
        response.bytes()
    }
}

//----------------------------------------------------------------------------------------------------------------------------



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpMediaRequest {
    pub media_type: String,
    pub media_data: Vec<u8>
}

impl WechatRequest for WechatMpMediaRequest {
    fn get_api_method_name(&self) -> String {
        MpMediaMethod::UploadMedia(self.media_type.to_string()).get_method()
    }

    fn get_request_body<T: Serialize>(&self) -> RequestBody<T> {
        let form = reqwest::multipart::Form::new().part("media", reqwest::multipart::Part::stream(self.media_data.to_vec()));
        form.into()
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpImageRequest {
    pub media_data: Vec<u8>
}

impl WechatRequest for WechatMpMediaRequest {
    fn get_api_method_name(&self) -> String {
        MpMediaMethod::UploadMedia(self.media_type.to_string()).get_method()
    }

    fn get_request_body<T: Serialize>(&self) -> RequestBody<T> {
        let form = reqwest::multipart::Form::new().part("media", reqwest::multipart::Part::stream(self.media_data.to_vec()));
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
