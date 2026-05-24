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

use crate::errors::LabradorResult;
use crate::wechat::client::WechatApiResponse;
use crate::wechat::miniapp::api::TemplateValue;
use crate::wechat::mp::api::MiniProgram;
use crate::wechat::mp::WechatMpClient;

/// 订阅通知服务接口（含一次性订阅消息）
#[derive(Debug, Clone)]
pub struct WechatMpSubscribeMessage<'a> {
    client: &'a WechatMpClient,
}

#[allow(unused)]
impl<'a> WechatMpSubscribeMessage<'a> {
    #[inline]
    pub fn new(client: &'a WechatMpClient) -> Self {
        Self { client }
    }

    // ==================== 一次性订阅消息 ====================

    /// 构造用户订阅一条模板消息授权的url链接
    ///
    /// 用于在网页中引导用户点击授权，获取一次性订阅资格。
    pub async fn subscribe_once_authorization_url(
        &self,
        template_id: &str,
        redirect_uri: &str,
        scene: i32,
        reserved: &str,
    ) -> String {
        let config = self.client.config();
        format!(
            "https://mp.weixin.qq.com/mp/subscribemsg?action=get_confirm&appid={}&scene={}&template_id={}&redirect_url={}&reserved={}#wechat_redirect",
            config.app_id, scene, template_id, urlencoding::encode(redirect_uri), reserved
        )
    }

    /// 发送一次性订阅消息
    ///
    /// 推送订阅模板消息给授权微信用户。
    /// 用户已关注公众号时消息下发到公众号会话，未关注时下发到服务通知。
    pub async fn send_subscribe_once(&self, msg: &SubscribeOnceRequest) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self.client.wechat_client()
            .post("/cgi-bin/message/template/subscribe", msg)
            .await?;
        Ok(response)
    }

    // ==================== 订阅通知（模板管理）====================

    /// 获取公众号类目
    ///
    /// 本接口用于获取小程序、公众号所属类目，用于查询公共模板。
    pub async fn get_category(&self) -> LabradorResult<Vec<Category>> {
        let response: WechatApiResponse<GetCategoryResponse> = self.client.wechat_client()
            .get("/wxaapi/newtmpl/getcategory")
            .await?;
        Ok(response.into_result()?.data)
    }

    /// 获取类目下的公共模板标题
    ///
    /// 该接口用于获取帐号所属类目下的公共模板，可从中选用模板使用。
    pub async fn get_pub_template_titles(
        &self,
        ids: &str,
        start: i32,
        limit: i32,
    ) -> LabradorResult<PubTemplateTitlesResponse> {
        let response: WechatApiResponse<PubTemplateTitlesResponse> = self.client.wechat_client()
            .get(&format!("/wxaapi/newtmpl/getpubtemplatetitles?ids={}&start={}&limit={}", ids, start, limit))
            .await?;
        response.into_result()
    }

    /// 获取模板中的关键词
    ///
    /// 该接口用于获取模板标题下的关键词列表。
    pub async fn get_pub_template_keywords(&self, tid: &str) -> LabradorResult<Vec<PubTemplateKeyword>> {
        let response: WechatApiResponse<GetPubTemplateKeywordsResponse> = self.client.wechat_client()
            .get(&format!("/wxaapi/newtmpl/getpubtemplatekeywords?tid={}", tid))
            .await?;
        Ok(response.into_result()?.data)
    }

    /// 组合模板并添加至个人模板库
    ///
    /// 从公共模板库中选用模板到私有模板库。
    pub async fn add_template(&self, tid: &str, kid_list: Vec<i32>, scene_desc: &str) -> LabradorResult<String> {
        let req = json!({
            "tid": tid,
            "kidList": kid_list,
            "sceneDesc": scene_desc,
        });
        let response: WechatApiResponse<AddTemplateResponse> = self.client.wechat_client()
            .post("/wxaapi/newtmpl/addtemplate", req)
            .await?;
        Ok(response.into_result()?.pri_tmpl_id)
    }

    /// 获取已有模板列表
    ///
    /// 该接口用于获取当前帐号下的已有的模板列表。
    pub async fn get_template_list(&self) -> LabradorResult<Vec<PrivateTemplate>> {
        let response: WechatApiResponse<GetTemplateListResponse> = self.client.wechat_client()
            .get("/wxaapi/newtmpl/gettemplate")
            .await?;
        Ok(response.into_result()?.data)
    }

    /// 删除帐号下的某个模板
    pub async fn delete_template(&self, pri_tmpl_id: &str) -> LabradorResult<WechatApiResponse> {
        let req = json!({ "priTmplId": pri_tmpl_id });
        let response: WechatApiResponse = self.client.wechat_client()
            .post("/wxaapi/newtmpl/deltemplate", req)
            .await?;
        Ok(response)
    }

    /// 发送订阅通知
    ///
    /// 该接口用于发送订阅消息（业务通知）。
    pub async fn send_subscribe_message(&self, msg: &SubscribeMessageRequest) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self.client.wechat_client()
            .post("/cgi-bin/message/subscribe/bizsend", msg)
            .await?;
        Ok(response)
    }
}

// ============================================================================
// 请求与响应结构体
// ============================================================================

// ----- 一次性订阅消息 -----

/// 一次性订阅消息请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeOnceRequest {
    /// 接收消息的用户openid
    pub touser: String,
    /// 订阅消息模板ID
    pub template_id: String,
    /// 点击消息跳转的链接，需要有 ICP 备案
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 跳小程序配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miniprogram: Option<MiniProgram>,
    /// 订阅场景值
    pub scene: String,
    /// 消息标题，15字以内
    pub title: String,
    /// 消息内容
    pub data: SubscribeOnceData,
}

/// 一次性订阅消息数据
#[derive(Debug, Clone, Serialize)]
pub struct SubscribeOnceData {
    /// 内容信息
    pub content: TemplateValue,
}

// ----- 订阅通知 -----

/// 发送订阅通知请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeMessageRequest {
    /// 接收者（用户）的 openid
    pub touser: String,
    /// 所需下发的订阅模板id
    pub template_id: String,
    /// 点击模板卡片后的跳转页面，仅限本小程序内的页面
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    /// 模板内容，格式形如 { "phrase3": { "value": "审核通过" } }
    pub data: serde_json::Map<String, serde_json::Value>,
    /// 跳转小程序类型：developer为开发版；trial为体验版；formal为正式版
    pub miniprogram_state: String,
    /// “进入小程序查看”的语言类型
    pub lang: String,
}

/// 类目
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: i32,
    pub name: String,
}

/// 公共模板标题响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PubTemplateTitlesResponse {
    pub count: i32,
    pub data: Vec<PubTemplateTitle>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PubTemplateTitle {
    pub tid: i32,
    pub title: String,
    #[serde(rename = "type")]
    pub type_: i32,
    pub category_id: String,
}

/// 公共模板关键词
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PubTemplateKeyword {
    pub kid: i32,
    pub name: String,
    pub example: String,
    pub rule: String,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword_enum_value_list: Option<Vec<KeywordEnumValue>>,
}

/// 枚举参数值范围
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeywordEnumValue {
    pub keyword_code: String,
    pub enum_value_list: Vec<String>,
}

// 辅助响应结构体
#[derive(Debug, Clone, Deserialize)]
struct GetCategoryResponse {
    data: Vec<Category>,
}

#[derive(Debug, Clone, Deserialize)]
struct GetPubTemplateKeywordsResponse {
    data: Vec<PubTemplateKeyword>,
}

#[derive(Debug, Clone, Deserialize)]
struct AddTemplateResponse {
    pri_tmpl_id: String,
}

#[derive(Debug, Clone, Deserialize)]
struct GetTemplateListResponse {
    data: Vec<PrivateTemplate>,
}