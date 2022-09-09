use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::{session::SessionStore, request::{RequestType}, WechatCommonResponse, LabradorResult, WechatCpClient};
use crate::wechat::cp::method::{CpAgentMethod, WechatCpMethod};

/// 管理企业号应用
#[derive(Debug, Clone)]
pub struct WechatCpAgent<'a, T: SessionStore> {
    client: &'a WechatCpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WechatCpAgent<'a, T> {

    #[inline]
    pub fn new(client: &WechatCpClient<T>) -> WechatCpAgent<T> {
        WechatCpAgent {
            client,
        }
    }

    /// <pre>
    /// 获取企业号应用信息
    /// 该API用于获取企业号某个应用的基本信息，包括头像、昵称、帐号类型、认证类型、可见范围等信息
    /// 详情请见: https://work.weixin.qq.com/api/doc#10087
    /// </pre>
    pub async fn get(&self, agent_id: i32) -> LabradorResult<WechatCpAgentInfo> {
        let v = self.client.get(WechatCpMethod::Agent(CpAgentMethod::Get(agent_id)), vec![], RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpAgentInfo>(v)
    }

    /// <pre>
    /// 设置应用.
    /// 仅企业可调用，可设置当前凭证对应的应用；第三方不可调用。
    /// 详情请见: https://work.weixin.qq.com/api/doc#10088
    /// </pre>
    pub async fn set(&self, req: WechatCpAgentInfo) -> LabradorResult<WechatCommonResponse> {
        self.client.post(WechatCpMethod::Agent(CpAgentMethod::Set), vec![], req,RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 获取应用列表.
    /// 企业仅可获取当前凭证对应的应用；第三方仅可获取被授权的应用。
    /// 详情请见: https://work.weixin.qq.com/api/doc#11214
    /// </pre>
    pub async fn list(&self) -> LabradorResult<WechatCpAgentListResponse> {
        let v = self.client.get(WechatCpMethod::Agent(CpAgentMethod::List), vec![],RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpAgentListResponse>(v)
    }
}

//----------------------------------------------------------------------------------------------------------------------------
/// 企业号应用信息
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WechatCpAgentInfo {
    pub agentid: Option<i32>,
    pub name: Option<String>,
    pub square_logo_url: Option<String>,
    pub logo_mediaid: Option<String>,
    pub description: Option<String>,
    pub allow_userinfos: Option<Users>,
    pub allow_partys: Option<Parties>,
    pub allow_tags: Option<Tags>,
    pub close: Option<i32>,
    pub redirect_domain: Option<String>,
    pub report_location_flag: Option<i32>,
    pub isreportenter: Option<i32>,
    pub home_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Users {
    pub user: Option<Vec<User>>,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Parties {
    pub partyid: Option<Vec<i64>>,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tags {
    pub tagid: Option<Vec<i64>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub userid: Option<String>,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WechatCpAgentListResponse {
    pub agentlist: Option<Vec<WechatCpAgentInfo>>,
}
