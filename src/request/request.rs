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
//! HTTP请求实现

use super::{HttpMethod, RequestBody};
use crate::errors::{LabraError, LabradorResult};
use http::{header, HeaderMap, HeaderValue};
use std::convert::TryFrom;
use std::fmt;
use std::fmt::Formatter;

/// HTTP请求
#[derive(Clone)]
pub struct Request {
    /// HTTP方法
    pub(crate) method: HttpMethod,
    /// 请求路径
    pub(crate) path: String,
    /// 查询参数
    pub(crate) query_params: Vec<(String, String)>,
    /// 请求头
    pub(crate) headers: HeaderMap,
    /// 请求体
    pub(crate) body: RequestBody,
}

impl fmt::Debug for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Request")
            .field("method", &self.method)
            .field("path", &self.path)
            .field("query_params", &self.query_params)
            .field("headers", &self.headers)
            .field("body", &self.body)
            .finish()
    }
}

impl Request {
    /// 创建请求构建器
    pub fn builder() -> RequestBuilder {
        RequestBuilder::default()
    }

    /// 获取请求方法
    pub fn method(&self) -> HttpMethod {
        self.method
    }

    /// 获取请求路径
    pub fn path(&self) -> &str {
        &self.path
    }

    /// 获取查询参数
    pub fn query_params(&self) -> &[(String, String)] {
        &self.query_params
    }

    /// 获取请求头
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// 获取请求体
    pub fn body(&self) -> &RequestBody {
        &self.body
    }

    /// 获取可变请求体
    pub fn body_mut(&mut self) -> &mut RequestBody {
        &mut self.body
    }

    /// 设置请求头
    pub fn set_header<K, V>(&mut self, key: K, value: V) -> LabradorResult<()>
    where
        header::HeaderName: TryFrom<K>,
        <header::HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        let header_name = header::HeaderName::try_from(key)
            .map_err(|e| LabraError::Interceptor(e.into().to_string()))?;
        let header_value = HeaderValue::try_from(value)
            .map_err(|e| LabraError::Interceptor(e.into().to_string()))?;

        self.headers.insert(header_name, header_value);
        Ok(())
    }

    /// 添加查询参数
    pub fn add_query_param<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.query_params.push((key.into(), value.into()));
    }
}

/// 请求构建器
#[derive(Debug, Default)]
pub struct RequestBuilder {
    method: Option<HttpMethod>,
    path: Option<String>,
    query_params: Vec<(String, String)>,
    headers: HeaderMap,
    body: Option<RequestBody>,
}

impl RequestBuilder {
    /// 设置HTTP方法
    pub fn method(mut self, method: HttpMethod) -> Self {
        self.method = Some(method);
        self
    }

    /// 设置请求路径
    pub fn path<I: AsRef<str>>(mut self, path: I) -> Self {
        self.path = Some(path.as_ref().to_string());
        self
    }

    /// 添加查询参数
    pub fn query_params<P: IntoIterator<Item = (String, String)>>(mut self, params: P) -> Self {
        self.query_params.extend(params);
        self
    }

    /// 添加单个查询参数
    pub fn query_param<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 设置请求头
    pub fn header<K, V>(mut self, key: K, value: V) -> Self
    where
        header::HeaderName: TryFrom<K>,
        <header::HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        if let (Ok(key), Ok(value)) = (
            header::HeaderName::try_from(key).map_err(|e| e.into()),
            HeaderValue::try_from(value).map_err(|e| e.into()),
        ) {
            self.headers.insert(key, value);
        }
        self
    }

    /// 设置请求体
    pub fn body<B: Into<RequestBody>>(mut self, body: B) -> Self {
        self.body = Some(body.into());
        self
    }

    /// 构建请求
    pub fn build(self) -> Request {
        Request {
            method: self.method.unwrap_or(HttpMethod::Get),
            path: self.path.unwrap_or_default(),
            query_params: self.query_params,
            headers: self.headers,
            body: self.body.unwrap_or(RequestBody::Empty),
        }
    }
}

impl From<RequestBuilder> for Request {
    fn from(builder: RequestBuilder) -> Self {
        builder.build()
    }
}
