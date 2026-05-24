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


/// 微信公众号配置
#[derive(Debug, Clone)]
pub struct WechatMpConfig {
    /// 应用ID
    pub app_id: String,
    /// 应用密钥
    pub app_secret: String,
    /// 令牌
    pub token: Option<String>,
    /// 消息加解密密钥
    pub encoding_aes_key: Option<String>,
    /// 是否启用沙箱模式
    pub sandbox: bool,
}

impl WechatMpConfig {
    /// 创建新的微信公众号配置
    pub fn new(app_id: &str, app_secret: &str) -> Self {
        Self {
            app_id: app_id.to_string(),
            app_secret: app_secret.to_string(),
            token: None,
            encoding_aes_key: None,
            sandbox: false,
        }
    }

    /// 设置令牌
    pub fn with_token(mut self, token: &str) -> Self {
        self.token = Some(token.to_string());
        self
    }

    /// 设置消息加解密密钥
    pub fn with_encoding_aes_key(mut self, encoding_aes_key: &str) -> Self {
        self.encoding_aes_key = Some(encoding_aes_key.to_string());
        self
    }

    /// 启用沙箱模式
    pub fn enable_sandbox(mut self) -> Self {
        self.sandbox = true;
        self
    }
}
