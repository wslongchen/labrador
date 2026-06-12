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
use crate::utils::encryption::base64_encode;
use crate::wechat::client::WechatApiResponse;
use crate::wechat::miniapp::WechatMiniAppClient;
use crate::{AesEncryptor, AesMode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct WechatMxaUser<'a> {
    client: &'a WechatMiniAppClient,
}

#[allow(unused)]
impl<'a> WechatMxaUser<'a> {
    #[inline]
    pub fn new(client: &'a WechatMiniAppClient) -> WechatMxaUser<'a> {
        WechatMxaUser { client }
    }

    /// 获取用户手机号
    ///
    /// 本接口用于获取小程序用户的手机号。用户在客户端调用
    /// `wx.getPhoneNumber` 获取动态令牌 code 后，服务端通过本接口
    /// 使用 code 换取用户手机号信息。
    ///
    /// # 参数
    /// * `code` - 小程序端获取手机号时返回的动态令牌 code，每个 code 只能使用一次
    ///
    /// # 返回
    /// 返回 `LabradorResult<PhoneInfo>`，包含用户绑定的手机号（含区号）、纯手机号、区号及数据水印。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/user-info/phone-number/getPhoneNumber.html>
    pub async fn get_user_phone_number(&self, code: &str) -> LabradorResult<PhoneInfo> {
        let request = serde_json::json!({
            "code": code,
        });
        let response: WechatApiResponse<PhoneInfo> = self
            .client
            .wechat_client()
            .post("/wxa/business/getuserphonenumber", request)
            .await?;

        response.into_result()
    }

    /// 获取用户加密密钥（encryptKey）
    ///
    /// 本接口用于获取用户的加密密钥（encryptKey）。会获取用户最近3次的key，
    /// 每个key的存活时间为3600秒（1小时）。该密钥可用于解密用户在小程序端
    /// 获取的加密数据（如手机号、运动步数等）。
    ///
    /// # 参数
    /// * `openid` - 用户的唯一标识，即 openid
    /// * `signature` - 用户在小程序端调用相关接口获取的签名
    /// * `sig_method` - 签名方法，如 `hmac_sha256`
    ///
    /// # 返回
    /// 返回 `LabradorResult<UserEncryptKeyResponse>`，包含加密密钥、版本号、有效时间和初始向量（IV）。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/user-info/basic-info/getUserEncryptKey.html>
    pub async fn get_user_encrypt_key(
        &self,
        openid: &str,
        signature: &str,
        sig_method: &str,
    ) -> LabradorResult<UserEncryptKeyResponse> {
        let response: WechatApiResponse<UserEncryptKeyResponse> = self
            .client
            .wechat_client()
            .get(&format!(
                "/wxa/business/getuserencryptkey?openid={}&signature={}&sig_method={}",
                openid, signature, sig_method
            ))
            .await?;
        response.into_result()
    }

    /// 获取插件用户 openPid
    ///
    /// 本接口用于换取插件用户的唯一标识。通过 `wx.pluginLogin` 接口获得
    /// 插件用户标志凭证 code 后传到开发者服务器，开发者服务器调用此接口
    /// 换取插件用户的唯一标识 openpid。
    ///
    /// # 参数
    /// * `request` - 请求参数，包含 `code`（通过 `wx.pluginLogin` 获得的凭证）
    ///
    /// # 返回
    /// 返回 `LabradorResult<PluginOpenPidResponse>`，包含插件用户的唯一标识 openpid。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/user-info/basic-info/getPluginOpenPId.html>
    pub async fn get_plugin_open_pid(
        &self,
        request: &PluginOpenPidRequest,
    ) -> LabradorResult<PluginOpenPidResponse> {
        let response: WechatApiResponse<PluginOpenPidResponse> = self
            .client
            .wechat_client()
            .post("/wxa/getpluginopenpid", request)
            .await?;
        response.into_result()
    }

    /// 支付后获取 UnionId
    ///
    /// 本接口用于在用户支付完成后获取该用户的 UnionId，无需用户授权。
    /// 调用本接口前需要用户完成支付，且支付后五分钟内有效。
    ///
    /// # 参数
    /// * `request` - 请求参数，包含 `openid`、微信支付订单号（transaction_id）
    ///   或商户订单号（out_trade_no）+ 商户号（mch_id）的组合
    ///
    /// # 返回
    /// 返回 `LabradorResult<PaidUnionIdResponse>`，包含用户的 UnionId。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/user-info/basic-info/getPaidUnionId.html>
    pub async fn get_paid_unionid(
        &self,
        request: &PaidUnionIdRequest,
    ) -> LabradorResult<PaidUnionIdResponse> {
        let response: WechatApiResponse<PaidUnionIdResponse> = self
            .client
            .wechat_client()
            .post("/wxa/getpaidunionid", request)
            .await?;
        response.into_result()
    }

    /// 解密用户手机号信息
    ///
    /// 本接口使用 AES-CBC 算法对微信加密的手机号数据进行解密。
    /// 解密所需的 session_key 来自小程序登录时获取的会话密钥。
    ///
    /// # 参数
    /// * `session_key` - 小程序登录时获取的会话密钥（session_key）
    /// * `encrypted_data` - 加密的手机号数据（Base64编码）
    /// * `iv` - 加密算法的初始向量（Base64编码）
    ///
    /// # 返回
    /// 返回 `LabradorResult<PhoneInfo>`，包含解密后的手机号信息。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/miniprogram/dev/framework/open-ability/signature.html>
    pub async fn decrypt_phone_info(
        &self,
        session_key: &str,
        encrypted_data: &str,
        iv: &str,
    ) -> LabradorResult<PhoneInfo> {
        let session_key = base64_encode(session_key.as_bytes());
        let iv = base64_encode(iv.as_bytes());
        let encrypted_data = base64_encode(encrypted_data.as_bytes());
        let encryptor = AesEncryptor::new(AesMode::Cbc, session_key.as_bytes(), iv.as_bytes())?;
        let decrypted = encryptor.decrypt(encrypted_data.as_bytes())?;
        serde_json::from_slice::<PhoneInfo>(&decrypted).map_err(LabraError::from)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEncryptKeyResponse {
    /// 加密key
    pub encrypt_key: String,
    /// key的版本号
    pub version: i32,
    /// 剩余有效时间
    pub expire_in: i64,
    /// 加密iv
    pub iv: String,
    /// 创建key的时间戳
    pub create_time: i64,
}

/// 手机号信息
#[derive(Debug, Clone, Deserialize)]
pub struct PhoneInfo {
    /// 用户绑定的手机号（国外手机号会有区号）
    pub phone_number: Option<String>,
    /// 没有区号的手机号
    pub pure_phone_number: Option<String>,
    /// 区号
    pub country_code: Option<String>,
    /// 数据水印
    pub watermark: Option<Watermark>,
}

/// 数据水印
#[derive(Debug, Clone, Deserialize)]
pub struct Watermark {
    /// 水印时间戳
    pub timestamp: i64,
    /// 小程序appid
    pub appid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginOpenPidRequest {
    /// 通过 wx.pluginLogin 获得的插件用户标志凭证 code，有效时间为5分钟，一个 code 只能获取一次 openpid。
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginOpenPidResponse {
    /// 插件用户的唯一标识。
    pub openpid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaidUnionIdRequest {
    /// 支付用户唯一标识。
    pub openid: String,
    /// 微信支付订单号
    pub transaction_id: Option<String>,
    /// 微信支付分配的商户号，和商户订单号配合使用
    pub mch_id: Option<String>,
    /// 微信支付商户订单号，和商户号配合使用
    pub out_trade_no: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaidUnionIdResponse {
    /// 用户唯一标识，调用成功后返回。
    pub unionid: String,
}
