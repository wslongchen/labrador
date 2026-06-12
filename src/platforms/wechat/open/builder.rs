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

use crate::errors::LabradorResult;
use crate::platforms::wechat::open::config::WechatOpenConfig;
use crate::platforms::wechat::open::WechatOpenClient;

/// 微信开放平台构建器
pub struct WechatOpenBuilder {
    config: WechatOpenConfig,
}

impl WechatOpenBuilder {
    /// 创建新的构建器
    pub fn new(app_id: &str, app_secret: &str) -> Self {
        Self {
            config: WechatOpenConfig::new(app_id, app_secret),
        }
    }

    /// 设置令牌（用于接收开放平台推送事件）
    pub fn token<S: Into<String>>(mut self, token: S) -> Self {
        self.config.token = Some(token.into());
        self
    }

    /// 设置消息加解密密钥
    pub fn encoding_aes_key<S: Into<String>>(mut self, encoding_aes_key: S) -> Self {
        self.config.encoding_aes_key = Some(encoding_aes_key.into());
        self
    }

    /// 启用沙箱模式
    pub fn sandbox(mut self, sandbox: bool) -> Self {
        self.config.sandbox = sandbox;
        self
    }

    /// 构建微信开放平台客户端
    pub fn build(self) -> LabradorResult<WechatOpenClient> {
        WechatOpenClient::new(self.config)
    }
}
