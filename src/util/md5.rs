
//! 
//! MD5加密类
//!
use std::ops::Deref;

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

        let mut result = String::default();
        if let Ok(mut h) = openssl::hash::Hasher::new(openssl::hash::MessageDigest::md5()) {
            h.update(input.as_bytes()).unwrap();
            let res = h.finish().unwrap();
            result = hex::encode(result).to_string();
        }
        result
    }

    #[cfg(not(feature = "openssl-crypto"))]
    fn crypto_md5(input: String) -> String {
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