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

/// 微信API基础URL
pub const API_BASE_URL: &str = "https://api.weixin.qq.com";
pub const API_SANDBOX_BASE_URL: &str = "https://api.weixin.qq.com/sandboxnew";

/// 企业微信API基础URL
pub const CP_API_BASE_URL: &str = "https://qyapi.weixin.qq.com";

/// 微信支付API基础URL
pub const PAY_API_BASE_URL: &str = "https://api.mch.weixin.qq.com";

/// 微信支付沙箱API基础URL
pub const PAY_SANDBOX_API_BASE_URL: &str = "https://api.mch.weixin.qq.com/sandboxnew";

/// 微信支付V3 API基础URL
pub const PAY_V3_API_BASE_URL: &str = "https://api.mch.weixin.qq.com/v3";

pub const PAY_SIGN_SCHEMA_V3: &str = "WECHATPAY2-SHA256-RSA2048";
pub static ACCEPT: &str = "Accept";
pub static AUTHORIZATION: &str = "Authorization";
pub static CONTENT_TYPE_JSON: &str = "application/json";
