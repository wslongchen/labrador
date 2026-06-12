/*
 *
 *  *
 *  *      Copyright (c) 2018-2025, WoofCloud All rights reserved.
 *  *
 *  *   Redistribution and use in source and binary forms, with or without
 *  *   modification, are permitted provided that the following conditions are met:
 *  *
 *  *   Redistributions of source code must retain the above copyright notice,
 *  *   this list of conditions and the following disclaimer.
 *  *   Redistributions in binary form must reproduce the above copyright
 *  *   notice, this list of conditions and the following disclaimer in the
 *  *   documentation and/or other materials provided with the distribution.
 *  *   Neither the name of the www.woofcloud.com developer nor the names of its
 *  *   contributors may be used to endorse or promote products derived from
 *  *   this software without specific prior written permission.
 *  *   Author: WoofCloud
 *  *
 *
 */
use serde::Deserialize;
use serde_json::json;

use crate::errors::LabradorResult;
use crate::wechat::client::WechatApiResponse;
use crate::wechat::cp::WechatCpClient;

/// 企业微信标签管理模块
///
/// 包含标签的创建、更新、删除、查询，以及标签成员的添加、移除等功能。
#[derive(Debug, Clone)]
pub struct WechatCpTag<'a> {
    client: &'a WechatCpClient,
}

#[allow(unused)]
impl<'a> WechatCpTag<'a> {
    #[inline]
    pub fn new(client: &'a WechatCpClient) -> Self {
        Self { client }
    }

    /// 创建标签
    ///
    /// 详情请见：<https://work.weixin.qq.com/api/doc/90210>
    ///
    /// # 参数说明
    /// * `name` - 标签名称，长度限制为1~32个字符
    /// * `tag_id` - 标签id，非负整型，指定此参数时新增的标签会生成对应的标签id，不指定时则由后台自动生成
    pub async fn create(&self, name: &str, tag_id: Option<i32>) -> LabradorResult<i32> {
        let mut req = json!({ "tagname": name });
        if let Some(id) = tag_id {
            req["tagid"] = json!(id);
        }
        let response: WechatApiResponse<CreateTagResponse> =
            self.client.post("/cgi-bin/tag/create", req).await?;
        Ok(response.into_result()?.tagid)
    }

    /// 更新标签
    ///
    /// 详情请见：<https://work.weixin.qq.com/api/doc/90211>
    ///
    /// # 参数说明
    /// * `tag_id` - 标签ID
    /// * `tag_name` - 标签名称，长度限制为1~32个字符
    pub async fn update(&self, tag_id: i32, tag_name: &str) -> LabradorResult<WechatApiResponse> {
        let req = json!({
            "tagid": tag_id,
            "tagname": tag_name,
        });
        let response: WechatApiResponse = self.client.post("/cgi-bin/tag/update", req).await?;
        Ok(response)
    }

    /// 删除标签
    ///
    /// 详情请见：<https://work.weixin.qq.com/api/doc/90212>
    ///
    /// # 参数说明
    /// * `tag_id` - 标签ID
    pub async fn delete(&self, tag_id: i32) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .get(&format!("/cgi-bin/tag/delete?tagid={}", tag_id))
            .await?;
        Ok(response)
    }

    /// 获取标签成员
    ///
    /// 详情请见：<https://work.weixin.qq.com/api/doc/90213>
    ///
    /// # 参数说明
    /// * `tag_id` - 标签ID
    pub async fn get(&self, tag_id: i32) -> LabradorResult<TagMemberResponse> {
        let response: WechatApiResponse<TagMemberResponse> = self
            .client
            .get(&format!("/cgi-bin/tag/get?tagid={}", tag_id))
            .await?;
        response.into_result()
    }

    /// 增加标签成员
    ///
    /// 详情请见：<https://work.weixin.qq.com/api/doc/90214>
    ///
    /// # 参数说明
    /// * `tag_id` - 标签ID
    /// * `user_ids` - 企业成员ID列表，最多支持1000个
    /// * `party_ids` - 企业部门ID列表，最多支持100个
    pub async fn add_users(
        &self,
        tag_id: i32,
        user_ids: Vec<String>,
        party_ids: Vec<i32>,
    ) -> LabradorResult<TagUserOperationResponse> {
        let req = json!({
            "tagid": tag_id,
            "userlist": user_ids,
            "partylist": party_ids,
        });
        let response: WechatApiResponse<TagUserOperationResponse> =
            self.client.post("/cgi-bin/tag/addtagusers", req).await?;
        response.into_result()
    }

    /// 移除标签成员
    ///
    /// 详情请见：<https://work.weixin.qq.com/api/doc/90215>
    ///
    /// # 参数说明
    /// * `tag_id` - 标签ID
    /// * `user_ids` - 企业成员ID列表，最多支持1000个
    /// * `party_ids` - 企业部门ID列表，最多支持100个
    pub async fn remove_users(
        &self,
        tag_id: i32,
        user_ids: Vec<String>,
        party_ids: Vec<i32>,
    ) -> LabradorResult<TagUserOperationResponse> {
        let req = json!({
            "tagid": tag_id,
            "userlist": user_ids,
            "partylist": party_ids,
        });
        let response: WechatApiResponse<TagUserOperationResponse> =
            self.client.post("/cgi-bin/tag/deltagusers", req).await?;
        response.into_result()
    }

    /// 获取标签列表
    ///
    /// 获取企业的所有标签列表。
    ///
    /// # 返回
    /// 返回 `LabradorResult<Vec<TagInfo>>`，包含标签 id 和名称列表
    ///
    /// # 官方文档
    /// <https://developer.work.weixin.qq.com/document/path/90216>
    pub async fn list(&self) -> LabradorResult<Vec<TagInfo>> {
        let response: WechatApiResponse<TagListResponse> =
            self.client.get("/cgi-bin/tag/list").await?;
        Ok(response.into_result()?.taglist)
    }
}

// ============================================================================
// 请求与响应结构体
// ============================================================================

/// 创建标签响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateTagResponse {
    /// 标签id
    pub tagid: i32,
}

/// 标签信息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagInfo {
    /// 标签id
    pub tagid: i32,
    /// 标签名
    pub tagname: String,
}

/// 标签成员响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagMemberResponse {
    /// 标签名
    pub tagname: String,
    /// 用户列表
    pub userlist: Vec<TagUserInfo>,
    /// 部门列表
    pub partylist: Vec<i32>,
}

/// 标签中的用户信息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagUserInfo {
    /// 成员UserID
    pub userid: String,
    /// 成员名称
    pub name: String,
    /// 手机号
    pub mobile: Option<String>,
    /// 成员所属部门列表
    pub department: Option<Vec<i32>>,
    /// 部门内的排序值
    pub order: Option<Vec<i32>>,
    /// 职位信息
    pub position: Option<String>,
    /// 性别，1男性，2女性
    pub gender: Option<i32>,
    /// 邮箱
    pub email: Option<String>,
    /// 企业邮箱
    pub biz_mail: Option<String>,
    /// 头像缩略图url
    pub thumb_avatar: Option<String>,
    /// 头像url
    pub avatar: Option<String>,
    /// 别名
    pub alias: Option<String>,
    /// 激活状态，1已激活，2已禁用，4未激活，5退出企业
    pub status: Option<i32>,
    /// 全局唯一。对于同一个服务商，不同应用获取到企业内同一个成员的open_userid是相同的
    pub open_userid: Option<String>,
    /// 成员对外属性
    pub external_attr: Option<Vec<TagExternalAttr>>,
    /// 成员对外职位
    pub external_position: Option<String>,
    /// 对外公司名称
    pub external_corp_name: Option<String>,
    /// 上级领导UserID列表
    pub direct_leader: Option<Vec<String>>,
    /// 视频号信息
    pub wechat_channels: Option<WechatChannels>,
}

/// 外部属性
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagExternalAttr {
    /// 属性类型: 0-本文 1-网页 2-小程序
    #[serde(rename = "type")]
    pub attr_type: i32,
    /// 属性名称
    pub name: String,
    /// 文本属性内容
    pub text: Option<ExternalAttrText>,
    /// 网页属性内容
    pub web: Option<ExternalAttrWeb>,
    /// 小程序属性内容
    pub miniprogram: Option<ExternalAttrMiniProgram>,
}

/// 外部属性-文本
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalAttrText {
    /// 文本内容
    pub value: String,
}

/// 外部属性-网页
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalAttrWeb {
    /// 网页标题
    pub title: String,
    /// 网页url
    pub url: String,
}

/// 外部属性-小程序
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalAttrMiniProgram {
    /// 小程序展示标题
    pub title: String,
    /// 小程序appid
    pub appid: String,
    /// 小程序页面路径
    pub pagepath: String,
}

/// 视频号信息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WechatChannels {
    /// 视频号名称
    pub nickname: String,
    /// 状态，1启用，2停用
    pub status: i32,
}

/// 标签用户操作响应（添加/移除）
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagUserOperationResponse {
    /// 非法的成员列表
    pub invalidlist: Option<String>,
    /// 非法的部门列表
    pub invalidparty: Option<Vec<i32>>,
}

/// 标签列表响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TagListResponse {
    /// 标签列表
    pub taglist: Vec<TagInfo>,
}
