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
use crate::wechat::miniapp::WechatMiniAppClient;

/// 生物认证与人脸核身模块
///
/// 包含 SOTER 生物认证秘钥签名验证，以及微信人脸核身服务的相关接口。
#[derive(Debug, Clone)]
pub struct WechatMxaAuth<'a> {
    client: &'a WechatMiniAppClient,
}

#[allow(unused)]
impl<'a> WechatMxaAuth<'a> {
    #[inline]
    pub fn new(client: &'a WechatMiniAppClient) -> Self {
        Self { client }
    }

    // ========================= 生物认证 (SOTER) =========================

    /// 生物认证秘钥签名验证
    ///
    /// 本接口用于 SOTER 生物认证秘钥签名验证。
    /// 通过小程序前端获取到的签名参数，在此接口进行验证。
    ///
    /// # 参数
    /// * `request` - 签名验证请求参数，包含 openid（用户唯一标识）、
    ///   json_string（通过 wx.startSoterAuthentication 获得的JSON字符串）和
    ///   json_signature（通过 wx.startSoterAuthentication 获得的签名）
    ///
    /// # 返回
    /// 成功返回空响应，表示验证通过。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/basic-info/soter/verifySignature.html>
    pub async fn verify_signature(
        &self,
        request: &VerifySignatureRequest,
    ) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/cgi-bin/soter/verify_signature", request)
            .await?;
        Ok(response)
    }

    // ========================= 微信人脸核身 =========================

    /// 获取用户人脸核身会话唯一标识
    ///
    /// 业务方后台根据「用户实名信息（姓名+身份证）」调用本接口，
    /// 获取人脸核身会话唯一标识 `verify_id`，然后给到小程序前端使用。
    ///
    /// # 参数
    /// * `request` - 请求参数，包含姓名（name）、身份证号（id_card_number），
    ///   可选参数：业务流水号（biz_id）和回调通知地址（notify_url）
    ///
    /// # 返回
    /// 返回 `LabradorResult<GetVerifyIdResponse>`，包含 `verify_id`（有效期10分钟，一次有效）
    /// 和 request_id。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/basic-info/face-identify/getFaceIdentifySession.html>
    pub async fn get_verify_id(
        &self,
        request: &GetVerifyIdRequest,
    ) -> LabradorResult<GetVerifyIdResponse> {
        let response: WechatApiResponse<GetVerifyIdResponse> = self
            .client
            .wechat_client()
            .post("/cityservice/face/identify/getverifyid", request)
            .await?;
        response.into_result()
    }

    /// 查询用户人脸核身真实验证结果
    ///
    /// 业务方后台根据人脸核身会话唯一标识 `verify_id` 调用本接口，
    /// 查询用户人脸核身真实验证结果。
    ///
    /// # 参数
    /// * `request` - 请求参数，包含 `verify_id`（人脸核身会话唯一标识，通过 getVerifyId 获得）
    ///
    /// # 返回
    /// 返回 `LabradorResult<QueryVerifyInfoResponse>`，包含核身结果（result：0-通过）、
    /// 核身凭证（verify_ticket）和脱敏的用户实名信息（verify_info）。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/basic-info/face-identify/queryFaceIdentifyResult.html>
    pub async fn query_verify_info(
        &self,
        request: &QueryVerifyInfoRequest,
    ) -> LabradorResult<QueryVerifyInfoResponse> {
        let response: WechatApiResponse<QueryVerifyInfoResponse> = self
            .client
            .wechat_client()
            .post("/cityservice/face/identify/queryverifyinfo", request)
            .await?;
        response.into_result()
    }
}

// ============================================================================
// 请求与响应结构体
// 注意：使用 #[serde(rename_all = "camelCase")] 处理驼峰字段，符合 Rust 命名规范。
// ============================================================================

// -------------------- 生物认证 (SOTER) 结构体 --------------------

/// 生物认证签名验证请求参数
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifySignatureRequest {
    /// 用户唯一标识，即用户的 openid
    pub openid: String,
    /// 通过小程序前端 wx.startSoterAuthentication 获得的 json 字符串
    pub json_string: String,
    /// 通过小程序前端 wx.startSoterAuthentication 获得的签名
    pub json_signature: String,
}

// -------------------- 微信人脸核身 结构体 --------------------

/// 获取人脸核身会话标识请求参数
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetVerifyIdRequest {
    /// 用户姓名
    pub name: String,
    /// 用户身份证号码
    pub id_card_number: String,
    /// 业务方自定义的流水号，可用于后续查询或对账
    pub biz_id: Option<String>,
    /// 回调通知地址，人脸核身完成后，微信服务器会向该地址发送通知
    pub notify_url: Option<String>,
}

/// 获取人脸核身会话标识响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetVerifyIdResponse {
    /// 人脸核身会话唯一标识，有效期10分钟，一次有效
    pub verify_id: String,
    /// 该次请求的唯一标识，可用于问题排查
    pub request_id: String,
}

/// 查询人脸核身结果请求参数
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryVerifyInfoRequest {
    /// 人脸核身会话唯一标识，通过 getVerifyId 接口获得
    pub verify_id: String,
}

/// 查询人脸核身结果响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryVerifyInfoResponse {
    /// 核身结果：0-验证通过，其他-验证失败
    pub result: i32,
    /// 人脸核身完成的凭证，可用于后续业务操作
    pub verify_ticket: Option<String>,
    /// 当核身成功时，返回用户的实名信息（脱敏）
    pub verify_info: Option<VerifyInfo>,
    /// 该次请求的唯一标识，可用于问题排查
    pub request_id: String,
}

/// 核身成功返回的用户实名信息（脱敏）
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyInfo {
    /// 用户真实姓名（脱敏，如：张*）
    pub name: String,
    /// 用户身份证号码（脱敏，如：1101*******1234）
    pub id_card_number: String,
}
