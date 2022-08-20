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
    pub async fn jscode_2_session(&self, code: &str) -> LabradorResult<JsCodeSession> {
        let v = self.client.get(WechatMaMethod::CodeSession, vec![
            ("grant_type", "authorization_code"),
            ("js_code", code),
            ("appid", &self.client.appid),
            ("secret", &self.client.secret),
        ], RequestType::Json).await?.json::<serde_json::Value>()?;
        WechatCommonResponse::parse::<JsCodeSession>(v)
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
