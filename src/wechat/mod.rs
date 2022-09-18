use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use serde_json::Value;

mod mp;
mod cp;
mod pay;
mod cryptos;
mod miniapp;
#[allow(unused)]
mod constants;

pub use cp::*;
pub use mp::*;
pub use pay::*;
pub use cryptos::*;
use crate::{LabradorResult, LabraError, Method, RequestBody, RequestType};

pub trait ReplyRenderer {
    fn render(&self) -> String;
}

pub trait XmlMessageParser {
    type WechatXmlMessage;

    fn from_xml(xml: &str) -> LabradorResult<Self::WechatXmlMessage>;
}

impl <T> XmlMessageParser for T where T: DeserializeOwned {
    type WechatXmlMessage = T;

    fn from_xml(xml: &str) -> LabradorResult<Self::WechatXmlMessage> {
        serde_xml_rs::from_str(xml).map_err(LabraError::from)
    }
}

pub trait WechatRequest {
    ///
    /// 获取TOP的API名称。
    ///
    /// @return API名称
    fn get_api_method_name(&self) -> String;

    /// 获取请求类型。
    fn get_request_type(&self) -> RequestType {
        RequestType::Json
    }

    /// 获取请求方法。
    fn get_request_method(&self) -> Method {
        Method::Post
    }

    fn get_query_params(&self) -> BTreeMap<String, String> {
        BTreeMap::new()
    }

    fn get_request_body<T: Serialize>(&self) -> RequestBody<T> {
        RequestBody::Null
    }

    /// 是否需要token
    fn is_need_token(&self) -> bool {
        true
    }

}

#[allow(unused)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCommonResponse {
    pub errcode: Option<i64>,
    pub errmsg: Option<String>,
    pub body: Option<String>,
}

impl WechatCommonResponse {
    pub fn is_success(&self) -> bool {
        self.errcode.unwrap_or(0) == 0
    }

    pub fn parse<T: DeserializeOwned>(v: Value) -> LabradorResult<T> {
        let resp = serde_json::from_value::<Self>(v.to_owned())?;
        if resp.is_success() {
            serde_json::from_str::<T>(&v.to_string()).map_err(LabraError::from)
        } else {
            Err(LabraError::ClientError { errcode: resp.errcode.to_owned().unwrap_or_default().to_string(), errmsg: resp.errmsg.to_owned().unwrap_or_default() })
        }
    }

    pub fn parse_with_key<T: DeserializeOwned>(v: Value, key: &str) -> LabradorResult<T> {
        let resp = serde_json::from_value::<Self>(v.to_owned())?;
        if resp.is_success() {
            let v = serde_json::from_str::<Value>(&v.to_string())?;
            let result = &v[key];
            if result.is_string() {
                serde_json::from_str::<T>(result.as_str().unwrap_or_default()).map_err(LabraError::from)
            } else {
                serde_json::from_value::<T>(v[key].to_owned()).map_err(LabraError::from)
            }
        } else {
            Err(LabraError::ClientError { errcode: resp.errcode.to_owned().unwrap_or_default().to_string(), errmsg: resp.errmsg.to_owned().unwrap_or_default() })
        }
    }

    pub fn from_value(v: Value) -> LabradorResult<Self> {
        let mut resp = serde_json::from_value::<Self>(v.to_owned())?;
        resp.body = v.to_string().into();
        Ok(resp)
    }

    pub fn from_str(str: &str) -> LabradorResult<Self> {
        let mut resp = serde_json::from_str::<Self>(str)?;
        resp.body = str.to_string().into();
        Ok(resp)
    }

    pub fn get_biz_model<T: DeserializeOwned>(&self, key: Option<&str>) -> LabradorResult<T> {
        if self.is_success() {
            if let Some(key) = key {
                let v = serde_json::from_str::<Value>(&self.body.to_owned().unwrap_or_default())?;
                let result = &v[key];
                if result.is_string() {
                    serde_json::from_str::<T>(result.as_str().unwrap_or_default()).map_err(LabraError::from)
                } else {
                    serde_json::from_value::<T>(v[key].to_owned()).map_err(LabraError::from)
                }
            } else {
                serde_json::from_str::<T>(&self.body.to_owned().unwrap_or_default()).map_err(LabraError::from)
            }
        } else {
            Err(LabraError::ClientError { errcode: self.errcode.to_owned().unwrap_or_default().to_string(), errmsg: self.errmsg.to_owned().unwrap_or_default() })
        }
    }
}