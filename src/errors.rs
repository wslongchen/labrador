use std::convert::Infallible;
use std::fmt;
use std::io;
use std::string::FromUtf8Error;
use base64::DecodeError;
use redis::RedisError;
use reqwest::header::InvalidHeaderValue;
use serde_json::{ error::Error as JsonError};
use tracing::error;
use x509_parser::der_parser::asn1_rs::SerializeError;

#[allow(unused)]
#[derive(Debug)]
pub enum LabraError {
    InvalidSignature(String),
    ApiError(String),
    InvalidAppId,
    ClientError { errcode: String, errmsg: String },
    IOError(io::Error),
    MissingField(String),
    RedundantField(String),
    RequestError(String),
    Unknown,
}

impl fmt::Display for LabraError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LabraError::InvalidSignature(ref err) => write!(f, "Invalid signature: {}", err),
            LabraError::InvalidAppId => write!(f, "Invalid app_id"),
            LabraError::ClientError { errcode, ref errmsg } => write!(f, "Client error code: {}, message: {}", errcode, errmsg),
            LabraError::IOError(ref err) => err.fmt(f),
            LabraError::MissingField(ref err) => write!(f, "Client MissingField message: {}", err),
            LabraError::RedundantField(ref err) => write!(f, "Client RedundantField , message: {}", err),
            LabraError::ApiError(ref err) => write!(f, "Client ApiError , message: {}", err),
            LabraError::RequestError(ref err) => write!(f, "Request Error {}", err),
            LabraError::Unknown => write!(f, "Unknown Error")
        }
    }
}

#[allow(deprecated, deprecated_in_future)]
impl std::error::Error for LabraError {
    fn description(&self) -> &str {
        match *self {
            LabraError::InvalidSignature(ref err) => err,
            LabraError::InvalidAppId => "Invalid app_id",
            LabraError::ClientError { ref errmsg, .. } => errmsg,
            LabraError::IOError(ref err) => err.description(),
            LabraError::MissingField(ref err) => err,
            LabraError::RedundantField(ref err) => err,
            LabraError::ApiError(ref err) => err,
            LabraError::RequestError(ref err) => err,
            LabraError::Unknown => "Request Error"
        }
    }
}

impl From<reqwest::Error> for LabraError {
    fn from(_err: reqwest::Error) -> Self {
        error!("error to request:{:?}", _err);
        LabraError::RequestError(_err.to_string())
    }
}

impl From<io::Error> for LabraError {
    fn from(err: io::Error) -> Self {
        LabraError::IOError(err)
    }
}

impl From<JsonError> for LabraError {
    fn from(_err: JsonError) -> Self {
        error!("error to parse json:{:?}", _err);
        LabraError::RedundantField(_err.to_string())
    }
}


impl From<serde_xml_rs::Error> for LabraError {
    fn from(_err: serde_xml_rs::Error) -> Self {
        error!("error to parse xml:{:?}", _err);
        LabraError::RedundantField(_err.to_string())
    }
}

#[cfg(feature = "openssl-crypto")]
impl From<openssl::error::ErrorStack> for LabraError {
    fn from(err: openssl::error::ErrorStack) -> Self {
        LabraError::InvalidSignature(format!("加解密出错：{}", err.to_string()))
    }
}


#[cfg(not(feature = "openssl-crypto"))]
impl From<block_modes::InvalidKeyIvLength> for LabraError {
    fn from(err: block_modes::InvalidKeyIvLength) -> Self {
        LabraError::InvalidSignature(format!("加解密出错：{}", err.to_string()))
    }
}

#[cfg(not(feature = "openssl-crypto"))]
impl From<block_modes::BlockModeError> for LabraError {
    fn from(err: block_modes::BlockModeError) -> Self {
        LabraError::InvalidSignature(format!("加解密出错：{}", err.to_string()))
    }
}

#[cfg(not(feature = "openssl-crypto"))]
impl From<hmac::digest::InvalidLength> for LabraError {
    fn from(err: hmac::digest::InvalidLength) -> Self {
        LabraError::InvalidSignature(format!("加解密出错：{}", err.to_string()))
    }
}

#[cfg(not(feature = "openssl-crypto"))]
impl From<aes_gcm::Error> for LabraError {
    fn from(err: aes_gcm::Error) -> Self {
        LabraError::InvalidSignature(format!("加解密出错：{}", err.to_string()))
    }
}

impl From<FromUtf8Error> for LabraError {
    fn from(err: FromUtf8Error) -> Self {
        LabraError::InvalidSignature(format!("字符转换错误：{}", err.to_string()))
    }
}


impl From<InvalidHeaderValue> for LabraError {
    fn from(err: InvalidHeaderValue) -> Self {
        LabraError::RequestError(format!("请求头转换出错：{}", err.to_string()))
    }
}

impl From<hex::FromHexError> for LabraError {
    fn from(err: hex::FromHexError) -> Self {
        LabraError::InvalidSignature(format!("字符转码出错：{}", err.to_string()))
    }
}

impl From<serde_urlencoded::ser::Error> for LabraError {
    fn from(err: serde_urlencoded::ser::Error) -> Self {
        LabraError::InvalidSignature(format!("URL转码：{}", err.to_string()))
    }
}

impl From<serde_urlencoded::de::Error> for LabraError {
    fn from(err: serde_urlencoded::de::Error) -> Self {
        LabraError::InvalidSignature(format!("URL转码：{}", err.to_string()))
    }
}

// #[cfg(feature = "wechat")]
// impl From<openssl::error::ErrorStack> for LabraError {
//     fn from(_err: openssl::error::ErrorStack) -> Self {
//         LabraError::RequestError(_err.to_string())
//     }
// }


impl From<DecodeError> for LabraError {
    fn from(err: DecodeError) -> Self {
        LabraError::InvalidSignature(format!("字符编码出错：{}", err.to_string()))
    }
}
impl From<r2d2::Error> for LabraError {
    fn from(err: r2d2::Error) -> Self {
        LabraError::RequestError(format!("redis连接错误：{}", err.to_string()))
    }
}

impl From<RedisError> for LabraError {
    fn from(err: RedisError) -> Self {
        LabraError::RequestError(format!("redis错误：{}", err.to_string()))
    }
}


impl From<rsa::pkcs8::Error> for LabraError {
    fn from(err: rsa::pkcs8::Error) -> Self {
        LabraError::RequestError(err.to_string())
    }
}


impl From<rsa::pkcs1::Error> for LabraError {
    fn from(err: rsa::pkcs1::Error) -> Self {
        LabraError::RequestError(err.to_string())
    }
}


impl From<rsa::pkcs8::spki::Error> for LabraError {
    fn from(err: rsa::pkcs8::spki::Error) -> Self {
        LabraError::RequestError(err.to_string())
    }
}


impl From<Infallible> for LabraError {
    fn from(err: Infallible) -> Self {
        LabraError::RequestError(err.to_string())
    }
}


impl From<rsa::errors::Error> for LabraError {
    fn from(err: rsa::errors::Error) -> Self {
        LabraError::RequestError(err.to_string())
    }
}
impl From<x509_parser::nom::Err<x509_parser::prelude::PEMError>> for LabraError {
    fn from(err: x509_parser::nom::Err<x509_parser::prelude::PEMError>) -> Self {
        LabraError::RequestError(err.to_string())
    }
}

impl From<x509_parser::nom::Err<x509_parser::prelude::X509Error>> for LabraError {
    fn from(err: x509_parser::nom::Err<x509_parser::prelude::X509Error>) -> Self {
        LabraError::RequestError(err.to_string())
    }
}
impl From<SerializeError> for LabraError {
    fn from(err: SerializeError) -> Self {
        LabraError::RequestError(err.to_string())
    }
}