/*
 *
 *  *
 *  *      Copyright (c) 2018-2025, SnackCloud All rights reserved.
 *  *
 *  *   Redistribution and use in source and binary forms, with or without
 *  *   modification, are permitted provided that the following conditions are met:
 *  *
 *  *   Redistributions of source code must retain the above copyright notice,
 *  *   this list of conditions and the following disclaimer.
 *  *   Redistributions in binary form must reproduce the above copyright
 *  *   notice, this list of conditions and the following disclaimer in the
 *  *   documentation and/or other materials provided with the distribution.
 *  *   Neither the name of the www.snackcloud.cn developer nor the names of its
 *  *   contributors may be used to endorse or promote products derived from
 *  *   this software without specific prior written permission.
 *  *   Author: SnackCloud
 *  *
 *
 */
use serde::{Deserialize, Serialize};
use serde_json::{json};

use crate::errors::{LabraError, LabradorResult};
use crate::wechat::client::WechatApiResponse;
use crate::wechat::cp::types::{WELCOME_MSG_TYPE_FILE, WELCOME_MSG_TYPE_IMAGE, WELCOME_MSG_TYPE_LINK, WELCOME_MSG_TYPE_MINIPROGRAM, WELCOME_MSG_TYPE_VIDEO};
use crate::wechat::cp::WechatCpClient;

/// 企业微信外部联系人管理模块
#[derive(Debug, Clone)]
pub struct WechatCpExternalContact<'a> {
    client: &'a WechatCpClient,
}

#[allow(unused)]
impl<'a> WechatCpExternalContact<'a> {
    #[inline]
    pub fn new(client: &'a WechatCpClient) -> Self {
        Self { client }
    }

    // ==================== 联系我方式 ====================

    /// 配置客户联系「联系我」方式
    ///
    /// 企业可通过此接口为具有客户联系功能的成员生成专属的「联系我」二维码或者「联系我」按钮。
    ///
    /// 注意:
    /// - 通过API添加的「联系我」不会在管理端进行展示，每个企业可通过API最多配置50万个「联系我」
    /// - 临时会话模式不占用「联系我」数量，但每日最多添加10万个，并且仅支持单人
    pub async fn add_contact_way(&self, contact_way: ContactWay) -> LabradorResult<ContactWayResponse> {
        // 校验用户数量
        if let Some(users) = &contact_way.user {
            if users.len() > 100 {
                return Err(LabraError::RequestError("「联系我」使用人数默认限制不超过100人".to_string()));
            }
        }
        let response: WechatApiResponse<ContactWayResponse> = self.client
            .post("/cgi-bin/externalcontact/add_contact_way", contact_way)
            .await?;
        response.into_result()
    }

    /// 获取企业已配置的「联系我」方式
    pub async fn get_contact_way(&self, config_id: &str) -> LabradorResult<ContactWay> {
        let response: WechatApiResponse<ContactWay> = self.client
            .post("/cgi-bin/externalcontact/get_contact_way", json!({ "config_id": config_id }))
            .await?;
        response.into_result()
    }

    /// 更新企业已配置的「联系我」方式
    pub async fn update_contact_way(&self, contact_way: ContactWay) -> LabradorResult<WechatApiResponse> {
        if contact_way.config_id.is_none() {
            return Err(LabraError::RequestError("更新「联系我」方式需要指定config_id".to_string()));
        }
        if let Some(users) = &contact_way.user {
            if users.len() > 100 {
                return Err(LabraError::RequestError("「联系我」使用人数默认限制不超过100人".to_string()));
            }
        }
        let response: WechatApiResponse = self.client
            .post("/cgi-bin/externalcontact/update_contact_way", contact_way)
            .await?;
        Ok(response)
    }

    /// 删除企业已配置的「联系我」方式
    pub async fn delete_contact_way(&self, config_id: &str) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self.client
            .post("/cgi-bin/externalcontact/del_contact_way", json!({ "config_id": config_id }))
            .await?;
        Ok(response)
    }

    /// 结束临时会话
    ///
    /// 将指定的企业成员和客户之前的临时会话断开，断开前会自动下发已配置的结束语。
    pub async fn close_temp_chat(&self, user_id: &str, external_user_id: &str) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self.client
            .post("/cgi-bin/externalcontact/close_temp_chat", json!({
                "userid": user_id,
                "external_userid": external_user_id
            }))
            .await?;
        Ok(response)
    }

    // ==================== 客户管理 ====================

    /// 获取客户列表
    ///
    /// 企业可通过此接口获取指定成员添加的客户列表。
    pub async fn list_external_contacts(&self, userid: &str) -> LabradorResult<Vec<String>> {
        let response: WechatApiResponse<ExternalContactListResponse> = self.client
            .get(&format!("/cgi-bin/externalcontact/list?userid={}", userid))
            .await?;
        Ok(response.into_result()?.external_userid)
    }

    /// 获取客户详情
    ///
    /// 企业可通过此接口，根据外部联系人的userid，拉取客户详情。
    pub async fn get_contact_detail(&self, external_userid: &str, cursor: Option<&str>) -> LabradorResult<CpExternalContactDetail> {
        let mut url = format!("/cgi-bin/externalcontact/get?external_userid={}", external_userid);
        if let Some(c) = cursor {
            url.push_str(&format!("&cursor={}", c));
        }
        let response: WechatApiResponse<CpExternalContactDetail> = self.client
            .get(&url)
            .await?;
        response.into_result()
    }

    /// 批量获取客户详情
    ///
    /// 企业/第三方可通过此接口获取指定成员添加的客户信息列表。
    pub async fn batch_get_contact_detail(
        &self,
        userid_list: Vec<String>,
        cursor: Option<&str>,
        limit: Option<i32>,
    ) -> LabradorResult<BatchExternalContactDetail> {
        let mut req = json!({ "userid_list": userid_list });
        if let Some(c) = cursor {
            req["cursor"] = json!(c);
        }
        if let Some(l) = limit {
            req["limit"] = json!(l);
        }
        let response: WechatApiResponse<BatchExternalContactDetail> = self.client
            .post("/cgi-bin/externalcontact/batch/get_by_user", req)
            .await?;
        response.into_result()
    }

    /// 修改客户备注信息
    pub async fn update_remark(&self, req: UpdateRemarkRequest) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self.client
            .post("/cgi-bin/externalcontact/remark", req)
            .await?;
        Ok(response)
    }

    /// 获取配置了客户联系功能的成员列表
    pub async fn list_followers(&self) -> LabradorResult<Vec<String>> {
        let response: WechatApiResponse<FollowUserListResponse> = self.client
            .get("/cgi-bin/externalcontact/get_follow_user_list")
            .await?;
        Ok(response.into_result()?.follow_user)
    }

    // ==================== 离职继承 ====================

    /// 获取待分配的离职成员列表
    pub async fn list_unassigned(&self, cursor: Option<&str>, page_size: Option<u64>) -> LabradorResult<UnassignedListResponse> {
        let mut req = json!({ "page_size": page_size.unwrap_or(1000) });
        if let Some(c) = cursor {
            req["cursor"] = json!(c);
        }
        let response: WechatApiResponse<UnassignedListResponse> = self.client
            .post("/cgi-bin/externalcontact/get_unassigned_list", req)
            .await?;
        response.into_result()
    }

    /// 分配离职成员的客户
    pub async fn transfer_resigned_customer(&self, req: TransferCustomerRequest) -> LabradorResult<TransferCustomerResponse> {
        let response: WechatApiResponse<TransferCustomerResponse> = self.client
            .post("/cgi-bin/externalcontact/resigned/transfer_customer", req)
            .await?;
        response.into_result()
    }

    /// 查询离职成员的客户分配情况
    pub async fn get_resigned_transfer_result(
        &self,
        handover_userid: &str,
        takeover_userid: &str,
        cursor: Option<&str>,
    ) -> LabradorResult<TransferResultResponse> {
        let mut req = json!({
            "handover_userid": handover_userid,
            "takeover_userid": takeover_userid,
        });
        if let Some(c) = cursor {
            req["cursor"] = json!(c);
        }
        let response: WechatApiResponse<TransferResultResponse> = self.client
            .post("/cgi-bin/externalcontact/resigned/transfer_result", req)
            .await?;
        response.into_result()
    }

    // ==================== 在职继承 ====================

    /// 转接在职成员的客户
    pub async fn transfer_customer(&self, req: TransferCustomerRequest) -> LabradorResult<TransferCustomerResponse> {
        let response: WechatApiResponse<TransferCustomerResponse> = self.client
            .post("/cgi-bin/externalcontact/transfer_customer", req)
            .await?;
        response.into_result()
    }

    /// 查询在职成员的客户转接情况
    pub async fn get_transfer_result(
        &self,
        handover_userid: &str,
        takeover_userid: &str,
        cursor: Option<&str>,
    ) -> LabradorResult<TransferResultResponse> {
        let mut req = json!({
            "handover_userid": handover_userid,
            "takeover_userid": takeover_userid,
        });
        if let Some(c) = cursor {
            req["cursor"] = json!(c);
        }
        let response: WechatApiResponse<TransferResultResponse> = self.client
            .post("/cgi-bin/externalcontact/transfer_result", req)
            .await?;
        response.into_result()
    }

    // ==================== 客户群管理 ====================

    /// 获取客户群列表
    pub async fn list_group_chat(
        &self,
        status_filter: Option<u8>,
        owner_filter: Option<OwnerFilter>,
        cursor: Option<&str>,
        limit: Option<u64>,
    ) -> LabradorResult<GroupChatListResponse> {
        let mut req = json!({ "limit": limit.unwrap_or(100) });
        if let Some(s) = status_filter {
            req["status_filter"] = json!(s);
        }
        if let Some(o) = owner_filter {
            req["owner_filter"] = json!(o);
        }
        if let Some(c) = cursor {
            req["cursor"] = json!(c);
        }
        let response: WechatApiResponse<GroupChatListResponse> = self.client
            .post("/cgi-bin/externalcontact/groupchat/list", req)
            .await?;
        response.into_result()
    }

    /// 获取客户群详情
    pub async fn get_group_chat(&self, chat_id: &str, need_name: Option<u8>) -> LabradorResult<GroupChatInfo> {
        let req = json!({
            "chat_id": chat_id,
            "need_name": need_name.unwrap_or(0),
        });
        let response: WechatApiResponse<GroupChatInfo> = self.client
            .post("/cgi-bin/externalcontact/groupchat/get", req)
            .await?;
        response.into_result()
    }

    /// 分配离职成员的客户群
    pub async fn transfer_group_chat(&self, chat_id_list: Vec<String>, new_owner: &str) -> LabradorResult<GroupChatTransferResponse> {
        let req = json!({
            "chat_id_list": chat_id_list,
            "new_owner": new_owner,
        });
        let response: WechatApiResponse<GroupChatTransferResponse> = self.client
            .post("/cgi-bin/externalcontact/groupchat/transfer", req)
            .await?;
        response.into_result()
    }

    // ==================== 消息群发 ====================

    /// 添加企业群发消息任务
    pub async fn add_msg_template(&self, msg_template: MsgTemplate) -> LabradorResult<MsgTemplateAddResponse> {
        let response: WechatApiResponse<MsgTemplateAddResponse> = self.client
            .post("/cgi-bin/externalcontact/add_msg_template", msg_template)
            .await?;
        response.into_result()
    }

    /// 获取群发记录列表
    pub async fn get_group_msg_list(
        &self,
        chat_type: &str,
        start_time: i64,
        end_time: i64,
        creator: &str,
        filter_type: u8,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> LabradorResult<GroupMsgListResponse> {
        let mut req = json!({
            "chat_type": chat_type,
            "start_time": start_time,
            "end_time": end_time,
            "creator": creator,
            "filter_type": filter_type,
            "limit": limit.unwrap_or(100),
        });
        if let Some(c) = cursor {
            req["cursor"] = json!(c);
        }
        let response: WechatApiResponse<GroupMsgListResponse> = self.client
            .post("/cgi-bin/externalcontact/get_group_msg_list_v2", req)
            .await?;
        response.into_result()
    }

    /// 获取群发成员发送任务列表
    pub async fn get_group_msg_task(&self, msgid: &str, limit: Option<i32>, cursor: Option<&str>) -> LabradorResult<GroupMsgTaskResponse> {
        let mut req = json!({ "msgid": msgid, "limit": limit.unwrap_or(100) });
        if let Some(c) = cursor {
            req["cursor"] = json!(c);
        }
        let response: WechatApiResponse<GroupMsgTaskResponse> = self.client
            .post("/cgi-bin/externalcontact/get_group_msg_task", req)
            .await?;
        response.into_result()
    }

    /// 获取企业群发成员执行结果
    pub async fn get_group_msg_send_result(
        &self,
        msgid: &str,
        userid: &str,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> LabradorResult<GroupMsgSendResultResponse> {
        let mut req = json!({
            "msgid": msgid,
            "userid": userid,
            "limit": limit.unwrap_or(100),
        });
        if let Some(c) = cursor {
            req["cursor"] = json!(c);
        }
        let response: WechatApiResponse<GroupMsgSendResultResponse> = self.client
            .post("/cgi-bin/externalcontact/get_group_msg_send_result", req)
            .await?;
        response.into_result()
    }

    /// 发送新客户欢迎语
    ///
    /// 企业微信在向企业推送添加外部联系人事件时，会额外返回一个welcome_code，
    /// 企业以此为凭据调用接口，即可通过成员向新添加的客户发送个性化的欢迎语。
    pub async fn send_welcome_msg(&self, welcome_msg: WelcomeMsg) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self.client
            .post("/cgi-bin/externalcontact/send_welcome_msg", welcome_msg)
            .await?;
        Ok(response)
    }

    // ==================== 入群欢迎语素材 ====================

    /// 添加入群欢迎语素材
    pub async fn add_group_welcome_template(&self, template: GroupWelcomeTemplate) -> LabradorResult<String> {
        let response: WechatApiResponse<AddGroupWelcomeTemplateResponse> = self.client
            .post("/cgi-bin/externalcontact/group_welcome_template/add", template)
            .await?;
        Ok(response.into_result()?.template_id)
    }

    /// 编辑入群欢迎语素材
    pub async fn edit_group_welcome_template(&self, template: GroupWelcomeTemplate) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self.client
            .post("/cgi-bin/externalcontact/group_welcome_template/edit", template)
            .await?;
        Ok(response)
    }

    /// 获取入群欢迎语素材
    pub async fn get_group_welcome_template(&self, template_id: &str) -> LabradorResult<GroupWelcomeTemplate> {
        let response: WechatApiResponse<GroupWelcomeTemplate> = self.client
            .post("/cgi-bin/externalcontact/group_welcome_template/get", json!({ "template_id": template_id }))
            .await?;
        response.into_result()
    }

    /// 删除入群欢迎语素材
    pub async fn delete_group_welcome_template(&self, template_id: &str, agent_id: Option<&str>) -> LabradorResult<WechatApiResponse> {
        let mut req = json!({ "template_id": template_id });
        if let Some(a) = agent_id {
            req["agentid"] = json!(a);
        }
        let response: WechatApiResponse = self.client
            .post("/cgi-bin/externalcontact/group_welcome_template/del", req)
            .await?;
        Ok(response)
    }

    // ==================== 客户标签管理 ====================

    /// 获取企业客户标签
    pub async fn get_corp_tag_list(&self, tag_id: Option<Vec<String>>, group_id: Option<Vec<String>>) -> LabradorResult<TagGroupList> {
        let mut req = json!({});
        if let Some(t) = tag_id {
            req["tag_id"] = json!(t);
        }
        if let Some(g) = group_id {
            req["group_id"] = json!(g);
        }
        let response: WechatApiResponse<TagGroupList> = self.client
            .post("/cgi-bin/externalcontact/get_corp_tag_list", req)
            .await?;
        response.into_result()
    }

    /// 添加企业客户标签
    pub async fn add_corp_tag(&self, tag_group: TagGroup) -> LabradorResult<TagGroup> {
        let response: WechatApiResponse<TagGroup> = self.client
            .post("/cgi-bin/externalcontact/add_corp_tag", tag_group)
            .await?;
        response.into_result()
    }

    /// 编辑企业客户标签
    pub async fn edit_corp_tag(&self, id: &str, name: &str, order: Option<u64>) -> LabradorResult<WechatApiResponse> {
        let mut req = json!({ "id": id, "name": name });
        if let Some(o) = order {
            req["order"] = json!(o);
        }
        let response: WechatApiResponse = self.client
            .post("/cgi-bin/externalcontact/edit_corp_tag", req)
            .await?;
        Ok(response)
    }

    /// 删除企业客户标签
    pub async fn delete_corp_tag(&self, tag_id: Option<Vec<String>>, group_id: Option<Vec<String>>) -> LabradorResult<WechatApiResponse> {
        let mut req = json!({});
        if let Some(t) = tag_id {
            req["tag_id"] = json!(t);
        }
        if let Some(g) = group_id {
            req["group_id"] = json!(g);
        }
        let response: WechatApiResponse = self.client
            .post("/cgi-bin/externalcontact/del_corp_tag", req)
            .await?;
        Ok(response)
    }

    /// 标记客户企业标签
    pub async fn mark_tag(
        &self,
        userid: &str,
        external_userid: &str,
        add_tag: Option<Vec<String>>,
        remove_tag: Option<Vec<String>>,
    ) -> LabradorResult<WechatApiResponse> {
        let mut req = json!({
            "userid": userid,
            "external_userid": external_userid,
        });
        if let Some(a) = add_tag {
            req["add_tag"] = json!(a);
        }
        if let Some(r) = remove_tag {
            req["remove_tag"] = json!(r);
        }
        let response: WechatApiResponse = self.client
            .post("/cgi-bin/externalcontact/mark_tag", req)
            .await?;
        Ok(response)
    }

    // ==================== 数据统计 ====================

    /// 获取联系客户统计数据
    pub async fn get_user_behavior_statistic(
        &self,
        start_time: i64,
        end_time: i64,
        userid_list: Option<Vec<String>>,
        partyid_list: Option<Vec<i64>>,
    ) -> LabradorResult<Vec<BehaviorData>> {
        let mut req = json!({
            "start_time": start_time,
            "end_time": end_time,
        });
        if let Some(u) = userid_list {
            req["userid"] = json!(u);
        }
        if let Some(p) = partyid_list {
            req["partyid"] = json!(p);
        }
        let response: WechatApiResponse<UserBehaviorStatisticResponse> = self.client
            .post("/cgi-bin/externalcontact/get_user_behavior_data", req)
            .await?;
        Ok(response.into_result()?.behavior_data)
    }

    /// 获取客户群统计数据
    pub async fn get_group_chat_statistic(
        &self,
        day_begin_time: i64,
        owner_filter: Option<OwnerFilter>,
        order_by: Option<u8>,
        order_asc: Option<u8>,
        offset: Option<u64>,
        limit: Option<u64>,
    ) -> LabradorResult<GroupChatStatisticResponse> {
        let mut req = json!({
            "day_begin_time": day_begin_time,
            "order_by": order_by.unwrap_or(1),
            "order_asc": order_asc.unwrap_or(0),
            "offset": offset.unwrap_or(0),
            "limit": limit.unwrap_or(500),
        });
        if let Some(o) = owner_filter {
            req["owner_filter"] = json!(o);
        }
        let response: WechatApiResponse<GroupChatStatisticResponse> = self.client
            .post("/cgi-bin/externalcontact/groupchat/statistic", req)
            .await?;
        response.into_result()
    }

    // ==================== 转换接口 ====================

    /// 将微信外部联系人的userid转为微信openid
    pub async fn convert_to_openid(&self, external_userid: &str) -> LabradorResult<String> {
        let response: WechatApiResponse<ConvertOpenidResponse> = self.client
            .post("/cgi-bin/externalcontact/convert_to_openid", json!({ "external_userid": external_userid }))
            .await?;
        Ok(response.into_result()?.openid)
    }

    /// 将unionid转为external_userid
    pub async fn unionid_to_external_userid(&self, unionid: &str, openid: Option<&str>) -> LabradorResult<String> {
        let mut req = json!({ "unionid": unionid });
        if let Some(o) = openid {
            req["openid"] = json!(o);
        }
        let response: WechatApiResponse<UnionidToExternalUseridResponse> = self.client
            .post("/cgi-bin/externalcontact/unionid_to_external_userid", req)
            .await?;
        Ok(response.into_result()?.external_userid)
    }
}

// ============================================================================
// 请求与响应结构体
// ============================================================================

/// 「联系我」方式
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactWay {
    /// 联系方式的配置id
    pub config_id: Option<String>,
    /// 联系方式类型: 1-单人, 2-多人
    #[serde(rename = "type")]
    pub way_type: u8,
    /// 场景: 1-在小程序中联系, 2-通过二维码联系
    pub scene: u8,
    /// 样式
    pub style: Option<u8>,
    /// 备注信息，不超过30个字符
    pub remark: Option<String>,
    /// 是否无需验证，默认为true
    pub skip_verify: Option<bool>,
    /// 企业自定义的state参数，不超过30个字符
    pub state: Option<String>,
    /// 联系二维码的URL
    pub qr_code: Option<String>,
    /// 使用该联系方式的用户userID列表
    pub user: Option<Vec<String>>,
    /// 使用该联系方式的部门id列表
    pub party: Option<Vec<String>>,
    /// 是否临时会话模式
    pub is_temp: Option<bool>,
    /// 临时会话二维码有效期，单位秒
    pub expires_in: Option<i64>,
    /// 临时会话有效期，单位秒
    pub chat_expires_in: Option<i64>,
    /// 可进行临时会话的客户unionid
    pub unionid: Option<String>,
    /// 结束语
    pub conclusions: Option<Conclusion>,
}

/// 结束语
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Conclusion {
    pub text: Option<TextConclusion>,
    pub image: Option<ImageConclusion>,
    pub link: Option<LinkConclusion>,
    pub miniprogram: Option<MiniProgramConclusion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextConclusion {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageConclusion {
    pub media_id: String,
    pub pic_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkConclusion {
    pub title: String,
    pub pic_url: Option<String>,
    pub desc: Option<String>,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MiniProgramConclusion {
    pub title: String,
    pub pic_media_id: String,
    pub appid: String,
    pub page: String,
}

/// 联系我方式响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactWayResponse {
    pub config_id: String,
    pub qr_code: Option<String>,
}

/// 外部联系人列表响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExternalContactListResponse {
    pub external_userid: Vec<String>,
}

/// 外部联系人详情
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CpExternalContactDetail {
    pub external_contact: ExternalContact,
    pub follow_user: Vec<FollowUser>,
    pub next_cursor: Option<String>,
}

/// 外部联系人
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalContact {
    pub external_userid: String,
    pub name: String,
    pub position: Option<String>,
    pub avatar: Option<String>,
    pub corp_name: Option<String>,
    pub corp_full_name: Option<String>,
    #[serde(rename = "type")]
    pub contact_type: u8,
    pub gender: u8,
    pub unionid: Option<String>,
    pub external_profile: Option<ExternalProfile>,
}

/// 外部资料
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalProfile {
    pub external_corp_name: Option<String>,
    pub wechat_channels: Option<WechatChannel>,
    pub external_attr: Vec<CpExternalAttr>,
}

/// 微信视频号
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WechatChannel {
    pub nickname: String,
    pub status: u8,
}

/// 外部属性
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CpExternalAttr {
    #[serde(rename = "type")]
    pub attr_type: u8,
    pub name: String,
    pub text: Option<AttrText>,
    pub web: Option<AttrWeb>,
    pub miniprogram: Option<AttrMiniProgram>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttrText {
    pub value: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttrWeb {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttrMiniProgram {
    pub title: String,
    pub appid: String,
    pub pagepath: String,
}

/// 跟进人
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowUser {
    pub userid: String,
    pub remark: Option<String>,
    pub description: Option<String>,
    pub state: Option<String>,
    pub remark_company: Option<String>,
    pub remark_corp_name: Option<String>,
    pub add_way: u8,
    pub oper_userid: Option<String>,
    pub tags: Option<Vec<FollowUserTag>>,
    pub remark_mobiles: Option<Vec<String>>,
    pub tag_id: Option<Vec<String>>,
    pub createtime: i64,
}

/// 跟进人标签
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowUserTag {
    pub group_name: Option<String>,
    pub tag_name: String,
    pub tag_id: Option<String>,
    #[serde(rename = "type")]
    pub tag_type: u8,
}

/// 批量外部联系人详情
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchExternalContactDetail {
    pub external_contact_list: Vec<ExternalContactInfo>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalContactInfo {
    pub external_contact: ExternalContact,
    pub follow_info: FollowUser,
}

/// 修改备注请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRemarkRequest {
    pub userid: String,
    pub external_userid: String,
    pub remark: Option<String>,
    pub description: Option<String>,
    pub remark_company: Option<String>,
    pub remark_mobiles: Option<Vec<String>>,
    pub remark_pic_mediaid: Option<String>,
}

/// 跟进人列表响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FollowUserListResponse {
    pub follow_user: Vec<String>,
}

/// 离职未分配列表响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnassignedListResponse {
    pub info: Vec<UnassignedInfo>,
    pub is_last: bool,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnassignedInfo {
    pub handover_userid: String,
    pub external_userid: String,
    pub dimission_time: i64,
}

/// 转移客户请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferCustomerRequest {
    pub handover_userid: String,
    pub takeover_userid: String,
    pub transfer_success_msg: Option<String>,
    pub external_userid: Vec<String>,
}

/// 转移客户响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferCustomerResponse {
    pub customer: Vec<TransferResultItem>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferResultItem {
    pub errcode: i32,
    pub external_userid: String,
}

/// 转移结果查询响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferResultResponse {
    pub customer: Vec<TransferStatus>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferStatus {
    pub status: u8,
    pub external_userid: String,
    pub takeover_time: Option<i64>,
}

/// 群主筛选
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnerFilter {
    pub userid_list: Option<Vec<String>>,
    pub partyid_list: Option<Vec<i64>>,
}

/// 客户群列表响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupChatListResponse {
    pub group_chat_list: Vec<GroupChatItem>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupChatItem {
    pub chat_id: String,
    pub status: u8,
}

/// 客户群详情
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupChatInfo {
    pub group_chat: GroupChat,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupChat {
    pub chat_id: String,
    pub name: String,
    pub owner: String,
    pub notice: Option<String>,
    pub create_time: i64,
    pub member_list: Vec<GroupMember>,
    pub admin_list: Option<Vec<AdminInfo>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMember {
    pub userid: String,
    #[serde(rename = "type")]
    pub member_type: u8,
    pub join_time: i64,
    pub join_scene: u8,
    pub invitor: Option<Invitor>,
    pub unionid: Option<String>,
    pub state: Option<String>,
    pub group_nickname: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invitor {
    pub userid: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminInfo {
    pub userid: String,
}

/// 客户群转移响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupChatTransferResponse {
    pub failed_chat_list: Vec<FailedChat>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FailedChat {
    pub chat_id: String,
}

/// 消息模板
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MsgTemplate {
    pub chat_type: Option<String>,
    pub external_userid: Option<Vec<String>>,
    pub sender: Option<String>,
    pub text: Option<TextMsg>,
    pub attachments: Option<Vec<Attachment>>,
}

/// 文本消息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextMsg {
    pub content: String,
}

/// 附件
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub msgtype: String,
    pub image: Option<ImageMsg>,
    pub link: Option<LinkMsg>,
    pub miniprogram: Option<MiniProgramMsg>,
    pub video: Option<VideoMsg>,
    pub file: Option<FileMsg>,
}

impl Attachment {
    pub fn new_image(image: ImageMsg) -> Self {
        Self {
            msgtype: WELCOME_MSG_TYPE_IMAGE.to_string(),
            image: Some(image),
            link: None,
            miniprogram: None,
            video: None,
            file: None,
        }
    }

    pub fn new_link(link: LinkMsg) -> Self {
        Self {
            msgtype: WELCOME_MSG_TYPE_LINK.to_string(),
            image: None,
            link: Some(link),
            miniprogram: None,
            video: None,
            file: None,
        }
    }

    pub fn new_miniprogram(miniprogram: MiniProgramMsg) -> Self {
        Self {
            msgtype: WELCOME_MSG_TYPE_MINIPROGRAM.to_string(),
            image: None,
            link: None,
            miniprogram: Some(miniprogram),
            video: None,
            file: None,
        }
    }

    pub fn new_video(video: VideoMsg) -> Self {
        Self {
            msgtype: WELCOME_MSG_TYPE_VIDEO.to_string(),
            image: None,
            link: None,
            miniprogram: None,
            video: Some(video),
            file: None,
        }
    }

    pub fn new_file(file: FileMsg) -> Self {
        Self {
            msgtype: WELCOME_MSG_TYPE_FILE.to_string(),
            image: None,
            link: None,
            miniprogram: None,
            video: None,
            file: Some(file),
        }
    }
}

/// 图片消息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageMsg {
    pub media_id: String,
    pub pic_url: Option<String>,
}

/// 图文消息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkMsg {
    pub title: String,
    pub picurl: Option<String>,
    pub desc: Option<String>,
    pub url: String,
    pub media_id: Option<String>,
}

/// 小程序消息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MiniProgramMsg {
    pub title: String,
    pub pic_media_id: String,
    pub appid: String,
    pub page: String,
}

/// 视频消息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoMsg {
    pub media_id: String,
    pub thumb_media_id: Option<String>,
}

/// 文件消息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileMsg {
    pub media_id: String,
}

/// 消息模板添加响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MsgTemplateAddResponse {
    pub fail_list: Option<Vec<String>>,
    pub msgid: String,
}

/// 欢迎语
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeMsg {
    pub welcome_code: String,
    pub text: Option<TextMsg>,
    pub attachments: Option<Vec<Attachment>>,
}

/// 群发记录列表响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMsgListResponse {
    pub group_msg_list: Vec<GroupMsgInfo>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMsgInfo {
    pub msgid: String,
    pub creator: String,
    pub create_type: u8,
    pub create_time: i64,
    pub text: Option<TextMsg>,
    pub attachments: Option<Vec<Attachment>>,
}

/// 群发任务响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMsgTaskResponse {
    pub task_list: Vec<GroupMsgTaskInfo>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMsgTaskInfo {
    pub userid: String,
    pub status: u8,
    pub send_time: Option<i64>,
}

/// 群发发送结果响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMsgSendResultResponse {
    pub send_list: Vec<GroupMsgSendInfo>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMsgSendInfo {
    pub external_userid: Option<String>,
    pub chat_id: Option<String>,
    pub userid: String,
    pub status: u8,
    pub send_time: Option<i64>,
}

/// 入群欢迎语模板
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupWelcomeTemplate {
    pub template_id: Option<String>,
    pub text: Option<TextMsg>,
    pub image: Option<ImageMsg>,
    pub link: Option<LinkMsg>,
    pub miniprogram: Option<MiniProgramMsg>,
    pub file: Option<FileMsg>,
    pub video: Option<VideoMsg>,
    pub notify: Option<u8>,
}

/// 添加入群欢迎语模板响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AddGroupWelcomeTemplateResponse {
    pub template_id: String,
}

/// 标签组列表
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagGroupList {
    pub tag_group: Vec<TagGroup>,
}

/// 标签组
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagGroup {
    pub group_id: Option<String>,
    pub group_name: String,
    pub create_time: Option<i64>,
    pub order: Option<u64>,
    pub deleted: Option<bool>,
    pub tag: Vec<Tag>,
}

/// 标签
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: Option<String>,
    pub name: String,
    pub create_time: Option<i64>,
    pub order: Option<u64>,
    pub deleted: Option<bool>,
}

/// 用户行为统计数据响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserBehaviorStatisticResponse {
    pub behavior_data: Vec<BehaviorData>,
}

/// 行为数据
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BehaviorData {
    pub stat_time: i64,
    pub chat_cnt: u64,
    pub message_cnt: u64,
    pub avg_reply_time: Option<u64>,
    pub negative_feedback_cnt: u64,
    pub new_apply_cnt: u64,
    pub new_contact_cnt: u64,
    pub reply_percentage: Option<f64>,
}

/// 客户群统计数据响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupChatStatisticResponse {
    pub total: u64,
    pub next_offset: u64,
    pub items: Vec<GroupChatStatItem>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupChatStatItem {
    pub owner: String,
    pub data: GroupChatStatData,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupChatStatData {
    pub new_chat_cnt: u64,
    pub chat_total: u64,
    pub chat_has_msg: u64,
    pub new_member_cnt: u64,
    pub member_total: u64,
    pub member_has_msg: u64,
    pub msg_total: u64,
}

/// 转换openid响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConvertOpenidResponse {
    pub openid: String,
}

/// unionid转external_userid响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UnionidToExternalUseridResponse {
    pub external_userid: String,
}