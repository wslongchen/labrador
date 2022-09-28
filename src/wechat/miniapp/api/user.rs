use std::collections::HashMap;
use serde_json::{json};

use serde::{Serialize, Deserialize};

use crate::{session::SessionStore, errors::LabraError, wechat::{cryptos::WechatCrypto}, request::RequestType, WechatCommonResponse, LabradorResult};
use crate::wechat::miniapp::method::{MaUserMethod, WechatMaMethod};
use crate::wechat::miniapp::WechatMaClient;

/// 用户信息相关操作
#[derive(Debug, Clone)]
pub struct WechatMaUser<'a, T: SessionStore> {
    client: &'a WechatMaClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WechatMaUser<'a, T> {

    #[inline]
    pub fn new(client: &WechatMaClient<T>) -> WechatMaUser<T> {
        WechatMaUser {
            client,
        }
    }

    /// 解密用户敏感数据
    pub fn decrypt_user_info(&self, session_key: &str, encrypted_data: &str, iv: &str) -> LabradorResult<WechatMaUserResponse> {
        let result = WechatCrypto::decrypt_data(session_key, encrypted_data, iv)?;
        serde_json::from_str::<WechatMaUserResponse>(&result).map_err(LabraError::from)
    }

    /// 上报用户数据后台接口.
    /// <p>小游戏可以通过本接口上报key-value数据到用户的CloudStorage。</p>
    ///
    /// [文档参考](https://developers.weixin.qq.com/minigame/dev/document/open-api/data/setUserStorage.html)
    pub async fn set_user_storage(&self,session_key: &str, openid: &str, kv: &HashMap<String, String>) -> LabradorResult<WechatCommonResponse> {
        let mut params = Vec::new();
        for (k, v) in kv.into_iter() {
            params.push(json!({
                "key": k,
                "value": v
            }));
        }
        let req = json!({
            "kv_list": params
        });
        let signature = WechatCrypto::create_hmac_sha256_sign(session_key, &req.to_string())?;
        self.client.post(WechatMaMethod::User(MaUserMethod::SetUserStorage), vec![("appid".to_string(), self.client.secret.to_string()),
          ("signature".to_string(), signature),("openid".to_string(), openid.to_string()),("sig_method".to_string(), "hmac_sha256".to_string()),], &req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// 获取手机号信息,基础库:2.21.2及以上
    /// [文档](https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/user-info/phone-number/getPhoneNumber.html)
    pub async fn get_phone_info(&self,code: &str) -> LabradorResult<PhoneInfo> {
        let req = json!({
            "code": code
        });
        let v = self.client.post(WechatMaMethod::User(MaUserMethod::GetPhoneNumber), vec![], &req, RequestType::Json).await?.json::<serde_json::Value>()?;
        WechatCommonResponse::parse(v)
    }

    /// 解密用户手机号信息.
    pub async fn decrypt_phone_info(&self, session_key: &str, encrypted_data: &str, iv: &str) -> LabradorResult<PhoneInfo> {
        let result = WechatCrypto::decrypt_data(session_key, encrypted_data, iv)?;
        serde_json::from_str::<PhoneInfo>(&result).map_err(LabraError::from)
    }
}

//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WechatMaUserResponse {
    pub nick_name: String,
    pub gender: u8,
    pub language: String,
    pub city: String,
    pub province: String,
    pub country: String,
    pub avatar_url: String,
    /// 不绑定开放平台不会返回这个字段
    pub union_id: Option<String>,
    pub watermark: Option<serde_json::Value>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhoneInfo {
    /// 用户绑定的手机号（国外手机号会有区号）
    pub phone_number: Option<String>,
    /// 没有区号的手机号
    pub pure_phone_number: Option<String>,
    /// 区号
    pub country_code: Vec<String>,
}
