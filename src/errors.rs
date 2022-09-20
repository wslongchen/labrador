use std::fmt;
use std::io;
use std::string::FromUtf8Error;
use base64::DecodeError;
use openssl::error::ErrorStack;
use redis::RedisError;
use reqwest::header::InvalidHeaderValue;
use rustc_serialize::hex::FromHexError;
use serde_json::{ error::Error as JsonError};
use tracing::error;

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

impl From<ErrorStack> for LabraError {
    fn from(err: ErrorStack) -> Self {
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

impl From<FromHexError> for LabraError {
    fn from(err: FromHexError) -> Self {
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


// impl From<reqwest::> for LabraError {
//     fn from(err: url::parser::ParseError) -> Self {
//         LabraError::InvalidSignature(format!("URL解析出错：{}", err.to_string()))
//     }
// }
