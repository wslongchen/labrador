
//! 
//! MD5加密类
//!
use openssl::hash::{Hasher, MessageDigest};
use rustc_serialize::hex::{ToHex};

#[allow(unused)]
static SALT: &'static str = "labrador";

///
/// MD5 encry
/// 
#[allow(unused)]
pub fn md5_with_salt_default<S:Into<String>>(input: S) -> String {
    let input: String = input.into();
    let mut result = String::default();
    if let Ok(mut h) = Hasher::new(MessageDigest::md5()) {
        h.update(SALT.as_bytes()).unwrap();
        h.update(input.as_bytes()).unwrap();
        let res = h.finish().unwrap();
        result = res.to_hex();
    }
    result
}

///
/// MD5 encry
///
#[allow(unused)]
pub fn md5<S:Into<String>>(input: S) -> String {
    let input: String = input.into();
    let mut result = String::default();

    if let Ok(mut h) = Hasher::new(MessageDigest::md5()) {
        h.update(input.as_bytes()).unwrap();
        let res = h.finish().unwrap();
        result = res.to_hex();
    }
    result
}
///
/// MD5 encry
///
#[allow(unused)]
pub fn md5_salt<S:Into<String>>(input: S, salt: S) -> String {
    let input: String = input.into();
    let salt: String = salt.into();
    let mut result = String::default();
    if let Ok(mut h) = Hasher::new(MessageDigest::md5()) {
        h.update(input.as_bytes()).unwrap();
        h.update(salt.as_bytes()).unwrap();
        let res = h.finish().unwrap();
        result = res.to_hex();
    }
    result
}

///
/// 校验加密串是否匹配
/// 
#[allow(unused)]
pub fn validate<S: Into<String>>(input_source: S, input_target: S) -> bool {
    let source = input_source.into();
    let target = md5_with_salt_default(input_target);
    source.eq(&target)
}

#[test]
fn test_md5() {
    let s = md5("sdfsdfasdfasf");
    println!("md5:{}", s);
}