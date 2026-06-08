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

//! 支付宝小程序模块
//!
//! 提供支付宝小程序相关接口实现

use super::client::AlipayClient;
use crate::alipay::method::AlipayMethod;
use crate::alipay::AlipayBizRequest;
use crate::errors::LabradorResult;
use crate::platforms::alipay::AlipayResponse;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// 小程序服务
pub struct AlipayMiniappService<'a> {
    client: &'a AlipayClient,
}

impl<'a> AlipayMiniappService<'a> {
    /// 创建新的小程序服务
    pub fn new(client: &'a AlipayClient) -> Self {
        Self { client }
    }

    /// 获取小程序模板列表
    pub async fn template_list(
        &self,
        template_type: Option<String>,
        page_num: Option<i32>,
        page_size: Option<i32>,
    ) -> LabradorResult<TemplateListResponse> {
        let mut request = AlipayBizRequest::new();
        let mut biz_content = BTreeMap::new();

        if let Some(t_type) = template_type {
            biz_content.insert("template_type".to_string(), t_type);
        }

        if let Some(page) = page_num {
            biz_content.insert("page_num".to_string(), page.to_string());
        }

        if let Some(size) = page_size {
            biz_content.insert("page_size".to_string(), size.to_string());
        }
        request.set_biz_model(biz_content);
        request.set_method(AlipayMethod::Custom(
            "alipay.open.mini.templatelist.query".to_string(),
        ));
        let response: AlipayResponse<TemplateListResponse> =
            self.client.request(request, None, None, None).await?;

        response.into_result()
    }

    /// 发送模板消息
    pub async fn send_template_message(
        &self,
        to_user_id: String,
        form_id: String,
        user_template_id: String,
        page: Option<String>,
        data: BTreeMap<String, TemplateDataItem>,
        emphasis_keyword: Option<String>,
    ) -> LabradorResult<SendTemplateMessageResponse> {
        let mut request = AlipayBizRequest::new();
        let mut biz_content = BTreeMap::new();
        biz_content.insert("to_user_id".to_string(), to_user_id);
        biz_content.insert("form_id".to_string(), form_id);
        biz_content.insert("user_template_id".to_string(), user_template_id);

        if let Some(page_path) = page {
            biz_content.insert("page".to_string(), page_path);
        }

        let data_json = serde_json::to_string(&data).unwrap_or_default();
        biz_content.insert("data".to_string(), data_json);

        if let Some(keyword) = emphasis_keyword {
            biz_content.insert("emphasis_keyword".to_string(), keyword);
        }
        request.set_biz_model(biz_content);
        request.set_method(AlipayMethod::Custom(
            "alipay.open.app.mini.templatemessage.send".to_string(),
        ));
        let response: AlipayResponse<SendTemplateMessageResponse> =
            self.client.request(request, None, None, None).await?;

        response.into_result()
    }

    /// 查询模板消息发送状态
    pub async fn query_template_message_status(
        &self,
        msg_id: String,
    ) -> LabradorResult<QueryTemplateMessageStatusResponse> {
        let mut request = AlipayBizRequest::new();
        let mut biz_content = BTreeMap::new();
        biz_content.insert("msg_id".to_string(), msg_id);
        request.set_biz_model(biz_content);
        request.set_method(AlipayMethod::Custom(
            "alipay.open.app.mini.templatemessage.query".to_string(),
        ));
        let response: AlipayResponse<QueryTemplateMessageStatusResponse> =
            self.client.request(request, None, None, None).await?;

        response.into_result()
    }

    /// 创建二维码
    pub async fn create_qrcode(
        &self,
        url_param: String,
        query_param: Option<String>,
        describe: Option<String>,
        color: Option<String>,
        size: Option<String>,
    ) -> LabradorResult<CreateQrcodeResponse> {
        let mut request = AlipayBizRequest::new();
        let mut biz_content = BTreeMap::new();
        biz_content.insert("url_param".to_string(), url_param);

        if let Some(query) = query_param {
            biz_content.insert("query_param".to_string(), query);
        }

        if let Some(desc) = describe {
            biz_content.insert("describe".to_string(), desc);
        }

        if let Some(color_code) = color {
            biz_content.insert("color".to_string(), color_code);
        }

        if let Some(size_str) = size {
            biz_content.insert("size".to_string(), size_str);
        }
        request.set_biz_model(biz_content);
        request.set_method(AlipayMethod::Custom(
            "alipay.open.app.qrcode.create".to_string(),
        ));
        let response: AlipayResponse<CreateQrcodeResponse> =
            self.client.request(request, None, None, None).await?;

        response.into_result()
    }

    /// 获取小程序摘要
    pub async fn get_summary(&self, mini_app_id: String) -> LabradorResult<GetSummaryResponse> {
        let mut request = AlipayBizRequest::new();
        let mut biz_content = BTreeMap::new();
        biz_content.insert("mini_app_id".to_string(), mini_app_id);
        request.set_biz_model(biz_content);
        request.set_method(AlipayMethod::Custom(
            "alipay.open.mini.summary.query".to_string(),
        ));
        let response: AlipayResponse<GetSummaryResponse> =
            self.client.request(request, None, None, None).await?;

        response.into_result()
    }

    /// 检查文本安全
    pub async fn check_text_security(
        &self,
        content: String,
        scene_codes: Vec<String>,
    ) -> LabradorResult<CheckTextSecurityResponse> {
        let mut request = AlipayBizRequest::new();
        let mut biz_content = BTreeMap::new();
        biz_content.insert("content".to_string(), content);

        let scene_json = serde_json::to_string(&scene_codes).unwrap_or_default();
        biz_content.insert("scene_codes".to_string(), scene_json);
        request.set_biz_model(biz_content);
        request.set_method(AlipayMethod::Custom(
            "alipay.security.risk.content.analyze".to_string(),
        ));
        let response: AlipayResponse<CheckTextSecurityResponse> =
            self.client.request(request, None, None, None).await?;

        response.into_result()
    }

    /// 检查图片安全
    pub async fn check_image_security(
        &self,
        image_urls: Vec<String>,
        scene_codes: Vec<String>,
    ) -> LabradorResult<CheckImageSecurityResponse> {
        let mut request = AlipayBizRequest::new();
        let mut biz_content = BTreeMap::new();

        let image_json = serde_json::to_string(&image_urls).unwrap_or_default();
        biz_content.insert("image_urls".to_string(), image_json);

        let scene_json = serde_json::to_string(&scene_codes).unwrap_or_default();
        biz_content.insert("scene_codes".to_string(), scene_json);
        request.set_biz_model(biz_content);
        request.set_method(AlipayMethod::Custom(
            "alipay.security.risk.content.analyze.image".to_string(),
        ));
        let response: AlipayResponse<CheckImageSecurityResponse> =
            self.client.request(request, None, None, None).await?;

        response.into_result()
    }

    /// 生成小程序scheme链接
    pub fn generate_scheme_url(
        &self,
        page_path: &str,
        query: Option<&str>,
        app_id: Option<&str>,
    ) -> String {
        let target_app_id = app_id.unwrap_or(&self.client.config().app_id);
        let query_str = query.unwrap_or("");

        format!(
            "alipays://platformapi/startapp?appId={}&page={}&query={}",
            target_app_id, page_path, query_str
        )
    }

    /// 生成小程序二维码链接
    pub fn generate_qrcode_url(
        &self,
        page_path: &str,
        query: Option<&str>,
        app_id: Option<&str>,
    ) -> String {
        let target_app_id = app_id.unwrap_or(&self.client.config().app_id);
        let query_str = query.unwrap_or("");

        format!(
            "https://render.alipay.com/p/s/i?scheme=alipays%3A%2F%2Fplatformapi%2Fstartapp%3FappId%3D{}%26page%3D{}%26query%3D{}",
            target_app_id, page_path, query_str
        )
    }
}

/// 模板数据项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateDataItem {
    /// 值
    pub value: String,
    /// 颜色
    pub color: Option<String>,
}

/// 模板列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateListResponse {
    /// 模板列表
    pub template_list: Vec<TemplateInfo>,
    /// 总数量
    pub total_count: i32,
    /// 当前页码
    pub page_num: i32,
    /// 每页数量
    pub page_size: i32,
}

/// 模板信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateInfo {
    /// 模板ID
    pub template_id: String,
    /// 模板标题
    pub template_title: String,
    /// 模板内容
    pub template_content: String,
    /// 模板类型
    pub template_type: String,
    /// 创建时间
    pub create_time: String,
}

/// 发送模板消息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTemplateMessageResponse {
    /// 消息ID
    pub msg_id: String,
    /// 发送状态
    pub send_status: String,
}

/// 查询模板消息状态响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryTemplateMessageStatusResponse {
    /// 消息ID
    pub msg_id: String,
    /// 发送状态
    pub send_status: String,
    /// 失败原因
    pub fail_reason: Option<String>,
    /// 发送时间
    pub send_time: Option<String>,
}

/// 创建二维码响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQrcodeResponse {
    /// 二维码图片地址
    pub qr_code_url: String,
    /// 二维码内容
    pub qr_code_content: String,
}

/// 获取小程序摘要响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSummaryResponse {
    /// 小程序ID
    pub mini_app_id: String,
    /// 小程序名称
    pub mini_app_name: String,
    /// 小程序描述
    pub mini_app_desc: Option<String>,
    /// 小程序图标
    pub mini_app_logo: Option<String>,
    /// 小程序分类
    pub mini_app_category: Option<String>,
    /// 小程序状态
    pub mini_app_status: String,
}

/// 检查文本安全响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckTextSecurityResponse {
    /// 处理建议
    pub action: String,
    /// 风险标签
    pub risk_labels: Vec<RiskLabel>,
    /// 唯一ID
    pub unique_id: String,
}

/// 检查图片安全响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckImageSecurityResponse {
    /// 处理结果列表
    pub result_list: Vec<ImageCheckResult>,
}

/// 风险标签
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskLabel {
    /// 风险标签
    pub label: String,
    /// 风险等级
    pub level: String,
    /// 风险子标签
    pub sub_labels: Option<Vec<SubLabel>>,
}

/// 子标签
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubLabel {
    /// 子标签
    pub label: String,
    /// 子标签描述
    pub description: String,
}

/// 图片检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageCheckResult {
    /// 图片URL
    pub image_url: String,
    /// 处理建议
    pub action: String,
    /// 风险标签
    pub risk_labels: Vec<RiskLabel>,
    /// 唯一ID
    pub unique_id: String,
}

/// 小程序场景值
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MiniappScene {
    /// 小程序首页
    Home,
    /// 小程序内页面
    Page,
    /// 分享
    Share,
}

impl MiniappScene {
    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            MiniappScene::Home => "1001",
            MiniappScene::Page => "1002",
            MiniappScene::Share => "1003",
        }
    }
}
