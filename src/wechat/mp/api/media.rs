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
use std::path::Path;
use bytes::Bytes;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

use crate::{session::SessionStore, LabradorResult, RequestBody, RequestType, WeChatMpClient, WechatCommonResponse, WechatRequest, get_nonce_str, request};
use crate::wechat::mp::constants::MATERIAL_TYPE_NEWS;
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
    /// 公众号经常有需要用到一些临时性的多媒体素材的场景，例如在使用接口特别是发送消息时，对多媒体文件、多媒体消息的获取和调用等操作，是通过media_id来进行的。
    /// 素材管理接口对所有认证的订阅号和服务号开放。通过本接口，公众号可以新增临时素材（即上传临时多媒体文件）。
    /// 请注意：
    ///  1、对于临时素材，每个素材（media_id）会在开发者上传或粉丝发送到微信服务器3天后自动删除（所以用户发送给开发者的素材，若开发者需要，应尽快下载到本地），以节省服务器资源。
    ///  2、media_id是可复用的。
    ///  3、素材的格式大小等要求与公众平台官网一致。具体是，图片大小不超过2M，支持png/jpeg/jpg/gif格式，语音大小不超过5M，长度不超过60秒，支持mp3/amr格式
    ///  4、需使用https调用本接口。
    ///  本接口即为原“上传多媒体文件”接口。
    ///  注意事项：
    ///    上传的临时多媒体文件有格式和大小限制，如下：
    ///    图片（image）: 2M，支持PNG\JPEG\JPG\GIF格式
    ///    语音（voice）：2M，播放长度不超过60s，支持AMR\MP3格式
    ///    视频（video）：10MB，支持MP4格式
    ///    缩略图（thumb）：64KB，支持JPG格式
    /// 媒体文件在后台保存时间为3天，即3天后media_id失效。
    /// 详情请见: <a href="http://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1444738726&token=&lang=zh_CN">新增临时素材</a>
    /// 接口url格式：https://api.weixin.qq.com/cgi-bin/media/upload?access_token=ACCESS_TOKEN&type=TYPE
    /// </pre>
    pub async fn upload_media(&self, media_type: &str, file_name: Option<&str>, data: Vec<u8>) -> LabradorResult<WechatMpMediaResponse> {
        let default_file_name = format!("{}.png", get_nonce_str());
        let req = WechatMpMediaRequest {
            media_type: media_type.to_string(),
            file_name: file_name.map(|v| v.to_string()).unwrap_or(default_file_name),
            media_data: data
        };
        let v = self.client.execute::<WechatMpMediaRequest, String>(req).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatMpMediaResponse>(v)
    }

    /// <pre>
    /// 新增临时素材
    /// 本接口即为原“上传多媒体文件”接口。
    ///
    /// 详情请见: <a href="http://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1444738726&token=&lang=zh_CN">新增临时素材</a>
    /// 接口url格式：https://api.weixin.qq.com/cgi-bin/media/upload?access_token=ACCESS_TOKEN&type=TYPE
    /// </pre>
    pub async fn upload_media_with_file(&self, media_type: &str, file_path: &str) -> LabradorResult<WechatMpMediaResponse> {
        let path = Path::new(file_path);
        let file_name = path.file_name().map(|v| v.to_str().unwrap_or_default()).unwrap_or_default();
        let mut f = File::open(path)?;
        let mut content: Vec<u8> = Vec::new();
        let _ = f.read_to_end(&mut content)?;
        self.upload_media(media_type, file_name.into(),content).await
    }

    /// <pre>
    /// 新增临时素材
    /// 本接口即为原“上传多媒体文件”接口。
    ///
    /// 详情请见: <a href="http://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1444738726&token=&lang=zh_CN">新增临时素材</a>
    /// 接口url格式：https://api.weixin.qq.com/cgi-bin/media/upload?access_token=ACCESS_TOKEN&type=TYPE
    /// </pre>
    pub async fn upload_media_with_url(&self, media_type: &str, url: &str) -> LabradorResult<WechatMpMediaResponse> {
        let result = request(|client| client.get(url)).await?;
        let content = result.bytes()?;
        self.upload_media(media_type, None,content.to_vec()).await
    }

    /// <pre>
    /// 获取临时素材
    /// 公众号可以使用本接口获取临时素材（即下载临时的多媒体文件）。请注意，视频文件不支持https下载，调用该接口需http协议。
    /// 本接口即为原“下载多媒体文件”接口。
    /// 根据微信文档，视频文件下载不了，会返回null
    /// 详情请见: <a href="http://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1444738727&token=&lang=zh_CN">获取临时素材</a>
    /// 接口url格式：https://api.weixin.qq.com/cgi-bin/media/get?access_token=ACCESS_TOKEN&media_id=MEDIA_ID
    /// </pre>
    pub async fn get_media(&self, media_id: &str) -> LabradorResult<Bytes> {
        let response = self.client.post(WechatMpMethod::Media(MpMediaMethod::GetMedia), vec![("media_id".to_string(), media_id.to_string())], serde_json::Value::Null, RequestType::Json).await?;
        response.bytes()
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
        let response = self.client.post(WechatMpMethod::Media(MpMediaMethod::GetMediaJssdk), vec![("media_id".to_string(), media_id.to_string())], serde_json::Value::Null, RequestType::Json).await?;
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
    pub async fn upload_img_media(&self, file_name: &str, data: Vec<u8>) -> LabradorResult<WechatMpMediaResponse> {
        let req = WechatMpImageRequest {
            media_data: data,
            file_name: file_name.to_string(),
        };
        let v = self.client.execute::<WechatMpImageRequest, String>(req).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatMpMediaResponse>(v)
    }

    /// <pre>
    /// 新增非图文永久素材
    /// 通过POST表单来调用接口，表单id为media，包含需要上传的素材内容，有filename、filelength、content-type等信息。请注意：图片素材将进入公众平台官网素材管理模块中的默认分组。
    /// 新增永久视频素材需特别注意：
    /// 在上传视频素材时需要POST另一个表单，id为description，包含素材的描述信息，内容格式为JSON，格式如下：
    /// {   "title":VIDEO_TITLE,   "introduction":INTRODUCTION   }
    /// 详情请见: <a href="http://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1444738729&token=&lang=zh_CN">新增永久素材</a>
    /// 接口url格式：https://api.weixin.qq.com/cgi-bin/material/add_material?access_token=ACCESS_TOKEN&type=TYPE
    ///
    /// 除了3天就会失效的临时素材外，开发者有时需要永久保存一些素材，届时就可以通过本接口新增永久素材。
    /// 永久图片素材新增后，将带有URL返回给开发者，开发者可以在腾讯系域名内使用（腾讯系域名外使用，图片将被屏蔽）。
    /// 请注意：
    /// 1、新增的永久素材也可以在公众平台官网素材管理模块中看到
    /// 2、永久素材的数量是有上限的，请谨慎新增。图文消息素材和图片素材的上限为5000，其他类型为1000
    /// 3、素材的格式大小等要求与公众平台官网一致。具体是，图片大小不超过2M，支持bmp/png/jpeg/jpg/gif格式，语音大小不超过5M，长度不超过60秒，支持mp3/wma/wav/amr格式
    /// 4、调用该接口需https协议
    /// </pre>
    pub async fn upload_material(&self, media_type: &str, req: WechatMpMaterialRequest) -> LabradorResult<WechatMpMediaResponse> {
        let v = self.client.execute::<WechatMpMaterialRequest, String>(req).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatMpMediaResponse>(v)
    }

    /// <pre>
    /// 获取声音或者图片永久素材
    ///
    /// 详情请见: <a href="http://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1444738729&token=&lang=zh_CN">获取永久素材</a>
    /// 接口url格式：https://api.weixin.qq.com/cgi-bin/material/get_material?access_token=ACCESS_TOKEN
    /// </pre>
    pub async fn get_material(&self, media_id: &str) -> LabradorResult<Bytes> {
        let response = self.client.post(WechatMpMethod::Media(MpMediaMethod::GetMaterial), vec![("media_id".to_string(), media_id.to_string())], serde_json::Value::Null, RequestType::Json).await?;
        response.bytes()
    }

    /// <pre>
    /// 获取视频永久素材的信息和下载地址
    ///
    /// 详情请见: <a href="http://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1444738729&token=&lang=zh_CN">获取永久素材</a>
    /// 接口url格式：https://api.weixin.qq.com/cgi-bin/material/get_material?access_token=ACCESS_TOKEN
    /// </pre>
    pub async fn get_material_video_info(&self, media_id: &str) -> LabradorResult<WechatMpMaterialVideoInfoResponse> {
        let response = self.client.post(WechatMpMethod::Media(MpMediaMethod::GetMaterial), vec![("media_id".to_string(), media_id.to_string())], serde_json::Value::Null, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatMpMaterialVideoInfoResponse>(response)
    }

    /// <pre>
    /// 修改永久图文素材
    ///
    /// 详情请见: <a href="http://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1444738732&token=&lang=zh_CN">修改永久图文素材</a>
    /// 接口url格式：https://api.weixin.qq.com/cgi-bin/material/update_news?access_token=ACCESS_TOKEN
    /// </pre>
    pub async fn get_material_news(&self, media_id: &str) -> LabradorResult<WechatMpMaterialNewsResponse> {
        let response = self.client.post(WechatMpMethod::Media(MpMediaMethod::GetMaterial), vec![("media_id".to_string(), media_id.to_string())], serde_json::Value::Null, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatMpMaterialNewsResponse>(response)
    }

    /// <pre>
    /// 删除永久素材
    /// 在新增了永久素材后，开发者可以根据本接口来删除不再需要的永久素材，节省空间。
    /// 请注意：
    ///  1、请谨慎操作本接口，因为它可以删除公众号在公众平台官网素材管理模块中新建的图文消息、语音、视频等素材（但需要先通过获取素材列表来获知素材的media_id）
    ///  2、临时素材无法通过本接口删除
    ///  3、调用该接口需https协议
    /// 详情请见: <a href="http://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1444738731&token=&lang=zh_CN">删除永久素材</a>
    /// 接口url格式：https://api.weixin.qq.com/cgi-bin/material/del_material?access_token=ACCESS_TOKEN
    /// </pre>
    pub async fn delete_material(&self, media_id: &str) -> LabradorResult<WechatCommonResponse> {
        self.client.post(WechatMpMethod::Media(MpMediaMethod::DeleteMaterial), vec![], json!({"media_id": media_id}), RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 获取各类素材总数
    /// 开发者可以根据本接口来获取永久素材的列表，需要时也可保存到本地。
    /// 请注意：
    ///  1.永久素材的总数，也会计算公众平台官网素材管理中的素材
    ///  2.图片和图文消息素材（包括单图文和多图文）的总数上限为5000，其他素材的总数上限为1000
    ///  3.调用该接口需https协议
    ///
    /// 详情请见: <a href="http://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1444738733&token=&lang=zh_CN">获取素材总数</a>
    /// 接口url格式：https://api.weixin.qq.com/cgi-bin/material/get_materialcount?access_token=ACCESS_TOKEN
    /// </pre>
    pub async fn get_material_count(&self) -> LabradorResult<WechatMpMaterialCountResponse> {
        let v = self.client.post(WechatMpMethod::Media(MpMediaMethod::DeleteMaterial), vec![], Value::Null, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatMpMaterialCountResponse>(v)
    }

    /// <pre>
    /// 分页获取图文素材列表
    ///
    /// 详情请见: <a href="http://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1444738734&token=&lang=zh_CN">获取素材列表</a>
    /// 接口url格式：https://api.weixin.qq.com/cgi-bin/material/batchget_material?access_token=ACCESS_TOKEN
    /// </pre>
    pub async fn get_material_news_batch(&self, offset: i32, count: i32) -> LabradorResult<WechatMpMaterialNewsBatchResponse> {

        let v = self.client.post(WechatMpMethod::Media(MpMediaMethod::GetMaterialList), vec![], json!({
            "type": MATERIAL_TYPE_NEWS,
            "offset": offset,
            "count": count
        }), RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatMpMaterialNewsBatchResponse>(v)
    }

    /// <pre>
    /// 分页获取其他媒体素材列表
    ///
    /// 详情请见: <a href="http://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1444738734&token=&lang=zh_CN">获取素材列表</a>
    /// 接口url格式：https://api.weixin.qq.com/cgi-bin/material/batchget_material?access_token=ACCESS_TOKEN
    /// </pre>
    pub async fn get_material_batch(&self,material_type: &str, offset: i32, count: i32) -> LabradorResult<WechatMpMaterialBatchResponse> {
        let v = self.client.post(WechatMpMethod::Media(MpMediaMethod::GetMaterialList), vec![], json!({
            "type": material_type,
            "offset": offset,
            "count": count
        }), RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatMpMaterialBatchResponse>(v)
    }


}

//----------------------------------------------------------------------------------------------------------------------------



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpMediaRequest {
    pub media_type: String,
    pub file_name: String,
    pub media_data: Vec<u8>
}

impl WechatRequest for WechatMpMediaRequest {
    fn get_api_method_name(&self) -> String {
        MpMediaMethod::UploadMedia(self.media_type.to_string()).get_method()
    }

    fn get_request_body<T: Serialize>(&self) -> RequestBody<T> {
        let form = reqwest::multipart::Form::new().part("media",    reqwest::multipart::Part::stream(self.media_data.to_owned()).file_name(self.file_name.to_string()));
        form.into()
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpImageRequest {
    pub file_name: String,
    pub media_data: Vec<u8>
}

impl WechatRequest for WechatMpImageRequest {
    fn get_api_method_name(&self) -> String {
        MpMediaMethod::UploadImage.get_method()
    }

    fn get_request_body<T: Serialize>(&self) -> RequestBody<T> {
        let form = reqwest::multipart::Form::new().part("media", reqwest::multipart::Part::stream(self.media_data.to_vec()).file_name(self.file_name.to_string()));
        form.into()
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpMediaResponse {
    pub url: Option<String>,
    pub media_id: Option<String>,
    #[serde(rename="type")]
    pub r#type: Option<String>,
    pub thumb_media_id: Option<String>,
    pub created_at: Option<i64>,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpMaterialRequest {
    pub media_type: String,
    pub filename: String,
    pub name: Option<String>,
    pub video_title: Option<String>,
    pub video_introduction: Option<String>,
    pub media_data: Vec<u8>
}

impl WechatRequest for WechatMpMaterialRequest {
    fn get_api_method_name(&self) -> String {
        MpMediaMethod::AddMaterial(self.media_type.to_string()).get_method()
    }

    fn get_request_body<T: Serialize>(&self) -> RequestBody<T> {
        let mut form = reqwest::multipart::Form::new().part("media", reqwest::multipart::Part::stream(self.media_data.to_vec()).file_name(self.filename.to_string()));
        if let Some(video_title) = &self.video_title {
            form = form.text("title", video_title.to_string());
        }
        if let Some(video_introduction) = &self.video_introduction {
            form = form.text("introduction", video_introduction.to_string());
        }
        form.into()
    }
}

/// 视频素材
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpMaterialVideoInfoResponse {
    pub title: Option<String>,
    pub description: Option<String>,
    pub down_url: Option<String>,
}

/// 图文素材
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpMaterialNewsResponse {
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    pub articles: Vec<WechatMpNewsArticle>,
}


/// 图文消息article.
/// 1. thumbMediaId  (必填) 图文消息的封面图片素材id（必须是永久mediaID）
/// 2. author          图文消息的作者
/// 3. title           (必填) 图文消息的标题
/// 4. contentSourceUrl 在图文消息页面点击“阅读原文”后的页面链接
/// 5. content (必填)  图文消息页面的内容，支持HTML标签
/// 6. digest          图文消息的描述
/// 7. showCoverPic  是否显示封面，true为显示，false为不显示
/// 8. url           点击图文消息跳转链接
/// 9. need_open_comment（新增字段） 否 Uint32 是否打开评论，0不打开，1打开
/// 10. only_fans_can_comment（新增字段） 否 Uint32 是否粉丝才可评论，0所有人可评论，1粉丝才可评论
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpNewsArticle {
    /// (必填) 图文消息缩略图的media_id，可以在基础支持-上传多媒体文件接口中获得.
    pub thumb_media_id: String,
    /// 图文消息的封面url
    pub thumb_url: Option<String>,
    /// 图文消息的作者
    pub author: Option<String>,
    /// (必填) 图文消息的标题.
    pub title: String,
    /// 在图文消息页面点击“阅读原文”后的页面链接.
    pub content_source_url: Option<String>,
    /// (必填) 图文消息页面的内容，支持HTML标签.
    pub content: String,
    /// 图文消息的描述
    pub digest: Option<String>,
    /// 是否显示封面，true为显示，false为不显示.
    pub show_cover_pic: bool,
    /// 点击图文消息跳转链接
    pub url: Option<String>,
    /// 是否打开评论，0不打开，1打开.
    pub need_open_comment: Option<u8>,
    /// 是否粉丝才可评论，0所有人可评论，1粉丝才可评论.
    pub only_fans_can_comment: Option<u8>,
}




/// 素材数量
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpMaterialCountResponse {
    pub voice_count: Option<i32>,
    pub video_count: Option<i32>,
    pub image_count: Option<i32>,
    pub news_count: Option<i32>,
}


/// 图文素材
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpMaterialNewsBatchResponse {
    pub total_count: Option<i32>,
    pub item_count: Option<i32>,
    pub items: Option<Vec<WechatMpMaterialNewsBatchItem>>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpMaterialNewsBatchItem {
    pub media_id: Option<String>,
    pub update_time: Option<String>,
    pub content: Option<WechatMpMaterialNewsResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpMaterialBatchResponse {
    pub total_count: Option<i32>,
    pub item_count: Option<i32>,
    pub items: Option<Vec<WechatMpMaterialBatchItem>>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpMaterialBatchItem {
    pub media_id: Option<String>,
    pub update_time: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
}


