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
use crate::wechat::mp::WechatMpClient;

#[derive(Debug, Clone)]
pub struct WechatMpOauth2<'a> {
    client: &'a WechatMpClient,
}

#[allow(unused)]
impl<'a> WechatMpOauth2<'a> {
    #[inline]
    pub fn new(client: &'a WechatMpClient) -> WechatMpOauth2<'a> {
        WechatMpOauth2 { client }
    }

    /// 通过 code 换取网页授权access_token
    ///
    /// 首先请注意，这里通过 code 换取的是一个特殊的网页授权access_token，与基础支持中的access_token不同。
    /// 如果网页授权的作用域为snsapi_base，则本步骤中获取到access_token的同时也获取到了openid，授权流程到此为止。
    /// 尤其注意：由于公众号的 secret 和获取到的access_token安全级别都非常高，必须只保存在服务器，不允许传给客户端。
    ///
    /// # 参数
    /// * `code` - 用户授权后微信回调返回的code参数
    ///
    /// # 返回
    /// 返回 `LabradorResult<WechatMpOauth2AccessTokenResponse>`，包含access_token、refresh_token、openid和scope等信息。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/doc/offiaccount/OA_Web_Apps/Wechat_webpage_authorization.html#0>
    pub async fn oauth2_token(
        &self,
        code: &str,
    ) -> LabradorResult<WechatMpOauth2AccessTokenResponse> {
        let config = self.client.config();
        let response: WechatApiResponse<WechatMpOauth2AccessTokenResponse> = self
            .client
            .wechat_client()
            .get(&format!(
                "/sns/oauth2/access_token?appid={}&secret={}&code={}&grant_type=authorization_code",
                config.app_id, config.app_secret, code
            ))
            .await?;
        response.into_result()
    }

    /// 刷新access_token
    ///
    /// 由于access_token拥有较短的有效期（2小时），当access_token超时后，可以使用refresh_token进行刷新。
    /// refresh_token有效期为30天，当refresh_token失效之后，需要用户重新授权。
    ///
    /// # 参数
    /// * `refresh_token` - 通过 `oauth2_token` 获取的refresh_token
    ///
    /// # 返回
    /// 返回 `LabradorResult<WechatMpOauth2AccessTokenResponse>`，包含新的access_token和refresh_token。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/doc/offiaccount/OA_Web_Apps/Wechat_webpage_authorization.html#1>
    pub async fn refresh_token(
        &self,
        refresh_token: &str,
    ) -> LabradorResult<WechatMpOauth2AccessTokenResponse> {
        let config = self.client.config();
        let response: WechatApiResponse<WechatMpOauth2AccessTokenResponse> = self
            .client
            .wechat_client()
            .get(&format!(
                "/sns/oauth2/refresh_token?appid={}&grant_type=refresh_token&refresh_token={}",
                config.app_id, refresh_token
            ))
            .await?;
        response.into_result()
    }

    /// 拉取用户信息(需 scope 为 snsapi_userinfo)
    ///
    /// 如果网页授权作用域为snsapi_userinfo，则开发者可以通过access_token和openid拉取用户详细信息。
    /// 包括昵称、性别、头像、城市、省份、国家、unionid等。
    ///
    /// # 参数
    /// * `access_token` - 网页授权access_token（非基础access_token）
    /// * `openid` - 用户唯一标识
    ///
    /// # 返回
    /// 返回 `LabradorResult<WechatMpOauth2UserInfo>`，包含用户昵称、性别、头像等信息。
    ///
    /// # 微信官方文档
    /// <https://developers.weixin.qq.com/doc/offiaccount/OA_Web_Apps/Wechat_webpage_authorization.html#2>
    pub async fn oauth2_userinfo(
        &self,
        access_token: &str,
        openid: &str,
    ) -> LabradorResult<WechatMpOauth2UserInfo> {
        let response: WechatApiResponse<WechatMpOauth2UserInfo> = self
            .client
            .wechat_client()
            .get(&format!(
                "/sns/userinfo?access_token={}&openid={}&lang=zh_CN",
                access_token, openid
            ))
            .await?;
        response.into_result()
    }
}

//----------------------------------------------------------------------------------------------------------------------------
#[allow(unused)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpOauth2AccessTokenResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub openid: String,
    pub scope: String,
    pub expires_in: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpOauth2UserInfo {
    pub openid: String,
    pub nickname: String,
    pub sex: u8,
    pub city: String,
    pub province: String,
    pub country: String,
    pub headimgurl: String,
    pub unionid: Option<String>,
}
