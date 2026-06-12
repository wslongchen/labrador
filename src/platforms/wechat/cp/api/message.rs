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

use crate::errors::LabradorResult;
use crate::wechat::client::WechatApiResponse;
use crate::wechat::cp::api::CpNewsArticle;
use crate::wechat::cp::WechatCpClient;

/// 企业微信消息发送模块
///
/// 包含应用消息发送、互联企业消息发送、学校通知发送、消息统计等功能。
#[derive(Debug, Clone)]
pub struct WechatCpMessage<'a> {
    client: &'a WechatCpClient,
}

#[allow(unused)]
impl<'a> WechatCpMessage<'a> {
    #[inline]
    pub fn new(client: &'a WechatCpClient) -> Self {
        Self { client }
    }

    /// 发送应用消息
    ///
    /// 企业可通过此接口发送应用消息给成员、部门或标签。支持文本、图片、语音、视频、文件、文本卡片、图文、模板卡片等多种消息类型。
    ///
    /// # 参数
    /// * `req` - 消息请求，包含接收人（touser/toparty/totag）、消息类型（msgtype）、消息体等
    ///
    /// # 返回
    /// 返回 `LabradorResult<MessageResponse>`，包含消息 id 及无效的接收者列表
    ///
    /// # 官方文档
    /// <https://developer.work.weixin.qq.com/document/path/90236>
    pub async fn send(&self, mut req: MessageRequest) -> LabradorResult<MessageResponse> {
        if req.agentid.is_none() {
            req.agentid = self.client.agent_id();
        }
        let response: WechatApiResponse<MessageResponse> =
            self.client.post("/cgi-bin/message/send", req).await?;
        response.into_result()
    }

    /// 发送互联企业消息
    ///
    /// 互联企业的应用支持推送文本、图片、视频、文件、图文等类型。
    ///
    /// # 参数
    /// * `req` - 互联企业消息请求，包含接收人、消息类型、消息体等
    ///
    /// # 返回
    /// 返回 `LabradorResult<LinkedCorpMessageResponse>`，包含无效的接收者列表
    ///
    /// # 官方文档
    /// <https://developer.work.weixin.qq.com/document/path/90250>
    pub async fn send_linked_corp(
        &self,
        mut req: LinkedCorpMessageRequest,
    ) -> LabradorResult<LinkedCorpMessageResponse> {
        if req.agentid.is_none() {
            req.agentid = self.client.agent_id();
        }
        let response: WechatApiResponse<LinkedCorpMessageResponse> = self
            .client
            .post("/cgi-bin/linkedcorp/message/send", req)
            .await?;
        response.into_result()
    }

    /// 发送学校通知
    ///
    /// 学校可以通过此接口来给家长发送不同类型的学校通知。
    ///
    /// # 参数
    /// * `req` - 学校通知消息请求，包含接收范围、家长/学生 userid、消息类型等
    ///
    /// # 返回
    /// 返回 `LabradorResult<SchoolContactMessageResponse>`，包含无效的接收者列表
    ///
    /// # 官方文档
    /// <https://developer.work.weixin.qq.com/document/path/92321>
    pub async fn send_school_contact(
        &self,
        mut req: SchoolContactMessageRequest,
    ) -> LabradorResult<SchoolContactMessageResponse> {
        if req.agentid.is_none() {
            req.agentid = self.client.agent_id();
        }
        let response: WechatApiResponse<SchoolContactMessageResponse> = self
            .client
            .post("/cgi-bin/externalcontact/send_school_contact_message", req)
            .await?;
        response.into_result()
    }

    /// 查询应用消息发送统计
    ///
    /// 查询指定时间范围内应用消息的发送统计数据。
    ///
    /// # 参数
    /// * `req` - 统计请求，包含应用 id 列表、开始时间和结束时间
    ///
    /// # 返回
    /// 返回 `LabradorResult<Vec<MessageStatistic>>`，包含各应用的消息发送成功人次
    ///
    /// # 官方文档
    /// <https://developer.work.weixin.qq.com/document/path/92369>
    pub async fn get_statistics(
        &self,
        req: StatisticsRequest,
    ) -> LabradorResult<Vec<MessageStatistic>> {
        let response: WechatApiResponse<StatisticsResponse> = self
            .client
            .post("/cgi-bin/message/get_statistics", req)
            .await?;
        Ok(response.into_result()?.statistics)
    }
}

// ============================================================================
// 请求与响应结构体
// ============================================================================

/// 应用消息请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageRequest {
    /// 成员ID列表（消息接收者，多个接收者用‘|’分隔，最多支持1000个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub touser: Option<String>,
    /// 部门ID列表，多个接收者用‘|’分隔，最多支持100个
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toparty: Option<String>,
    /// 标签ID列表，多个接收者用‘|’分隔，最多支持100个
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totag: Option<String>,
    /// 企业应用的id，整型。可在应用的设置页面查看
    pub agentid: Option<i32>,
    /// 消息类型
    pub msgtype: String,
    /// 表示是否是保密消息，0表示否，1表示是，默认0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe: Option<i32>,
    /// 表示是否开启id转译，0表示否，1表示是，默认0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_id_trans: Option<i32>,
    /// 表示是否开启重复消息检查，0表示否，1表示是，默认0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_duplicate_check: Option<i32>,
    /// 表示是否重复消息检查的时间间隔，默认1800s，最大不超过4小时
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_check_interval: Option<i32>,

    // 文本消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextMessage>,

    // 图片消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<MediaMessage>,

    // 语音消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<MediaMessage>,

    // 视频消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<VideoMessage>,

    // 文件消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<MediaMessage>,

    // 文本卡片消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub textcard: Option<TextCardMessage>,

    // 图文消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub news: Option<NewsMessage>,

    // 图文消息（mpnews）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpnews: Option<MpNewsMessage>,

    // 模板卡片消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_card: Option<TemplateCardMessage>,

    // 任务卡片消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taskcard: Option<TaskCardMessage>,

    // 小程序通知消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miniprogram_notice: Option<MiniProgramNoticeMessage>,
}

/// 文本消息
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextMessage {
    /// 消息内容，最长不超过2048个字节
    pub content: String,
}

/// 媒体消息（图片、语音、文件）
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaMessage {
    /// 媒体文件id，可以调用上传临时素材接口获取
    pub media_id: String,
}

/// 视频消息
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoMessage {
    /// 视频媒体文件id，可以调用上传临时素材接口获取
    pub media_id: String,
    /// 视频消息的标题，不超过128个字节
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 视频消息的描述，不超过512个字节
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 文本卡片消息
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextCardMessage {
    /// 标题，不超过128个字节
    pub title: String,
    /// 描述，不超过512个字节
    pub description: String,
    /// 点击后跳转的链接
    pub url: String,
    /// 按钮文字，默认为“详情”
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btntxt: Option<String>,
}

/// 图文消息
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewsMessage {
    /// 图文消息列表，支持1-8条图文
    pub articles: Vec<CpNewsArticle>,
}

/// 图文消息（mpnews）
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MpNewsMessage {
    /// 图文消息列表，支持1-8条图文
    pub articles: Vec<MpNewsArticle>,
}

/// 图文消息文章（mpnews专用）
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MpNewsArticle {
    /// 标题，不超过128个字节
    pub title: String,
    /// 图文消息缩略图的media_id
    pub thumb_media_id: String,
    /// 图文消息的作者，不超过64个字节
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// 图文消息的摘要，不超过512个字节
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// 图文消息的内容，支持html标签，不超过666K个字节
    pub content: String,
    /// 图文消息的原文地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_source_url: Option<String>,
    /// 是否显示封面，0不显示，1显示
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_cover_pic: Option<i32>,
}

/// 模板卡片消息
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateCardMessage {
    /// 模板卡片类型
    pub card_type: String,
    /// 卡片来源样式信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<TemplateCardSource>,
    /// 卡片主体样式信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_title: Option<TemplateCardMainTitle>,
    /// 关键数据样式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emphasis_content: Option<TemplateCardEmphasisContent>,
    /// 卡片二级垂直内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_title_text: Option<String>,
    /// 二级标题+文本列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_content_list: Option<Vec<HorizontalContent>>,
    /// 跳转指引样式的列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jump_list: Option<Vec<TemplateCardJump>>,
    /// 卡片二级垂直内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_content_list: Option<Vec<VerticalContent>>,
    /// 整体卡片的点击跳转事件
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_action: Option<TemplateCardAction>,
    /// 按钮列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_list: Option<Vec<TemplateCardButton>>,
    /// 选择题key值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkbox_question_key: Option<String>,
    /// 选择题模式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkbox_mode: Option<i32>,
    /// 选项list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_list: Option<Vec<CheckboxOption>>,
    /// 提交按钮
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_button: Option<TemplateCardSubmitButton>,
    /// 下拉式的选择器列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_list: Option<Vec<TemplateCardSelect>>,
    /// 引用文献样式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_area: Option<QuoteArea>,
}

/// 卡片来源样式
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateCardSource {
    /// 来源图片的url
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// 来源图片的描述，建议不超过20个字
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// 来源文字的颜色，0灰色，1黑色，2红色，3绿色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc_color: Option<i32>,
}

/// 卡片主体标题
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateCardMainTitle {
    /// 一级标题，建议不超过36个字
    pub title: String,
    /// 标题辅助信息，建议不超过44个字
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
}

/// 关键数据样式
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateCardEmphasisContent {
    /// 关键数据样式的数据内容，建议不超过14个字
    pub title: String,
    /// 关键数据样式的数据描述内容，建议不超过22个字
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
}

/// 二级标题+文本列表项
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalContent {
    /// 链接类型，0不是链接，1跳转url，2下载附件，3@成员
    pub r#type: i32,
    /// 二级标题，建议不超过5个字
    pub keyname: String,
    /// 二级文本
    pub value: String,
    /// 链接跳转的url
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 附件的media_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_id: Option<String>,
    /// 成员详情的userid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userid: Option<String>,
}

/// 卡片跳转指引
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateCardJump {
    /// 跳转链接类型，1跳转url，2跳转小程序
    pub r#type: i32,
    /// 跳转链接样式的文案内容
    pub title: String,
    /// 跳转链接的url
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 跳转链接的小程序的appid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,
    /// 跳转链接的小程序的pagepath
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagepath: Option<String>,
}

/// 卡片按钮
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateCardButton {
    /// 按钮文案
    pub text: String,
    /// 按钮样式
    pub style: i32,
    /// 按钮key值
    pub key: String,
}

/// 卡片提交按钮
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateCardSubmitButton {
    /// 按钮文案
    pub text: String,
    /// 按钮key值
    pub key: String,
    /// 替换文案
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_text: Option<String>,
}

/// 卡片选择器
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateCardSelect {
    /// 选择器key
    pub question_key: String,
    /// 选择器标题
    pub title: String,
    /// 默认选定的id
    pub selected_id: String,
    /// 选项列表
    pub option_list: Vec<CheckboxOption>,
}

/// 复选框选项
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckboxOption {
    /// 选项id
    pub id: String,
    /// 选项文案描述
    pub text: String,
    /// 是否默认选中
    pub is_checked: bool,
}

/// 引用文献样式
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteArea {
    /// 点击事件类型，0无事件，1跳转url，2跳转小程序
    pub r#type: i32,
    /// 点击跳转的url
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 点击跳转的小程序的appid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,
    /// 点击跳转的小程序的pagepath
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagepath: Option<String>,
    /// 引用文献样式的标题
    pub title: String,
    /// 引用文献样式的引用文案
    pub quote_text: String,
}

/// 卡片整体点击事件
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateCardAction {
    /// 跳转事件类型，1跳转url，2打开小程序
    pub r#type: i32,
    /// 跳转事件的url
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 跳转事件的小程序的appid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,
    /// 跳转事件的小程序的pagepath
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagepath: Option<String>,
}

/// 二级垂直内容
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerticalContent {
    /// 卡片二级标题
    pub title: String,
    /// 二级普通文本
    pub desc: String,
}

/// 任务卡片消息
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskCardMessage {
    /// 任务id
    pub task_id: String,
    /// 任务标题
    pub title: String,
    /// 任务描述
    pub description: String,
    /// 点击后跳转的链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 按钮列表
    pub btn: Vec<TaskCardButton>,
}

/// 任务卡片按钮
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskCardButton {
    /// 按钮key值
    pub key: String,
    /// 按钮名称
    pub name: String,
    /// 按钮颜色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// 是否加粗
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bold: Option<bool>,
}

/// 小程序通知消息
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MiniProgramNoticeMessage {
    /// 小程序appid
    pub appid: String,
    /// 小程序页面路径
    pub page: String,
    /// 消息标题
    pub title: String,
    /// 消息描述
    pub description: String,
    /// 消息内容
    pub content_item: Vec<MiniProgramContentItem>,
    /// 是否强调第一个content_item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emphasis_first_item: Option<bool>,
}

/// 小程序通知内容项
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MiniProgramContentItem {
    /// 内容项key
    pub key: String,
    /// 内容项value
    pub value: String,
}

/// 消息发送响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageResponse {
    /// 无效的成员
    pub invaliduser: Option<String>,
    /// 无效的部门
    pub invalidparty: Option<String>,
    /// 无效的标签
    pub invalidtag: Option<String>,
    /// 消息id
    pub msgid: Option<String>,
}

/// 互联企业消息请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkedCorpMessageRequest {
    /// 成员ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub touser: Option<Vec<String>>,
    /// 部门ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toparty: Option<Vec<String>>,
    /// 标签ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totag: Option<Vec<String>>,
    /// 是否发送给所有人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toall: Option<i32>,
    /// 企业应用的id
    pub agentid: Option<i32>,
    /// 消息类型
    pub msgtype: String,
    /// 消息内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextMessage>,
    /// 图片消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<MediaMessage>,
    /// 视频消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<VideoMessage>,
    /// 文件消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<MediaMessage>,
    /// 图文消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub news: Option<NewsMessage>,
    /// 图文消息（mpnews）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpnews: Option<MpNewsMessage>,
    /// 表示是否是保密消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe: Option<i32>,
}

/// 互联企业消息响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkedCorpMessageResponse {
    /// 无效的成员
    pub invaliduser: Option<Vec<String>>,
    /// 无效的部门
    pub invalidparty: Option<Vec<String>>,
    /// 无效的标签
    pub invalidtag: Option<Vec<String>>,
}

/// 学校通知消息请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SchoolContactMessageRequest {
    /// 接收范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_scope: Option<i32>,
    /// 家长userid列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_parent_userid: Option<Vec<String>>,
    /// 学生userid列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_student_userid: Option<Vec<String>>,
    /// 部门列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_party: Option<Vec<String>>,
    /// 是否发送给所有人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_all: Option<i32>,
    /// 企业应用的id
    pub agentid: Option<i32>,
    /// 消息类型
    pub msgtype: String,
    /// 消息内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextMessage>,
    /// 图片消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<MediaMessage>,
    /// 语音消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<MediaMessage>,
    /// 视频消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<VideoMessage>,
    /// 文件消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<MediaMessage>,
    /// 图文消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub news: Option<NewsMessage>,
    /// 图文消息（mpnews）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpnews: Option<MpNewsMessage>,
    /// 是否开启id转译
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_id_trans: Option<i32>,
    /// 是否开启重复消息检查
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_duplicate_check: Option<i32>,
    /// 重复消息检查的时间间隔
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_check_interval: Option<i32>,
}

/// 学校通知消息响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchoolContactMessageResponse {
    /// 无效的家长
    pub invalid_parent_userid: Option<Vec<String>>,
    /// 无效的学生
    pub invalid_student_userid: Option<Vec<String>>,
    /// 无效的部门
    pub invalid_party: Option<Vec<String>>,
}

/// 消息统计请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatisticsRequest {
    /// 应用id列表
    pub agentid_list: Vec<i32>,
    /// 开始时间
    pub start_time: i64,
    /// 结束时间
    pub end_time: i64,
}

/// 消息统计响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StatisticsResponse {
    /// 统计数据列表
    pub statistics: Vec<MessageStatistic>,
}

/// 消息统计
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageStatistic {
    /// 应用名
    pub app_name: String,
    /// 应用id
    pub agentid: i32,
    /// 发消息成功人次
    pub count: i32,
}
