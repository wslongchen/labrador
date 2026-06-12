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

use crate::errors::LabradorResult;
use crate::wechat::client::WechatApiResponse;
use crate::wechat::mp::WechatMpClient;

/// 自动回复服务接口
#[derive(Debug, Clone)]
pub struct WechatMpAutoReply<'a> {
    client: &'a WechatMpClient,
}

#[allow(unused)]
impl<'a> WechatMpAutoReply<'a> {
    #[inline]
    pub fn new(client: &'a WechatMpClient) -> Self {
        Self { client }
    }

    /// 获取公众号的自动回复规则
    ///
    /// 获取公众号当前使用的自动回复规则，包括关注后自动回复、消息自动回复、关键词自动回复。
    /// 本接口仅能获取公众号在公众平台官网的自动回复功能中设置的规则。
    pub async fn get_current_autoreply_info(&self) -> LabradorResult<AutoReplyInfo> {
        let response: WechatApiResponse<AutoReplyInfo> = self
            .client
            .wechat_client()
            .get("/cgi-bin/get_current_autoreply_info")
            .await?;
        response.into_result()
    }
}

// ============================================================================
// 响应结构体
// ============================================================================

/// 自动回复规则响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AutoReplyInfo {
    /// 关注后自动回复是否开启，0-未开启，1-开启
    pub is_add_friend_reply_open: i32,
    /// 消息自动回复是否开启，0-未开启，1-开启
    pub is_autoreply_open: i32,
    /// 关注后自动回复的信息
    pub add_friend_autoreply_info: Option<ReplyRule>,
    /// 消息自动回复的信息
    pub message_default_autoreply_info: Option<ReplyRule>,
    /// 关键词自动回复的信息
    pub keyword_autoreply_info: Option<KeywordReplyInfo>,
}

/// 自动回复规则（关注后/消息默认）
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReplyRule {
    /// 自动回复的类型：text, img, voice, video, news
    pub r#type: String,
    /// 对于文本类型，content是文本内容；对于图文、图片、语音、视频类型，content是mediaID
    pub content: String,
    /// 图文消息的信息（当type为news时返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub news_info: Option<NewsInfo>,
}

/// 关键词自动回复信息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KeywordReplyInfo {
    /// 自动回复列表
    pub list: Vec<KeywordRule>,
}

/// 关键词规则
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KeywordRule {
    /// 规则名称
    pub rule_name: String,
    /// 创建时间
    pub create_time: i64,
    /// 回复模式：reply_all（全部回复），random_one（随机回复其中一条）
    pub reply_mode: String,
    /// 匹配的关键词列表
    pub keyword_list_info: Vec<KeywordInfo>,
    /// 回复列表
    pub reply_list_info: Vec<WechatReplyInfo>,
}

/// 关键词信息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KeywordInfo {
    /// 关键词类型
    pub r#type: String,
    /// 匹配模式：contain（包含），equal（严格相同）
    pub match_mode: String,
    /// 关键词内容
    pub content: String,
}

/// 回复信息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WechatReplyInfo {
    /// 回复类型
    pub r#type: String,
    /// 对于文本类型，content是文本内容；对于图文、图片、语音、视频类型，content是mediaID
    pub content: String,
    /// 图文消息的信息（当type为news时返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub news_info: Option<NewsInfo>,
}

/// 图文消息信息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NewsInfo {
    /// 图文消息列表
    pub list: Vec<NewsItem>,
}

/// 图文消息项
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NewsItem {
    /// 图文消息的标题
    pub title: String,
    /// 摘要
    pub digest: String,
    /// 作者
    pub author: String,
    /// 是否显示封面，0为不显示，1为显示
    pub show_cover: i32,
    /// 封面图片的URL
    pub cover_url: String,
    /// 正文的URL
    pub content_url: String,
    /// 原文的URL，若置空则无查看原文入口
    pub source_url: String,
}
