use serde_json::{Value};
use serde::{Serialize, Deserialize};

use crate::{session::SessionStore, request::{RequestType}, WechatCommonResponse, LabradorResult};
use crate::wechat::miniapp::method::WechatMaMethod;
use crate::wechat::miniapp::WeChatMaClient;


#[derive(Debug, Clone)]
pub struct WechatMaCodeSession<'a, T: SessionStore> {
    client: &'a WeChatMaClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WechatMaCodeSession<'a, T> {

    #[inline]
    pub fn new(client: &WeChatMaClient<T>) -> WechatMaCodeSession<T> {
        WechatMaCodeSession {
            client,
        }
    }

    /// # code换取session
    /// [文档](https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/user-login/code2Session.html)
    ///
    /// 登录凭证校验。通过 wx.login 接口获得临时登录凭证 code 后传到开发者服务器调用此接口完成登录流程。更多使用方法详见[小程序登录](https://developers.weixin.qq.com/miniprogram/dev/framework/open-ability/login.html)。
    pub async fn jscode_2_session(&self, code: &str) -> LabradorResult<WechatCommonResponse<JsCodeSession>> {
        let v = self.client.get(WechatMaMethod::CodeSession, vec![
            ("grant_type", "authorization_code"),
            ("js_code", code),
            ("appid", &self.client.appid),
            ("secret", &self.client.secret),
        ], RequestType::Json).await?.json::<serde_json::Value>()?;
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
    /// 用户唯一标识
    pub openid: String,
    /// 会话密钥
    pub session_key: String,
    /// 用户在开放平台的唯一标识符，若当前小程序已绑定到微信开放平台帐号下会返回，详见 [UnionID](https://developers.weixin.qq.com/miniprogram/dev/framework/open-ability/union-id.html) 机制说明。
    pub unionid: Option<String>,
}
