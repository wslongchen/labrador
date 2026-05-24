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
use crate::wechat::mp::WechatMpClient;

/// 留言管理模块
#[derive(Debug, Clone)]
pub struct WechatMpComment<'a> {
    client: &'a WechatMpClient,
}

#[allow(unused)]
impl<'a> WechatMpComment<'a> {
    #[inline]
    pub fn new(client: &'a WechatMpClient) -> Self {
        Self { client }
    }

    /// 打开已群发文章评论
    ///
    /// 本接口用于打开已群发图文的评论功能，公众号需具备留言功能权限。
    pub async fn open_comment(&self, msg_data_id: i64, index: Option<u32>) -> LabradorResult<WechatApiResponse> {
        let mut request = json!({ "msg_data_id": msg_data_id });
        if let Some(idx) = index {
            request["index"] = json!(idx);
        }
        let response: WechatApiResponse = self.client.wechat_client()
            .post("/cgi-bin/comment/open", request)
            .await?;
        Ok(response)
    }

    /// 关闭已群发文章评论
    ///
    /// 本接口用于关闭已群发文章评论。
    pub async fn close_comment(&self, msg_data_id: i64, index: Option<u32>) -> LabradorResult<WechatApiResponse> {
        let mut request = json!({ "msg_data_id": msg_data_id });
        if let Some(idx) = index {
            request["index"] = json!(idx);
        }
        let response: WechatApiResponse = self.client.wechat_client()
            .post("/cgi-bin/comment/close", request)
            .await?;
        Ok(response)
    }

    /// 查看指定文章的评论数据
    ///
    /// 本接口用于查看指定文章的评论数据。
    /// type: 0-普通评论&精选评论，1-普通评论，2-精选评论
    pub async fn list_comment(&self, request: &ListCommentRequest) -> LabradorResult<ListCommentResponse> {
        let response: WechatApiResponse<ListCommentResponse> = self.client.wechat_client()
            .post("/cgi-bin/comment/list", request)
            .await?;
        response.into_result()
    }

    /// 评论标记精选
    ///
    /// 本接口用于将评论标记为精选。
    pub async fn mark_elect(&self, msg_data_id: i64, user_comment_id: i64, index: Option<u32>) -> LabradorResult<WechatApiResponse> {
        let mut request = json!({
            "msg_data_id": msg_data_id,
            "user_comment_id": user_comment_id
        });
        if let Some(idx) = index {
            request["index"] = json!(idx);
        }
        let response: WechatApiResponse = self.client.wechat_client()
            .post("/cgi-bin/comment/markelect", request)
            .await?;
        Ok(response)
    }

    /// 取消标记精选
    ///
    /// 本接口用于将评论取消精选标记。
    pub async fn unmark_elect(&self, msg_data_id: i64, user_comment_id: i64, index: Option<u32>) -> LabradorResult<WechatApiResponse> {
        let mut request = json!({
            "msg_data_id": msg_data_id,
            "user_comment_id": user_comment_id
        });
        if let Some(idx) = index {
            request["index"] = json!(idx);
        }
        let response: WechatApiResponse = self.client.wechat_client()
            .post("/cgi-bin/comment/unmarkelect", request)
            .await?;
        Ok(response)
    }

    /// 删除评论
    ///
    /// 本接口用于删除评论。
    pub async fn delete_comment(&self, msg_data_id: i64, user_comment_id: i64, index: Option<u32>) -> LabradorResult<WechatApiResponse> {
        let mut request = json!({
            "msg_data_id": msg_data_id,
            "user_comment_id": user_comment_id
        });
        if let Some(idx) = index {
            request["index"] = json!(idx);
        }
        let response: WechatApiResponse = self.client.wechat_client()
            .post("/cgi-bin/comment/delete", request)
            .await?;
        Ok(response)
    }

    /// 回复评论
    ///
    /// 本接口用于回复评论。
    pub async fn reply_comment(&self, msg_data_id: i64, user_comment_id: i64, content: &str, index: Option<u32>) -> LabradorResult<WechatApiResponse> {
        let mut request = json!({
            "msg_data_id": msg_data_id,
            "user_comment_id": user_comment_id,
            "content": content
        });
        if let Some(idx) = index {
            request["index"] = json!(idx);
        }
        let response: WechatApiResponse = self.client.wechat_client()
            .post("/cgi-bin/comment/reply/add", request)
            .await?;
        Ok(response)
    }

    /// 删除回复
    ///
    /// 本接口用于删除评论回复。
    pub async fn delete_reply(&self, msg_data_id: i64, user_comment_id: i64, index: Option<u32>) -> LabradorResult<WechatApiResponse> {
        let mut request = json!({
            "msg_data_id": msg_data_id,
            "user_comment_id": user_comment_id
        });
        if let Some(idx) = index {
            request["index"] = json!(idx);
        }
        let response: WechatApiResponse = self.client.wechat_client()
            .post("/cgi-bin/comment/reply/delete", request)
            .await?;
        Ok(response)
    }
}

/// 查看评论请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListCommentRequest {
    /// 群发返回的msg_data_id
    pub msg_data_id: i64,
    /// 多图文时，用来指定第几篇图文，从0开始
    pub index: Option<u32>,
    /// 起始位置
    pub begin: u32,
    /// 获取数目（<=50）
    pub count: u32,
    /// type=0 普通评论&精选评论 type=1 普通评论 type=2 精选评论
    pub r#type: u32,
}

/// 查看评论响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListCommentResponse {
    /// 评论列表
    pub comment: Vec<CommentInfo>,
    /// 评论总数
    pub total: u32,
}

/// 评论信息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentInfo {
    /// 用户评论id
    pub user_comment_id: i64,
    /// 评论时间
    pub create_time: i64,
    /// 评论内容
    pub content: String,
    /// 是否精选评论，0为非精选，1为精选
    pub comment_type: u32,
    /// openid，用户如果用非微信身份评论，不返回openid
    pub openid: Option<String>,
    /// 回复信息
    pub reply: Option<CommentReplyInfo>,
}

/// 回复信息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentReplyInfo {
    /// 回复内容
    pub content: String,
    /// 回复时间
    pub create_time: i64,
}
