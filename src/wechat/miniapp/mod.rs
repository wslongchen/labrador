use crate::{session::SessionStore, client::APIClient, request::{Method, RequestType, LabraResponse, LabraRequest, RequestMethod}, util::current_timestamp, LabradorResult, SimpleStorage, WechatCrypto, WechatRequest};
use serde::{Serialize, Deserialize};

mod method;
mod api;
#[allow(unused)]
mod constants;

pub use api::*;
use crate::wechat::miniapp::constants::{ACCESS_TOKEN, APPID, CLIENT_CREDENTIAL, GRANT_TYPE, SECRET};
use crate::wechat::miniapp::method::WechatMaMethod;

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct WechatMaClient<T: SessionStore> {
    appid: String,
    secret: String,
    token: Option<String>,
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
impl<T: SessionStore> WechatMaClient<T> {

    fn from_client(client: APIClient<T>) -> WechatMaClient<T> {
        WechatMaClient {
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
    pub fn new<S: Into<String>>(appid: S, secret: S) -> WechatMaClient<SimpleStorage> {
        let client = APIClient::<SimpleStorage>::from_session(appid.into(), secret.into(), "https://api.weixin.qq.com", SimpleStorage::new());
        WechatMaClient::<SimpleStorage>::from_client(client)
    }

    /// get the wechat client
    pub fn from_session<S: Into<String>>(appid: S, secret: S, session: T) -> WechatMaClient<T> {
        let client = APIClient::from_session(appid.into(), secret.into(), "https://api.weixin.qq.com", session);
        Self::from_client(client)
    }

    #[inline]
    pub async fn access_token(&self, force_refresh: bool) -> LabradorResult<String> {
        let mut session = self.client.session();
        let token_key = format!("{}_access_token", self.appid);
        let expires_key = format!("{}_expires_at", self.appid);
        let token: String = session.get(&token_key, Some("".to_owned()))?.unwrap_or_default();
        let timestamp = current_timestamp();
        let expires_at: i64 = session.get(&expires_key, Some(timestamp))?.unwrap_or_default();
        if expires_at <= timestamp || force_refresh {
            let mut req = LabraRequest::<String>::new().url(WechatMaMethod::AccessToken.get_method()).params(vec![
                (GRANT_TYPE.to_string(), CLIENT_CREDENTIAL.to_string()),
                (APPID.to_string(), self.client.app_key.to_string()),
                (SECRET.to_string(), self.client.secret.to_string()),
            ]).method(Method::Get).req_type(RequestType::Json);
            let res = self.client.request(req).await?.json::<AccessTokenResponse>()?;
            let token = res.access_token;
            let expires_in = res.expires_in;
            // ??????200????????????
            let expires_at = current_timestamp() + expires_in - 200;
            session.set(&token_key, token.to_owned(), Some(expires_in as usize));
            session.set(&expires_key, expires_at, Some(expires_in as usize));
            Ok(token)
        } else {
            Ok(token)
        }
    }

    ///
    /// <pre>
    /// ???????????????????????????????????????.
    /// ??????(http://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1421135319&token=&lang=zh_CN)
    /// </pre>
    pub fn check_signature(&self, signature: &str, timestamp: i64, nonce: &str) -> LabradorResult<bool> {
        let crp = WechatCrypto::new(&self.aes_key.to_owned().unwrap_or_default()).token(&self.token.to_owned().unwrap_or_default());
        let _ = crp.check_signature(signature, timestamp, nonce, "")?;
        Ok(true)
    }

    ///<pre>
    /// Service??????????????????API??????????????????????????????
    /// ??? get ??? post ??????????????????????????????????????????????????????????????????????????????????????????
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

    /// ??????POST??????
    async fn post<D: Serialize>(&self, method: WechatMaMethod, mut querys: Vec<(String, String)>, data: D, request_type: RequestType) -> LabradorResult<LabraResponse> {
        let access_token = self.access_token(false).await?;
        if !access_token.is_empty() && method.need_token() {
            querys.push((ACCESS_TOKEN.to_string(), access_token));
        }
        self.client.post(method, querys, data, request_type).await
    }

    /// ??????GET??????
    async fn get(&self, method: WechatMaMethod, mut params: Vec<(String, String)>, request_type: RequestType) -> LabradorResult<LabraResponse> {
        let access_token = self.access_token(false).await?;
        if !access_token.is_empty() && method.need_token() {
            params.push((ACCESS_TOKEN.to_string(), access_token));
        }
        self.client.get(method, params, request_type).await
    }

    /// codesssion????????????
    pub fn code_session(&self) -> WechatMaCodeSession<T> {
        WechatMaCodeSession::new(self)
    }

    /// ???????????????????????????
    pub fn qrcode(&self) -> WechatMaQrcode<T> {
        WechatMaQrcode::new(self)
    }
    /// ????????????????????????
    pub fn user(&self) -> WechatMaUser<T> {
        WechatMaUser::new(self)
    }
    /// ??????????????????
    pub fn media(&self) -> WechatMaMedia<T> {
        WechatMaMedia::new(self)
    }
    /// ??????????????????
    pub fn message(&self) -> WechatMaMessage<T> {
        WechatMaMessage::new(self)
    }

}
