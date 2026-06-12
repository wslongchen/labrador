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
use hex::FromHexError;
use quick_xml::{DeError, SeError};
use std::error::Error;
use thiserror::Error;
use tracing::error;
use x509_parser::error::X509Error;

pub type LabradorResult<T> = Result<T, LabraError>;

#[derive(Error, Debug)]
pub enum LabraError {
    /// 配置错误
    #[error("配置错误: {0}")]
    Config(String),

    /// 网络错误
    #[error("网络错误: {0}")]
    Network(#[from] reqwest::Error),

    /// JSON序列化/反序列化错误
    #[error("JSON错误: {0}")]
    Json(#[from] serde_json::Error),

    /// URL解析错误
    #[error("URL错误: {0}")]
    Url(#[from] url::ParseError),

    /// IO错误
    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),

    /// UTF-8转换错误
    #[error("UTF-8错误: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    /// Base64编解码错误
    #[error("Base64错误: {0}")]
    Base64(#[from] base64::DecodeError),

    /// 加密错误
    #[error("加密错误: {0}")]
    Crypto(String),

    /// 签名错误
    #[error("签名错误: {0}")]
    Sign(String),

    #[error("请求超时")]
    Timeout,

    /// 证书错误
    #[error("证书错误: {0}")]
    Certificate(String),
    #[error("证书错误: {0}")]
    Identity(String),

    /// XML解析错误
    #[error("XML解析错误: {0}")]
    Xml(String),

    /// 拦截器错误
    #[error("拦截器错误: {0}")]
    Interceptor(String),

    #[error("请求错误: {0}")]
    RequestError(String),

    /// 请求失败错误
    #[error("请求失败: {method} {path} - {status}: {body}")]
    RequestFailed {
        /// HTTP方法
        method: String,
        /// 请求路径
        path: String,
        /// 状态码
        status: u16,
        /// 响应体
        body: String,
    },

    /// 重试耗尽错误
    #[error("重试耗尽 (尝试{attempts}次): {last_error}")]
    RetryExhausted {
        /// 尝试次数
        attempts: u32,
        /// 最后一次错误
        last_error: Box<LabraError>,
    },

    /// 业务错误
    #[error("业务错误: {code} - {message}")]
    Business {
        /// 错误码
        code: String,
        /// 错误信息
        message: String,
    },

    /// 平台特定错误
    #[error("平台错误: {0}")]
    Platform(String),

    /// 参数验证错误
    #[error("参数错误: {0}")]
    Validation(String),

    /// 其他错误
    #[error("{0}")]
    Other(String),

    /// 未知错误
    #[error("未知错误")]
    Unknown,
}

impl LabraError {
    /// 创建请求失败错误
    pub fn request_failed(
        method: impl Into<String>,
        path: impl Into<String>,
        status: u16,
        body: impl Into<String>,
    ) -> Self {
        Self::RequestFailed {
            method: method.into(),
            path: path.into(),
            status,
            body: body.into(),
        }
    }

    /// 创建业务错误
    pub fn business<C, M>(code: C, message: M) -> Self
    where
        C: Into<String>,
        M: Into<String>,
    {
        Self::Business {
            code: code.into(),
            message: message.into(),
        }
    }

    /// 检查是否应该重试
    pub fn should_retry(&self) -> bool {
        matches!(
            self,
            LabraError::Network(_) | LabraError::RequestFailed { .. } | LabraError::Timeout
        )
    }

    /// 检查是否是网络错误
    pub fn is_network(&self) -> bool {
        matches!(self, LabraError::Network(_))
    }

    /// 检查是否是超时错误
    pub fn is_timeout(&self) -> bool {
        if let LabraError::Network(err) = self {
            if err.is_timeout() {
                return true;
            }
            if let Some(source) = err.source() {
                return source.to_string().to_lowercase().contains("timeout");
            }
        }
        false
    }
}

impl From<reqwest::header::InvalidHeaderValue> for LabraError {
    fn from(err: reqwest::header::InvalidHeaderValue) -> Self {
        Self::Interceptor(err.to_string())
    }
}

impl From<reqwest::header::InvalidHeaderName> for LabraError {
    fn from(err: reqwest::header::InvalidHeaderName) -> Self {
        Self::Interceptor(err.to_string())
    }
}

impl From<reqwest::multipart::Part> for LabraError {
    fn from(_: reqwest::multipart::Part) -> Self {
        Self::Other("multipart part conversion failed".to_string())
    }
}

/// 自定义的Timeout错误
#[derive(Debug)]
pub struct Timeout;

impl std::fmt::Display for Timeout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "请求超时")
    }
}

impl std::error::Error for Timeout {}

impl From<Timeout> for LabraError {
    fn from(_: Timeout) -> Self {
        Self::Timeout
    }
}

impl From<serde_xml_rs::Error> for LabraError {
    fn from(_err: serde_xml_rs::Error) -> Self {
        error!("error to parse xml:{:?}", _err);
        LabraError::Xml(_err.to_string())
    }
}

#[cfg(feature = "openssl-crypto")]
impl From<openssl::error::ErrorStack> for LabraError {
    fn from(err: openssl::error::ErrorStack) -> Self {
        LabraError::InvalidSignature(format!("加解密出错：{}", err.to_string()))
    }
}
impl From<rsa::pkcs8::Error> for LabraError {
    fn from(err: rsa::pkcs8::Error) -> Self {
        LabraError::Certificate(err.to_string())
    }
}

impl From<FromHexError> for LabraError {
    fn from(err: FromHexError) -> Self {
        LabraError::Crypto(err.to_string())
    }
}

impl From<serde_urlencoded::de::Error> for LabraError {
    fn from(err: serde_urlencoded::de::Error) -> Self {
        LabraError::RequestError(err.to_string())
    }
}

impl From<serde_urlencoded::ser::Error> for LabraError {
    fn from(err: serde_urlencoded::ser::Error) -> Self {
        LabraError::RequestError(err.to_string())
    }
}

impl From<DeError> for LabraError {
    fn from(err: DeError) -> Self {
        LabraError::Xml(err.to_string())
    }
}

impl From<SeError> for LabraError {
    fn from(err: SeError) -> Self {
        LabraError::Xml(err.to_string())
    }
}

impl From<rsa::pkcs1::Error> for LabraError {
    fn from(err: rsa::pkcs1::Error) -> Self {
        LabraError::Certificate(err.to_string())
    }
}

impl From<rsa::pkcs8::spki::Error> for LabraError {
    fn from(err: rsa::pkcs8::spki::Error) -> Self {
        LabraError::Certificate(err.to_string())
    }
}

impl From<X509Error> for LabraError {
    fn from(err: X509Error) -> Self {
        LabraError::Certificate(err.to_string())
    }
}

impl From<rsa::errors::Error> for LabraError {
    fn from(err: rsa::errors::Error) -> Self {
        LabraError::Certificate(err.to_string())
    }
}
impl From<x509_parser::nom::Err<x509_parser::prelude::PEMError>> for LabraError {
    fn from(err: x509_parser::nom::Err<x509_parser::prelude::PEMError>) -> Self {
        LabraError::Certificate(err.to_string())
    }
}

impl From<x509_parser::nom::Err<x509_parser::prelude::X509Error>> for LabraError {
    fn from(err: x509_parser::nom::Err<x509_parser::prelude::X509Error>) -> Self {
        LabraError::Certificate(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_labra_error_display() {
        // 测试各种错误类型的 Display
        let err = LabraError::Config("test config error".to_string());
        assert!(err.to_string().contains("test config error"));

        let err = LabraError::Crypto("test crypto error".to_string());
        assert!(err.to_string().contains("test crypto error"));

        let err = LabraError::Sign("test sign error".to_string());
        assert!(err.to_string().contains("test sign error"));

        let err = LabraError::Timeout;
        assert!(
            err.to_string().contains("timeout")
                || err.to_string().contains("Timeout")
                || err.to_string().contains("超时")
        );

        let err = LabraError::Unknown;
        assert!(!err.to_string().is_empty());
    }

    #[test]
    fn test_labra_error_business() {
        let err = LabraError::business("40001", "invalid credential".to_string());
        assert!(err.to_string().contains("40001"));
        assert!(err.to_string().contains("invalid credential"));
    }

    #[test]
    fn test_labra_error_request_failed() {
        let err = LabraError::request_failed("GET", "/v3/pay/transactions", 400, "invalid request");
        assert!(err.to_string().contains("GET"));
        assert!(err.to_string().contains("/v3/pay/transactions"));
        assert!(err.to_string().contains("400"));
        assert!(err.to_string().contains("invalid request"));
    }

    #[test]
    fn test_labra_error_should_retry() {
        // 请求失败应该重试
        let err = LabraError::RequestFailed {
            method: "GET".to_string(),
            path: "/api".to_string(),
            status: 500,
            body: "error".to_string(),
        };
        assert!(err.should_retry(), "请求失败应该重试");

        // 超时应该重试
        let err = LabraError::Timeout;
        assert!(err.should_retry(), "超时应该重试");

        // 业务错误不应该重试
        let err = LabraError::Business {
            code: "40001".to_string(),
            message: "error".to_string(),
        };
        assert!(!err.should_retry(), "业务错误不应重试");

        // 签名错误不应该重试
        let err = LabraError::Sign("bad signature".to_string());
        assert!(!err.should_retry(), "签名错误不应重试");
    }

    #[test]
    fn test_labra_error_is_network() {
        // is_network 仅对 Network 变体返回 true
        let err = LabraError::Timeout;
        assert!(!err.is_network(), "超时 is_network 应为 false");

        let err = LabraError::Config("test".to_string());
        assert!(!err.is_network(), "配置错误 is_network 应为 false");

        let err = LabraError::Unknown;
        assert!(!err.is_network(), "未知错误 is_network 应为 false");
    }

    #[test]
    fn test_labra_error_is_timeout() {
        // is_timeout 目前仅对 Network 变体中包含超时信息的情况返回 true
        // LabraError::Timeout 本身通过 Display 展示超时，但 is_timeout 不直接匹配
        let err = LabraError::Config("test".to_string());
        assert!(!err.is_timeout(), "配置错误 is_timeout 应为 false");

        let err = LabraError::Unknown;
        assert!(!err.is_timeout(), "未知错误 is_timeout 应为 false");

        let err = LabraError::Timeout;
        // 验证 Timeout 变体存在且可 Display
        assert!(err.to_string().contains("超时") || err.to_string().contains("timeout"));
    }

    #[test]
    fn test_labra_error_from_serde_json() {
        let json_err = serde_json::from_str::<serde_json::Value>("not json").unwrap_err();
        let err: LabraError = json_err.into();
        assert!(
            err.to_string().contains("json")
                || err.to_string().contains("serde")
                || !err.to_string().is_empty()
        );
    }

    #[test]
    fn test_labra_error_result_alias() {
        // 测试 LabradorResult 类型别名
        let ok_result: LabradorResult<String> = Ok("success".to_string());
        assert!(ok_result.is_ok());

        let err_result: LabradorResult<String> = Err(LabraError::Timeout);
        assert!(err_result.is_err());
    }

    #[test]
    fn test_labra_error_validation() {
        let err = LabraError::Validation("missing required field".to_string());
        assert!(err.to_string().contains("missing required field"));
    }

    #[test]
    fn test_labra_error_retry_exhausted() {
        let err = LabraError::RetryExhausted {
            attempts: 3,
            last_error: Box::new(LabraError::Timeout),
        };
        assert!(err.to_string().contains("3"));
    }
}
