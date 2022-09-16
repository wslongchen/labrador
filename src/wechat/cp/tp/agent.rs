use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{LabradorResult, request::RequestType, session::SessionStore, WechatCommonResponse, WechatCpTpClient};
use crate::wechat::cp::constants::{PROVIDER_ACCESS_TOKEN};
use crate::wechat::cp::method::{CpDepartmentMethod, WechatCpMethod};

/// 部门管理
#[derive(Debug, Clone)]
pub struct WechatCpTpAgent<'a, T: SessionStore> {
    client: &'a WechatCpTpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WechatCpTpAgent<'a, T> {
    #[inline]
    pub fn new(client: &WechatCpTpClient<T>) -> WechatCpTpAgent<T> {
        WechatCpTpAgent {
            client,
        }
    }

    /// <pre>
    /// 获取带参授权链接
    /// 该API用于获取代开发自建应用授权链接，用于生成带参临时二维码。
    /// 详情请见: https://developer.work.weixin.qq.com/document/path/95436
    /// </pre>
    pub async fn create(&self, state: &str, templateid_list: Vec<&str>) -> LabradorResult<WechatCpTpProxyResponse> {
        let req = json!({"state":state,"templateid_list":templateid_list});
        let access_token = self.client.get_wechat_provider_token().await?;
        let query = vec![(PROVIDER_ACCESS_TOKEN.to_string(), access_token)];
        let v = self.client.post(WechatCpMethod::GetCustomizedAuthUrl, query, req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpTpProxyResponse>(v)
    }
}

//----------------------------------------------------------------------------------------------------------------------------

/// 应用代开发 获取带参授权链接返回结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpTpProxyResponse {
    pub qrcode_url: String,
    pub expires_in: i32,
}
