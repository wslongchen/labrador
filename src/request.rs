use std::net::SocketAddr;
use bytes::Bytes;
use reqwest::{self, multipart, StatusCode, Url};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::errors::LabraError;
use crate::LabradorResult;

use x509_parser::der_parser::asn1_rs::ToDer;

/// Parse Data For Response
pub trait Response <T> where T: Serialize {
    fn parse_result(&self) -> LabradorResult<T>;
}

const APP_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/63.0.3239.132 Safari/537.36";


/// Common Params Format
/// If custom paramters.
pub trait Params{
    fn get_params(&self) -> Vec<(String, String)>;

    fn build_biz_content(&self) -> String where Self: Serialize {
        serde_json::to_string(&self).unwrap_or_default()
    }
}

pub trait RequestMethod {
    fn get_method(&self) -> String ;

    fn get_response_key(&self) -> String {
        String::default()
    }
}

#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum Method {
    Options,
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Connect,
    Patch,
}

impl From<Method> for reqwest::Method {
    fn from(m: Method) -> Self {
        match m {
            Method::Get => reqwest::Method::GET,
            Method::Post => reqwest::Method::POST,
            Method::Options => reqwest::Method::OPTIONS,
            Method::Put => reqwest::Method::PUT,
            Method::Delete => reqwest::Method::DELETE,
            Method::Head => reqwest::Method::HEAD,
            Method::Trace => reqwest::Method::TRACE,
            Method::Connect => reqwest::Method::CONNECT,
            Method::Patch => reqwest::Method::PATCH,
        }
    }
}

impl ToString for Method {
    fn to_string(&self) -> String {
        match self {
            Method::Options => String::from("OPTIONS"),
            Method::Get => String::from("GET"),
            Method::Post => String::from("POST"),
            Method::Put => String::from("PUT"),
            Method::Delete => String::from("DELETE"),
            Method::Head => String::from("HEAD"),
            Method::Trace => String::from("TRACE"),
            Method::Connect => String::from("CONNECT"),
            Method::Patch => String::from("PATCH"),
        }
    }
}

#[derive(Debug)]
pub enum RequestBody<T: Serialize> {
    Json(T),
    Form(T),
    Multipart(multipart::Form),
    Xml(String),
    Text(String),
    Raw(Bytes),
    Null
}

impl <T: Serialize> RequestBody<T> {

    pub fn to_string(&self) -> String {
        match self {
            RequestBody::Json(v) => serde_json::to_string(&v).unwrap_or_default(),
            RequestBody::Form(v) => serde_json::to_string(&v).unwrap_or_default(),
            RequestBody::Multipart(v) => {
                v.boundary().to_string()
            }
            RequestBody::Xml(v) => v.to_string(),
            RequestBody::Text(v) => v.to_string(),
            RequestBody::Raw(_v) => String::from("bytes"),
            RequestBody::Null => String::default(),
        }
    }
}

impl <T: Serialize> From<multipart::Form> for RequestBody<T> {
    fn from(v: multipart::Form) -> Self {
           RequestBody::Multipart(v)
    }
}
impl <T: Serialize> From<String> for RequestBody<T> {
    fn from(v: String) -> Self {
           RequestBody::Text(v)
    }
}

impl <T: Serialize> From<&str> for RequestBody<T> {
    fn from(v: &str) -> Self {
           RequestBody::Text(v.to_string())
    }
}

impl <T: Serialize> From<Bytes> for RequestBody<T> {
    fn from(v: Bytes) -> Self {
           RequestBody::Raw(v)
    }
}

impl <T: Serialize> From<Vec<u8>> for RequestBody<T> {
    fn from(vec: Vec<u8>) -> Self {
        RequestBody::Raw(Bytes::from(vec))
    }
}

impl <T: Serialize> From<&'static [u8]> for RequestBody<T> {
    fn from(s: &'static [u8]) -> Self {
        RequestBody::Raw(Bytes::from_static(s))
    }
}

#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum RequestType {
    Json,
    Form,
    Multipart,
    Xml,
}

/// LabraRequest
///
/// # Examples
///
/// ```no_run
/// use labrador::APIClient;
/// use serde_json::json;
/// use labrador::Method;
/// use labrador::{LabraRequest,LabradorResult};
/// use std::error::Error;
/// fn main() -> LabradorResult<(), Error> {
///     let api = APIClient::new("appkey", "secret", "http_url");
///     let params = vec![(String::from("key"), String::from("value"))];
///     let data = json!({
///         "key": "value"
///     });
///     let req = LabraRequest::new().method(Method::Post).data(data).req_type(request_type).params(params);
///     let _ = req.request().await?;
///     let _ = req.request_blocking().await?;
/// }
/// ```
///
#[allow(unused)]
#[derive(Debug)]
pub struct LabraRequest <T> where T: Serialize {
    pub url: String,
    pub method: Method,
    pub req_type : RequestType,
    pub identity: Option<LabraIdentity>,
    pub cert: Option<LabraCertificate>,
    pub params: Option<Vec<(String, String)>>,
    pub headers: Option<Vec<(String, String)>>,
    pub body: RequestBody<T>
}

#[allow(unused)]
pub struct LabraResponse {
    url: Url,
    status: StatusCode,
    headers: HeaderMap,
    remote_addr: Option<SocketAddr>,
    body: Bytes
}

impl LabraResponse {
    fn new(url: Url, status: StatusCode, remote_addr: Option<SocketAddr>, headers: HeaderMap, body: Bytes) -> LabraResponse {
        LabraResponse {
            url,
            headers,
            remote_addr,
            status,
            body
        }
    }

    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn remote_addr(&self) -> Option<SocketAddr> {
        self.remote_addr
    }

    pub fn header(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn json<T: DeserializeOwned>(&self) -> LabradorResult<T> {
        serde_json::from_slice(&self.body).map_err(LabraError::from)
    }

    pub fn text(&self) -> LabradorResult<String> {
        unsafe {
            // decoding returned Cow::Borrowed, meaning these bytes
            // are already valid utf8
            Ok(String::from_utf8_unchecked(self.body.to_vec()))
        }
    }

    pub fn bytes(&self) -> LabradorResult<Bytes> {
        Ok(self.body.clone())
    }
}

#[allow(unused)]
impl <T> LabraRequest <T> where T: Serialize {
    pub fn new() -> Self {
        LabraRequest { url: String::default(), method: Method::Post, req_type: RequestType::Json, identity: None, cert: None, params: None, headers: None, body: RequestBody::Null }
    }

    pub fn url(mut self, url: String) -> Self {
        self.url = url;
        self
    }

    pub fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    pub fn req_type(mut self, req_type: RequestType) -> Self {
        self.req_type = req_type;
        self
    }

    pub fn headers(mut self, headers: Vec<(String, String)>) -> Self {
        self.headers = headers.into();
        self
    }

    pub fn identity(mut self, identity: LabraIdentity) -> Self {
        self.identity = identity.into();
        self
    }

    pub fn cert(mut self, cert: LabraCertificate) -> Self {
        self.cert = cert.into();
        self
    }

    pub fn params(mut self, params: Vec<(String, String)>) -> Self {
        self.params = params.into();
        self
    }

    pub fn body(mut self, body: RequestBody<T>) -> Self {
        self.body = body.into();
        self
    }

    pub fn json(mut self, data: T) -> Self {
        self.body = RequestBody::Json(data);
        self
    }

    pub fn form(mut self, data: T) -> Self {
        self.body = RequestBody::Form(data);
        self
    }

    pub fn multipart_form(mut self, data: multipart::Form) -> Self {
        self.body = RequestBody::Multipart(data);
        self
    }

    pub fn bytes(mut self, bytes: Bytes) -> Self {
        self.body = bytes.into();
        self
    }

    pub fn text(mut self, data: &str) -> Self {
        self.body = data.into();
        self
    }

    #[inline]
    pub async fn request(self) -> LabradorResult<LabraResponse> {
        let mut http_url = Url::parse(&self.url).unwrap();
        if let Some(params) = &self.params {
            http_url.query_pairs_mut().extend_pairs(params.into_iter());
        }
        let mut client = reqwest::Client::builder().user_agent(APP_USER_AGENT);
        if let Some(identity) = &self.identity {
            client = client.identity(identity.identity());
        }
        if let Some(cert) = &self.cert {
            client = client.add_root_certificate(cert.reqwest_cert()?);
        }
        let client = client.build()?;
        let mut request = client.request(self.method.clone().into(), http_url.to_owned()).header(
            reqwest::header::CONTENT_TYPE,
            self.req_type.get_content_type(),
        );
        let mut data = &self.body.to_string();
        match self.body {
            RequestBody::Json(v) => {
                request = request.json(&v);
            }
            RequestBody::Form(v) => {
                request = request.form(&v);
            }
            RequestBody::Multipart(v) => {
                request = request.multipart(v);
            }
            RequestBody::Xml(v) => {
                request = request.body(v);
            }
            RequestBody::Text(v) => {
                request = request.body(v);
            }
            RequestBody::Raw(v) => {
                request = request.body(v);
            }
            RequestBody::Null => {}
        }
        // if let Some(data) = &self.data {
        //     match self.req_type {
        //         RequestType::Json => {
        //             request = request.json(data);
        //         }
        //         RequestType::Form => {
        //             let value = serde_json::to_value(data.clone()).unwrap_or(Value::Null);
        //             if value.is_string() {
        //                 let v = value.to_string();
        //                 request = request.body(v.replace("\"",""));
        //             } {
        //                 request = request.form(data);
        //             }
        //         }
        //         RequestType::Multipart => {
        //
        //         }
        //         _ => {
        //             request = request.body(serde_json::to_string(data).unwrap_or_default())
        //         }
        //     }
        // }
        if let Some(headers) = &self.headers {
            for (k, v) in headers.into_iter() {
                request = request.header(k, HeaderValue::from_str(v)?);
            }
        }
        tracing::info!("[请求第三方接口参数] url: {}, data:{}", http_url.as_str(), data);
        let result = request.send().await?;
        let status = result.status();
        let remote_addr = result.remote_addr();
        let headers = result.headers();
        let response = LabraResponse::new(result.url().clone(), status, remote_addr, headers.clone(), result.bytes().await?);
        tracing::info!("[请求第三方接口响应] data:{}", &response.text().unwrap_or_default());
        Ok(response)
    }
}


#[derive(Debug, Clone)]
pub struct LabraIdentity {
    identity: reqwest::Identity,
}


#[derive(Debug, Clone)]
pub struct LabraCertificate {
    /// 序列号
    pub serial_no: String,
    /// 颁发时间
    pub effective_time: String,
    /// 过期时间
    pub expire_time: String,
    /// PublicKey
    pub public_key: Vec<u8>,
    /// 证书
    pub content: Vec<u8>,
}

impl LabraIdentity {

    pub fn from_pkcs12_der(der: Vec<u8>, password: &str) -> LabradorResult<Self> {
        let identity = reqwest::Identity::from_pkcs12_der(&der, password)?;
        Ok(Self {
            identity,
        })
    }

    pub fn from_pem(der: Vec<u8>) -> LabradorResult<Self> {
        let identity = reqwest::Identity::from_pem(&der)?;
        Ok(Self {
            identity,
        })
    }

    pub fn identity(&self) -> reqwest::Identity {
        self.identity.clone()
    }

}

impl LabraCertificate {

    #[cfg(not(feature = "openssl-crypto"))]
    pub fn from_pem(pem: Vec<u8>) -> LabradorResult<Self> {
        let (data, x509) = x509_parser::pem::parse_x509_pem(&pem)?;
        let x509 = x509.parse_x509()?;
        let sn = x509.serial.to_string();
        Ok(Self {
            serial_no: sn.to_string(),
            effective_time: "".to_string(),
            expire_time: "".to_string(),
            public_key: data.to_der_vec()?,
            content: pem,
        })
    }

    #[cfg(not(feature = "openssl-crypto"))]
    pub fn from(pem: &str) -> LabradorResult<Self> {
        let content = pem.as_bytes();
        let (data, _x509) = x509_parser::pem::parse_x509_pem(content)?;
        Ok(Self {
            serial_no: "".to_string(),
            effective_time: "".to_string(),
            expire_time: "".to_string(),
            public_key: data.to_der_vec()?,
            content: content.to_vec(),
        })
    }

    #[cfg(feature = "openssl-crypto")]
    pub fn from_pem(pem: Vec<u8>) -> LabradorResult<Self> {
        let x509 = openssl::x509::X509::from_pem(&pem).unwrap();
        let pk = x509.public_key().unwrap();
        let rpk = pk.public_key_to_pem().unwrap();
        let sn = x509.serial_number().to_bn().unwrap().to_string();
        Ok(Self {
            serial_no: sn.to_string(),
            effective_time: "".to_string(),
            expire_time: "".to_string(),
            public_key: rpk,
            content: pem,
        })
    }

    #[cfg(feature = "openssl-crypto")]
    pub fn from(pem: &str) -> LabradorResult<Self> {
        let content = pem.as_bytes();
        let x509 = openssl::x509::X509::from_pem(content).unwrap();
        let pk = x509.public_key().unwrap();
        let rpk = pk.public_key_to_pem().unwrap();
        Ok(Self {
            serial_no: "".to_string(),
            effective_time: "".to_string(),
            expire_time: "".to_string(),
            public_key: rpk,
            content: content.to_vec(),
        })
    }


    pub fn reqwest_cert(&self) -> LabradorResult<reqwest::Certificate> {
        let cert = reqwest::Certificate::from_pem(self.content.as_ref())?;
        Ok(cert)
    }

}


#[allow(unused)]
impl RequestType {
    pub fn get_content_type(&self) -> String {
        match *self {
            RequestType::Json => String::from("application/json;charset=UTF-8"),
            RequestType::Form => String::from("application/x-www-form-urlencoded;charset=UTF-8"),
            RequestType::Multipart => String::from("multipart/form-data;charset=UTF-8"),
            RequestType::Xml => String::from("application/xml;charset=UTF-8"),
        }
    }
}

pub async fn request<F>(f: F) -> LabradorResult<LabraResponse>
where
    F: Fn(reqwest::Client) -> reqwest::RequestBuilder,
{
    let result = f(reqwest::Client::new()).send().await?;
    Ok(LabraResponse::new(result.url().clone(), result.status(), result.remote_addr(), result.headers().clone(), result.bytes().await?))
}

#[allow(unused)]
pub fn request_blocking<F>(f: F) -> LabradorResult<LabraResponse>
where
    F: Fn(reqwest::blocking::Client) -> reqwest::blocking::RequestBuilder,
{
    let result = f(reqwest::blocking::Client::new()).send()?;
    Ok(LabraResponse::new(result.url().clone(), result.status(), result.remote_addr(), result.headers().clone(), result.bytes()?))
}