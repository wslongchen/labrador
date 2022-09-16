use bytes::Bytes;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

use crate::{session::SessionStore, request::{RequestType}, WechatCommonResponse, LabradorResult, WechatCrypto, current_timestamp, LabraError, JsapiTicket, JsapiSignature, get_timestamp, get_nonce_str, APIClient, WechatRequest, LabraResponse, LabraRequest, SimpleStorage, WechatCpProviderToken};
use crate::wechat::cp::constants::{ACCESS_TOKEN, ACCESS_TOKEN_KEY, AUTH_URL_INSTALL, SUITE_ACCESS_TOKEN, TYPE};
use crate::wechat::cp::method::WechatCpMethod;
use crate::wechat::cp::AccessTokenResponse;

mod tag;
mod license;
mod media;
mod department;
mod user;
mod order;
mod agent;

pub use tag::*;
pub use license::*;
pub use media::*;
pub use department::*;
pub use user::*;
pub use order::*;
pub use agent::*;


/// 企业微信第三方应用API
#[allow(unused)]
#[derive(Debug, Clone)]
pub struct WechatCpTpClient<T: SessionStore> {
    token: Option<String>,
    /// 企微服务商企业ID，来自于企微配置
    corp_id: String,
    /// 第三方应用的EncodingAESKey，用来检查签名
    aes_key: Option<String>,
    ///企业secret，来自于企微配置
    corp_secret: String,
    /// 服务商secret
    provider_secret: Option<String>,
    agent_id: Option<i32>,
    /// 第三方应用的其他配置
    suite_id: Option<String>,
    suite_secret: Option<String>,
    client: APIClient<T>,
}

#[allow(unused)]
impl<T: SessionStore> WechatCpTpClient<T> {

    fn from_client(client: APIClient<T>) -> WechatCpTpClient<T> {
        WechatCpTpClient {
            corp_id: client.app_key.to_owned(),
            corp_secret: client.secret.to_owned(),
            token: None,
            aes_key: None,
            agent_id: None,
            suite_id: None,
            suite_secret: None,
            client,
            provider_secret: None
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

    pub fn agent_id(mut self, agent_id: i32) -> Self {
        self.agent_id = agent_id.into();
        self
    }

    pub fn provider_secret(mut self, provider_secret: &str) -> Self {
        self.provider_secret = provider_secret.to_string().into();
        self
    }

    pub fn suite_id(mut self, suite_id: &str) -> Self {
        self.suite_id = suite_id.to_string().into();
        self
    }

    pub fn suite_secret(mut self, suite_secret: &str) -> Self {
        self.suite_secret = suite_secret.to_string().into();
        self
    }

    fn key_with_prefix(&self, key: &str) -> String {
        format!("cp:{}:{}", self.suite_id.to_owned().unwrap_or_default(), key)
    }

    /// get the wechat client
    pub fn new<S: Into<String>>(crop_id: S, crop_secret: S) -> WechatCpTpClient<SimpleStorage> {
        let client = APIClient::<SimpleStorage>::from_session(crop_id.into(), crop_secret.into(), "https://qyapi.weixin.qq.com", SimpleStorage::new());
        WechatCpTpClient::<SimpleStorage>::from_client(client)
    }

    /// get the wechat client
    pub fn from_session<S: Into<String>>(crop_id: S, crop_secret: S, session: T) -> WechatCpTpClient<T> {
        let client = APIClient::from_session(crop_id.into(), crop_secret.into(), "https://qyapi.weixin.qq.com", session);
        Self::from_client(client)
    }

    /// 授权企业的access token相关
    fn get_access_token(&self, auth_corp_id: &str) -> String {
        let session = self.client.session();
        session.get::<_,String>(self.key_with_prefix(auth_corp_id) + ACCESS_TOKEN_KEY, None).unwrap_or(None).unwrap_or_default()
    }

    /// <pre>
    /// 验证推送过来的消息的正确性
    /// 详情请见: <a href="https://work.weixin.qq.com/api/doc#90000/90139/90968/消息体签名校验">文档</a>
    /// </pre>
    pub fn check_signature(&self, signature: &str, timestamp: i64, nonce: &str, data: &str) -> LabradorResult<bool> {
        let crp = WechatCrypto::new(&self.aes_key.to_owned().unwrap_or_default()).token(&self.token.to_owned().unwrap_or_default());
        let _ = crp.check_signature(signature, timestamp, nonce, data)?;
        Ok(true)
    }

    /// 获取加密工具
    pub fn get_crypto(&self) -> WechatCrypto {
        let crp = WechatCrypto::new(&self.aes_key.to_owned().unwrap_or_default()).token(&self.token.to_owned().unwrap_or_default());
        crp
    }

    /// 获得suite_ticket,不强制刷新suite_ticket
    /// 由微信服务器推送
    pub fn get_suite_ticket(&self) -> LabradorResult<String> {
        let session = self.client.session();
        let token_key = format!("{}_suite_ticket_key_cp", self.corp_id);
        let expires_key = format!("{}_suite_ticket_expires_at_cp", self.corp_id);
        let token: String = session.get(&token_key, Some("".to_owned()))?.unwrap_or_default();
        let timestamp = current_timestamp();
        let expires_at: i64 = session.get(&expires_key, Some(timestamp))?.unwrap_or_default();
        if expires_at <= timestamp {
            return Err(LabraError::ApiError("invaild suite ticket".to_string()))
        }
        Ok(token)
    }

    /// 获得suite_ticket,不强制刷新suite_ticket
    /// 由微信服务器推送
    pub fn set_suite_ticket_expire(&self, suite_ticket: &str, expire_second: i64) -> LabradorResult<()> {
        let expires_at = current_timestamp() + expire_second;
        let session = self.client.session();
        let token_key = format!("{}_suite_ticket_key_cp", self.corp_id);
        let expires_key = format!("{}_suite_ticket_expires_at_cp", self.corp_id);
        session.set(token_key, suite_ticket, Some(expire_second as usize))?;
        session.set(expires_key, expires_at, Some(expire_second as usize))?;
        Ok(())
    }

    /// <pre>
    /// 保存企业微信定时推送的suite_ticket,（每10分钟）
    /// 详情请见：<a href="https://work.weixin.qq.com/api/doc#90001/90143/90628">文档</a>
    ///
    /// 注意：微信不是固定10分钟推送suite_ticket的, 且suite_ticket的有效期为30分钟
    /// <a href="https://work.weixin.qq.com/api/doc/10975#%E8%8E%B7%E5%8F%96%E7%AC%AC%E4%B8%89%E6%96%B9%E5%BA%94%E7%94%A8%E5%87%AD%E8%AF%81">文档</a>
    /// </pre>
    pub fn set_suite_ticket(&self, suite_ticket: &str) -> LabradorResult<()> {
        self.set_suite_ticket_expire(suite_ticket, 28 * 60)
    }

    /// <pre>
    /// 获取suite_access_token，本方法线程安全
    /// 且在多线程同时刷新时只刷新一次，避免超出2000次/日的调用次数上限
    /// 另：本service的所有方法都会在suite_access_token过期是调用此方法
    /// 程序员在非必要情况下尽量不要主动调用此方法
    /// 详情请见: <a href="https://work.weixin.qq.com/api/doc#90001/90143/90600">文档</a>
    /// </pre>
    pub async fn get_suite_access_token(&self) -> LabradorResult<String> {
        self.get_suite_access_token_force(false).await
    }

    /// <pre>
    /// 获取suite_access_token，本方法线程安全
    /// 且在多线程同时刷新时只刷新一次，避免超出2000次/日的调用次数上限
    /// 另：本service的所有方法都会在suite_access_token过期是调用此方法
    /// 详情请见: <a href="https://work.weixin.qq.com/api/doc#90001/90143/90600">文档</a>
    /// </pre>
    pub async fn get_suite_access_token_force(&self, force_refresh: bool) -> LabradorResult<String> {
        let session = self.client.session();
        let token_key = format!("{}_suite_access_token_cp", self.corp_id);
        let expires_key = format!("{}_suite_access_token_expires_at_cp", self.corp_id);
        let token: String = session.get(&token_key, Some("".to_owned()))?.unwrap_or_default();
        let timestamp = current_timestamp();
        let expires_at: i64 = session.get(&expires_key, Some(timestamp))?.unwrap_or_default();
        if expires_at <= timestamp || force_refresh {
            let suite_ticket = self.get_suite_ticket()?;
            let req = json!({
                "suite_id": self.suite_id,
                "suite_secret": self.suite_secret,
                "suite_ticket": suite_ticket
            });
            let result = self.client.post(WechatCpMethod::GetSuiteToken, vec![], req, RequestType::Json).await?.json::<WechatCpSuiteAccessTokenResponse>()?;
            let token = result.suite_access_token;
            let expires_in = result.expires_in;
            // 预留200秒的时间
            let expires_at = current_timestamp() + expires_in - 200;
            session.set(&token_key, token.to_owned(), Some(expires_in as usize));
            session.set(&expires_key, expires_at, Some(expires_in as usize));
            Ok(token.to_string())
        } else {
            Ok(token)
        }
    }

    ///
    /// <pre>
    /// 获取应用的 jsapi ticket
    /// </pre>
    pub async fn get_suite_jsapi_ticket(&self, auth_corp_id: &str) -> LabradorResult<String> {
        self.get_suite_jsapi_ticket_force(auth_corp_id, false).await
    }

    ///
    /// <pre>
    /// 获取应用的 jsapi ticket， 支持强制刷新
    /// </pre>
    pub async fn get_suite_jsapi_ticket_force(&self, auth_corp_id: &str, force_refresh: bool) -> LabradorResult<String> {
        let mut session = self.client.session();
        let ticket_key = format!("{}_suite_jsapi_ticket_cp", self.corp_id);
        let expires_key = format!("{}_suite_jsapi_ticket_expires_at_cp", self.corp_id);
        let ticket: String = session.get(&ticket_key, Some("".to_string()))?.unwrap_or_default();
        let timestamp = current_timestamp();
        let expires_at: i64 = session.get(&expires_key, Some(timestamp))?.unwrap_or_default();
        if expires_at <= timestamp || force_refresh {
            let res = self.client.get(WechatCpMethod::GetSuiteJsapiTicket, vec![(TYPE.to_string(), "agent_config".to_string()), (ACCESS_TOKEN.to_string(), self.get_access_token(auth_corp_id))], RequestType::Json).await?.json::<JsapiTicket>()?;
            let ticket = res.ticket;
            let expires_in = res.expires_in;
            // 预留200秒的时间
            let expires_at = current_timestamp() + expires_in - 200;
            session.set(&ticket_key, ticket.to_string(), Some(expires_in as usize));
            session.set(&expires_key, expires_at, Some(expires_in as usize));
            Ok(ticket.to_string())
        } else {
            Ok(ticket)
        }
    }

    ///
    /// <pre>
    /// 获取授权企业的 jsapi ticket
    /// </pre>
    pub async fn get_auth_corp_jsapi_ticket(&self, auth_corp_id: &str) -> LabradorResult<String> {
        self.get_auth_corp_jsapi_ticket_force(auth_corp_id, false).await
    }

    ///
    /// <pre>
    /// 获取授权企业的 jsapi ticket， 支持强制刷新
    /// </pre>
    pub async fn get_auth_corp_jsapi_ticket_force(&self, auth_corp_id: &str, force_refresh: bool) -> LabradorResult<String> {
        let mut session = self.client.session();
        let ticket_key = format!("{}_auth_corp_jsapi_ticket_cp", self.corp_id);
        let expires_key = format!("{}_auth_corp_jsapi_ticket_expires_at_cp", self.corp_id);
        let ticket: String = session.get(&ticket_key, Some("".to_string()))?.unwrap_or_default();
        let timestamp = current_timestamp();
        let expires_at: i64 = session.get(&expires_key, Some(timestamp))?.unwrap_or_default();
        if expires_at <= timestamp || force_refresh {
            let res = self.client.get(WechatCpMethod::GetJsapiTicket, vec![(ACCESS_TOKEN.to_string(), self.get_access_token(auth_corp_id))], RequestType::Json).await?.json::<JsapiTicket>()?;
            let ticket = res.ticket;
            let expires_in = res.expires_in;
            // 预留200秒的时间
            let expires_at = current_timestamp() + expires_in - 200;
            session.set(&ticket_key, ticket.to_string(), Some(expires_in as usize));
            session.set(&expires_key, expires_at, Some(expires_in as usize));
            Ok(ticket.to_string())
        } else {
            Ok(ticket)
        }
    }



    /// <pre>
    /// 获取企业凭证
    /// </pre>
    pub async fn get_corp_token(&self, auth_corpid: &str, permanent_code: &str) -> LabradorResult<AccessTokenResponse> {
        self.get_corp_token_force(auth_corpid, permanent_code,false).await
    }

    /// <pre>
    /// 获取企业凭证, 支持强制刷新
    /// </pre>
    pub async fn get_corp_token_force(&self, auth_corpid: &str, permanent_code: &str, force_refresh: bool) -> LabradorResult<AccessTokenResponse> {
        let session = self.client.session();
        let token_key = format!("{}_corp_access_token_cp", auth_corpid);
        let expires_key = format!("{}_corp_access_token_expires_at_cp", auth_corpid);
        let token: String = session.get(&token_key, Some("".to_owned()))?.unwrap_or_default();
        let timestamp = current_timestamp();
        let expires_at: i64 = session.get(&expires_key, Some(timestamp))?.unwrap_or_default();
        if expires_at <= timestamp || force_refresh {
            let suite_ticket = self.get_suite_ticket()?;
            let req = json!({
                "auth_corpid": auth_corpid,
                "permanent_code": permanent_code,
            });
            let result = self.client.post(WechatCpMethod::GetCorpToken, vec![], req, RequestType::Json).await?.json::<AccessTokenResponse>()?;
            let token = result.access_token.to_string();
            let expires_in = result.expires_in;
            // 预留200秒的时间
            let expires_at = current_timestamp() + expires_in - 200;
            session.set(&token_key, token.to_owned(), Some(expires_in as usize));
            session.set(&expires_key, expires_at, Some(expires_in as usize));
            Ok(result)
        } else {
            Ok(AccessTokenResponse{ access_token: token.to_string(), expires_in: expires_at })
        }
    }

    /// <pre>
    /// 获取服务商providerToken
    /// </pre>
    pub async fn get_wechat_provider_token(&self) -> LabradorResult<String> {
        let session = self.client.session();
        let token_key = format!("{}_provider_access_token_cp", self.corp_id);
        let expires_key = format!("{}_provider_access_token_expires_at_cp", self.corp_id);
        let token: String = session.get(&token_key, Some("".to_owned()))?.unwrap_or_default();
        let timestamp = current_timestamp();
        let expires_at: i64 = session.get(&expires_key, Some(timestamp))?.unwrap_or_default();
        if expires_at <= timestamp {
            let suite_ticket = self.get_suite_ticket()?;
            let req = json!({
                "corpid": self.corp_id,
                "provider_secret": self.provider_secret,
            });
            let result = self.client.post(WechatCpMethod::GetProviderToken, vec![], req, RequestType::Json).await?.json::<WechatCpProviderToken>()?;
            let token = result.provider_access_token.to_string();
            let expires_in = result.expires_in;
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
    /// 获取企业永久授权码信息
    /// </pre>
    pub async fn get_permanent_code_info(&self, auth_code: &str) -> LabradorResult<WechatCpThirdPermanentCodeInfo> {
        let req = json!({
            "auth_code": auth_code,
        });
        let result = self.post(WechatCpMethod::GetPermanentCode, vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpThirdPermanentCodeInfo>(result)
    }

    /// <pre>
    /// 获取预授权链接
    /// </pre>
    pub async fn get_pre_auth_url(&self, redirect_uri: &str, state: Option<&str>) -> LabradorResult<String> {
        let result = self.get(WechatCpMethod::GetPreAuthCode, vec![], RequestType::Json).await?.json::<WechatCpThirdPreauthCode>()?;
        let mut pre_auth_url = format!("{}?suite_id={}&pre_auth_code={}&redirect_uri={}", AUTH_URL_INSTALL, self.suite_id.to_owned().unwrap_or_default(), result.pre_auth_code, urlencoding::encode(redirect_uri));
        if let Some(state) = state {
            pre_auth_url.push_str(&format!("&state={}", state));
        }
        Ok(pre_auth_url)
    }

    /// <pre>
    /// 设置授权配置
    /// </pre>
    pub async fn set_session_info(&self, pre_auth_code: &str, app_ids: Vec<&str>, auth_type: u8) -> LabradorResult<WechatCommonResponse> {
        let req = json!({
            "pre_auth_code": pre_auth_code,
            "session_info":
            {
                "appid": app_ids,
                "auth_type": auth_type
            }
        });
        self.post(WechatCpMethod::SetSessionInfo, vec![], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 获取企业的授权信息
    /// </pre>
    pub async fn get_auth_info(&self, auth_corp_id: &str, permanent_code: &str) -> LabradorResult<WechatCpThirdAuthInfo> {
        let req = json!({
           "auth_corpid": auth_corp_id,
           "permanent_code": permanent_code
        });
        let result = self.client.post(WechatCpMethod::GetAuthInfo, vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpThirdAuthInfo>(result)
    }


    /// <pre>
    /// 获取应用的管理员列表
    /// 第三方服务商可以用此接口获取授权企业中某个第三方应用的管理员列表(不包括外部管理员)，以便服务商在用户进入应用主页之后根据是否管理员身份做权限的区分。
    /// </pre>
    pub async fn get_admin_info(&self, auth_corp_id: &str, agent_id: i32) -> LabradorResult<Vec<AdminUserInfo>> {
        let req = json!({
           "auth_corpid": auth_corp_id,
           "agentid": agent_id
        });
        let result = self.client.post(WechatCpMethod::GetAdminInfo, vec![], req, RequestType::Json).await?.json::<Value>()?;
        let v = WechatCommonResponse::parse::<Value>(result)?;
        serde_json::from_value::<Vec<AdminUserInfo>>(v["admin"].to_owned()).map_err(LabraError::from)
    }


    /// <pre>
    /// 获取应用二维码
    /// 用于获取第三方应用二维码。
    /// </pre>
    pub async fn get_app_qrcode_buffer(&self, suite_id: &str, appid: Option<i32>, state: Option<&str>, style: Option<u8>) -> LabradorResult<Bytes> {
        let req = json!({
           "suite_id": suite_id,
            "appid": appid,
            "state": state,
            "style": style,
            "result_type": 1
        });
        self.client.post(WechatCpMethod::GetAppQrcode, vec![], req, RequestType::Json).await?.bytes()
    }

    /// <pre>
    /// 获取应用二维码
    /// 用于获取第三方应用二维码。
    /// </pre>
    pub async fn get_app_qrcode_url(&self, suite_id: &str, appid: Option<i32>, state: Option<&str>, style: Option<u8>, result_type: Option<u8>) -> LabradorResult<String> {
        let req = json!({
           "suite_id": suite_id,
            "appid": appid,
            "state": state,
            "style": style,
            "result_type": 2
        });
        let v = self.client.post(WechatCpMethod::GetAppQrcode, vec![], req, RequestType::Json).await?.json::<Value>()?;
        let v = WechatCommonResponse::parse::<Value>(v)?;
        let qrcode = v["qrcode"].as_str().unwrap_or_default();
        Ok(qrcode.to_string())
    }

    /// <pre>
    /// 明文corpid转换为加密corpid
    /// 为更好地保护企业与用户的数据，第三方应用获取的corpid不再是明文的corpid，将升级为第三方服务商级别的加密corpid（了解更多）。第三方可以将已有的明文corpid转换为第三方的加密corpid。。
    /// </pre>
    pub async fn corpid_to_opencorpid(&self, corpid: &str) -> LabradorResult<String> {
        let req = json!({
           "corpid": corpid,
        });
        let v = self.client.post(WechatCpMethod::CorpToOpenCorpid, vec![], req, RequestType::Json).await?.json::<Value>()?;
        let v = WechatCommonResponse::parse::<Value>(v)?;
        let qrcode = v["open_corpid"].as_str().unwrap_or_default();
        Ok(qrcode.to_string())
    }

    ///
    /// <pre>
    /// 创建机构级jsApiTicket签名
    /// 详情参见企业微信第三方应用开发文档[请见](https://work.weixin.qq.com/api/doc/90001/90144/90539)
    /// </pre>
    pub async fn create_auth_corp_jsapi_signature(&self, url: &str, auth_corp_id: &str) -> LabradorResult<JsapiSignature> {
        Ok(self.created_wechat_jsapi_signature(url, auth_corp_id, &self.get_auth_corp_jsapi_ticket(auth_corp_id).await?))
    }

    ///
    /// <pre>
    /// 创建应用级jsapiTicket签名
    /// 详情参见企业微信第三方应用开发文档[请见](https://work.weixin.qq.com/api/doc/90001/90144/90539)
    /// </pre>
    pub async fn create_suite_jsapi_signature(&self, url: &str, auth_corp_id: &str) -> LabradorResult<JsapiSignature> {
        Ok(self.created_wechat_jsapi_signature(url, auth_corp_id, &self.get_suite_jsapi_ticket(auth_corp_id).await?))
    }

    fn created_wechat_jsapi_signature(&self, url: &str, auth_corp_id: &str, jsapi_ticket: &str) -> JsapiSignature {
        let timestamp = get_timestamp() / 1000;
        let noncestr = get_nonce_str();
        let signature = WechatCrypto::get_sha1_sign(&vec!["jsapi_ticket=".to_string() + &jsapi_ticket,
                                                          "noncestr=".to_string() + &noncestr,
                                                          "timestamp=".to_string() + &timestamp.to_string(),"url=".to_string() + &url].join("&"));
        JsapiSignature{
            app_id: auth_corp_id.to_string(),
            nonce_str: noncestr,
            url: url.to_string(),
            signature,
            timestamp,
        }
    }

    ///<pre>
    /// Service没有实现某个API的时候，可以用这个，
    /// 比 get 和 post 方法更灵活，可以自己构造用来处理不同的参数和不同的返回类型。
    /// </pre>
    async fn execute<D: WechatRequest, B: Serialize>(&self, request: D, corp_id: Option<&str>) -> LabradorResult<LabraResponse> {
        let mut querys = request.get_query_params();
        if request.is_need_token() {
            if let Some(corp_id) = corp_id {
                let access_token = self.get_access_token(corp_id);
                if !access_token.is_empty() {
                    querys.insert(ACCESS_TOKEN.to_string(), access_token);
                }
            }
        }
        let params = querys.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect::<Vec<(String, String)>>();
        let mut req = LabraRequest::<B>::new().url(request.get_api_method_name())
            .params(params).method(request.get_request_method()).req_type(request.get_request_type()).body(request.get_request_body::<B>());
        self.client.request(req).await
    }

    /// 发送POST请求
    async fn post<D: Serialize>(&self, method: WechatCpMethod, mut querys: Vec<(String, String)>, data: D, request_type: RequestType) -> LabradorResult<LabraResponse> {
        if method.need_token() {
            let token = self.get_suite_access_token_force(false).await?;
            querys.push((SUITE_ACCESS_TOKEN.to_string(), token));
        }
        self.client.post(method, querys, data, request_type).await
    }

    /// 发送GET请求
    async fn get(&self, method: WechatCpMethod, mut params: Vec<(String, String)>, request_type: RequestType) -> LabradorResult<LabraResponse> {
        if method.need_token() {
            let token = self.get_suite_access_token_force(false).await?.to_string();
            params.push((SUITE_ACCESS_TOKEN.to_string(), token));
        }
        self.client.get(method, params, request_type).await
    }

    /// 部门
    pub fn department(&self) -> WechatCpTpDepartment<T> {
        WechatCpTpDepartment::new(self)
    }

    /// 接口调用许可
    pub fn license(&self) -> WechatCpTpLicense<T> {
        WechatCpTpLicense::new(self)
    }

    /// 媒体
    pub fn media(&self) -> WechatCpTpMedia<T> {
        WechatCpTpMedia::new(self)
    }

    /// 订单
    pub fn order(&self) -> WechatCpTpOrder<T> {
        WechatCpTpOrder::new(self)
    }

    /// 标签
    pub fn tag(&self) -> WechatCpTpTag<T> {
        WechatCpTpTag::new(self)
    }

    /// 用户
    pub fn user(&self) -> WechatCpTpUser<T> {
        WechatCpTpUser::new(self)
    }
}

//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WechatCpSuiteAccessTokenResponse {
    pub suite_access_token: String,
    pub expires_in: i64,
}

/// 服务商模式获取永久授权码信息
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WechatCpThirdPermanentCodeInfo {
    pub access_token: String,
    pub permanent_code: String,
    /// 授权企业信息
    pub auth_corp_info: AuthCorpInfo,
    /// 授权信息。如果是通讯录应用，且没开启实体应用，是没有该项的。通讯录应用拥有企业通讯录的全部信息读写权限
    pub auth_info: Option<AuthInfo>,
    /// 授权用户信息
    pub auth_user_info: Option<AuthUserInfo>,
    /// 企业当前生效的版本信息
    pub edition_info: Option<EditionInfo>,
    pub expires_in: i64,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthCorpInfo {
    pub corpid: String,
    pub corp_name: String,
    pub corp_type: Option<String>,
    pub corp_square_logo_url: Option<String>,
    pub corp_round_logo_url: Option<String>,
    pub corp_user_max: Option<String>,
    pub corp_agent_max: Option<String>,
    /// 所绑定的企业微信主体名称(仅认证过的企业有)
    pub corp_full_name: Option<String>,
    /// 授权企业在微工作台（原企业号）的二维码，可用于关注微工作台
    pub corp_wxqrcode: Option<String>,
    pub corp_scale: Option<String>,
    pub corp_industry: Option<String>,
    pub corp_sub_industry: Option<String>,
    pub location: Option<String>,
    /// 认证到期时间
    pub verified_end_time: Option<i64>,
    /// 企业类型，1. 企业; 2. 政府以及事业单位; 3. 其他组织, 4.团队号
    pub subject_type: Option<u8>,
}


/// 企业当前生效的版本信息
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditionInfo {
    pub agent: Option<Vec<Agent>>,
}


/// 授权人员信息
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthUserInfo {
    pub userid: Option<String>,
    pub name: Option<String>,
    pub avatar: Option<String>,
    /// 授权管理员的open_userid，可能为空
    pub open_userid: Option<String>,
}


/// 管理员信息
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminUserInfo {
    pub userid: Option<String>,
    /// 管理员的open_userid，可能为空
    pub open_userid: Option<String>,
    /// 该管理员对应用的权限：0=发消息权限，1=管理权限
    pub auth_type: Option<u8>,
}


/// 授权信息
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthInfo {
    /// 授权的应用信息，注意是一个数组，但仅旧的多应用套件授权时会返回多个agent，对新的单应用授权，永远只返回一个agent
    pub agent: Vec<Agent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Agent {
    pub agentid: i32,
    pub name: String,
    pub round_logo_url: Option<String>,
    pub square_logo_url: Option<String>,
    /// 版本id
    pub edition_id: Option<String>,
    /// 版本名称
    pub edition_name: Option<String>,
    /// 付费状态
    ///<br/>
    ///<ul>
    ///  <li>0-没有付费;</li>
    ///  <li>1-限时试用;</li>
    ///  <li>2-试用过期;</li>
    ///  <li>3-购买期内;</li>
    ///  <li>4-购买过期;</li>
    ///  <li>5-不限时试用;</li>
    ///  <li>6-购买期内，但是人数超标, 注意，超标后还可以用7天;</li>
    ///  <li>7-购买期内，但是人数超标, 且已经超标试用7天</li>
    ///</ul>
    pub app_status: Option<u8>,
    /// 授权模式，0为管理员授权；1为成员授权
    pub auth_mode: Option<u8>,
    /// 是否为代开发自建应用
    pub is_customized_app: Option<u8>,
    /// 是否虚拟版本
    pub is_virtual_version: Option<u8>,
    /// 是否由互联企业分享安装。详见 <a href='https://developer.work.weixin.qq.com/document/path/93360#24909'>企业互联</a>
    pub is_shared_from_other_corp: Option<u8>,
    /// 用户上限。
    /// <p>特别注意, 以下情况该字段无意义，可以忽略：</p>
    /// <ul>
    ///   <li>1. 固定总价购买</li>
    ///   <li>2. app_status = 限时试用/试用过期/不限时试用</li>
    ///   <li>3. 在第2条“app_status=不限时试用”的情况下，如果该应用的配置为“小企业无使用限制”，user_limit有效，且为限制的人数</li>
    /// </ul>
    pub user_limit: Option<i32>,
    /// 版本到期时间, 秒级时间戳, 根据需要自行乘以1000（根据购买版本，可能是试用到期时间或付费使用到期时间）。
    /// <p>特别注意，以下情况该字段无意义，可以忽略：</p>
    /// <ul>
    ///   <li>1. app_status = 不限时试用</li>
    /// </ul>
    pub expired_time: Option<i64>,
    /// 应用权限
    pub privilege: Option<Privilege>,
}

/// 应用对应的权限
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Privilege {
    /// 权限等级。
    /// 1:通讯录基本信息只读
    /// 2:通讯录全部信息只读
    /// 3:通讯录全部信息读写
    /// 4:单个基本信息只读
    /// 5:通讯录全部信息只写
    pub level: Option<u8>,
    pub allow_party: Option<Vec<String>>,
    pub allow_user: Option<Vec<i32>>,
    pub extra_party: Option<Vec<i32>>,
    pub extra_tag: Option<Vec<i32>>,
    pub extra_user: Option<Vec<String>>,
}


/// 预授权码返回
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WechatCpThirdPreauthCode {
    pub pre_auth_code: String,
    pub expires_in: i64,
}



/// 服务商模式获取授权信息
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WechatCpThirdAuthInfo {
    /// 服务商信息
    pub dealer_corp_info: Option<DealerCorpInfo>,
    /// 授权企业信息
    pub auth_corp_info: Option<AuthCorpInfo>,
    /// 授权信息。如果是通讯录应用，且没开启实体应用，是没有该项的。通讯录应用拥有企业通讯录的全部信息读写权限
    pub auth_info: Option<AuthInfo>,
    /// 企业当前生效的版本信息
    pub edition_info: Option<EditionInfo>,
}




/// 服务商模式获取授权信息
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DealerCorpInfo {
    pub corpid: Option<String>,
    pub corp_name: Option<String>,
}
