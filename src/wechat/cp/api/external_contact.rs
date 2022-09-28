use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

use crate::{session::SessionStore, LabradorResult, RequestType, WechatCpClient, LabraError, WechatCommonResponse};
use crate::wechat::cp::constants::{CURSOR, EXTERNAL_USERID, USERID, WELCOME_MSG_TYPE_FILE, WELCOME_MSG_TYPE_IMAGE, WELCOME_MSG_TYPE_LINK, WELCOME_MSG_TYPE_MINIPROGRAM, WELCOME_MSG_TYPE_VIDEO};
use crate::wechat::cp::method::{CpExternalContactMethod, WechatCpMethod};


/// 外部联系人管理接口
#[derive(Debug, Clone)]
pub struct WechatCpExternalContact<'a, T: SessionStore> {
    client: &'a WechatCpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WechatCpExternalContact<'a, T> {
    #[inline]
    pub fn new(client: &WechatCpClient<T>) -> WechatCpExternalContact<T> {
        WechatCpExternalContact {
            client,
        }
    }

    /// 配置客户联系「联系我」方式
    /// <pre>
    /// 企业可以在管理后台-客户联系中配置成员的「联系我」的二维码或者小程序按钮，客户通过扫描二维码或点击小程序上的按钮，即可获取成员联系方式，主动联系到成员。
    /// 企业可通过此接口为具有客户联系功能的成员生成专属的「联系我」二维码或者「联系我」按钮。
    /// 如果配置的是「联系我」按钮，需要开发者的小程序接入小程序插件。
    ///
    /// 注意:
    /// 通过API添加的「联系我」不会在管理端进行展示，每个企业可通过API最多配置50万个「联系我」。
    /// 用户需要妥善存储返回的config_id，config_id丢失可能导致用户无法编辑或删除「联系我」。
    /// 临时会话模式不占用「联系我」数量，但每日最多添加10万个，并且仅支持单人。
    /// 临时会话模式的二维码，添加好友完成后该二维码即刻失效。
    /// </pre>
    pub async fn add_contact_way(&self, req: WechatCpContactWayInfo) -> LabradorResult<WechatCpContactWayInfoResponse> {
        if let Some(user) = &req.contact_way.user {
            if user.len() > 100 {
                return Err(LabraError::RequestError("「联系我」使用人数默认限制不超过100人(包括部门展开后的人数)".to_string()));
            }
        }
        let v = self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::AddContactWay), vec![], req.contact_way, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpContactWayInfoResponse>(v)
    }

    /// 获取企业已配置的「联系我」方式
    ///
    /// <pre>
    /// <b>批量</b>获取企业配置的「联系我」二维码和「联系我」小程序按钮。
    /// </pre>
    pub async fn get_contact_way(&self, config_id: &str) -> LabradorResult<WechatCpContactWayInfo> {
        let v = self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::GetContactWay), vec![], json!({"config_id": config_id}), RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpContactWayInfo>(v)
    }

    /// 更新企业已配置的「联系我」方式
    ///
    /// <pre>
    /// 更新企业配置的「联系我」二维码和「联系我」小程序按钮中的信息，如使用人员和备注等。
    /// </pre>
    pub async fn update_contact_way(&self, req: WechatCpContactWayInfo) -> LabradorResult<WechatCommonResponse> {
        if req.contact_way.config_id.is_none() {
            return Err(LabraError::RequestError("更新「联系我」方式需要指定configId".to_string()));
        }
        if let Some(user) = &req.contact_way.user {
            if user.len() > 100 {
                return Err(LabraError::RequestError("「联系我」使用人数默认限制不超过100人(包括部门展开后的人数)".to_string()));
            }
        }
        self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::UpdateContactWay), vec![], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// 删除企业已配置的「联系我」方式
    ///
    /// <pre>
    /// 删除一个已配置的「联系我」二维码或者「联系我」小程序按钮。
    /// </pre>
    pub async fn delete_contact_way(&self, config_id: &str) -> LabradorResult<WechatCommonResponse> {
        self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::DeleteContactWay), vec![], json!({"config_id": config_id}), RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// 结束临时会话
    ///
    /// <pre>
    /// 将指定的企业成员和客户之前的临时会话断开，断开前会自动下发已配置的结束语。
    ///
    /// 注意：请保证传入的企业成员和客户之间有仍然有效的临时会话, 通过<b>其他方式的添加外部联系人无法通过此接口关闭会话</b>。
    /// </pre>
    pub async fn close_temp_chat(&self, user_id: &str, external_user_id: &str) -> LabradorResult<WechatCommonResponse> {
        self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::CloseTmpChat), vec![], json!({"userid": user_id, "external_userid": external_user_id}), RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// 获取客户详情.
    /// <pre>
    ///
    /// 企业可通过此接口，根据外部联系人的userid（如何获取?），拉取客户详情。
    ///
    /// 请求方式：GET（HTTPS）
    /// 请求地址：<a href="https://qyapi.weixin.qq.com/cgi-bin/externalcontact/get?access_token=ACCESS_TOKEN&external_userid=EXTERNAL_USERID">地址</a>
    ///
    /// 权限说明：
    ///
    /// 企业需要使用“客户联系”secret或配置到“可调用应用”列表中的自建应用secret所获取的accesstoken来调用（accesstoken如何获取？）；
    /// 第三方/自建应用调用时，返回的跟进人follow_user仅包含应用可见范围之内的成员。
    /// </pre>
    pub async fn get_contact_detail(&self, user_id: &str, cursor: &str) -> LabradorResult<WechatCpExternalContactInfoResponse> {
        let v = self.client.get(WechatCpMethod::ExternalContact(CpExternalContactMethod::GetContactWayDetail), vec![(EXTERNAL_USERID.to_string(), user_id.to_string()), (CURSOR.to_string(), cursor.to_string())], RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpExternalContactInfoResponse>(v)
    }

    /// 企业和服务商可通过此接口，将微信外部联系人的userid转为微信openid，用于调用支付相关接口。暂不支持企业微信外部联系人（ExternalUserid为wo开头）的userid转openid。
    pub async fn convert_openid(&self, external_userid: &str) -> LabradorResult<String> {
        let v = self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::ConvertToOpenid), vec![], json!({"external_userid": external_userid}), RequestType::Json).await?.json::<Value>()?;
        let v = WechatCommonResponse::parse::<Value>(v)?;
        let openid = v["openid"].as_str().unwrap_or_default().to_string();
        Ok(openid)
    }

    /// 服务商为企业代开发微信小程序的场景，服务商可通过此接口，将微信客户的unionid转为external_userid。
    /// <pre>
    ///
    /// 文档地址：<a href="https://work.weixin.qq.com/api/doc/90001/90143/93274">地址</a>
    ///
    /// 服务商代开发小程序指企业使用的小程序为企业主体的，非服务商主体的小程序。
    /// 场景：企业客户在微信端从企业主体的小程序（非服务商应用）登录，同时企业在企业微信安装了服务商的第三方应用，服务商可以调用该接口将登录用户的unionid转换为服务商全局唯一的外部联系人id
    ///
    /// 权限说明：
    ///
    /// 仅认证企业可调用
    /// unionid必须是企业主体下的unionid。即unionid的主体（为绑定了该小程序的微信开放平台账号主体）需与当前企业的主体一致。
    /// unionid的主体（即微信开放平台账号主体）需认证
    /// 该客户的跟进人必须在应用的可见范围之内
    /// </pre>
    pub async fn unionid_to_external_userid(&self, unionid: &str, openid: Option<&str>) -> LabradorResult<String> {
        let mut req = json!({
            "unionid": unionid
        });
        if let Some(openid) = openid {
            req["openid"] = openid.into();
        }
        let v = self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::UnionidToExternalUserid), vec![], req, RequestType::Json).await?.json::<Value>()?;
        let v = WechatCommonResponse::parse::<Value>(v)?;
        let external_userid = v["external_userid"].as_str().unwrap_or_default().to_string();
        Ok(external_userid)
    }

    /// <pre>
    /// 配置客户群进群方式
    /// 企业可以在管理后台-客户联系中配置「加入群聊」的二维码或者小程序按钮，客户通过扫描二维码或点击小程序上的按钮，即可加入特定的客户群。
    /// 企业可通过此接口为具有客户联系功能的成员生成专属的二维码或者小程序按钮。
    /// 如果配置的是小程序按钮，需要开发者的小程序接入小程序插件。
    /// 注意:
    /// 通过API添加的配置不会在管理端进行展示，每个企业可通过API最多配置50万个「加入群聊」(与「联系我」共用50万的额度)。
    /// 文档地址：<a href="https://developer.work.weixin.qq.com/document/path/92229">地址</a>
    /// </pre>
    pub async fn add_join_way(&self, unionid: &str, req: WechatCpGroupJoinWayInfo) -> LabradorResult<WechatCpGroupJoinWayResponse> {
        if let Some(chat_ids) = &req.join_way.chat_id_list {
            if chat_ids.len() > 5 {
                return Err(LabraError::RequestError("使用该配置的客户群ID列表，支持5个".to_string()));
            }
        }
        let v = self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::GroupChatAddJoinWay), vec![], req.join_way, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpGroupJoinWayResponse>(v)
    }

    /// <pre>
    /// 更新客户群进群方式配置
    /// 更新进群方式配置信息。注意：使用覆盖的方式更新。
    /// 文档地址：<a href="https://developer.work.weixin.qq.com/document/path/92229">地址</a>
    /// </pre>
    pub async fn update_join_way(&self, unionid: &str, req: WechatCpGroupJoinWayInfo) -> LabradorResult<WechatCommonResponse> {
        if let Some(chat_ids) = &req.join_way.chat_id_list {
            if chat_ids.len() > 5 {
                return Err(LabraError::RequestError("使用该配置的客户群ID列表，支持5个".to_string()));
            }
        }
        self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::GroupChatUpdateJoinWay), vec![], req.join_way, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 获取客户群进群方式配置
    /// 获取企业配置的群二维码或小程序按钮。
    /// 文档地址：<a href="https://developer.work.weixin.qq.com/document/path/92229">地址</a>
    /// </pre>
    pub async fn get_join_way(&self, unionid: &str, config_id: &str) -> LabradorResult<WechatCpContactWayInfo> {
        let v = self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::GroupChatGetJoinWay), vec![], json!({"config_id": config_id}), RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpContactWayInfo>(v)
    }

    /// <pre>
    /// 删除客户群进群方式配置
    /// 文档地址：<a href="https://developer.work.weixin.qq.com/document/path/92229">地址</a>
    /// </pre>
    pub async fn delete_join_way(&self, unionid: &str, config_id: &str) -> LabradorResult<WechatCommonResponse> {
        self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::GroupChatDeleteJoinWay), vec![], json!({"config_id": config_id}), RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// 批量获取客户详情.
    /// <pre>
    ///
    /// 企业/第三方可通过此接口获取指定成员添加的客户信息列表。
    ///
    /// 请求方式：POST（HTTPS）
    /// 请求地址：<a href="https://qyapi.weixin.qq.com/cgi-bin/externalcontact/batch/get_by_user?access_token=ACCESS_TOKEN">文档</a>
    ///
    /// 权限说明：
    ///
    /// 企业需要使用“客户联系”secret或配置到“可调用应用”列表中的自建应用secret所获取的accesstoken来调用（accesstoken如何获取？）；
    /// 第三方/自建应用调用时，返回的跟进人follow_user仅包含应用可见范围之内的成员。
    /// </pre>
    pub async fn get_contact_detail_batch(&self, userid_list: Vec<String>, cursor: Option<&str>, limit: Option<i32>) -> LabradorResult<WechatCpExternalContactBatchInfoResponse> {
        let mut req = json!({
            "userid_list": userid_list,
        });
        if let Some(cursor) = cursor {
            req["cursor"] = cursor.into();
        }
        if let Some(limit) = limit {
            req["limit"] = limit.into();
        }
        let v = self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::BatchGetByUser), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpExternalContactBatchInfoResponse>(v)
    }

    /// 修改客户备注信息.
    /// <pre>
    /// 企业可通过此接口修改指定用户添加的客户的备注信息。
    /// 请求方式: POST(HTTP)
    /// 请求地址: <a href="https://qyapi.weixin.qq.com/cgi-bin/externalcontact/remark?access_token=ACCESS_TOKEN">地址</a>
    /// 文档地址：<a href="https://work.weixin.qq.com/api/doc/90000/90135/92115">地址</a>
    /// </pre>
    pub async fn update_remark(&self, req: WechatCpUpdateRemarkRequest) -> LabradorResult<WechatCommonResponse> {
        self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::Remark), vec![], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// 获取客户列表.
    /// <pre>
    ///   企业可通过此接口获取指定成员添加的客户列表。客户是指配置了客户联系功能的成员所添加的外部联系人。没有配置客户联系功能的成员，所添加的外部联系人将不会作为客户返回。
    ///
    /// 请求方式：GET（HTTPS）
    /// 请求地址：<a href="https://qyapi.weixin.qq.com/cgi-bin/externalcontact/list?access_token=ACCESS_TOKEN&userid=USERID">地址</a>
    ///
    /// 权限说明：
    ///
    /// 企业需要使用“客户联系”secret或配置到“可调用应用”列表中的自建应用secret所获取的accesstoken来调用（accesstoken如何获取？）；
    /// 第三方应用需拥有“企业客户”权限。
    /// 第三方/自建应用只能获取到可见范围内的配置了客户联系功能的成员。
    /// </pre>
    pub async fn list_external_contacts(&self, userid: &str) -> LabradorResult<Vec<String>> {
        let v = self.client.get(WechatCpMethod::ExternalContact(CpExternalContactMethod::List), vec![(USERID.to_string(), userid.to_string())], RequestType::Json).await?.json::<Value>()?;
        let v = WechatCommonResponse::parse::<Value>(v)?;
        let external_userids = v["external_userid"].as_array().unwrap_or(&vec![]).iter().map(|v| v.as_str().unwrap_or_default().to_string()).collect::<Vec<String>>();
        Ok(external_userids)
    }

    /// 企业和第三方服务商可通过此接口获取配置了客户联系功能的成员(Customer Contact)列表。
    /// <pre>
    ///   企业需要使用外部联系人管理secret所获取的accesstoken来调用（accesstoken如何获取？）；
    ///   第三方应用需拥有“企业客户”权限。
    ///   第三方应用只能获取到可见范围内的配置了客户联系功能的成员
    /// </pre>
    pub async fn list_followers(&self) -> LabradorResult<Vec<String>> {
        let v = self.client.get(WechatCpMethod::ExternalContact(CpExternalContactMethod::GetFollowUserList), vec![], RequestType::Json).await?.json::<Value>()?;
        let v = WechatCommonResponse::parse::<Value>(v)?;
        let follow_users = v["follow_user"].as_array().unwrap_or(&vec![]).iter().map(|v| v.as_str().unwrap_or_default().to_string()).collect::<Vec<String>>();
        Ok(follow_users)
    }

    /// 获取待分配的离职成员列表
    /// 企业和第三方可通过此接口，获取所有离职成员的客户列表，并可进一步调用分配离职成员的客户接口将这些客户重新分配给其他企业成员。
    ///
    /// 请求方式：POST（HTTPS）
    /// 请求地址：<a href="https://qyapi.weixin.qq.com/cgi-bin/externalcontact/get_unassigned_list?access_token=ACCESS_TOKEN">地址</a>
    pub async fn list_unassigned(&self, page_id: Option<u64>, cursor: &str, page_size: Option<u64>) -> LabradorResult<WechatCpUserExternalUnassignList> {
        let mut req = json!({
            "cursor": cursor,
            "page_size": page_size.unwrap_or(1000)
        });
        if let Some(page_id) = page_id {
            req["page_id"] = page_id.into();
        }
        let v = self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::GetUnassignedList), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpUserExternalUnassignList>(v)
    }

    /// 企业可通过此接口，转接在职成员的客户给其他成员。
    /// <per>
    /// external_userid必须是handover_userid的客户（即配置了客户联系功能的成员所添加的联系人）。
    /// 在职成员的每位客户最多被分配2次。客户被转接成功后，将有90个自然日的服务关系保护期，保护期内的客户无法再次被分配。
    /// <p>
    /// 权限说明：
    /// * 企业需要使用“客户联系”secret或配置到“可调用应用”列表中的自建应用secret所获取的accesstoken来调用（accesstoken如何获取？）。
    /// 第三方应用需拥有“企业客户权限->客户联系->在职继承”权限
    /// 接替成员必须在此第三方应用或自建应用的可见范围内。
    /// 接替成员需要配置了客户联系功能。
    /// 接替成员需要在企业微信激活且已经过实名认证。
    /// </per>
    pub async fn transfer_customer(&self, req: WechatCpUserTransferCustomerRequest) -> LabradorResult<WechatCpUserTransferCustomerResponse> {
        let v = self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::TransferCustomer), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpUserTransferCustomerResponse>(v)
    }

    /// 企业和第三方可通过此接口查询在职成员的客户转接情况。
    /// <per>
    /// 权限说明：
    /// <p>
    /// 企业需要使用“客户联系”secret或配置到“可调用应用”列表中的自建应用secret所获取的accesstoken来调用（accesstoken如何获取？）。
    /// 第三方应用需拥有“企业客户权限->客户联系->在职继承”权限
    /// 接替成员必须在此第三方应用或自建应用的可见范围内。
    /// </per>
    pub async fn transfer_result(&self, hand_over_userid: &str, take_over_userid: &str, cursor: &str) -> LabradorResult<WechatCpUserTransferResultResponse> {
        let req = json!({
            "cursor": cursor,
            "handover_userid": hand_over_userid,
            "takeover_userid": take_over_userid,
        });
        let v = self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::TransferResult), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpUserTransferResultResponse>(v)
    }

    /// 企业可通过此接口，分配离职成员的客户给其他成员。
    /// <per>
    /// handover_userid必须是已离职用户。
    /// external_userid必须是handover_userid的客户（即配置了客户联系功能的成员所添加的联系人）。
    /// 在职成员的每位客户最多被分配2次。客户被转接成功后，将有90个自然日的服务关系保护期，保护期内的客户无法再次被分配。
    /// <p>
    /// 权限说明：
    /// <p>
    /// 企业需要使用“客户联系”secret或配置到“可调用应用”列表中的自建应用secret所获取的accesstoken来调用（accesstoken如何获取？）。
    /// 第三方应用需拥有“企业客户权限->客户联系->离职分配”权限
    /// 接替成员必须在此第三方应用或自建应用的可见范围内。
    /// 接替成员需要配置了客户联系功能。
    /// 接替成员需要在企业微信激活且已经过实名认证。
    /// </per>
    pub async fn resigned_transfer_customer(&self, req: WechatCpUserTransferCustomerRequest) -> LabradorResult<WechatCpUserTransferCustomerResponse> {
        let v = self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::ResignedTransferCustomer), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpUserTransferCustomerResponse>(v)
    }

    /// 企业和第三方可通过此接口查询离职成员的客户分配情况。
    /// <per>
    /// 权限说明：
    /// <p>
    /// 企业需要使用“客户联系”secret或配置到“可调用应用”列表中的自建应用secret所获取的accesstoken来调用（accesstoken如何获取？）。
    /// 第三方应用需拥有“企业客户权限->客户联系->在职继承”权限
    /// 接替成员必须在此第三方应用或自建应用的可见范围内。
    /// </per>
    pub async fn resigned_transfer_result(&self, hand_over_userid: &str, take_over_userid: &str, cursor: &str) -> LabradorResult<WechatCpUserTransferResultResponse> {
        let req = json!({
            "cursor": cursor,
            "handover_userid": hand_over_userid,
            "takeover_userid": take_over_userid,
        });
        let v = self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::ResignedTransferResult), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpUserTransferResultResponse>(v)
    }

    /// <pre>
    /// 该接口用于获取配置过客户群管理的客户群列表。
    /// 企业需要使用“客户联系”secret或配置到“可调用应用”列表中的自建应用secret所获取的accesstoken来调用（accesstoken如何获取？）。
    /// 暂不支持第三方调用。
    /// 微信文档：<a href="https://work.weixin.qq.com/api/doc/90000/90135/92119">地址</a>
    /// </pre>
    pub async fn list_group_chat(&self, limit: Option<u64>, cursor: &str, status: u8, user_ids: Vec<String>) -> LabradorResult<WechatCpUserExternalGroupChatList> {
        let mut req = json!({
            "cursor": cursor,
            "limit": limit.unwrap_or(100),
            "status_filter": status,
        });
        if !user_ids.is_empty() {
            req["owner_filter"] = json!({
                "userid_list": user_ids
            });
        }
        let v = self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::GroupChatList), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpUserExternalGroupChatList>(v)
    }

    /// <pre>
    /// 通过客户群ID，获取详情。包括群名、群成员列表、群成员入群时间、入群方式。（客户群是由具有客户群使用权限的成员创建的外部群）
    /// 企业需要使用“客户联系”secret或配置到“可调用应用”列表中的自建应用secret所获取的accesstoken来调用（accesstoken如何获取？）。
    /// 暂不支持第三方调用。
    /// 微信文档：<a href="https://work.weixin.qq.com/api/doc/90000/90135/92122">地址</a>
    /// </pre>
    pub async fn get_group_chat(&self, chat_id: &str, need_name: u8) -> LabradorResult<WechatCpUserExternalGroupChatInfoResponse> {
        let req = json!({
            "chat_id": chat_id,
            "need_name": need_name,
        });
        let v = self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::GroupChatGet), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpUserExternalGroupChatInfoResponse>(v)
    }

    /// 企业可通过此接口，将已离职成员为群主的群，分配给另一个客服成员。
    ///
    /// <pre>
    /// 注意：：
    /// 群主离职了的客户群，才可继承
    /// 继承给的新群主，必须是配置了客户联系功能的成员
    /// 继承给的新群主，必须有设置实名
    /// 继承给的新群主，必须有激活企业微信
    /// 同一个人的群，限制每天最多分配300个给新群主
    /// 权限说明:
    /// 企业需要使用“客户联系”secret或配置到“可调用应用”列表中的自建应用secret所获取的accesstoken来调用（accesstoken如何获取？）。
    /// 第三方应用需拥有“企业客户权限->客户联系->分配离职成员的客户群”权限
    /// 对于第三方/自建应用，群主必须在应用的可见范围。
    /// </pre>
    pub async fn transfer_group_chat(&self, chat_ids: Vec<&str>, new_owner: &str) -> LabradorResult<WechatCpUserExternalGroupChatTransferResponse> {
        let mut req = json!({
            "new_owner": new_owner,
        });
        if !chat_ids.is_empty() {
            req["chat_id_list"] = chat_ids.into()
        }
        let v = self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::GroupChatTransfer), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpUserExternalGroupChatTransferResponse>(v)
    }

    /// <pre>
    /// 企业可通过此接口获取成员联系客户的数据，包括发起申请数、新增客户数、聊天数、发送消息数和删除/拉黑成员的客户数等指标。
    /// 企业需要使用“客户联系”secret或配置到“可调用应用”列表中的自建应用secret所获取的accesstoken来调用（accesstoken如何获取？）。
    /// 第三方应用需拥有“企业客户”权限。
    /// 第三方/自建应用调用时传入的userid和partyid要在应用的可见范围内;
    /// </pre>
    pub async fn get_user_behavior_statistic(&self, start_time: u64, end_time: u64, user_ids: Vec<&str>, party_ids: Vec<String>) -> LabradorResult<WechatCpUserExternalUserBehaviorStatistic> {
        let mut req = json!({
            "start_time": start_time / 1000,
            "end_time": end_time / 1000,
        });
        if !user_ids.is_empty() {
            req["userid"] = user_ids.into()
        }
        if !party_ids.is_empty() {
            req["partyid"] = party_ids.into()
        }
        let v = self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::GetUserBehaviorData), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpUserExternalUserBehaviorStatistic>(v)
    }

    /// <pre>
    /// 获取指定日期全天的统计数据。注意，企业微信仅存储60天的数据。
    /// 企业需要使用“客户联系”secret或配置到“可调用应用”列表中的自建应用secret所获取的accesstoken来调用（accesstoken如何获取？）。
    /// 暂不支持第三方调用。
    /// </pre>
    pub async fn get_group_chat_statistic(&self, start_time: u64, order_by: Option<u8>, order_asc: Option<u8>, page_index: Option<u64>, page_size: Option<u64>, user_ids: Vec<&str>, party_ids: Vec<String>) -> LabradorResult<WechatCpUserExternalGroupChatStatistic> {
        let mut req = json!({
            "day_begin_time": start_time / 1000,
            "order_by": order_by.unwrap_or(1),
            "order_asc": order_asc.unwrap_or(0),
            "offset": page_index.unwrap_or(0),
            "limit": page_size.unwrap_or(500),
        });
        if !user_ids.is_empty() || !party_ids.is_empty() {
            req["owner_filter"] = json!({
                "userid_list": user_ids,
                "partyid_list": party_ids,
            });
        }
        let v = self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::GroupChatStatistic), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpUserExternalGroupChatStatistic>(v)
    }

    /// <pre>
    /// 添加企业群发消息任务
    /// 企业可通过此接口添加企业群发消息的任务并通知客服人员发送给相关客户或客户群。（注：企业微信终端需升级到2.7.5版本及以上）
    /// 注意：调用该接口并不会直接发送消息给客户/客户群，需要相关的客服人员操作以后才会实际发送（客服人员的企业微信需要升级到2.7.5及以上版本）
    /// 同一个企业每个自然月内仅可针对一个客户/客户群发送4条消息，超过限制的用户将会被忽略。
    /// 请求方式: POST(HTTP)
    /// 请求地址:<a href="https://qyapi.weixin.qq.com/cgi-bin/externalcontact/add_msg_template?access_token=ACCESS_TOKEN">地址</a>
    /// <p>
    /// 文档地址：<a href="https://work.weixin.qq.com/api/doc/90000/90135/92135">地址</a>
    /// </pre>
    pub async fn add_msg_template(&self, msg_template: WechatCpMsgTemplate) -> LabradorResult<WechatCpMsgTemplateAddResponse> {
        let v = self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::AddMsgTemplate), vec![], msg_template, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpMsgTemplateAddResponse>(v)
    }

    /// 发送新客户欢迎语
    /// <pre>
    /// 企业微信在向企业推送添加外部联系人事件时，会额外返回一个welcome_code，企业以此为凭据调用接口，即可通过成员向新添加的客户发送个性化的欢迎语。
    /// 为了保证用户体验以及避免滥用，企业仅可在收到相关事件后20秒内调用，且只可调用一次。
    /// 如果企业已经在管理端为相关成员配置了可用的欢迎语，则推送添加外部联系人事件时不会返回welcome_code。
    /// 每次添加新客户时可能有多个企业自建应用/第三方应用收到带有welcome_code的回调事件，但仅有最先调用的可以发送成功。后续调用将返回41051（externaluser has started chatting）错误，请用户根据实际使用需求，合理设置应用可见范围，避免冲突。
    /// 请求方式: POST(HTTP)
    ///
    /// 请求地址:<a href="https://qyapi.weixin.qq.com/cgi-bin/externalcontact/send_welcome_msg?access_token=ACCESS_TOKEN">地址</a>
    ///
    /// 文档地址：<a href="https://work.weixin.qq.com/api/doc/90000/90135/92137">地址</a>
    /// </pre>
    pub async fn send_welcome_msg(&self, msg: WechatCpWelcomeMsg) -> LabradorResult<WechatCommonResponse> {
        self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::SendWelcomeMsg), vec![], msg, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 企业可通过此接口获取企业客户标签详情。
    /// </pre>
    pub async fn get_corp_tag_list(&self, tag_id: Vec<&str>, group_id: Vec<Vec<&str>>) -> LabradorResult<WechatCpUserExternalTagGroupInfo> {
        let v = self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::GetCorpTagList), vec![], json!({"tag_id": tag_id, "group_id": group_id}), RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpUserExternalTagGroupInfo>(v)
    }

    /// <pre>
    /// 企业可通过此接口向客户标签库中添加新的标签组和标签，每个企业最多可配置3000个企业标签。
    /// 暂不支持第三方调用。
    /// </pre>
    pub async fn add_corp_tag(&self, req: WechatCpUserExternalTagGroupInfo) -> LabradorResult<WechatCpUserExternalTagGroupInfo> {
        let v = self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::AddCorpTag), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpUserExternalTagGroupInfo>(v)
    }

    /// <pre>
    /// 企业可通过此接口编辑客户标签/标签组的名称或次序值。
    /// 暂不支持第三方调用。
    /// </pre>
    pub async fn edit_corp_tag(&self, id: &str, name: &str, order: u64) -> LabradorResult<WechatCommonResponse> {
        let req = json!({
            "id": id,
            "name": name,
            "order": order
        });
        self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::EditCorpTag), vec![], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 企业可通过此接口删除客户标签库中的标签，或删除整个标签组。
    /// 暂不支持第三方调用。
    /// </pre>
    pub async fn delete_corp_tag(&self, tag_id: Vec<&str>, group_id: Vec<&str>) -> LabradorResult<WechatCommonResponse> {
        let req = json!({
            "tag_id": tag_id,
            "group_id": group_id
        });
        self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::DeleteCorpTag), vec![], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 企业可通过此接口为指定成员的客户添加上由企业统一配置的标签。
    /// <a href="https://work.weixin.qq.com/api/doc/90000/90135/92117">地址</a>
    /// </pre>
    pub async fn mark_tag(&self, userid: &str, external_userid: &str, add_tag: Vec<&str>, remove_tag: Vec<&str>) -> LabradorResult<WechatCommonResponse> {
        let req = json!({
            "userid": userid,
            "external_userid": external_userid,
            "add_tag": add_tag,
            "remove_tag": remove_tag
        });
        self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::MarkTag), vec![], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 企业和第三方应用可通过此接口获取企业与成员的群发记录。
    /// <a href="https://work.weixin.qq.com/api/doc/90000/90135/93338">地址</a>
    /// </pre>
    pub async fn get_group_msg_list_v2(&self, chat_type: &str, start_time: u64, end_time: u64, creator: &str, filter_type: u8, limit: i32, cursor: &str) -> LabradorResult<WechatCpGroupMsgListResult> {
        let req = json!({
            "chat_type": chat_type,
            "start_time": start_time / 1000,
            "end_time": end_time / 1000,
            "creator": creator,
            "filter_type": filter_type,
            "limit": limit,
            "cursor": cursor
        });
        let v = self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::GetGroupMsgListV2), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpGroupMsgListResult>(v)
    }

    /// <pre>
    /// 企业和第三方应用可通过此接口获取企业与成员的群发记录。
    /// <a href="https://work.weixin.qq.com/api/doc/90000/90135/93338#获取企业群发成员执行结果">地址</a>
    /// </pre>
    pub async fn get_group_msg_send_result(&self, msgid: &str, userid: &str, limit: i32, cursor: &str) -> LabradorResult<WechatCpGroupMsgSendResult> {
        let req = json!({
            "msgid": msgid,
            "userid": userid,
            "limit": limit,
            "cursor": cursor
        });
        let v = self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::GetGroupMsgSendResult), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpGroupMsgSendResult>(v)
    }

    /// <pre>
    /// 企业跟第三方应用可通过该接口获取到创建企业群发的群发发送结果。
    /// <a href="https://work.weixin.qq.com/api/doc/16251">地址</a>
    /// </pre>
    pub async fn get_group_msg_result(&self, msgid: &str, limit: i32, cursor: &str) -> LabradorResult<WechatCpGroupMsgResult> {
        let req = json!({
            "msgid": msgid,
            "limit": limit,
            "cursor": cursor
        });
        let v = self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::GetGroupMsgResult), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpGroupMsgResult>(v)
    }

    /// <pre>
    /// 获取群发成员发送任务列表。
    /// <a href="https://work.weixin.qq.com/api/doc/90000/90135/93338#获取群发成员发送任务列表">地址</a>
    /// </pre>
    pub async fn get_group_msg_task(&self, msgid: &str, limit: i32, cursor: &str) -> LabradorResult<WechatCpGroupMsgTaskResult> {
        let req = json!({
            "msgid": msgid,
            "limit": limit,
            "cursor": cursor
        });
        let v = self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::GetGroupMsgTask), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpGroupMsgTaskResult>(v)
    }

    /// <pre>
    /// 添加入群欢迎语素材。
    /// <a href="https://open.work.weixin.qq.com/api/doc/90000/90135/92366#添加入群欢迎语素材">地址</a>
    /// </pre>
    pub async fn add_group_welcome_template(&self, req: WechatCpGroupWelcomeTemplateInfo) -> LabradorResult<String> {
        let v = self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::AddGroupWelcomeTemplate), vec![], req, RequestType::Json).await?.json::<Value>()?;
        let v = WechatCommonResponse::parse::<Value>(v)?;
        let template_id = v["template_id"].as_str().unwrap_or_default().to_string();
        Ok(template_id)
    }

    /// <pre>
    /// 编辑入群欢迎语素材。
    /// <a href="https://open.work.weixin.qq.com/api/doc/90000/90135/92366#编辑入群欢迎语素材">地址</a>
    /// </pre>
    pub async fn edit_group_welcome_template(&self, req: WechatCpGroupWelcomeTemplateInfo) -> LabradorResult<WechatCommonResponse> {
        self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::EditGroupWelcomeTemplate), vec![], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 获取入群欢迎语素材。
    /// <a href="https://open.work.weixin.qq.com/api/doc/90000/90135/92366#获取入群欢迎语素材">地址</a>
    /// </pre>
    pub async fn get_group_welcome_template(&self, template_id: &str) -> LabradorResult<WechatCpGroupWelcomeTemplateInfo> {
        let v = self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::GetGroupWelcomeTemplate), vec![], json!({"template_id": template_id}), RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpGroupWelcomeTemplateInfo>(v)
    }

    /// <pre>
    /// 删除入群欢迎语素材。
    /// 企业可通过此API删除入群欢迎语素材，且仅能删除调用方自己创建的入群欢迎语素材。
    /// <a href="https://open.work.weixin.qq.com/api/doc/90000/90135/92366#删除入群欢迎语素材">地址</a>
    /// </pre>
    pub async fn delete_group_welcome_template(&self, template_id: &str, agent_id: Option<&str>) -> LabradorResult<WechatCommonResponse> {
        let mut req = json!({"template_id": template_id});
        if let Some(agent) = agent_id {
            req["agentid"] = agent.into();
        }
        self.client.post(WechatCpMethod::ExternalContact(CpExternalContactMethod::DeleteGroupWelcomeTemplate), vec![], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }
}

//----------------------------------------------------------------------------------------------------------------------------


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpContactWayInfo {
    pub contact_way: ContactWay,
}


/// 「联系我」方式 对象
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactWay {
    /// 联系方式的配置id
    pub config_id: Option<String>,
    /// 联系方式类型,1-单人, 2-多人
    #[serde(rename = "type")]
    pub r#type: u8,
    /// 场景，1-在小程序中联系，2-通过二维码联系
    pub scene: u8,
    /// <pre>
    /// 非必填
    /// 在小程序中联系时使用的控件样式
    /// <b>单人样式(type=1)时可选1,2,3</b>
    /// <b>多人样式(type=2)时可选1,2</b>
    /// </pre>
    pub style: Option<u8>,
    /// 联系方式的备注信息，用于助记，不超过30个字符
    pub remark: Option<String>,
    /// 外部客户添加时是否无需验证，默认为true
    pub skip_verify: Option<bool>,
    /// 企业自定义的state参数，用于区分不同的添加渠道，在调用“获取外部联系人详情(getContactDetail)”  时会返回该参数值，不超过30个字符
    pub state: Option<String>,
    /// 联系二维码的URL，仅在scene为2时返回
    pub qr_code: Option<String>,
    /// 使用该联系方式的用户userID列表，在type为1时为必填，且只能有一个
    pub user: Option<Vec<String>>,
    /// 使用该联系方式的部门id列表，只在type为2时有效
    pub party: Option<Vec<String>>,
    /// 是否临时会话模式，true表示使用临时会话模式，默认为false
    pub is_temp: Option<bool>,
    /// 临时会话二维码有效期，以秒为单位。该参数仅在is_temp为true时有效，默认7天
    pub expires_in: Option<i64>,
    /// 临时会话有效期，以秒为单位。该参数仅在is_temp为true时有效，默认为添加好友后24小时
    pub chat_expires_in: Option<i64>,
    /// 可进行临时会话的客户unionid，该参数仅在is_temp为true时有效，如不指定则不进行限制
    pub unionid: Option<String>,
    /// 结束语，会话结束时自动发送给客户，可参考“结束语定义”，仅在is_temp为true时有效
    pub conclusions: Option<Conclusion>,
}

/// 结束语定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conclusion {
    pub text_content: Option<String>,
    pub img_media_id: Option<String>,
    pub img_pic_url: Option<String>,
    pub link_title: Option<String>,
    pub link_pic_url: Option<String>,
    pub link_desc: Option<String>,
    pub link_url: Option<String>,
    pub mini_program_title: Option<String>,
    pub mini_program_pic_media_id: Option<String>,
    pub mini_program_app_id: Option<String>,
    pub mini_program_page: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpContactWayInfoResponse {
    pub config_id: Option<String>,
    pub qr_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpExternalContactInfoResponse {
    pub external_contact: Option<ExternalContact>,
    pub follow_user: Option<Vec<FollowedUser>>,
    pub next_cursor: Option<String>,
}

/// 外部联系人
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContact {
    pub external_userid: Option<String>,
    pub position: Option<String>,
    pub name: Option<String>,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub corp_name: Option<String>,
    pub corp_full_name: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<u8>,
    pub gender: Option<u8>,
    pub unionid: Option<String>,
    pub external_profile: Option<ExternalProfile>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalProfile {
    pub external_corp_name: Option<String>,
    pub external_attr: Option<ExternalAttribute>,
    pub wechat_channels: Option<WechatChannel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatChannel {
    pub nickname: Option<String>,
    pub status: Option<u8>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalAttribute {
    #[serde(rename = "type")]
    pub r#type: Option<u8>,
    pub name: Option<String>,
    pub text: Option<Text>,
    pub web: Option<Web>,
    pub miniprogram: Option<MiniProgram>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Web {
    pub title: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgram {
    pub pagepath: Option<String>,
    pub appid: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowedUserTag {
    /// 该成员添加此外部联系人所打标签的分组名称（标签功能需要企业微信升级到2.7.5及以上版本）
    pub group_name: Option<String>,
    /// 该成员添加此外部联系人所打标签名称
    pub tag_name: Option<String>,
    /// 该成员添加此外部联系人所打企业标签的id，仅企业设置（type为1）的标签返回
    pub tag_id: Option<String>,
    /// 该成员添加此外部联系人所打标签类型, 1-企业设置, 2-用户自定义
    #[serde(rename = "type")]
    pub r#type: Option<u8>,
}

/// 添加了外部联系人的企业成员
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowedUser {
    pub userid: Option<String>,
    pub remark: Option<String>,
    pub description: Option<String>,
    pub state: Option<String>,
    pub remark_company: Option<String>,
    pub remark_corp_name: Option<String>,
    pub add_way: Option<u8>,
    pub oper_userid: Option<String>,
    /// 获取客户详情  接口专用
    pub tags: Option<Vec<FollowedUserTag>>,
    pub remark_mobiles: Option<Vec<String>>,
    /// 批量获取客户详情 接口专用
    pub tag_id: Option<Vec<String>>,
    pub createtime: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpGroupJoinWayInfo {
    pub join_way: JoinWay,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinWay {
    /// 联系方式的配置id
    pub config_id: Option<String>,
    /// 场景。
    /// 1 - 群的小程序插件
    /// 2 - 群的二维码插件
    pub scene: Option<u8>,
    /// 联系方式的备注信息，用于助记，超过30个字符将被截断
    pub remark: Option<String>,
    /// 当群满了后，是否自动新建群。0-否；1-是。 默认为1
    pub auto_create_room: Option<u8>,
    /// 自动建群的群名前缀，当auto_create_room为1时有效。最长40个utf8字符
    pub room_base_name: Option<String>,
    /// 自动建群的群起始序号，当auto_create_room为1时有效
    pub room_base_id: Option<u64>,
    /// 使用该配置的客户群ID列表，支持5个。
    pub chat_id_list: Option<Vec<String>>,
    /// 联系二维码的URL，仅在配置为群二维码时返回
    pub qr_code: Option<Vec<String>>,
    /// 企业自定义的state参数，用于区分不同的入群渠道。不超过30个UTF-8字符
    /// 如果有设置此参数，在调用获取客户群详情接口时会返回每个群成员对应的该参数值
    pub state: Option<Vec<String>>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpGroupJoinWayResponse {
    pub config_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpExternalContactBatchInfoResponse {
    pub external_contact_list: Option<ExternalContactInfo>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactInfo {
    pub external_contact: Option<ExternalContact>,
    pub follow_info: Option<FollowedUser>,
}


/// 修改客户备注信息请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpUpdateRemarkRequest {
    ///  <pre>
    /// 字段名：userid
    /// 是否必须：是
    /// 描述：企业成员的userid
    /// </pre>
    pub userid: String,
    /// <pre>
    /// 字段名：external_userid
    /// 是否必须：是
    /// 描述：外部联系人userid
    /// </pre>
    pub external_userid: String,
    /// <pre>
    /// 字段名：remark
    /// 是否必须：否
    /// 描述：此用户对外部联系人的备注，最多20个字符
    /// </pre>
    pub remark: Option<String>,
    /// <pre>
    /// 字段名：description
    /// 是否必须：否
    /// 描述：此用户对外部联系人的描述，最多150个字符
    /// </pre>
    pub description: Option<String>,
    /// <pre>
    /// 字段名：remark_company
    /// 是否必须：否
    /// 描述：此用户对外部联系人备注的所属公司名称，最多20个字符
    /// </pre>
    pub remark_company: Option<String>,
    /// <pre>
    /// 字段名：remark_mobiles
    /// 是否必须：否
    /// 描述：此用户对外部联系人备注的手机号
    /// </pre>
    pub remark_mobiles: Option<Vec<String>>,
    /// <pre>
    /// 字段名：remark_pic_mediaid
    /// 是否必须：否
    /// 描述：备注图片的mediaid，
    /// </pre>
    pub remark_pic_mediaid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpUserExternalUnassignList {
    pub info: Option<Vec<ExternalContact>>,
    pub is_last: Option<bool>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnassignInfo {
    /// 离职成员userid
    pub handover_userid: Option<String>,
    /// 外部联系人userid
    pub external_userid: Option<String>,
    /// 成员离职时间
    pub dimission_time: Option<u64>,
}

/// /// 转接在职成员的客户给其他成员
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpUserTransferCustomerRequest {
    /// 原跟进成员的userid
    pub handover_userid: String,
    /// 接替成员的userid
    pub takeover_userid: String,
    /// 转移成功后发给客户的消息，最多200个字符，不填则使用默认文案
    pub transfer_success_msg: String,
    /// 客户的external_userid列表，每次最多分配100个客户
    pub external_userid: Vec<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpUserTransferCustomerResponse {
    /// 客户转移结果列表
    pub customer: Vec<TransferCustomer>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferCustomer {
    /// 对此客户进行分配的结果, 0表示成功发起接替,待24小时后自动接替,并不代表最终接替成功
    pub errcode: u8,
    /// 客户的external_userid
    pub external_userid: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpUserTransferResultResponse {
    /// 客户转移结果列表
    pub customer: Vec<TransferResult>,
    pub next_cursor: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferResult {
    /// 接替状态， 1-接替完毕 2-等待接替 3-客户拒绝 4-接替成员客户达到上限 5-无接替记录
    pub status: u8,
    /// 客户的external_userid
    pub external_userid: Option<String>,
    /// 接替客户的时间，如果是等待接替状态，则为未来的自动接替时间
    pub takeover_time: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpUserExternalGroupChatList {
    pub group_chat_list: Vec<ChatStatus>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatStatus {
    pub chat_id: Option<String>,
    /// 客户群状态
    /// 0 - 正常
    /// 1 - 跟进人离职
    /// 2 - 离职继承中
    /// 3 - 离职继承完成
    pub status: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpUserExternalGroupChatInfoResponse {
    pub group_chat: GroupChat,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupChat {
    pub chat_id: Option<String>,
    pub name: Option<String>,
    pub owner: Option<String>,
    pub notice: Option<String>,
    pub create_time: Option<u64>,
    pub member_list: Option<Vec<GroupMember>>,
    pub admin_list: Option<Vec<GroupAdmin>>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMember {
    pub userid: Option<String>,
    /// 成员类型。
    /// 1 - 企业成员
    /// 2 - 外部联系人
    #[serde(rename = "type")]
    pub r#type: Option<u8>,
    /// 入群方式。
    /// 1 - 由成员邀请入群（直接邀请入群）
    /// 2 - 由成员邀请入群（通过邀请链接入群）
    /// 3 - 通过扫描群二维码入群
    pub join_scene: Option<u8>,
    /// 外部联系人在微信开放平台的唯一身份标识（微信unionid）
    /// 通过此字段企业可将外部联系人与公众号/小程序用户关联起来
    /// 仅当群成员类型是微信用户（包括企业成员未添加好友），且企业或第三方服务商绑定了微信开发者ID有此字段
    pub unionid: Option<String>,
    /// 该成员入群方式对应的state参数
    pub state: Option<String>,
    /// 在群里的昵称
    pub group_nickname: Option<String>,
    /// 名字。仅当 need_name = 1 时返回
    /// 如果是微信用户，则返回其在微信中设置的名字
    /// 如果是企业微信联系人，则返回其设置对外展示的别名或实名
    pub name: Option<String>,
    pub join_time: Option<u64>,
    pub invitor: Option<Invitor>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupAdmin {
    /// 群管理员userid
    pub userid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invitor {
    /// 邀请者的userid
    pub userid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpUserExternalGroupChatTransferResponse {
    /// 没有成功继承的群列表
    pub failed_chat_list: Vec<GroupChatFailedTransfer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupChatFailedTransfer {
    pub chat_id: Option<String>,
}

/// 联系客户统计数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpUserExternalUserBehaviorStatistic {
    pub behavior_data: Vec<Behavior>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Behavior {
    /// 数据日期，为当日0点的时间戳
    pub stat_time: Option<u64>,
    /// 聊天总数， 成员有主动发送过消息的聊天数，包括单聊和群聊。
    pub chat_cnt: Option<u64>,
    /// 发送消息数，成员在单聊和群聊中发送的消息总数。
    pub message_cnt: Option<u64>,
    /// 平均首次回复时长，单位为分钟，即客户主动发起聊天后，成员在一个自然日内首次回复的时长间隔为首次回复时长，所有聊天的首次回复总时长/已回复的聊天总数即为平均首次回复时长，不包括群聊，仅在确有回复时返回。
    pub avg_reply_time: Option<u64>,
    /// 删除/拉黑成员的客户数，即将成员删除或加入黑名单的客户数。
    pub negative_feedback_cnt: Option<u64>,
    /// 发起申请数，成员通过「搜索手机号」、「扫一扫」、「从微信好友中添加」、「从群聊中添加」、「添加共享、分配给我的客户」、「添加单向、双向删除好友关系的好友」、「从新的联系人推荐中添加」等渠道主动向客户发起的好友申请数量。
    pub new_apply_cnt: Option<u64>,
    /// 新增客户数，成员新添加的客户数量。
    pub new_contact_cnt: Option<u64>,
    /// 已回复聊天占比，客户主动发起聊天后，成员在一个自然日内有回复过消息的聊天数/客户主动发起的聊天数比例，不包括群聊，仅在确有回复时返回。
    pub reply_percentage: Option<f64>,
}


/// 联系客户群统计数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpUserExternalGroupChatStatistic {
    pub total: Vec<u64>,
    pub next_offset: Vec<u64>,
    pub items: Vec<StatisticItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticItem {
    pub owner: Option<String>,
    pub data: Option<ItemData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemData {
    /// 新增客户群数量
    pub new_chat_cnt: Option<u64>,
    /// 截至当天客户群总数量
    pub chat_total: Option<u64>,
    /// 截至当天有发过消息的客户群数量
    pub chat_has_msg: Option<u64>,
    /// 客户群新增群人数
    pub new_member_cnt: Option<u64>,
    /// 截至当天客户群总人数
    pub member_total: Option<u64>,
    /// 截至当天有发过消息的群成员数
    pub member_has_msg: Option<u64>,
    /// 截至当天客户群消息总数
    pub msg_total: Option<u64>,
}

/// 企业群发消息任务
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpMsgTemplate {
    /// 群发任务的类型，默认为single，表示发送给客户，group表示发送给客户群
    pub chat_type: Option<String>,
    /// 客户的外部联系人id列表，仅在chat_type为single时有效，不可与sender同时为空，最多可传入1万个客户
    pub external_userid: Option<Vec<String>>,
    /// 发送企业群发消息的成员userid，当类型为发送给客户群时必填
    pub sender: Option<String>,
    /// 消息文本内容，最多4000个字节
    pub text: Option<WechatCpTextMsg>,
    /// 附件，最多支持添加9个附件
    pub attachments: Option<Vec<WechatCpAttachment>>,
}

/// 消息文本消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpTextMsg {
    pub content: String,
}

/// 图片消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpImageMsg {
    pub media_id: String,
    pub pic_url: String,
}


/// 图文消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpLinkMsg {
    pub title: String,
    pub picurl: String,
    pub desc: String,
    pub url: String,
    pub media_id: String,
}

/// 小程序消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpMiniProgramMsg {
    pub title: String,
    pub pic_media_id: String,
    pub appid: String,
    pub page: String,
}

/// 视频消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpVideoMsg {
    pub media_id: String,
    pub thumb_media_id: String,
}

/// 文件消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpFileMsg {
    pub media_id: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpAttachment {
    pub msgtype: String,
    pub image: Option<WechatCpImageMsg>,
    pub link: Option<WechatCpLinkMsg>,
    pub miniprogram: Option<WechatCpMiniProgramMsg>,
    pub video: Option<WechatCpVideoMsg>,
    pub file: Option<WechatCpFileMsg>,
}

impl WechatCpAttachment {
    pub fn new() -> Self {
        Self {
            msgtype: "".to_string(),
            image: None,
            link: None,
            miniprogram: None,
            video: None,
            file: None,
        }
    }

    pub fn image(mut self, image: WechatCpImageMsg) -> Self {
        self.image = image.into();
        self.msgtype = WELCOME_MSG_TYPE_IMAGE.to_string();
        self
    }

    pub fn link(mut self, link: WechatCpLinkMsg) -> Self {
        self.link = link.into();
        self.msgtype = WELCOME_MSG_TYPE_LINK.to_string();
        self
    }

    pub fn video(mut self, video: WechatCpVideoMsg) -> Self {
        self.video = video.into();
        self.msgtype = WELCOME_MSG_TYPE_VIDEO.to_string();
        self
    }

    pub fn file(mut self, file: WechatCpFileMsg) -> Self {
        self.file = file.into();
        self.msgtype = WELCOME_MSG_TYPE_FILE.to_string();
        self
    }

    pub fn miniprogram(mut self, miniprogram: WechatCpMiniProgramMsg) -> Self {
        self.miniprogram = miniprogram.into();
        self.msgtype = WELCOME_MSG_TYPE_MINIPROGRAM.to_string();
        self
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpMsgTemplateAddResponse {
    pub fail_list: Option<Vec<String>>,
    pub msgid: Option<String>,
}

/// 新客户欢迎语
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpWelcomeMsg {
    pub welcome_code: String,
    /// 消息文本内容，最多4000个字节
    pub text: Option<WechatCpTextMsg>,
    /// 附件，最多支持添加9个附件
    pub attachments: Option<Vec<WechatCpAttachment>>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpUserExternalTagGroupInfo {
    pub tag_group: Option<TagGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagGroup {
    pub group_id: Option<String>,
    pub group_name: Option<String>,
    pub create_time: Option<u64>,
    pub order: Option<u64>,
    pub deleted: Option<bool>,
    pub tag: Option<Vec<CustomerTag>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerTag {
    /// 客户群ID
    pub id: Option<String>,
    pub name: Option<String>,
    pub create_time: Option<u64>,
    pub order: Option<u64>,
    pub deleted: Option<bool>,
}

/// 获取企业群发成员执行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpGroupMsgListResult {
    pub group_msg_list: Vec<ExternalContactGroupMsgInfo>,
    pub next_cursor: Option<String>,
}

/// 获取企业群发成员执行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpGroupMsgSendResult {
    pub send_list: Vec<ExternalContactGroupMsgSendInfo>,
    pub next_cursor: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactGroupMsgInfo {
    pub msgid: Option<String>,
    pub creator: Option<String>,
    pub create_type: Option<u8>,
    pub create_time: Option<u64>,
    pub text: Option<WechatCpTextMsg>,
    pub attachments: Option<Vec<WechatCpAttachment>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactGroupMsgSendInfo {
    /// 外部联系人userid，群发消息到企业的客户群不吐出该字段
    pub external_userid: Option<String>,
    /// 外部客户群id，群发消息到客户不吐出该字段
    pub chat_id: Option<String>,
    /// 企业服务人员的userid
    pub userid: Option<String>,
    /// 发送状态 0-未发送 1-已发送 2-因客户不是好友导致发送失败 3-因客户已经收到其他群发消息导致发送失败
    pub status: Option<u8>,
    /// 发送时间，发送状态为1时返回
    pub send_time: Option<u64>,
}


/// 获取企业群发成员执行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpGroupMsgResult {
    pub detail_list: Vec<ExternalContactGroupMsgDetailInfo>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactGroupMsgDetailInfo {
    /// 外部联系人userid，群发消息到企业的客户群不吐出该字段
    pub external_userid: Option<String>,
    /// 外部客户群id，群发消息到客户不吐出该字段
    pub chat_id: Option<String>,
    /// 企业服务人员的userid
    pub userid: Option<String>,
    /// 发送状态 0-未发送 1-已发送 2-因客户不是好友导致发送失败 3-因客户已经收到其他群发消息导致发送失败
    pub status: Option<u8>,
    /// 发送时间，发送状态为1时返回
    pub send_time: Option<u64>,
}


/// 获取群发成员发送任务列表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpGroupMsgTaskResult {
    pub task_list: Vec<ExternalContactGroupMsgTaskInfo>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactGroupMsgTaskInfo {
    /// 企业服务人员的userid
    pub userid: Option<String>,
    pub status: Option<u8>,
    /// 发送时间，发送状态为1时返回
    pub send_time: Option<u64>,
}

/// 入群欢迎语素材
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpGroupWelcomeTemplateInfo {
    /// 企业服务人员的userid
    pub text: Option<WechatCpTextMsg>,
    pub image: Option<WechatCpImageMsg>,
    pub link: Option<WechatCpLinkMsg>,
    pub miniprogram: Option<WechatCpMiniProgramMsg>,
    pub file: Option<WechatCpFileMsg>,
    pub video: Option<WechatCpVideoMsg>,
    /// 欢迎语素材id
    pub template_id: Option<String>,
    /// 是否通知成员将这条入群欢迎语应用到客户群中，0-不通知，1-通知， 不填则通知
    pub notify: Option<u8>,
}
