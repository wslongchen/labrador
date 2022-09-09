use serde_json::{json, Value};

use crate::{session::SessionStore, request::{RequestType}, WechatCommonResponse, LabradorResult, WechatCpTpClient, WechatCpTagAddOrRemoveUsersResponse, WechatCpTagGetResponse, WechatCpTagInfo};
use crate::wechat::cp::method::{CpTagMethod, WechatCpMethod};

/// 企业微信第三方开发-标签相关
#[derive(Debug, Clone)]
pub struct WechatCpTpTag<'a, T: SessionStore> {
    client: &'a WechatCpTpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WechatCpTpTag<'a, T> {

    #[inline]
    pub fn new(client: &WechatCpTpClient<T>) -> WechatCpTpTag<T> {
        WechatCpTpTag {
            client,
        }
    }

    /// 创建标签.
    /// <pre>
    /// 请求地址：<a href="https://qyapi.weixin.qq.com/cgi-bin/tag/create?access_token=ACCESS_TOKEN">文档</a>
    /// 文档地址：<a href="https://work.weixin.qq.com/api/doc/90001/90143/90346">文档</a>
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
        WechatCommonResponse::parse::<Vec<WechatCpTagInfo>>(v)
    }
}

//----------------------------------------------------------------------------------------------------------------------------
