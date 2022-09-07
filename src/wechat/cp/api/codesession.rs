use serde::{Serialize, Deserialize};

use crate::{session::SessionStore, request::{RequestType}, WechatCommonResponse, LabradorResult, WechatCpClient};
use crate::wechat::cp::constants::{AUTHORIZATION_CODE, GRANT_TYPE, JS_CODE};
use crate::wechat::cp::method::WechatCpMethod;


#[derive(Debug, Clone)]
pub struct WechatCpCodeSession<'a, T: SessionStore> {
    client: &'a WechatCpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WechatCpCodeSession<'a, T> {

    #[inline]
    pub fn new(client: &WechatCpClient<T>) -> WechatCpCodeSession<T> {
        WechatCpCodeSession {
            client,
        }
    }

    /// # 小程序登录凭证校验
    pub async fn jscode_2_session(&self, code: &str) -> LabradorResult<WechatCpJsCodeSession> {
        let v = self.client.get(WechatCpMethod::JsCode2Session, vec![
            (GRANT_TYPE.to_string(), AUTHORIZATION_CODE.to_string()),
            (JS_CODE.to_string(), code.to_string()),
        ], RequestType::Json).await?.json::<serde_json::Value>()?;
        WechatCommonResponse::parse::<WechatCpJsCodeSession>(v)
    }
}

//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WechatCpJsCodeSession {
    /// 企业编号
    pub corpid: String,
    /// 会话密钥
    pub session_key: String,
    pub userid: Option<String>,
}
