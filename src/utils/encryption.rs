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
use base64::{engine::general_purpose, Engine as _};

/// Base64编码
pub fn base64_encode(data: &[u8]) -> String {
    general_purpose::STANDARD.encode(data)
}

/// Base64解码
pub fn base64_decode(data: &str) -> Result<Vec<u8>, base64::DecodeError> {
    general_purpose::STANDARD.decode(data)
}

/// URL安全的Base64编码
pub fn base64_url_encode(data: &[u8]) -> String {
    general_purpose::URL_SAFE.encode(data)
}

/// URL安全的Base64解码
pub fn base64_url_decode(data: &str) -> Result<Vec<u8>, base64::DecodeError> {
    general_purpose::URL_SAFE.decode(data)
}

/// 十六进制编码
pub fn hex_encode(data: &[u8]) -> String {
    hex::encode(data)
}

/// 十六进制解码
pub fn hex_decode(data: &str) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(data)
}
