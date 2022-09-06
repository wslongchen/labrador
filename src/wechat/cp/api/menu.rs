use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::{session::SessionStore, request::{RequestType}, WechatCommonResponse, LabradorResult, WechatCpClient};
use crate::wechat::cp::method::{CpMenuMethod, WechatCpMethod};

/// 菜单管理相关接口
#[derive(Debug, Clone)]
pub struct WechatCpMenu<'a, T: SessionStore> {
    client: &'a WechatCpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WechatCpMenu<'a, T> {

    #[inline]
    pub fn new(client: &WechatCpClient<T>) -> WechatCpMenu<T> {
        WechatCpMenu {
            client,
        }
    }

    /// <pre>
    /// 自定义菜单创建接口
    /// 详情请见: <a href="http://mp.weixin.qq.com/wiki/index.php?title=自定义菜单创建接口">文档</a>
    ///
    /// 注意: 这个方法使用配置里的agentId
    /// </pre>
    pub async fn create(&self, menu: WechatCpMenuInfo) -> LabradorResult<WechatCommonResponse> {
        self.create_with_agentid(self.client.agent_id.to_owned().unwrap_or_default(), menu).await
    }

    /// <pre>
    /// 自定义菜单创建接口
    /// 详情请见: <a href="http://mp.weixin.qq.com/wiki/index.php?title=自定义菜单创建接口">文档</a>
    /// </pre>
    pub async fn create_with_agentid(&self, agent_id: i32, req: WechatCpMenuInfo) -> LabradorResult<WechatCommonResponse> {
       self.client.post(WechatCpMethod::Menu(CpMenuMethod::Create(agent_id)), vec![], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 自定义菜单删除接口
    /// 详情请见: <a href="http://mp.weixin.qq.com/wiki/index.php?title=自定义菜单删除接口">文档</a>
    ///
    /// 注意: 这个方法使用配置里的agentId
    /// </pre>
    pub async fn delete(&self) -> LabradorResult<WechatCommonResponse> {
        self.delete_with_agentid(self.client.agent_id.to_owned().unwrap_or_default()).await
    }

    /// <pre>
    /// 自定义菜单删除接口
    /// 详情请见: <a href="http://mp.weixin.qq.com/wiki/index.php?title=自定义菜单删除接口">文档</a>
    /// </pre>
    pub async fn delete_with_agentid(&self, agent_id: i32) -> LabradorResult<WechatCommonResponse> {
       self.client.post(WechatCpMethod::Menu(CpMenuMethod::Delete(agent_id)), vec![], Value::Null, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 自定义菜单查询接口
    /// 详情请见: <a href="http://mp.weixin.qq.com/wiki/index.php?title=自定义菜单查询接口">文档</a>
    ///
    /// 注意: 这个方法使用配置里的agentId
    /// </pre>
    pub async fn get(&self) -> LabradorResult<WechatCommonResponse> {
        self.delete_with_agentid(self.client.agent_id.to_owned().unwrap_or_default()).await
    }

    /// <pre>
    /// 自定义菜单查询接口
    /// 详情请见: <a href="http://mp.weixin.qq.com/wiki/index.php?title=自定义菜单查询接口">文档</a>
    /// </pre>
    pub async fn get_with_agentid(&self, agent_id: i32) -> LabradorResult<WechatCpMenuInfo> {
       let v = self.client.post(WechatCpMethod::Menu(CpMenuMethod::Get(agent_id)), vec![], Value::Null, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpMenuInfo>(v)
    }
}

//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpMenuInfo {
    /// 企业编号
    pub buttons: Vec<WechatCpMenuButton>,
    pub match_rule: Option<WechatCpMenuRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpMenuButton {
    /// <pre>
    /// 菜单的响应动作类型.
    /// view表示网页类型，
    /// click表示点击类型，
    /// miniprogram表示小程序类型
    /// </pre>
    #[serde(rename="type")]
    pub r#type: String,
    /// 菜单标题，不超过16个字节，子菜单不超过60个字节.
    pub name: String,
    /// <pre>
    /// 菜单KEY值，用于消息接口推送，不超过128字节.
    /// click等点击类型必须
    /// </pre>
    pub key: Option<String>,
    /// <pre>
    /// 网页链接.
    /// 用户点击菜单可打开链接，不超过1024字节。type为miniprogram时，不支持小程序的老版本客户端将打开本url。
    /// view、miniprogram类型必须
    /// </pre>
    pub url: Option<String>,
    /// <pre>
    /// 调用新增永久素材接口返回的合法media_id.
    /// media_id类型和view_limited类型必须
    /// </pre>
    pub media_id: Option<String>,
    /// <pre>
    /// 调用发布图文接口获得的article_id.
    /// article_id类型和article_view_limited类型必须
    /// </pre>
    pub article_id: Option<String>,
    /// <pre>
    /// 小程序的appid.
    /// miniprogram类型必须
    /// </pre>
    pub appid: Option<String>,
    /// <pre>
    /// 小程序的页面路径.
    /// miniprogram类型必须
    /// </pre>
    pub pagepath: Option<String>,
    pub sub_button: Vec<WechatCpMenuButton>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpMenuRule {
    pub tag_id: Option<String>,
    pub sex: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    pub client_platform_type: Option<String>,
    pub language: Option<String>,
}
