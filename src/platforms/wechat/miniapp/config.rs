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

/// 微信小程序配置
#[derive(Debug, Clone)]
pub struct WechatMiniAppConfig {
    /// 小程序ID
    pub app_id: String,
    /// 小程序密钥
    pub app_secret: String,
    /// 是否启用沙箱模式
    pub sandbox: bool,
}

impl WechatMiniAppConfig {
    /// 创建新的微信小程序配置
    pub fn new(app_id: &str, app_secret: &str) -> Self {
        Self {
            app_id: app_id.to_string(),
            app_secret: app_secret.to_string(),
            sandbox: false,
        }
    }

    /// 启用沙箱模式
    pub fn enable_sandbox(mut self) -> Self {
        self.sandbox = true;
        self
    }
}
