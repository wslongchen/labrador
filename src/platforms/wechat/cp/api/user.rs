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
use serde::{Deserialize};
use serde_json::json;

use crate::errors::LabradorResult;
use crate::wechat::client::WechatApiResponse;
use crate::wechat::cp::api::{ExternalContact, FollowUser};
use crate::wechat::cp::WechatCpClient;
use crate::wechat::mp::api::UserInfo;
// 假设这些类型定义在 types 模块中

/// 企业微信用户管理模块
///
/// 包含成员的创建、更新、删除、查询，以及部门成员列表、邀请成员、ID转换等功能。
#[derive(Debug, Clone)]
pub struct WechatCpUser<'a> {
    client: &'a WechatCpClient,
}

#[allow(unused)]
impl<'a> WechatCpUser<'a> {
    #[inline]
    pub fn new(client: &'a WechatCpClient) -> Self {
        Self { client }
    }

    /// 创建成员
    ///
    /// 详情请见：<https://work.weixin.qq.com/api/doc/90195>
    pub async fn create(&self, user: UserInfo) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self.client
            .post("/cgi-bin/user/create", user)
            .await?;
        Ok(response)
    }

    /// 读取成员
    ///
    /// 详情请见：<https://work.weixin.qq.com/api/doc/90196>
    pub async fn get(&self, userid: &str) -> LabradorResult<UserInfo> {
        let response: WechatApiResponse<UserInfo> = self.client
            .get(&format!("/cgi-bin/user/get?userid={}", userid))
            .await?;
        response.into_result()
    }

    /// 更新成员
    ///
    /// 详情请见：<https://work.weixin.qq.com/api/doc/90197>
    pub async fn update(&self, user: UserInfo) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self.client
            .post("/cgi-bin/user/update", user)
            .await?;
        Ok(response)
    }

    /// 删除成员
    ///
    /// 支持单个删除或批量删除。
    /// 详情请见：<https://work.weixin.qq.com/api/doc/90198>、<https://work.weixin.qq.com/api/doc/90199>
    pub async fn delete(&self, userids: Vec<&str>) -> LabradorResult<WechatApiResponse> {
        if userids.len() == 1 {
            // 单个删除
            let response: WechatApiResponse = self.client
                .get(&format!("/cgi-bin/user/delete?userid={}", userids[0]))
                .await?;
            Ok(response)
        } else {
            // 批量删除
            let response: WechatApiResponse = self.client
                .post("/cgi-bin/user/batchdelete", json!({ "useridlist": userids }))
                .await?;
            Ok(response)
        }
    }

    /// 获取部门成员详情
    ///
    /// 请求方式：GET（HTTPS）
    /// 请求地址：https://qyapi.weixin.qq.com/cgi-bin/user/list?access_token=ACCESS_TOKEN&department_id=DEPARTMENT_ID&fetch_child=FETCH_CHILD
    /// 文档地址：<https://work.weixin.qq.com/api/doc/90201>
    ///
    /// # 参数说明
    /// * `department_id` - 获取的部门id
    /// * `fetch_child` - 是否递归获取子部门下面的成员
    /// * `status` - 0获取全部成员，1获取已关注成员，2获取禁用成员，4获取未关注成员，6获取已关注+禁用成员
    pub async fn list_by_department(
        &self,
        department_id: i64,
        fetch_child: Option<bool>,
        status: Option<i32>,
    ) -> LabradorResult<Vec<UserInfo>> {
        let mut url = format!("/cgi-bin/user/list?department_id={}", department_id);
        if let Some(fetch) = fetch_child {
            url.push_str(&format!("&fetch_child={}", if fetch { "1" } else { "0" }));
        }
        let status_val = status.unwrap_or(0);
        url.push_str(&format!("&status={}", status_val));

        let response: WechatApiResponse<UserListResponse> = self.client
            .get(&url)
            .await?;
        Ok(response.into_result()?.userlist)
    }

    /// 获取部门成员（简略版）
    ///
    /// 只返回成员的UserID、Name、Department等信息。
    /// 文档地址：<https://work.weixin.qq.com/api/doc/90200>
    ///
    /// # 参数说明
    /// * `department_id` - 获取的部门id
    /// * `fetch_child` - 是否递归获取子部门下面的成员
    /// * `status` - 0获取全部成员，1获取已关注成员，2获取禁用成员，4获取未关注成员
    pub async fn list_simple_by_department(
        &self,
        department_id: i64,
        fetch_child: Option<bool>,
        status: Option<i32>,
    ) -> LabradorResult<Vec<SimpleUserInfo>> {
        let mut url = format!("/cgi-bin/user/simplelist?department_id={}", department_id);
        if let Some(fetch) = fetch_child {
            url.push_str(&format!("&fetch_child={}", if fetch { "1" } else { "0" }));
        }
        let status_val = status.unwrap_or(0);
        url.push_str(&format!("&status={}", status_val));

        let response: WechatApiResponse<SimpleUserListResponse> = self.client
            .get(&url)
            .await?;
        Ok(response.into_result()?.userlist)
    }

    /// 二次验证
    ///
    /// 企业在员工验证成功后，调用本方法告诉企业号平台该员工关注成功。
    /// 文档地址：<https://work.weixin.qq.com/api/doc/90202>
    pub async fn authenticate(&self, userid: &str) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self.client
            .get(&format!("/cgi-bin/user/authsucc?userid={}", userid))
            .await?;
        Ok(response)
    }

    /// 邀请成员
    ///
    /// 企业可通过接口批量邀请成员使用企业微信，邀请后将通过短信或邮件下发通知。
    /// 文档地址：<https://work.weixin.qq.com/api/doc/12543>
    ///
    /// # 参数说明
    /// * `userids` - 成员ID列表，最多支持1000个
    /// * `party_ids` - 部门ID列表，最多支持100个
    /// * `tag_ids` - 标签ID列表，最多支持100个
    pub async fn invite(
        &self,
        userids: Vec<String>,
        party_ids: Vec<i32>,
        tag_ids: Vec<i32>,
    ) -> LabradorResult<InviteResponse> {
        let req = json!({
            "user": userids,
            "party": party_ids,
            "tag": tag_ids,
        });
        let response: WechatApiResponse<InviteResponse> = self.client
            .post("/cgi-bin/batch/invite", req)
            .await?;
        response.into_result()
    }

    /// 获取加入企业二维码
    ///
    /// 文档地址：<https://work.weixin.qq.com/api/doc/91714>
    ///
    /// # 参数说明
    /// * `size_type` - 二维码类型，1: 默认值，适用于部分用户扫码; 2: 适用于成员活码
    pub async fn get_join_qrcode(&self, size_type: Option<i32>) -> LabradorResult<String> {
        let size = size_type.unwrap_or(1);
        let response: WechatApiResponse<JoinQrcodeResponse> = self.client
            .get(&format!("/cgi-bin/corp/get_join_qrcode?size_type={}", size))
            .await?;
        Ok(response.into_result()?.join_qrcode)
    }

    /// 获取企业活跃成员数
    ///
    /// 文档地址：<https://developer.work.weixin.qq.com/document/path/92714>
    ///
    /// # 参数说明
    /// * `date` - 具体某天的活跃信息，最长支持获取30天前数据
    pub async fn get_active_count(&self, date: &str) -> LabradorResult<u64> {
        let response: WechatApiResponse<ActiveCountResponse> = self.client
            .post("/cgi-bin/user/get_active_stat", json!({ "date": date }))
            .await?;
        Ok(response.into_result()?.active_cnt)
    }

    // ==================== ID转换接口 ====================

    /// userid转openid
    ///
    /// 该接口使用场景为微信支付、微信红包和企业转账。
    /// 需要成员使用微信登录企业微信或者关注微信插件才能转成openid。
    /// 文档地址：<https://work.weixin.qq.com/api/doc/11279>
    ///
    /// # 参数说明
    /// * `userid` - 企业内的成员id
    /// * `agent_id` - 应用id，若指定则返回该应用关联的openid，否则返回源应用的openid
    pub async fn userid_to_openid(&self, userid: &str, agent_id: Option<i32>) -> LabradorResult<UseridToOpenidResponse> {
        let mut req = json!({ "userid": userid });
        if let Some(agent) = agent_id {
            req["agentid"] = json!(agent);
        }
        let response: WechatApiResponse<UseridToOpenidResponse> = self.client
            .post("/cgi-bin/user/convert_to_openid", req)
            .await?;
        response.into_result()
    }

    /// openid转userid
    ///
    /// 该接口主要应用于使用微信支付、微信红包和企业转账之后的结果查询。
    /// 文档地址：<https://work.weixin.qq.com/api/doc/11279>
    ///
    /// # 参数说明
    /// * `openid` - 在使用微信支付、微信红包和企业转账之后，返回的openid
    pub async fn openid_to_userid(&self, openid: &str) -> LabradorResult<String> {
        let response: WechatApiResponse<OpenidToUseridResponse> = self.client
            .post("/cgi-bin/user/convert_to_userid", json!({ "openid": openid }))
            .await?;
        Ok(response.into_result()?.userid)
    }

    /// 手机号获取userid
    ///
    /// 通过手机号获取其所对应的userid。
    /// 文档地址：<https://work.weixin.qq.com/api/doc/91693>
    ///
    /// # 参数说明
    /// * `mobile` - 手机号码
    pub async fn get_userid_by_mobile(&self, mobile: &str) -> LabradorResult<String> {
        let response: WechatApiResponse<MobileToUseridResponse> = self.client
            .post("/cgi-bin/user/getuserid", json!({ "mobile": mobile }))
            .await?;
        Ok(response.into_result()?.userid)
    }

    // ==================== 外部联系人接口 ====================

    /// 获取外部联系人详情
    ///
    /// 企业可通过此接口，根据外部联系人的userid，拉取外部联系人详情。
    /// 文档地址：<https://work.weixin.qq.com/api/doc/90001/90143/91617>
    ///
    /// # 参数说明
    /// * `userid` - 外部联系人的userid
    pub async fn get_external_contact(&self, userid: &str) -> LabradorResult<UserExternalContactDetail> {
        let response: WechatApiResponse<UserExternalContactDetail> = self.client
            .get(&format!("/cgi-bin/crm/get_external_contact?external_userid={}", userid))
            .await?;
        response.into_result()
    }
}

// ============================================================================
// 请求与响应结构体
// ============================================================================

/// 用户列表响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserListResponse {
    /// 成员列表
    pub userlist: Vec<UserInfo>,
}

/// 简略用户信息（用于simplelist接口）
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleUserInfo {
    /// 成员UserID
    pub userid: String,
    /// 成员名称
    pub name: String,
    /// 成员所属部门列表
    pub department: Vec<i32>,
    /// 手机号
    pub mobile: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 头像url
    pub avatar: Option<String>,
    /// 成员启用状态，1表示启用，0表示禁用
    pub status: Option<i32>,
}

/// 简略用户列表响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SimpleUserListResponse {
    /// 成员列表
    pub userlist: Vec<SimpleUserInfo>,
}

/// 邀请成员响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteResponse {
    /// 无效的成员
    pub invaliduser: Option<Vec<String>>,
    /// 无效的部门
    pub invalidparty: Option<Vec<i32>>,
    /// 无效的标签
    pub invalidtag: Option<Vec<i32>>,
}

/// 加入企业二维码响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JoinQrcodeResponse {
    /// 二维码base64编码
    pub join_qrcode: String,
}

/// 活跃成员数响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ActiveCountResponse {
    /// 活跃成员数
    pub active_cnt: u64,
}

/// userid转openid响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UseridToOpenidResponse {
    /// 企业微信成员对应微信的openid
    pub openid: String,
    /// 该openid对应哪个应用的appid，第三方应用专用
    pub appid: Option<String>,
}

/// openid转userid响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OpenidToUseridResponse {
    /// 该openid对应的企业微信成员userid
    pub userid: String,
}

/// 手机号转userid响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MobileToUseridResponse {
    /// 该手机号对应的企业微信成员userid
    pub userid: String,
}

/// 外部联系人详情
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserExternalContactDetail {
    /// 外部联系人信息
    pub external_contact: ExternalContact,
    /// 跟进人信息
    pub follow_user: Vec<FollowUser>,
    /// 下一页的cursor
    pub next_cursor: Option<String>,
}