
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

//!
//! MD5加密类
//!


#[allow(unused)]
static SALT: &'static str = "labrador";

///
/// MD5 encry
/// 
#[allow(unused)]
pub fn md5<S:Into<String>>(input: S) -> String {
    let input: String = input.into();

    #[cfg(feature = "openssl-crypto")]
    fn crypto_md5(input: String) -> String {
        match openssl::hash::Hasher::new(openssl::hash::MessageDigest::md5()) {
            Ok(mut h) => {
                h.update(input.as_bytes()).unwrap();
                let res = h.finish().unwrap();
                hex::encode(res)
            }
            Err(_) => String::new(),
        }
    }

    #[cfg(not(feature = "openssl-crypto"))]
    fn crypto_md5(input: String) -> String {
        use std::ops::Deref;
        let mut input_salt: String = String::new();
        input_salt.push_str(input.as_str());
        let result = md5::compute(input_salt.as_bytes());
        hex::encode(result.deref()).to_string()
    }
    crypto_md5(input).to_string()
}


///
/// 校验加密串是否匹配
/// 
#[allow(unused)]
pub fn validate<S: Into<String>>(input_source: S, input_target: S) -> bool {
    let source = input_source.into();
    let target = md5(input_target);
    source.eq(&target)
}

#[test]
fn test_md5() {
    let s = md5("sdfsdfasdfasf");
    println!("md5:{}", s);
}