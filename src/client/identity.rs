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
use crate::client::certificate::Certificate;
use crate::errors::{LabraError, LabradorResult};
use openssl::pkcs12::Pkcs12;
use reqwest::Identity as ReqwestIdentity;
use serde::{Deserialize, Serialize};
use std::fmt;

/// SSL/TLS客户端身份标识（支持序列化）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    /// 原始数据格式
    raw_data: RawIdentityData,

    /// 证书信息（用于业务逻辑）
    pub certificates: Vec<Certificate>,

    /// 私钥类型
    pub key_type: PrivateKeyType,

    /// 是否包含完整的证书链
    pub has_chain: bool,

    /// 私钥（PEM格式）- 从P12解析出来的，用于业务逻辑
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<Vec<u8>>,
}

/// 原始身份数据格式
#[derive(Debug, Clone, Serialize, Deserialize)]
enum RawIdentityData {
    /// PKCS#12格式 (二进制)
    P12 {
        data: Vec<u8>,
        #[serde(skip_serializing_if = "Option::is_none")]
        password_hint: Option<String>, // 仅用于提示，不保存真实密码
    },
    /// PEM格式 (文本)
    Pem { data: Vec<u8> },
}

/// 私钥类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PrivateKeyType {
    RSA,
    EC,
    Other(String),
}

impl Identity {
    /// 从PKCS#12格式创建Identity
    pub fn from_pkcs12(der: &[u8], password: &str) -> LabradorResult<Self> {
        // 验证P12是否有效
        let _reqwest_identity = ReqwestIdentity::from_pkcs12_der(der, password)
            .map_err(|e| LabraError::Identity(format!("Invalid PKCS12: {}", e)))?;

        // 解析证书信息和私钥
        let (certificates, private_key, key_type, has_chain) =
            Self::parse_pkcs12_info(der, password)?;

        Ok(Self {
            raw_data: RawIdentityData::P12 {
                data: der.to_vec(),
                password_hint: Some(password.to_string()),
            },
            certificates,
            private_key: Some(private_key),
            key_type,
            has_chain,
        })
    }

    /// 从PKCS#12文件创建Identity
    pub fn from_pkcs12_file(path: &str, password: &str) -> LabradorResult<Self> {
        let data = std::fs::read(path)
            .map_err(|e| LabraError::Identity(format!("Failed to read file: {}", e)))?;
        Self::from_pkcs12(&data, password)
    }

    /// 从PEM格式创建Identity
    pub fn from_pem(pem_data: &[u8]) -> LabradorResult<Self> {
        // 验证PEM是否有效
        let _reqwest_identity = ReqwestIdentity::from_pkcs8_pem(pem_data, pem_data)
            .map_err(|e| LabraError::Identity(format!("Invalid PEM: {}", e)))?;

        // 解析证书信息和私钥
        let (certificates, private_key, key_type, has_chain) = Self::parse_pem_info(pem_data)?;

        Ok(Self {
            raw_data: RawIdentityData::Pem {
                data: pem_data.to_vec(),
            },
            certificates,
            private_key: Some(private_key),
            key_type,
            has_chain,
        })
    }

    /// 从PEM文件创建Identity
    pub fn from_pem_file(path: &str) -> LabradorResult<Self> {
        let data = std::fs::read(path)
            .map_err(|e| LabraError::Identity(format!("Failed to read file: {}", e)))?;
        Self::from_pem(&data)
    }

    /// 转换为reqwest Identity（需要提供密码）
    pub fn to_reqwest_identity(&self) -> LabradorResult<ReqwestIdentity> {
        match &self.raw_data {
            RawIdentityData::P12 {
                data,
                password_hint,
            } => {
                // 尝试提供的密码
                if let Some(pwd) = password_hint {
                    return ReqwestIdentity::from_pkcs12_der(data, pwd).map_err(|e| {
                        LabraError::Identity(format!("Failed to create from P12: {}", e))
                    });
                }

                Err(LabraError::Identity(
                    "Password required for P12".to_string(),
                ))
            }
            RawIdentityData::Pem { data } => ReqwestIdentity::from_pkcs8_pem(data, data)
                .map_err(|e| LabraError::Identity(format!("Failed to create from PEM: {}", e))),
        }
    }

    /// 解析PKCS#12信息
    fn parse_pkcs12_info(
        der: &[u8],
        password: &str,
    ) -> LabradorResult<(Vec<Certificate>, Vec<u8>, PrivateKeyType, bool)> {
        let pfx = Pkcs12::from_der(der)
            .map_err(|e| LabraError::Identity(format!("Failed to parse PKCS12: {}", e)))?;

        let parsed = pfx.parse2(password).map_err(|e| {
            LabraError::Identity(format!("Failed to parse PKCS12 with password: {}", e))
        })?;

        let mut certificates = Vec::new();

        // 客户端证书
        if let Some(cert) = parsed.cert {
            certificates.push(Certificate::from_pem(
                &cert
                    .to_pem()
                    .map_err(|e| LabraError::Identity(e.to_string()))?,
            )?);
        }

        // CA证书链
        if let Some(ca_certs) = parsed.ca {
            for ca_cert in ca_certs {
                certificates.push(Certificate::from_pem(
                    &ca_cert
                        .to_pem()
                        .map_err(|e| LabraError::Identity(e.to_string()))?,
                )?);
            }
        }

        // 获取私钥
        let private_key = if let Some(pkey) = parsed.pkey {
            pkey.private_key_to_pem_pkcs8().map_err(|e| {
                LabraError::Identity(format!("Failed to convert private key: {}", e))
            })?
        } else {
            return Err(LabraError::Identity("No private key found".to_string()));
        };

        // 检测私钥类型
        let key_type = Self::detect_private_key_type(&private_key)?;

        let len = certificates.len();
        Ok((certificates, private_key, key_type, len > 1))
    }

    /// 解析PEM信息（不使用外部crate）
    fn parse_pem_info(
        pem_data: &[u8],
    ) -> LabradorResult<(Vec<Certificate>, Vec<u8>, PrivateKeyType, bool)> {
        let pem_str = std::str::from_utf8(pem_data)
            .map_err(|e| LabraError::Identity(format!("Invalid UTF-8 in PEM: {}", e)))?;

        let mut certificates = Vec::new();
        let mut private_key = None;
        let mut key_type = PrivateKeyType::Other("unknown".to_string());
        let mut current_pem = String::new();
        let mut in_pem = false;

        for line in pem_str.lines() {
            if line.starts_with("-----BEGIN ") {
                in_pem = true;
                current_pem.clear();
                current_pem.push_str(line);
                current_pem.push('\n');
            } else if line.starts_with("-----END ") && in_pem {
                current_pem.push_str(line);
                in_pem = false;

                let pem_bytes = current_pem.as_bytes();

                if current_pem.contains("CERTIFICATE") {
                    let cert = Certificate::from_pem(pem_bytes)?;
                    certificates.push(cert);
                } else if current_pem.contains("PRIVATE KEY") {
                    private_key = Some(pem_bytes.to_vec());
                    key_type = Self::detect_private_key_type(pem_bytes)?;
                }
            } else if in_pem {
                current_pem.push_str(line);
                current_pem.push('\n');
            }
        }

        let private_key = private_key
            .ok_or_else(|| LabraError::Identity("No private key found in PEM".to_string()))?;
        let len = certificates.len();

        Ok((certificates, private_key, key_type, len > 1))
    }

    /// 检测私钥类型
    fn detect_private_key_type(key_data: &[u8]) -> LabradorResult<PrivateKeyType> {
        let key_str = std::str::from_utf8(key_data).unwrap_or("");

        if key_str.contains("BEGIN RSA PRIVATE KEY") {
            Ok(PrivateKeyType::RSA)
        } else if key_str.contains("BEGIN EC PRIVATE KEY") {
            Ok(PrivateKeyType::EC)
        } else if key_str.contains("BEGIN PRIVATE KEY") {
            if key_str.contains("RSA") {
                Ok(PrivateKeyType::RSA)
            } else if key_str.contains("EC") {
                Ok(PrivateKeyType::EC)
            } else {
                Ok(PrivateKeyType::Other("PKCS8".to_string()))
            }
        } else {
            Ok(PrivateKeyType::Other("unknown".to_string()))
        }
    }

    /// 获取客户端证书
    pub fn client_certificate(&self) -> Option<&Certificate> {
        self.certificates.first()
    }

    /// 获取证书链
    pub fn certificate_chain(&self) -> &[Certificate] {
        if self.certificates.len() > 1 {
            &self.certificates[1..]
        } else {
            &[]
        }
    }

    /// 检查证书是否即将过期
    pub fn is_expiring_soon(&self, threshold_days: u64) -> bool {
        self.client_certificate()
            .map(|cert| cert.is_expiring_soon(threshold_days))
            .unwrap_or(false)
    }

    /// 获取摘要信息
    pub fn summary(&self) -> String {
        let cert_info = self
            .client_certificate()
            .map(|cert| {
                format!(
                    "serial: {}, expires: {}",
                    cert.serial_number, cert.not_after
                )
            })
            .unwrap_or_else(|| "no certificate".to_string());

        let format = match &self.raw_data {
            RawIdentityData::P12 { .. } => "P12",
            RawIdentityData::Pem { .. } => "PEM",
        };

        format!(
            "Identity(format: {}, key_type: {:?}, certificates: {}, has_chain: {}, private_key: {}, {})",
            format,
            self.key_type,
            self.certificates.len(),
            self.has_chain,
            self.private_key.is_some(),
            cert_info
        )
    }

    /// 获取数据大小
    pub fn data_size(&self) -> usize {
        match &self.raw_data {
            RawIdentityData::P12 { data, .. } => data.len(),
            RawIdentityData::Pem { data } => data.len(),
        }
    }

    /// 是否需要密码
    pub fn requires_password(&self) -> bool {
        matches!(self.raw_data, RawIdentityData::P12 { .. })
    }

    /// 获取私钥（PEM格式）
    pub fn private_key(&self) -> Option<&[u8]> {
        self.private_key.as_deref()
    }

    /// 获取私钥（PEM格式）
    pub fn private_key_pem(&self) -> Option<String> {
        self.private_key
            .as_deref()
            .map(|key| String::from_utf8_lossy(key).to_string())
    }

    /// 获取证书序列号
    pub fn serial_number(&self) -> Option<String> {
        self.client_certificate()
            .map(|cert| cert.serial_number.to_string())
    }

    /// 导出为PKCS#12（如果原始格式是P12）
    pub fn to_pkcs12(&self) -> Option<&[u8]> {
        match &self.raw_data {
            RawIdentityData::P12 { data, .. } => Some(data),
            _ => None,
        }
    }

    /// 导出为PEM（如果原始格式是PEM）
    pub fn to_pem(&self) -> Option<&[u8]> {
        match &self.raw_data {
            RawIdentityData::Pem { data } => Some(data),
            _ => None,
        }
    }
}

/// 为Identity实现Display
impl fmt::Display for Identity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.summary())
    }
}
