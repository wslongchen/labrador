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
use crate::alipay::client::AlipayClient;
use crate::alipay::config::AlipayClientConfig;
use crate::errors::LabradorResult;
use std::time::Duration;

/// 支付宝客户端构建器
pub struct AlipayClientBuilder {
    config: AlipayClientConfig,
}

impl AlipayClientBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            config: AlipayClientConfig::default(),
        }
    }

    /// 设置应用ID
    pub fn app_id<S: Into<String>>(mut self, app_id: S) -> Self {
        self.config.app_id = app_id.into();
        self
    }

    /// 设置应用私钥
    pub fn app_private_key<S: Into<String>>(mut self, private_key: S) -> Self {
        self.config.app_private_key = private_key.into();
        self
    }

    /// 设置支付宝公钥
    pub fn alipay_public_key<S: Into<String>>(mut self, public_key: S) -> Self {
        self.config.alipay_public_key = Some(public_key.into());
        self
    }

    /// 设置应用证书路径
    pub fn app_cert_path<S: Into<String>>(mut self, cert_path: S) -> Self {
        self.config.app_cert_path = Some(cert_path.into());
        self
    }

    /// 设置支付宝公钥证书路径
    pub fn alipay_public_cert_path<S: Into<String>>(mut self, cert_path: S) -> Self {
        self.config.alipay_public_cert_path = Some(cert_path.into());
        self
    }

    /// 设置支付宝根证书路径
    pub fn alipay_root_cert_path<S: Into<String>>(mut self, cert_path: S) -> Self {
        self.config.alipay_root_cert_path = Some(cert_path.into());
        self
    }

    /// 启用沙箱模式
    pub fn sandbox(mut self, sandbox: bool) -> Self {
        self.config.sandbox = sandbox;
        self
    }

    /// 启用证书模式
    pub fn use_cert(mut self, use_cert: bool) -> Self {
        self.config.use_cert = use_cert;
        self
    }

    /// 设置加密密钥
    pub fn encrypt_key<S: Into<String>>(mut self, encrypt_key: S) -> Self {
        self.config.encrypt_key = Some(encrypt_key.into());
        self
    }

    /// 设置签名类型
    pub fn sign_type<S: Into<String>>(mut self, sign_type: S) -> Self {
        self.config.sign_type = sign_type.into();
        self
    }

    /// 设置字符集
    pub fn charset<S: Into<String>>(mut self, charset: S) -> Self {
        self.config.charset = charset.into();
        self
    }

    /// 设置格式
    pub fn format<S: Into<String>>(mut self, format: S) -> Self {
        self.config.format = format.into();
        self
    }

    /// 设置请求超时时间
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = Some(timeout);
        self
    }

    /// 构建支付宝客户端
    pub fn build(self) -> LabradorResult<AlipayClient> {
        AlipayClient::new(self.config)
    }
}

impl Default for AlipayClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
