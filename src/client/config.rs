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
//! 客户端配置模块

use crate::client::builder::ClientBuilder;
use crate::client::certificate::Certificate;
use crate::client::identity::Identity;
use crate::response::Response;
use base64::Engine;
use reqwest::{Certificate as ReqwestCertificate, Proxy};
use std::time::Duration;

/// TLS配置
#[derive(Debug, Clone, Default)]
pub struct TlsConfig {
    /// 最小TLS协议版本
    pub min_protocol_version: Option<reqwest::tls::Version>,
    /// 最大TLS协议版本
    pub max_protocol_version: Option<reqwest::tls::Version>,
    /// 支持的加密套件
    pub ciphers: Vec<String>,
    /// 根证书
    pub root_certificates: Vec<ReqwestCertificate>,
}

/// 重试配置
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// 最大重试次数
    pub max_retries: u32,
    /// 初始延迟
    pub initial_delay: Duration,
    /// 最大延迟
    pub max_delay: Duration,
    /// 退避乘数
    pub backoff_multiplier: f32,
    /// 可重试的状态码
    pub retryable_statuses: Vec<reqwest::StatusCode>,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0,
            retryable_statuses: vec![
                reqwest::StatusCode::TOO_MANY_REQUESTS,
                reqwest::StatusCode::INTERNAL_SERVER_ERROR,
                reqwest::StatusCode::BAD_GATEWAY,
                reqwest::StatusCode::SERVICE_UNAVAILABLE,
                reqwest::StatusCode::GATEWAY_TIMEOUT,
                reqwest::StatusCode::REQUEST_TIMEOUT,
            ],
        }
    }
}

impl RetryConfig {
    /// 检查响应是否应该重试
    pub fn should_retry(&self, response: &Response) -> bool {
        self.retryable_statuses.contains(&response.status())
    }

    /// 获取第N次重试的延迟时间
    pub fn delay_for_attempt(&self, attempt: u32) -> Duration {
        let delay =
            self.initial_delay.as_secs_f32() * self.backoff_multiplier.powi(attempt as i32 - 1);
        let delay_ms = (delay * 1000.0) as u64;
        Duration::from_millis(delay_ms.min(self.max_delay.as_millis() as u64))
    }
}

/// 认证头配置
#[derive(Debug, Clone)]
pub struct AuthHeader {
    /// 头名称
    pub name: String,
    /// 头值
    pub value: String,
}

impl AuthHeader {
    /// 创建新的认证头
    pub fn new<N, V>(name: N, value: V) -> Self
    where
        N: Into<String>,
        V: Into<String>,
    {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }

    /// 创建Bearer Token认证头
    pub fn bearer_token(token: impl Into<String>) -> Self {
        Self {
            name: "Authorization".to_string(),
            value: format!("Bearer {}", token.into()),
        }
    }

    /// 创建Basic认证头
    pub fn basic_auth(username: impl Into<String>, password: impl Into<String>) -> Self {
        let credentials = format!("{}:{}", username.into(), password.into());
        let encoded = base64::engine::general_purpose::STANDARD.encode(credentials);

        Self {
            name: "Authorization".to_string(),
            value: format!("Basic {}", encoded),
        }
    }
}

/// 客户端配置
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// 应用密钥
    pub app_key: String,
    /// 应用密钥
    pub secret: String,
    /// API基础URL
    pub api_base_url: String,
    /// 用户代理
    pub user_agent: Option<String>,
    /// 请求超时
    pub timeout: Option<Duration>,
    /// 连接超时
    pub connect_timeout: Option<Duration>,
    /// 连接池空闲超时
    pub pool_idle_timeout: Option<Duration>,
    /// 每个主机的最大空闲连接数
    pub pool_max_idle_per_host: Option<usize>,
    /// HTTP代理
    pub proxy: Option<Proxy>,
    /// TLS配置
    pub tls_config: Option<TlsConfig>,
    /// 客户端身份
    pub identity: Option<Identity>,
    /// 根证书
    pub root_certificates: Vec<Certificate>,
    /// 认证头
    pub auth_header: Option<AuthHeader>,
    /// 重试配置
    pub retry_config: RetryConfig,
    /// 是否自动重试
    pub auto_retry: bool,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            app_key: String::new(),
            secret: String::new(),
            api_base_url: String::new(),
            user_agent: None,
            timeout: Some(Duration::from_secs(30)),
            connect_timeout: Some(Duration::from_secs(10)),
            pool_idle_timeout: Some(Duration::from_secs(90)),
            pool_max_idle_per_host: Some(1),
            proxy: None,
            tls_config: None,
            identity: None,
            root_certificates: Vec::new(),
            auth_header: None,
            retry_config: RetryConfig::default(),
            auto_retry: false,
        }
    }
}

impl ClientConfig {
    /// 创建配置构建器
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }
}
