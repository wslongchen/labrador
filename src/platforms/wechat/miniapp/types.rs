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
use serde::{Deserialize, Serialize};

/// 登录凭证校验响应
#[derive(Debug, Clone, Deserialize)]
pub struct Code2SessionResponse {
    /// 用户唯一标识
    pub openid: String,
    /// 会话密钥
    pub session_key: String,
    /// 用户在开放平台的唯一标识符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unionid: Option<String>,
    /// 错误码
    pub errcode: i32,
    /// 错误信息
    pub errmsg: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WechatResetUserSessionKeyResponse {
    /// 用户唯一标识符
    pub openid: String,
    /// 重置后的用户登录态
    pub session_key: String,
}
