use std::collections::BTreeMap;
use crate::{session::SessionStore, client::APIClient, request::{Method, RequestType, LabraResponse, LabraRequest, RequestMethod}, util::current_timestamp, LabradorResult, SimpleStorage, WeChatCrypto, RequestBody};
use serde::{Serialize, Deserialize};

mod method;
mod api;

pub use api::*;
use crate::wechat::miniapp::method::WechatMaMethod;

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct WeChatMaClient<T: SessionStore> {
    appid: String,
    secret: String,
    token: Option<String>,
    aes_key: Option<String>,
    client: APIClient<T>,
}

pub trait WechatRequest {
    ///
    /// 获取TOP的API名称。
    ///
    /// @return API名称
    fn get_api_method_name(&self) -> String;

    /// 获取请求类型。
    fn get_request_type(&self) -> RequestType {
        RequestType::Json
    }

    /// 获取请求方法。
    fn get_request_method(&self) -> Method {
        Method::Post
    }

    fn get_query_params(&self) -> BTreeMap<String, String> {
        BTreeMap::new()
    }

    fn get_request_body<T: Serialize>(&self) -> RequestBody<T> {
        RequestBody::Null
    }

    /// 是否需要token
    fn is_need_token(&self) -> bool {
        true
    }

}

#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct AccessTokenResponse{
    pub access_token: String,
    pub expires_in: i64,
}

#[allow(unused)]
impl<T: SessionStore> WeChatMaClient<T> {

    fn from_client(client: APIClient<T>) -> WeChatMaClient<T> {
        WeChatMaClient {
            appid: client.app_key.to_owned(),
            secret: client.secret.to_owned(),
            token: None,
            aes_key: None,
            client
        }
    }

    pub fn aes_key(mut self, aes_key: &str) -> Self {
        self.aes_key = aes_key.to_string().into();
        self
    }

    pub fn token(mut self, token: &str) -> Self {
        self.token = token.to_string().into();
        self
    }

    /// get the wechat client
    pub fn new<S: Into<String>>(appid: S, secret: S) -> WeChatMaClient<SimpleStorage> {
        let client = APIClient::<SimpleStorage>::from_session(appid.into(), secret.into(), "https://api.weixin.qq.com", SimpleStorage::new());
        WeChatMaClient::<SimpleStorage>::from_client(client)
    }

    /// get the wechat client
    pub fn from_session<S: Into<String>>(appid: S, secret: S, session: T) -> WeChatMaClient<T> {
        let client = APIClient::from_session(appid.into(), secret.into(), "https://api.weixin.qq.com", session);
        Self::from_client(client)
    }

    #[inline]
    pub fn access_token(&self) -> String {
        let mut session = self.client.session();
        let token_key = format!("{}_access_token", self.appid);
        let expires_key = format!("{}_expires_at", self.appid);
        let token: String = session.get(&token_key, Some("".to_owned())).unwrap_or_default();
        let timestamp = current_timestamp();
        let expires_at: i64 = session.get(&expires_key, Some(timestamp)).unwrap_or_default();
        if expires_at <= timestamp {
            "".to_owned()
        } else {
            token
        }
    }

    async fn fetch_access_token(&self) -> LabradorResult<String> {
        let mut session = self.client.session();
        let mut req = LabraRequest::<String>::new().url(WechatMaMethod::AccessToken.get_method()).params(vec![
            ("grant_type".to_string(), "client_credential".to_string()),
            ("appid".to_string(), self.client.app_key.to_string()),
            ("secret".to_string(), self.client.secret.to_string()),
        ]).method(Method::Get).req_type(RequestType::Json);
        let res = self.client.request(req).await?.json::<AccessTokenResponse>()?;
        let token = res.access_token;
        let expires_in = res.expires_in;
        let expires_at = current_timestamp() + expires_in;
        let token_key = format!("{}_access_token", self.appid);
        let expires_key = format!("{}_expires_at", self.appid);
        session.set(&token_key, token.to_owned(), Some(expires_in as usize));
        session.set(&expires_key, expires_at, Some(expires_in as usize));
        Ok(token.to_owned())
    }

    ///
    /// <pre>
    /// 验证消息的确来自微信服务器.
    /// 详情(http://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1421135319&token=&lang=zh_CN)
    /// </pre>
    pub fn check_signature(&self, signature: &str, timestamp: i64, nonce: &str, echo_str: &str) -> LabradorResult<bool> {
        let crp = WeChatCrypto::new(&self.aes_key.to_owned().unwrap_or_default());
        let _ = crp.check_signature(signature, timestamp, nonce, echo_str, "", &self.token.to_owned().unwrap_or_default())?;
        Ok(true)
    }

    ///<pre>
    /// Service没有实现某个API的时候，可以用这个，
    /// 比 get 和 post 方法更灵活，可以自己构造用来处理不同的参数和不同的返回类型。
    /// </pre>
    async fn execute<D: WechatRequest, B: Serialize>(&self, request: D) -> LabradorResult<LabraResponse> {
        let mut querys = Vec::new();
        if request.is_need_token() {
            let mut access_token = self.access_token();
            if access_token.is_empty() {
                access_token = self.fetch_access_token().await.unwrap_or_default();
            }
            if !access_token.is_empty() {
                querys.push(("access_token".to_string(), access_token));
            }
        }
        let mut req = LabraRequest::<B>::new().url(request.get_api_method_name())
            .params(querys).method(request.get_request_method()).req_type(request.get_request_type()).body(request.get_request_body::<B>());
        self.client.request(req).await
    }

    /// 发送POST请求
    async fn post<D: Serialize>(&self, method: WechatMaMethod, mut querys: Vec<(String, String)>, data: D, request_type: RequestType) -> LabradorResult<LabraResponse> {
        let mut access_token = self.access_token();
        if access_token.is_empty() && method.need_token() {
            access_token = self.fetch_access_token().await.unwrap_or_default();
        }

        if !access_token.is_empty() {
            querys.push(("access_token".to_string(), access_token));
        }
        let mut req = LabraRequest::new().url(method.get_method()).params(querys).method(Method::Post).json(data).req_type(request_type);
        self.client.request(req).await
    }

    /// 发送GET请求
    async fn get(&self, method: WechatMaMethod, params: Vec<(&str, &str)>, request_type: RequestType) -> LabradorResult<LabraResponse> {
        let mut access_token = self.access_token();
        if access_token.is_empty() && method.need_token() {
            access_token = self.fetch_access_token().await?;
        }
        let mut querys = params.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect::<Vec<(String,String)>>();
        if !access_token.is_empty() {
            querys.push(("access_token".to_string(), access_token));
        }
        let mut req = LabraRequest::<String>::new().url(method.get_method()).params(querys).method(Method::Get).req_type(request_type);
        self.client.request(req).await
    }

    /// codesssion相关服务
    pub fn code_session(&self) -> WechatMaCodeSession<T> {
        WechatMaCodeSession::new(self)
    }

    /// 二维码相关操作接口
    pub fn qrcode(&self) -> WechatMaQrcode<T> {
        WechatMaQrcode::new(self)
    }
    /// 用户相关操作接口
    pub fn user(&self) -> WeChatMaUser<T> {
        WeChatMaUser::new(self)
    }

}
