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

use crate::errors::{LabraError, LabradorResult};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use x509_parser::extensions::GeneralName;
use x509_parser::pem::parse_x509_pem;
use x509_parser::prelude::{SubjectPublicKeyInfo, X509Certificate, X509Name};

/// SSL/TLS证书
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Certificate {
    /// 证书序列号
    pub serial_number: String,
    /// 颁发时间（Unix时间戳）
    pub not_before: u64,
    /// 过期时间（Unix时间戳）
    pub not_after: u64,
    /// 公钥（PEM格式）
    pub public_key: Vec<u8>,
    /// 证书内容（PEM格式）
    pub content: Vec<u8>,
    /// 缓存解析后的X509证书（可选，使用OnceCell实现懒加载）
    #[serde(skip)]
    parsed: OnceCell<X509Certificate<'static>>,
}

impl Certificate {
    /// 从PEM格式创建证书
    pub fn from_pem(pem: &[u8]) -> LabradorResult<Self> {
        let _cert = reqwest::Certificate::from_pem(pem)
            .map_err(|e| LabraError::Certificate(e.to_string()))?;

        // 解析证书信息
        let (serial_number, not_before, not_after, public_key) =
            Self::parse_x509_info(pem)?;

        Ok(Self {
            serial_number,
            not_before,
            not_after,
            public_key,
            content: pem.to_vec(),
            parsed: OnceCell::new(),
        })
    }
    
    /// 从DER格式创建证书
    pub fn from_der(der: &[u8]) -> LabradorResult<Self> {
        let _cert = reqwest::Certificate::from_der(der)
            .map_err(|e| LabraError::Certificate(e.to_string()))?;

        // 解析证书信息
        let (serial_number, not_before, not_after, public_key) =
            Self::parse_x509_info(der)?;
        Ok(Self {
            serial_number,
            not_before,
            not_after,
            public_key,
            content: der.to_vec(),
            parsed: OnceCell::new(),
        })
    }

    /// 检查证书是否有效
    pub fn is_valid(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        now >= self.not_before && now <= self.not_after
    }

    /// 检查证书是否即将过期
    pub fn is_expiring_soon(&self, threshold_days: u64) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let remaining = self.not_after.saturating_sub(now);
        remaining <= threshold_days * 24 * 60 * 60
    }

    /// 转换为reqwest证书
    pub fn to_reqwest_certificate(&self) -> LabradorResult<reqwest::Certificate> {
        reqwest::Certificate::from_pem(&self.content)
            .map_err(|e| LabraError::Certificate(e.to_string()))
    }

    /// 解析X509证书信息
    fn parse_x509_info(pem: &[u8]) -> LabradorResult<(String, u64, u64, Vec<u8>)> {

        let (_, x509_pem) = parse_x509_pem(pem)
            .map_err(|e| LabraError::Certificate(e.to_string()))?;

        let x509 = x509_pem.parse_x509()
            .map_err(|e| LabraError::Certificate(e.to_string()))?;

        let serial_number = x509.serial.to_str_radix(16).to_uppercase();
        let not_before = x509.validity.not_before.timestamp() as u64;
        let not_after = x509.validity.not_after.timestamp() as u64;
        let public_key = x509.public_key().raw.to_vec();

        Ok((serial_number, not_before, not_after, public_key))
    }

    fn get_parsed(&self) -> LabradorResult<&X509Certificate<'_>> {
        self.parsed.get_or_try_init(|| {
            // 使用 unchecked 版本，需要我们自己确保证据数据的有效性
            let x509 = x509_parser::parse_x509_certificate(&self.content)
                .map_err(|e| LabraError::Certificate(e.to_string()))?
                .1;  // 获取 X509Certificate

            // 转换为 'static 生命周期
            // 因为 self.content 是 &self 的一部分，而 self 是 'static 的
            // 所以我们可以安全地将生命周期提升为 'static
            let x509_static: X509Certificate<'static> = unsafe {
                std::mem::transmute(x509)
            };

            Ok(x509_static)
        })
    }

    /// 获取证书主题（现在使用缓存）
    pub fn get_subject(&self) -> LabradorResult<&X509Name<'_>> {
        Ok(self.get_parsed()?.subject())
    }
    
    /// 获取证书颁发机构（现在使用缓存）
    pub fn get_issuer(&self) -> LabradorResult<&X509Name<'_>> {
        Ok(self.get_parsed()?.issuer())
    }
    
    /// 获取证书的公用密钥（现在使用缓存）
    pub fn public_key(&self) -> LabradorResult<&SubjectPublicKeyInfo<'_>> {
        Ok(self.get_parsed()?.public_key())
    }

    pub fn serial_number(&self) -> String {
        self.serial_number.to_string()
    }

    /// 获取证书的主题（如果有解析能力）
    pub fn subject(&self) -> LabradorResult<String> {
        let (_, x509_pem) = parse_x509_pem(&self.content)
            .map_err(|e| LabraError::Certificate(e.to_string()))?;

        let x509 = x509_pem.parse_x509()
            .map_err(|e| LabraError::Certificate(e.to_string()))?;

        Ok(x509.subject().to_string())
    }

    /// 获取证书的颁发者
    pub fn issuer(&self) -> LabradorResult<String> {
        let (_, x509_pem) = parse_x509_pem(&self.content)
            .map_err(|e| LabraError::Certificate(e.to_string()))?;

        let x509 = x509_pem.parse_x509()
            .map_err(|e| LabraError::Certificate(e.to_string()))?;

        Ok(x509.issuer().to_string())
    }

    /// 获取证书的SANs（Subject Alternative Names）
    pub fn subject_alternative_names(&self) -> LabradorResult<Vec<String>> {
        let (_, x509_pem) = parse_x509_pem(&self.content)
            .map_err(|e| LabraError::Certificate(e.to_string()))?;

        let x509 = x509_pem.parse_x509()
            .map_err(|e| LabraError::Certificate(e.to_string()))?;

        let mut sans = Vec::new();

        if let Some(subject_alt_name) = x509.subject_alternative_name()? {
            for general_name in subject_alt_name.value.general_names.iter() {
                match general_name {
                    GeneralName::DNSName(name) => sans.push(format!("DNS:{}", name)),
                    GeneralName::IPAddress(ip) => sans.push(format!("IP:{}", String::from_utf8_lossy(ip))),
                    GeneralName::URI(uri) => sans.push(format!("URI:{}", uri)),
                    GeneralName::RFC822Name(email) => sans.push(format!("email:{}", email)),
                    _ => {}
                }
            }
        }

        Ok(sans)
    }
}