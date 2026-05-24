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
use std::time::Duration;

/// 企业微信配置
#[derive(Debug, Clone)]
pub struct WechatCpConfig {
    pub corp_id: String,
    pub corp_secret: String,
    /// 请求超时
    pub timeout: Option<Duration>,
    pub token: Option<String>,
    pub aes_key: Option<String>,
    pub oauth2_redirect_uri: Option<String>,
    pub webhook_url: Option<String>,
    pub agent_id: Option<i32>,
}

impl WechatCpConfig {
    /// 创建新的企业微信配置
    pub fn new(corp_id: &str, corp_secret: &str) -> Self {
        Self {
            corp_id: corp_id.to_string(),
            corp_secret: corp_secret.to_string(),
            timeout: None,
            token: None,
            aes_key: None,
            oauth2_redirect_uri: None,
            webhook_url: None,
            agent_id: None,
        }
    }

    pub fn token(mut self, token: &str) -> Self {
        self.token = Some(token.to_string());
        self
    }
    pub fn aes_key(mut self, aes_key: &str) -> Self {
        self.aes_key = Some(aes_key.to_string());
        self
    }
    pub fn oauth2_redirect_uri(mut self, oauth2_redirect_uri: &str) -> Self {
        self.oauth2_redirect_uri = Some(oauth2_redirect_uri.to_string());
        self
    }
    pub fn webhook_url(mut self, webhook_url: &str) -> Self {
        self.webhook_url = Some(webhook_url.to_string());
        self
    }
    pub fn agent_id(mut self, agent_id: i32) -> Self {
        self.agent_id = Some(agent_id);
        self
    }
}
