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
    /// 该接口用于获取帐号所属类目下的公共模板，可从中选用模板使用
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
    /// 该接口用于获取模板标题下的关键词列表
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
    /// 从公共模板库中选用模板到私有模板库
    pub async fn add_template(&self, request: &AddTemplateRequest) -> LabradorResult<String> {
        let response: WechatApiResponse<AddTemplateResponse> = self
            .client
            .wechat_client()
            .post("/wxaapi/newtmpl/addtemplate", request)
            .await?;
        Ok(response.into_result()?.pri_tmpl_id)
    }

    /// 获取已有模板列表
    /// 该接口用于获取当前帐号下的已有的模板列表
    pub async fn get_template_list(&self) -> LabradorResult<Vec<PrivateTemplate>> {
        let response: WechatApiResponse<GetTemplateListResponse> = self
            .client
            .wechat_client()
            .get("/wxaapi/newtmpl/gettemplate")
            .await?;
        Ok(response.into_result()?.data)
    }

    /// 删除模板
    /// 删除私有模板库中的模板
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
    /// 本接口用于获取小程序、公众号所属类目用于查询公共模板
    pub async fn get_category(&self) -> LabradorResult<Vec<Category>> {
        let response: WechatApiResponse<GetCategoryResponse> = self
            .client
            .wechat_client()
            .get("/wxaapi/newtmpl/getcategory")
            .await?;
        Ok(response.into_result()?.data)
    }

    /// 激活与更新服务卡片
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
