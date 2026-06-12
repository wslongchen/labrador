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
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::errors::LabradorResult;
use crate::wechat::client::WechatApiResponse;
use crate::wechat::miniapp::WechatMiniAppClient;

#[derive(Debug, Clone)]
pub struct WechatMxaMessage<'a> {
    client: &'a WechatMiniAppClient,
}

#[allow(unused)]
impl<'a> WechatMxaMessage<'a> {
    #[inline]
    pub fn new(client: &'a WechatMiniAppClient) -> WechatMxaMessage<'a> {
        WechatMxaMessage { client }
    }

    /// 发送订阅消息
    ///
    /// 本接口用于向指定用户发送订阅消息。用户需要先在客户端完成订阅授权，
    /// 服务端方可调用此接口向该用户推送模板消息。
    ///
    /// # 参数
    /// * `request` - 发送订阅消息请求参数，包含接收用户 openid（touser）、
    ///   模板ID（template_id）、模板数据（data）、跳转页面（page）等
    ///
    /// # 返回
    /// 返回 `LabradorResult<WechatApiResponse>`，成功时返回空响应。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/mp-message-management/subscribe-message/sendMessage.html>
    pub async fn send(
        &self,
        request: &SubscribeMessageRequest,
    ) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/cgi-bin/message/subscribe/send", request)
            .await?;
        Ok(response)
    }

    /// 获取类目下的公共模板
    ///
    /// 本接口用于获取帐号所属类目下的公共模板标题列表，开发者可从中选用模板使用。
    ///
    /// # 参数
    /// * `request` - 请求参数，包含类目ID（ids）、起始位置（start）和获取数量（limit）
    ///
    /// # 返回
    /// 返回 `LabradorResult<PubTemplateTitlesResponse>`，包含公共模板标题列表。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/mp-message-management/subscribe-message/getPubTemplateTitles.html>
    pub async fn get_pub_template_titles(
        &self,
        request: &PubTemplateTitlesRequest,
    ) -> LabradorResult<PubTemplateTitlesResponse> {
        let response: WechatApiResponse<PubTemplateTitlesResponse> = self
            .client
            .wechat_client()
            .get(&format!(
                "/wxaapi/newtmpl/getpubtemplatetitles?ids={}&start={}&limit={}",
                request.ids, request.start, request.limit
            ))
            .await?;
        response.into_result()
    }

    /// 获取模板中的关键词
    ///
    /// 本接口用于获取指定模板标题下的关键词列表，用于了解模板中可填充的关键词字段。
    ///
    /// # 参数
    /// * `tid` - 公共模板ID，通过获取类目下的公共模板接口获得
    ///
    /// # 返回
    /// 返回 `LabradorResult<Vec<PubTemplateKeyword>>`，包含模板关键词列表。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/mp-message-management/subscribe-message/getPubTemplateKeyWordsById.html>
    pub async fn get_pub_template_keywords(
        &self,
        tid: &str,
    ) -> LabradorResult<Vec<PubTemplateKeyword>> {
        let response: WechatApiResponse<GetPubTemplateKeywordsResponse> = self
            .client
            .wechat_client()
            .get(&format!(
                "/wxaapi/newtmpl/getpubtemplatekeywords?tid={}",
                tid
            ))
            .await?;
        Ok(response.into_result()?.data)
    }

    /// 选用模板
    ///
    /// 本接口用于从公共模板库中选用模板到私有模板库，选用后可用于发送订阅消息。
    ///
    /// # 参数
    /// * `request` - 请求参数，包含公共模板ID（tid）、关键词ID列表（kid_list）
    ///   和场景描述（scene_desc）
    ///
    /// # 返回
    /// 返回 `LabradorResult<String>`，即选用成功后的私有模板ID（priTmplId）。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/mp-message-management/subscribe-message/addMessageTemplate.html>
    pub async fn add_template(&self, request: &AddTemplateRequest) -> LabradorResult<String> {
        let response: WechatApiResponse<AddTemplateResponse> = self
            .client
            .wechat_client()
            .post("/wxaapi/newtmpl/addtemplate", request)
            .await?;
        Ok(response.into_result()?.pri_tmpl_id)
    }

    /// 获取已有模板列表
    ///
    /// 本接口用于获取当前帐号下已有的私有模板列表，包含模板ID、标题、内容等信息。
    ///
    /// # 返回
    /// 返回 `LabradorResult<Vec<PrivateTemplate>>`，包含私有模板信息列表。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/mp-message-management/subscribe-message/getMessageTemplateList.html>
    pub async fn get_template_list(&self) -> LabradorResult<Vec<PrivateTemplate>> {
        let response: WechatApiResponse<GetTemplateListResponse> = self
            .client
            .wechat_client()
            .get("/wxaapi/newtmpl/gettemplate")
            .await?;
        Ok(response.into_result()?.data)
    }

    /// 删除模板
    ///
    /// 本接口用于删除私有模板库中的指定模板。
    ///
    /// # 参数
    /// * `pri_tmpl_id` - 私有模板ID，通过选用模板接口获得
    ///
    /// # 返回
    /// 返回 `LabradorResult<WechatApiResponse>`，成功时返回空响应。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/mp-message-management/subscribe-message/deleteMessageTemplate.html>
    pub async fn delete_template(&self, pri_tmpl_id: &str) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post(
                "/wxaapi/newtmpl/deltemplate",
                serde_json::json!({ "priTmplId": pri_tmpl_id }),
            )
            .await?;
        Ok(response)
    }

    /// 获取类目
    ///
    /// 本接口用于获取小程序/公众号所属的类目列表，用于查询公共模板时按类目筛选。
    ///
    /// # 返回
    /// 返回 `LabradorResult<Vec<Category>>`，包含类目ID和名称的列表。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/mp-message-management/subscribe-message/getCategory.html>
    pub async fn get_category(&self) -> LabradorResult<Vec<Category>> {
        let response: WechatApiResponse<GetCategoryResponse> = self
            .client
            .wechat_client()
            .get("/wxaapi/newtmpl/getcategory")
            .await?;
        Ok(response.into_result()?.data)
    }

    /// 激活与更新服务卡片
    ///
    /// 本接口用于激活或更新用户的微信服务卡片通知。服务卡片是微信小程序
    /// 的一种消息触达方式，用于在微信聊天列表中展示服务信息。
    ///
    /// # 参数
    /// * `request` - 请求参数，包含用户 openid 和服务卡片数据（data）
    ///
    /// # 返回
    /// 返回 `LabradorResult<WechatApiResponse>`，成功时返回空响应。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/mp-message-management/updatable-message/activateMessage.html>
    pub async fn set_user_notify(
        &self,
        request: &SetUserNotifyRequest,
    ) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/wxa/set_user_notify", request)
            .await?;
        Ok(response)
    }

    /// 更新服务卡片扩展信息
    ///
    /// 本接口用于更新已激活的服务卡片的扩展信息。
    ///
    /// # 参数
    /// * `request` - 请求参数，包含用户 openid 和扩展数据（data）
    ///
    /// # 返回
    /// 返回 `LabradorResult<WechatApiResponse>`，成功时返回空响应。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/mp-message-management/updatable-message/updateMessage.html>
    pub async fn set_user_notifyext(
        &self,
        request: &SetUserNotifyExtRequest,
    ) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/wxa/set_user_notifyext", request)
            .await?;
        Ok(response)
    }

    /// 查询服务卡片状态
    ///
    /// 本接口用于查询指定用户的服务卡片当前状态。
    ///
    /// # 参数
    /// * `request` - 请求参数，包含用户 openid
    ///
    /// # 返回
    /// 返回 `LabradorResult<UserNotifyStatus>`，包含用户的 openid 和通知信息。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/mp-message-management/updatable-message/queryMessageStatus.html>
    pub async fn get_user_notify(
        &self,
        request: &GetUserNotifyRequest,
    ) -> LabradorResult<UserNotifyStatus> {
        let response: WechatApiResponse<UserNotifyStatus> = self
            .client
            .wechat_client()
            .post("/wxa/get_user_notify", request)
            .await?;
        response.into_result()
    }
}

//----------------------------------------------------------------------------------------------------------------------------
// 请求与响应结构体

/// 发送订阅消息请求
#[derive(Debug, Clone, Serialize)]
pub struct SubscribeMessageRequest {
    pub touser: String,
    pub template_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    pub data: HashMap<String, TemplateValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miniprogram_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}

impl SubscribeMessageRequest {
    pub fn new(touser: &str, template_id: &str, data: HashMap<String, TemplateValue>) -> Self {
        Self {
            touser: touser.to_string(),
            template_id: template_id.to_string(),
            page: None,
            data,
            miniprogram_state: None,
            lang: None,
        }
    }

    pub fn with_page(mut self, page: &str) -> Self {
        self.page = Some(page.to_string());
        self
    }

    pub fn with_miniprogram_state(mut self, state: &str) -> Self {
        self.miniprogram_state = Some(state.to_string());
        self
    }

    pub fn with_lang(mut self, lang: &str) -> Self {
        self.lang = Some(lang.to_string());
        self
    }
}

/// 模板值
#[derive(Debug, Clone, Serialize)]
pub struct TemplateValue {
    pub value: String,
}

impl TemplateValue {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

/// 获取公共模板标题请求
#[derive(Debug, Clone)]
pub struct PubTemplateTitlesRequest {
    pub ids: String,
    pub start: i32,
    pub limit: i32,
}

impl PubTemplateTitlesRequest {
    pub fn new(ids: &str, start: i32, limit: i32) -> Self {
        Self {
            ids: ids.to_string(),
            start,
            limit,
        }
    }
}

/// 获取公共模板标题响应
#[derive(Debug, Clone, Deserialize)]
pub struct PubTemplateTitlesResponse {
    pub data: Vec<PubTemplateTitle>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PubTemplateTitle {
    pub tid: i32,
    pub title: String,
    #[serde(rename = "type")]
    pub type_: i32,
    #[serde(default)]
    pub category_id: String,
}

/// 公共模板关键词响应
#[derive(Debug, Clone, Deserialize)]
struct GetPubTemplateKeywordsResponse {
    data: Vec<PubTemplateKeyword>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PubTemplateKeyword {
    pub kid: i32,
    pub name: String,
    pub example: String,
    pub rule: String,
}

/// 选用模板请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddTemplateRequest {
    pub tid: String,
    pub kid_list: Vec<i32>,
    pub scene_desc: String,
}

impl AddTemplateRequest {
    pub fn new(tid: &str, kid_list: Vec<i32>, scene_desc: &str) -> Self {
        Self {
            tid: tid.to_string(),
            kid_list,
            scene_desc: scene_desc.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AddTemplateResponse {
    pri_tmpl_id: String,
}

/// 私有模板
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivateTemplate {
    pub pri_tmpl_id: String,
    pub title: String,
    pub content: String,
    pub example: String,
    #[serde(rename = "type")]
    pub type_: i32,
}

#[derive(Debug, Clone, Deserialize)]
struct GetTemplateListResponse {
    data: Vec<PrivateTemplate>,
}

/// 类目
#[derive(Debug, Clone, Deserialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
struct GetCategoryResponse {
    data: Vec<Category>,
}

/// 设置用户通知请求
#[derive(Debug, Clone, Serialize)]
pub struct SetUserNotifyRequest {
    pub openid: String,
    pub data: Value,
}

impl SetUserNotifyRequest {
    pub fn new(openid: &str, data: Value) -> Self {
        Self {
            openid: openid.to_string(),
            data,
        }
    }
}

/// 设置用户通知扩展请求
#[derive(Debug, Clone, Serialize)]
pub struct SetUserNotifyExtRequest {
    pub openid: String,
    pub data: Value,
}

impl SetUserNotifyExtRequest {
    pub fn new(openid: &str, data: Value) -> Self {
        Self {
            openid: openid.to_string(),
            data,
        }
    }
}

/// 查询用户通知请求
#[derive(Debug, Clone, Serialize)]
pub struct GetUserNotifyRequest {
    pub openid: String,
}

impl GetUserNotifyRequest {
    pub fn new(openid: &str) -> Self {
        Self {
            openid: openid.to_string(),
        }
    }
}

/// 用户通知状态
#[derive(Debug, Clone, Deserialize)]
pub struct UserNotifyStatus {
    pub openid: String,
    pub notify_info: Value,
}
