use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use bytes::Bytes;
use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::{session::SessionStore, LabradorResult, RequestBody, RequestType, WechatCpClient, WechatRequest, WechatCommonResponse, request, get_nonce_str};
use crate::wechat::cp::constants::{ATTACHMENT_TYPE, MEDIA_TYPE};
use crate::wechat::cp::method::{CpMediaMethod, WechatCpMethod};


#[derive(Debug, Clone)]
pub struct WechatCpMedia<'a, T: SessionStore> {
    client: &'a WechatCpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WechatCpMedia<'a, T> {

    #[inline]
    pub fn new(client: &WechatCpClient<T>) -> WechatCpMedia<T> {
        WechatCpMedia {
            client,
        }
    }

    /// <pre>
    /// 上传多媒体文件.
    /// 上传的多媒体文件有格式和大小限制，如下：
    ///   图片（image）: 1M，支持JPG格式
    ///   语音（voice）：2M，播放长度不超过60s，支持AMR\MP3格式
    ///   视频（video）：10MB，支持MP4格式
    ///   缩略图（thumb）：64KB，支持JPG格式
    /// 详情请见: http://mp.weixin.qq.com/wiki/index.php?title=上传下载多媒体文件
    /// </pre>
    pub async fn upload_media(&self, media_type: &str, file_name: Option<&str>, data: Vec<u8>) -> LabradorResult<WechatCpMediaResponse> {
        let default_file_name = format!("{}.png", get_nonce_str());
        let req = WechatCpMediaRequest {
            media_type: media_type.to_string(),
            file_name: file_name.map(|v| v.to_string()).unwrap_or(default_file_name),
            media_data: data
        };
        let v = self.client.execute::<WechatCpMediaRequest, String>(req).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpMediaResponse>(v)
    }

    /// <pre>
    /// 上传图片.
    /// 上传图片得到图片URL，该URL永久有效
    /// 返回的图片URL，仅能用于图文消息（mpnews）正文中的图片展示；若用于非企业微信域名下的页面，图片将被屏蔽。
    /// 每个企业每天最多可上传100张图片
    /// 接口url格式：https://qyapi.weixin.qq.com/cgi-bin/media/uploadimg?access_token=ACCESS_TOKEN
    /// </pre>
    pub async fn upload_img(&self, media_type: &str, file_name: &str, data: Vec<u8>) -> LabradorResult<WechatCpMediaResponse> {
        let req = WechatCpMediaRequest {
            media_type: media_type.to_string(),
            file_name: file_name.to_string(),
            media_data: data
        };
        let v= self.client.execute::<WechatCpMediaRequest, String>(req).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpMediaResponse>(v)
    }

    /// <pre>
    /// 新增临时素材
    /// 小程序可以使用本接口把媒体文件（目前仅支持图片）上传到微信服务器，用户发送客服消息或被动回复用户消息。
    ///
    /// 详情请见: <a href="https://mp.weixin.qq.com/debug/wxadoc/dev/api/custommsg/material.html#新增临时素材">新增临时素材</a>
    /// 接口url格式：https://api.weixin.qq.com/cgi-bin/media/upload?access_token=ACCESS_TOKEN&type=TYPE
    /// </pre>
    pub async fn upload_media_with_file(&self, media_type: &str, file_path: &str) -> LabradorResult<WechatCpMediaResponse> {
        let path = Path::new(file_path);
        let file_name = path.file_name().map(|v| v.to_str().unwrap_or_default()).unwrap_or_default();
        let mut f = File::open(path)?;
        let mut content: Vec<u8> = Vec::new();
        let _ = f.read_to_end(&mut content)?;
        self.upload_media(media_type, file_name.into(),content).await
    }

    /// <pre>
    /// 新增临时素材
    /// 小程序可以使用本接口把媒体文件（目前仅支持图片）上传到微信服务器，用户发送客服消息或被动回复用户消息。
    ///
    /// 详情请见: <a href="https://mp.weixin.qq.com/debug/wxadoc/dev/api/custommsg/material.html#新增临时素材">新增临时素材</a>
    /// 接口url格式：https://api.weixin.qq.com/cgi-bin/media/upload?access_token=ACCESS_TOKEN&type=TYPE
    /// </pre>
    pub async fn upload_media_with_url(&self, media_type: &str, url: &str) -> LabradorResult<WechatCpMediaResponse> {
        let result = request(|client| client.get(url)).await?;
        let content = result.bytes()?;
        self.upload_media(media_type, None,content.to_vec()).await
    }

    /// <pre>
    /// 下载多媒体文件.
    /// 根据微信文档，视频文件下载不了，会返回null
    /// 详情请见: http://mp.weixin.qq.com/wiki/index.php?title=上传下载多媒体文件
    /// </pre>
    pub async fn get_media(&self, media_id: &str) -> LabradorResult<Bytes> {
        let response = self.client.post(WechatCpMethod::Media(CpMediaMethod::GetMedia), vec![("media_id".to_string(), media_id.to_string())], serde_json::Value::Null, RequestType::Json).await?;
        response.bytes()
    }


    /// <pre>
    /// 上传附件资源
    /// <a href="https://open.work.weixin.qq.com/api/doc/90001/90143/95178">上传附件资源</a>
    /// </pre>
    pub async fn upload_attachment_with_file(&self, media_type: &str, attachment_type: &str, file_path: &str) -> LabradorResult<WechatCpMediaResponse> {
        let path = Path::new(file_path);
        let file_name = path.file_name().map(|v| v.to_str().unwrap_or_default()).unwrap_or_default();
        let mut f = File::open(path)?;
        let mut content: Vec<u8> = Vec::new();
        let _ = f.read_to_end(&mut content)?;
        self.upload_attachment(media_type, attachment_type, file_name.into(), content).await
    }


    /// <pre>
    /// 上传附件资源
    /// <a href="https://open.work.weixin.qq.com/api/doc/90001/90143/95178">上传附件资源</a>
    /// </pre>
    pub async fn upload_attachment_with_url(&self, media_type: &str, attachment_type: &str, url: &str) -> LabradorResult<WechatCpMediaResponse> {
        let result = request(|client| client.get(url)).await?;
        let content = result.bytes()?;
        self.upload_attachment(media_type, attachment_type,None, content.to_vec()).await
    }


    /// <pre>
    /// 上传附件资源
    /// <a href="https://open.work.weixin.qq.com/api/doc/90001/90143/95178">上传附件资源</a>
    /// </pre>
    pub async fn upload_attachment(&self, media_type: &str, attachment_type: &str, file_name: Option<&str>, data: Vec<u8>) -> LabradorResult<WechatCpMediaResponse> {
        let default_file_name = format!("{}.png", get_nonce_str());
        let req = WechatCpAttachmentRequest {
            media_type: media_type.to_string(),
            attachment_type: attachment_type.to_string(),
            file_name: file_name.map(|v| v.to_string()).unwrap_or(default_file_name),
            media_data: data
        };
        let v = self.client.execute::<WechatCpAttachmentRequest, String>(req).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpMediaResponse>(v)
    }

    /// <pre>
    /// 获取高清语音素材
    /// 公众号可以使用本接口获取从JSSDK的uploadVoice接口上传的临时语音素材，格式为speex，16K采样率。
    /// 该音频比上文的临时素材获取接口（格式为amr，8K采样率）更加清晰，适合用作语音识别等对音质要求较高的业务。
    /// 详情请见: <a href="https://developers.weixin.qq.com/doc/offiaccount/Asset_Management/Get_temporary_materials.html">
    /// 获取高清语音素材</a>
    /// 接口url格式：https://api.weixin.qq.com/cgi-bin/media/get/jssdk?access_token=ACCESS_TOKEN&media_id=MEDIA_ID
    /// </pre>
    pub async fn get_media_jssdk(&self, media_id: &str) -> LabradorResult<Bytes> {
        let response = self.client.post(WechatCpMethod::Media(CpMediaMethod::GetMediaJssdk), vec![("media_id".to_string(), media_id.to_string())], serde_json::Value::Null, RequestType::Json).await?;
        response.bytes()
    }
}

//----------------------------------------------------------------------------------------------------------------------------



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpMediaRequest {
    pub media_type: String,
    pub file_name: String,
    pub media_data: Vec<u8>
}

impl WechatRequest for WechatCpMediaRequest {
    fn get_api_method_name(&self) -> String {
        CpMediaMethod::UploadMedia(self.media_type.to_string()).get_method()
    }

    fn get_request_body<T: Serialize>(&self) -> RequestBody<T> {
        let form = reqwest::multipart::Form::new().part("media", reqwest::multipart::Part::stream(self.media_data.to_owned()).file_name(self.file_name.to_string()));
        form.into()
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpMediaResponse {
    pub url: Option<String>,
    pub media_id: Option<String>,
    #[serde(rename="type")]
    pub r#type: Option<String>,
    pub thumb_media_id: Option<String>,
    pub created_at: Option<i64>,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpAttachmentRequest {
    pub media_type: String,
    pub attachment_type: String,
    pub file_name: String,
    pub media_data: Vec<u8>
}

impl WechatRequest for WechatCpAttachmentRequest {
    fn get_api_method_name(&self) -> String {
        CpMediaMethod::UploadAttachment.get_method()
    }

    fn get_query_params(&self) -> BTreeMap<String, String> {
        BTreeMap::from([(MEDIA_TYPE.to_string(), self.media_type.to_string()), (ATTACHMENT_TYPE.to_string(), self.attachment_type.to_string())])
    }

    fn get_request_body<T: Serialize>(&self) -> RequestBody<T> {
        let form = reqwest::multipart::Form::new().part("file", reqwest::multipart::Part::stream(self.media_data.to_owned()).file_name(self.file_name.to_string()));
        form.into()
    }
}
