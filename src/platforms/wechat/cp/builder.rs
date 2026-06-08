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
use crate::errors::LabradorResult;
use crate::wechat::cp::config::WechatCpConfig;
use crate::wechat::cp::WechatCpClient;

/// 企业微信构建器
pub struct WechatCpBuilder {
    config: WechatCpConfig,
}

impl WechatCpBuilder {
    /// 创建新的构建器
    pub fn new(app_id: &str, app_secret: &str) -> Self {
        Self {
            config: WechatCpConfig::new(app_id, app_secret),
        }
    }

    pub fn token(mut self, token: &str) -> Self {
        self.config.token = Some(token.to_string());
        self
    }

    pub fn aes_key(mut self, aes_key: &str) -> Self {
        self.config.aes_key = Some(aes_key.to_string());
        self
    }

    pub fn agent_id(mut self, agent_id: i32) -> Self {
        self.config.agent_id = Some(agent_id);
        self
    }

    pub fn oauth2_redirect_uri(mut self, oauth2_redirect_uri: &str) -> Self {
        self.config.oauth2_redirect_uri = Some(oauth2_redirect_uri.to_string());
        self
    }

    pub fn webhook_url(mut self, webhook_url: &str) -> Self {
        self.config.webhook_url = Some(webhook_url.to_string());
        self
    }

    /// 构建企业微信客户端
    pub fn build(self) -> LabradorResult<WechatCpClient> {
        WechatCpClient::new(self.config)
    }
}
