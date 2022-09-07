use serde::{Serialize, Deserialize};
use serde_json::{Value};

use crate::{session::SessionStore, request::{RequestType}, WechatCommonResponse, LabradorResult, WechatCpTpClient};
use crate::wechat::cp::constants::ACCESS_TOKEN;
use crate::wechat::cp::method::{CpDepartmentMethod, WechatCpMethod};

/// 部门管理
#[derive(Debug, Clone)]
pub struct WechatCpTpDepartment<'a, T: SessionStore> {
    client: &'a WechatCpTpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WechatCpTpDepartment<'a, T> {

    #[inline]
    pub fn new(client: &WechatCpTpClient<T>) -> WechatCpTpDepartment<T> {
        WechatCpTpDepartment {
            client,
        }
    }

    /// <pre>
    /// 部门管理接口 - 创建部门.
    /// 最多支持创建500个部门
    /// 详情请见: https://work.weixin.qq.com/api/doc#90000/90135/90205
    /// </pre>
    pub async fn create(&self, req: WechatCpTpDepartInfo) -> LabradorResult<i64> {

        let v = self.client.post(WechatCpMethod::Department(CpDepartmentMethod::Create), vec![], req, RequestType::Json).await?.json::<Value>()?;
        let v = WechatCommonResponse::parse::<Value>(v)?;
        let tag_id = v["id"].as_i64().unwrap_or_default();
        Ok(tag_id)
    }

    /// <pre>
    /// 部门管理接口 - 获取部门列表.
    /// 详情请见: https://work.weixin.qq.com/api/doc#90000/90135/90208
    /// </pre>
    pub async fn list_byid(&self, id: Option<i64>, corp_id: &str) -> LabradorResult<Vec<WechatCpTpDepartInfo>> {
        let access_token = self.client.get_access_token(corp_id);
        let mut query = vec![(ACCESS_TOKEN.to_string(), access_token)];
        let access_token = self.client.get_access_token(corp_id);
        if let Some(id) = id {
            query.push(("id".to_string(), id.to_string()));
        }
        let v = self.client.get(WechatCpMethod::Department(CpDepartmentMethod::List), query, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<Vec<WechatCpTpDepartInfo>>(v)
    }


    /// <pre>
    /// 部门管理接口 - 获取部门列表.
    /// 详情请见: https://work.weixin.qq.com/api/doc#90000/90135/90208
    /// </pre>
    pub async fn list(&self, corp_id: &str) -> LabradorResult<Vec<WechatCpTpDepartInfo>> {
        self.list_byid(None, corp_id).await
    }

    /// <pre>
    /// 部门管理接口 - 更新部门.
    /// 详情请见: https://work.weixin.qq.com/api/doc#90000/90135/90206
    /// 如果id为0(未部门),1(黑名单),2(星标组)，或者不存在的id，微信会返回系统繁忙的错误
    /// </pre>
    pub async fn update(&self, req: WechatCpTpDepartInfo) -> LabradorResult<WechatCommonResponse> {
        self.client.post(WechatCpMethod::Department(CpDepartmentMethod::Update), vec![], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 部门管理接口 - 删除部门.
    /// 详情请见: https://work.weixin.qq.com/api/doc#90000/90135/90207
    /// 应用须拥有指定部门的管理权限
    /// </pre>
    pub async fn delete(&self, depart_id: i64) -> LabradorResult<WechatCommonResponse> {
        self.client.get(WechatCpMethod::Department(CpDepartmentMethod::Delete(depart_id)), vec![], RequestType::Json).await?.json::<WechatCommonResponse>()
    }
}

//----------------------------------------------------------------------------------------------------------------------------
/// 企业微信的部门
#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct WechatCpTpDepartInfo {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub en_name: Option<String>,
    pub parentid: Option<i32>,
    pub order: Option<i32>,
}