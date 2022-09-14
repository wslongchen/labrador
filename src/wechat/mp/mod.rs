use crate::{session::SessionStore, client::APIClient, request::{Method, RequestType, LabraResponse, LabraRequest, RequestMethod}, WechatCrypto, util::current_timestamp, LabradorResult, SimpleStorage, WechatRequest, WechatCommonResponse, JsapiSignature, get_timestamp, get_nonce_str};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use crate::wechat::mp::method::WechatMpMethod;

mod api;
mod method;
pub mod events;
pub mod messages;
pub mod replies;
#[allow(unused)]
mod constants;

pub use api::*;
use crate::wechat::mp::constants::{ACCESS_TOKEN, APPID, CLIENT_CREDENTIAL, GRANT_TYPE, SECRET, TICKET_TYPE, TICKET_TYPE_JSAPI, TICKET_TYPE_SDK, TICKET_TYPE_WXCARD};
use crate::wechat::mp::method::WechatMpMethod::QrConnectUrl;

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct WechatMpClient<T: SessionStore> {
    appid: String,
    secret: String,
    token: Option<String>,
    template_id: Option<String>,
    aes_key: Option<String>,
    client: APIClient<T>,
}


#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct AccessTokenResponse{
    pub access_token: String,
    pub expires_in: i64,
}

#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct WechatMpShortKeyResponse{
    /// 长信息
    pub long_data: Option<String>,
    /// 创建的时间戳
    pub create_time: Option<i64>,
    /// 剩余的过期秒数
    pub expire_seconds: Option<i64>,
}

pub enum TicketType {
    /// jsapi
    JSAPI,
    /// sdk
    SDK,
    /// 微信卡券
    WxCard
}

impl ToString for TicketType {
    fn to_string(&self) -> String {
        match self {
            TicketType::JSAPI => TICKET_TYPE_JSAPI.to_string(),
            TicketType::SDK => TICKET_TYPE_SDK.to_string(),
            TicketType::WxCard => TICKET_TYPE_WXCARD.to_string(),
        }
    }
}

#[allow(unused)]
impl<T: SessionStore> WechatMpClient<T> {

    fn from_client(client: APIClient<T>) -> WechatMpClient<T> {
        WechatMpClient {
            appid: client.app_key.to_owned(),
            secret: client.secret.to_owned(),
            token: None,
            template_id: None,
            aes_key: None,
            client
        }
    }

    /// get the wechat client
    pub fn new<S: Into<String>>(appid: S, secret: S) -> WechatMpClient<SimpleStorage> {
        let client = APIClient::<SimpleStorage>::from_session(appid.into(), secret.into(), "https://api.weixin.qq.com", SimpleStorage::new());
        WechatMpClient::from_client(client)
    }

    /// get the wechat client
    pub fn from_session<S: Into<String>>(appid: S, secret: S, session: T) -> WechatMpClient<T> {
        let client = APIClient::from_session(appid.into(), secret.into(), "https://api.weixin.qq.com", session);
        Self::from_client(client)
    }

    pub fn aes_key(mut self, aes_key: &str) -> Self {
        self.aes_key = aes_key.to_string().into();
        self
    }

    pub fn token(mut self, token: &str) -> Self {
        self.token = token.to_string().into();
        self
    }

    pub fn template_id(mut self, template_id: &str) -> Self {
        self.template_id = template_id.to_string().into();
        self
    }

    #[inline]
    pub async fn access_token(&self, force_refresh: bool) -> LabradorResult<String> {
        let session = self.client.session();
        let token_key = format!("{}_access_token", self.appid);
        let expires_key = format!("{}_expires_at", self.appid);
        let token: String = session.get(&token_key, Some("".to_owned()))?.unwrap_or_default();
        let timestamp = current_timestamp();
        let expires_at: i64 = session.get(&expires_key, Some(timestamp))?.unwrap_or_default();
        if expires_at <= timestamp || force_refresh {
            let mut req = LabraRequest::<String>::new().url(WechatMpMethod::AccessToken.get_method()).params(vec![
                (GRANT_TYPE.to_string(), CLIENT_CREDENTIAL.to_string()),
                (APPID.to_string(), self.client.app_key.to_string()),
                (SECRET.to_string(), self.client.secret.to_string()),
            ]).method(Method::Get).req_type(RequestType::Json);
            let res = self.client.request(req).await?.json::<AccessTokenResponse>()?;
            let token = res.access_token;
            let expires_in = res.expires_in;
            // 预留200秒的时间
            let expires_at = current_timestamp() + expires_in - 200;
            session.set(&token_key, token.to_owned(), Some(expires_in as usize));
            session.set(&expires_key, expires_at, Some(expires_in as usize));
            Ok(token)
        } else {
            Ok(token)
        }
    }

    /// <pre>
    /// 短key托管 类似于短链API.
    /// 详情请见: https://developers.weixin.qq.com/doc/offiaccount/Account_Management/KEY_Shortener.html
    /// </pre>
    #[inline]
    pub async fn gen_shorten(&self, long_data: &str, expire_seconds: u64) -> LabradorResult<String> {
        let res = self.post(WechatMpMethod::GenShortenUrl, vec![], json!({"long_data": long_data, "expire_seconds": expire_seconds}), RequestType::Json).await?.json::<Value>()?;
        let v = WechatCommonResponse::parse::<Value>(res)?;
        let short_key = v["short_key"].as_str().unwrap_or_default();
        Ok(short_key.to_string())
    }

    /// <pre>
    /// 短key解析 将短key还原为长信息。
    /// 详情请见: https://developers.weixin.qq.com/doc/offiaccount/Account_Management/KEY_Shortener.html
    /// </pre>
    #[inline]
    pub async fn fetch_shorten(&self, short_key: &str) -> LabradorResult<WechatMpShortKeyResponse> {
        let res = self.post(WechatMpMethod::GenShortenUrl, vec![], json!({"short_key": short_key}), RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatMpShortKeyResponse>(res)
    }

    /// <pre>
    /// 获得ticket,不强制刷新ticket.
    /// <a href="https://developers.weixin.qq.com/doc/offiaccount/OA_Web_Apps/JS-SDK.html#63">链接</a>
    /// </pre>
    #[inline]
    pub async fn get_ticket(&self, ticket_type: TicketType) -> LabradorResult<String> {
        self.get_ticket_force(ticket_type, false).await
    }

    /// <pre>
    /// 获得ticket.
    /// 获得时会检查 Token是否过期，如果过期了，那么就刷新一下，否则就什么都不干
    /// <a href="https://developers.weixin.qq.com/doc/offiaccount/OA_Web_Apps/JS-SDK.html#63">链接</a>
    /// </pre>
    #[inline]
    pub async fn get_ticket_force(&self, ticket_type: TicketType, force_refresh: bool) -> LabradorResult<String> {
        let session = self.client.session();
        let key = format!("{}_{}_ticket", self.appid, &ticket_type.to_string());
        let expires_key = format!("{}_{}_ticket_expires_at", self.appid, &ticket_type.to_string());
        let ticket: String = session.get(&key, Some("".to_owned()))?.unwrap_or_default();
        let timestamp = current_timestamp();
        let expires_at: i64 = session.get(&expires_key, Some(timestamp))?.unwrap_or_default();
        if expires_at <= timestamp || force_refresh {
            let res = self.get(WechatMpMethod::GetTicket, vec![(TICKET_TYPE.to_string(), ticket_type.to_string())], RequestType::Json).await?.json::<Value>()?;
            let v = WechatCommonResponse::parse::<Value>(res)?;
            let ticket = v["ticket"].as_str().unwrap_or_default();
            let expires_in = v["expires_in"].as_i64().unwrap_or_default();
            // 预留200秒的时间
            let expires_at = current_timestamp() + expires_in - 200;
            session.set(&key, ticket.to_string(), Some(expires_in as usize));
            session.set(&expires_key, expires_at, Some(expires_in as usize));
            Ok(ticket.to_string())
        } else {
            Ok(ticket)
        }
    }

    ///
    /// <pre>
    /// 创建调用jsapi时所需要的签名.
    ///
    /// 详情请见：<a href="http://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1421141115&token=&lang=zh_CN">链接</a>
    /// </pre>
    pub async fn create_jsapi_signature(&self, url: &str) -> LabradorResult<JsapiSignature> {
        let timestamp = get_timestamp() / 1000;
        let noncestr = get_nonce_str();
        let jsapi_ticket = self.get_jsapi_ticket(false).await?;
        let signature = WechatCrypto::get_sha1_sign(&vec!["jsapi_ticket=".to_string() + &jsapi_ticket,
                                                          "noncestr=".to_string() + &noncestr,
                                                          "timestamp=".to_string() + &timestamp.to_string(),"url=".to_string() + &url].join("&"));
        Ok(JsapiSignature{
            app_id: self.appid.to_string(),
            nonce_str: noncestr,
            url: url.to_string(),
            signature,
            timestamp,
        })
    }

    ///
    /// <pre>
    /// 构造第三方使用网站应用授权登录的url.
    /// 详情请见: <a href="https://open.weixin.qq.com/cgi-bin/showdocument?action=dir_list&t=resource/res_list&verify=1&id=open1419316505&token=&lang=zh_CN">网站应用微信登录开发指南</a>
    /// URL格式为https://open.weixin.qq.com/connect/qrconnect?appid=APPID&redirect_uri=REDIRECT_URI&response_type=code&scope=SCOPE&state=STATE#wechat_redirect
    /// </pre>
    pub async fn build_qr_connect_url(&self, redirect_url: &str, scope: &str, state: &str, ) -> LabradorResult<String> {
        Ok(format!("{}?appid={}&redirect_uri={}&response_type=code&scope={}&state={}#wechat_redirect", QrConnectUrl.get_method(), self.appid.to_string(), urlencoding::encode(redirect_url), scope, state))
    }

    ///
    /// <pre>
    /// 获取微信服务器的ip段
    /// [文档](http://mp.weixin.qq.com/wiki/0/2ad4b6bfd29f30f71d39616c2a0fcedc.html)
    /// </pre>
    pub async fn get_callback_ip(&self, force_refresh: bool) -> LabradorResult<Vec<String>> {
        let v = self.get(WechatMpMethod::GetCallbackIp, vec![], RequestType::Json).await?.json::<Value>()?;
        let v = WechatCommonResponse::parse::<Value>(v)?;
        let ip_list = v["ip_list"].as_array().unwrap_or(&vec![]).iter().map(|v| v.as_str().unwrap_or_default().to_string()).collect::<Vec<String>>();
        Ok(ip_list)
    }

    ///
    /// <pre>
    /// 获得jsapi_ticket.
    /// 获得时会检查jsapiToken是否过期，如果过期了，那么就刷新一下，否则就什么都不干
    ///
    /// 详情请见：<a href="http://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1421141115&token=&lang=zh_CN">链接</a>
    /// </pre>
    pub async fn get_jsapi_ticket(&self, force_refresh: bool) -> LabradorResult<String> {
        self.get_ticket_force(TicketType::JSAPI, force_refresh).await
    }



    ///
    /// <pre>
    /// 验证消息的确来自微信服务器.
    /// 详情(http://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1421135319&token=&lang=zh_CN)
    /// </pre>
    pub fn check_signature(&self, signature: &str, timestamp: i64, nonce: &str) -> LabradorResult<bool> {
        let crp = WechatCrypto::new(&self.aes_key.to_owned().unwrap_or_default());
        let _ = crp.check_signature(signature, timestamp, nonce, "", &self.token.to_owned().unwrap_or_default())?;
        Ok(true)
    }

    /// 发送POST请求
    async fn post<D: Serialize>(&self, method: WechatMpMethod, mut querys: Vec<(String, String)>, data: D, request_type: RequestType) -> LabradorResult<LabraResponse> {
        let access_token = self.access_token(false).await?;
        if !access_token.is_empty() && method.need_token() {
            querys.push((ACCESS_TOKEN.to_string(), access_token));
        }
        self.client.post(method, querys, data, request_type).await
    }

    ///<pre>
    /// Service没有实现某个API的时候，可以用这个，
    /// 比 get 和 post 方法更灵活，可以自己构造用来处理不同的参数和不同的返回类型。
    /// </pre>
    async fn execute<D: WechatRequest, B: Serialize>(&self, request: D) -> LabradorResult<LabraResponse> {
        let mut querys = request.get_query_params();
        if request.is_need_token() {
            let access_token = self.access_token(false).await?;
            if !access_token.is_empty() {
                querys.insert(ACCESS_TOKEN.to_string(), access_token);
            }
        }
        let params = querys.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect::<Vec<(String, String)>>();
        let mut req = LabraRequest::<B>::new().url(request.get_api_method_name())
            .params(params).method(request.get_request_method()).req_type(request.get_request_type()).body(request.get_request_body::<B>());
        self.client.request(req).await
    }

    /// 发送GET请求
    async fn get(&self, method: WechatMpMethod, mut params: Vec<(String, String)>, request_type: RequestType) -> LabradorResult<LabraResponse> {
        let access_token = self.access_token(false).await?;
        if !access_token.is_empty() && method.need_token() {
            params.push((ACCESS_TOKEN.to_string(), access_token));
        }
        self.client.get(method, params, request_type).await
    }

    /// 用户相关服务
    pub fn user(&self) -> WechatMpUser<T> {
        WechatMpUser::new(self)
    }

    /// Oauth2授权相关服务
    pub fn oauth2(&self) -> WechatMpOauth2<T> {
        WechatMpOauth2::new(self)
    }

    /// qrcode相关服务
    pub fn qrcode(&self) -> WechatMpQRCode<T> {
        WechatMpQRCode::new(self)
    }

    /// 客服相关服务
    pub fn custom_service(&self) -> WechatMpCustomService<T> {
        WechatMpCustomService::new(self)
    }

    /// 菜单相关服务
    pub fn menu(&self) -> WechatMpMenu<T> {
        WechatMpMenu::new(self)
    }

    /// 多媒体服务
    pub fn media(&self) -> WechatMpMedia<T> {
        WechatMpMedia::new(self)
    }

    /// 模板消息服务
    pub fn template_msg(&self) -> WechatMpTemplateMessage<T> {
        WechatMpTemplateMessage::new(self)
    }

    /// 订阅消息服务
    pub fn subscribe_msg(&self) -> WechatMpSubscribeMessage<T> {
        WechatMpSubscribeMessage::new(self)
    }

    /// Wifi服务
    pub fn wifi(&self) -> WechatMpWifi<T> {
        WechatMpWifi::new(self)
    }

    /// OCR服务
    pub fn ocr(&self) -> WechatMpOcr<T> {
        WechatMpOcr::new(self)
    }

}
