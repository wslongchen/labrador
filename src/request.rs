use std::net::SocketAddr;
use bytes::Bytes;
use openssl::x509::X509;
use reqwest::{self, multipart, StatusCode, Url};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{Value};
use crate::errors::LabraError;
use crate::LabradorResult;

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


#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum RequestType {
    Json,
    Form,
    Multipart,
    Xml,
}

#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum ResponseType {
    Json,
    Bytes,
    Text,
}

#[allow(unused)]
#[derive(Debug)]
pub struct LabraResponse {
   response: Option<reqwest::Response>,
   block_response: Option<reqwest::blocking::Response>,
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
#[derive(Debug, Clone)]
pub struct LabraRequest <T> where T: Serialize {
    pub url: String,
    pub method: Method,
    pub req_type : RequestType,
    pub identity: Option<LabraIdentity>,
    pub cert: Option<LabraCertificate>,
    pub params: Option<Vec<(String, String)>>,
    pub headers: Option<Vec<(String, String)>>,
    pub data: Option<T>,

}

#[allow(unused)]
impl <T> LabraRequest <T> where T: Serialize {
    pub fn new() -> Self {
        LabraRequest { url: String::default(), method: Method::Post, req_type: RequestType::Json, identity: None, cert: None, params: None, data: None, headers: None }
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

    pub fn data(mut self, data: T) -> Self {
        self.data = data.into();
        self
    }

    #[inline]
    pub async fn request(&self) -> LabradorResult<LabraResponse> {
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
        if let Some(data) = &self.data {
            match self.req_type {
                RequestType::Json => {
                    request = request.json(data);
                }
                RequestType::Form => {
                    let value = serde_json::to_value(data.clone()).unwrap_or(Value::Null);
                    if value.is_string() {
                        let v = value.to_string();
                        request = request.body(v.replace("\"",""));
                    } {
                        request = request.form(data);
                    }
                }
                RequestType::Multipart => {
                    let value = serde_json::to_value(data.clone()).unwrap_or(Value::Null);
                    if value.is_object() {
                        let mut form = multipart::Form::new();
                        if let Some(v) = value.as_object() {
                            for (k, v) in v.into_iter() {
                                let v = v.as_str().unwrap_or_default();
                                form = form.text(k.to_owned(), v.to_owned());
                            }
                        }
                        request = request.multipart(form);
                    } else {
                        let v = value.to_string();
                        request = request.body(v.replace("\"",""));
                    }
                }
                _ => {
                    request = request.body(serde_json::to_string(data).unwrap_or_default())
                }
            }
        }
        if let Some(headers) = &self.headers {
            for (k, v) in headers.into_iter() {
                request = request.header(k, HeaderValue::from_str(v)?);
            }
        }
        tracing::info!("[请求第三方接口参数] url: {}, data:{}", http_url.as_str(), serde_json::to_string(&self.data).unwrap_or_default());
        ResponseType::from_response(request.send().await?)
    }

    #[inline]
    pub fn request_blocking(&self) -> LabradorResult<LabraResponse> {
        let mut http_url = Url::parse(&self.url).unwrap();
        if let Some(params) = &self.params {
            http_url.query_pairs_mut().extend_pairs(params.into_iter());
        }
        let mut client = reqwest::blocking::Client::builder().user_agent(APP_USER_AGENT);
        if let Some(identity) = &self.identity {
            client = client.identity(identity.identity());
        }
        if let Some(cert) = &self.cert {
            client = client.add_root_certificate(cert.reqwest_cert()?);
        }
        let mut request = client.build()?.request(self.method.clone().into(), http_url.to_owned()).header(
            reqwest::header::CONTENT_TYPE,
            self.req_type.get_content_type(),
        );
        if let Some(data) = &self.data {
            request = request.body(serde_json::to_string(data).unwrap_or_default());
        }
        if let Some(headers) = &self.headers {
            for (k, v) in headers.into_iter() {
                request = request.header(k, v);
            }
        }
        tracing::info!("[请求第三方接口参数] url: {}, data:{}", http_url.as_str(), serde_json::to_string(&self.data).unwrap_or_default());
        ResponseType::from_response_block(request.send()?)
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


    pub fn from_pem(pem: Vec<u8>) -> LabradorResult<Self> {
        let x509 = X509::from_pem(&pem).unwrap();
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

    pub fn from(pem: &str) -> LabradorResult<Self> {
        let content = pem.as_bytes();
        let x509 = X509::from_pem(content).unwrap();
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

#[allow(unused)]
impl ResponseType {
    pub fn from_response(res: reqwest::Response) -> LabradorResult<LabraResponse> {
        Ok(LabraResponse {
            response: res.into(),
            block_response: None,
        })
    }

    pub fn from_response_block(res: reqwest::blocking::Response) -> LabradorResult<LabraResponse> {
        Ok(LabraResponse {
            response: None,
            block_response: res.into(),
        })
    }
}

#[allow(unused)]
impl LabraResponse {

    pub fn status(&self) -> StatusCode {
        if let Some(res) = &self.response {
            res.status()
        } else {
            if let Some(res) = &self.block_response {
                res.status()
            } else {
                StatusCode::default()
            }
        }
    }

    pub fn remote_addr(&self) -> Option<SocketAddr> {
        if let Some(res) = &self.response {
            res.remote_addr()
        } else {
            if let Some(res) = &self.block_response {
                res.remote_addr()
            } else {
                return None;
            }
        }
    }

    pub fn header(&self) -> HeaderMap {
        if let Some(res) = &self.response {
            res.headers().to_owned()
        } else {
            if let Some(res) = &self.block_response {
                res.headers().to_owned()
            } else {
                return HeaderMap::new();
            }
        }
    }

    pub async fn json<T: DeserializeOwned + Serialize>(self) -> LabradorResult<T> {
        match self.response {
            Some(v) => v.json::<T>().await.map(|v| {
                tracing::info!("[请求第三方接口响应] data:{}", serde_json::to_string(&v).unwrap_or_default());
                v
            }).map_err(LabraError::from),
            _ => Err(LabraError::RequestError(format!("error to parse http response: {:?}", self))),
        }
    }

    pub async fn text(self) -> LabradorResult<String> {
        match self.response {
            Some(v) => v.text().await.map(|v| {
                tracing::info!("[请求第三方接口响应] data:{}", serde_json::to_string(&v).unwrap_or_default());
                v
            }).map_err(LabraError::from),
            _ => Err(LabraError::RequestError(format!("error to parse http response: {:?}", self))),
        }
    }

    pub async fn bytes(self) -> LabradorResult<Bytes> {
        match self.response {
            Some(v) => v.bytes().await.map_err(LabraError::from),
            _ => Err(LabraError::RequestError(format!("error to parse http response: {:?}", self))),
        }
    }

    pub fn json_blocking<T: DeserializeOwned + Serialize>(self) -> LabradorResult<T> {
        match self.block_response {
            Some(v) => v.json::<T>().map(|v| {
                tracing::info!("[请求第三方接口响应] data:{}", serde_json::to_string(&v).unwrap_or_default());
                v
            }).map_err(LabraError::from),
            _ => Err(LabraError::RequestError(format!("error to parse http response: {:?}", self))),
        }
    }

    pub fn text_blocking(self) -> LabradorResult<String> {
        match self.block_response {
            Some(v) => v.text().map(|v| {
                tracing::info!("[请求第三方接口响应] data:{}", serde_json::to_string(&v).unwrap_or_default());
                v
            }).map_err(LabraError::from),
            _ => Err(LabraError::RequestError(format!("error to parse http response: {:?}", self))),
        }
    }

    pub fn bytes_blocking(self) -> LabradorResult<Bytes> {
        match self.block_response {
            Some(v) => v.bytes().map_err(LabraError::from),
            _ => Err(LabraError::RequestError(format!("error to parse http response: {:?}", self))),
        }
    }
}

pub async fn request<F>(f: F) -> LabradorResult<LabraResponse>
where
    F: Fn(reqwest::Client) -> reqwest::RequestBuilder,
{
    ResponseType::from_response(f(reqwest::Client::new()).send().await?)
}

#[allow(unused)]
pub fn request_blocking<F>(f: F) -> LabradorResult<LabraResponse>
where
    F: Fn(reqwest::blocking::Client) -> reqwest::blocking::RequestBuilder,
{
    ResponseType::from_response_block(f(reqwest::blocking::Client::new()).send()?)
}
