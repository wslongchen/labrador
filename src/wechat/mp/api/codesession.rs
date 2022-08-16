use serde_json::{Value};
use serde::{Serialize, Deserialize};

use crate::{session::SessionStore, request::{RequestType}, wechat::{mp::method::WechatMpMethod}, WechatCommonResponse, WeChatMpClient, LabradorResult};


#[derive(Debug, Clone)]
pub struct CodeSession<'a, T: SessionStore> {
    client: &'a WeChatMpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> CodeSession<'a, T> {

    #[inline]
    pub fn new(client: &WeChatMpClient<T>) -> CodeSession<T> {
        CodeSession {
            client,
        }
    }

    /// code换取session
    pub async fn jscode_2_session(&self, code: &str) -> LabradorResult<WechatCommonResponse<JsCodeSession>> {
        let v = self.client.get(WechatMpMethod::CodeSession, vec![
            ("grant_type", "authorization_code"),
            ("js_code", code),
            ("appid", &self.client.appid),
            ("secret", &self.client.secret),
        ], RequestType::Json).await?.json::<serde_json::Value>().await?;
        let mut result = serde_json::from_value::<WechatCommonResponse<_>>(v.to_owned())?;
        if result.is_success() {
            result.result = self.json_to_session(&v).into();
        }
        Ok(result)
    }

    fn json_to_session(&self, res: &Value) -> JsCodeSession {
        let _subscribe = &res["subscribe"];
        /* let subscribe = match _subscribe.as_u64()? {
            1 => true,
            0 => false,
            _ => unreachable!(),
        }; */
        let openid = &res["openid"];
        let openid = openid.as_str().unwrap_or_default();
        let session_key = &res["session_key"];
        let session_key = session_key.as_str().unwrap_or_default();
        let unionid = match res.get("unionid") {
            Some(ref uid) => {
                let _uid = uid.as_str().unwrap_or_default();
                Some(_uid.to_owned())
            },
            None => None,
        };
        let remark = &res["remark"];
        let remark = remark.as_str().unwrap_or_default();
        let group_id = &res["groupid"];
        let group_id = group_id.as_u64().unwrap_or_default();
        JsCodeSession {
            openid: openid.to_string(),
            session_key: session_key.to_string(),
            unionid,
            
        }
    }
}

//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JsCodeSession {
    pub openid: String,
    pub session_key: String,
    pub unionid: Option<String>,
}
