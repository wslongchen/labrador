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
//! # Labrador
//!
//! 统一的多平台支付/开放平台 SDK，支持微信支付、支付宝、微信公众号、微信小程序、企业微信等。
//!
//! ## 架构
//!
//! - `client` — 通用 HTTP 客户端，支持拦截器、签名器、重试机制
//! - `request` / `response` — 请求/响应抽象层
//! - `crypto` — 加解密工具集
//! - `errors` — 统一错误类型
//! - `platforms` — 各平台实现（wechat / alipay）

// 核心模块
pub mod client;
pub mod crypto;
pub mod errors;
pub mod request;
pub mod response;
pub mod utils;

// 平台模块（feature-gated）
pub mod platforms;

// 平台模块别名 re-exports（labrador 内部 crate::alipay 等引用需要）
#[cfg(feature = "alipay")]
pub use platforms::alipay;
#[cfg(feature = "wechat")]
pub use platforms::wechat;

// 便捷 re-exports
pub use client::builder::ClientBuilder;
pub use client::{ApiClient, ClientConfig};
pub use errors::{LabraError, LabradorResult};
pub use platforms::signer::{DefaultSigner, RequestSigner, SignMethod};

// crypto 模块类型 re-exports（方便外部使用）
pub use crypto::{
    AesEncryptor, AesMode, Crypto, CryptoError, CryptoUtils, HashAlgorithm, HashType,
    HmacAlgorithm, PrpCrypto, RsaEncryptor, RsaKeyFormat,
};
