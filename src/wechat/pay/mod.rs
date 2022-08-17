use std::fs;
use std::sync::Arc;
use dashmap::DashMap;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::{APIClient, LabraCertificate, LabraError, LabraIdentity, LabraRequest, LabraResponse, Method, RequestType, SessionStore, RequestMethod, LabradorResult};
use crate::util::{current_timestamp, get_nonce_str, get_timestamp};

mod method;
mod api;
mod request;
mod response;

pub use request::*;
pub use response::*;
use tracing::info;
use crate::wechat::cryptos::{SignatureHeader, WeChatCryptoV3};
use crate::wechat::pay::api::WxPay;
use crate::wechat::pay::method::WechatPayMethod;

const SCHEMA: &str = "WECHATPAY2-SHA256-RSA2048";

/// 交易类型
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum TradeType {
    /// `MICRO`
    Micro,
    /// `MWEB`
    MWeb,
    /// `JSAPI`
    Jsapi,
    /// `NATIVE`
    Native,
    /// `APP` : app支付，统一下单接口trade_type的传参可参考这里
    App,
}

impl TradeType {
    fn get_trade_type(&self) -> &str {
        match *self {
            TradeType::Micro => "MICROPAY",
            TradeType::Jsapi => "JSAPI",
            TradeType::Native => "NATIVE",
            TradeType::App => "APP",
            TradeType::MWeb => "MWEB"
        }
    }


    pub fn from(str: &str) -> Self {
        let data = &str.to_uppercase();
        match data.as_str() {
            "MICROPAY" => TradeType::Micro,
            "JSAPI" => TradeType::Jsapi,
            "NATIVE" => TradeType::Native,
            "APP" => TradeType::App,
            "MWEB" => TradeType::MWeb,
            _ => unreachable!()
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct WeChatPayClient<T: SessionStore> {
    pub appid: String,
    secret: String,
    /// 私钥 V3
    api_key_v3: Option<String>,
    /// 私钥
    api_key: Option<String>,
    /// 商户编号
    pub mch_id: Option<String>,
    /// API证书序列号
    serial_no: Option<String>,
    /// API商户证书秘钥
    private_key: Option<String>,
    /// 证书文件
    pkcs12_path: Option<String>,
    client: APIClient<T>,
    /// 缓存的证书文件
    certs: Arc<DashMap<String, LabraCertificate>>,
}


#[allow(unused)]
impl<T: SessionStore> WeChatPayClient<T> {

    fn from_client(client: APIClient<T>) -> WeChatPayClient<T> {
        WeChatPayClient {
            appid: client.app_key.to_owned(),
            secret: client.secret.to_owned(),
            api_key_v3: None,
            api_key: None,
            mch_id: None,
            serial_no: None,
            private_key: None,
            client,
            pkcs12_path: None,
            certs: Arc::new(DashMap::new())
        }
    }

    /// get the wechat client
    pub fn new<S: Into<String>>(appid: S, secret: S, session: T) -> WeChatPayClient<T> {
        let client = APIClient::from_session(appid.into(), secret.into(), "https://api.mch.weixin.qq.com", session);
        Self::from_client(client)
    }

    pub fn key_v3(mut self, key: String) -> Self {
        self.api_key_v3 = key.into();
        self
    }

    pub fn key(mut self, key: String) -> Self {
        self.api_key = key.into();
        self
    }

    pub fn mch_id(mut self, mch_id: String) -> Self {
        self.mch_id = mch_id.into();
        self
    }

    pub fn private_key(mut self, private_key: String) -> Self {
        self.private_key = private_key.into();
        self
    }

    // pub async fn private_key_path(mut self, private_key_path: &str) -> LabradorResult<Self> {
    //     // 根据url路径获取对应的文件信息
    //     match request_async(|client| client.get(private_key_path)).await {
    //         Ok(res) => {
    //             let content = res.text().await?;
    //             self.private_key = content.into();
    //             Ok(self)
    //         }
    //         Err(e) => {
    //             Err(LabraError::RequestError("无法获取私钥文件，请检查后再试".to_string()))
    //         }
    //     }
    //
    // }

    pub fn set_private_key_path(mut self, private_key_path: &str) -> LabradorResult<Self> {
        if private_key_path.is_empty() {
            return Err(LabraError::InvalidSignature("证书文件有误！".to_string()));
        }
        let content = fs::read_to_string(private_key_path)?;
        self.private_key = content.into();
        Ok(self)
    }

    pub fn serial_no(mut self, serial_no: String) -> Self {
        self.serial_no = serial_no.into();
        self
    }

    pub fn pkcs12_path(mut self, pkcs12_path: String) -> Self {
        self.pkcs12_path = pkcs12_path.into();
        self
    }

    fn get_identity(&self, password: Option<String>) -> LabradorResult<LabraIdentity> {
        let password = if let Some(password) = password {
            password
        } else {
            self.mch_id.to_owned().unwrap_or_default()
        };
        let path = self.pkcs12_path.to_owned().unwrap_or_default();
        if path.is_empty() {
            return Err(LabraError::InvalidSignature("pkcs12证书文件路径有误！".to_string()));
        }
        let content = fs::read_to_string(path)?;
        let buf = content.as_bytes();
        LabraIdentity::from_pkcs12_der(buf.to_vec(), &password)
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


    #[inline]
    pub fn token<F: Serialize>(&self, req: &LabraRequest<F>, mch_id: Option<String>) -> LabradorResult<String> {
        let api_path = self.client.api_path.to_owned();
        let LabraRequest { url, method, data, ..} = req;
        let method = method.to_string();
        let body = if data.is_none() { String::from("") } else { serde_json::to_string(data).unwrap_or_default() };
        let mut mch_id = mch_id.unwrap_or_default();
        let mut private_key = self.private_key.to_owned().unwrap_or_default();
        let mut serial_no = self.serial_no.to_owned().unwrap_or_default();
        if let Some(mchid) = &self.mch_id {
            if mch_id.is_empty() {
                mch_id = mchid.to_owned();
            }
        }
        if mch_id.is_empty() || serial_no.is_empty()  || private_key.is_empty() {
            return Err(LabraError::InvalidSignature("商户参数有误，无法进行操作".to_string()))
        }
        let nonce_str = get_nonce_str().to_uppercase();

        let timestamp = get_timestamp() / 1000;
        let signature = WeChatCryptoV3::signature_v3(&method, url, timestamp, &nonce_str, &body, &private_key)?;
        let token = format!("{} mchid=\"{}\",nonce_str=\"{}\",signature=\"{}\",timestamp=\"{}\",serial_no=\"{}\"",
                            SCHEMA, mch_id, nonce_str, signature, timestamp, serial_no);
        Ok(token)
    }

    /// 发送POST请求
    async fn post<D: Serialize>(&self, method: WechatPayMethod, data: D, request_type: RequestType) -> LabradorResult<LabraResponse> {
        let mut querys = Vec::new();
        let access_token = self.access_token();
        if !access_token.is_empty() {
            querys.push(("access_token".to_string(), access_token));
        }
        let mut req = LabraRequest::new().url(method.get_method()).params(querys).method(Method::Post).data(data).req_type(request_type);
        if let Some(_) = &self.pkcs12_path {
            req = req.identity(self.get_identity(None)?);
        }
        self.client.request(req).await
    }

    /// 发送POST请求
    /// <pre>
    /// mchid 商户编号 - 如果传入则会替换token中的商户
    /// method 请求方法
    /// data 请求数据
    /// request_type 请求方式
    /// </pre>
    async fn post_v3<D: Serialize>(&self, mchid: Option<String>, method: WechatPayMethod, mut querys: Vec<(String, String)>, data: D, request_type: RequestType) -> LabradorResult<LabraResponse> {
        let mut req = LabraRequest::new().url(method.get_method()).params(querys).method(Method::Post).data(data).req_type(request_type);
        let auth = self.token(&req, mchid)?;
        self.auto_load_cert().await?;
        let headers = vec![(String::from("Authorization"), auth),(String::from("Accept"), String::from("application/json"))];
        req = req.headers(headers);
        if let Some(cert) = self.certs.iter().take(1).next() {
            req = req.cert(cert.clone());
        }
        let result = self.client.request(req).await?;
        // v3已经改为通过状态码判断200 204 成功
        let status = result.status();
        // 返回结果验签
        // let header = SignatureHeader::from_header(result.header());

        if status.as_u16() == 200 || status.as_u16() == 204 {
            Ok(result)
        } else {
            Err(LabraError::RequestError(result.text().await?))
        }
    }

    /// 校验通知签名
    /// header 通知头信息
    /// data   通知数据
    /// true:校验通过 false:校验不通过
    async fn verify_notify_sign(&self, header: &SignatureHeader, data: &str) -> bool {
        let serial_no = header.serial.to_owned();
        let before_sign = format!("{}\n{}\n{}\n", header.time_stamp, header.nonce, data);
        let result = self.certs.contains_key(&serial_no);
        // V3  验证签名
        let verify = if let Some(cert) = self.certs.get(&serial_no) {
            let content = String::from_utf8_lossy(&cert.public_key).to_string();
            WeChatCryptoV3::verify(&before_sign, &header.signature, &content).unwrap_or(false)
        } else {
            false
        };
        result && verify
    }

    /// V3  验证签名
    pub async fn verify(&self, serial_number: &str, message: &str, signature: &str) -> bool {
        if let Some(cert) = self.certs.get(serial_number) {
            let content = String::from_utf8_lossy(&cert.content).to_string();
            WeChatCryptoV3::verify(message, signature, &content).unwrap_or(false)
        } else {
            false
        }
    }

    /// 自动加载证书
    pub async fn auto_load_cert(&self) -> LabradorResult<()> {
        // 如果已经有证书了，则不用自动获取
        if self.certs.is_empty() {
            let response = self.get_v3(WechatPayMethod::Certificate, vec![], RequestType::Json).await?;
            let status_code = response.status().as_u16();
            if status_code == 200 {
                let body = response.json::<Value>().await?;
                info!("获取平台证书:{}", serde_json::to_string(&body).unwrap_or_default());
                let bodys = serde_json::from_value::<Vec<PlatformCertificateResponse>>(body["data"].to_owned())?;
                for body in bodys {
                    let data =body.encrypt_certificate;
                    let crypto = WeChatCryptoV3::new(&self.api_key_v3.to_owned().unwrap_or_default());
                    let res = crypto.decrypt_data_v3(&data)?;
                    let mut cert = LabraCertificate::from_pem(res)?;
                    let serial_no = body.serial_no;
                    cert.serial_no = serial_no.to_owned();
                    cert.effective_time = body.effective_time.to_owned();
                    cert.expire_time = body.expire_time.to_owned();
                    self.certs.insert(serial_no, cert);
                }
            }
        }
        Ok(())
    }



    /// 发送GET请求
    async fn get(&self, method: WechatPayMethod, params: Vec<(&str, &str)>, request_type: RequestType) -> LabradorResult<LabraResponse> {
        let access_token = self.access_token();
        let mut querys = params.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect::<Vec<(String,String)>>();
        if !access_token.is_empty() {
            querys.push(("access_token".to_string(), access_token));
        }
        let mut req = LabraRequest::<String>::new().url(method.get_method()).params(querys).method(Method::Get).req_type(request_type);
        if let Some(_) = &self.pkcs12_path {
            req = req.identity(self.get_identity(None)?);
        }
        self.client.request(req).await
    }

    /// 发送GET请求
    async fn get_v3(&self, method: WechatPayMethod, params: Vec<(&str, &str)>, request_type: RequestType) -> LabradorResult<LabraResponse> {
        let querys = params.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect::<Vec<(String,String)>>();
        let mut req = LabraRequest::<String>::new().url(method.get_method()).params(querys).method(Method::Get).req_type(request_type);
        let auth = self.token(&req, None)?;
        let headers = vec![(String::from("Authorization"), auth),(String::from("Accept"), String::from("application/json"))];
        req = req.headers(headers);
        self.client.request(req).await
    }

    /// # 获取平台证书 - V3版本
    pub async fn get_certificates(&self) -> LabradorResult<Vec<PlatformCertificateResponse>> {
        let response = self.get_v3(WechatPayMethod::Certificate, vec![], RequestType::Json).await?;
        let status_code = response.status().as_u16();
        if status_code == 200 {
            let body = response.json::<Value>().await?;
            serde_json::from_value::<Vec<PlatformCertificateResponse>>(body["data"].to_owned()).map_err(LabraError::from)
        } else {
            Ok(vec![])
        }
    }

    /// 微信支付服务
    pub fn wxpay(&self) -> WxPay<T> {
        WxPay::new(self)
    }


}
