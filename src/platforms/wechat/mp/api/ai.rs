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
use serde_json::json;

use crate::errors::LabradorResult;
use crate::wechat::client::WechatApiResponse;
use crate::wechat::mp::WechatMpClient;

/// AI开放接口模块
#[derive(Debug, Clone)]
pub struct WechatMpAI<'a> {
    client: &'a WechatMpClient,
}

#[allow(unused)]
impl<'a> WechatMpAI<'a> {
    #[inline]
    pub fn new(client: &'a WechatMpClient) -> Self {
        Self { client }
    }

    /// 微信翻译
    ///
    /// 本接口用于文本内容翻译。
    /// # 参数说明
    /// * `content` - 源内容（utf8格式，最大600Byte）
    /// * `lfrom` - 源语言，zh_CN 或 en_US
    /// * `lto` - 目标语言，zh_CN 或 en_US
    pub async fn translate_content(
        &self,
        content: &str,
        lfrom: &str,
        lto: &str,
    ) -> LabradorResult<TranslateResponse> {
        let request = json!({ "content": content });
        let response: WechatApiResponse<TranslateResponse> = self
            .client
            .wechat_client()
            .post(
                &format!(
                    "/cgi-bin/media/voice/translatecontent?lfrom={}&lto={}",
                    lfrom, lto
                ),
                request,
            )
            .await?;
        response.into_result()
    }

    /// 获取语音识别结果
    ///
    /// 本接口用于查询语音转文字结果。添加完文件之后10s内调用这个接口。
    /// # 参数说明
    /// * `voice_id` - 语音唯一标识
    /// * `lang` - 语言，zh_CN 或 en_US，默认中文
    pub async fn query_reco_result(
        &self,
        voice_id: &str,
        lang: Option<&str>,
    ) -> LabradorResult<String> {
        let lang = lang.unwrap_or("zh_CN");
        let response: WechatApiResponse<QueryRecoResultResponse> = self
            .client
            .wechat_client()
            .get(&format!(
                "/cgi-bin/media/voice/queryrecoresultfortext?voice_id={}&lang={}",
                voice_id, lang
            ))
            .await?;
        Ok(response.into_result()?.result)
    }
}

/// 翻译响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslateResponse {
    /// 原文内容
    pub from_content: String,
    /// 译文内容
    pub to_content: String,
}

/// 语音识别响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct QueryRecoResultResponse {
    /// 识别结果
    pub result: String,
}
