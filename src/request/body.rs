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

//! HTTP请求体实现

use bytes::Bytes;
use reqwest::multipart;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::Formatter;

/// 请求内容类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentType {
    /// JSON格式
    Json,
    /// 表单格式
    Form,
    /// 多部分表单格式
    Multipart,
    /// XML格式
    Xml,
    /// 文本格式
    Text,
    /// 二进制格式
    Binary,
}

impl ContentType {
    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            ContentType::Json => "application/json",
            ContentType::Form => "application/x-www-form-urlencoded",
            ContentType::Multipart => "multipart/form-data",
            ContentType::Xml => "application/xml",
            ContentType::Text => "text/plain",
            ContentType::Binary => "application/octet-stream",
        }
    }

    /// 带有字符集的Content-Type
    pub fn with_charset(&self) -> String {
        format!("{}; charset=UTF-8", self.as_str())
    }

    /// 从字符串创建ContentType
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "application/json" | "json" => Some(ContentType::Json),
            "application/x-www-form-urlencoded" | "form" => Some(ContentType::Form),
            "multipart/form-data" | "multipart" => Some(ContentType::Multipart),
            "application/xml" | "text/xml" | "xml" => Some(ContentType::Xml),
            "text/plain" | "text" => Some(ContentType::Text),
            "application/octet-stream" | "binary" => Some(ContentType::Binary),
            _ => None,
        }
    }
}

impl std::fmt::Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// HTTP请求体
pub enum RequestBody {
    /// JSON格式请求体
    Json(serde_json::Value),
    /// 表单格式请求体
    Form(BTreeMap<String, String>),
    /// 多部分表单请求体
    Multipart(multipart::Form),
    /// XML格式请求体
    Xml(String),
    /// 文本格式请求体
    Text(String),
    /// 二进制请求体
    Binary(Bytes),
    /// 空请求体
    Empty,
}

impl fmt::Debug for RequestBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RequestBody::Json(s) => write!(f, "{}", s),
            RequestBody::Form(v) => {
                for (k, v) in v {
                    write!(f, "{}={}&", k, v)?;
                }
                Ok(())
            }
            RequestBody::Multipart(_) => write!(f, "Multipart/Form-Data"),
            RequestBody::Xml(s) => write!(f, "{}", s),
            RequestBody::Text(s) => write!(f, "{}", s),
            RequestBody::Binary(_bytes) => write!(f, "Bytes Body..."),
            RequestBody::Empty => write!(f, "Empty Body"),
        }
    }
}

impl Clone for RequestBody {
    fn clone(&self) -> Self {
        match self {
            RequestBody::Json(s) => RequestBody::Json(s.clone()),
            RequestBody::Form(v) => RequestBody::Form(v.clone()),
            RequestBody::Multipart(_form) => {
                // 对于 multipart，我们创建一个新的空表单
                let new_form = multipart::Form::new();
                RequestBody::Multipart(new_form)
            }
            RequestBody::Xml(s) => RequestBody::Xml(s.clone()),
            RequestBody::Text(s) => RequestBody::Text(s.clone()),
            RequestBody::Binary(bytes) => RequestBody::Binary(bytes.clone()),
            RequestBody::Empty => RequestBody::Empty,
        }
    }
}

impl RequestBody {
    /// 获取内容类型
    pub fn content_type(&self) -> ContentType {
        match self {
            RequestBody::Json(_) => ContentType::Json,
            RequestBody::Form(_) => ContentType::Form,
            RequestBody::Multipart(_) => ContentType::Multipart,
            RequestBody::Xml(_) => ContentType::Xml,
            RequestBody::Text(_) => ContentType::Text,
            RequestBody::Binary(_) => ContentType::Binary,
            RequestBody::Empty => ContentType::Json,
        }
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        matches!(self, RequestBody::Empty)
    }

    /// 获取长度（字节数）
    pub fn len(&self) -> usize {
        match self {
            RequestBody::Json(s) => s.as_str().map(|s| s.len()).unwrap_or(0),
            RequestBody::Form(v) => v.iter().map(|(k, v)| k.len() + v.len()).sum(),
            RequestBody::Multipart(_) => 0, // 难以计算
            RequestBody::Xml(s) => s.len(),
            RequestBody::Text(s) => s.len(),
            RequestBody::Binary(b) => b.len(),
            RequestBody::Empty => 0,
        }
    }

    /// 创建JSON请求体
    pub fn json<T: Serialize>(value: &T) -> Result<Self, serde_json::Error> {
        let json = serde_json::to_value(value)?;
        Ok(RequestBody::Json(json))
    }

    /// 创建表单请求体
    pub fn form(data: impl Into<BTreeMap<String, String>>) -> Self {
        RequestBody::Form(data.into())
    }

    /// 创建XML请求体
    pub fn xml(data: impl Into<String>) -> Self {
        RequestBody::Xml(data.into())
    }

    /// 创建文本请求体
    pub fn text(data: impl Into<String>) -> Self {
        RequestBody::Text(data.into())
    }

    /// 创建二进制请求体
    pub fn binary(data: impl Into<Bytes>) -> Self {
        RequestBody::Binary(data.into())
    }

    /// 创建多部分表单请求体
    pub fn multipart() -> Self {
        RequestBody::Multipart(multipart::Form::new())
    }

    /// 转换为JSON字符串（如果是JSON格式）
    pub fn as_json(&self) -> Option<&str> {
        match self {
            RequestBody::Json(s) => Some(s.as_str().unwrap_or("")),
            RequestBody::Text(s) => Some(s),
            RequestBody::Xml(s) => Some(s),
            _ => None,
        }
    }

    /// 转换为表单数据（如果是Form格式）
    pub fn as_form(&self) -> Option<&BTreeMap<String, String>> {
        match self {
            RequestBody::Form(v) => Some(v),
            _ => None,
        }
    }

    /// 转换为文本（如果是Text格式）
    pub fn as_text(&self) -> Option<&str> {
        match self {
            RequestBody::Text(s) => Some(s),
            RequestBody::Json(s) => Some(s.as_str().unwrap_or("")),
            RequestBody::Xml(s) => Some(s),
            _ => None,
        }
    }

    /// 返回用于签名计算的规范字符串表示。
    ///
    /// JSON body 返回完整序列化 JSON（如 `{"appid":"...","amount":{...}}`），
    /// 避免 `as_text` 对 `serde_json::Value::Object` 返回空串导致签名不匹配。
    pub fn to_sign_string(&self) -> String {
        match self {
            RequestBody::Json(v) => v.to_string(),
            RequestBody::Text(s) => s.clone(),
            RequestBody::Xml(s) => s.clone(),
            _ => String::new(),
        }
    }
}

impl<T: Serialize> From<T> for RequestBody {
    fn from(value: T) -> Self {
        match serde_json::to_value(&value) {
            Ok(json) => RequestBody::Json(json),
            Err(_) => RequestBody::Empty,
        }
    }
}
