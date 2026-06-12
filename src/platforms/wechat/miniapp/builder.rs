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
use crate::platforms::wechat::miniapp::config::WechatMiniAppConfig;
use crate::platforms::wechat::miniapp::WechatMiniAppClient;

/// 微信小程序构建器
pub struct WechatMiniAppBuilder {
    config: WechatMiniAppConfig,
}

impl WechatMiniAppBuilder {
    /// 创建新的构建器
    pub fn new(app_id: &str, app_secret: &str) -> Self {
        Self {
            config: WechatMiniAppConfig::new(app_id, app_secret),
        }
    }

    /// 启用沙箱模式
    pub fn sandbox(mut self, sandbox: bool) -> Self {
        self.config.sandbox = sandbox;
        self
    }

    /// 构建微信小程序客户端
    pub fn build(self) -> LabradorResult<WechatMiniAppClient> {
        WechatMiniAppClient::new(self.config)
    }
}
