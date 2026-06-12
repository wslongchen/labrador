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
use crate::errors::{LabraError, LabradorResult};
use crate::wechat::client::WechatApiResponse;
use crate::wechat::mp::types::{QR_LIMIT_SCENE, QR_LIMIT_STR_SCENE, QR_SCENE, QR_STR_SCENE};
use crate::wechat::mp::WechatMpClient;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// 服务号二维码管理模块
///
/// 包含：临时/永久二维码生成、二维码跳转规则、短链接生成
#[derive(Debug, Clone)]
pub struct WechatMpQRCode<'a> {
    client: &'a WechatMpClient,
}

#[allow(unused)]
impl<'a> WechatMpQRCode<'a> {
    #[inline]
    pub fn new(client: &'a WechatMpClient) -> Self {
        Self { client }
    }

    // ==================== 二维码生成接口 ====================

    /// 创建临时二维码（场景值ID）
    ///
    /// 本接口用于创建临时二维码，二维码有效时间最长30天（2592000秒），场景值ID不能为0。
    ///
    /// # 参数
    /// * `scene_id` - 场景值ID（32位非0整型）
    /// * `expire_seconds` - 二维码有效时间（秒），最大不超过2592000（即30天）
    ///
    /// # 返回
    /// 返回 `LabradorResult<QRCodeTicket>`，包含ticket和二维码URL。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/doc/offiaccount/Account_Management/Generating_a_Parametric_QR_Code.html>
    pub async fn create_temp_by_id(
        &self,
        scene_id: i32,
        expire_seconds: u64,
    ) -> LabradorResult<QRCodeTicket> {
        if scene_id == 0 {
            return Err(LabraError::RequestError(
                "临时二维码场景值不能为0".to_string(),
            ));
        }
        self.create_qrcode(QR_SCENE, None, Some(scene_id), Some(expire_seconds))
            .await
    }

    /// 创建临时二维码（场景值字符串）
    ///
    /// 本接口用于创建临时二维码，使用字符串作为场景值，长度限制为1到64字节。
    ///
    /// # 参数
    /// * `scene_str` - 场景值字符串（1到64字节）
    /// * `expire_seconds` - 二维码有效时间（秒），最大不超过2592000（即30天）
    ///
    /// # 返回
    /// 返回 `LabradorResult<QRCodeTicket>`，包含ticket和二维码URL。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/doc/offiaccount/Account_Management/Generating_a_Parametric_QR_Code.html>
    pub async fn create_temp_by_str(
        &self,
        scene_str: &str,
        expire_seconds: u64,
    ) -> LabradorResult<QRCodeTicket> {
        if scene_str.is_empty() {
            return Err(LabraError::RequestError(
                "临时二维码场景值不能为空".to_string(),
            ));
        }
        self.create_qrcode(QR_STR_SCENE, Some(scene_str), None, Some(expire_seconds))
            .await
    }

    /// 创建永久二维码（场景值ID）
    ///
    /// 本接口用于创建永久二维码，无过期时间，但数量有限（最多10万个）。
    ///
    /// # 参数
    /// * `scene_id` - 场景值ID（1~100000）
    ///
    /// # 返回
    /// 返回 `LabradorResult<QRCodeTicket>`，包含ticket和二维码URL。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/doc/offiaccount/Account_Management/Generating_a_Parametric_QR_Code.html>
    pub async fn create_perm_by_id(&self, scene_id: i32) -> LabradorResult<QRCodeTicket> {
        self.create_qrcode(QR_LIMIT_SCENE, None, Some(scene_id), None)
            .await
    }

    /// 创建永久二维码（场景值字符串）
    ///
    /// 本接口用于创建永久二维码，使用字符串作为场景值，长度限制为1到64字节。
    ///
    /// # 参数
    /// * `scene_str` - 场景值字符串（1到64字节）
    ///
    /// # 返回
    /// 返回 `LabradorResult<QRCodeTicket>`，包含ticket和二维码URL。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/doc/offiaccount/Account_Management/Generating_a_Parametric_QR_Code.html>
    pub async fn create_perm_by_str(&self, scene_str: &str) -> LabradorResult<QRCodeTicket> {
        self.create_qrcode(QR_LIMIT_STR_SCENE, Some(scene_str), None, None)
            .await
    }

    /// 创建二维码的核心方法
    async fn create_qrcode(
        &self,
        action_name: &str,
        scene_str: Option<&str>,
        scene_id: Option<i32>,
        expire_seconds: Option<u64>,
    ) -> LabradorResult<QRCodeTicket> {
        // 临时二维码有效期校验
        if let Some(expire) = expire_seconds {
            if expire > 2592000 {
                return Err(LabraError::RequestError(
                    "临时二维码有效时间最大不能超过30天".to_string(),
                ));
            }
        }

        let scene = match (scene_str, scene_id) {
            (Some(s), _) => json!({ "scene_str": s }),
            (_, Some(id)) => json!({ "scene_id": id }),
            _ => Value::Null,
        };

        let mut req = json!({
            "action_name": action_name,
            "action_info": { "scene": scene }
        });

        if let Some(expire) = expire_seconds {
            req["expire_seconds"] = json!(expire);
        }

        let response: WechatApiResponse<QRCodeTicket> = self
            .client
            .wechat_client()
            .post("/cgi-bin/qrcode/create", req)
            .await?;
        response.into_result()
    }

    /// 通过ticket换取二维码图片URL
    ///
    /// 本方法用于根据ticket生成二维码图片的URL。可直接在浏览器中打开或作为img标签的src。
    ///
    /// # 参数
    /// * `ticket` - 通过创建二维码接口获取的ticket
    ///
    /// # 返回
    /// 返回二维码图片的URL字符串，格式为 `https://mp.weixin.qq.com/cgi-bin/showqrcode?ticket={ticket}`
    pub fn get_image_url(&self, ticket: &str) -> String {
        format!(
            "https://mp.weixin.qq.com/cgi-bin/showqrcode?ticket={}",
            ticket
        )
    }

    // ==================== 二维码跳转规则 ====================

    /// 获取二维码跳转规则列表
    ///
    /// 本接口用于获取已设置的二维码跳转规则。
    ///
    /// # 返回
    /// 返回 `LabradorResult<QRCodeJumpRules>`，包含所有跳转规则列表。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/doc/offiaccount/Account_Management/QRCode_Jump_Rules.html>
    pub async fn get_jump_rules(&self) -> LabradorResult<QRCodeJumpRules> {
        let response: WechatApiResponse<QRCodeJumpRules> = self
            .client
            .wechat_client()
            .get("/cgi-bin/qrcodejump/get")
            .await?;
        response.into_result()
    }

    /// 添加二维码跳转规则
    ///
    /// 本接口用于添加二维码跳转规则，用于配置扫码后的跳转行为。
    ///
    /// # 参数
    /// * `request` - 跳转规则请求，包含二维码规则前缀、跳转方式、小程序appid和path等
    ///
    /// # 返回
    /// 返回 `LabradorResult<WechatApiResponse>`，包含微信API的通用响应。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/doc/offiaccount/Account_Management/QRCode_Jump_Rules.html>
    pub async fn add_jump_rule(
        &self,
        request: &AddQRCodeJumpRuleRequest,
    ) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/cgi-bin/qrcodejump/add", request)
            .await?;
        Ok(response)
    }

    /// 发布二维码跳转规则
    ///
    /// 本接口用于发布已添加的二维码跳转规则，发布后规则生效。
    ///
    /// # 参数
    /// * `prefix` - 二维码规则前缀
    ///
    /// # 返回
    /// 返回 `LabradorResult<WechatApiResponse>`，包含微信API的通用响应。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/doc/offiaccount/Account_Management/QRCode_Jump_Rules.html>
    pub async fn publish_jump_rule(&self, prefix: &str) -> LabradorResult<WechatApiResponse> {
        let request = json!({ "prefix": prefix });
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/cgi-bin/qrcodejump/publish", request)
            .await?;
        Ok(response)
    }

    /// 删除二维码跳转规则
    ///
    /// 本接口用于删除指定的二维码跳转规则。
    ///
    /// # 参数
    /// * `prefix` - 要删除的二维码规则前缀
    ///
    /// # 返回
    /// 返回 `LabradorResult<WechatApiResponse>`，包含微信API的通用响应。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/doc/offiaccount/Account_Management/QRCode_Jump_Rules.html>
    pub async fn delete_jump_rule(&self, prefix: &str) -> LabradorResult<WechatApiResponse> {
        let request = json!({ "prefix": prefix });
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/cgi-bin/qrcodejump/delete", request)
            .await?;
        Ok(response)
    }

    // ==================== 短链接生成 ====================

    /// 生成短key
    ///
    /// 本接口用于将长链接转换为短key，可设置有效期。
    ///
    /// # 参数
    /// * `long_url` - 需要转换的长链接
    /// * `expire_seconds` - 短链接有效期（秒），可选
    ///
    /// # 返回
    /// 返回 `LabradorResult<ShortKeyResponse>`，包含短key和短链接URL。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/doc/offiaccount/Account_Management/Short_Link_Generation.html>
    pub async fn gen_short_key(
        &self,
        long_url: &str,
        expire_seconds: Option<u64>,
    ) -> LabradorResult<ShortKeyResponse> {
        let mut request = json!({ "long_url": long_url });
        if let Some(expire) = expire_seconds {
            request["expire_seconds"] = json!(expire);
        }
        let response: WechatApiResponse<ShortKeyResponse> = self
            .client
            .wechat_client()
            .post("/cgi-bin/shorten/gen", request)
            .await?;
        response.into_result()
    }

    /// 获取短链接
    ///
    /// 本接口用于通过短key获取对应的原始长链接。
    ///
    /// # 参数
    /// * `short_key` - 通过 `gen_short_key` 接口生成的短key
    ///
    /// # 返回
    /// 返回 `LabradorResult<String>`，包含原始长链接。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/doc/offiaccount/Account_Management/Short_Link_Generation.html>
    pub async fn fetch_short_url(&self, short_key: &str) -> LabradorResult<String> {
        let request = json!({ "short_key": short_key });
        let response: WechatApiResponse<FetchShortUrlResponse> = self
            .client
            .wechat_client()
            .post("/cgi-bin/shorten/fetch", request)
            .await?;
        Ok(response.into_result()?.long_url)
    }
}

// ============================================================================
// 请求与响应结构体
// ============================================================================

/// 二维码票据
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QRCodeTicket {
    /// 获取的二维码ticket，凭借此ticket可以在有效时间内换取二维码
    pub ticket: Option<String>,
    /// 二维码的有效时间，以秒为单位。最大不超过2592000（即30天）
    pub expire_seconds: Option<i32>,
    /// 二维码图片解析后的地址，开发者可根据该地址自行生成需要的二维码图片
    pub url: Option<String>,
}

/// 添加二维码跳转规则请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddQRCodeJumpRuleRequest {
    /// 二维码规则
    pub prefix: String,
    /// 跳转到的发布ID
    pub publish_id: i32,
    /// 规则名称
    pub rule_name: Option<String>,
    /// 跳转方式：1-通过官方页面跳转，2-通过接口跳转
    pub jump_type: i32,
    /// 小程序appid
    pub appid: String,
    /// 小程序path
    pub path: String,
    /// 是否校验权限：0-否，1-是
    pub is_need_check: Option<i32>,
}

/// 二维码跳转规则列表
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QRCodeJumpRules {
    pub rule_list: Vec<QRCodeJumpRule>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QRCodeJumpRule {
    pub prefix: String,
    pub path: String,
    pub publish_id: i32,
    pub rule_name: String,
    pub jump_type: i32,
    pub is_need_check: i32,
}

/// 短链接生成响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortKeyResponse {
    /// 短key
    pub short_key: String,
    /// 短链接
    pub short_url: String,
    /// 有效期（秒）
    pub expire_seconds: Option<i32>,
}

/// 获取短链接响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FetchShortUrlResponse {
    /// 原始长链接
    pub long_url: String,
}
