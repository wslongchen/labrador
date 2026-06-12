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
//! 请求拦截器模块

use crate::errors::{LabraError, LabradorResult};
use crate::request::Request;
use crate::response::{Response, StreamResponse};
use async_trait::async_trait;
use tracing::{error, info};

/// 请求拦截器特质
#[async_trait]
pub trait RequestInterceptor: Send + Sync {
    /// 请求前拦截
    async fn before_request(&self, request: &mut Request) -> LabradorResult<()>;

    /// 响应后拦截
    async fn after_response(&self, response: &Response) -> LabradorResult<()>;

    /// 流式响应后拦截
    async fn after_stream_response(&self, _response: &StreamResponse) -> LabradorResult<()> {
        // 默认实现，什么也不做
        Ok(())
    }
}

/// 无操作拦截器
#[derive(Debug, Clone)]
pub struct NoopInterceptor;

impl Default for NoopInterceptor {
    fn default() -> Self {
        Self::new()
    }
}

impl NoopInterceptor {
    /// 创建新的无操作拦截器
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl RequestInterceptor for NoopInterceptor {
    async fn before_request(&self, _request: &mut Request) -> LabradorResult<()> {
        Ok(())
    }

    async fn after_response(&self, _response: &Response) -> LabradorResult<()> {
        Ok(())
    }
}

/// 日志拦截器
#[derive(Debug, Clone)]
pub struct LoggingInterceptor {
    /// 是否记录请求头
    log_headers: bool,
    /// 是否记录请求体
    log_body: bool,
}

impl LoggingInterceptor {
    /// 创建新的日志拦截器
    pub fn new() -> Self {
        Self {
            log_headers: true,
            log_body: false,
        }
    }

    /// 设置是否记录请求头
    pub fn with_log_headers(mut self, log_headers: bool) -> Self {
        self.log_headers = log_headers;
        self
    }

    /// 设置是否记录请求体
    pub fn with_log_body(mut self, log_body: bool) -> Self {
        self.log_body = log_body;
        self
    }
}

impl Default for LoggingInterceptor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RequestInterceptor for LoggingInterceptor {
    async fn before_request(&self, request: &mut Request) -> LabradorResult<()> {
        info!("[Request] {} {}", request.method(), request.path());

        if self.log_headers && !request.headers().is_empty() {
            info!("[Request Headers] {:?}", request.headers());
        }

        if self.log_body {
            match request.body() {
                crate::request::RequestBody::Json(json) => {
                    info!("[Request Body] JSON: {}", json);
                }
                crate::request::RequestBody::Form(form) => {
                    info!("[Request Body] Form: {:?}", form);
                }
                crate::request::RequestBody::Text(text) => {
                    info!("[Request Body] Text: {}", text);
                }
                crate::request::RequestBody::Xml(xml) => {
                    info!("[Request Body] XML: {}", xml);
                }
                _ => {}
            }
        }

        Ok(())
    }

    async fn after_response(&self, response: &Response) -> LabradorResult<()> {
        let status = response.status();

        if response.is_success() {
            info!("[Response] Status: {}", status);
        } else {
            error!("[Response] Status: {}", status);
        }

        if self.log_headers && !response.headers().is_empty() {
            info!("[Response Headers] {:?}", response.headers());
        }

        if self.log_body {
            if let Ok(text) = response.text() {
                if response.is_success() {
                    info!("[Response Body] {}", text);
                } else {
                    error!("[Response Body] {}", text);
                }
            }
        }

        Ok(())
    }
}

/// 认证拦截器
#[derive(Debug, Clone)]
pub struct AuthInterceptor {
    /// 令牌
    token: String,
    /// 头名称
    header_name: String,
}

impl AuthInterceptor {
    /// 创建新的认证拦截器
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            header_name: "Authorization".to_string(),
        }
    }

    /// 设置头名称
    pub fn with_header_name(mut self, header_name: impl Into<String>) -> Self {
        self.header_name = header_name.into();
        self
    }

    /// 创建Bearer Token认证拦截器
    pub fn bearer(token: impl Into<String>) -> Self {
        let mut interceptor = Self::new(token);
        interceptor.header_name = "Authorization".to_string();
        interceptor
    }
}

#[async_trait]
impl RequestInterceptor for AuthInterceptor {
    async fn before_request(&self, request: &mut Request) -> LabradorResult<()> {
        request.set_header(&self.header_name, format!("Bearer {}", self.token))?;
        Ok(())
    }

    async fn after_response(&self, _response: &Response) -> LabradorResult<()> {
        Ok(())
    }
}

/// 超时拦截器
#[derive(Debug, Clone)]
pub struct TimeoutInterceptor {
    /// 超时时间
    timeout: std::time::Duration,
}

impl TimeoutInterceptor {
    /// 创建新的超时拦截器
    pub fn new(timeout: std::time::Duration) -> Self {
        Self { timeout }
    }
}

#[async_trait]
impl RequestInterceptor for TimeoutInterceptor {
    async fn before_request(&self, _request: &mut Request) -> LabradorResult<()> {
        // 这里我们无法直接设置请求超时，因为 reqwest::Client 是在 ApiClient 中构建的
        // 这个拦截器主要用于演示，实际超时应该在 ClientConfig 中设置
        Ok(())
    }

    async fn after_response(&self, _response: &Response) -> LabradorResult<()> {
        Ok(())
    }
}

/// 请求限流拦截器
#[derive(Debug, Clone)]
pub struct RateLimitInterceptor {
    /// 最大请求数
    max_requests: u32,
    /// 时间窗口（秒）
    time_window: u64,
    /// 当前请求数
    current_requests: std::sync::Arc<tokio::sync::Mutex<u32>>,
    /// 最后重置时间
    last_reset: std::sync::Arc<tokio::sync::Mutex<std::time::Instant>>,
}

impl RateLimitInterceptor {
    /// 创建新的限流拦截器
    pub fn new(max_requests: u32, time_window_secs: u64) -> Self {
        Self {
            max_requests,
            time_window: time_window_secs,
            current_requests: std::sync::Arc::new(tokio::sync::Mutex::new(0)),
            last_reset: std::sync::Arc::new(tokio::sync::Mutex::new(std::time::Instant::now())),
        }
    }
}

#[async_trait]
impl RequestInterceptor for RateLimitInterceptor {
    async fn before_request(&self, _request: &mut Request) -> LabradorResult<()> {
        let mut current = self.current_requests.lock().await;
        let mut last_reset = self.last_reset.lock().await;

        // 检查是否需要重置计数器
        if last_reset.elapsed().as_secs() >= self.time_window {
            *current = 0;
            *last_reset = std::time::Instant::now();
        }

        // 检查是否超过限制
        if *current >= self.max_requests {
            return Err(LabraError::Other("Rate limit exceeded".to_string()));
        }

        // 增加计数器
        *current += 1;

        Ok(())
    }

    async fn after_response(&self, _response: &Response) -> LabradorResult<()> {
        Ok(())
    }
}
