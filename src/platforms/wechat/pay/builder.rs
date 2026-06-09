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
use crate::client::certificate::Certificate;
use crate::errors::LabradorResult;
use crate::platforms::wechat::pay::config::WechatPayApiVersion;
use crate::platforms::wechat::pay::{WechatPayClient, WechatPayConfig};

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

    /// 设置 API 证书序列号（V3 商户签名必填）
    pub fn serial_no<S: Into<String>>(mut self, serial_no: S) -> Self {
        self.config.serial_no = Some(serial_no.into());
        self
    }

    /// 设置商户 API 私钥（PEM 格式，V3 商户签名必填）
    pub fn private_key<S: Into<String>>(mut self, private_key: S) -> Self {
        self.config.private_key = Some(private_key.into());
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

    /// 设置微信支付公钥（微信支付公钥模式）。
    ///
    /// 传入微信支付后台提供的公钥 PEM 和公钥 ID。
    /// 启用后不再通过 `/v3/certificates` 下载平台证书，
    /// 响应验签将使用此公钥。
    pub fn platform_public_key<S: Into<String>>(
        mut self,
        public_key_pem: S,
        key_id: S,
    ) -> Self {
        self.config.platform_public_key = Some(public_key_pem.into());
        self.config.platform_public_key_id = Some(key_id.into());
        self
    }

    /// 构建微信支付客户端
    pub async fn build(mut self) -> LabradorResult<WechatPayClient> {
        if self.config.api_version == WechatPayApiVersion::V3 {
            let root_certs = self.config.root_certificates.clone().unwrap_or_default();
            if root_certs.is_empty() {
                // 优先使用手动配置的微信支付公钥（微信支付公钥模式）
                if let (Some(pk), Some(kid)) = (
                    self.config.platform_public_key.as_ref(),
                    self.config.platform_public_key_id.as_ref(),
                ) {
                    let cert = Certificate::from_public_key_pem(pk.as_bytes(), kid)?;
                    tracing::info!(
                        "微信支付公钥模式: 已从 PEM 构建公钥条目, key_id={}, 跳过 /v3/certificates",
                        kid,
                    );
                    self.config.root_certificates = Some(vec![cert]);
                } else {
                    // 无公钥 → 自动下载微信支付平台证书
                    let certs = WechatPayClient::get_certificates(&self.config).await?;
                    self.config.root_certificates = Some(certs);
                }
            }
        }
        WechatPayClient::new(self.config)
    }
}
