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
use crate::errors::LabradorResult;
use base64::Engine;
use std::fmt;
use thiserror::Error;

// cfg_if! {
//     if #[cfg(feature = "openssl-crypto")] {
/// OpenSSL后端实现
#[allow(unused)]
mod openssl_iml;
// use openssl_iml as crypto;
pub use openssl_iml::OpenSslCrypto;
// } else {
/// 纯Rust后端实现（使用ring、aes等）
#[allow(unused)]
mod rust_impl;
use crate::crypto::crypto::extract_der_from_pem;
use rust_impl as crypto;
pub use rust_impl::RustCrypto;
// }
// }

/// 加密
pub trait Crypto: Send + Sync {
    /// 计算哈希值
    fn hash(&self, algorithm: HashAlgorithm, data: &[u8]) -> Vec<u8>;

    /// 计算HMAC
    fn hmac(&self, algorithm: HmacAlgorithm, key: &[u8], data: &[u8]) -> LabradorResult<Vec<u8>>;

    /// AES加密
    fn aes_encrypt(
        &self,
        mode: AesMode,
        key: &[u8],
        iv: &[u8],
        plaintext: &[u8],
    ) -> LabradorResult<Vec<u8>>;

    /// AES解密
    fn aes_decrypt(
        &self,
        mode: AesMode,
        key: &[u8],
        iv: &[u8],
        ciphertext: &[u8],
    ) -> LabradorResult<Vec<u8>>;

    /// RSA签名
    fn rsa_sign(
        &self,
        private_key: &[u8],
        data: &[u8],
        hash_type: HashType,
    ) -> LabradorResult<Vec<u8>>;

    /// RSA验证
    fn rsa_verify(
        &self,
        public_key: &[u8],
        data: &[u8],
        signature: &[u8],
        hash_type: HashType,
    ) -> LabradorResult<bool>;

    /// 生成随机字节
    fn random_bytes(&self, len: usize) -> LabradorResult<Vec<u8>>;

    /// PBKDF2密钥派生
    fn pbkdf2(
        &self,
        password: &[u8],
        salt: &[u8],
        iterations: u32,
        key_len: usize,
    ) -> LabradorResult<Vec<u8>>;
}

/// 哈希算法
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HashAlgorithm {
    /// SHA-1
    Sha1,
    /// SHA-256
    Sha256,
    /// SHA-384
    Sha384,
    /// SHA-512
    Sha512,
    /// MD5
    Md5,
}

#[allow(unused)]
pub enum HashType {
    Sha1,
    Sha256,
}

impl fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HashAlgorithm::Sha1 => write!(f, "SHA1"),
            HashAlgorithm::Sha256 => write!(f, "SHA256"),
            HashAlgorithm::Sha384 => write!(f, "SHA384"),
            HashAlgorithm::Sha512 => write!(f, "SHA512"),
            HashAlgorithm::Md5 => write!(f, "MD5"),
        }
    }
}

impl HashAlgorithm {
    /// 计算哈希值
    pub fn hash(&self, data: &[u8]) -> Vec<u8> {
        crypto::hash(*self, data)
    }

    /// 计算哈希值的十六进制字符串
    pub fn hash_hex(&self, data: &[u8]) -> String {
        hex::encode(self.hash(data))
    }

    /// 验证哈希值
    pub fn verify(&self, data: &[u8], hash: &[u8]) -> bool {
        self.hash(data) == hash
    }
}

/// HMAC算法
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HmacAlgorithm {
    /// HMAC-SHA1
    Sha1,
    /// HMAC-SHA256
    Sha256,
    /// HMAC-SHA384
    Sha384,
    /// HMAC-SHA512
    Sha512,
}

impl fmt::Display for HmacAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HmacAlgorithm::Sha1 => write!(f, "HMAC-SHA1"),
            HmacAlgorithm::Sha256 => write!(f, "HMAC-SHA256"),
            HmacAlgorithm::Sha384 => write!(f, "HMAC-SHA384"),
            HmacAlgorithm::Sha512 => write!(f, "HMAC-SHA512"),
        }
    }
}

impl HmacAlgorithm {
    /// 计算HMAC
    pub fn hmac(&self, key: &[u8], data: &[u8]) -> LabradorResult<Vec<u8>> {
        crypto::hmac(*self, key, data)
    }

    /// 计算HMAC的十六进制字符串
    pub fn hmac_hex(&self, key: &[u8], data: &[u8]) -> LabradorResult<String> {
        self.hmac(key, data).map(hex::encode)
    }

    /// 验证HMAC
    pub fn verify(&self, key: &[u8], data: &[u8], hmac: &[u8]) -> LabradorResult<bool> {
        let calculated = self.hmac(key, data)?;
        Ok(calculated == hmac)
    }
}

/// AES加密模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AesMode {
    /// CBC模式
    Cbc,
    /// GCM模式
    Gcm,
    /// CTR模式
    Ctr,
    /// ECB模式
    Ecb,
    /// CFB模式
    Cfb,
}

impl fmt::Display for AesMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AesMode::Cbc => write!(f, "AES-CBC"),
            AesMode::Gcm => write!(f, "AES-GCM"),
            AesMode::Ctr => write!(f, "AES-CTR"),
            AesMode::Ecb => write!(f, "AES-ECB"),
            AesMode::Cfb => write!(f, "AES-CFB"),
        }
    }
}

/// AES加密器
pub struct AesEncryptor {
    mode: AesMode,
    key: Vec<u8>,
    iv: Vec<u8>,
}

impl AesEncryptor {
    /// 创建新的AES加密器
    pub fn new(mode: AesMode, key: &[u8], iv: &[u8]) -> LabradorResult<Self> {
        // 验证密钥长度
        let key_len = key.len();
        match mode {
            AesMode::Cbc | AesMode::Ctr | AesMode::Cfb | AesMode::Ecb => {
                if key_len != 16 && key_len != 24 && key_len != 32 {
                    return Err(CryptoError::Key(format!(
                        "无效的AES密钥长度: {} (必须是16, 24或32字节)",
                        key_len
                    ))
                    .into());
                }
            }
            AesMode::Gcm => {
                if key_len != 16 && key_len != 32 {
                    return Err(CryptoError::Key(format!(
                        "无效的AES-GCM密钥长度: {} (必须是16或32字节)",
                        key_len
                    ))
                    .into());
                }
            }
        }

        // 验证IV长度
        match mode {
            AesMode::Gcm => {
                // GCM 推荐使用 12 字节 (96 位) IV
                if iv.is_empty() {
                    return Err(CryptoError::Key("GCM模式需要IV".to_string()).into());
                }
                // GCM 可以接受任意长度的 IV，但 12 字节是最优的
                if iv.len() != 12 {
                    // 只是警告，不是错误，因为 GCM 支持任意长度 IV
                    println!("警告: GCM模式推荐使用12字节IV，当前长度为: {}", iv.len());
                }
            }
            AesMode::Ecb => {
                // ECB 不需要 IV
                if !iv.is_empty() {
                    println!("警告: ECB模式忽略IV");
                }
            }
            _ => {
                // CBC/CFB/CTR 等需要 16 字节 IV
                if iv.len() != 16 {
                    return Err(CryptoError::Key(format!(
                        "无效的IV长度: {} (必须是16字节)",
                        iv.len()
                    ))
                    .into());
                }
            }
        }

        Ok(Self {
            mode,
            key: key.to_vec(),
            iv: iv.to_vec(),
        })
    }

    /// 加密数据
    pub fn encrypt(&self, plaintext: &[u8]) -> LabradorResult<Vec<u8>> {
        crypto::aes_encrypt(self.mode, &self.key, &self.iv, plaintext)
    }

    /// 解密数据
    pub fn decrypt(&self, ciphertext: &[u8]) -> LabradorResult<Vec<u8>> {
        crypto::aes_decrypt(self.mode, &self.key, &self.iv, ciphertext)
    }

    pub fn gcm_decrypt(
        &self,
        nonce: &[u8],
        associated_data: &[u8],
        ciphertext: &[u8],
        tag: &[u8],
    ) -> LabradorResult<Vec<u8>> {
        crypto::aes_gcm_decrypt(&self.key, nonce, associated_data, ciphertext, tag)
    }

    pub fn gcm_encrypt(
        &self,
        nonce: &[u8],
        associated_data: &[u8],
        plaintext: &[u8],
    ) -> LabradorResult<Vec<u8>> {
        crypto::aes_gcm_encrypt(&self.key, nonce, associated_data, plaintext)
    }
}

/// RSA密钥格式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RsaKeyFormat {
    /// PKCS#1格式
    Pkcs1,
    /// PKCS#8格式
    Pkcs8,
    /// PEM格式
    Pem,
}

/// RSA加密器
#[allow(unused)]
pub struct RsaEncryptor {
    public_key: Vec<u8>,
    private_key: Option<Vec<u8>>,
    key_format: RsaKeyFormat,
}

impl RsaEncryptor {
    /// 使用公钥创建RSA加密器
    pub fn with_public_key(public_key: &[u8], format: RsaKeyFormat) -> Self {
        let public_key = if format == RsaKeyFormat::Pem {
            extract_der_from_pem(public_key).unwrap_or_default()
        } else {
            public_key.to_vec()
        };
        Self {
            public_key: public_key.to_vec(),
            private_key: None,
            key_format: format,
        }
    }

    /// 使用公钥和私钥创建RSA加密器
    pub fn with_keys(public_key: &[u8], private_key: &[u8], format: RsaKeyFormat) -> Self {
        let (public_key, private_key) = if format == RsaKeyFormat::Pem {
            let pub_pem = extract_der_from_pem(private_key).unwrap_or_default();
            let pri_pem = extract_der_from_pem(public_key).unwrap_or_default();
            (pub_pem, pri_pem)
        } else {
            (public_key.to_vec(), private_key.to_vec())
        };
        Self {
            public_key,
            private_key: Some(private_key),
            key_format: format,
        }
    }

    /// 使用私钥创建RSA加密器
    pub fn with_private_key(private_key: &[u8], format: RsaKeyFormat) -> Self {
        let private_key = if format == RsaKeyFormat::Pem {
            extract_der_from_pem(private_key).expect("加载私钥异常")
        } else {
            private_key.to_vec()
        };

        Self {
            public_key: Vec::new(), // 占位，实际使用时可能需要提取公钥
            private_key: Some(private_key),
            key_format: format,
        }
    }

    /// 加密数据
    pub fn encrypt(&self, plaintext: &[u8]) -> LabradorResult<Vec<u8>> {
        crypto::rsa_encrypt(&self.public_key, plaintext)
    }

    /// 解密数据
    pub fn decrypt(&self, ciphertext: &[u8]) -> LabradorResult<Vec<u8>> {
        match &self.private_key {
            Some(private_key) => crypto::rsa_decrypt(private_key, ciphertext),
            None => Err(CryptoError::Decryption("没有私钥，无法解密".to_string()).into()),
        }
    }

    /// 签名数据
    pub fn sign(&self, data: &[u8], hash_type: HashType) -> LabradorResult<Vec<u8>> {
        match &self.private_key {
            Some(private_key) => crypto::rsa_sign(private_key, data, hash_type),
            None => Err(CryptoError::Signing("没有私钥，无法签名".to_string()).into()),
        }
    }

    /// 验证签名
    pub fn verify(
        &self,
        data: &[u8],
        signature: &[u8],
        hash_type: HashType,
    ) -> LabradorResult<bool> {
        crypto::rsa_verify(&self.public_key, data, signature, hash_type)
    }
}

/// 统一的加密工具
pub struct CryptoUtils;

impl CryptoUtils {
    /// 生成随机字节
    pub fn random_bytes(len: usize) -> LabradorResult<Vec<u8>> {
        crypto::random_bytes(len)
    }

    /// 生成随机字符串
    pub fn random_string(len: usize) -> LabradorResult<String> {
        let bytes = Self::random_bytes(len)?;
        Ok(base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&bytes))
    }

    /// 生成UUID
    pub fn uuid() -> String {
        uuid::Uuid::new_v4().to_string()
    }

    /// PBKDF2密钥派生
    pub fn pbkdf2(
        password: &[u8],
        salt: &[u8],
        iterations: u32,
        key_len: usize,
    ) -> LabradorResult<Vec<u8>> {
        crypto::pbkdf2(password, salt, iterations, key_len)
    }

    /// 计算哈希值
    pub fn hash(algorithm: HashAlgorithm, data: &[u8]) -> Vec<u8> {
        algorithm.hash(data)
    }

    /// 计算哈希值的十六进制字符串
    pub fn hash_hex(algorithm: HashAlgorithm, data: &[u8]) -> String {
        algorithm.hash_hex(data)
    }

    /// 计算HMAC
    pub fn hmac(algorithm: HmacAlgorithm, key: &[u8], data: &[u8]) -> LabradorResult<Vec<u8>> {
        algorithm.hmac(key, data)
    }

    /// 计算HMAC的十六进制字符串
    pub fn hmac_hex(algorithm: HmacAlgorithm, key: &[u8], data: &[u8]) -> LabradorResult<String> {
        algorithm.hmac_hex(key, data)
    }

    /// 验证HMAC
    pub fn verify_hmac(
        algorithm: HmacAlgorithm,
        key: &[u8],
        data: &[u8],
        signature: &[u8],
    ) -> LabradorResult<bool> {
        algorithm.verify(key, data, signature)
    }

    /// 快捷方法：计算SHA256哈希
    pub fn sha256(data: &[u8]) -> Vec<u8> {
        HashAlgorithm::Sha256.hash(data)
    }
    /// 快捷方法：计算SHA1哈希
    pub fn sha1(data: &[u8]) -> Vec<u8> {
        HashAlgorithm::Sha1.hash(data)
    }
    pub fn sha1_hex(data: &[u8]) -> String {
        HashAlgorithm::Sha1.hash_hex(data)
    }

    /// 快捷方法：计算SHA256哈希的十六进制字符串
    pub fn sha256_hex(data: &[u8]) -> String {
        HashAlgorithm::Sha256.hash_hex(data)
    }

    /// 快捷方法：计算SHA512哈希
    pub fn sha512(data: &[u8]) -> Vec<u8> {
        HashAlgorithm::Sha512.hash(data)
    }

    /// 快捷方法：计算SHA512哈希的十六进制字符串
    pub fn sha512_hex(data: &[u8]) -> String {
        HashAlgorithm::Sha512.hash_hex(data)
    }

    /// 快捷方法：计算MD5哈希
    pub fn md5(data: &[u8]) -> String {
        HashAlgorithm::Md5.hash_hex(data)
    }

    /// 快捷方法：计算HMAC-SHA256
    pub fn hmac_sha256(key: &[u8], data: &[u8]) -> LabradorResult<Vec<u8>> {
        HmacAlgorithm::Sha256.hmac(key, data)
    }

    /// 快捷方法：计算HMAC-SHA256的十六进制字符串
    pub fn hmac_sha256_hex(key: &[u8], data: &[u8]) -> LabradorResult<String> {
        HmacAlgorithm::Sha256.hmac_hex(key, data)
    }

    /// 快捷方法：验证HMAC-SHA256
    pub fn verify_hmac_sha256(key: &[u8], data: &[u8], signature: &[u8]) -> LabradorResult<bool> {
        HmacAlgorithm::Sha256.verify(key, data, signature)
    }

    /// Base64编码
    pub fn base64_encode(data: &[u8]) -> String {
        base64::engine::general_purpose::STANDARD.encode(data)
    }

    /// Base64解码
    pub fn base64_decode(data: &str) -> LabradorResult<Vec<u8>> {
        base64::engine::general_purpose::STANDARD
            .decode(data)
            .map_err(|e| CryptoError::Other(format!("Base64解码失败: {}", e)).into())
    }

    /// Hex编码
    pub fn hex_encode(data: &[u8]) -> String {
        hex::encode(data)
    }

    /// Hex解码
    pub fn hex_decode(data: &str) -> LabradorResult<Vec<u8>> {
        hex::decode(data).map_err(|e| CryptoError::Other(format!("Hex解码失败: {}", e)).into())
    }
}

/// 向后兼容的PrpCrypto结构
#[derive(Debug, Clone)]
pub struct PrpCrypto {
    key: Vec<u8>,
}

impl PrpCrypto {
    pub fn new(key: Vec<u8>) -> PrpCrypto {
        PrpCrypto { key }
    }

    /// # 加密消息(aes_128_cbc)
    pub fn aes_128_cbc_encrypt_data(
        &self,
        plaintext: &str,
        iv_data: Option<Vec<u8>>,
    ) -> LabradorResult<Vec<u8>> {
        let iv = iv_data.unwrap_or_else(|| self.key[..16].to_vec());
        let encryptor = AesEncryptor::new(AesMode::Cbc, &self.key, &iv)?;
        encryptor.encrypt(plaintext.as_bytes())
    }

    /// # 解密消息(aes_128_cbc)
    pub fn aes_128_cbc_decrypt_data(
        &self,
        ciphertext: Vec<u8>,
        iv_data: Option<Vec<u8>>,
    ) -> LabradorResult<Vec<u8>> {
        let iv = iv_data.unwrap_or_else(|| self.key[..16].to_vec());
        let encryptor = AesEncryptor::new(AesMode::Cbc, &self.key, &iv)?;
        encryptor.decrypt(&ciphertext)
    }

    /// RSA签名
    pub fn rsa_sha256_sign(content: &str, private_key: &str) -> LabradorResult<String> {
        let key_data = CryptoUtils::base64_decode(private_key)?;
        let encryptor = RsaEncryptor::with_private_key(&key_data, RsaKeyFormat::Pkcs1);
        let signature = encryptor.sign(content.as_bytes(), HashType::Sha256)?;
        Ok(CryptoUtils::base64_encode(&signature))
    }

    pub fn rsa_sha256_sign_with_pem(content: &str, private_key: &str) -> LabradorResult<String> {
        let encryptor = RsaEncryptor::with_private_key(private_key.as_bytes(), RsaKeyFormat::Pem);
        let signature = encryptor.sign(content.as_bytes(), HashType::Sha256)?;
        Ok(CryptoUtils::base64_encode(&signature))
    }

    pub fn rsa_sha256_sign_pkcs1(&self, content: &str) -> LabradorResult<String> {
        let encryptor = RsaEncryptor::with_private_key(&self.key, RsaKeyFormat::Pkcs1);
        let signature = encryptor.sign(content.as_bytes(), HashType::Sha256)?;
        Ok(CryptoUtils::base64_encode(&signature))
    }

    pub fn rsa_sha256_sign_pkcs8(&self, content: &str) -> LabradorResult<String> {
        let encryptor = RsaEncryptor::with_private_key(&self.key, RsaKeyFormat::Pkcs8);
        let signature = encryptor.sign(content.as_bytes(), HashType::Sha256)?;
        Ok(CryptoUtils::base64_encode(&signature))
    }

    /// RSA签名验证
    pub fn rsa_sha256_verify(public_key: &str, content: &str, sign: &str) -> LabradorResult<bool> {
        let key_data = CryptoUtils::base64_decode(public_key)?;
        let signature = CryptoUtils::base64_decode(sign)?;
        let encryptor = RsaEncryptor::with_public_key(&key_data, RsaKeyFormat::Pkcs1);
        encryptor.verify(content.as_bytes(), &signature, HashType::Sha256)
    }

    pub fn rsa_sha256_verify_with_pem(
        public_key: &str,
        content: &str,
        sign: &str,
    ) -> LabradorResult<bool> {
        let signature = CryptoUtils::base64_decode(sign)?;
        let encryptor = RsaEncryptor::with_public_key(public_key.as_bytes(), RsaKeyFormat::Pem);
        encryptor.verify(content.as_bytes(), &signature, HashType::Sha256)
    }

    pub fn hmac_sha256_sign(&self, message: &str) -> LabradorResult<String> {
        HmacAlgorithm::Sha256.hmac_hex(&self.key, message.as_bytes())
    }

    pub fn hmac_sha256_verify(&self, content: &str, sign: &str) -> LabradorResult<bool> {
        let signature = CryptoUtils::hex_decode(sign)?;
        HmacAlgorithm::Sha256.verify(&self.key, content.as_bytes(), &signature)
    }

    pub fn hmac_sha1_sign(&self, message: &str) -> LabradorResult<Vec<u8>> {
        HmacAlgorithm::Sha1.hmac(&self.key, message.as_bytes())
    }

    pub fn hmac_sha1(&self, data: &[u8]) -> LabradorResult<Vec<u8>> {
        HmacAlgorithm::Sha1.hmac(&self.key, data)
    }

    /// # 加密(aes_256_gcm)
    pub fn aes_256_gcm_encrypt(
        &self,
        associated_data: &[u8],
        nonce: &[u8],
        plain_text: &[u8],
    ) -> LabradorResult<Vec<u8>> {
        // GCM模式在backend中统一处理
        crypto::aes_gcm_encrypt(&self.key, nonce, associated_data, plain_text)
    }

    /// # 解密(aes_256_gcm)
    pub fn aes_256_gcm_decrypt(
        &self,
        associated_data: &[u8],
        nonce: &[u8],
        ciphertext: &[u8],
        tag: &[u8],
    ) -> LabradorResult<Vec<u8>> {
        crypto::aes_gcm_decrypt(&self.key, nonce, associated_data, ciphertext, tag)
    }

    /// # 加密(aes_256_ecb)
    pub fn aes_256_ecb_encrypt(&self, data: &[u8]) -> LabradorResult<Vec<u8>> {
        // ECB模式不需要IV
        let iv = vec![0u8; 16]; // 占位
        let encryptor = AesEncryptor::new(AesMode::Ecb, &self.key, &iv)?;
        encryptor.encrypt(data)
    }

    /// # 解密(aes_256_ecb)
    pub fn aes_256_ecb_decrypt(&self, data: &[u8]) -> LabradorResult<String> {
        let iv = vec![0u8; 16]; // 占位
        let encryptor = AesEncryptor::new(AesMode::Ecb, &self.key, &iv)?;
        let decrypted = encryptor.decrypt(data)?;
        String::from_utf8(decrypted)
            .map_err(|e| CryptoError::Decryption(format!("UTF-8解码失败: {}", e)).into())
    }

    /// # 加密(aes_128_cfb)
    pub fn aes_128_cfb_encrypt(&self, data: &[u8]) -> LabradorResult<Vec<u8>> {
        if self.key.len() != 16 {
            return Err(CryptoError::Key("AES-128 需要 16 字节的 key".to_string()).into());
        }
        let iv = &self.key; // CFB模式通常使用key作为IV
        let encryptor = AesEncryptor::new(AesMode::Cfb, &self.key, iv)?;
        encryptor.encrypt(data)
    }

    /// # 解密(aes_128_cfb)
    pub fn aes_128_cfb_decrypt(&self, data: &[u8]) -> LabradorResult<String> {
        if self.key.len() != 16 {
            return Err(CryptoError::Key("AES-128 需要 16 字节的 key".to_string()).into());
        }
        let iv = &self.key; // CFB模式通常使用key作为IV
        let encryptor = AesEncryptor::new(AesMode::Cfb, &self.key, iv)?;
        let decrypted = encryptor.decrypt(data)?;
        String::from_utf8(decrypted)
            .map_err(|e| CryptoError::Decryption(format!("UTF-8解码失败: {}", e)).into())
    }
}

// 定义错误类型
#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("加密错误: {0}")]
    Encryption(String),

    #[error("解密错误: {0}")]
    Decryption(String),

    #[error("签名错误: {0}")]
    Signing(String),

    #[error("验证错误: {0}")]
    Verification(String),

    #[error("密钥错误: {0}")]
    Key(String),

    #[error("算法不支持: {0}")]
    AlgorithmUnsupported(String),

    #[error("其他错误: {0}")]
    Other(String),
}

impl From<CryptoError> for crate::errors::LabraError {
    fn from(err: CryptoError) -> Self {
        crate::errors::LabraError::Crypto(err.to_string())
    }
}
