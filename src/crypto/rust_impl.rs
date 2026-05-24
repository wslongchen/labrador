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
use crate::{AesMode, Crypto, CryptoError, HashAlgorithm, HashType, HmacAlgorithm};
use aes::cipher::generic_array::GenericArray;
use ring::rand::{SecureRandom, SystemRandom};
use std::num::NonZeroU32;
use typenum::U16;

/// 纯Rust后端实现
pub struct RustCrypto;

impl Crypto for RustCrypto {
    fn hash(&self, algorithm: HashAlgorithm, data: &[u8]) -> Vec<u8> {
        use digest::Digest;
        match algorithm {
            HashAlgorithm::Sha1 => {
                let mut hasher = sha1::Sha1::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::Sha256 => {
                let mut ctx = sha2::Sha256::new();
                ctx.update(data);
                ctx.finalize().to_vec()
            }
            HashAlgorithm::Sha384 => {
                let mut ctx = sha2::Sha384::new();
                ctx.update(data);
                ctx.finalize().to_vec()
            }
            HashAlgorithm::Sha512 => {
                let mut ctx = sha2::Sha512::new();
                ctx.update(data);
                ctx.finalize().to_vec()
            }
            HashAlgorithm::Md5 => {
                let ctx = md5::compute(data);
                ctx.to_vec()
            }
        }
    }


    fn hmac(&self, algorithm: HmacAlgorithm, key: &[u8], data: &[u8]) -> LabradorResult<Vec<u8>> {
        use hmac::Mac;
        match algorithm {
            HmacAlgorithm::Sha1 => {
                let mut mac = hmac::Hmac::<sha1::Sha1>::new_from_slice(key)
                    .map_err(|e| CryptoError::Other(format!("HMAC密钥无效: {}", e)))?;
                mac.update(data);
                Ok(mac.finalize().into_bytes().to_vec())
            }
            HmacAlgorithm::Sha256 => {
                let mut mac = hmac::Hmac::<sha2::Sha256>::new_from_slice(key)
                    .map_err(|e| CryptoError::Other(format!("HMAC密钥无效: {}", e)))?;
                mac.update(data);
                Ok(mac.finalize().into_bytes().to_vec())
            }
            HmacAlgorithm::Sha384 => {
                let mut mac = hmac::Hmac::<sha2::Sha384>::new_from_slice(key)
                    .map_err(|e| CryptoError::Other(format!("HMAC密钥无效: {}", e)))?;
                mac.update(data);
                Ok(mac.finalize().into_bytes().to_vec())
            }
            HmacAlgorithm::Sha512 => {
                let mut mac = hmac::Hmac::<sha2::Sha512>::new_from_slice(key)
                    .map_err(|e| CryptoError::Other(format!("HMAC密钥无效: {}", e)))?;
                mac.update(data);
                Ok(mac.finalize().into_bytes().to_vec())
            }
        }
    }

    fn aes_encrypt(&self, mode: AesMode, key: &[u8], iv: &[u8], plaintext: &[u8]) -> LabradorResult<Vec<u8>> {
        match mode {
            AesMode::Cbc => aes_cbc_encrypt(key, iv, plaintext),
            AesMode::Gcm => Err(CryptoError::AlgorithmUnsupported("AES-ECB模式请单独调用gcm_encrypt".to_string()).into()),
            AesMode::Ctr => aes_ctr_encrypt(key, iv, plaintext),
            AesMode::Ecb => aes_ecb_encrypt(key, plaintext),
            AesMode::Cfb => aes_cfb_encrypt(key, iv, plaintext),
        }
    }


    fn aes_decrypt(&self, mode: AesMode, key: &[u8], iv: &[u8], ciphertext: &[u8]) -> LabradorResult<Vec<u8>> {
        match mode {
            AesMode::Cbc => aes_cbc_decrypt(key, iv, ciphertext),
            AesMode::Gcm => Err(CryptoError::AlgorithmUnsupported("AES-ECB模式请单独调用gcm_decrypt".to_string()).into()),
            AesMode::Ctr => aes_ctr_decrypt(key, iv, ciphertext),
            AesMode::Ecb => aes_ecb_decrypt(key, ciphertext),
            AesMode::Cfb => aes_cfb_decrypt(key, iv, ciphertext),
        }
    }

    fn rsa_sign(&self, private_key: &[u8], data: &[u8], hash_type: HashType) -> LabradorResult<Vec<u8>> {
        use rsa::pkcs1::DecodeRsaPrivateKey;
        use rsa::pkcs8::DecodePrivateKey;
        use rsa::{Pkcs1v15Sign, RsaPrivateKey};

        // 尝试不同格式加载私钥
        let private_key_result = match hash_type {
            HashType::Sha1 => {
                RsaPrivateKey::from_pkcs1_der(private_key)
                    .or_else(|_| RsaPrivateKey::from_pkcs8_der(private_key).map_err(|err| CryptoError::Signing(format!("加载私钥失败: {}", err))))
            }
            HashType::Sha256 => {
                RsaPrivateKey::from_pkcs8_der(private_key)
                    .or_else(|_| RsaPrivateKey::from_pkcs1_der(private_key).map_err(|err| CryptoError::Signing(format!("加载私钥失败: {}", err))))
            }
        };

        let private_key = private_key_result
            .map_err(|e| CryptoError::Signing(format!("加载私钥失败: {}", e)))?;

        match hash_type {
            HashType::Sha1 => {
                use sha1::{Digest, Sha1};
                let hashed = Sha1::digest(data).to_vec();
                private_key
                    .sign(Pkcs1v15Sign::new::<sha1::Sha1>(), &hashed)
                    .map_err(|e| CryptoError::Signing(format!("SHA1签名失败: {}", e)).into())
            }
            HashType::Sha256 => {
                use sha2::{Digest, Sha256};
                let hashed = Sha256::digest(data).to_vec();
                private_key
                    .sign(Pkcs1v15Sign::new::<sha2::Sha256>(), &hashed)
                    .map_err(|e| CryptoError::Signing(format!("SHA256签名失败: {}", e)).into())
            }
        }
    }

    fn rsa_verify(&self, public_key: &[u8], data: &[u8], signature: &[u8], hash_type: HashType) -> LabradorResult<bool> {
        use rsa::pkcs1::DecodeRsaPublicKey;
        use rsa::pkcs8::DecodePublicKey;
        use rsa::{Pkcs1v15Sign, RsaPublicKey};

        let public_key = RsaPublicKey::from_public_key_der(public_key)
            .or_else(|_| RsaPublicKey::from_pkcs1_der(public_key))
            .map_err(|e| CryptoError::Verification(format!("加载公钥失败: {}", e)))?;

        let hashed = match hash_type {
            HashType::Sha1 => {
                use sha1::{Digest, Sha1};
                Sha1::digest(data).to_vec()
            }
            HashType::Sha256 => {
                use sha2::{Digest, Sha256};
                Sha256::digest(data).to_vec()
            }
        };

        match hash_type {
            HashType::Sha1 => {
                public_key
                    .verify(Pkcs1v15Sign::new::<sha1::Sha1>(), &hashed, signature).map(|_| true)
                    .map_err(|e| CryptoError::Verification(format!("验证失败: {}", e)).into())
            }
            HashType::Sha256 => {
                public_key
                    .verify(Pkcs1v15Sign::new::<sha2::Sha256>(), &hashed, signature).map(|_| true)
                    .map_err(|e| CryptoError::Verification(format!("验证失败: {}", e)).into())
            }
        }
    }

    fn random_bytes(&self, len: usize) -> LabradorResult<Vec<u8>> {
        let rng = SystemRandom::new();
        let mut bytes = vec![0u8; len];
        rng.fill(&mut bytes)
            .map_err(|e| CryptoError::Other(format!("生成随机数失败: {}", e)))?;
        Ok(bytes)
    }

    fn pbkdf2(&self, password: &[u8], salt: &[u8], iterations: u32, key_len: usize) -> LabradorResult<Vec<u8>> {
        use ring::pbkdf2;
        let iterations = NonZeroU32::new(iterations)
            .ok_or_else(|| CryptoError::Other("迭代次数必须大于0".to_string()))?;

        let mut derived_key = vec![0u8; key_len];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            iterations,
            salt,
            password,
            &mut derived_key,
        );

        Ok(derived_key)
    }
}

// 通用 ECB 加密函数
fn aes_ecb_encrypt_generic<T>(key: &[u8], plaintext: &[u8]) -> LabradorResult<Vec<u8>>
where
    T: aes::cipher::KeyInit + aes::cipher::BlockEncryptMut,
{
    let block_size = 16;
    let padded_len = (plaintext.len() + block_size - 1) / block_size * block_size;
    let mut buffer = vec![0u8; padded_len];
    buffer[..plaintext.len()].copy_from_slice(plaintext);

    // PKCS7 填充
    let pad_byte = (block_size - plaintext.len() % block_size) as u8;
    for i in plaintext.len()..padded_len {
        buffer[i] = pad_byte;
    }

    // 直接操作切片，避免创建 Vec<GenericArray>
    let mut cipher = T::new(GenericArray::from_slice(key));

    // 将 buffer 按块大小分成切片并加密
    for chunk in buffer.chunks_exact_mut(block_size) {
        let block = GenericArray::from_mut_slice(chunk);
        cipher.encrypt_block_mut(block);  // 逐块加密
    }

    Ok(buffer)
}

// 通用 ECB 解密函数
fn aes_ecb_decrypt_generic<T>(key: &[u8], ciphertext: &[u8]) -> LabradorResult<Vec<u8>>
where
    T: aes::cipher::KeyInit + aes::cipher::BlockDecryptMut,
{
    let block_size = 16;
    let mut buffer = ciphertext.to_vec();

    // 初始化解密器并解密
    let mut cipher = T::new(GenericArray::from_slice(key));
    let decrypted = cipher
        .decrypt_padded_mut::<block_padding::Pkcs7>(&mut buffer)
        .map_err(|e| CryptoError::Decryption(format!("ECB解密失败: {}", e)))?
        .to_vec();

    Ok(decrypted)
}

// ECB 加密入口函数
fn aes_ecb_encrypt(key: &[u8], plaintext: &[u8]) -> LabradorResult<Vec<u8>> {
    match key.len() {
        16 => aes_ecb_encrypt_generic::<ecb::Encryptor<aes::Aes128>>(key, plaintext),
        24 => aes_ecb_encrypt_generic::<ecb::Encryptor<aes::Aes192>>(key, plaintext),
        32 => aes_ecb_encrypt_generic::<ecb::Encryptor<aes::Aes256>>(key, plaintext),
        _ => Err(CryptoError::Encryption(format!(
            "无效的AES密钥长度: {} (必须是16, 24或32字节)",
            key.len()
        ))
            .into()),
    }
}

// ECB 解密入口函数
fn aes_ecb_decrypt(key: &[u8], ciphertext: &[u8]) -> LabradorResult<Vec<u8>> {
    match key.len() {
        16 => aes_ecb_decrypt_generic::<ecb::Decryptor<aes::Aes128>>(key, ciphertext),
        24 => aes_ecb_decrypt_generic::<ecb::Decryptor<aes::Aes192>>(key, ciphertext),
        32 => aes_ecb_decrypt_generic::<ecb::Decryptor<aes::Aes256>>(key, ciphertext),
        _ => Err(CryptoError::Decryption(format!(
            "无效的AES密钥长度: {} (必须是16, 24或32字节)",
            key.len()
        ))
            .into()),
    }
}

//
// // AES-ECB加密
// fn aes_ecb_encrypt(key: &[u8], plaintext: &[u8]) -> LabradorResult<Vec<u8>> {
//     use aes::cipher::{BlockEncryptMut, KeyInit};
//     use block_padding::Pkcs7;
//
//     // 检查密钥长度
//     match key.len() {
//         16 => {
//             type Aes128EcbEnc = ecb::Encryptor<aes::Aes128>;
//             let mut cipher = Aes128EcbEnc::new(GenericArray::from_slice(key));
//             // 计算填充后的大小
//             let block_size = 16;
//             let padded_len = (plaintext.len() + block_size - 1) / block_size * block_size;
//             let mut buffer = vec![0u8; padded_len];
//             buffer[..plaintext.len()].copy_from_slice(plaintext);
//
//             // 手动进行PKCS7填充
//             let pad_byte = (block_size - plaintext.len() % block_size) as u8;
//             for i in plaintext.len()..padded_len {
//                 buffer[i] = pad_byte;
//             }
//
//             // 将缓冲区转换为 block 数组
//             let blocks = buffer.chunks_exact_mut(block_size);
//             let mut blocks_vec: Vec<GenericArray<u8, typenum::U16>> = Vec::new();
//
//             for chunk in blocks {
//                 let block = GenericArray::from_mut_slice(chunk);
//                 blocks_vec.push(*block);
//             }
//
//             // 现在传递 block 数组
//             cipher.encrypt_blocks_mut(&mut blocks_vec);
//
//             // 将 block 数组转回字节数组
//             let mut result = Vec::with_capacity(padded_len);
//             for block in blocks_vec {
//                 result.extend_from_slice(&block);
//             }
//
//             Ok(result)
//         }
//         24 => {
//             type Aes192EcbEnc = ecb::Encryptor<aes::Aes192>;
//             let mut cipher = Aes192EcbEnc::new(GenericArray::from_slice(key));
//             // 计算填充后的大小
//             let block_size = 16;
//             let padded_len = (plaintext.len() + block_size - 1) / block_size * block_size;
//             let mut buffer = vec![0u8; padded_len];
//             buffer[..plaintext.len()].copy_from_slice(plaintext);
//
//             // 手动进行PKCS7填充
//             let pad_byte = (block_size - plaintext.len() % block_size) as u8;
//             for i in plaintext.len()..padded_len {
//                 buffer[i] = pad_byte;
//             }
//
//             // 将缓冲区转换为 block 数组
//             let blocks = buffer.chunks_exact_mut(block_size);
//             let mut blocks_vec: Vec<GenericArray<u8, typenum::U16>> = Vec::new();
//
//             for chunk in blocks {
//                 let block = GenericArray::from_mut_slice(chunk);
//                 blocks_vec.push(*block);
//             }
//
//             // 现在传递 block 数组
//             cipher.encrypt_blocks_mut(&mut blocks_vec);
//
//             // 将 block 数组转回字节数组
//             let mut result = Vec::with_capacity(padded_len);
//             for block in blocks_vec {
//                 result.extend_from_slice(&block);
//             }
//
//             Ok(result)
//         }
//         32 => {
//             type Aes256EcbEnc = ecb::Encryptor<aes::Aes256>;
//             let mut cipher = Aes256EcbEnc::new(GenericArray::from_slice(key));
//             // 计算填充后的大小
//             let block_size = 16;
//             let padded_len = (plaintext.len() + block_size - 1) / block_size * block_size;
//             let mut buffer = vec![0u8; padded_len];
//             buffer[..plaintext.len()].copy_from_slice(plaintext);
//
//             // 手动进行PKCS7填充
//             let pad_byte = (block_size - plaintext.len() % block_size) as u8;
//             for i in plaintext.len()..padded_len {
//                 buffer[i] = pad_byte;
//             }
//
//             // 将缓冲区转换为 block 数组
//             let blocks = buffer.chunks_exact_mut(block_size);
//             let mut blocks_vec: Vec<GenericArray<u8, typenum::U16>> = Vec::new();
//
//             for chunk in blocks {
//                 let block = GenericArray::from_mut_slice(chunk);
//                 blocks_vec.push(*block);
//             }
//
//             // 现在传递 block 数组
//             cipher.encrypt_blocks_mut(&mut blocks_vec);
//
//             // 将 block 数组转回字节数组
//             let mut result = Vec::with_capacity(padded_len);
//             for block in blocks_vec {
//                 result.extend_from_slice(&block);
//             }
//
//             Ok(result)
//         }
//         _ => {
//             return Err(CryptoError::Encryption(format!(
//                 "无效的AES密钥长度: {} (必须是16, 24或32字节)",
//                 key.len()
//             ))
//                 .into());
//         }
//     }
// }


// AES-CBC加密
fn aes_cbc_encrypt(key: &[u8], iv: &[u8], plaintext: &[u8]) -> LabradorResult<Vec<u8>> {
    use aes::cipher::{generic_array::GenericArray, BlockEncryptMut, KeyIvInit};

    // 检查密钥长度
    match key.len() {
        16 => {
            type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;

            let key_arr = GenericArray::from_slice(key);
            let iv_arr = GenericArray::from_slice(iv);
            let mut cipher = Aes128CbcEnc::new(key_arr, iv_arr);

            // 计算填充后的大小
            let block_size = 16;
            let padded_len = (plaintext.len() + block_size - 1) / block_size * block_size;
            let mut buffer = vec![0u8; padded_len];
            buffer[..plaintext.len()].copy_from_slice(plaintext);

            // 手动进行PKCS7填充
            let pad_byte = (block_size - plaintext.len() % block_size) as u8;
            for i in plaintext.len()..padded_len {
                buffer[i] = pad_byte;
            }

            // 将缓冲区转换为 block 数组
            let blocks = buffer.chunks_exact_mut(block_size);
            let mut blocks_vec: Vec<GenericArray<u8, typenum::U16>> = Vec::new();

            for chunk in blocks {
                let block = GenericArray::from_mut_slice(chunk);
                blocks_vec.push(*block);
            }

            // 现在传递 block 数组
            cipher.encrypt_blocks_mut(&mut blocks_vec);

            // 将 block 数组转回字节数组
            let mut result = Vec::with_capacity(padded_len);
            for block in blocks_vec {
                result.extend_from_slice(&block);
            }

            Ok(result)
        }
        24 => {
            type Aes192CbcEnc = cbc::Encryptor<aes::Aes192>;

            let key_arr = GenericArray::from_slice(key);
            let iv_arr = GenericArray::from_slice(iv);
            let mut cipher = Aes192CbcEnc::new(key_arr, iv_arr);

            let block_size = 16;
            let padded_len = (plaintext.len() + block_size - 1) / block_size * block_size;
            let mut buffer = vec![0u8; padded_len];
            buffer[..plaintext.len()].copy_from_slice(plaintext);

            let pad_byte = (block_size - plaintext.len() % block_size) as u8;
            for i in plaintext.len()..padded_len {
                buffer[i] = pad_byte;
            }

            // 简化的方法：使用 encrypt_padded_mut（如果可用）
            // 或者使用 chunks_exact_mut 创建 block 数组
            let blocks_count = padded_len / block_size;
            let mut blocks: Vec<GenericArray<u8, typenum::U16>> = Vec::with_capacity(blocks_count);

            for i in 0..blocks_count {
                let start = i * block_size;
                let end = start + block_size;
                let block = GenericArray::clone_from_slice(&buffer[start..end]);
                blocks.push(block);
            }

            cipher.encrypt_blocks_mut(&mut blocks);

            // 将 blocks 转回字节
            let mut result = Vec::with_capacity(padded_len);
            for block in blocks {
                result.extend_from_slice(&block);
            }

            Ok(result)
        }
        32 => {
            type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;

            let key_arr = GenericArray::from_slice(key);
            let iv_arr = GenericArray::from_slice(iv);
            let mut cipher = Aes256CbcEnc::new(key_arr, iv_arr);

            let block_size = 16;
            let padded_len = (plaintext.len() + block_size - 1) / block_size * block_size;
            let mut buffer = vec![0u8; padded_len];
            buffer[..plaintext.len()].copy_from_slice(plaintext);

            let pad_byte = (block_size - plaintext.len() % block_size) as u8;
            for i in plaintext.len()..padded_len {
                buffer[i] = pad_byte;
            }
            use typenum::U16;

            let block_count = buffer.len() / 16;
            let mut blocks: Vec<GenericArray<u8, U16>> = Vec::with_capacity(block_count);

            for i in 0..block_count {
                let mut block = GenericArray::default();
                block.copy_from_slice(&buffer[i * 16..(i + 1) * 16]);
                blocks.push(block);
            }

            cipher.encrypt_blocks_mut(&mut blocks);

            // 将结果复制回 buffer
            for (i, block) in blocks.into_iter().enumerate() {
                buffer[i * 16..(i + 1) * 16].copy_from_slice(&block);
            }

            Ok(buffer)
        }
        _ => Err(CryptoError::Encryption(format!(
            "无效的AES密钥长度: {} (必须是16, 24或32字节)",
            key.len()
        )).into()),
    }
}

// AES-CBC解密
fn aes_cbc_decrypt(key: &[u8], iv: &[u8], ciphertext: &[u8]) -> LabradorResult<Vec<u8>> {
    use aes::cipher::{generic_array::GenericArray, BlockDecryptMut, KeyIvInit};
    use block_padding::Pkcs7;
    use cbc::Decryptor;

    match key.len() {
        16 => {
            type Aes128CbcDec = Decryptor<aes::Aes128>;
            let key_arr = GenericArray::from_slice(key);
            let iv_arr = GenericArray::from_slice(iv);
            let mut cipher = Aes128CbcDec::new(key_arr, iv_arr);

            let mut buffer = ciphertext.to_vec();

            // 使用 decrypt_padded_mut 方法
            let result = cipher.decrypt_padded_mut::<Pkcs7>(&mut buffer)
                .map_err(|e| CryptoError::Decryption(format!("解密失败: {}", e)))?
                .to_vec();

            Ok(result)
        }
        24 => {
            type Aes192CbcDec = Decryptor<aes::Aes192>;
            let key_arr = GenericArray::from_slice(key);
            let iv_arr = GenericArray::from_slice(iv);
            let mut cipher = Aes192CbcDec::new(key_arr, iv_arr);

            let mut buffer = ciphertext.to_vec();
            let result = cipher.decrypt_padded_mut::<Pkcs7>(&mut buffer)
                .map_err(|e| CryptoError::Decryption(format!("解密失败: {}", e)))?
                .to_vec();

            Ok(result)
        }
        32 => {
            type Aes256CbcDec = Decryptor<aes::Aes256>;
            let key_arr = GenericArray::from_slice(key);
            let iv_arr = GenericArray::from_slice(iv);
            let mut cipher = Aes256CbcDec::new(key_arr, iv_arr);

            let mut buffer = ciphertext.to_vec();
            let result = cipher.decrypt_padded_mut::<Pkcs7>(&mut buffer)
                .map_err(|e| CryptoError::Decryption(format!("解密失败: {}", e)))?
                .to_vec();

            Ok(result)
        }
        _ => Err(CryptoError::Decryption(format!(
            "无效的AES密钥长度: {} (必须是16, 24或32字节)",
            key.len()
        )).into()),
    }
}

// AES-CTR加密
fn aes_ctr_encrypt(key: &[u8], iv: &[u8], plaintext: &[u8]) -> LabradorResult<Vec<u8>> {
    use aes::cipher::{generic_array::GenericArray, KeyIvInit, StreamCipher};

    match key.len() {
        16 => {
            type Aes128Ctr = ctr::Ctr128BE<aes::Aes128>;

            let key_arr = GenericArray::from_slice(key);
            let iv_arr = GenericArray::from_slice(iv);
            let mut cipher = Aes128Ctr::new(key_arr, iv_arr);

            let mut buffer = plaintext.to_vec();
            cipher.apply_keystream(&mut buffer);
            Ok(buffer)
        }
        24 => {
            type Aes192Ctr = ctr::Ctr128BE<aes::Aes192>;

            let key_arr = GenericArray::from_slice(key);
            let iv_arr = GenericArray::from_slice(iv);
            let mut cipher = Aes192Ctr::new(key_arr, iv_arr);

            let mut buffer = plaintext.to_vec();
            cipher.apply_keystream(&mut buffer);
            Ok(buffer)
        }
        32 => {
            type Aes256Ctr = ctr::Ctr128BE<aes::Aes256>;

            let key_arr = GenericArray::from_slice(key);
            let iv_arr = GenericArray::from_slice(iv);
            let mut cipher = Aes256Ctr::new(key_arr, iv_arr);

            let mut buffer = plaintext.to_vec();
            cipher.apply_keystream(&mut buffer);
            Ok(buffer)
        }
        _ => Err(CryptoError::Encryption(format!(
            "无效的AES密钥长度: {} (必须是16, 24或32字节)",
            key.len()
        )).into()),
    }
}

// AES-CTR解密
fn aes_ctr_decrypt(key: &[u8], iv: &[u8], ciphertext: &[u8]) -> LabradorResult<Vec<u8>> {
    // CTR模式的加密和解密是相同的
    aes_ctr_encrypt(key, iv, ciphertext)
}

// AES-CFB加密
fn aes_cfb_encrypt(key: &[u8], iv: &[u8], plaintext: &[u8]) -> LabradorResult<Vec<u8>> {
    use aes::cipher::{AsyncStreamCipher, KeyIvInit};
    match key.len() {
        16 => {
            type Aes128CfbEnc = cfb_mode::Encryptor<aes::Aes128>;
            let mut cipher = Aes128CfbEnc::new_from_slices(key, iv)
                .map_err(|e| CryptoError::Encryption(format!("创建CFB密码失败: {}", e)))?;
            let mut buffer = plaintext.to_vec();
            cipher.encrypt(&mut buffer);
            Ok(buffer)
        }
        24 => {
            type Aes192CfbEnc = cfb_mode::Encryptor<aes::Aes192>;
            let mut cipher = Aes192CfbEnc::new_from_slices(key, iv)
                .map_err(|e| CryptoError::Encryption(format!("创建CFB密码失败: {}", e)))?;
            let mut buffer = plaintext.to_vec();
            cipher.encrypt(&mut buffer);
            Ok(buffer)
        }
        32 => {
            type Aes256CfbEnc = cfb_mode::Encryptor<aes::Aes256>;
            let mut cipher = Aes256CfbEnc::new_from_slices(key, iv)
                .map_err(|e| CryptoError::Encryption(format!("创建CFB密码失败: {}", e)))?;
            let mut buffer = plaintext.to_vec();
            cipher.encrypt(&mut buffer);
            Ok(buffer)
        }
        _ => Err(CryptoError::Encryption(format!(
            "无效的AES密钥长度: {} (必须是16, 24或32字节)",
            key.len()
        )).into()),
    }
}

fn aes_cfb_decrypt(key: &[u8], iv: &[u8], ciphertext: &[u8]) -> LabradorResult<Vec<u8>> {
    use aes::cipher::{AsyncStreamCipher, KeyIvInit};
    match key.len() {
        16 => {
            type Aes128CfbDec = cfb_mode::Decryptor<aes::Aes128>;
            let mut cipher = Aes128CfbDec::new_from_slices(key, iv)
                .map_err(|e| CryptoError::Decryption(format!("创建CFB密码失败: {}", e)))?;
            let mut buffer = ciphertext.to_vec();
            cipher.decrypt(&mut buffer);
            Ok(buffer)
        }
        24 => {
            type Aes192CfbDec = cfb_mode::Decryptor<aes::Aes192>;
            let mut cipher = Aes192CfbDec::new_from_slices(key, iv)
                .map_err(|e| CryptoError::Decryption(format!("创建CFB密码失败: {}", e)))?;
            let mut buffer = ciphertext.to_vec();
            cipher.decrypt(&mut buffer);
            Ok(buffer)
        }
        32 => {
            type Aes256CfbDec = cfb_mode::Decryptor<aes::Aes256>;
            let mut cipher = Aes256CfbDec::new_from_slices(key, iv)
                .map_err(|e| CryptoError::Decryption(format!("创建CFB密码失败: {}", e)))?;
            let mut buffer = ciphertext.to_vec();
            cipher.decrypt(&mut buffer);
            Ok(buffer)
        }
        _ => Err(CryptoError::Decryption(format!(
            "无效的AES密钥长度: {} (必须是16, 24或32字节)",
            key.len()
        )).into()),
    }
}


// 简化的AES-GCM加密（无关联数据）
fn aes_gcm_encrypt_simple(key: &[u8], nonce: &[u8], plaintext: &[u8]) -> LabradorResult<Vec<u8>> {
    use aes_gcm::{
        aead::{Aead, KeyInit},
        Aes128Gcm, Aes256Gcm, Key, Nonce
    };
    match key.len() {
        16 => {
            let key = Key::<Aes128Gcm>::from_slice(key);
            let cipher = Aes128Gcm::new(key);
            let nonce = Nonce::from_slice(nonce);

            cipher.encrypt(nonce, plaintext)
                .map_err(|e| CryptoError::Encryption(format!("GCM加密失败: {}", e)).into())
        }
        32 => {
            let key = Key::<Aes256Gcm>::from_slice(key);
            let cipher = Aes256Gcm::new(key);
            let nonce = Nonce::from_slice(nonce);

            cipher.encrypt(nonce, plaintext)
                .map_err(|e| CryptoError::Encryption(format!("GCM加密失败: {}", e)).into())
        }
        _ => Err(CryptoError::Encryption(format!(
            "无效的AES-GCM密钥长度: {} (必须是16或32字节)",
            key.len()
        )).into()),
    }
}

// 简化的AES-GCM解密（无关联数据）
fn aes_gcm_decrypt_simple(key: &[u8], nonce: &[u8], ciphertext: &[u8], tag: &[u8]) -> LabradorResult<Vec<u8>> {
    use aes_gcm::{
        aead::{Aead, KeyInit, Payload},
        Aes128Gcm, Aes256Gcm, Key, Nonce
    };
    // 合并密文和标签
    let mut combined = ciphertext.to_vec();
    combined.extend_from_slice(tag);

    match key.len() {
        16 => {
            let key = Key::<Aes128Gcm>::from_slice(key);
            let cipher = Aes128Gcm::new(key);
            let nonce = Nonce::from_slice(nonce);

            cipher.decrypt(nonce, Payload::from(&combined[..]))
                .map_err(|e| CryptoError::Decryption(format!("GCM解密失败: {}", e)).into())
        }
        32 => {
            let key = Key::<Aes256Gcm>::from_slice(key);
            let cipher = Aes256Gcm::new(key);
            let nonce = Nonce::from_slice(nonce);

            cipher.decrypt(nonce, Payload::from(&combined[..]))
                .map_err(|e| CryptoError::Decryption(format!("GCM解密失败: {}", e)).into())
        }
        _ => Err(CryptoError::Decryption(format!(
            "无效的AES-GCM密钥长度: {} (必须是16或32字节)",
            key.len()
        )).into()),
    }
}


// 公共接口函数
pub fn hash(algorithm: HashAlgorithm, data: &[u8]) -> Vec<u8> {
    RustCrypto.hash(algorithm, data)
}

pub fn hmac(algorithm: HmacAlgorithm, key: &[u8], data: &[u8]) -> LabradorResult<Vec<u8>> {
    RustCrypto.hmac(algorithm, key, data)
}

pub fn aes_encrypt(mode: AesMode, key: &[u8], iv: &[u8], plaintext: &[u8]) -> LabradorResult<Vec<u8>> {
    RustCrypto.aes_encrypt(mode, key, iv, plaintext)
}

pub fn aes_decrypt(mode: AesMode, key: &[u8], iv: &[u8], ciphertext: &[u8]) -> LabradorResult<Vec<u8>> {
    RustCrypto.aes_decrypt(mode, key, iv, ciphertext)
}

pub fn aes_gcm_encrypt(key: &[u8], nonce: &[u8], associated_data: &[u8], plaintext: &[u8]) -> LabradorResult<Vec<u8>> {
    use aes_gcm::{
        aead::{AeadInPlace, KeyInit},
        Aes128Gcm, Aes256Gcm, Key, Nonce
    };
    match key.len() {
        16 => {
            let key = Key::<Aes128Gcm>::from_slice(key);
            let cipher = Aes128Gcm::new(key);
            let nonce = Nonce::from_slice(nonce);

            // 创建缓冲区并加密
            let mut buffer = plaintext.to_vec();
            cipher.encrypt_in_place_detached(nonce, associated_data, &mut buffer)
                .map_err(|e| CryptoError::Encryption(format!("GCM加密失败: {}", e)))?
                .to_vec();

            // 注意：aes-gcm 0.10.3 版本中 encrypt_in_place_detached 已经包含了标签
            Ok(buffer)
        }
        32 => {
            let key = Key::<Aes256Gcm>::from_slice(key);
            let cipher = Aes256Gcm::new(key);
            let nonce = Nonce::from_slice(nonce);

            let mut buffer = plaintext.to_vec();
            cipher.encrypt_in_place_detached(nonce, associated_data, &mut buffer)
                .map_err(|e| CryptoError::Encryption(format!("GCM加密失败: {}", e)))?
                .to_vec();

            Ok(buffer)
        }
        _ => Err(CryptoError::Encryption(format!(
            "无效的AES-GCM密钥长度: {} (必须是16或32字节)",
            key.len()
        )).into()),
    }
}

pub fn aes_gcm_decrypt(key: &[u8], nonce: &[u8], associated_data: &[u8], ciphertext: &[u8], tag: &[u8]) -> LabradorResult<Vec<u8>> {
    use aes_gcm::{
        aead::{AeadInPlace, KeyInit},
        Aes128Gcm, Aes256Gcm, Key, Nonce, Tag
    };
    match key.len() {
        16 => {
            let key = Key::<Aes128Gcm>::from_slice(key);
            let cipher = Aes128Gcm::new(key);
            let nonce = Nonce::from_slice(nonce);

            // 复制密文并解密
            let mut buffer = ciphertext.to_vec();
            let tag = Tag::from_slice(tag);
            cipher.decrypt_in_place_detached(nonce, associated_data, &mut buffer, tag)
                .map_err(|e| CryptoError::Decryption(format!("GCM解密失败: {}", e)))?;

            Ok(buffer)
        }
        32 => {
            let key = Key::<Aes256Gcm>::from_slice(key);
            let cipher = Aes256Gcm::new(key);
            let nonce = Nonce::from_slice(nonce);

            let mut buffer = ciphertext.to_vec();
            let tag = Tag::from_slice(tag);
            cipher.decrypt_in_place_detached(nonce, associated_data, &mut buffer, tag)
                .map_err(|e| CryptoError::Decryption(format!("GCM解密失败: {}", e)))?;

            Ok(buffer)
        }
        _ => Err(CryptoError::Decryption(format!(
            "无效的AES-GCM密钥长度: {} (必须是16或32字节)",
            key.len()
        )).into()),
    }
}

pub fn random_bytes(len: usize) -> LabradorResult<Vec<u8>> {
    RustCrypto.random_bytes(len)
}

pub fn pbkdf2(password: &[u8], salt: &[u8], iterations: u32, key_len: usize) -> LabradorResult<Vec<u8>> {
    RustCrypto.pbkdf2(password, salt, iterations, key_len)
}

pub fn rsa_sign(private_key: &[u8], data: &[u8], hash_type: HashType) -> LabradorResult<Vec<u8>> {
    RustCrypto.rsa_sign(private_key, data, hash_type)
}

pub fn rsa_verify(public_key: &[u8], data: &[u8], signature: &[u8], hash_type: HashType) -> LabradorResult<bool> {
    RustCrypto.rsa_verify(public_key, data, signature, hash_type)
}

pub fn rsa_encrypt(public_key: &[u8], plaintext: &[u8]) -> LabradorResult<Vec<u8>> {
    use rsa::pkcs1::DecodeRsaPublicKey;
    use rsa::pkcs8::DecodePublicKey;
    use rsa::{Oaep, RsaPublicKey};
    use sha2::Sha256;
    
    let public_key = RsaPublicKey::from_public_key_der(public_key)
        .or_else(|_| RsaPublicKey::from_pkcs1_der(public_key))
        .map_err(|e| CryptoError::Encryption(format!("加载公钥失败: {}", e)))?;

    let padding = Oaep::new::<Sha256>();
    public_key.encrypt(&mut rand::thread_rng(), padding, plaintext)
        .map_err(|e| CryptoError::Encryption(format!("RSA加密失败: {}", e)).into())
}

/// 从 PEM 提取 DER 数据
pub fn extract_der_from_pem(pem_data: &[u8]) -> LabradorResult<Vec<u8>> {
    use x509_parser::pem::Pem;
    let mut iter = Pem::iter_from_buffer(pem_data);

    iter.next()
        .ok_or_else(|| LabraError::Certificate("PEM 文件为空".to_string()))?
        .map(|pem| pem.contents.to_vec())
        .map_err(|e| LabraError::Certificate(format!("解析PEM失败: {}", e)))
}
pub fn rsa_decrypt(private_key: &[u8], ciphertext: &[u8]) -> LabradorResult<Vec<u8>> {
    use rsa::pkcs1::DecodeRsaPrivateKey;
    use rsa::pkcs8::DecodePrivateKey;
    use rsa::{Oaep, RsaPrivateKey};
    use sha2::Sha256;

    let private_key = RsaPrivateKey::from_pkcs8_der(private_key)
        .or_else(|_| RsaPrivateKey::from_pkcs1_der(private_key))
        .map_err(|e| CryptoError::Decryption(format!("加载私钥失败: {}", e)))?;

    let padding = Oaep::new::<Sha256>();
    private_key.decrypt(padding, ciphertext)
        .map_err(|e| CryptoError::Decryption(format!("RSA解密失败: {}", e)).into())
}


#[cfg(test)]
mod tests {
    use crate::{RsaEncryptor, RsaKeyFormat};
    use crate::utils::encryption::{base64_decode, base64_encode};
    use super::*;
    
    
    #[test]
    fn test_rsa() {
        let private_key = "";
        let private_key = base64_decode(private_key).unwrap();
        let si = RsaEncryptor::with_private_key(&private_key, RsaKeyFormat::Pkcs1);
        let s = si.sign("ssss".as_bytes(), HashType::Sha256).unwrap();
        println!("{}", base64_encode(&s));
        
    }
}