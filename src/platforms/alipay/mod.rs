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

//! 支付宝平台实现

use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use crate::alipay::method::AlipayMethod;
use crate::errors::{LabraError, LabradorResult};

/// 支付宝客户端
pub mod client;
/// 支付宝支付
pub mod pay;
/// 支付宝开放平台
pub mod open;
/// 支付宝小程序
pub mod miniapp;
#[allow(unused)]
pub mod constants;
pub mod builder;
pub mod config;
pub mod method;
pub mod types;

/// 支付宝错误码
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlipayErrorCode {
    /// 成功
    Success = 10000,
    /// 服务不可用
    ServiceUnavailable = 20000,
    /// 授权权限不足
    InsufficientPermissions = 20001,
    /// 缺少必选参数
    MissingParameter = 40001,
    /// 非法的参数
    InvalidParameter = 40002,
    /// 业务处理失败
    BusinessFailed = 40004,
    /// 权限不足
    PermissionDenied = 40006,
    /// 系统繁忙
    SystemBusy = 50000,
    /// 用户支付中
    UserPaying = 10003,
    /// 交易不存在
    TradeNotExist = 40003,
    /// 订单已关闭
    TradeClosed = 10001,
    /// 退款金额超限
    RefundAmountExceed = 40005,
    /// 重复退款
    DuplicateRefund = 40008,
    /// 余额不足
    InsufficientBalance = 40007,
}

impl From<&str> for AlipayErrorCode {
    fn from(code: &str) -> Self {
        match code {
            "10000" => Self::Success,
            "20000" => Self::ServiceUnavailable,
            "20001" => Self::InsufficientPermissions,
            "40001" => Self::MissingParameter,
            "40002" => Self::InvalidParameter,
            "40004" => Self::BusinessFailed,
            "40006" => Self::PermissionDenied,
            "50000" => Self::SystemBusy,
            "10003" => Self::UserPaying,
            _ => Self::BusinessFailed,
        }
    }
}




#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayBizRequest<T: Serialize> {
    /// API版本
    pub api_version: String,
    /// 回调地址
    pub notify_url: Option<String>,
    /// 跳转地址
    pub return_url: Option<String>,
    /// 终端类型
    pub terminal_type: Option<String>,
    /// 终端信息
    pub terminal_info: Option<String>,
    /// 产品编码
    pub prod_code: Option<String>,
    /// 是否需要加密
    pub need_encrypt: bool,
    /// 参数
    pub udf_params: BTreeMap<String, String>,
    /// 业务实体
    pub biz_model: Option<T>,
    #[serde(skip)]
    pub method: AlipayMethod,
}

impl <T: Serialize> AlipayBizRequest<T> {
    pub fn new() -> Self {
        Self {
            api_version: "1.0".to_string(),
            notify_url: None,
            return_url: None,
            terminal_type: None,
            terminal_info: None,
            prod_code: None,
            need_encrypt: false,
            udf_params: BTreeMap::new(),
            biz_model: None,
            method: Default::default(),
        }
    }

    pub fn set_biz_model(&mut self, biz_model: T) {
        self.biz_model = Some(biz_model);
    }

    pub fn set_method(&mut self, method: AlipayMethod) {
        self.method = method;
    }

    pub fn put_other_text_param(&mut self, key: String, value: String) {
        self.udf_params.insert(key, value);
    }
}

impl<T: Serialize> From<T> for AlipayBizRequest<T> {
    fn from(value: T) -> Self {
        let mut req = AlipayBizRequest::new();
        req.set_biz_model(value);
        req
    }
}

impl<T: Serialize + Clone> From<&T> for AlipayBizRequest<T> {
    fn from(value: &T) -> Self {
        let mut req = AlipayBizRequest::new();
        req.set_biz_model(value.clone());
        req
    }
}


impl <T> AlipayRequest<T> for AlipayBizRequest<T> where T: Serialize {
    fn get_api_method(&self) -> &AlipayMethod {
        &self.method
    }

    fn get_text_params(&self) -> BTreeMap<String, String> {
        let mut txt_params = BTreeMap::new();
        txt_params.insert(constants::BIZ_CONTENT_KEY.to_string(), serde_json::to_string(&self.get_biz_model()).unwrap_or_default());
        if !self.udf_params.is_empty() {
            for (k, v) in &self.udf_params {
                txt_params.insert(k.to_string(), v.to_string());
            }
        }
        txt_params
    }

    fn get_api_version(&self) -> String {
        if self.api_version.is_empty() {
            "1.0".to_string()
        } else {
            self.api_version.to_string()
        }
    }

    fn get_terminal_type(&self) -> String {
        self.terminal_type.to_owned().unwrap_or_default()
    }

    fn get_terminal_info(&self) -> String {
        self.terminal_info.to_owned().unwrap_or_default()
    }

    fn get_prod_code(&self) -> String {
        self.prod_code.to_owned().unwrap_or_default()
    }

    fn get_notify_url(&self) -> String {
        self.notify_url.to_owned().unwrap_or_default()
    }

    fn get_return_url(&self) -> String {
        self.return_url.to_owned().unwrap_or_default()
    }

    fn is_need_encrypt(&self) -> bool {
        self.need_encrypt
    }

    fn get_biz_model(&self) -> Option<&T> {
        self.biz_model.as_ref()
    }
}



/// 支付宝响应
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AlipayResponse<T = serde_json::Value> {
    /// 响应码
    pub code: String,
    /// 响应消息
    pub msg: String,
    /// 子响应码
    pub sub_code: Option<String>,
    /// 子响应消息
    pub sub_msg: Option<String>,
    /// 响应数据
    #[serde(flatten)]
    pub data: Option<T>,
    /// 签名
    pub sign: Option<String>,
}

impl<T> AlipayResponse<T> {
    /// 检查是否成功
    pub fn is_success(&self) -> bool {
        self.code == "10000" && self.sub_code.is_none()
    }

    /// 转换为Result
    pub fn into_result(self) -> LabradorResult<T> {
        if self.is_success() {
            self.data.ok_or_else(|| LabraError::Other("No data in response".to_string()))
        } else {
            let errmsg = self.sub_msg.unwrap_or(self.msg.clone());
            Err(LabraError::business(
                self.sub_code.unwrap_or(self.code.clone()),
                errmsg,
            ))
        }
    }
}


pub trait AlipayRequest<T: Serialize> {
    ///
    /// 获取TOP的API名称。
    ///
    /// @return API名称
    fn get_api_method(&self) -> &AlipayMethod;

    ///
    /// 获取所有的Key-Value形式的文本请求参数集合。其中：
    /// <ul>
    /// <li>Key: 请求参数名</li>
    /// <li>Value: 请求参数值</li>
    /// </ul>
    ///
    /// @return 文本请求参数集合
    fn get_text_params(&self) -> BTreeMap<String, String> {
        let mut txt_params = BTreeMap::new();
        let mut value = serde_json::to_value(self.get_biz_model()).unwrap_or_default();

        if let serde_json::Value::Object(map) = &mut value {
            map.remove("method");
        }

        txt_params.insert(
            constants::BIZ_CONTENT_KEY.to_string(),
            serde_json::to_string(&value).unwrap_or_default()
        );
        txt_params
    }

    ///
    /// 得到当前接口的版本
    ///
    /// @return API版本
    fn get_api_version(&self) -> String {
        "1.0".to_string()
    }


    ///
    /// 获取终端类型
    ///
    /// @return 终端类型
    fn get_terminal_type(&self) -> String {
        "".to_string()
    }

    ///
    /// 获取终端信息
    ///
    /// @return 终端信息
    fn get_terminal_info(&self) -> String {
        "".to_string()
    }


    ///
    /// 获取产品码
    ///
    /// @return 产品码
    fn get_prod_code(&self) -> String {
        "".to_string()
    }

    ///
    /// 返回通知地址
    ///
    /// @return
    fn get_notify_url(&self) -> String {
        "".to_string()
    }

    ///
    /// 返回回跳地址
    ///
    /// @return
    fn get_return_url(&self) -> String {
        "".to_string()
    }

    ///
    /// 判断是否需要加密
    ///
    /// @return
    fn is_need_encrypt(&self) -> bool {
        false
    }

    fn get_biz_model(&self) -> Option<&T> {
        None
    }
}

#[test]
fn test_alipay_request() {
    let req = serde_json::from_str::<AlipayResponse>("{\"code\":\"10000\",\"msg\":\"Success\",\"out_trade_no\":\"20201212121212\",\"retry_flag\":\"N\"}").unwrap();
    println!("data:{:?}", req.data);
}