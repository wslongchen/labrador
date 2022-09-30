use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

use crate::{session::SessionStore, request::{RequestType}, WechatCommonResponse, LabradorResult, WechatCpClient, LabraError};
use crate::wechat::cp::method::{CpTagMethod, WechatCpMethod};

/// 标签相关
#[derive(Debug, Clone)]
pub struct WechatCpTag<'a, T: SessionStore> {
    client: &'a WechatCpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WechatCpTag<'a, T> {

    #[inline]
    pub fn new(client: &WechatCpClient<T>) -> WechatCpTag<T> {
        WechatCpTag {
            client,
        }
    }

    /// 创建标签.
    /// <pre>
    /// 请求地址：<a href="https://qyapi.weixin.qq.com/cgi-bin/tag/create?access_token=ACCESS_TOKEN">文档</a>
    /// 文档地址：<a href="https://work.weixin.qq.com/api/doc#90000/90135/90210">文档</a>
    /// </pre>
    pub async fn create(&self, name: &str, id: Option<i32>) -> LabradorResult<String> {
        let req = json!({
            "tagname": name,
            "tagid": id,
        });
        let v = self.client.post(WechatCpMethod::Tag(CpTagMethod::Create), vec![], req, RequestType::Json).await?.json::<Value>()?;
        let v = WechatCommonResponse::parse::<Value>(v)?;
        let tag_id = v["tagid"].as_str().unwrap_or_default();
        Ok(tag_id.to_string())
    }

    /// 更新标签.
    pub async fn update(&self, tag_id: &str, tag_name: &str) -> LabradorResult<WechatCommonResponse> {
        let req = json!({
            "tagname": tag_name,
            "tagid": tag_id,
        });
        self.client.post(WechatCpMethod::Tag(CpTagMethod::Update), vec![], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// 删除标签.
    pub async fn delete(&self, tag_id: &str) -> LabradorResult<WechatCommonResponse> {
        self.client.get(WechatCpMethod::Tag(CpTagMethod::Delete(tag_id.to_string())), vec![], RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// 获取标签成员.
    pub async fn get(&self, tag_id: &str) -> LabradorResult<WechatCpTagGetResponse> {
        let v = self.client.get(WechatCpMethod::Tag(CpTagMethod::Get(tag_id.to_string())), vec![], RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpTagGetResponse>(v)
    }

    /// 增加标签成员.
    pub async fn add_users_tag(&self, tag_id: &str, user_ids: Vec<String>, party_ids: Vec<String>) -> LabradorResult<WechatCpTagAddOrRemoveUsersResponse> {
        let req = json!({
            "tagid": tag_id,
            "userlist": user_ids,
            "partylist": party_ids
        });
        let v = self.client.post(WechatCpMethod::Tag(CpTagMethod::AddTagUsers), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpTagAddOrRemoveUsersResponse>(v)
    }

    /// 移除标签成员.
    pub async fn remove_users_tag(&self, tag_id: &str, user_ids: Vec<String>, party_ids: Vec<String>) -> LabradorResult<WechatCpTagAddOrRemoveUsersResponse> {
        let req = json!({
            "tagid": tag_id,
            "userlist": user_ids,
            "partylist": party_ids
        });
        let v = self.client.post(WechatCpMethod::Tag(CpTagMethod::DeleteTagUsers), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpTagAddOrRemoveUsersResponse>(v)
    }

    /// 获得标签列表.
    pub async fn list_all(&self) -> LabradorResult<Vec<WechatCpTagInfo>> {
        let v = self.client.get(WechatCpMethod::Tag(CpTagMethod::List), vec![], RequestType::Json).await?.json::<Value>()?;
        let v = WechatCommonResponse::parse::<Value>(v)?;
        serde_json::from_value::<Vec<WechatCpTagInfo>>(v["taglist"].to_owned()).map_err(LabraError::from)
    }
}

//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct WechatCpTagGetResponse {
    /// 用户列表
    pub userid: Vec<WechatCpUserInfo>,
    /// 部门列表
    pub partylist: Vec<i32>,
    pub tagname: Option<String>,
}

/// 为标签添加或移除用户结果对象类
#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct WechatCpTagAddOrRemoveUsersResponse {
    pub invalidlist: Option<String>,
    pub invalidparty: Option<Vec<String>>,
}


#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct WechatCpTagInfo {
    pub tagid: Option<String>,
    pub tagname: Option<Vec<String>>,
}

/// 微信用户信息
#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct WechatCpUserInfo {
    pub userid: Option<String>,
    pub new_user_id: Option<String>,
    pub name: Option<String>,
    pub depart_ids: Option<Vec<i32>>,
    pub orders: Option<Vec<i32>>,
    pub position: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub biz_mail: Option<String>,
    pub thumb_avatar: Option<String>,
    pub main_department: Option<String>,
    /// 全局唯一。对于同一个服务商，不同应用获取到企业内同一个成员的open_userid是相同的，最多64个字节。仅第三方应用可获取
    pub open_user_id: Option<String>,
    pub address: Option<String>,
    pub avatar_media_id: Option<String>,
    /// 别名；第三方仅通讯录应用可获取
    pub alias: Option<String>,
    pub status: Option<i32>,
    pub is_leader: Option<i32>,
    /// is_leader_in_dept.
    /// 个数必须和department一致，表示在所在的部门内是否为上级。1表示为上级，0表示非上级。在审批等应用里可以用来标识上级审批人
    pub is_leader_in_dept: Option<Vec<i32>>,
    pub ext_attrs: Option<Vec<Attr>>,
    pub enable: Option<i32>,
    pub avatar: Option<String>,
    pub gender: Option<u8>,
    pub hide_mobile: Option<u8>,
    pub english_name: Option<u8>,
    pub telephone: Option<u8>,
    pub to_invite: Option<u8>,
    pub qr_code: Option<u8>,
    pub positions: Option<Vec<String>>,
    /// 成员对外信息
    pub external_attrs: Option<Vec<ExternalAttribute>>,
    pub external_position: Option<String>,
    pub external_corp_name: Option<String>,
    pub direct_leader: Option<Vec<String>>,
    pub wechat_channels: Option<WechatChannels>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalAttribute {
    /// 属性类型: 0-本文 1-网页 2-小程序.
    #[serde(rename = "type")]
    pub r#type: Option<u8>,
    /// 属性名称： 需要先确保在管理端有创建改属性，否则会忽略.
    pub name: Option<String>,
    /// 文本属性内容,长度限制12个UTF8字符.
    pub value: Option<String>,
    /// 网页的url,必须包含http或者https头.
    pub url: Option<String>,
    /// 小程序的展示标题,长度限制12个UTF8字符.
    pub title: Option<String>,
    /// 小程序appid，必须是有在本企业安装授权的小程序，否则会被忽略.
    pub appid: Option<String>,
    /// 小程序的页面路径
    pub page_path: Option<String>,
}


#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct Attr {
    /// 属性类型: 0-文本 1-网页
    #[serde(rename="type")]
    pub r#type: Option<i32>,
    pub name: Option<String>,
    pub text_value: Option<String>,
    pub web_url: Option<String>,
    pub web_title: Option<String>,
}


#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct WechatChannels {
    pub nickname: Option<String>,
    pub status: Option<i32>,
}
