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
//! 请求签名器模块

use crate::errors::{LabraError, LabradorResult};
use crate::request::{Request, RequestBody};
use crate::response::Response;
use hmac::{Hmac, Mac};
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use std::any::Any;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// 签名方法
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignMethod {
    /// MD5签名
    Md5,
    /// SHA1签名
    Sha1,
    /// SHA256签名
    Sha256,
    /// SHA512签名
    Sha512,
    /// HMAC-SHA256签名
    HmacSha256,
    /// HMAC-SHA512签名
    HmacSha512,
    /// RSA-SHA256签名
    RsaSha256,
}

impl SignMethod {
    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            SignMethod::Md5 => "MD5",
            SignMethod::Sha1 => "SHA1",
            SignMethod::Sha256 => "SHA256",
            SignMethod::Sha512 => "SHA512",
            SignMethod::HmacSha256 => "HMAC-SHA256",
            SignMethod::HmacSha512 => "HMAC-SHA512",
            SignMethod::RsaSha256 => "RSA-SHA256",
        }
    }

    /// 是否是HMAC签名
    pub fn is_hmac(&self) -> bool {
        matches!(self, SignMethod::HmacSha256 | SignMethod::HmacSha512)
    }

    /// 是否是哈希签名
    pub fn is_hash(&self) -> bool {
        matches!(
            self,
            SignMethod::Md5 | SignMethod::Sha1 | SignMethod::Sha256 | SignMethod::Sha512
        )
    }
}

/// 默认签名器配置
#[derive(Debug, Clone)]
pub struct SignerConfig {
    /// 应用密钥
    pub app_key: String,
    /// 应用密钥
    pub app_secret: String,
    /// 签名方法
    pub sign_method: SignMethod,
    /// 签名字段名
    pub sign_field: String,
    /// 时间戳字段名
    pub timestamp_field: String,
    /// 随机数字段名
    pub nonce_field: String,
    /// 版本字段名
    pub version_field: String,
    /// 密钥字段名
    pub key_field: String,
    /// 排除字段
    pub exclude_fields: Vec<String>,
}

impl Default for SignerConfig {
    fn default() -> Self {
        Self {
            app_key: String::new(),
            app_secret: String::new(),
            sign_method: SignMethod::Sha256,
            sign_field: "sign".to_string(),
            timestamp_field: "timestamp".to_string(),
            nonce_field: "nonce".to_string(),
            version_field: "version".to_string(),
            key_field: "app_key".to_string(),
            exclude_fields: vec!["sign".to_string(), "sig".to_string()],
        }
    }
}

/// 请求签名器特质
pub trait RequestSigner: Send + Sync {
    /// 签名请求
    fn sign_request(&self, request: &mut Request) -> LabradorResult<()>;

    /// 获取签名方法
    fn sign_method(&self) -> SignMethod;

    /// 验证签名
    fn verify_signature(&self, _response: &Response) -> LabradorResult<bool> {
        Ok(true)
    }

    /// 获取签名密钥
    fn secret_key(&self) -> &str;

    fn as_any(&self) -> &dyn Any;
}

/// 默认签名器
#[derive(Debug, Clone)]
pub struct DefaultSigner {
    config: SignerConfig,
}

impl DefaultSigner {
    /// 创建新的签名器
    pub fn new(app_key: impl Into<String>, app_secret: impl Into<String>) -> Self {
        Self {
            config: SignerConfig {
                app_key: app_key.into(),
                app_secret: app_secret.into(),
                ..SignerConfig::default()
            },
        }
    }

    /// 使用配置创建签名器
    pub fn with_config(config: SignerConfig) -> Self {
        Self { config }
    }

    /// 设置签名方法
    pub fn with_sign_method(mut self, sign_method: SignMethod) -> Self {
        self.config.sign_method = sign_method;
        self
    }

    /// 设置签名字段名
    pub fn with_sign_field(mut self, sign_field: impl Into<String>) -> Self {
        self.config.sign_field = sign_field.into();
        self
    }

    /// 设置时间戳字段名
    pub fn with_timestamp_field(mut self, timestamp_field: impl Into<String>) -> Self {
        self.config.timestamp_field = timestamp_field.into();
        self
    }

    /// 设置随机数字段名
    pub fn with_nonce_field(mut self, nonce_field: impl Into<String>) -> Self {
        self.config.nonce_field = nonce_field.into();
        self
    }

    /// 设置版本字段名
    pub fn with_version_field(mut self, version_field: impl Into<String>) -> Self {
        self.config.version_field = version_field.into();
        self
    }

    /// 设置密钥字段名
    pub fn with_key_field(mut self, key_field: impl Into<String>) -> Self {
        self.config.key_field = key_field.into();
        self
    }

    /// 排除字段
    pub fn exclude_fields(mut self, fields: Vec<String>) -> Self {
        self.config.exclude_fields.extend(fields);
        self
    }

    /// 生成时间戳
    fn generate_timestamp(&self) -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64
    }

    /// 生成随机字符串
    fn generate_nonce(&self) -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789";
        let mut rng = rand::thread_rng();
        (0..16)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    /// 收集所有待签名参数
    fn collect_params(&self, request: &Request) -> BTreeMap<String, String> {
        let mut params = BTreeMap::new();

        // 添加查询参数
        for (key, value) in &request.query_params {
            if !self.config.exclude_fields.contains(key) {
                params.insert(key.clone(), value.clone());
            }
        }

        // 添加请求体参数（如果是Form格式）
        if let RequestBody::Form(form_data) = &request.body {
            for (key, value) in form_data {
                if !self.config.exclude_fields.contains(key) {
                    params.insert(key.clone(), value.clone());
                }
            }
        }

        // 添加JSON参数（如果需要）
        if let RequestBody::Json(json) = &request.body {
            self.extract_json_params(&json, "", &mut params);
        }

        params
    }

    /// 从JSON中提取参数
    fn extract_json_params(
        &self,
        value: &serde_json::Value,
        prefix: &str,
        params: &mut BTreeMap<String, String>,
    ) {
        match value {
            serde_json::Value::Object(obj) => {
                for (key, val) in obj {
                    let new_prefix = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", prefix, key)
                    };
                    self.extract_json_params(val, &new_prefix, params);
                }
            }
            serde_json::Value::Array(arr) => {
                for (i, val) in arr.iter().enumerate() {
                    let new_prefix = format!("{}[{}]", prefix, i);
                    self.extract_json_params(val, &new_prefix, params);
                }
            }
            serde_json::Value::String(s) => {
                if !self.config.exclude_fields.contains(&prefix.to_string()) {
                    params.insert(prefix.to_string(), s.clone());
                }
            }
            serde_json::Value::Number(n) => {
                if !self.config.exclude_fields.contains(&prefix.to_string()) {
                    params.insert(prefix.to_string(), n.to_string());
                }
            }
            serde_json::Value::Bool(b) => {
                if !self.config.exclude_fields.contains(&prefix.to_string()) {
                    params.insert(prefix.to_string(), b.to_string());
                }
            }
            serde_json::Value::Null => {
                // 忽略null值
            }
        }
    }

    /// 生成签名字符串
    fn generate_sign_string(&self, params: &BTreeMap<String, String>) -> String {
        params
            .iter()
            .filter(|(k, v)| !k.is_empty() && !v.is_empty())
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("&")
    }

    /// 计算签名
    fn calculate_signature(&self, sign_string: &str) -> LabradorResult<String> {
        match self.config.sign_method {
            SignMethod::Md5 => {
                let final_hasher =
                    md5::compute(format!("{}{}", sign_string, self.config.app_secret));
                Ok(hex::encode(*final_hasher))
            }
            SignMethod::Sha1 => {
                use sha1::Digest;
                let mut hasher = Sha1::new();
                hasher.update(sign_string.as_bytes());
                hasher.update(self.config.app_secret.as_bytes());
                let result = hasher.finalize();
                Ok(hex::encode(result))
            }
            SignMethod::Sha256 => {
                use sha2::Digest;
                let mut hasher = Sha256::new();
                hasher.update(sign_string.as_bytes());
                hasher.update(self.config.app_secret.as_bytes());
                let result = hasher.finalize();
                Ok(hex::encode(result))
            }
            SignMethod::Sha512 => {
                use sha2::Digest;
                let mut hasher = Sha512::new();
                hasher.update(sign_string.as_bytes());
                hasher.update(self.config.app_secret.as_bytes());
                let result = hasher.finalize();
                Ok(hex::encode(result))
            }
            SignMethod::HmacSha256 => {
                type HmacSha256 = Hmac<Sha256>;
                let mut mac = HmacSha256::new_from_slice(self.config.app_secret.as_bytes())
                    .map_err(|e| LabraError::Sign(format!("HMAC key error: {}", e)))?;
                mac.update(sign_string.as_bytes());
                let result = mac.finalize();
                let bytes = result.into_bytes();
                Ok(hex::encode(bytes))
            }
            SignMethod::HmacSha512 => {
                type HmacSha512 = Hmac<Sha512>;
                let mut mac = HmacSha512::new_from_slice(self.config.app_secret.as_bytes())
                    .map_err(|e| LabraError::Sign(format!("HMAC key error: {}", e)))?;
                mac.update(sign_string.as_bytes());
                let result = mac.finalize();
                let bytes = result.into_bytes();
                Ok(hex::encode(bytes))
            }
            SignMethod::RsaSha256 => {
                // RSA-SHA256 签名使用 app_secret 作为私钥
                use crate::{HashType, RsaEncryptor, RsaKeyFormat};
                let encryptor = RsaEncryptor::with_private_key(
                    self.config.app_secret.as_bytes(),
                    RsaKeyFormat::Pem,
                );
                let sig = encryptor
                    .sign(sign_string.as_bytes(), HashType::Sha256)
                    .map_err(|e| LabraError::Sign(format!("RSA签名失败: {}", e)))?;
                use crate::utils::encryption::base64_encode;
                Ok(base64_encode(&sig))
            }
        }
    }

    /// 添加系统参数到请求
    fn add_system_params(&self, request: &mut Request) {
        let timestamp = self.generate_timestamp();
        let nonce = self.generate_nonce();

        // 添加系统参数到查询参数
        request.add_query_param(self.config.key_field.clone(), self.config.app_key.clone());
        request.add_query_param(self.config.timestamp_field.clone(), timestamp.to_string());
        request.add_query_param(self.config.nonce_field.clone(), nonce);
        request.add_query_param(self.config.version_field.clone(), "1.0".to_string());
    }
}

impl RequestSigner for DefaultSigner {
    fn sign_request(&self, request: &mut Request) -> LabradorResult<()> {
        // 添加系统参数
        self.add_system_params(request);

        // 收集所有参数
        let params = self.collect_params(request);

        // 生成签名字符串
        let sign_string = self.generate_sign_string(&params);

        // 计算签名
        let signature = self.calculate_signature(&sign_string)?;

        // 将签名添加到请求参数中
        request.add_query_param(self.config.sign_field.clone(), signature);

        Ok(())
    }

    fn sign_method(&self) -> SignMethod {
        self.config.sign_method
    }

    fn verify_signature(&self, _response: &Response) -> LabradorResult<bool> {
        // TODO: 收集参数（排除签名字段）
        // let params = self.collect_params(request);
        //
        // // 生成签名字符串并计算签名
        // let sign_string = self.generate_sign_string(&params);
        // let calculated_signature = self.calculate_signature(&sign_string)?;
        //
        // Ok(calculated_signature == signature)
        Ok(true)
    }

    fn secret_key(&self) -> &str {
        &self.config.app_secret
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// 签名工厂
pub struct SignerFactory;

impl SignerFactory {
    /// 创建默认签名器
    pub fn default_signer(app_key: &str, app_secret: &str) -> Box<dyn RequestSigner> {
        Box::new(DefaultSigner::new(app_key, app_secret))
    }

    /// 根据配置创建签名器
    pub fn from_config(config: SignerConfig) -> Box<dyn RequestSigner> {
        Box::new(DefaultSigner::with_config(config))
    }
}
