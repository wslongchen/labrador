use serde::{Serialize, Deserialize};

use crate::{session::SessionStore, request::{RequestType}, wechat::{mp::method::WechatMpMethod}, WechatCommonResponse, WechatMpClient, LabradorResult, LabraError};
use crate::wechat::mp::constants::{ACCESS_TOKEN, APPID, CODE, GRANT_TYPE, LANG, OPENID, REFRESH_TOKEN, SECRET, ZH_CN};
use crate::wechat::mp::method::Oauth2Method;


#[derive(Debug, Clone)]
pub struct WechatMpOauth2<'a, T: SessionStore> {
    client: &'a WechatMpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WechatMpOauth2<'a, T> {

    #[inline]
    pub fn new(client: &WechatMpClient<T>) -> WechatMpOauth2<T> {
        WechatMpOauth2 {
            client,
        }
    }


    /// # 通过 code 换取网页授权access_token
    ///
    /// 首先请注意，这里通过 code 换取的是一个特殊的网页授权access_token,与基础支持中的access_token（该access_token用于调用其他接口）不同。公众号可通过下述接口来获取网页授权access_token。如果网页授权的作用域为snsapi_base，则本步骤中获取到网页授权access_token的同时，也获取到了openid，snsapi_base式的网页授权流程即到此为止。
    ///
    /// 尤其注意：由于公众号的 secret 和获取到的access_token安全级别都非常高，必须只保存在服务器，不允许传给客户端。后续刷新access_token、通过access_token获取用户信息等步骤，也必须从服务器发起。
    pub async fn oauth2_token(&self, code: &str) -> LabradorResult<WechatMpOauth2AccessTokenResponse> {
        let v = self.client.get(WechatMpMethod::Oauth2(Oauth2Method::AccessToken), vec![
            (GRANT_TYPE.to_string(), "authorization_code".to_string()),
            (CODE.to_string(), code.to_string()),
            (APPID.to_string(), self.client.appid.to_string()),
            (SECRET.to_string(), self.client.secret.to_string()),
        ], RequestType::Json).await?.json::<serde_json::Value>()?;
        let mut result = WechatCommonResponse::from_value(v.clone())?;
        if result.is_success() {
            Ok(serde_json::from_value::<WechatMpOauth2AccessTokenResponse>(v)?)
        } else {
            Err(LabraError::ClientError {errcode: result.errcode.to_owned().unwrap_or_default().to_string(), errmsg: result.errmsg.to_owned().unwrap_or_default()})
        }
    }


    /// # 刷新access_token
    ///
    /// 由于access_token拥有较短的有效期，当access_token超时后，可以使用refresh_token进行刷新，refresh_token有效期为30天，当refresh_token失效之后，需要用户重新授权。
    pub async fn refresh_token(&self, refresh_token: &str) -> LabradorResult<WechatMpOauth2AccessTokenResponse> {
        let v = self.client.get(WechatMpMethod::Oauth2(Oauth2Method::RefreshToken), vec![
            (GRANT_TYPE.to_string(), REFRESH_TOKEN.to_string()),
            (REFRESH_TOKEN.to_string(), refresh_token.to_string()),
            (APPID.to_string(), self.client.appid.to_string()),
        ], RequestType::Json).await?.json::<serde_json::Value>()?;
        let mut result = WechatCommonResponse::from_value(v.to_owned())?;
        if result.is_success() {
            Ok(serde_json::from_value::<WechatMpOauth2AccessTokenResponse>(v)?)
        } else {
            Err(LabraError::ClientError {errcode: result.errcode.to_owned().unwrap_or_default().to_string(), errmsg: result.errmsg.to_owned().unwrap_or_default()})
        }
    }

    /// # 拉取用户信息(需 scope 为 snsapi_userinfo)
    ///
    /// 如果网页授权作用域为snsapi_userinfo，则此时开发者可以通过access_token和 openid 拉取用户信息了。
    pub async fn oauth2_userinfo(&self, access_token: &str, openid: &str) -> LabradorResult<WechatMpOauth2UserInfo> {
        let v = self.client.get(WechatMpMethod::Oauth2(Oauth2Method::UserInfo), vec![
            (ACCESS_TOKEN.to_string(), access_token.to_string()),
            (OPENID.to_string(), openid.to_string()),
            (LANG.to_string(), ZH_CN.to_string()),
        ], RequestType::Json).await?.json::<serde_json::Value>()?;
        let mut result = WechatCommonResponse::from_value(v.to_owned())?;
        if result.is_success() {
            Ok(serde_json::from_value::<WechatMpOauth2UserInfo>(v)?)
        } else {
            Err(LabraError::ClientError {errcode: result.errcode.to_owned().unwrap_or_default().to_string(), errmsg: result.errmsg.to_owned().unwrap_or_default()})
        }
    }

}

//----------------------------------------------------------------------------------------------------------------------------
#[allow(unused)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpOauth2AccessTokenResponse{
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub openid: String,
    pub scope: String,
    pub expires_in: i64,
}


#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct WechatMpOauth2UserInfo {
    pub openid: String,
    pub nickname: String,
    pub sex: u8,
    pub city: String,
    pub province: String,
    pub country: String,
    pub headimgurl: String,
    pub unionid: Option<String>,
}