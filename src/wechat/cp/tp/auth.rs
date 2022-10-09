use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{LabradorResult, request::RequestType, session::SessionStore, WechatCommonResponse, WechatCpTpClient};
use crate::wechat::cp::constants::{CODE, PROVIDER_ACCESS_TOKEN};
use crate::wechat::cp::method::WechatCpMethod;

#[derive(Debug, Clone)]
pub struct WechatCpTpAuth<'a, T: SessionStore> {
    client: &'a WechatCpTpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WechatCpTpAuth<'a, T> {
    #[inline]
    pub fn new(client: &WechatCpTpClient<T>) -> WechatCpTpAuth<T> {
        WechatCpTpAuth {
            client,
        }
    }

    /// <pre>
    /// 获取访问用户身份
    /// <a href="https://developer.work.weixin.qq.com/document/path/91121">获取访问用户身份</a>
    /// 该接口用于根据code获取成员信息，适用于自建应用与代开发应用
    pub async fn get_user_info_auth_3rd(&self, code: &str) -> LabradorResult<WechatCpOauth2UserInfo3rd> {
        let v = self.client.get(WechatCpMethod::GetAuthUserInfo3rd, vec![(CODE.to_string(), code.to_string())], RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpOauth2UserInfo3rd>(v)
    }
}

//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WechatCpOauth2UserInfo3rd {
    pub corpid: Option<String>,
    #[serde(alias = "UserId", alias = "userid")]
    pub user_id: Option<String>,
    pub user_ticket: Option<String>,
    pub expires_in: Option<i64>,
    pub open_userid: Option<String>,
}