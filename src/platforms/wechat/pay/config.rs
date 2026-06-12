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
use crate::client::certificate::Certificate;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum WechatPayApiVersion {
    V2,
    V3,
}

/// 微信支付配置
#[derive(Debug, Clone)]
pub struct WechatPayConfig {
    /// 应用ID
    pub(crate) app_id: String,
    /// 商户号
    pub(crate) mch_id: String,
    /// API密钥
    pub(crate) api_key: Option<String>,
    /// API版本
    pub(crate) api_version: WechatPayApiVersion,
    /// API密钥V3
    pub(crate) api_key_v3: Option<String>,
    /// 证书路径（PEM格式）
    pub(crate) cert_path: Option<String>,
    /// 密钥路径（PEM格式）
    pub(crate) key_path: Option<String>,
    /// P12证书路径（新增）
    pub(crate) p12_path: Option<String>,
    /// P12证书密码（新增，默认使用商户号）
    pub(crate) p12_password: Option<String>,
    /// API证书序列号
    pub(crate) serial_no: Option<String>,
    /// API商户证书秘钥
    pub(crate) private_key: Option<String>,
    /// 根证书
    pub(crate) root_certificates: Option<Vec<Certificate>>,
    /// 是否启用沙箱模式
    pub(crate) sandbox: bool,
    /// 支付结果通知URL
    pub(crate) notify_url: String,
    /// 退款结果通知URL
    pub(crate) refund_notify_url: Option<String>,
    /// 微信支付公钥 PEM（微信支付公钥模式，优先级高于自动下载平台证书）
    pub(crate) platform_public_key: Option<String>,
    /// 微信支付公钥 ID（与 platform_public_key 配对，用于匹配 Wechatpay-Serial 响应头）
    pub(crate) platform_public_key_id: Option<String>,
}

impl Default for WechatPayConfig {
    fn default() -> Self {
        Self {
            app_id: "".to_string(),
            api_version: WechatPayApiVersion::V3,
            mch_id: "".to_string(),
            api_key: None,
            api_key_v3: None,
            cert_path: None,
            key_path: None,
            p12_path: None,
            p12_password: None,
            serial_no: None,
            private_key: None,
            root_certificates: None,
            sandbox: false,
            notify_url: "".to_string(),
            refund_notify_url: None,
            platform_public_key: None,
            platform_public_key_id: None,
        }
    }
}

impl WechatPayConfig {
    /// 创建新的微信支付配置
    pub fn new(app_id: &str, mch_id: &str, api_key: &str, notify_url: &str) -> Self {
        Self {
            app_id: app_id.to_string(),
            mch_id: mch_id.to_string(),
            api_key: Some(api_key.to_string()),
            api_version: WechatPayApiVersion::V3,
            api_key_v3: None,
            cert_path: None,
            key_path: None,
            p12_path: None,
            p12_password: None,
            serial_no: None,
            private_key: None,
            root_certificates: None,
            sandbox: false,
            notify_url: notify_url.to_string(),
            refund_notify_url: None,
            platform_public_key: None,
            platform_public_key_id: None,
        }
    }

    pub fn with_private_key(mut self, private_key: &str) -> Self {
        self.private_key = Some(private_key.to_string());
        self
    }

    pub fn with_api_key_v3(mut self, api_key_v3: &str) -> Self {
        self.api_key_v3 = Some(api_key_v3.to_string());
        self
    }

    pub fn with_root_certificates(mut self, root_certificates: Vec<Certificate>) -> Self {
        self.root_certificates = Some(root_certificates);
        self
    }

    pub fn with_api_version(mut self, api_version: WechatPayApiVersion) -> Self {
        self.api_version = api_version;
        self
    }

    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.api_key = Some(api_key.to_string());
        self
    }

    pub fn with_mch_id(mut self, mch_id: &str) -> Self {
        self.mch_id = mch_id.to_string();
        self
    }

    pub fn with_app_id(mut self, app_id: &str) -> Self {
        self.app_id = app_id.to_string();
        self
    }

    pub fn with_notify_url(mut self, notify_url: &str) -> Self {
        self.notify_url = notify_url.to_string();
        self
    }

    pub fn with_sandbox(mut self) -> Self {
        self.sandbox = true;
        self
    }

    pub fn with_cert_path_and_key_path(mut self, cert_path: &str, key_path: &str) -> Self {
        self.cert_path = Some(cert_path.to_string());
        self.key_path = Some(key_path.to_string());
        self
    }

    pub fn with_p12_path(mut self, p12_path: &str) -> Self {
        self.p12_path = Some(p12_path.to_string());
        self
    }

    pub fn with_p12_password(mut self, p12_password: &str) -> Self {
        self.p12_password = Some(p12_password.to_string());
        self
    }

    pub fn with_serial_no(mut self, serial_no: &str) -> Self {
        self.serial_no = Some(serial_no.to_string());
        self
    }

    /// 设置证书路径
    pub fn with_cert_path(mut self, cert_path: &str) -> Self {
        self.cert_path = Some(cert_path.to_string());
        self
    }

    /// 设置密钥路径
    pub fn with_key_path(mut self, key_path: &str) -> Self {
        self.key_path = Some(key_path.to_string());
        self
    }

    /// 设置PEM格式证书
    pub fn with_pem_cert(mut self, cert_path: &str, key_path: &str) -> Self {
        self.cert_path = Some(cert_path.to_string());
        self.key_path = Some(key_path.to_string());
        self
    }

    /// 设置P12格式证书
    pub fn with_p12_cert(mut self, p12_path: &str, password: Option<&str>) -> Self {
        self.p12_path = Some(p12_path.to_string());
        self.p12_password = password.map(|s| s.to_string());
        self
    }

    /// 启用沙箱模式
    pub fn enable_sandbox(mut self) -> Self {
        self.sandbox = true;
        self
    }

    /// 设置退款通知URL
    pub fn with_refund_notify_url(mut self, refund_notify_url: &str) -> Self {
        self.refund_notify_url = Some(refund_notify_url.to_string());
        self
    }

    /// 设置微信支付公钥（微信支付公钥模式）。
    ///
    /// 当商户在微信支付后台启用「微信支付公钥」而非「平台证书」时，
    /// 不再需要通过 `/v3/certificates` 下载平台证书，
    /// 而是使用微信提供的单一公钥 + 公钥 ID 进行响应验签。
    pub fn with_platform_public_key(mut self, public_key_pem: &str, key_id: &str) -> Self {
        self.platform_public_key = Some(public_key_pem.to_string());
        self.platform_public_key_id = Some(key_id.to_string());
        self
    }
}
