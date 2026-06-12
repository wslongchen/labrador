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
use crate::errors::{LabraError, LabradorResult};
use crate::{AesEncryptor, AesMode, CryptoUtils, HashAlgorithm};

/// 微信消息加解密器
pub struct MessageCrypto {
    /// 应用ID
    app_id: String,
    /// 消息加解密密钥
    encoding_aes_key: String,
    /// 令牌
    token: String,
}

impl MessageCrypto {
    /// 创建新的消息加解密器
    pub fn new(app_id: &str, encoding_aes_key: &str, token: &str) -> LabradorResult<Self> {
        if encoding_aes_key.len() != 43 {
            return Err(LabraError::Crypto(
                "encoding_aes_key长度必须为43个字符".to_string(),
            ));
        }

        Ok(Self {
            app_id: app_id.to_string(),
            encoding_aes_key: encoding_aes_key.to_string(),
            token: token.to_string(),
        })
    }

    /// 加密消息
    pub fn encrypt_message(&self, plaintext: &str) -> LabradorResult<String> {
        use base64::{engine::general_purpose::STANDARD, Engine as _};

        // 生成随机字符串
        let nonce = crate::utils::string::random_string(16);

        // 构造待加密的字符串：16字节随机字符串 + 4字节消息长度 + 明文 + app_id
        let message_len = plaintext.len() as u32;
        let mut data = Vec::new();
        data.extend_from_slice(nonce.as_bytes());
        data.extend_from_slice(&message_len.to_be_bytes());
        data.extend_from_slice(plaintext.as_bytes());
        data.extend_from_slice(self.app_id.as_bytes());

        // Base64解码AES密钥
        let aes_key = STANDARD
            .decode(&self.encoding_aes_key)
            .map_err(|e| LabraError::Crypto(format!("解码AES密钥失败: {}", e)))?;

        // 使用AES-CBC加密
        let iv = &aes_key[..16]; // 使用AES密钥的前16字节作为IV
        let encryptor = AesEncryptor::new(AesMode::Cbc, &aes_key, iv)?;
        let encrypted = encryptor.encrypt(&data)?;

        // Base64编码加密后的数据
        let ciphertext = STANDARD.encode(encrypted);

        // 生成消息签名
        let timestamp = chrono::Utc::now().timestamp().to_string();
        let nonce = crate::utils::string::random_string(16);
        let signature = self.generate_signature(&timestamp, &nonce, &ciphertext)?;

        // 构造XML格式的加密消息
        let xml = format!(
            r#"<xml>
<Encrypt><![CDATA[{}]]></Encrypt>
<MsgSignature><![CDATA[{}]]></MsgSignature>
<TimeStamp>{}</TimeStamp>
<Nonce><![CDATA[{}]]></Nonce>
</xml>"#,
            ciphertext, signature, timestamp, nonce
        );

        Ok(xml)
    }

    /// 解密消息
    pub fn decrypt_message(
        &self,
        ciphertext: &str,
        timestamp: &str,
        nonce: &str,
        signature: &str,
    ) -> LabradorResult<String> {
        use base64::{engine::general_purpose::STANDARD, Engine as _};

        // 验证签名
        let expected_signature = self.generate_signature(timestamp, nonce, ciphertext)?;
        if expected_signature != signature {
            return Err(LabraError::Sign("签名验证失败".to_string()));
        }

        // Base64解码加密数据
        let encrypted_data = STANDARD
            .decode(ciphertext)
            .map_err(|e| LabraError::Crypto(format!("解码加密数据失败: {}", e)))?;

        // Base64解码AES密钥
        let aes_key = STANDARD
            .decode(&self.encoding_aes_key)
            .map_err(|e| LabraError::Crypto(format!("解码AES密钥失败: {}", e)))?;

        // 使用AES-CBC解密
        let iv = &aes_key[..16];
        let encryptor = AesEncryptor::new(AesMode::Cbc, &aes_key, iv)?;
        let decrypted = encryptor.decrypt(&encrypted_data)?;

        // 解析解密后的数据：16字节随机字符串 + 4字节消息长度 + 明文 + app_id
        if decrypted.len() < 20 {
            return Err(LabraError::Crypto("解密数据长度不足".to_string()));
        }

        let message_len =
            u32::from_be_bytes([decrypted[16], decrypted[17], decrypted[18], decrypted[19]])
                as usize;

        if decrypted.len() < 20 + message_len + self.app_id.len() {
            return Err(LabraError::Crypto("消息长度不匹配".to_string()));
        }

        let message_start = 20;
        let message_end = message_start + message_len;
        let message = &decrypted[message_start..message_end];

        let app_id_start = message_end;
        let app_id_end = app_id_start + self.app_id.len();
        let app_id_from_message = &decrypted[app_id_start..app_id_end];

        // 验证app_id
        if app_id_from_message != self.app_id.as_bytes() {
            return Err(LabraError::Crypto("app_id验证失败".to_string()));
        }

        String::from_utf8(message.to_vec()).map_err(LabraError::Utf8)
    }

    /// 生成消息签名
    fn generate_signature(
        &self,
        timestamp: &str,
        nonce: &str,
        ciphertext: &str,
    ) -> LabradorResult<String> {
        // 构造签名字符串：token + timestamp + nonce + ciphertext
        let mut items = [&self.token, timestamp, nonce, ciphertext];
        items.sort();

        let sign_string = items.join("");
        let signature = CryptoUtils::hash(HashAlgorithm::Sha1, sign_string.as_bytes());

        // 微信要求小写的十六进制字符串
        let signature_hex = hex::encode(signature);
        Ok(signature_hex)
    }

    /// 验证回调URL签名
    pub fn verify_url_signature(&self, timestamp: &str, nonce: &str, signature: &str) -> bool {
        match self.generate_signature(timestamp, nonce, "") {
            Ok(expected_signature) => expected_signature == signature,
            Err(_) => false,
        }
    }
}
