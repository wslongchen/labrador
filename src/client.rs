use serde::Serialize;

use crate::{request::{LabraResponse, LabraRequest}, session::{SessionStore, SimpleStorage}, LabradorResult, RequestMethod, RequestType, Method};

/// API請求
#[derive(Debug, Clone)]
pub struct APIClient<T: SessionStore> {
    pub app_key: String,
    pub secret: String,
    pub api_path: String,
    pub session: T,
}

/// APIClient
/// 
/// Custom Client's Way
/// 
/// # Examples
/// ```no_run
/// use labrador::{APIClient, SessionStore};
/// struct DemoClient<T: SessionStore> {
///     api_client: APIClient<T>,
/// }
///
/// api_client.request()
/// ```
/// 
#[allow(unused)]
impl<T: SessionStore> APIClient<T> {

    /// # Init the client
    /// 
    /// `app_key` - ThirdPlatform Assign.
    /// `secret` - ThirdPlatform Assign.
    /// `api_path` - ThirdPlatform Url.
    ///
    #[inline]
    pub fn new<Q: Into<String>, S: Into<String>, R: Into<String>>(app_key: Q, secret: R, api_path: S) -> APIClient<SimpleStorage> {
        APIClient {
            app_key: app_key.into(),
            secret: secret.into(),
            api_path: api_path.into(),
            session: SimpleStorage::new()
        }
    }

    #[inline]
    pub fn from_session<Q: Into<String>, S: Into<String>, R: Into<String>>(app_key: Q, secret: R, api_path: S, session: T) -> APIClient<T> {
        APIClient {
            app_key: app_key.into(),
            secret: secret.into(),
            api_path: api_path.into(),
            session: session,
        }
    }

    pub fn session(&self) -> &T {
        &self.session
    }

    /// Request Http/Https
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use labrador::APIClient;
    /// use serde_json::json;
    /// use labrador::Method;
    /// use labrador::{LabraRequest, LabradorResult};
    /// use std::error::Error;
    /// async fn main() -> LabradorResult<(), Error> {
    ///     let api = APIClient::new("appkey", "secret", "http_url");
    ///     let params = vec![(String::from("key"), String::from("value"))];
    ///     let data = json!({
    ///         "key": "value"
    ///     });
    ///     api.request(LabraRequest::new().method(Method::Post).data(data).req_type(request_type).params(params)).await?;
    /// }
    /// ```
    ///
    #[inline]
    pub async fn request<D: Serialize>(&self, mut req: LabraRequest<D>) -> LabradorResult<LabraResponse> {
        let mut api_path = self.api_path.to_owned();
        let LabraRequest { url, ..} = req;
        if url.starts_with("http") {
            req.url = url;
        } else {
            req.url = api_path + &url;
        }
        req.request().await
    }

    /// 发送POST请求
    pub async fn post<D: Serialize, R: RequestMethod>(&self, method: R, mut querys: Vec<(String, String)>, data: D, request_type: RequestType) -> LabradorResult<LabraResponse> {
        let req = LabraRequest::new().url(method.get_method()).params(querys).method(Method::Post).json(data).req_type(request_type);
        self.request(req).await
    }

    /// 发送GET请求
    pub async fn get<R: RequestMethod>(&self, method: R, params: Vec<(String, String)>, request_type: RequestType) -> LabradorResult<LabraResponse> {
        let req = LabraRequest::<String>::new().url(method.get_method()).params(params).method(Method::Get).req_type(request_type);
        self.request(req).await
    }
}



