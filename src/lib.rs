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
pub mod errors;
pub mod request;
pub mod response;
pub mod client;
pub mod crypto;
pub mod utils;

// 平台模块（feature-gated）
pub mod platforms;

// 便捷 re-exports
pub use client::{ApiClient, ClientBuilder, ClientConfig};
pub use errors::{LabraError, LabradorResult};
pub use platforms::signer::{RequestSigner, DefaultSigner, SignMethod};
