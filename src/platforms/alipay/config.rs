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
use crate::alipay::constants;

/// 支付宝客户端配置
#[derive(Debug, Clone)]
pub struct AlipayClientConfig {
    /// 应用ID
    pub app_id: String,
    /// 应用私钥
    pub app_private_key: String,
    /// 支付宝公钥
    pub alipay_public_key: Option<String>,
    /// 应用公钥证书路径
    pub app_cert_path: Option<String>,
    /// 支付宝公钥证书路径
    pub alipay_public_cert_path: Option<String>,
    /// 支付宝根证书路径
    pub alipay_root_cert_path: Option<String>,
    /// 是否启用沙箱模式
    pub sandbox: bool,
    /// 请求超时时间
    pub timeout: Option<Duration>,
    /// 签名类型
    pub sign_type: String,
    /// 字符集
    pub charset: String,
    /// 是否使用证书模式
    pub use_cert: bool,
    /// 加密密钥
    pub encrypt_key: Option<String>,
    /// 加密类型
    pub encrypt_type: String,
    /// 格式
    pub format: String,
}

impl Default for AlipayClientConfig {
    fn default() -> Self {
        Self {
            app_id: String::new(),
            app_private_key: String::new(),
            alipay_public_key: None,
            app_cert_path: None,
            alipay_public_cert_path: None,
            alipay_root_cert_path: None,
            sandbox: false,
            timeout: Some(Duration::from_secs(30)),
            sign_type: constants::SIGN_TYPE_RSA.to_string(),
            charset: constants::CHARSET_UTF8.to_string(),
            use_cert: false,
            encrypt_key: None,
            encrypt_type: constants::ENCRYPT_TYPE_AES.to_string(),
            format: constants::FORMAT_JSON.to_string(),
        }
    }
}