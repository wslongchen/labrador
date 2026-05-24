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
use crate::platforms::wechat::pay::{WechatPayClient, WechatPayConfig};
use crate::platforms::wechat::pay::config::WechatPayApiVersion;

/// 微信支付构建器
pub struct WechatPayBuilder {
    config: WechatPayConfig,
}

impl WechatPayBuilder {
    /// 创建新的构建器
    pub fn new(app_id: &str, mch_id: &str, api_key: &str, notify_url: &str) -> Self {
        Self {
            config: WechatPayConfig::new(app_id, mch_id, api_key, notify_url),
        }
    }

    /// 设置证书路径
    pub fn cert_path<S: Into<String>>(mut self, cert_path: S) -> Self {
        self.config.cert_path = Some(cert_path.into());
        self
    }
    
    /// 设置p12证书路径
    pub fn p12_path<S: Into<String>>(mut self, p12_path: S, p12_password: Option<S>) -> Self {
        self.config.p12_path = Some(p12_path.into());
        self.config.p12_password = p12_password.map(|s| s.into());
        self
    }
    
    /// 设置apiKey
    pub fn api_key<S: Into<String>>(mut self, api_key: S) -> Self {
        self.config.api_key = Some(api_key.into());
        self
    }
    
    /// 设置v3版本密钥
    pub fn api_key_v3<S: Into<String>>(mut self, api_key_v3: S) -> Self {
        self.config.api_key_v3 = Some(api_key_v3.into());
        self.config.api_version = WechatPayApiVersion::V3;
        self
    }
    
    /// 设置api版本
    pub fn api_version(mut self, api_version: WechatPayApiVersion) -> Self {
        self.config.api_version = api_version;
        self
    }

    /// 设置密钥路径
    pub fn key_path<S: Into<String>>(mut self, key_path: S) -> Self {
        self.config.key_path = Some(key_path.into());
        self
    }

    /// 启用沙箱模式
    pub fn sandbox(mut self, sandbox: bool) -> Self {
        self.config.sandbox = sandbox;
        self
    }

    /// 设置退款通知URL
    pub fn refund_notify_url<S: Into<String>>(mut self, refund_notify_url: S) -> Self {
        self.config.refund_notify_url = Some(refund_notify_url.into());
        self
    }

    /// 构建微信支付客户端
    pub async fn build(mut self) -> LabradorResult<WechatPayClient> {
        // 判断是否v3版本，没有设置证书则需要自动获取
        if self.config.api_version == WechatPayApiVersion::V3 {
            let root_certs = self.config.root_certificates.clone().unwrap_or_default();
            if root_certs.is_empty() {
                let certs = WechatPayClient::get_certificates(&self.config).await?;
                self.config.root_certificates = Some(certs);
            }
        }
        WechatPayClient::new(self.config)
    }
}