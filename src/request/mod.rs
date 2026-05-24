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

//! HTTP请求相关模块

mod request;
mod body;

pub use body::{RequestBody};
pub use request::{Request};

/// HTTP请求方法
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    /// GET方法
    Get,
    /// POST方法
    Post,
    /// PUT方法
    Put,
    /// DELETE方法
    Delete,
    /// HEAD方法
    Head,
    /// OPTIONS方法
    Options,
    /// PATCH方法
    Patch,
    /// TRACE方法
    Trace,
    /// CONNECT方法
    Connect,
}

impl From<HttpMethod> for reqwest::Method {
    fn from(method: HttpMethod) -> Self {
        match method {
            HttpMethod::Get => reqwest::Method::GET,
            HttpMethod::Post => reqwest::Method::POST,
            HttpMethod::Put => reqwest::Method::PUT,
            HttpMethod::Delete => reqwest::Method::DELETE,
            HttpMethod::Head => reqwest::Method::HEAD,
            HttpMethod::Options => reqwest::Method::OPTIONS,
            HttpMethod::Patch => reqwest::Method::PATCH,
            HttpMethod::Trace => reqwest::Method::TRACE,
            HttpMethod::Connect => reqwest::Method::CONNECT,
        }
    }
}

impl From<reqwest::Method> for HttpMethod {
    fn from(method: reqwest::Method) -> Self {
        match method {
            reqwest::Method::GET => HttpMethod::Get,
            reqwest::Method::POST => HttpMethod::Post,
            reqwest::Method::PUT => HttpMethod::Put,
            reqwest::Method::DELETE => HttpMethod::Delete,
            reqwest::Method::HEAD => HttpMethod::Head,
            reqwest::Method::OPTIONS => HttpMethod::Options,
            reqwest::Method::PATCH => HttpMethod::Patch,
            reqwest::Method::TRACE => HttpMethod::Trace,
            reqwest::Method::CONNECT => HttpMethod::Connect,
            _ => HttpMethod::Get,
        }
    }
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Head => "HEAD",
            HttpMethod::Options => "OPTIONS",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Trace => "TRACE",
            HttpMethod::Connect => "CONNECT",
        })
    }
}