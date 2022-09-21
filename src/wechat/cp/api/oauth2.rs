use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

use crate::{session::SessionStore, request::{RequestType}, WechatCommonResponse, LabradorResult, WechatCpClient};
use crate::wechat::cp::constants::{AGENTID, CODE, SNSAPI_BASE, SNSAPI_PRIVATEINFO, SNSAPI_USERINFO, USER_TICKET};
use crate::wechat::cp::method::{CpOauth2Method, WechatCpMethod};


#[derive(Debug, Clone)]
pub struct WechatCpOauth2<'a, T: SessionStore> {
    client: &'a WechatCpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WechatCpOauth2<'a, T> {

    #[inline]
    pub fn new(client: &WechatCpClient<T>) -> WechatCpOauth2<T> {
        WechatCpOauth2 {
            client,
        }
    }


    /// <pre>
    /// 构造oauth2授权的url连接
    /// 详情请见:  <a href="http://qydev.weixin.qq.com/wiki/index.php?title=企业获取code">文档</a>
    /// </pre>
    pub fn build_authorization_url(&self, redirect_uri: &str, scope: &str, state: Option<&str>) -> String {
        let mut url = format!("{}?appid={}&redirect_uri={}&response_type=code&scope={}", CpOauth2Method::Oauth2Authorize.get_method(), &self.client.corp_id, urlencoding::encode(redirect_uri), scope);
        if SNSAPI_PRIVATEINFO.eq(scope) || SNSAPI_USERINFO.eq(scope) {
            url.push_str("&agentid=");
            url.push_str(&self.client.agent_id.to_owned().unwrap_or_default().to_string())
        }

        if let Some(state) = state {
            url.push_str("&state=");
            url.push_str(state);
        }

        url.push_str("#wechat_redirect");
        url
    }

    /// <pre>
    /// 构造oauth2授权的url连接
    /// 详情请见: <a href="http://qydev.weixin.qq.com/wiki/index.php?title=企业获取code">文档</a>
    /// </pre>
    pub fn build_authorization_url_with_state(&self, state: &str) -> String {
        self.build_authorization_with_url(&self.client.oauth2_redirect_uri.to_owned().unwrap_or_default(), state.into())
    }

    /// <pre>
    /// 构造oauth2授权的url连接
    /// 详情请见: <a href="http://qydev.weixin.qq.com/wiki/index.php?title=企业获取code">文档</a>
    /// </pre>
    pub fn build_authorization_with_url(&self, redirect_uri: &str, state: Option<&str>) -> String {
        self.build_authorization_url(redirect_uri, SNSAPI_BASE, state)
    }


    /// <pre>
    /// 企业号 - 用oauth2获取用户信息
    /// <a href="http://qydev.weixin.qq.com/wiki/index.php?title=根据code获取成员信息">根据code获取成员信息</a>
    /// 因为企业号oauth2.0必须在应用设置里设置通过ICP备案的可信域名，所以无法测试，因此这个方法很可能是坏的。
    ///
    /// 注意: 这个方法使用client里的agentId
    /// </pre>
    pub async fn get_user_info(&self, code: &str) -> LabradorResult<WechatCpOauth2UserInfo> {
        self.get_user_info_with_agent(code, self.client.agent_id.to_owned().unwrap_or_default()).await
    }

    /// <pre>
    /// 获取访问用户身份
    /// <a href="https://work.weixin.qq.com/api/doc#90000/90135/91023">获取访问用户身份</a>
    /// 该接口用于根据code获取成员信息，适用于自建应用与代开发应用
    ///
    /// 注意: 这个方法里的agentId，需要开发人员自己给出
    pub async fn get_user_info_new(&self, code: &str) -> LabradorResult<WechatCpOauth2UserInfo> {
        let v = self.client.get(WechatCpMethod::Oauth2(CpOauth2Method::GetAuthUserInfo), vec![(CODE.to_string(), code.to_string())], RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpOauth2UserInfo>(v)
    }

    /// <pre>
    /// 根据code获取成员信息
    /// <a href="http://qydev.weixin.qq.com/wiki/index.php?title=根据code获取成员信息">根据code获取成员信息</a>
    /// <a href="https://work.weixin.qq.com/api/doc#10028/根据code获取成员信息">根据code获取成员信息</a>
    /// 因为企业号oauth2.0必须在应用设置里设置通过ICP备案的可信域名，所以无法测试，因此这个方法很可能是坏的。
    ///
    /// 注意: 这个方法里的agentId，需要开发人员自己给出
    pub async fn get_user_info_with_agent(&self, code: &str, agent_id: i32) -> LabradorResult<WechatCpOauth2UserInfo> {
        let agent_id = agent_id.to_string();
        let v = self.client.get(WechatCpMethod::Oauth2(CpOauth2Method::GetUserInfo), vec![(CODE.to_string(), code.to_string()), (AGENTID.to_string(), agent_id)], RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpOauth2UserInfo>(v)
    }

    /// <pre>
    /// 使用user_ticket获取成员详情.
    ///
    /// <a href="https://work.weixin.qq.com/api/doc#10028/%E4%BD%BF%E7%94%A8user_ticket%E8%8E%B7%E5%8F%96%E6%88%90%E5%91%98%E8%AF%A6%E6%83%85">文档地址</a>
    /// 请求方式：POST（HTTPS）
    /// <a href="https://qyapi.weixin.qq.com/cgi-bin/user/getuserdetail?access_token=ACCESS_TOKEN">请求地址</a>
    ///
    /// 权限说明：
    /// 需要有对应应用的使用权限，且成员必须在授权应用的可见范围内。
    pub async fn get_user_detail(&self, user_ticket: &str) -> LabradorResult<WechatCpUserDetail> {
        let v = self.client.post(WechatCpMethod::Oauth2(CpOauth2Method::GetUserDetail), vec![], json!({USER_TICKET: user_ticket}), RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpUserDetail>(v)
    }

    /// <pre>
    /// 获取访问用户敏感信息
    /// 自建应用与代开发应用可通过该接口获取成员授权的敏感字段.
    /// <a href="https://developer.work.weixin.qq.com/document/path/95833">获取访问用户敏感信息</a>
    pub async fn get_user_detail_new(&self, user_ticket: &str) -> LabradorResult<WechatCpUserDetail> {
        let v = self.client.post(WechatCpMethod::Oauth2(CpOauth2Method::GetAuthUserDetail), vec![], json!({USER_TICKET: user_ticket}), RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpUserDetail>(v)
    }

}

//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WechatCpOauth2UserInfo {
    #[serde(alias="OpenId", alias="openid")]
    pub openid: Option<String>,
    pub external_userid: Option<String>,
    #[serde(alias="UserId", alias="userid")]
    pub user_id: Option<String>,
    pub user_ticket: Option<String>,
    pub expires_in: Option<i64>,
    #[serde(rename="DeviceId")]
    pub device_id: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WechatCpUserDetail {
    /// 成员UserID
    pub userid: String,
    /// 成员姓名
    pub name: Option<String>,
    /// 成员手机号，仅在用户同意snsapi_privateinfo授权时返回
    pub mobile: Option<String>,
    /// 性别。0表示未定义，1表示男性，2表示女性
    pub gender: Option<String>,
    /// 成员邮箱，仅在用户同意snsapi_privateinfo授权时返回
    pub email: Option<String>,
    /// 头像url。注：如果要获取小图将url最后的”/0”改成”/100”即可。仅在用户同意snsapi_privateinfo授权时返回
    pub avatar: Option<String>,
    /// 员工个人二维码（扫描可添加为外部联系人），仅在用户同意snsapi_privateinfo授权时返回
    pub qr_code: Option<String>,
}