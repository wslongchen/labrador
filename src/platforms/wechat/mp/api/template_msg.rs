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
use serde_json::json;
use std::collections::HashMap;

use crate::errors::LabradorResult;
use crate::wechat::client::WechatApiResponse;
use crate::wechat::mp::WechatMpClient;

/// 模版消息服务接口
#[derive(Debug, Clone)]
pub struct WechatMpTemplateMessage<'a> {
    client: &'a WechatMpClient,
}

#[allow(unused)]
impl<'a> WechatMpTemplateMessage<'a> {
    #[inline]
    pub fn new(client: &'a WechatMpClient) -> Self {
        Self { client }
    }

    /// 设置所属行业
    ///
    /// 设置行业可在微信公众平台后台完成，每月可修改行业1次。帐号仅可使用所属行业中相关的模板。
    pub async fn set_industry(
        &self,
        industry_id1: &str,
        industry_id2: &str,
    ) -> LabradorResult<WechatApiResponse> {
        let req = json!({
            "industry_id1": industry_id1,
            "industry_id2": industry_id2,
        });
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/cgi-bin/template/api_set_industry", req)
            .await?;
        Ok(response)
    }

    /// 获取设置的行业信息
    pub async fn get_industry(&self) -> LabradorResult<IndustryResponse> {
        let response: WechatApiResponse<IndustryResponse> = self
            .client
            .wechat_client()
            .get("/cgi-bin/template/get_industry")
            .await?;
        response.into_result()
    }

    /// 获得模板ID
    ///
    /// 从行业模板库选择模板到帐号后台，获得模板ID。
    /// `template_id_short` 模板库中模板的编号，有“TM**”和“OPENTMTM**”等形式。
    pub async fn get_template_id(&self, template_id_short: &str) -> LabradorResult<String> {
        let req = json!({ "template_id_short": template_id_short });
        let response: WechatApiResponse<GetTemplateIdResponse> = self
            .client
            .wechat_client()
            .post("/cgi-bin/template/api_add_template", req)
            .await?;
        Ok(response.into_result()?.template_id)
    }

    /// 获取模板列表
    ///
    /// 获取已添加至帐号下的所有模板列表。
    pub async fn get_template_list(&self) -> LabradorResult<Vec<TemplateMessageInfo>> {
        let response: WechatApiResponse<GetTemplateListResponse> = self
            .client
            .wechat_client()
            .get("/cgi-bin/template/get_all_private_template")
            .await?;
        Ok(response.into_result()?.template_list)
    }

    /// 删除模板
    ///
    /// 删除帐号下的某个模板。
    pub async fn delete_template(&self, template_id: &str) -> LabradorResult<WechatApiResponse> {
        let req = json!({ "template_id": template_id });
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/cgi-bin/template/del_private_template", req)
            .await?;
        Ok(response)
    }

    /// 发送模板消息
    ///
    /// 该接口用于发送模板消息。
    pub async fn send_template_message(
        &self,
        request: &TemplateMessageRequest,
    ) -> LabradorResult<TemplateMessageResponse> {
        let response: WechatApiResponse<TemplateMessageResponse> = self
            .client
            .wechat_client()
            .post("/cgi-bin/message/template/send", request)
            .await?;
        response.into_result()
    }

    /// 查询模板消息发送状态
    ///
    /// 该接口用于查询模板消息的发送状态。
    pub async fn get_template_message_status(
        &self,
        msg_id: i64,
    ) -> LabradorResult<TemplateMessageStatus> {
        let response: WechatApiResponse<TemplateMessageStatus> = self
            .client
            .wechat_client()
            .post(
                "/cgi-bin/message/template/queryblock",
                json!({ "msg_id": msg_id }),
            )
            .await?;
        response.into_result()
    }
}

// ============================================================================
// 请求与响应结构体
// ============================================================================

/// 行业信息响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndustryResponse {
    /// 主营行业
    pub primary_industry: IndustryClass,
    /// 副营行业
    pub secondary_industry: IndustryClass,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndustryClass {
    /// 一级行业
    pub first_class: String,
    /// 二级行业
    pub second_class: String,
}

/// 模板信息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateMessageInfo {
    /// 模板ID
    pub template_id: String,
    /// 模板标题
    pub title: String,
    /// 模板所属行业的一级行业
    pub primary_industry: String,
    /// 模板所属行业的二级行业
    pub deputy_industry: String,
    /// 模板内容
    pub content: String,
    /// 模板示例
    pub example: String,
}

/// 模板消息请求
#[derive(Debug, Clone, Serialize)]
pub struct TemplateMessageRequest {
    /// 接收者openid
    pub touser: String,
    /// 模板ID
    pub template_id: String,
    /// 模板跳转链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 跳小程序所需数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miniprogram: Option<MiniProgram>,
    /// 模板数据
    pub data: HashMap<String, TemplateData>,
    /// 防重入ID。对于同一个openid + client_msg_id, 只发送一条消息,10分钟有效
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_msg_id: Option<String>,
}

impl TemplateMessageRequest {
    /// 创建新的模板消息请求
    pub fn new(touser: &str, template_id: &str, data: HashMap<String, TemplateData>) -> Self {
        Self {
            touser: touser.to_string(),
            template_id: template_id.to_string(),
            url: None,
            miniprogram: None,
            data,
            client_msg_id: None,
        }
    }

    /// 设置跳转链接
    pub fn with_url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }

    /// 设置小程序跳转
    pub fn with_miniprogram(mut self, appid: &str, pagepath: &str) -> Self {
        self.miniprogram = Some(MiniProgram {
            appid: appid.to_string(),
            pagepath: pagepath.to_string(),
        });
        self
    }

    /// 设置防重入ID
    pub fn with_client_msg_id(mut self, client_msg_id: &str) -> Self {
        self.client_msg_id = Some(client_msg_id.to_string());
        self
    }
}

/// 小程序跳转数据
#[derive(Debug, Clone, Serialize)]
pub struct MiniProgram {
    /// 所需跳转到的小程序appid
    pub appid: String,
    /// 所需跳转到小程序的具体页面路径，支持带参数
    pub pagepath: String,
}

/// 模板数据
#[derive(Debug, Clone, Serialize)]
pub struct TemplateData {
    /// 模板内容
    pub value: String,
    /// 模板内容字体颜色，不填默认为黑色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

impl TemplateData {
    /// 创建新的模板数据
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
            color: None,
        }
    }

    /// 设置字体颜色
    pub fn with_color(mut self, color: &str) -> Self {
        self.color = Some(color.to_string());
        self
    }
}

/// 模板消息响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateMessageResponse {
    /// 错误码
    pub errcode: i32,
    /// 错误信息
    pub errmsg: String,
    /// 消息ID
    pub msgid: i64,
}

/// 模板消息状态响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateMessageStatus {
    /// 消息ID
    pub msg_id: i64,
    /// 消息状态
    pub status: String,
    /// 错误原因
    pub reason: Option<String>,
}

// 辅助响应结构体
#[derive(Debug, Clone, Deserialize)]
struct GetTemplateIdResponse {
    template_id: String,
}

#[derive(Debug, Clone, Deserialize)]
struct GetTemplateListResponse {
    template_list: Vec<TemplateMessageInfo>,
}
