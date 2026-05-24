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

//! 流式响应实现

use futures::StreamExt;
use bytes::Bytes;
use futures::Stream;
use http::HeaderMap;
use reqwest;
use std::pin::Pin;
use std::task::{Context, Poll};
use url::Url;
use crate::errors::LabraError;

/// 流式响应
pub struct StreamResponse {
    /// 响应URL
    url: Url,
    /// 状态码
    status: reqwest::StatusCode,
    /// 响应头
    headers: HeaderMap,
    /// 响应流
    stream: reqwest::Response,
}

impl StreamResponse {
    /// 创建新的流式响应
    pub fn new(
        url: Url,
        status: reqwest::StatusCode,
        headers: HeaderMap,
        stream: reqwest::Response,
    ) -> Self {
        Self {
            url,
            status,
            headers,
            stream,
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

    /// 获取响应头
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// 获取特定的响应头
    pub fn header(&self, name: &str) -> Option<&reqwest::header::HeaderValue> {
        self.headers.get(name)
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

    /// 检查是否成功
    pub fn is_success(&self) -> bool {
        self.status.is_success()
    }

    /// 转换为字节流
    pub fn into_bytes_stream(self) -> impl Stream<Item = Result<Bytes, LabraError>> {
        self.stream.bytes_stream()
            .map(|result| result.map_err(LabraError::Network))
    }

    /// 转换为响应流
    pub fn into_response(self) -> reqwest::Response {
        self.stream
    }

    /// 消费整个响应体
    pub async fn consume(self) -> Result<(), LabraError> {
        let _ = self.stream.bytes().await.map_err(LabraError::Network)?;
        Ok(())
    }
}

impl std::fmt::Debug for StreamResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StreamResponse")
            .field("url", &self.url)
            .field("status", &self.status)
            .finish()
    }
}

/// 分块响应流
#[allow(unused)]
pub struct ChunkedResponse {
    stream: Pin<Box<dyn Stream<Item = Result<Bytes, LabraError>> + Send + Sync>>,
}

#[allow(unused)]
impl ChunkedResponse {
    /// 创建新的分块响应流
    pub fn new<S>(stream: S) -> Self
    where
        S: Stream<Item = Result<Bytes, LabraError>> + Send + Sync + 'static,
    {
        Self {
            stream: Box::pin(stream),
        }
    }

    /// 从StreamResponse创建
    pub fn from_stream_response(response: StreamResponse) -> Self {
        Self::new(response.into_bytes_stream())
    }
}

impl Stream for ChunkedResponse {
    type Item = Result<Bytes, LabraError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.stream.as_mut().poll_next(cx)
    }
}

/// 响应体分块
#[derive(Debug)]
#[allow(unused)]
pub struct ResponseChunk {
    /// 分块数据
    pub data: Bytes,
    /// 分块序号
    pub sequence: usize,
    /// 是否为最后一个分块
    pub is_last: bool,
}

#[allow(unused)]
impl ResponseChunk {
    /// 创建新的分块
    pub fn new(data: Bytes, sequence: usize, is_last: bool) -> Self {
        Self {
            data,
            sequence,
            is_last,
        }
    }
}

/// 流式响应构建器
#[allow(unused)]
pub struct StreamResponseBuilder {
    url: Option<Url>,
    status: Option<reqwest::StatusCode>,
    headers: Option<HeaderMap>,
}

#[allow(unused)]
impl StreamResponseBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            url: None,
            status: None,
            headers: None,
        }
    }

    /// 设置URL
    pub fn url(mut self, url: Url) -> Self {
        self.url = Some(url);
        self
    }

    /// 设置状态码
    pub fn status(mut self, status: reqwest::StatusCode) -> Self {
        self.status = Some(status);
        self
    }

    /// 设置响应头
    pub fn headers(mut self, headers: HeaderMap) -> Self {
        self.headers = Some(headers);
        self
    }

    /// 构建StreamResponse
    pub fn build(self, stream: reqwest::Response) -> StreamResponse {
        StreamResponse::new(
            self.url.unwrap_or_else(|| "http://localhost".parse().unwrap()),
            self.status.unwrap_or(reqwest::StatusCode::OK),
            self.headers.unwrap_or_default(),
            stream,
        )
    }
}

impl Default for StreamResponseBuilder {
    fn default() -> Self {
        Self::new()
    }
}