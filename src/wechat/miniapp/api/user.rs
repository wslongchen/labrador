use std::collections::HashMap;
use serde_json::{json};

use serde::{Serialize, Deserialize};

use crate::{session::SessionStore, errors::LabraError, wechat::{cryptos::WeChatCrypto}, request::RequestType, WechatCommonResponse, LabradorResult};
use crate::wechat::miniapp::method::{MaUserMethod, WechatMaMethod};
use crate::wechat::miniapp::WeChatMaClient;

/// 用户信息相关操作
#[derive(Debug, Clone)]
pub struct WeChatMaUser<'a, T: SessionStore> {
    client: &'a WeChatMaClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WeChatMaUser<'a, T> {

    #[inline]
    pub fn new(client: &WeChatMaClient<T>) -> WeChatMaUser<T> {
        WeChatMaUser {
            client,
        }
    }

    /// 解密用户敏感数据
    pub fn decrypt_user_info(&self, session_key: &str, encrypted_data: &str, iv: &str) -> LabradorResult<WechatMaUserResponse> {
        let result = WeChatCrypto::decrypt_data(session_key, encrypted_data, iv)?;
        serde_json::from_str::<WechatMaUserResponse>(&result).map_err(LabraError::from)
    }

    /// 上报用户数据后台接口.
    /// <p>小游戏可以通过本接口上报key-value数据到用户的CloudStorage。</p>
    ///
    /// [文档参考](https://developers.weixin.qq.com/minigame/dev/document/open-api/data/setUserStorage.html)
    pub async fn set_user_storage(&self,session_key: &str, openid: &str, kv: &HashMap<String, String>) -> LabradorResult<WechatCommonResponse<String>> {
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
        let signature = WeChatCrypto::create_hmac_sha256_sign(session_key, &req.to_string())?;
        let v = self.client.post(WechatMaMethod::User(MaUserMethod::SetUserStorage), vec![("appid".to_string(), self.client.secret.to_string()),
          ("signature".to_string(), signature),("openid".to_string(), openid.to_string()),("sig_method".to_string(), "hmac_sha256".to_string()),], &req, RequestType::Json).await?.json::<serde_json::Value>()?;
        serde_json::from_value::<WechatCommonResponse<_>>(v).map_err(LabraError::from)
    }
}

//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WechatMaUserResponse {
    pub nick_name: String,
    pub gender: String,
    pub language: String,
    pub city: String,
    pub province: String,
    pub country: String,
    pub avatar_url: String,
    /// 不绑定开放平台不会返回这个字段
    pub union_id: Option<String>,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Followers {
    pub total: u64,
    pub count: u64,
    pub openids: Vec<String>,
    pub next_openid: String,
}
