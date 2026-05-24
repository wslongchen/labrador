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
//! HTTP响应实现

use crate::errors::{LabraError, LabradorResult};
use bytes::Bytes;
use reqwest;
use serde::de::DeserializeOwned;
use std::net::SocketAddr;
use url::Url;

/// HTTP响应
#[derive(Debug, Clone)]
pub struct Response {
    /// 响应URL
    url: Url,
    /// 状态码
    status: reqwest::StatusCode,
    /// 响应头
    headers: reqwest::header::HeaderMap,
    /// 远程地址
    remote_addr: Option<SocketAddr>,
    /// 响应体
    body: Bytes,
}

impl Response {
    /// 创建新的响应
    pub fn new(
        url: Url,
        status: reqwest::StatusCode,
        headers: reqwest::header::HeaderMap,
        remote_addr: Option<SocketAddr>,
        body: Bytes,
    ) -> Self {
        Self {
            url,
            status,
            headers,
            remote_addr,
            body,
        }
    }

    /// 获取响应状态码
    pub fn status(&self) -> reqwest::StatusCode {
        self.status
    }

    /// 获取响应URL
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// 获取远程地址
    pub fn remote_addr(&self) -> Option<SocketAddr> {
        self.remote_addr
    }

    /// 获取响应头
    pub fn headers(&self) -> &reqwest::header::HeaderMap {
        &self.headers
    }

    /// 获取特定的响应头
    pub fn header(&self, name: &str) -> Option<&reqwest::header::HeaderValue> {
        self.headers.get(name)
    }

    /// 解析JSON响应
    pub fn json<T: DeserializeOwned>(&self) -> LabradorResult<T> {
        serde_json::from_slice(&self.body)
            .map_err(LabraError::Json)
    }

    /// 获取文本响应
    pub fn text(&self) -> LabradorResult<String> {
        String::from_utf8(self.body.to_vec())
            .map_err(LabraError::Utf8)
    }

    /// 获取原始字节
    pub fn bytes(&self) -> Bytes {
        self.body.clone()
    }

    /// 检查是否成功
    pub fn is_success(&self) -> bool {
        self.status.is_success() || self.status.is_redirection() || self.status.is_informational()
        || self.status.is_server_error() || self.status.is_client_error()
    }

    /// 检查是否是客户端错误
    pub fn is_client_error(&self) -> bool {
        self.status.is_client_error()
    }

    /// 检查是否是服务器错误
    pub fn is_server_error(&self) -> bool {
        self.status.is_server_error()
    }

    /// 获取Content-Type
    pub fn content_type(&self) -> Option<&str> {
        self.headers
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
    }

    /// 获取Content-Length
    pub fn content_length(&self) -> Option<u64> {
        self.headers
            .get(reqwest::header::CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok())
    }
}
