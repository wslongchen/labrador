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

use crate::errors::{LabraError, LabradorResult};
use crate::wechat::client::WechatApiResponse;
use crate::wechat::cp::types::{
    GROUP_ROBOT_MSG_IMAGE, GROUP_ROBOT_MSG_MARKDOWN, GROUP_ROBOT_MSG_NEWS, GROUP_ROBOT_MSG_TEXT,
};
use crate::wechat::cp::WechatCpClient;

/// 企业微信群机器人消息发送模块
///
/// 文档地址：<https://work.weixin.qq.com/help?doc_id=13376>
#[derive(Debug, Clone)]
pub struct WechatCpGroupRobot<'a> {
    client: &'a WechatCpClient,
}

#[allow(unused)]
impl<'a> WechatCpGroupRobot<'a> {
    #[inline]
    pub fn new(client: &'a WechatCpClient) -> Self {
        Self { client }
    }

    /// 获取webhook URL
    fn get_webhook_url(&self) -> LabradorResult<String> {
        self.client
            .webhook_url()
            .ok_or_else(|| LabraError::Config("请先设置webhook_url".to_string()))
            .map(|s| s.to_string())
    }

    /// 发送文本消息
    ///
    /// # 参数说明
    /// * `content` - 文本内容，最长不超过2048个字节，必须是utf8编码
    /// * `mentioned_list` - userid的列表，提醒群中的指定成员(@某个成员)，@all表示提醒所有人
    /// * `mentioned_mobile_list` - 手机号列表，提醒手机号对应的群成员
    pub async fn send_text(
        &self,
        content: &str,
        mentioned_list: Vec<String>,
        mentioned_mobile_list: Vec<String>,
    ) -> LabradorResult<WechatApiResponse> {
        self.send_text_with_url(
            &self.get_webhook_url()?,
            content,
            mentioned_list,
            mentioned_mobile_list,
        )
        .await
    }

    /// 发送文本消息（指定webhook URL）
    pub async fn send_text_with_url(
        &self,
        webhook_url: &str,
        content: &str,
        mentioned_list: Vec<String>,
        mentioned_mobile_list: Vec<String>,
    ) -> LabradorResult<WechatApiResponse> {
        let msg = RobotMessage {
            msgtype: GROUP_ROBOT_MSG_TEXT.to_string(),
            text: Some(TextMessage {
                content: content.to_string(),
                mentioned_list: if mentioned_list.is_empty() {
                    None
                } else {
                    Some(mentioned_list)
                },
                mentioned_mobile_list: if mentioned_mobile_list.is_empty() {
                    None
                } else {
                    Some(mentioned_mobile_list)
                },
            }),
            markdown: None,
            image: None,
            news: None,
            file: None,
        };

        let response: WechatApiResponse = self.client.post(webhook_url, msg).await?;
        Ok(response)
    }

    /// 发送Markdown消息
    ///
    /// # 参数说明
    /// * `content` - markdown内容，最长不超过4096个字节，必须是utf8编码
    pub async fn send_markdown(&self, content: &str) -> LabradorResult<WechatApiResponse> {
        self.send_markdown_with_url(&self.get_webhook_url()?, content)
            .await
    }

    /// 发送Markdown消息（指定webhook URL）
    pub async fn send_markdown_with_url(
        &self,
        webhook_url: &str,
        content: &str,
    ) -> LabradorResult<WechatApiResponse> {
        let msg = RobotMessage {
            msgtype: GROUP_ROBOT_MSG_MARKDOWN.to_string(),
            text: None,
            markdown: Some(MarkdownMessage {
                content: content.to_string(),
            }),
            image: None,
            news: None,
            file: None,
        };

        let response: WechatApiResponse = self.client.post(webhook_url, msg).await?;
        Ok(response)
    }

    /// 发送图片消息
    ///
    /// # 参数说明
    /// * `base64` - 图片内容的base64编码
    /// * `md5` - 图片内容（base64编码前）的md5值
    pub async fn send_image(&self, base64: &str, md5: &str) -> LabradorResult<WechatApiResponse> {
        self.send_image_with_url(&self.get_webhook_url()?, base64, md5)
            .await
    }

    /// 发送图片消息（指定webhook URL）
    pub async fn send_image_with_url(
        &self,
        webhook_url: &str,
        base64: &str,
        md5: &str,
    ) -> LabradorResult<WechatApiResponse> {
        let msg = RobotMessage {
            msgtype: GROUP_ROBOT_MSG_IMAGE.to_string(),
            text: None,
            markdown: None,
            image: Some(ImageMessage {
                base64: base64.to_string(),
                md5: md5.to_string(),
            }),
            news: None,
            file: None,
        };

        let response: WechatApiResponse = self.client.post(webhook_url, msg).await?;
        Ok(response)
    }

    /// 发送图文消息
    ///
    /// # 参数说明
    /// * `articles` - 图文消息列表，支持1到8条图文
    pub async fn send_news(
        &self,
        articles: Vec<CpNewsArticle>,
    ) -> LabradorResult<WechatApiResponse> {
        self.send_news_with_url(&self.get_webhook_url()?, articles)
            .await
    }

    /// 发送图文消息（指定webhook URL）
    pub async fn send_news_with_url(
        &self,
        webhook_url: &str,
        articles: Vec<CpNewsArticle>,
    ) -> LabradorResult<WechatApiResponse> {
        let msg = RobotMessage {
            msgtype: GROUP_ROBOT_MSG_NEWS.to_string(),
            text: None,
            markdown: None,
            image: None,
            news: Some(NewsMessage { articles }),
            file: None,
        };

        let response: WechatApiResponse = self.client.post(webhook_url, msg).await?;
        Ok(response)
    }

    /// 发送文件消息
    ///
    /// # 参数说明
    /// * `media_id` - 文件id，通过文件上传接口获取
    pub async fn send_file(&self, media_id: &str) -> LabradorResult<WechatApiResponse> {
        self.send_file_with_url(&self.get_webhook_url()?, media_id)
            .await
    }

    /// 发送文件消息（指定webhook URL）
    pub async fn send_file_with_url(
        &self,
        webhook_url: &str,
        media_id: &str,
    ) -> LabradorResult<WechatApiResponse> {
        let msg = RobotMessage {
            msgtype: "file".to_string(),
            text: None,
            markdown: None,
            image: None,
            news: None,
            file: Some(FileMessage {
                media_id: media_id.to_string(),
            }),
        };

        let response: WechatApiResponse = self.client.post(webhook_url, msg).await?;
        Ok(response)
    }
}

// ============================================================================
// 请求与响应结构体
// ============================================================================

/// 机器人消息
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct RobotMessage {
    /// 消息类型
    pub msgtype: String,
    /// 文本消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextMessage>,
    /// Markdown消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<MarkdownMessage>,
    /// 图片消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ImageMessage>,
    /// 图文消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub news: Option<NewsMessage>,
    /// 文件消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<FileMessage>,
}

/// 文本消息
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct TextMessage {
    /// 文本内容，最长不超过2048个字节
    pub content: String,
    /// userid的列表，提醒群中的指定成员
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentioned_list: Option<Vec<String>>,
    /// 手机号列表，提醒手机号对应的群成员
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentioned_mobile_list: Option<Vec<String>>,
}

/// Markdown消息
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct MarkdownMessage {
    /// markdown内容，最长不超过4096个字节
    pub content: String,
}

/// 图片消息
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ImageMessage {
    /// 图片内容的base64编码
    pub base64: String,
    /// 图片内容（base64编码前）的md5值
    pub md5: String,
}

/// 图文消息
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct NewsMessage {
    /// 图文消息列表
    pub articles: Vec<CpNewsArticle>,
}

/// 图文消息文章
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CpNewsArticle {
    /// 标题，不超过128个字节，超过会自动截断
    pub title: String,
    /// 描述，不超过512个字节，超过会自动截断
    pub description: String,
    /// 点击后跳转的链接
    pub url: String,
    /// 图文消息的图片链接，支持JPG、PNG格式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picurl: Option<String>,
    /// 按钮文字，仅在图文数为1条时才生效，默认为"阅读全文"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btn_text: Option<String>,
    /// 小程序appid，与pagepath同时填写时会忽略url字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,
    /// 点击消息卡片后的小程序页面，仅限本小程序内的页面
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagepath: Option<String>,
}

/// 文件消息
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct FileMessage {
    /// 文件id，通过文件上传接口获取
    pub media_id: String,
}
