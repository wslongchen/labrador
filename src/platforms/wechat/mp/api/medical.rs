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
use serde::Serialize;
use serde_json::Value;

use crate::errors::LabradorResult;
use crate::wechat::client::WechatApiResponse;
use crate::wechat::mp::WechatMpClient;

/// 微信就医助手模块
///
/// 提供医疗相关的消息推送功能。
#[derive(Debug, Clone)]
pub struct WechatMpMedicalAssistant<'a> {
    client: &'a WechatMpClient,
}

#[allow(unused)]
impl<'a> WechatMpMedicalAssistant<'a> {
    #[inline]
    pub fn new(client: &'a WechatMpClient) -> Self {
        Self { client }
    }

    /// 发送医疗行业消息
    ///
    /// 该接口用于向用户发送医疗相关的消息通知。
    pub async fn send_channel_msg(
        &self,
        request: &SendMedicalMsgRequest,
    ) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/cityservice/sendchannelmsg", request)
            .await?;
        Ok(response)
    }
}

// ============================================================================
// 请求与响应结构体
// ============================================================================

/// 发送医疗消息请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendMedicalMsgRequest {
    /// 接收者的openid
    pub touser: String,
    /// 小程序appid
    pub appid: String,
    /// 消息模板ID
    pub template_id: String,
    /// 点击消息跳转的小程序页面
    pub page: Option<String>,
    /// 消息数据
    pub data: Value,
    /// 消息颜色
    pub color: Option<String>,
    /// 表情符号
    pub emoji: Option<String>,
}
