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
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::rsa::{Padding, Rsa};
use openssl::sign::{Signer, Verifier};
use openssl::symm;

use super::*;

/// OpenSSL后端实现
pub struct OpenSslCrypto;


impl Crypto for OpenSslCrypto {
    fn hash(&self, algorithm: HashAlgorithm, data: &[u8]) -> Vec<u8> {
        let digest = match algorithm {
            HashAlgorithm::Sha1 => MessageDigest::sha1(),
            HashAlgorithm::Sha256 => MessageDigest::sha256(),
            HashAlgorithm::Sha384 => MessageDigest::sha384(),
            HashAlgorithm::Sha512 => MessageDigest::sha512(),
            HashAlgorithm::Md5 => MessageDigest::md5(),
        };

        openssl::hash::hash(digest, data)
            .map(|d| d.to_vec())
            .unwrap_or_default()
    }

    fn hmac(&self, algorithm: HmacAlgorithm, key: &[u8], data: &[u8]) -> LabradorResult<Vec<u8>> {
        let pkey = PKey::hmac(key)
            .map_err(|e| CryptoError::Other(format!("创建HMAC密钥失败: {}", e)))?;

        let digest = match algorithm {
            HmacAlgorithm::Sha1 => MessageDigest::sha1(),
            HmacAlgorithm::Sha256 => MessageDigest::sha256(),
            HmacAlgorithm::Sha384 => MessageDigest::sha384(),
            HmacAlgorithm::Sha512 => MessageDigest::sha512(),
        };

        let mut signer = Signer::new(digest, &pkey)
            .map_err(|e| CryptoError::Other(format!("创建签名器失败: {}", e)))?;

        signer.update(data)
            .map_err(|e| CryptoError::Other(format!("更新数据失败: {}", e)))?;

        signer.sign_to_vec()
            .map_err(|e| CryptoError::Other(format!("签名失败: {}", e)).into())
    }

    fn aes_encrypt(&self, mode: AesMode, key: &[u8], iv: &[u8], plaintext: &[u8]) -> LabradorResult<Vec<u8>> {
        let cipher = match mode {
            AesMode::Cbc => match key.len() {
                16 => symm::Cipher::aes_128_cbc(),
                24 => symm::Cipher::aes_192_cbc(),
                32 => symm::Cipher::aes_256_cbc(),
                _ => return Err(CryptoError::Encryption("无效的密钥长度".to_string()).into()),
            },
            AesMode::Gcm => match key.len() {
                16 => symm::Cipher::aes_128_gcm(),
                32 => symm::Cipher::aes_256_gcm(),
                _ => return Err(CryptoError::Encryption("无效的密钥长度".to_string()).into()),
            },
            AesMode::Ctr => match key.len() {
                16 => symm::Cipher::aes_128_ctr(),
                24 => symm::Cipher::aes_192_ctr(),
                32 => symm::Cipher::aes_256_ctr(),
                _ => return Err(CryptoError::Encryption("无效的密钥长度".to_string()).into()),
            },
            AesMode::Ecb => match key.len() {
                16 => symm::Cipher::aes_128_ecb(),
                24 => symm::Cipher::aes_192_ecb(),
                32 => symm::Cipher::aes_256_ecb(),
                _ => return Err(CryptoError::Encryption("无效的密钥长度".to_string()).into()),
            },
            AesMode::Cfb => match key.len() {
                16 => symm::Cipher::aes_128_cfb128(),
                24 => symm::Cipher::aes_192_cfb128(),
                32 => symm::Cipher::aes_256_cfb128(),
                _ => return Err(CryptoError::Encryption("无效的密钥长度".to_string()).into()),
            },
        };

        let iv_opt = if mode == AesMode::Ecb { None } else { Some(iv) };

        symm::encrypt(cipher, key, iv_opt, plaintext)
            .map_err(|e| CryptoError::Encryption(format!("加密失败: {}", e)).into())
    }

    fn aes_decrypt(&self, mode: AesMode, key: &[u8], iv: &[u8], ciphertext: &[u8]) -> LabradorResult<Vec<u8>> {
        let cipher = match mode {
            AesMode::Cbc => match key.len() {
                16 => symm::Cipher::aes_128_cbc(),
                24 => symm::Cipher::aes_192_cbc(),
                32 => symm::Cipher::aes_256_cbc(),
                _ => return Err(CryptoError::Decryption("无效的密钥长度".to_string()).into()),
            },
            AesMode::Gcm => match key.len() {
                16 => symm::Cipher::aes_128_gcm(),
                32 => symm::Cipher::aes_256_gcm(),
                _ => return Err(CryptoError::Decryption("无效的密钥长度".to_string()).into()),
            },
            AesMode::Ctr => match key.len() {
                16 => symm::Cipher::aes_128_ctr(),
                24 => symm::Cipher::aes_192_ctr(),
                32 => symm::Cipher::aes_256_ctr(),
                _ => return Err(CryptoError::Decryption("无效的密钥长度".to_string()).into()),
            },
            AesMode::Ecb => match key.len() {
                16 => symm::Cipher::aes_128_ecb(),
                24 => symm::Cipher::aes_192_ecb(),
                32 => symm::Cipher::aes_256_ecb(),
                _ => return Err(CryptoError::Decryption("无效的密钥长度".to_string()).into()),
            },
            AesMode::Cfb => match key.len() {
                16 => symm::Cipher::aes_128_cfb128(),
                24 => symm::Cipher::aes_192_cfb128(),
                32 => symm::Cipher::aes_256_cfb128(),
                _ => return Err(CryptoError::Decryption("无效的密钥长度".to_string()).into()),
            },
        };

        let iv_opt = if mode == AesMode::Ecb { None } else { Some(iv) };

        symm::decrypt(cipher, key, iv_opt, ciphertext)
            .map_err(|e| CryptoError::Decryption(format!("解密失败: {}", e)).into())
    }

    fn rsa_sign(&self, private_key: &[u8], data: &[u8], hash_type: HashType) -> LabradorResult<Vec<u8>> {
        let digest = match hash_type {
            HashType::Sha1 => MessageDigest::sha1(),
            HashType::Sha256 => MessageDigest::sha256(),
        };

        // 尝试从PKCS#8 DER格式加载
        if let Ok(pkey) = PKey::private_key_from_pkcs8(private_key) {
            let mut signer = Signer::new(digest, &pkey)
                .map_err(|e| CryptoError::Signing(format!("创建签名器失败: {}", e)))?;

            signer.update(data)
                .map_err(|e| CryptoError::Signing(format!("更新数据失败: {}", e)))?;

            return signer.sign_to_vec()
                .map_err(|e| CryptoError::Signing(format!("签名失败: {}", e)).into());
        }

        // 尝试从PKCS#1 DER格式加载
        if let Ok(rsa) = Rsa::private_key_from_der(private_key) {
            let pkey = PKey::from_rsa(rsa)
                .map_err(|e| CryptoError::Signing(format!("创建PKey失败: {}", e)))?;

            let mut signer = Signer::new(digest, &pkey)
                .map_err(|e| CryptoError::Signing(format!("创建签名器失败: {}", e)))?;

            signer.set_rsa_padding(Padding::PKCS1)
                .map_err(|e| CryptoError::Signing(format!("设置填充失败: {}", e)))?;

            signer.update(data)
                .map_err(|e| CryptoError::Signing(format!("更新数据失败: {}", e)))?;

            return signer.sign_to_vec()
                .map_err(|e| CryptoError::Signing(format!("签名失败: {}", e)).into());
        }

        // 尝试从PEM格式加载
        if let Ok(rsa) = Rsa::private_key_from_pem(private_key) {
            let pkey = PKey::from_rsa(rsa)
                .map_err(|e| CryptoError::Signing(format!("创建PKey失败: {}", e)))?;

            let mut signer = Signer::new(digest, &pkey)
                .map_err(|e| CryptoError::Signing(format!("创建签名器失败: {}", e)))?;

            signer.set_rsa_padding(Padding::PKCS1)
                .map_err(|e| CryptoError::Signing(format!("设置填充失败: {}", e)))?;

            signer.update(data)
                .map_err(|e| CryptoError::Signing(format!("更新数据失败: {}", e)))?;

            return signer.sign_to_vec()
                .map_err(|e| CryptoError::Signing(format!("签名失败: {}", e)).into());
        }

        Err(CryptoError::Signing("无法加载私钥".to_string()).into())
    }

    fn rsa_verify(&self, public_key: &[u8], data: &[u8], signature: &[u8], hash_type: HashType) -> LabradorResult<bool> {
        let digest = match hash_type {
            HashType::Sha1 => MessageDigest::sha1(),
            HashType::Sha256 => MessageDigest::sha256(),
        };

        // 尝试从DER格式加载
        if let Ok(rsa) = Rsa::public_key_from_der(public_key) {
            let pkey = PKey::from_rsa(rsa)
                .map_err(|e| CryptoError::Verification(format!("创建PKey失败: {}", e)))?;

            let mut verifier = Verifier::new(digest, &pkey)
                .map_err(|e| CryptoError::Verification(format!("创建验证器失败: {}", e)))?;

            verifier.set_rsa_padding(Padding::PKCS1)
                .map_err(|e| CryptoError::Verification(format!("设置填充失败: {}", e)))?;

            verifier.update(data)
                .map_err(|e| CryptoError::Verification(format!("更新数据失败: {}", e)))?;

            return verifier.verify(signature)
                .map_err(|e| CryptoError::Verification(format!("验证失败: {}", e)).into());
        }

        // 尝试从PEM格式加载
        if let Ok(rsa) = Rsa::public_key_from_pem(public_key) {
            let pkey = PKey::from_rsa(rsa)
                .map_err(|e| CryptoError::Verification(format!("创建PKey失败: {}", e)))?;

            let mut verifier = Verifier::new(digest, &pkey)
                .map_err(|e| CryptoError::Verification(format!("创建验证器失败: {}", e)))?;

            verifier.set_rsa_padding(Padding::PKCS1)
                .map_err(|e| CryptoError::Verification(format!("设置填充失败: {}", e)))?;

            verifier.update(data)
                .map_err(|e| CryptoError::Verification(format!("更新数据失败: {}", e)))?;

            return verifier.verify(signature)
                .map_err(|e| CryptoError::Verification(format!("验证失败: {}", e)).into());
        }

        Ok(false)
    }

    fn random_bytes(&self, len: usize) -> LabradorResult<Vec<u8>> {
        let mut bytes = vec![0u8; len];
        openssl::rand::rand_bytes(&mut bytes)
            .map_err(|e| CryptoError::Other(format!("生成随机数失败: {}", e)))?;
        Ok(bytes)
    }

    fn pbkdf2(&self, password: &[u8], salt: &[u8], iterations: u32, key_len: usize) -> LabradorResult<Vec<u8>> {
        let mut derived_key = vec![0u8; key_len];
        openssl::pkcs5::pbkdf2_hmac(
            password,
            salt,
            iterations as usize,
            MessageDigest::sha256(),
            &mut derived_key,
        ).map_err(|e| CryptoError::Other(format!("PBKDF2失败: {}", e)))?;

        Ok(derived_key)
    }
}

// 公共接口函数
pub fn hash(algorithm: HashAlgorithm, data: &[u8]) -> Vec<u8> {
    OpenSslCrypto.hash(algorithm, data)
}

pub fn hmac(algorithm: HmacAlgorithm, key: &[u8], data: &[u8]) -> LabradorResult<Vec<u8>> {
    OpenSslCrypto.hmac(algorithm, key, data)
}

pub fn aes_encrypt(mode: AesMode, key: &[u8], iv: &[u8], plaintext: &[u8]) -> LabradorResult<Vec<u8>> {
    OpenSslCrypto.aes_encrypt(mode, key, iv, plaintext)
}

pub fn aes_decrypt(mode: AesMode, key: &[u8], iv: &[u8], ciphertext: &[u8]) -> LabradorResult<Vec<u8>> {
    OpenSslCrypto.aes_decrypt(mode, key, iv, ciphertext)
}

pub fn aes_gcm_encrypt(key: &[u8], nonce: &[u8], associated_data: &[u8], plaintext: &[u8]) -> LabradorResult<Vec<u8>> {
    let cipher = match key.len() {
        16 => symm::Cipher::aes_128_gcm(),
        32 => symm::Cipher::aes_256_gcm(),
        _ => return Err(CryptoError::Encryption("无效的密钥长度".to_string()).into()),
    };

    let mut tag = vec![0u8; 16];
    let encrypted = symm::encrypt_aead(cipher, key, Some(nonce), associated_data, plaintext, &mut tag)
        .map_err(|e| CryptoError::Encryption(format!("加密失败: {}", e)))?;

    // 合并加密数据和tag
    let mut result = encrypted;
    result.extend_from_slice(&tag);
    Ok(result)
}

pub fn aes_gcm_decrypt(key: &[u8], nonce: &[u8], associated_data: &[u8], ciphertext: &[u8], tag: &[u8]) -> LabradorResult<Vec<u8>> {
    let cipher = match key.len() {
        16 => symm::Cipher::aes_128_gcm(),
        32 => symm::Cipher::aes_256_gcm(),
        _ => return Err(CryptoError::Decryption("无效的密钥长度".to_string()).into()),
    };

    symm::decrypt_aead(cipher, key, Some(nonce), associated_data, ciphertext, tag)
        .map_err(|e| CryptoError::Decryption(format!("解密失败: {}", e)).into())
}

pub fn random_bytes(len: usize) -> LabradorResult<Vec<u8>> {
    OpenSslCrypto.random_bytes(len)
}

pub fn pbkdf2(password: &[u8], salt: &[u8], iterations: u32, key_len: usize) -> LabradorResult<Vec<u8>> {
    OpenSslCrypto.pbkdf2(password, salt, iterations, key_len)
}

pub fn rsa_sign(private_key: &[u8], data: &[u8], hash_type: HashType) -> LabradorResult<Vec<u8>> {
    OpenSslCrypto.rsa_sign(private_key, data, hash_type)
}

pub fn rsa_verify(public_key: &[u8], data: &[u8], signature: &[u8], hash_type: HashType) -> LabradorResult<bool> {
    OpenSslCrypto.rsa_verify(public_key, data, signature, hash_type)
}

pub fn rsa_encrypt(_public_key: &[u8], _plaintext: &[u8], _format: RsaKeyFormat) -> LabradorResult<Vec<u8>> {
    Err(CryptoError::AlgorithmUnsupported("RSA加密暂不支持".to_string()).into())
}

pub fn rsa_decrypt(_private_key: &[u8], _ciphertext: &[u8], _format: RsaKeyFormat) -> LabradorResult<Vec<u8>> {
    Err(CryptoError::AlgorithmUnsupported("RSA解密暂不支持".to_string()).into())
}
