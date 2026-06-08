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

use serde::Deserialize;
use serde_json::json;

use crate::errors::LabradorResult;
use crate::wechat::client::WechatApiResponse;
use crate::wechat::mp::WechatMpClient;

/// 发布能力模块
#[derive(Debug, Clone)]
pub struct WechatMpFreePublish<'a> {
    client: &'a WechatMpClient,
}

#[allow(unused)]
impl<'a> WechatMpFreePublish<'a> {
    #[inline]
    pub fn new(client: &'a WechatMpClient) -> Self {
        Self { client }
    }

    /// 提交发布任务
    ///
    /// 开发者需要先将图文素材以草稿的形式保存（见"草稿箱-新建草稿"），选择要发布的草稿media_id，提交发布任务。
    pub async fn submit(&self, media_id: i64) -> LabradorResult<SubmitPublishResponse> {
        let request = json!({ "media_id": media_id });
        let response: WechatApiResponse<SubmitPublishResponse> = self
            .client
            .wechat_client()
            .post("/cgi-bin/freepublish/submit", request)
            .await?;
        response.into_result()
    }

    /// 发布任务状态查询
    ///
    /// 用于获取发布任务的状态和结果。
    pub async fn get(&self, publish_id: &str) -> LabradorResult<PublishInfo> {
        let request = json!({ "publish_id": publish_id });
        let response: WechatApiResponse<PublishInfo> = self
            .client
            .wechat_client()
            .post("/cgi-bin/freepublish/get", request)
            .await?;
        response.into_result()
    }

    /// 删除发布
    ///
    /// 发布成功之后，用来删除永久图文素材。
    pub async fn delete(
        &self,
        article_id: &str,
        index: Option<u32>,
    ) -> LabradorResult<WechatApiResponse> {
        let mut request = json!({ "article_id": article_id });
        if let Some(idx) = index {
            request["index"] = json!(idx);
        }
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/cgi-bin/freepublish/delete", request)
            .await?;
        Ok(response)
    }

    /// 通过 article_id 获取已发布文章
    ///
    /// 通过 article_id 获取已发布文章，包括已发布时设置的阅读原文等。
    pub async fn get_article(&self, article_id: &str) -> LabradorResult<PublishArticle> {
        let request = json!({ "article_id": article_id });
        let response: WechatApiResponse<PublishArticle> = self
            .client
            .wechat_client()
            .post("/cgi-bin/freepublish/getarticle", request)
            .await?;
        response.into_result()
    }

    /// 获取成功发布列表
    ///
    /// 获取成功发布列表。
    pub async fn batch_get(
        &self,
        offset: u32,
        count: u32,
        no_content: Option<u32>,
    ) -> LabradorResult<BatchPublishResponse> {
        let mut request = json!({
            "offset": offset,
            "count": count
        });
        if let Some(nc) = no_content {
            request["no_content"] = json!(nc);
        }
        let response: WechatApiResponse<BatchPublishResponse> = self
            .client
            .wechat_client()
            .post("/cgi-bin/freepublish/batchget", request)
            .await?;
        response.into_result()
    }
}

/// 提交发布响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitPublishResponse {
    /// 发布任务id
    pub publish_id: String,
}

/// 发布信息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishInfo {
    /// 发布任务id
    pub publish_id: String,
    /// 发布状态：0-成功，1-发布中，2-原创失败，3-发布失败，4-部分失败
    pub status: u32,
    /// 发布任务失败原因
    pub fail_idx: Option<Vec<u32>>,
    /// 当status=0时，返回的文章列表
    pub article_id_list: Option<Vec<String>>,
}

/// 已发布文章
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishArticle {
    /// 标题
    pub title: String,
    /// 作者
    pub author: String,
    /// 图文消息的摘要，仅有单图文消息才有摘要，多图文此处为空
    pub digest: String,
    /// 图文消息的具体内容，支持HTML标签，必须少于2万字符
    pub content: String,
    /// 图文消息的原文地址，即点击“阅读原文”后的URL
    pub content_source_url: String,
    /// 图文消息的封面图片素材id
    pub thumb_media_id: String,
    /// 是否显示封面，0为false，即不显示，1为true，即显示
    pub show_cover_pic: u32,
    /// 草稿的临时素材
    pub url: String,
    /// 是否开启评论，0不开启，1开启
    pub need_open_comment: u32,
    /// 是否粉丝才可评论，0所有人可评论，1粉丝才可评论
    pub only_fans_can_comment: u32,
}

/// 批量获取发布列表响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchPublishResponse {
    /// 发布记录总数
    pub total_count: u32,
    /// 发布记录列表
    pub item: Vec<PublishItem>,
}

/// 发布记录项
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishItem {
    /// 发布记录id
    pub article_id: String,
    /// 发布时间
    pub update_time: i64,
    /// 发布内容
    pub content: PublishContent,
}

/// 发布内容
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishContent {
    /// 文章列表
    pub news_item: Vec<PublishArticle>,
}
