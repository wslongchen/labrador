use crate::{session::SessionStore, client::APIClient, request::{Method, RequestType, LabraResponse, LabraRequest, RequestMethod}, util::current_timestamp, LabradorResult, SimpleStorage, WechatCrypto, WechatRequest, get_timestamp, get_nonce_str, WechatCommonResponse};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

mod method;
mod api;
#[allow(unused)]
mod constants;
mod tp;
mod messages;
mod msg_parser;
mod events;
mod replies;
mod msg_router;

pub use api::*;
pub use tp::*;
pub use messages::*;
pub use replies::*;
pub use events::*;
pub use msg_parser::*;
pub use msg_router::*;
use crate::wechat::cp::constants::{ACCESS_TOKEN, CORPID, CORPSECRET};
use crate::wechat::cp::method::{WechatCpMethod};

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct WechatCpClient<T: SessionStore> {
    corp_id: String,
    corp_secret: String,
    token: Option<String>,
    aes_key: Option<String>,
    oauth2_redirect_uri: Option<String>,
    webhook_url: Option<String>,
    agent_id: Option<i32>,
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
pub struct JsapiTicket {
    pub ticket: String,
    pub expires_in: i64,
}

#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct JsapiSignature {
    pub app_id: String,
    #[serde(rename="nonceStr")]
    pub nonce_str: String,
    pub url: String,
    pub signature: String,
    pub timestamp: i64,
}
#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct AgentJsapiSignature {
    pub agentid: String,
    pub corpid: String,
    #[serde(rename="nonceStr")]
    pub nonce_str: String,
    pub url: String,
    pub signature: String,
    pub timestamp: i64,
}


#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct WechatCpProviderToken {
    /// 服务商的access_token，最长为512字节。
    pub provider_access_token: String,
    /// provider_access_token有效期（秒）
    pub expires_in: i64,
}

#[allow(unused)]
impl<T: SessionStore> WechatCpClient<T> {

    fn from_client(client: APIClient<T>) -> WechatCpClient<T> {
        WechatCpClient {
            corp_id: client.app_key.to_owned(),
            corp_secret: client.secret.to_owned(),
            token: None,
            aes_key: None,
            oauth2_redirect_uri: None,
            webhook_url: None,
            agent_id: None,
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

    pub fn oauth2_redirect_uri(mut self, oauth2_redirect_uri: &str) -> Self {
        self.oauth2_redirect_uri = oauth2_redirect_uri.to_string().into();
        self
    }

    pub fn webhook_url(mut self, webhook_url: &str) -> Self {
        self.webhook_url = webhook_url.to_string().into();
        self
    }

    /// get the wechat client
    pub fn new<S: Into<String>>(corp_id: S, corp_secret: S) -> WechatCpClient<SimpleStorage> {
        let session = SimpleStorage::new();
        let client = APIClient::from_session(corp_id.into(), corp_secret.into(), "https://qyapi.weixin.qq.com", session);
        WechatCpClient::from_client(client)
    }

    /// get the wechat client
    pub fn from_session<S: Into<String>>(corp_id: S, corp_secret: S, session: T) -> WechatCpClient<T> {
        let client = APIClient::from_session(corp_id.into(), corp_secret.into(), "https://qyapi.weixin.qq.com", session);
        Self::from_client(client)
    }

    #[inline]
    pub async fn access_token(&self, force_refresh: bool) -> LabradorResult<String> {
        let mut session = self.client.session();
        let token_key = format!("{}_access_token_cp", self.corp_id);
        let expires_key = format!("{}_expires_at_cp", self.corp_id);
        let token: String = session.get(&token_key, Some("".to_owned()))?.unwrap_or_default();
        let timestamp = current_timestamp();
        let expires_at: i64 = session.get(&expires_key, Some(timestamp))?.unwrap_or_default();
        if expires_at <= timestamp || force_refresh {
            let mut req = LabraRequest::<String>::new().url(WechatCpMethod::AccessToken.get_method()).params(vec![
                (CORPID.to_string(), self.corp_id.to_string()),
                (CORPSECRET.to_string(), self.corp_secret.to_string()),
            ]).method(Method::Get).req_type(RequestType::Json);
            let res = self.client.request(req).await?.json::<AccessTokenResponse>()?;
            let token = res.access_token;
            let expires_in = res.expires_in;
            // 预留200秒的时间
            let expires_at = current_timestamp() + expires_in - 200;
            session.set(&token_key, token.to_owned(), Some(expires_in as usize));
            session.set(&expires_key, expires_at, Some(expires_in as usize));
            Ok(token.to_string())
        } else {
            Ok(token)
        }
    }
    
    /// <pre>
    /// 获取服务商凭证
    /// 文档地址：<a href="https://work.weixin.qq.com/api/doc#90001/90143/91200">地址</a>
    /// 请求方式：POST（HTTPS）
    /// 请求地址： <a href="https://qyapi.weixin.qq.com/cgi-bin/service/get_provider_token">地址</a>
    /// </pre>
    #[inline]
    pub async fn get_provider_token(&self, corp_id: &str, provider_secret: &str) -> LabradorResult<WechatCpProviderToken> {
        let mut req = LabraRequest::new().url(WechatCpMethod::GetProviderToken.get_method()).json(json!({
            "corpid": corp_id,
            "provider_secret": provider_secret,
        })).method(Method::Post).req_type(RequestType::Json);
        let res = self.client.request(req).await?.json::<WechatCpProviderToken>()?;
        Ok(res)
    }

    ///
    /// <pre>
    /// 验证消息的确来自微信服务器.
    /// [详情](http://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1421135319&token=&lang=zh_CN)
    /// </pre>
    pub fn check_signature(&self, signature: &str, timestamp: i64, nonce: &str, data: &str) -> LabradorResult<bool> {
        let crp = WechatCrypto::new(&self.aes_key.to_owned().unwrap_or_default()).token(&self.token.to_owned().unwrap_or_default());
        let _ = crp.check_signature(signature, timestamp, nonce, data)?;
        Ok(true)
    }

    ///
    /// <pre>
    /// 创建调用jsapi时所需要的签名
    ///
    /// 详情[请见](http://qydev.weixin.qq.com/wiki/index.php?title=微信JS接口)
    /// </pre>
    pub async fn create_jsapi_signature(&self, url: &str) -> LabradorResult<JsapiSignature> {
        let timestamp = get_timestamp() / 1000;
        let noncestr = get_nonce_str();
        let jsapi_ticket = self.get_jsapi_ticket(false).await?;
        let signature = WechatCrypto::get_sha1_sign(&vec!["jsapi_ticket=".to_string() + &jsapi_ticket,
                                                         "noncestr=".to_string() + &noncestr,
                                                         "timestamp=".to_string() + &timestamp.to_string(),"url=".to_string() + &url].join("&"));
        Ok(JsapiSignature{
            app_id: self.corp_id.to_string(),
            nonce_str: noncestr,
            url: url.to_string(),
            signature,
            timestamp,
        })
    }

    ///
    /// <pre>
    /// 创建调用wx.agentConfig时所需要的签名
    ///
    /// 详情[请见](https://open.work.weixin.qq.com/api/doc/90000/90136/94313)
    /// </pre>
    pub async fn create_agent_jsapi_signature(&self, url: &str) -> LabradorResult<AgentJsapiSignature> {
        let timestamp = get_timestamp() / 1000;
        let noncestr = get_nonce_str();
        let jsapi_ticket = self.get_jsapi_ticket(false).await?;
        let signature = WechatCrypto::get_sha1_sign(&vec!["jsapi_ticket=".to_string() + &jsapi_ticket,
                                                         "noncestr=".to_string() + &noncestr,
                                                         "timestamp=".to_string() + &timestamp.to_string(),"url=".to_string() + &url].join("&"));
        Ok(AgentJsapiSignature{
            agentid: self.agent_id.unwrap_or_default().to_string(),
            corpid: self.corp_id.to_string(),
            nonce_str: noncestr,
            url: url.to_string(),
            signature,
            timestamp,
        })
    }

    ///
    /// <pre>
    /// 获得jsapi_ticket,不强制刷新jsapi_ticket
    /// </pre>
    pub async fn get_jsapi_ticket(&self, force_refresh: bool) -> LabradorResult<String> {
        let mut session = self.client.session();
        let token_key = format!("{}_jsapi_ticket_cp", self.corp_id);
        let expires_key = format!("{}_jsapi_ticket_expires_at_cp", self.corp_id);
        let ticket: String = session.get(&token_key, Some("".to_owned()))?.unwrap_or_default();
        let timestamp = current_timestamp();
        let expires_at: i64 = session.get(&expires_key, Some(timestamp))?.unwrap_or_default();
        if expires_at <= timestamp || force_refresh {
            let mut req = LabraRequest::<String>::new().url(WechatCpMethod::GetJsapiTicket.get_method()).params(vec![]).method(Method::Get).req_type(RequestType::Json);
            let res = self.client.request(req).await?.json::<JsapiTicket>()?;
            let ticket = res.ticket;
            let expires_in = res.expires_in;
            // 预留200秒的时间
            let expires_at = current_timestamp() + expires_in - 200;
            let ticket_key = format!("{}_jsapi_ticket_cp", self.corp_id);
            let expires_key = format!("{}_jsapi_ticket_expires_at_cp", self.corp_id);
            session.set(&ticket_key, ticket.to_owned(), Some(expires_in as usize));
            session.set(&expires_key, expires_at, Some(expires_in as usize));
            Ok(ticket.to_string())
        } else {
            Ok(ticket)
        }
    }

    ///
    /// <pre>
    /// 获得jsapi_ticket,不强制刷新jsapi_ticket
    /// 应用的jsapi_ticket用于计算agentConfig（参见“通过agentConfig注入应用的权限”）的签名，签名计算方法与上述介绍的config的签名算法完全相同，但需要注意以下区别：
    /// <p>
    /// 签名的jsapi_ticket必须使用以下接口获取。且必须用wx.agentConfig中的agentid对应的应用secret去获取access_token。
    /// 签名用的noncestr和timestamp必须与wx.agentConfig中的nonceStr和timestamp相同。
    /// </pre>
    pub async fn get_agent_jsapi_ticket(&self, force_refresh: bool) -> LabradorResult<String> {
        let mut session = self.client.session();
        let token_key = format!("{}_agent_jsapi_ticket_cp", self.corp_id);
        let expires_key = format!("{}_agent_jsapi_ticket_expires_at_cp", self.corp_id);
        let ticket: String = session.get(&token_key, Some("".to_owned()))?.unwrap_or_default();
        let timestamp = current_timestamp();
        let expires_at: i64 = session.get(&expires_key, Some(timestamp))?.unwrap_or_default();
        if expires_at <= timestamp || force_refresh {
            let mut req = LabraRequest::<String>::new().url(WechatCpMethod::GetAgentConfigTicket.get_method()).params(vec![]).method(Method::Get).req_type(RequestType::Json);
            let res = self.client.request(req).await?.json::<JsapiTicket>()?;
            let ticket = res.ticket;
            let expires_in = res.expires_in;
            // 预留200秒的时间
            let expires_at = current_timestamp() + expires_in - 200;
            let ticket_key = format!("{}_agent_jsapi_ticket_cp", self.corp_id);
            let expires_key = format!("{}_agent_jsapi_ticket_expires_at_cp", self.corp_id);
            session.set(&ticket_key, ticket.to_owned(), Some(expires_in as usize));
            session.set(&expires_key, expires_at, Some(expires_in as usize));
            Ok(ticket.to_string())
        } else {
            Ok(ticket)
        }
    }

    ///
    /// <pre>
    /// 获取微信服务器的ip段
    /// [文档](http://qydev.weixin.qq.com/wiki/index.php?title=回调模式#.E8.8E.B7.E5.8F.96.E5.BE.AE.E4.BF.A1.E6.9C.8D.E5.8A.A1.E5.99.A8.E7.9A.84ip.E6.AE.B5)
    /// </pre>
    pub async fn get_callback_ip(&self, force_refresh: bool) -> LabradorResult<Vec<String>> {
        let v = self.get(WechatCpMethod::GetCallbackIp, vec![], RequestType::Json).await?.json::<Value>()?;
        let v = WechatCommonResponse::parse::<Value>(v)?;
        let ip_list = v["ip_list"].as_array().unwrap_or(&vec![]).iter().map(|v| v.as_str().unwrap_or_default().to_string()).collect::<Vec<String>>();
        Ok(ip_list)
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

    /// 发送POST请求
    async fn post<D: Serialize>(&self, method: WechatCpMethod, mut querys: Vec<(String, String)>, data: D, request_type: RequestType) -> LabradorResult<LabraResponse> {
        let access_token = self.access_token(false).await?;
        if !access_token.is_empty() && method.need_token() {
            querys.push((ACCESS_TOKEN.to_string(), access_token));
        }
        self.client.post(method, querys, data, request_type).await
    }

    /// 发送GET请求
    async fn get(&self, method: WechatCpMethod, mut params: Vec<(String, String)>, request_type: RequestType) -> LabradorResult<LabraResponse> {
        let access_token = self.access_token(false).await?;
        if !access_token.is_empty() && method.need_token() {
            params.push((ACCESS_TOKEN.to_string(), access_token));
        }
        self.client.get(method, params, request_type).await
    }

    /// codesssion相关服务
    pub fn code_session(&self) -> WechatCpCodeSession<T> {
        WechatCpCodeSession::new(self)
    }

    /// 媒体操作接口
    pub fn media(&self) -> WechatCpMedia<T> {
        WechatCpMedia::new(self)
    }

    /// 自建应用
    pub fn agent(&self) -> WechatCpAgent<T> {
        WechatCpAgent::new(self)
    }

    /// 部门
    pub fn department(&self) -> WechatCpDepartment<T> {
        WechatCpDepartment::new(self)
    }

    /// 外部联系人
    pub fn external_contact(&self) -> WechatCpExternalContact<T> {
        WechatCpExternalContact::new(self)
    }

    /// 群机器人
    pub fn group_robot(&self) -> WechatCpGroupRobot<T> {
        WechatCpGroupRobot::new(self)
    }

    /// 菜单
    pub fn menu(&self) -> WechatCpMenu<T> {
        WechatCpMenu::new(self)
    }

    /// 消息
    pub fn message(&self) -> WechatCpMessage<T> {
        WechatCpMessage::new(self)
    }

    /// 认证
    pub fn oauth2(&self) -> WechatCpOauth2<T> {
        WechatCpOauth2::new(self)
    }

    /// 标签
    pub fn tag(&self) -> WechatCpTag<T> {
        WechatCpTag::new(self)
    }

    /// 用户
    pub fn user(&self) -> WechatCpUser<T> {
        WechatCpUser::new(self)
    }

}
