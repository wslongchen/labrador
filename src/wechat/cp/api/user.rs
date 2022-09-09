use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

use crate::{session::SessionStore, request::{RequestType}, WechatCommonResponse, LabradorResult, WechatCpClient, ExternalContact, FollowedUser, WechatCpUserInfo};
use crate::wechat::cp::constants::ACCESS_TOKEN;
use crate::wechat::cp::method::{CpUserMethod, WechatCpMethod};

/// 部门管理
#[derive(Debug, Clone)]
pub struct WechatCpUser<'a, T: SessionStore> {
    client: &'a WechatCpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WechatCpUser<'a, T> {

    #[inline]
    pub fn new(client: &WechatCpClient<T>) -> WechatCpUser<T> {
        WechatCpUser {
            client,
        }
    }

    /// <pre>
    ///   用在二次验证的时候.
    ///   企业在员工验证成功后，调用本方法告诉企业号平台该员工关注成功。
    /// </pre>
    pub async fn authenticate(&self, user_id: &str) -> LabradorResult<WechatCommonResponse> {
       self.client.get(WechatCpMethod::User(CpUserMethod::AuthSuccess(user_id.to_string())), vec![], RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 获取部门成员详情
    /// 请求方式：GET（HTTPS）
    /// 请求地址：https://qyapi.weixin.qq.com/cgi-bin/user/list?access_token=ACCESS_TOKEN&department_id=DEPARTMENT_ID&fetch_child=FETCH_CHILD
    ///
    /// 文档地址：https://work.weixin.qq.com/api/doc/90000/90135/90201
    /// </pre>
    pub async fn list_by_department(&self, depart_id: i64, fetch_child: Option<bool>, status: Option<i32>) -> LabradorResult<Vec<WechatCpUserInfo>> {
        let mut query = vec![];
        if let Some(fetch_child) = fetch_child {
            query.push(("fetch_child".to_string(), fetch_child.to_string()));
        }
        if let Some(status) = status {
            query.push(("status".to_string(), status.to_string()));
        } else {
            query.push(("status".to_string(), "0".to_string()));
        }
        let v = self.client.get(WechatCpMethod::User(CpUserMethod::List(depart_id)), query, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<Vec<WechatCpUserInfo>>(v)
    }

    /// <pre>
    /// 获取部门成员.
    ///
    /// http://qydev.weixin.qq.com/wiki/index.php?title=管理成员#.E8.8E.B7.E5.8F.96.E9.83.A8.E9.97.A8.E6.88.90.E5.91.98
    /// </pre>
    pub async fn list_simple_by_department(&self, depart_id: i64, fetch_child: Option<bool>, status: Option<i32>) -> LabradorResult<Vec<WechatCpUserInfo>> {
        let mut query = vec![];
        if let Some(fetch_child) = fetch_child {
            query.push(("fetch_child".to_string(), fetch_child.to_string()));
        }
        if let Some(status) = status {
            query.push(("status".to_string(), status.to_string()));
        } else {
            query.push(("status".to_string(), "0".to_string()));
        }
        let v = self.client.get(WechatCpMethod::User(CpUserMethod::SimpleList(depart_id)), query, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<Vec<WechatCpUserInfo>>(v)
    }


    /// <pre>
    /// 新建用户
    /// </pre>
    pub async fn create(&self, req: WechatCpUserInfo) -> LabradorResult<WechatCommonResponse> {
        self.client.post(WechatCpMethod::User(CpUserMethod::Create), vec![], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 更新用户
    /// </pre>
    pub async fn update(&self, req: WechatCpUserInfo) -> LabradorResult<WechatCommonResponse> {
        self.client.post(WechatCpMethod::User(CpUserMethod::Update), vec![], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 删除用户/批量删除成员.
    /// http://qydev.weixin.qq.com/wiki/index.php?title=管理成员#.E6.89.B9.E9.87.8F.E5.88.A0.E9.99.A4.E6.88.90.E5.91.98
    /// </pre>
    pub async fn delete(&self, user_ids: Vec<&str>) -> LabradorResult<WechatCommonResponse> {
        if user_ids.len() == 1 {
            self.client.get(WechatCpMethod::User(CpUserMethod::Delete(user_ids[0].to_string())), vec![], RequestType::Json).await?.json::<WechatCommonResponse>()
        } else {
            self.client.post(WechatCpMethod::User(CpUserMethod::BatchDelete), vec![], json!({"useridlist": user_ids}), RequestType::Json).await?.json::<WechatCommonResponse>()
        }

    }

    /// <pre>
    /// 获取用户
    /// </pre>
    pub async fn get_by_id(&self, userid: &str, corp_id: &str) -> LabradorResult<WechatCpUserInfo> {
        let access_token = self.client.get_access_token(corp_id);
        let query = vec![(ACCESS_TOKEN.to_string(), access_token)];
        let v = self.client.get(WechatCpMethod::User(CpUserMethod::Get(userid.to_string())), query,RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpUserInfo>(v)
    }

    /// <pre>
    /// 邀请成员.
    /// 企业可通过接口批量邀请成员使用企业微信，邀请后将通过短信或邮件下发通知。
    /// 请求方式：POST（HTTPS）
    /// 请求地址： https://qyapi.weixin.qq.com/cgi-bin/batch/invite?access_token=ACCESS_TOKEN
    /// 文档地址：https://work.weixin.qq.com/api/doc#12543
    /// </pre>
    pub async fn invite(&self, userids: Vec<&str>, party_ids: Vec<&str>, tag_ids: Vec<&str>) -> LabradorResult<WxCpInviteResponse> {
        let req = json!({
            "user": userids,
            "party": party_ids,
            "tag": tag_ids,
        });
        let v = self.client.post(WechatCpMethod::User(CpUserMethod::Invite), vec![],req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WxCpInviteResponse>(v)
    }

    /// <pre>
    ///  userid转openid.
    ///  该接口使用场景为微信支付、微信红包和企业转账。
    ///
    /// 在使用微信支付的功能时，需要自行将企业微信的userid转成openid。
    /// 在使用微信红包功能时，需要将应用id和userid转成appid和openid才能使用。
    /// 注：需要成员使用微信登录企业微信或者关注微信插件才能转成openid
    ///
    /// 文档地址：https://work.weixin.qq.com/api/doc#11279
    /// </pre>
    pub async fn userid_2_openid(&self, userid: &str, agent_id: i32) -> LabradorResult<WxCpUseridToOpenidResponse> {
        let req = json!({
            "userid": userid,
            "agentid": agent_id,
        });
        let v = self.client.post(WechatCpMethod::User(CpUserMethod::ConvertToOpenid), vec![],req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WxCpUseridToOpenidResponse>(v)
    }

    /// <pre>
    /// openid转userid.
    ///
    /// 该接口主要应用于使用微信支付、微信红包和企业转账之后的结果查询。
    /// 开发者需要知道某个结果事件的openid对应企业微信内成员的信息时，可以通过调用该接口进行转换查询。
    /// 权限说明：
    /// 管理组需对openid对应的企业微信成员有查看权限。
    ///
    /// 文档地址：https://work.weixin.qq.com/api/doc#11279
    /// </pre>
    pub async fn openid_2_userid(&self, openid: &str) -> LabradorResult<String> {
        let req = json!({
            "openid": openid,
        });
        let v = self.client.post(WechatCpMethod::User(CpUserMethod::ConvertToUserid), vec![],req, RequestType::Json).await?.json::<Value>()?;
        let v= WechatCommonResponse::parse::<Value>(v)?;
        let userid = v["userid"].as_str().unwrap_or_default();
        Ok(userid.to_string())
    }

    /// <pre>
    ///
    /// 通过手机号获取其所对应的userid。
    ///
    /// 请求方式：POST（HTTPS）
    /// 请求地址：https://qyapi.weixin.qq.com/cgi-bin/user/getuserid?access_token=ACCESS_TOKEN
    ///
    /// 文档地址：https://work.weixin.qq.com/api/doc#90001/90143/91693
    /// </pre>
    pub async fn get_userid(&self, mobile: &str) -> LabradorResult<String> {
        let req = json!({
            "mobile": mobile,
        });
        let v = self.client.post(WechatCpMethod::User(CpUserMethod::GetUserid), vec![],req, RequestType::Json).await?.json::<Value>()?;
        let v= WechatCommonResponse::parse::<Value>(v)?;
        let userid = v["userid"].as_str().unwrap_or_default();
        Ok(userid.to_string())
    }

    /// 获取外部联系人详情.
    /// <pre>
    ///   企业可通过此接口，根据外部联系人的userid，拉取外部联系人详情。权限说明：
    /// 企业需要使用外部联系人管理secret所获取的accesstoken来调用
    /// 第三方应用需拥有“企业客户”权限。
    /// 第三方应用调用时，返回的跟进人follow_user仅包含应用可见范围之内的成员。
    /// </pre>
    pub async fn get_external_contact(&self, userid: &str) -> LabradorResult<WechatCpUserExternalContactInfo> {
        let v = self.client.get(WechatCpMethod::User(CpUserMethod::GetExternalContact(userid.to_string())), vec![],RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpUserExternalContactInfo>(v)
    }

    /// <pre>
    /// 获取加入企业二维码。
    ///
    /// 请求方式：GET（HTTPS）
    /// 请求地址：https://qyapi.weixin.qq.com/cgi-bin/corp/get_join_qrcode?access_token=ACCESS_TOKEN&size_type=SIZE_TYPE
    ///
    /// 文档地址：https://work.weixin.qq.com/api/doc/90000/90135/91714
    /// </pre>
    pub async fn get_join_qrcode(&self, size_type: i32) -> LabradorResult<String> {
        let v = self.client.get(WechatCpMethod::User(CpUserMethod::GetJoinQrcode(size_type)), vec![],RequestType::Json).await?.json::<Value>()?;
        let v = WechatCommonResponse::parse::<Value>(v)?;
        let qrcode = v["join_qrcode"].as_str().unwrap_or_default();
        Ok(qrcode.to_string())
    }

    /// <pre>
    /// 获取企业活跃成员数。
    ///
    /// 请求方式：POST（HTTPS）
    /// 请求地址：<a href="https://qyapi.weixin.qq.com/cgi-bin/user/get_active_stat?access_token=ACCESS_TOKEN">https://qyapi.weixin.qq.com/cgi-bin/user/get_active_stat?access_token=ACCESS_TOKEN</a>
    ///
    /// 文档地址：<a href="https://developer.work.weixin.qq.com/document/path/92714">https://developer.work.weixin.qq.com/document/path/92714</a>
    /// </pre>
    pub async fn get_active_count(&self, date: &str) -> LabradorResult<u64> {
        let v = self.client.post(WechatCpMethod::User(CpUserMethod::GetActiveStat), vec![], json!({"date": date}),RequestType::Json).await?.json::<Value>()?;
        let active_cnt = v["active_cnt"].as_u64().unwrap_or_default();
        Ok(active_cnt)
    }
}

//----------------------------------------------------------------------------------------------------------------------------
/// 邀请成员的结果对象类
#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct WxCpInviteResponse {
    pub invaliduser: Option<Vec<String>>,
    pub invalidparty: Option<Vec<String>>,
    pub invalidtag: Option<Vec<String>>,
}
#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct WxCpUseridToOpenidResponse {
    pub openid: Option<String>,
    pub appid: Option<String>,
}
/// 外部联系人详情
#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct WechatCpUserExternalContactInfo {
    pub external_contact: Option<ExternalContact>,
    pub follow_user: Option<FollowedUser>,
}