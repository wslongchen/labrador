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
use crate::errors::{LabraError, LabradorResult};
use crate::platforms::wechat::pay::WechatPayConfig;
use crate::utils::string::random_string;
use crate::utils::time::timestamp_millis;
use crate::utils::xml::{XmlMap, XmlSerializer};
use crate::{CryptoUtils, HashType, RsaEncryptor, RsaKeyFormat};
use base64::engine::general_purpose;
use base64::Engine;
use chrono::{DateTime, Duration, Utc};
use http::HeaderMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

/// 交易类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeType {
    /// 付款码支付
    Micropay,
    /// 公众号支付
    Jsapi,
    /// 扫码支付
    Native,
    /// APP支付
    App,
    /// H5支付
    H5,
    /// 小程序支付
    Miniapp,
}

impl TradeType {
    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            TradeType::Micropay => "MICROPAY",
            TradeType::Jsapi => "JSAPI",
            TradeType::Native => "NATIVE",
            TradeType::App => "APP",
            TradeType::H5 => "MWEB",
            TradeType::Miniapp => "JSAPI",
        }
    }

    pub fn req_path(&self) -> &'static str {
        match self {
            TradeType::Micropay => "pay/micropay",
            _ => "pay/unifiedorder",
        }
    }

    pub fn req_path_v3(&self) -> &'static str {
        match self {
            TradeType::H5 => "/v3/pay/transactions/h5",
            TradeType::Jsapi => "/v3/pay/transactions/jsapi",
            TradeType::Native => "/v3/pay/transactions/native",
            TradeType::App => "/v3/pay/transactions/app",
            TradeType::Micropay => "/v3/pay/transactions/codepay",
            TradeType::Miniapp => "/v3/pay/transactions/jsapi",
        }
    }

    pub fn isv_req_path_v3(&self) -> &'static str {
        match self {
            TradeType::H5 => "/v3/pay/partner/transactions/h5",
            TradeType::Jsapi => "/v3/pay/partner/transactions/jsapi",
            TradeType::Native => "/v3/pay/partner/transactions/native",
            TradeType::App => "/v3/pay/partner/transactions/app",
            TradeType::Micropay => "/v3/pay/partner/transactions/codepay",
            TradeType::Miniapp => "/v3/pay/partner/transactions/jsapi",
        }
    }
}

/// 货币类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeeType {
    /// 人民币
    CNY,
}

impl FeeType {
    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            FeeType::CNY => "CNY",
        }
    }
}

/// 统一下单请求
#[derive(Debug, Clone, Serialize)]
pub struct UnifiedOrderRequest {
    /// 商品描述
    pub body: String,
    /// 商户订单号
    pub out_trade_no: String,
    /// 总金额（单位：分）
    pub total_fee: i32,
    /// 终端IP
    pub spbill_create_ip: String,
    /// 通知地址
    pub notify_url: String,
    /// 交易类型
    pub trade_type: TradeType,
    /// 设备信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_info: Option<String>,
    /// 商品ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// 无
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,
    /// 商品详情（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// 附加数据（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
    /// 货币类型（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_type: Option<FeeType>,
    /// 交易开始时间（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_start: Option<DateTime<Utc>>,
    /// 交易结束时间（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_expire: Option<DateTime<Utc>>,
    /// 商品标记（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_tag: Option<String>,
    /// 指定支付方式（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_pay: Option<String>,
    /// 场景信息（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_info: Option<String>,
    /// OpenID（JSAPI支付必填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openid: Option<String>,
}

impl UnifiedOrderRequest {
    /// 创建新的统一下单请求
    pub fn new(
        body: &str,
        out_trade_no: &str,
        total_fee: i32,
        spbill_create_ip: &str,
        notify_url: &str,
        trade_type: TradeType,
    ) -> Self {
        Self {
            body: body.to_string(),
            out_trade_no: out_trade_no.to_string(),
            total_fee,
            spbill_create_ip: spbill_create_ip.to_string(),
            notify_url: notify_url.to_string(),
            trade_type,
            device_info: None,
            product_id: None,
            auth_code: None,
            detail: None,
            attach: None,
            fee_type: None,
            time_start: None,
            time_expire: None,
            goods_tag: None,
            limit_pay: None,
            scene_info: None,
            openid: None,
        }
    }

    /// 设置商品详情
    pub fn with_detail(mut self, detail: &str) -> Self {
        self.detail = Some(detail.to_string());
        self
    }

    /// 添加商品ID
    pub fn with_product_id(mut self, product_id: &str) -> Self {
        self.product_id = Some(product_id.to_string());
        self
    }

    /// 添加授权码
    pub fn with_auth_code(mut self, auth_code: &str) -> Self {
        self.auth_code = Some(auth_code.to_string());
        self
    }

    /// 添加设备信息
    pub fn with_device_info(mut self, device_info: &str) -> Self {
        self.device_info = Some(device_info.to_string());
        self
    }

    /// 设置附加数据
    pub fn with_attach(mut self, attach: &str) -> Self {
        self.attach = Some(attach.to_string());
        self
    }

    /// 设置货币类型
    pub fn with_fee_type(mut self, fee_type: FeeType) -> Self {
        self.fee_type = Some(fee_type);
        self
    }

    /// 设置交易开始时间
    pub fn with_time_start(mut self, time_start: DateTime<Utc>) -> Self {
        self.time_start = Some(time_start);
        self
    }

    /// 设置交易结束时间
    pub fn with_time_expire(mut self, time_expire: DateTime<Utc>) -> Self {
        self.time_expire = Some(time_expire);
        self
    }

    /// 设置商品标记
    pub fn with_goods_tag(mut self, goods_tag: &str) -> Self {
        self.goods_tag = Some(goods_tag.to_string());
        self
    }

    /// 设置指定支付方式
    pub fn with_limit_pay(mut self, limit_pay: &str) -> Self {
        self.limit_pay = Some(limit_pay.to_string());
        self
    }

    /// 设置场景信息
    pub fn with_scene_info(mut self, scene_info: &str) -> Self {
        self.scene_info = Some(scene_info.to_string());
        self
    }

    /// 设置OpenID
    pub fn with_openid(mut self, openid: &str) -> Self {
        self.openid = Some(openid.to_string());
        self
    }

    /// 转换为XML字符串
    pub fn to_xml(&self, config: &WechatPayConfig) -> LabradorResult<String> {
        let mut params = BTreeMap::new();

        // 添加公共参数
        params.insert("appid".to_string(), config.app_id.clone());
        params.insert("mch_id".to_string(), config.mch_id.clone());
        params.insert("nonce_str".to_string(), random_string(32));

        // 添加业务参数
        params.insert("body".to_string(), self.body.clone());
        params.insert("out_trade_no".to_string(), self.out_trade_no.clone());
        params.insert("total_fee".to_string(), self.total_fee.to_string());
        params.insert(
            "spbill_create_ip".to_string(),
            self.spbill_create_ip.clone(),
        );
        params.insert("notify_url".to_string(), self.notify_url.clone());
        params.insert(
            "trade_type".to_string(),
            self.trade_type.as_str().to_string(),
        );

        if let Some(ref detail) = self.detail {
            params.insert("detail".to_string(), detail.clone());
        }
        if let Some(ref attach) = self.attach {
            params.insert("attach".to_string(), attach.clone());
        }
        if let Some(ref fee_type) = self.fee_type {
            params.insert("fee_type".to_string(), fee_type.as_str().to_string());
        }
        if let Some(ref time_start) = self.time_start {
            params.insert(
                "time_start".to_string(),
                time_start.format("%Y%m%d%H%M%S").to_string(),
            );
        }
        if let Some(ref time_expire) = self.time_expire {
            params.insert(
                "time_expire".to_string(),
                time_expire.format("%Y%m%d%H%M%S").to_string(),
            );
        }
        if let Some(ref goods_tag) = self.goods_tag {
            params.insert("goods_tag".to_string(), goods_tag.clone());
        }
        if let Some(ref limit_pay) = self.limit_pay {
            params.insert("limit_pay".to_string(), limit_pay.clone());
        }
        if let Some(ref scene_info) = self.scene_info {
            params.insert("scene_info".to_string(), scene_info.clone());
        }
        if let Some(ref openid) = self.openid {
            params.insert("openid".to_string(), openid.clone());
        }

        // 生成签名
        let sign = Self::generate_sign(&params, &config.api_key.to_owned().unwrap_or_default())?;
        params.insert("sign".to_string(), sign);

        // 转换为XML
        Self::map_to_xml(&params)
    }

    /// 生成签名
    pub fn generate_sign(
        params: &BTreeMap<String, String>,
        api_key: &str,
    ) -> LabradorResult<String> {
        // 构造签名字符串
        let mut sign_string = String::new();
        for (key, value) in params {
            if !value.is_empty() && key != "sign" {
                sign_string.push_str(&format!("{}={}&", key, value));
            }
        }
        sign_string.push_str(&format!("key={}", api_key));

        // MD5签名并转为大写
        let sign = CryptoUtils::md5(sign_string.as_bytes());
        Ok(sign.to_uppercase())
    }

    /// 将Map转换为XML
    pub fn map_to_xml(params: &BTreeMap<String, String>) -> LabradorResult<String> {
        // 需要增加xml根节点
        XmlSerializer::serialize(&XmlMap(params))
    }
}

/// 统一下单响应
#[derive(Debug, Clone, Deserialize)]
pub struct UnifiedOrderResponse {
    /// 返回状态码
    pub return_code: String,
    /// 返回信息
    pub return_msg: String,
    /// 应用ID
    #[serde(rename = "appid")]
    pub app_id: Option<String>,
    /// 商户号
    pub mch_id: Option<String>,
    /// 设备号
    pub device_info: Option<String>,
    /// 随机字符串
    pub nonce_str: Option<String>,
    /// 签名
    pub sign: Option<String>,
    /// 业务结果
    pub result_code: Option<String>,
    /// 错误代码
    pub err_code: Option<String>,
    /// 错误代码描述
    pub err_code_des: Option<String>,
    /// 交易类型
    pub trade_type: Option<String>,
    /// 预支付交易会话标识
    pub prepay_id: Option<String>,
    /// 二维码链接
    pub code_url: Option<String>,
    /// 商户订单号
    pub out_trade_no: Option<String>,
    /// 微信支付订单号
    pub transaction_id: Option<String>,
}

impl UnifiedOrderResponse {
    /// 从XML解析
    pub fn from_xml(xml: &str) -> LabradorResult<Self> {
        let mut response: Self = XmlSerializer::deserialize(xml)?;
        if response.err_code.is_none() {
            response.err_code = Some(response.return_code.to_string());
        }
        if response.err_code_des.is_none() {
            response.err_code_des = Some(response.return_msg.to_string());
        }
        Ok(response)
    }

    /// 检查是否成功
    pub fn is_success(&self) -> bool {
        self.return_code == "SUCCESS" && self.result_code.as_deref() == Some("SUCCESS")
    }

    /// 验证签名
    pub fn verify_signature(&self, api_key: &str) -> LabradorResult<bool> {
        let mut params = BTreeMap::new();

        // 添加所有字段到Map
        if let Some(ref app_id) = self.app_id {
            params.insert("appid".to_string(), app_id.clone());
        }
        if let Some(ref mch_id) = self.mch_id {
            params.insert("mch_id".to_string(), mch_id.clone());
        }
        if let Some(ref device_info) = self.device_info {
            params.insert("device_info".to_string(), device_info.clone());
        }
        if let Some(ref nonce_str) = self.nonce_str {
            params.insert("nonce_str".to_string(), nonce_str.clone());
        }
        params.insert("return_code".to_string(), self.return_code.clone());
        params.insert("return_msg".to_string(), self.return_msg.clone());
        if let Some(ref result_code) = self.result_code {
            params.insert("result_code".to_string(), result_code.clone());
        }
        if let Some(ref err_code) = self.err_code {
            params.insert("err_code".to_string(), err_code.clone());
        }
        if let Some(ref err_code_des) = self.err_code_des {
            params.insert("err_code_des".to_string(), err_code_des.clone());
        }
        if let Some(ref trade_type) = self.trade_type {
            params.insert("trade_type".to_string(), trade_type.clone());
        }
        if let Some(ref prepay_id) = self.prepay_id {
            params.insert("prepay_id".to_string(), prepay_id.clone());
        }
        if let Some(ref code_url) = self.code_url {
            params.insert("code_url".to_string(), code_url.clone());
        }
        if let Some(ref out_trade_no) = self.out_trade_no {
            params.insert("out_trade_no".to_string(), out_trade_no.clone());
        }
        if let Some(ref transaction_id) = self.transaction_id {
            params.insert("transaction_id".to_string(), transaction_id.clone());
        }

        // 生成签名
        let calculated_sign = UnifiedOrderRequest::generate_sign(&params, api_key)?;

        if let Some(ref sign) = self.sign {
            Ok(calculated_sign == *sign)
        } else {
            Ok(false)
        }
    }

    /// 获取JSAPI支付参数
    pub fn get_jsapi_params(
        &self,
        app_id: &str,
        api_key: &str,
    ) -> LabradorResult<JsapiPaymentParams> {
        if self.trade_type.as_deref() != Some("JSAPI") {
            return Err(LabraError::Validation("不是JSAPI支付类型".to_string()));
        }

        let prepay_id = self
            .prepay_id
            .as_ref()
            .ok_or_else(|| LabraError::Validation("缺少prepay_id".to_string()))?;

        JsapiPaymentParams::new(app_id, prepay_id, api_key)
    }
}

/// JSAPI支付参数
#[derive(Debug, Clone, Serialize)]
pub struct JsapiPaymentParams {
    /// 应用ID
    pub app_id: String,
    /// 时间戳
    pub timestamp: String,
    /// 随机字符串
    pub nonce_str: String,
    /// 订单详情扩展字符串
    pub package: String,
    /// 签名类型
    pub sign_type: String,
    /// 签名
    pub pay_sign: String,
}

impl JsapiPaymentParams {
    /// 创建新的JSAPI支付参数
    pub fn new(app_id: &str, prepay_id: &str, api_key: &str) -> LabradorResult<Self> {
        use rand::Rng;

        // 生成随机字符串和时间戳
        let mut rng = rand::thread_rng();
        let nonce_str: String = (0..32)
            .map(|_| {
                let idx = rng.gen_range(0..crate::utils::string::CHARSET.len());
                crate::utils::string::CHARSET[idx] as char
            })
            .collect();

        let timestamp = chrono::Utc::now().timestamp().to_string();
        let package = format!("prepay_id={}", prepay_id);
        let sign_type = "MD5".to_string();

        // 构造签名字符串
        let mut params = BTreeMap::new();
        params.insert("appId".to_string(), app_id.to_string());
        params.insert("timeStamp".to_string(), timestamp.clone());
        params.insert("nonceStr".to_string(), nonce_str.clone());
        params.insert("package".to_string(), package.clone());
        params.insert("signType".to_string(), sign_type.clone());

        let mut sign_string = String::new();
        for (key, value) in &params {
            sign_string.push_str(&format!("{}={}&", key, value));
        }
        sign_string.push_str(&format!("key={}", api_key));

        // 生成签名
        let pay_sign = CryptoUtils::md5(sign_string.as_bytes()).to_uppercase();

        Ok(Self {
            app_id: app_id.to_string(),
            timestamp,
            nonce_str,
            package,
            sign_type,
            pay_sign,
        })
    }

    /// 转换为JSON字符串
    pub fn to_json(&self) -> LabradorResult<String> {
        serde_json::to_string(self).map_err(LabraError::Json)
    }
}

/// 查询订单请求
#[derive(Debug, Clone, Serialize)]
pub struct OrderQueryRequest {
    /// 商户订单号（二选一）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_trade_no: Option<String>,
    /// 微信支付订单号（二选一）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

impl OrderQueryRequest {
    /// 通过商户订单号查询
    pub fn by_out_trade_no(out_trade_no: &str) -> Self {
        Self {
            out_trade_no: Some(out_trade_no.to_string()),
            transaction_id: None,
        }
    }

    /// 通过微信订单号查询
    pub fn by_transaction_id(transaction_id: &str) -> Self {
        Self {
            out_trade_no: None,
            transaction_id: Some(transaction_id.to_string()),
        }
    }

    /// 转换为XML字符串
    pub fn to_xml(&self, config: &WechatPayConfig) -> LabradorResult<String> {
        let mut params = BTreeMap::new();

        // 添加公共参数
        params.insert("appid".to_string(), config.app_id.clone());
        params.insert("mch_id".to_string(), config.mch_id.clone());
        params.insert("nonce_str".to_string(), random_string(32));

        // 添加业务参数
        if let Some(ref out_trade_no) = self.out_trade_no {
            params.insert("out_trade_no".to_string(), out_trade_no.clone());
        }
        if let Some(ref transaction_id) = self.transaction_id {
            params.insert("transaction_id".to_string(), transaction_id.clone());
        }

        // 生成签名
        let sign = UnifiedOrderRequest::generate_sign(
            &params,
            &config.api_key.to_owned().unwrap_or_default(),
        )?;
        params.insert("sign".to_string(), sign);

        // 转换为XML
        UnifiedOrderRequest::map_to_xml(&params)
    }
}

/// 查询订单响应
#[derive(Debug, Clone, Deserialize)]
pub struct OrderQueryResponse {
    /// 返回状态码
    pub return_code: String,
    /// 返回信息
    pub return_msg: String,
    /// 应用ID
    #[serde(rename = "appid")]
    pub app_id: Option<String>,
    /// 商户号
    pub mch_id: Option<String>,
    /// 随机字符串
    pub nonce_str: Option<String>,
    /// 签名
    pub sign: Option<String>,
    /// 业务结果
    pub result_code: Option<String>,
    /// 错误代码
    pub err_code: Option<String>,
    /// 错误代码描述
    pub err_code_des: Option<String>,
    /// 设备号
    pub device_info: Option<String>,
    /// 用户标识
    pub openid: Option<String>,
    /// 是否关注公众账号
    pub is_subscribe: Option<String>,
    /// 交易类型
    pub trade_type: Option<String>,
    /// 交易状态
    pub trade_state: Option<String>,
    /// 付款银行
    pub bank_type: Option<String>,
    /// 订单金额
    pub total_fee: Option<i32>,
    /// 应结订单金额
    pub settlement_total_fee: Option<i32>,
    /// 货币种类
    pub fee_type: Option<String>,
    /// 现金支付金额
    pub cash_fee: Option<i32>,
    /// 现金支付货币类型
    pub cash_fee_type: Option<String>,
    /// 代金券金额
    pub coupon_fee: Option<i32>,
    /// 代金券使用数量
    pub coupon_count: Option<i32>,
    /// 微信支付订单号
    pub transaction_id: Option<String>,
    /// 商户订单号
    pub out_trade_no: Option<String>,
    /// 附加数据
    pub attach: Option<String>,
    /// 支付完成时间
    pub time_end: Option<String>,
    /// 交易状态描述
    pub trade_state_desc: Option<String>,
}

impl OrderQueryResponse {
    /// 从XML解析
    pub fn from_xml(xml: &str) -> LabradorResult<Self> {
        let mut response: Self = XmlSerializer::deserialize(xml)?;
        if response.err_code.is_none() {
            response.err_code = Some(response.return_code.to_string());
        }
        if response.err_code_des.is_none() {
            response.err_code_des = Some(response.return_msg.to_string());
        }
        Ok(response)
    }

    /// 检查是否成功
    pub fn is_success(&self) -> bool {
        self.return_code == "SUCCESS" && self.result_code.as_deref() == Some("SUCCESS")
    }

    /// 检查交易状态
    pub fn is_trade_success(&self) -> bool {
        self.trade_state.as_deref() == Some("SUCCESS")
    }

    /// 验证签名
    pub fn verify_signature(&self, api_key: &str) -> LabradorResult<bool> {
        let mut params = BTreeMap::new();

        // 添加所有字段到Map
        if let Some(ref app_id) = self.app_id {
            params.insert("appid".to_string(), app_id.clone());
        }
        if let Some(ref mch_id) = self.mch_id {
            params.insert("mch_id".to_string(), mch_id.clone());
        }
        if let Some(ref nonce_str) = self.nonce_str {
            params.insert("nonce_str".to_string(), nonce_str.clone());
        }
        params.insert("return_code".to_string(), self.return_code.clone());
        params.insert("return_msg".to_string(), self.return_msg.clone());
        if let Some(ref result_code) = self.result_code {
            params.insert("result_code".to_string(), result_code.clone());
        }
        // if let Some(ref err_code) = self.err_code {
        //     params.insert("err_code".to_string(), err_code.clone());
        // }
        // if let Some(ref err_code_des) = self.err_code_des {
        //     params.insert("err_code_des".to_string(), err_code_des.clone());
        // }
        if let Some(ref device_info) = self.device_info {
            params.insert("device_info".to_string(), device_info.clone());
        }
        if let Some(ref openid) = self.openid {
            params.insert("openid".to_string(), openid.clone());
        }
        if let Some(ref is_subscribe) = self.is_subscribe {
            params.insert("is_subscribe".to_string(), is_subscribe.clone());
        }
        if let Some(ref trade_type) = self.trade_type {
            params.insert("trade_type".to_string(), trade_type.clone());
        }
        if let Some(ref trade_state) = self.trade_state {
            params.insert("trade_state".to_string(), trade_state.clone());
        }
        if let Some(ref bank_type) = self.bank_type {
            params.insert("bank_type".to_string(), bank_type.clone());
        }
        if let Some(ref total_fee) = self.total_fee {
            params.insert("total_fee".to_string(), total_fee.to_string());
        }
        if let Some(ref settlement_total_fee) = self.settlement_total_fee {
            params.insert(
                "settlement_total_fee".to_string(),
                settlement_total_fee.to_string(),
            );
        }
        if let Some(ref fee_type) = self.fee_type {
            params.insert("fee_type".to_string(), fee_type.clone());
        }
        if let Some(ref cash_fee) = self.cash_fee {
            params.insert("cash_fee".to_string(), cash_fee.to_string());
        }
        if let Some(ref cash_fee_type) = self.cash_fee_type {
            params.insert("cash_fee_type".to_string(), cash_fee_type.clone());
        }
        if let Some(ref coupon_fee) = self.coupon_fee {
            params.insert("coupon_fee".to_string(), coupon_fee.to_string());
        }
        if let Some(ref coupon_count) = self.coupon_count {
            params.insert("coupon_count".to_string(), coupon_count.to_string());
        }
        if let Some(ref transaction_id) = self.transaction_id {
            params.insert("transaction_id".to_string(), transaction_id.clone());
        }
        if let Some(ref out_trade_no) = self.out_trade_no {
            params.insert("out_trade_no".to_string(), out_trade_no.clone());
        }
        if let Some(ref attach) = self.attach {
            params.insert("attach".to_string(), attach.clone());
        }
        if let Some(ref time_end) = self.time_end {
            params.insert("time_end".to_string(), time_end.clone());
        }
        if let Some(ref trade_state_desc) = self.trade_state_desc {
            params.insert("trade_state_desc".to_string(), trade_state_desc.clone());
        }

        // 生成签名
        let calculated_sign = UnifiedOrderRequest::generate_sign(&params, api_key)?;

        if let Some(ref sign) = self.sign {
            Ok(calculated_sign == *sign)
        } else {
            Ok(false)
        }
    }
}

/// 退款请求
#[derive(Debug, Clone, Deserialize)]
pub struct RefundRequest {
    pub out_trade_no: String,
    pub out_refund_no: String,
    pub total_fee: i32,
    pub refund_fee: i32,
    pub refund_desc: Option<String>,
}
impl RefundRequest {
    /// 创建一个退款请求
    pub fn new(out_trade_no: &str, out_refund_no: &str, total_fee: i32, refund_fee: i32) -> Self {
        Self {
            out_trade_no: out_trade_no.to_string(),
            out_refund_no: out_refund_no.to_string(),
            total_fee,
            refund_fee,
            refund_desc: None,
        }
    }

    /// 设置退款描述
    pub fn refund_desc(mut self, refund_desc: &str) -> Self {
        self.refund_desc = Some(refund_desc.to_string());
        self
    }
}

/// 退款响应
#[derive(Debug, Clone, Deserialize)]
pub struct RefundResponse {
    /// 返回状态码
    pub return_code: String,
    /// 返回信息
    pub return_msg: String,
    /// 业务结果
    pub result_code: Option<String>,
    /// 错误代码
    pub err_code: Option<String>,
    /// 错误代码描述
    pub err_code_des: Option<String>,
    /// 应用ID
    #[serde(rename = "appid")]
    pub app_id: Option<String>,
    /// 商户号
    pub mch_id: Option<String>,
    /// 随机字符串
    pub nonce_str: Option<String>,
    /// 签名
    pub sign: Option<String>,
    /// 微信支付订单号
    pub transaction_id: Option<String>,
    /// 商户订单号
    pub out_trade_no: Option<String>,
    /// 商户退款单号
    pub out_refund_no: Option<String>,
    /// 微信退款单号
    pub refund_id: Option<String>,
    /// 退款金额
    pub refund_fee: Option<i32>,
    /// 应结退款金额
    pub settlement_refund_fee: Option<i32>,
    /// 订单金额
    pub total_fee: Option<i32>,
    /// 应结订单金额
    pub settlement_total_fee: Option<i32>,
    /// 货币种类
    pub fee_type: Option<String>,
    /// 现金支付金额
    pub cash_fee: Option<i32>,
    /// 现金退款金额
    pub cash_refund_fee: Option<i32>,
}

impl RefundResponse {
    /// 从XML解析
    pub fn from_xml(xml: &str) -> LabradorResult<Self> {
        let mut response: Self = XmlSerializer::deserialize(xml)?;
        if response.err_code.is_none() {
            response.err_code = Some(response.return_code.to_string());
        }
        if response.err_code_des.is_none() {
            response.err_code_des = Some(response.return_msg.to_string());
        }
        Ok(response)
    }

    /// 检查是否成功
    pub fn is_success(&self) -> bool {
        self.return_code == "SUCCESS" && self.result_code.as_deref() == Some("SUCCESS")
    }

    /// 验证签名
    pub fn verify_signature(&self, api_key: &str) -> LabradorResult<bool> {
        let mut params = BTreeMap::new();

        // 添加所有字段到Map
        if let Some(ref app_id) = self.app_id {
            params.insert("appid".to_string(), app_id.clone());
        }
        if let Some(ref mch_id) = self.mch_id {
            params.insert("mch_id".to_string(), mch_id.clone());
        }
        if let Some(ref nonce_str) = self.nonce_str {
            params.insert("nonce_str".to_string(), nonce_str.clone());
        }
        params.insert("return_code".to_string(), self.return_code.clone());
        params.insert("return_msg".to_string(), self.return_msg.clone());
        if let Some(ref result_code) = self.result_code {
            params.insert("result_code".to_string(), result_code.clone());
        }
        if let Some(ref err_code) = self.err_code {
            params.insert("err_code".to_string(), err_code.clone());
        }
        if let Some(ref err_code_des) = self.err_code_des {
            params.insert("err_code_des".to_string(), err_code_des.clone());
        }
        if let Some(ref transaction_id) = self.transaction_id {
            params.insert("transaction_id".to_string(), transaction_id.clone());
        }
        if let Some(ref out_trade_no) = self.out_trade_no {
            params.insert("out_trade_no".to_string(), out_trade_no.clone());
        }
        if let Some(ref out_refund_no) = self.out_refund_no {
            params.insert("out_refund_no".to_string(), out_refund_no.clone());
        }
        if let Some(ref refund_id) = self.refund_id {
            params.insert("refund_id".to_string(), refund_id.clone());
        }
        if let Some(ref refund_fee) = self.refund_fee {
            params.insert("refund_fee".to_string(), refund_fee.to_string());
        }
        if let Some(ref settlement_refund_fee) = self.settlement_refund_fee {
            params.insert(
                "settlement_refund_fee".to_string(),
                settlement_refund_fee.to_string(),
            );
        }
        if let Some(ref total_fee) = self.total_fee {
            params.insert("total_fee".to_string(), total_fee.to_string());
        }
        if let Some(ref settlement_total_fee) = self.settlement_total_fee {
            params.insert(
                "settlement_total_fee".to_string(),
                settlement_total_fee.to_string(),
            );
        }
        if let Some(ref fee_type) = self.fee_type {
            params.insert("fee_type".to_string(), fee_type.clone());
        }
        if let Some(ref cash_fee) = self.cash_fee {
            params.insert("cash_fee".to_string(), cash_fee.to_string());
        }
        if let Some(ref cash_refund_fee) = self.cash_refund_fee {
            params.insert("cash_refund_fee".to_string(), cash_refund_fee.to_string());
        }

        // 生成签名
        let calculated_sign = UnifiedOrderRequest::generate_sign(&params, api_key)?;

        if let Some(ref sign) = self.sign {
            Ok(calculated_sign == *sign)
        } else {
            Ok(false)
        }
    }
}

/// 退款查询响应
#[derive(Debug, Clone, Deserialize)]
pub struct RefundQueryResponse {
    /// 返回状态码
    pub return_code: String,
    /// 返回信息
    pub return_msg: String,
    /// 业务结果
    pub result_code: Option<String>,
    /// 错误代码
    pub err_code: Option<String>,
    /// 错误代码描述
    pub err_code_des: Option<String>,
    /// 应用ID
    #[serde(rename = "appid")]
    pub app_id: Option<String>,
    /// 商户号
    pub mch_id: Option<String>,
    /// 随机字符串
    pub nonce_str: Option<String>,
    /// 签名
    pub sign: Option<String>,
    /// 微信支付订单号
    pub transaction_id: Option<String>,
    /// 商户订单号
    pub out_trade_no: Option<String>,
    /// 订单金额
    pub total_fee: Option<i32>,
    /// 应结订单金额
    pub settlement_total_fee: Option<i32>,
    /// 货币种类
    pub fee_type: Option<String>,
    /// 现金支付金额
    pub cash_fee: Option<i32>,
    /// 退款笔数
    pub refund_count: Option<i32>,
}

pub struct RefundQueryRequest {
    /// 微信支付订单号
    pub transaction_id: Option<String>,
    /// 商户订单号
    pub out_trade_no: Option<String>,
    /// 商户退款单号
    pub out_refund_no: Option<String>,
    /// 微信退款单号
    pub refund_id: Option<String>,
}

impl RefundQueryRequest {
    pub fn new(
        transaction_id: Option<String>,
        out_trade_no: Option<String>,
        out_refund_no: Option<String>,
        refund_id: Option<String>,
    ) -> Self {
        Self {
            transaction_id,
            out_trade_no,
            out_refund_no,
            refund_id,
        }
    }
}

impl RefundQueryResponse {
    /// 从XML解析
    pub fn from_xml(xml: &str) -> LabradorResult<Self> {
        let mut response: Self = XmlSerializer::deserialize(xml)?;
        if response.err_code.is_none() {
            response.err_code = Some(response.return_code.to_string());
        }
        if response.err_code_des.is_none() {
            response.err_code_des = Some(response.return_msg.to_string());
        }
        Ok(response)
    }

    /// 检查是否成功
    pub fn is_success(&self) -> bool {
        self.return_code == "SUCCESS" && self.result_code.as_deref() == Some("SUCCESS")
    }

    /// 验证签名
    pub fn verify_signature(&self, api_key: &str) -> LabradorResult<bool> {
        let mut params = BTreeMap::new();

        // 添加所有字段到Map
        if let Some(ref app_id) = self.app_id {
            params.insert("appid".to_string(), app_id.clone());
        }
        if let Some(ref mch_id) = self.mch_id {
            params.insert("mch_id".to_string(), mch_id.clone());
        }
        if let Some(ref nonce_str) = self.nonce_str {
            params.insert("nonce_str".to_string(), nonce_str.clone());
        }
        params.insert("return_code".to_string(), self.return_code.clone());
        params.insert("return_msg".to_string(), self.return_msg.clone());
        if let Some(ref result_code) = self.result_code {
            params.insert("result_code".to_string(), result_code.clone());
        }
        if let Some(ref err_code) = self.err_code {
            params.insert("err_code".to_string(), err_code.clone());
        }
        if let Some(ref err_code_des) = self.err_code_des {
            params.insert("err_code_des".to_string(), err_code_des.clone());
        }
        if let Some(ref transaction_id) = self.transaction_id {
            params.insert("transaction_id".to_string(), transaction_id.clone());
        }
        if let Some(ref out_trade_no) = self.out_trade_no {
            params.insert("out_trade_no".to_string(), out_trade_no.clone());
        }
        if let Some(ref total_fee) = self.total_fee {
            params.insert("total_fee".to_string(), total_fee.to_string());
        }
        if let Some(ref settlement_total_fee) = self.settlement_total_fee {
            params.insert(
                "settlement_total_fee".to_string(),
                settlement_total_fee.to_string(),
            );
        }
        if let Some(ref fee_type) = self.fee_type {
            params.insert("fee_type".to_string(), fee_type.clone());
        }
        if let Some(ref cash_fee) = self.cash_fee {
            params.insert("cash_fee".to_string(), cash_fee.to_string());
        }
        if let Some(ref refund_count) = self.refund_count {
            params.insert("refund_count".to_string(), refund_count.to_string());
        }

        // 生成签名
        let calculated_sign = UnifiedOrderRequest::generate_sign(&params, api_key)?;

        if let Some(ref sign) = self.sign {
            Ok(calculated_sign == *sign)
        } else {
            Ok(false)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WechatPayNotifyResponse {
    pub appid: Option<String>,
    /// 交易类型
    pub trade_type: String,
    /// 付款银行
    pub bank_type: Option<String>,
    /// 货币种类
    pub fee_type: Option<String>,
    /// 是否关注公众账号
    pub is_subscribe: Option<String>,
    /// 加密字符串
    pub nonce_str: Option<String>,
    /// 商户号
    pub mch_id: String,
    /// 用户号
    pub openid: String,
    /// 商户订单号
    pub out_trade_no: String,
    /// 业务结果
    pub return_code: String,
    /// 签名
    pub sign: String,
    /// 支付完成时间
    pub time_end: String,
    /// 订单金额
    pub total_fee: String,
    /// 实际现金支付金额
    pub cash_fee: String,
    /// 总代金券金额
    pub coupon_fee: Option<String>,
    /// 代金券使用数量
    pub coupon_count: Option<String>,
    /// 代金券类型
    pub coupon_type: Option<String>,
    /// 代金券ID
    pub coupon_id: Option<String>,
    /// 微信支付订单号
    pub transaction_id: String,
    ///商家数据包
    pub attach: Option<String>,
    /// 业务结果
    pub result_code: String,
    /// 返回结果
    pub return_msg: String,
    /// 错误码
    pub err_code: Option<String>,
    pub err_code_des: Option<String>,
}

impl WechatPayNotifyResponse {
    pub fn from_xml(xml: &str) -> LabradorResult<Self> {
        let mut response: Self = XmlSerializer::deserialize(xml)?;
        if response.return_code.ne(&"SUCCESS") && !response.return_code.is_empty() {
            return Err(LabraError::RequestError(format!(
                "微信回调失败: {}",
                response.return_msg
            )));
        }

        if response.result_code.ne(&"SUCCESS") && !response.result_code.is_empty() {
            return Err(LabraError::RequestError(format!(
                "微信回调失败: {}",
                response.return_msg
            )));
        }
        if response.err_code.is_none() {
            response.err_code = Some(response.return_code.to_string());
        }
        if response.err_code_des.is_none() {
            response.err_code_des = Some(response.return_msg.to_string());
        }
        Ok(response)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WechatScanPayNotifyResponse {
    /// 用户标识
    pub openid: String,
    /// <pre>
    /// 是否关注公众账号.
    /// 仅在公众账号类型支付有效，取值范围：Y或N;Y-关注;N-未关注
    /// </pre>
    pub is_subscribe: String,
    /// <pre>
    /// 商品ID.
    /// 商户定义的商品id 或者订单号
    /// </pre>
    pub product_id: String,
    pub return_code: Option<String>,
    pub return_msg: Option<String>,
    pub result_code: Option<String>,
}

impl WechatScanPayNotifyResponse {
    pub fn from_xml(xml: &str) -> LabradorResult<Self> {
        let response: Self = XmlSerializer::deserialize(xml)?;

        if let Some(return_code) = response.return_code.as_ref() {
            if return_code.ne(&"SUCCESS") {
                return Err(LabraError::RequestError(format!(
                    "微信回调失败: {}",
                    response.return_msg.unwrap_or_default()
                )));
            }

            if let Some(result_code) = response.result_code.as_ref() {
                if result_code.ne(&"SUCCESS") {
                    return Err(LabraError::RequestError(format!(
                        "微信回调失败: {}",
                        response.return_msg.unwrap_or_default()
                    )));
                }
            }
        }
        Ok(response)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WechatPayShorturlRequest {
    /// <pre>
    /// URL链接
    /// long_url
    /// 是
    /// String(512)
    /// weixin：//wxpay/bizpayurl?sign=XXXXX&appid=XXXXX&mch_id=XXXXX&product_id=XXXXXX&time_stamp=XXXXXX&nonce_str=XXXXX
    /// 需要转换的URL，签名用原串，传输需URLencode
    /// </pre>
    pub long_url: Option<String>,
    pub appid: Option<String>,
    /// 商户号
    pub mch_id: Option<String>,
    /// 签名
    pub sign: String,
    /// 加密字符串
    pub nonce_str: Option<String>,
}

#[allow(unused)]
impl WechatPayShorturlRequest {
    pub fn parse_xml(&self) -> String {
        let msg = format!(
            "<xml>\n\
                <appid>{appid}</appid>\n\
                <mch_id>{mch_id}</mch_id>\n\
                <nonce_str>{nonce_str}</nonce_str>\n\
                <long_url>{long_url}</long_url>\n\
                <sign>{sign}</sign>\n\
            </xml>",
            appid = self.appid.to_owned().unwrap_or_default(),
            mch_id = self.mch_id.to_owned().unwrap_or_default(),
            nonce_str = self.nonce_str.to_owned().unwrap_or_default(),
            long_url = self.long_url.to_owned().unwrap_or_default(),
            sign = self.sign,
        );
        msg
    }

    pub fn generate_sign(&self, api_key: &str) -> LabradorResult<String> {
        let mut pairs = BTreeMap::new();
        if let Some(appid) = self.appid.to_owned() {
            pairs.insert("appid".to_string(), appid);
        }
        pairs.insert(
            "mch_id".to_string(),
            self.mch_id.to_owned().unwrap_or_default(),
        );
        pairs.insert(
            "long_url".to_string(),
            self.long_url.to_owned().unwrap_or_default(),
        );
        if let Some(nonce_str) = self.nonce_str.to_owned() {
            pairs.insert("nonce_str".to_string(), nonce_str);
        }
        // 构造签名字符串
        let mut sign_string = String::new();
        for (key, value) in pairs {
            if !value.is_empty() && key != "sign" {
                sign_string.push_str(&format!("{}={}&", key, value));
            }
        }
        sign_string.push_str(&format!("key={}", api_key));

        // MD5签名并转为大写
        let sign = CryptoUtils::md5(sign_string.as_bytes());
        Ok(sign.to_uppercase())
    }
}

/// 解密后的退款通知数据格式示例：
///
/// ```text
/// <root>
/// <out_refund_no><![CDATA[131811191610442717309]]></out_refund_no>
/// <out_trade_no><![CDATA[71106718111915575302817]]></out_trade_no>
/// <refund_account><![CDATA[REFUND_SOURCE_RECHARGE_FUNDS]]></refund_account>
/// <refund_fee><![CDATA[3960]]></refund_fee>
/// <refund_id><![CDATA[50000408942018111907145868882]]></refund_id>
/// <refund_recv_accout><![CDATA[支付用户零钱]]></refund_recv_accout>
/// <refund_request_source><![CDATA[API]]></refund_request_source>
/// <refund_status><![CDATA[SUCCESS]]></refund_status>
/// <settlement_refund_fee><![CDATA[3960]]></settlement_refund_fee>
/// <settlement_total_fee><![CDATA[3960]]></settlement_total_fee>
/// <success_time><![CDATA[2018-11-19 16:24:13]]></success_time>
/// <total_fee><![CDATA[3960]]></total_fee>
/// <transaction_id><![CDATA[4200000215201811190261405420]]></transaction_id>
/// </root>
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WechatDecryptRefundNotifyResponse {
    /// 退款编号
    pub out_refund_no: String,
    /// 退款商户订单号
    pub out_trade_no: String,
    /// 退款账户
    pub refund_account: String,
    /// 退款金额
    pub refund_fee: String,
    /// 退款编号
    pub refund_id: String,
    /// 退款接受账户
    pub refund_recv_accout: String,
    /// 退款来源
    pub refund_request_source: String,
    /// 退款状态
    pub refund_status: String,
    /// 结算退款金额
    pub settlement_refund_fee: String,
    /// 结算退款金额
    pub settlement_total_fee: String,
    /// 成功时间
    pub success_time: String,
    /// 总金额
    pub total_fee: String,
    /// 微信交易编号
    pub transaction_id: String,
}

/// 支付通知的XML数据格式示例：
///
/// ```text
/// <xml>
/// <return_code>SUCCESS</return_code>
/// <appid><![CDATA[wx2421b1c4370ec43b]]></appid>
/// <mch_id><![CDATA[10000100]]></mch_id>
/// <nonce_str><![CDATA[TeqClE3i0mvn3DrK]]></nonce_str>
/// <req_info><![CDATA[T87GAHG17TGAHG1TGHAHAHA1Y1CIOA9UGJH1GAHV871HAGAGQYQQPOOJMXNBCXBVNMNMAJAA]]></req_info>
/// </xml>
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct WechatEncryptResponse {
    pub appid: Option<String>,
    /// 加密字符串
    pub nonce_str: Option<String>,
    /// 商户号
    pub mch_id: String,
    /// 加密信息
    pub req_info: String,
    /// 业务结果
    pub return_code: String,
    /// 返回信息
    pub return_msg: Option<String>,
}

impl WechatEncryptResponse {
    pub fn from_xml(xml: &str) -> LabradorResult<WechatEncryptResponse> {
        let response: WechatEncryptResponse = XmlSerializer::deserialize(xml)?;
        if response.return_code.ne(&"SUCCESS") && !response.return_code.is_empty() {
            return Err(LabraError::RequestError(format!(
                "微信回调失败: {}",
                response.return_msg.unwrap_or_default()
            )));
        }

        Ok(response)
    }
}

/// 账单类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BillType {
    /// 所有订单账单
    All,
    /// 成功支付的订单账单
    Success,
    /// 退款账单
    Refund,
}
/// 账户类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountType {
    /// 基本账户
    BASIC,
    /// 运营账户
    OPERATION,
    /// 手续费账户
    FEES,
}

impl AccountType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AccountType::BASIC => "BASIC",
            AccountType::OPERATION => "OPERATION",
            AccountType::FEES => "FEES",
        }
    }
}

impl BillType {
    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            BillType::All => "ALL",
            BillType::Success => "SUCCESS",
            BillType::Refund => "REFUND",
        }
    }
}

/// 撤销订单请求类
#[derive(Debug, Serialize, Deserialize)]
pub struct WechatOrderReverseRequest {
    pub appid: Option<String>,
    /// 商户号
    pub mch_id: String,
    /// 商户订单号
    pub out_trade_no: String,
    /// 交易编号
    pub transaction_id: String,
    /// 签名
    pub sign: String,
    /// 加密字符串
    pub nonce_str: Option<String>,
}

impl WechatOrderReverseRequest {
    pub fn parse_xml(&self) -> String {
        let msg = format!(
            "<xml>\n\
                <appid>{appid}</appid>\n\
                <mch_id>{mch_id}</mch_id>\n\
                <nonce_str>{nonce_str}</nonce_str>\n\
                <out_trade_no>{out_trade_no}</out_trade_no>\n\
                <transaction_id>{transaction_id}</transaction_id>\n\
                <sign>{sign}</sign>\n\
            </xml>",
            appid = self.appid.to_owned().unwrap_or_default(),
            mch_id = self.mch_id,
            nonce_str = self.nonce_str.to_owned().unwrap_or_default(),
            transaction_id = self.transaction_id,
            out_trade_no = self.out_trade_no,
            sign = self.sign,
        );
        msg
    }

    pub fn generate_sign(&self, api_key: &str) -> LabradorResult<String> {
        let mut pairs = BTreeMap::new();
        if let Some(appid) = self.appid.to_owned() {
            pairs.insert("appid".to_string(), appid);
        }
        pairs.insert("mch_id".to_string(), self.mch_id.to_owned());
        pairs.insert("out_trade_no".to_string(), self.out_trade_no.to_owned());
        pairs.insert("transaction_id".to_string(), self.transaction_id.to_owned());

        // 构造签名字符串
        let mut sign_string = String::new();
        for (key, value) in pairs {
            if !value.is_empty() && key != "sign" {
                sign_string.push_str(&format!("{}={}&", key, value));
            }
        }
        sign_string.push_str(&format!("key={}", api_key));

        // MD5签名并转为大写
        let sign = CryptoUtils::md5(sign_string.as_bytes());
        Ok(sign.to_uppercase())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WechatOrderReverseResponse {
    pub appid: Option<String>,
    /// 商户号
    pub mch_id: String,
    /// 签名
    pub sign: String,
    /// 微信交易编号
    pub recall: String,
    /// 加密字符串
    pub nonce_str: Option<String>,
    /// 业务结果
    pub result_code: String,
    pub return_code: String,
    /// 返回结果
    pub return_msg: String,
    /// 错误码
    pub err_code: Option<String>,
    pub err_code_des: Option<String>,
}

impl WechatOrderReverseResponse {
    pub fn from_xml(xml: &str) -> LabradorResult<Self> {
        let mut response: WechatOrderReverseResponse = XmlSerializer::deserialize(xml)?;
        if response.return_code.ne(&"SUCCESS") && !response.return_code.is_empty() {
            return Err(LabraError::RequestError(format!(
                "微信处理失败: {}",
                response.return_msg
            )));
        }

        if response.result_code.ne(&"SUCCESS") && !response.result_code.is_empty() {
            return Err(LabraError::RequestError(format!(
                "微信回调失败: {}",
                response.return_msg
            )));
        }

        if response.err_code.is_none() {
            response.err_code = Some(response.return_code.to_string());
        }
        if response.err_code_des.is_none() {
            response.err_code_des = Some(response.return_msg.to_string());
        }
        Ok(response)
    }

    pub fn is_success(&self) -> bool {
        self.return_code == "SUCCESS" && self.result_code == "SUCCESS"
    }

    /// 验证签名
    pub fn verify_signature(&self, api_key: &str) -> LabradorResult<bool> {
        let mut params = BTreeMap::new();

        // 添加所有字段到Map
        if let Some(ref app_id) = self.appid {
            params.insert("appid".to_string(), app_id.clone());
        }
        params.insert("mch_id".to_string(), self.mch_id.clone());
        params.insert(
            "nonce_str".to_string(),
            self.nonce_str.clone().unwrap_or_default(),
        );
        params.insert("result_code".to_string(), self.result_code.clone());
        params.insert("return_code".to_string(), self.return_code.clone());
        params.insert("return_msg".to_string(), self.return_msg.clone());
        params.insert("recall".to_string(), self.recall.clone());
        if let Some(ref err_code) = self.err_code {
            params.insert("err_code".to_string(), err_code.clone());
        }
        if let Some(ref err_code_des) = self.err_code_des {
            params.insert("err_code_des".to_string(), err_code_des.clone());
        }
        params.insert("sign_type".to_string(), "HMAC-SHA256".to_string());
        params.insert("sign".to_string(), self.sign.clone());

        // 生成签名
        let calculated_sign = UnifiedOrderRequest::generate_sign(&params, api_key)?;
        Ok(calculated_sign == self.sign)
    }
}

// 微信支付V3版本 ↓

/// 统一下单请求V3
#[derive(Debug, Clone, Serialize)]
pub struct UnifiedOrderRequestV3 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,
    /// 直连商户号
    #[serde(rename = "mchid")]
    pub mch_id: String,
    /// 商品描述
    pub description: String,
    /// 商户订单号
    pub out_trade_no: String,
    /// 交易结束时间
    /// ```text
    /// 1、定义：支付结束时间是指用户能够完成该笔订单支付的最后时限，并非订单关闭的时间。超过此时间后，用户将无法对该笔订单进行支付。如商户需在超时后关闭订单，请调用关闭订单API接口。
    ///
    /// 2、格式要求：支付结束时间需遵循rfc3339标准格式：yyyy-MM-DDTHH:mm:ss+TIMEZONE。yyyy-MM-DD 表示年月日；T 字符用于分隔日期和时间部分；HH:mm:ss 表示具体的时分秒；TIMEZONE 表示时区（例如，+08:00 对应东八区时间，即北京时间。
    ///
    /// 示例：2015-05-20T13:29:35+08:00 表示北京时间2015年5月20日13点29分35秒。
    ///
    /// 3、若未指定支付结束时间，系统默认以下单时间为起始点计算时效；超过 7 天未支付的订单，无法再支付。
    ///
    /// 4、注意事项：
    ///
    /// 若当前实际时间已超过订单设置的支付结束时间（time_expire），建议先使用关单接口关闭订单，再使用新的商户订单号重新下单，生成全新订单供用户支付。
    ///
    /// 支付结束时间不能早于下单时间后1分钟，若设置的支付结束时间早于该时间，系统将自动调整为下单时间后1分钟作为支付结束时间。
    ///
    /// 传递的支付结束时间需在下单时间的15天以内，如超过15天，微信支付会自动将该时间调整为下单时间后的第15天。
    /// ```
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_expire: Option<String>,
    #[serde(skip_serializing)]
    pub trade_type: TradeType,
    /// 附加数据
    /// 商户在创建订单时可传入自定义数据包，该数据对用户不可见，
    /// 用于存储订单相关的商户自定义信息，其总长度限制在128字符以内。
    /// 支付成功后查询订单API和支付成功回调通知均会将此字段返回给商户，并且该字段还会体现在交易账单。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
    /// 订单优惠标记
    /// 代金券在创建时可以配置多个订单优惠标记，标记的内容由创券商户自定义设置。
    /// 详细参考：创建代金券批次API。如果代金券有配置订单优惠标记，则必须在该参数传任意一个配置的订单优惠标记才能使用券。
    /// 如果代金券没有配置订单优惠标记，则可以不传该参数。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_tag: Option<String>,
    /// 【电子发票入口开放标识】 传入true时，支付成功消息和支付详情页将出现开票入口。
    /// 需要在微信支付商户平台或微信公众平台开通电子发票功能，传此字段才可生效。 详细参考：电子发票介绍
    /// true：是
    /// false：否
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_fapiao: Option<bool>,
    /// 通知地址
    pub notify_url: String,
    /// 订单金额
    pub amount: Amount,
    /// 支付者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer: Option<Payer>,
    /// 优惠功能
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<Detail>,
    /// 场景信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_info: Option<SceneInfo>,
    /// 结算信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_info: Option<WechatSettleInfo>,
}

impl UnifiedOrderRequestV3 {
    /// 创建新的统一下单请求
    pub fn new(
        trade_type: TradeType,
        out_trade_no: &str,
        description: &str,
        amount: Amount,
        detail: Detail,
        notify_url: &str,
    ) -> Self {
        let time_expire = Utc::now()
            .checked_add_signed(Duration::hours(2))
            .unwrap()
            .to_rfc3339();
        Self {
            appid: None,
            mch_id: "".to_string(),
            description: description.to_string(),
            out_trade_no: out_trade_no.to_string(),
            notify_url: notify_url.to_string(),
            amount,
            payer: None,
            detail: Some(detail),
            attach: None,
            goods_tag: None,
            time_expire: Some(time_expire),
            scene_info: None,
            settle_info: None,
            trade_type,
            support_fapiao: None,
        }
    }

    pub fn trade_type(&mut self, trade_type: TradeType) -> &mut Self {
        self.trade_type = trade_type;
        self
    }

    /// 添加附加数据
    pub fn attach(&mut self, attach: &str) -> &mut Self {
        self.attach = Some(attach.to_string());
        self
    }

    /// 添加商户号
    pub fn mch_id(&mut self, mch_id: &str) -> &mut Self {
        self.mch_id = mch_id.to_string();
        self
    }

    /// 添加应用ID
    pub fn appid(&mut self, appid: &str) -> &mut Self {
        self.appid = Some(appid.to_string());
        self
    }

    /// 添加商品描述
    pub fn description(&mut self, description: &str) -> &mut Self {
        self.description = description.to_string();
        self
    }

    /// 添加通知地址
    pub fn notify_url(&mut self, notify_url: &str) -> &mut Self {
        self.notify_url = notify_url.to_string();
        self
    }

    /// 添加商户订单号
    pub fn out_trade_no(&mut self, out_trade_no: &str) -> &mut Self {
        self.out_trade_no = out_trade_no.to_string();
        self
    }

    /// 添加交易结束时间
    pub fn time_expire(&mut self, time_expire: &str) -> &mut Self {
        self.time_expire = Some(time_expire.to_string());
        self
    }

    /// 添加金额
    pub fn amount(&mut self, amount: Amount) -> &mut Self {
        self.amount = amount;
        self
    }

    /// 添加支付者
    pub fn payer(&mut self, payer: Payer) -> &mut Self {
        self.payer = Some(payer);
        self
    }

    /// 添加详情
    pub fn detail(&mut self, detail: Detail) -> &mut Self {
        self.detail = Some(detail);
        self
    }

    /// 添加场景信息
    pub fn scene_info(&mut self, scene_info: SceneInfo) -> &mut Self {
        self.scene_info = Some(scene_info);
        self
    }

    /// 添加结算信息
    pub fn settle_info(&mut self, settle_info: WechatSettleInfo) -> &mut Self {
        self.settle_info = Some(settle_info);
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Amount {
    /// 订单总金额，单位为分。
    pub total: i64,
    /// 币类型, CNY：人民币，境内商户号仅支持人民币。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// 用户支付金额
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_total: Option<i64>,
    /// 用户支付币种
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_currency: Option<String>,
}

impl Amount {
    pub fn new(total: i64) -> Self {
        Self {
            total,
            currency: None,
            payer_total: None,
            payer_currency: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SceneInfo {
    /// 用户终端IP 用户的客户端IP，支持IPv4和IPv6两种格式的IP地址。 示例值：14.23.150.211
    pub payer_client_ip: String,
    /// 商户端设备号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// 商户端设备IP（codepay场景使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_ip: Option<String>,
    /// 商户门店信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_info: Option<StoreInfo>,
    /// H5场景信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h5_info: Option<H5Info>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct H5Info {
    /// 场景类型 iOS, Android, Wap
    #[serde(rename = "type")]
    pub r#type: String,
    /// 应用名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    /// 网站URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_url: Option<String>,
    /// iOS平台BundleID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// Android平台PackageName
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoreInfo {
    /// 门店编号（微信支付线下场所ID）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 商户系统的门店编码（与id二选一必填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_id: Option<String>,
    /// 详细地址
    pub address: Option<String>,
    /// 门店名称
    pub name: Option<String>,
    /// 地区编码
    pub area_code: Option<String>,
}

impl StoreInfo {
    /// 通过微信支付场所ID创建
    pub fn with_id(id: &str) -> Self {
        StoreInfo {
            id: Some(id.to_string()),
            out_id: None,
            address: None,
            name: None,
            area_code: None,
        }
    }

    /// 通过商户门店编码创建
    pub fn with_out_id(out_id: &str) -> Self {
        StoreInfo {
            id: None,
            out_id: Some(out_id.to_string()),
            address: None,
            name: None,
            area_code: None,
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Payer {
    /// 用户号,用户在直连商户appid下的唯一标识。
    #[serde(skip_serializing_if = "String::is_empty")]
    pub openid: String,
    /// 付款码支付授权码，用户打开微信钱包显示的码。仅codepay场景使用。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,
}

impl Payer {
    pub fn new(open_id: &str) -> Self {
        Payer {
            openid: open_id.to_string(),
            auth_code: None,
        }
    }

    /// 创建付款码支付的payer（使用auth_code而非openid）
    pub fn with_auth_code(auth_code: &str) -> Self {
        Payer {
            openid: String::new(),
            auth_code: Some(auth_code.to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Detail {
    /// 订单原价
    /// 1、商户侧一张小票订单可能被分多次支付，订单原价用于记录整张小票的交易金额。
    /// 2、当订单原价与支付金额不相等，则不享受优惠。
    /// 3、该字段主要用于防止同一张小票分多次支付，以享受多次优惠的情况，正常支付订单不必上传此参数。
    /// 示例值：608800
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_price: Option<i32>,
    /// 商品小票ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<i32>,
    /// 单品列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_detail: Option<Vec<GoodsDetail>>,
}

impl Detail {
    pub fn new(goods_detail: Vec<GoodsDetail>) -> Self {
        Self {
            cost_price: None,
            invoice_id: None,
            goods_detail: Some(goods_detail),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WechatSettleInfo {
    /// 分账标识
    /// 订单的分账标识在下单时设置，传入true表示在订单支付成功后可进行分账操作。以下是详细说明：
    ///
    /// 需要分账（传入true）：
    /// 订单收款成功后，资金将被冻结并转入基本账户的不可用余额。商户可通过请求分账API，将收款资金分配给其他商户或用户。完成分账操作后，可通过接口解冻剩余资金，或在支付成功30天后自动解冻。
    ///
    /// 不需要分账（传入false或不传，默认为false）：
    /// 订单收款成功后，资金不会被冻结，而是直接转入基本账户的可用余额。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profit_sharing: Option<bool>,
}

/// 付款码支付请求 V3
#[derive(Debug, Clone, Serialize)]
pub struct CodepayOrderRequestV3 {
    /// 应用ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,
    /// 直连商户号
    #[serde(rename = "mchid")]
    pub mch_id: String,
    /// 商品描述
    pub description: String,
    /// 商户订单号
    pub out_trade_no: String,
    /// 附加数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
    /// 订单优惠标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_tag: Option<String>,
    /// 电子发票入口开放标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_fapiao: Option<bool>,
    /// 支付者（包含auth_code）
    pub payer: Payer,
    /// 订单金额
    pub amount: Amount,
    /// 场景信息（必填）
    pub scene_info: SceneInfo,
    /// 优惠功能
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<Detail>,
    /// 结算信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_info: Option<WechatSettleInfo>,
}

impl CodepayOrderRequestV3 {
    pub fn new(
        out_trade_no: &str,
        description: &str,
        auth_code: &str,
        amount: Amount,
        scene_info: SceneInfo,
    ) -> Self {
        Self {
            appid: None,
            mch_id: String::new(),
            description: description.to_string(),
            out_trade_no: out_trade_no.to_string(),
            attach: None,
            goods_tag: None,
            support_fapiao: None,
            payer: Payer::with_auth_code(auth_code),
            amount,
            scene_info,
            detail: None,
            settle_info: None,
        }
    }

    pub fn mch_id(mut self, mch_id: &str) -> Self {
        self.mch_id = mch_id.to_string();
        self
    }

    pub fn appid(mut self, appid: &str) -> Self {
        self.appid = Some(appid.to_string());
        self
    }

    pub fn attach(mut self, attach: &str) -> Self {
        self.attach = Some(attach.to_string());
        self
    }

    pub fn goods_tag(mut self, goods_tag: &str) -> Self {
        self.goods_tag = Some(goods_tag.to_string());
        self
    }

    pub fn support_fapiao(mut self, support_fapiao: bool) -> Self {
        self.support_fapiao = Some(support_fapiao);
        self
    }

    pub fn detail(mut self, detail: Detail) -> Self {
        self.detail = Some(detail);
        self
    }

    pub fn settle_info(mut self, settle_info: WechatSettleInfo) -> Self {
        self.settle_info = Some(settle_info);
        self
    }
}

// ==================== 合单支付 (Combine) V3 ====================

/// 合单支付子订单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubOrder {
    /// 商户号
    #[serde(rename = "mchid")]
    pub mch_id: String,
    /// 附加数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
    /// 订单金额
    pub amount: Amount,
    /// 商户订单号
    pub out_trade_no: String,
    /// 商品描述
    pub description: String,
    /// 结算信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_info: Option<WechatSettleInfo>,
}

impl SubOrder {
    pub fn new(mch_id: &str, out_trade_no: &str, description: &str, amount: Amount) -> Self {
        Self {
            mch_id: mch_id.to_string(),
            attach: None,
            amount,
            out_trade_no: out_trade_no.to_string(),
            description: description.to_string(),
            settle_info: None,
        }
    }
}

/// 合单支付-统一下单请求 V3
#[derive(Debug, Clone, Serialize)]
pub struct CombineOrderRequestV3 {
    /// 合单发起方的appid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combine_appid: Option<String>,
    /// 合单发起方的商户号
    #[serde(rename = "combine_mchid")]
    pub combine_mch_id: String,
    /// 合单商户订单号
    pub combine_out_trade_no: String,
    /// 场景信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_info: Option<SceneInfo>,
    /// 子订单列表（最多50笔）
    pub sub_orders: Vec<SubOrder>,
    /// 支付者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combine_payer_info: Option<Payer>,
    /// 交易起始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_start: Option<String>,
    /// 交易结束时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_expire: Option<String>,
    /// 通知地址
    pub notify_url: String,
    /// 交易类型
    #[serde(skip_serializing)]
    pub trade_type: TradeType,
}

impl CombineOrderRequestV3 {
    pub fn new(
        combine_mch_id: &str,
        combine_out_trade_no: &str,
        sub_orders: Vec<SubOrder>,
        notify_url: &str,
        trade_type: TradeType,
    ) -> Self {
        Self {
            combine_appid: None,
            combine_mch_id: combine_mch_id.to_string(),
            combine_out_trade_no: combine_out_trade_no.to_string(),
            scene_info: None,
            sub_orders,
            combine_payer_info: None,
            time_start: None,
            time_expire: None,
            notify_url: notify_url.to_string(),
            trade_type,
        }
    }

    pub fn combine_appid(mut self, appid: &str) -> Self {
        self.combine_appid = Some(appid.to_string());
        self
    }

    pub fn scene_info(mut self, scene_info: SceneInfo) -> Self {
        self.scene_info = Some(scene_info);
        self
    }

    pub fn combine_payer_info(mut self, payer: Payer) -> Self {
        self.combine_payer_info = Some(payer);
        self
    }

    pub fn time_expire(mut self, time_expire: &str) -> Self {
        self.time_expire = Some(time_expire.to_string());
        self
    }

    /// 获取合单支付的请求路径
    pub fn req_path(&self) -> &'static str {
        match self.trade_type {
            TradeType::H5 => "/v3/combine-transactions/h5",
            TradeType::Jsapi => "/v3/combine-transactions/jsapi",
            TradeType::Native => "/v3/combine-transactions/native",
            TradeType::App => "/v3/combine-transactions/app",
            TradeType::Miniapp => "/v3/combine-transactions/jsapi",
            TradeType::Micropay => "/v3/combine-transactions/jsapi",
        }
    }
}

/// 合单支付响应 V3
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombineOrderResponseV3 {
    /// 预支付交易会话标识
    pub prepay_id: Option<String>,
    /// H5支付跳转链接
    #[serde(rename = "h5_url")]
    pub h5_url: Option<String>,
    /// Native支付二维码链接
    #[serde(rename = "code_url")]
    pub code_url: Option<String>,
}

/// 合单支付-查询订单请求 V3
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombineOrderQueryRequestV3 {
    /// 合单商户订单号
    pub combine_out_trade_no: String,
}

impl CombineOrderQueryRequestV3 {
    pub fn new(combine_out_trade_no: &str) -> Self {
        Self {
            combine_out_trade_no: combine_out_trade_no.to_string(),
        }
    }

    pub fn req_path(&self) -> String {
        format!(
            "/v3/combine-transactions/out-trade-no/{}",
            self.combine_out_trade_no
        )
    }
}

/// 合单支付-关单请求 V3
pub struct CombineCloseOrderRequestV3 {
    pub combine_out_trade_no: String,
    /// 合单发起方商户号
    pub combine_mch_id: String,
    /// 子订单关单信息
    pub sub_orders: Vec<SubOrderCloseInfo>,
}

/// 子订单关单信息
#[derive(Debug, Clone, Serialize)]
pub struct SubOrderCloseInfo {
    /// 子订单商户号
    #[serde(rename = "mchid")]
    pub mch_id: String,
    /// 子订单商户订单号
    pub out_trade_no: String,
}

impl CombineCloseOrderRequestV3 {
    pub fn new(
        combine_out_trade_no: &str,
        combine_mch_id: &str,
        sub_orders: Vec<SubOrderCloseInfo>,
    ) -> Self {
        Self {
            combine_out_trade_no: combine_out_trade_no.to_string(),
            combine_mch_id: combine_mch_id.to_string(),
            sub_orders,
        }
    }

    pub fn req_path(&self) -> String {
        format!(
            "/v3/combine-transactions/out-trade-no/{}/close",
            self.combine_out_trade_no
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoodsDetail {
    /// 商户侧商品编码
    pub merchant_goods_id: String,
    /// 微信侧商品编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechatpay_goods_id: Option<String>,
    /// 商品名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_name: Option<String>,
    /// 商品数量
    pub quantity: i32,
    /// 商品单价
    pub unit_price: i32,
    /// 商品退款金额
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_amount: Option<i32>,
    /// 商品退货数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_quantity: Option<i32>,
}

impl GoodsDetail {
    pub fn new(
        merchant_goods_id: String,
        goods_name: String,
        quantity: i32,
        unit_price: i32,
    ) -> Self {
        Self {
            merchant_goods_id,
            wechatpay_goods_id: None,
            goods_name: Some(goods_name),
            quantity,
            unit_price,
            refund_amount: None,
            refund_quantity: None,
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformCertificateResponse {
    pub data: Option<Vec<PlatformCertificate>>,
}

#[allow(unused)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformCertificate {
    /// 加密前的对象类型
    pub effective_time: String,
    /// 加密前的对象类型
    pub expire_time: String,
    /// 加密算法
    pub encrypt_certificate: WechatEncryptResponseV3,
    /// Base64编码后的密文
    pub serial_no: String,
}

#[allow(unused)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatEncryptResponseV3 {
    /// 加密前的对象类型
    pub original_type: Option<String>,
    /// 加密算法
    pub algorithm: String,
    /// Base64编码后的密文
    pub ciphertext: Option<String>,
    /// 加密使用的随机串初始化向量）
    pub nonce: String,
    /// 附加数据包（可能为空）
    pub associated_data: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WechatSignatureHeader {
    /// 时间戳
    pub time_stamp: String,
    /// 随机串
    pub nonce: String,
    /// 已签名字符串
    pub signature: String,
    /// 证书序列号
    pub serial: String,
}

impl WechatSignatureHeader {
    pub fn from_header(header: &HeaderMap) -> Self {
        let timpestamp = header.get("Wechatpay-Timestamp");
        let time_stamp = timpestamp
            .map(|h| h.to_str().unwrap_or_default().to_string())
            .unwrap_or_default();
        let nonce = header.get("Wechatpay-Nonce");
        let nonce = nonce
            .map(|h| h.to_str().unwrap_or_default().to_string())
            .unwrap_or_default();
        let signature = header.get("Wechatpay-Signature");
        let signature = signature
            .map(|h| h.to_str().unwrap_or_default().to_string())
            .unwrap_or_default();
        let serial = header.get("Wechatpay-Serial");
        let serial = serial
            .map(|h| h.to_str().unwrap_or_default().to_string())
            .unwrap_or_default();
        WechatSignatureHeader {
            time_stamp,
            nonce,
            signature,
            serial,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WechatPayCommonResponse<T> {
    pub code: Option<String>,
    pub detail: Option<Value>,
    pub message: Option<String>,
    #[serde(flatten)]
    pub data: T,
}

impl<T> WechatPayCommonResponse<T> {
    /// 检查是否成功
    pub fn is_success(&self) -> bool {
        self.code.is_none()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WechatPayResponseV3 {
    pub prepay_id: Option<String>,
    /// 支付跳转链接（H5支付 会返回）
    pub h5_url: Option<String>,
    /// 二维码链接（NATIVE支付 会返回）
    pub code_url: Option<String>,
}

impl WechatPayResponseV3 {
    pub fn get_pay_info(
        &self,
        trade_type: TradeType,
        appid: Option<String>,
        mchid: String,
        private_key: Option<String>,
    ) -> LabradorResult<Value> {
        let timpstamp = timestamp_millis() / 1000;
        let nonce_str = random_string(16);
        let private_key = private_key.unwrap_or_default();
        let appid = appid.unwrap_or_default();
        let encryptor = RsaEncryptor::with_private_key(private_key.as_bytes(), RsaKeyFormat::Pem);
        match trade_type {
            TradeType::H5 => Ok(Value::String(self.h5_url.to_owned().unwrap_or_default())),
            TradeType::Jsapi => {
                let mut result = JsapiResult {
                    app_id: appid.to_owned(),
                    time_stamp: timpstamp.to_string(),
                    nonce_str,
                    prepay_id: self.prepay_id.to_owned().unwrap_or_default(),
                    package: format!(
                        "prepay_id={}",
                        self.prepay_id.to_owned().unwrap_or_default()
                    ),
                    sign_type: "RSA".to_string(), //签名类型，默认为RSA，仅支持RSA。
                    pay_sign: String::default(),
                };
                let signature = encryptor
                    .sign(result.get_sign_str().as_bytes(), HashType::Sha256)
                    .map_err(|e| LabraError::Sign(format!("RSA加密错误: {}", e)))?;
                result.pay_sign = general_purpose::STANDARD.encode(&signature);
                Ok(serde_json::to_value(result)?)
            }
            TradeType::Native => Ok(Value::String(self.code_url.to_owned().unwrap_or_default())),
            TradeType::App => {
                let mut result = AppResult {
                    partner_id: mchid,
                    appid: appid.to_owned(),
                    time_stamp: timpstamp.to_string(),
                    nonce_str,
                    package_value: "Sign=WXPay".to_string(),
                    prepay_id: self.prepay_id.to_owned().unwrap_or_default(),
                    sign: "".to_string(),
                };
                let signature = encryptor
                    .sign(result.get_sign_str().as_bytes(), HashType::Sha256)
                    .map_err(|e| LabraError::Sign(format!("RSA加密错误: {}", e)))?;
                result.sign = general_purpose::STANDARD.encode(&signature);
                Ok(serde_json::to_value(result)?)
            }
            _ => Err(LabraError::Validation("不支持的支付类型".to_string())),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsapiResult {
    app_id: String,
    time_stamp: String,
    nonce_str: String,
    package: String,
    sign_type: String,
    pay_sign: String,
    prepay_id: String,
}

impl JsapiResult {
    pub fn get_sign_str(&self) -> String {
        format!(
            "{}\n{}\n{}\n{}\n",
            self.app_id, self.time_stamp, self.nonce_str, self.package
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct AppResult {
    appid: String,
    partner_id: String,
    prepay_id: String,
    time_stamp: String,
    nonce_str: String,
    package_value: String,
    sign: String,
}

impl AppResult {
    pub fn get_sign_str(&self) -> String {
        format!(
            "{}\n{}\n{}\n{}\n",
            self.appid, self.time_stamp, self.nonce_str, self.prepay_id
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PromotionDetail {
    /// 券ID
    pub coupon_id: String,
    /// 优惠券面额
    pub amount: i64,
    /// 优惠名称
    pub name: Option<String>,
    /// 活动ID
    pub stock_id: Option<String>,
    /// 微信出资
    pub wechatpay_contribute: Option<i64>,
    /// 商户出资
    pub merchant_contribute: Option<i64>,
    /// 其他出资
    pub other_contribute: Option<i64>,
    /// CNY：人民币，境内商户号仅支持人民币。
    pub currency: Option<String>,
    /// 优惠范围 GLOBAL：全场代金券 SINGLE：单品优惠
    pub scope: Option<String>,
    /// 单品列表
    pub goods_detail: Option<GoodsDetail>,
    /// COUPON：代金券，需要走结算资金的充值型代金券 *  DISCOUNT：优惠券，不走结算资金的免充值型优惠券
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefundPromotionDetail {
    /// 券ID
    pub promotion_id: String,
    /// 优惠范围 GLOBAL：全场代金券 SINGLE：单品优惠
    pub scope: Option<String>,
    /// COUPON：代金券，需要走结算资金的充值型代金券 *  DISCOUNT：优惠券，不走结算资金的免充值型优惠券
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// 优惠券面额
    pub amount: i64,
    /// 优惠退款金额<=退款金额，退款金额-代金券或立减优惠退款金额为用户支付的现金，说明详见代金券或立减优惠，单位为分
    pub refund_amount: i64,
    /// 单品列表
    pub goods_detail: Option<GoodsDetail>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderQueryRequestV3 {
    /// 微信支付订单号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// 商户订单号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_trade_no: Option<String>,
}

impl OrderQueryRequestV3 {
    pub fn req_path(&self) -> String {
        if let Some(otr) = self.out_trade_no.as_ref() {
            format!("/v3/pay/transactions/out-trade-no/{}", otr)
        } else {
            let tid = self.transaction_id.to_owned().unwrap_or_default();
            format!("/v3/pay/transactions/id/{}", tid)
        }
    }

    /// 通过商户订单号查询
    pub fn by_out_trade_no(out_trade_no: &str) -> Self {
        Self {
            out_trade_no: Some(out_trade_no.to_string()),
            transaction_id: None,
        }
    }

    /// 通过微信订单号查询
    pub fn by_transaction_id(transaction_id: &str) -> Self {
        Self {
            out_trade_no: None,
            transaction_id: Some(transaction_id.to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderQueryResponseV3 {
    pub appid: String, //1
    /// 商户号
    #[serde(rename = "mchid")]
    pub mch_id: String, //2
    /// 商户系统的订单号，与请求一致。
    pub out_trade_no: String, //3
    /// 交易类型 调用接口提交的交易类型，取值如下：JSAPI，NATIVE，APP，MICROPAY，详细说明见参数规定
    #[serde(default)]
    pub trade_type: String,
    /// 微信支付订单号
    #[serde(default)]
    pub transaction_id: String,
    /// SUCCESS—支付成功,REFUND—转入退款,NOTPAY—未支付,CLOSED—已关闭,REVOKED—已撤销（刷卡支付）,USERPAYING--用户支付中,PAYERROR--支付失败(其他原因，如银行返回失败)
    pub trade_state: String, //5
    /// 交易状态描述
    pub trade_state_desc: String, //6
    /// 付款银行
    #[serde(default)]
    pub bank_type: String,
    /// 附加数据，原样返回
    #[serde(default)]
    pub attach: Option<String>,
    /// 支付完成时间，遵循rfc3339标准格式，格式为YYYY-MM-DDTHH:mm:ss+TIMEZONE，YYYY-MM-DD表示年月日，T出现在字符串中，表示time元素的开头，HH:mm:ss表示时分秒，TIMEZONE表示时区（+08:00表示东八区时间，领先UTC 8小时，即北京时间）。例如：2015-05-20T13:29:35+08:00表示，北京时间2015年5月20日 13点29分35秒。
    #[serde(default)]
    pub success_time: String,
    /// 支付者
    #[serde(default)]
    pub payer: Payer,
    /// 订单金额信息，当支付成功时返回该字段。
    pub amount: Option<Amount>, //4
    /// 场景信息
    pub scene_info: Option<SceneInfo>, //7
    /// 优惠功能，享受优惠时返回该字段。
    #[serde(default)]
    pub promotion_detail: Vec<Option<PromotionDetail>>, // 8
    /// 货币类型，符合ISO 4217标准的三位字母代码，默认人民币：CNY，其他值列表详见货币类型
    #[serde(default)]
    pub fee_type: Option<String>,
    /// 订单金额
    #[serde(default)]
    pub total_fee: i64,
    /// 应结订单金额=订单金额-非充值代金券金额，应结订单金额<=订单金额。
    #[serde(default)]
    pub settlement_total_fee: Option<i64>,
    /// “代金券”金额<=订单金额，订单金额-“代金券”金额=现金支付金额，详见支付金额
    #[serde(default)]
    pub coupon_fee: Option<i64>,
    /// 代金券使用数量
    #[serde(default)]
    pub coupon_count: Option<i64>,
    /// 现金支付金额订单现金支付金额，详见支付金额
    #[serde(default)]
    pub cash_fee: i64,
    /// 货币类型，符合ISO 4217标准的三位字母代码，默认人民币：CNY，其他值列表详见货币类型
    #[serde(default)]
    pub cash_fee_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundRequestV3 {
    /// 交易编号 原支付交易对应的微信订单号。 与out_order_no二选一
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// 商户订单号 原支付交易对应的商户订单号。 与transaction_id二选一
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_trade_no: Option<String>,
    /// 退款订单号 商户系统内部的退款单号，商户系统内部唯一，只能是数字、大小写字母_-|*@ ，同一退款单号多次请求只退一笔。
    pub out_refund_no: String,
    /// 原因 若商户传入，会在下发给用户的退款消息中体现退款原因。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 回调地址 异步接收微信支付退款结果通知的回调地址，通知url必须为外网可访问的url，不能携带参数。 如果参数中传了notify_url，则商户平台上配置的回调地址将不会生效，优先回调当前传的这个地址。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_url: Option<String>,
    /// 订单金额
    pub amount: RefundAmount,
    /// 指定商品退款需要传此参数，其他场景无需传递。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_detail: Option<Vec<GoodsDetail>>,
}

impl RefundRequestV3 {
    pub fn new(out_refund_no: &str, amount: RefundAmount) -> Self {
        Self {
            transaction_id: None,
            out_trade_no: None,
            out_refund_no: out_refund_no.to_string(),
            reason: None,
            notify_url: None,
            amount,
            goods_detail: None,
        }
    }

    pub fn reason(mut self, reason: &str) -> Self {
        self.reason = Some(reason.to_string());
        self
    }
    pub fn notify_url(mut self, notify_url: &str) -> Self {
        self.notify_url = Some(notify_url.to_string());
        self
    }
    pub fn goods_detail(mut self, goods_detail: Vec<GoodsDetail>) -> Self {
        self.goods_detail = Some(goods_detail);
        self
    }

    pub fn transaction_id(mut self, transaction_id: &str) -> Self {
        self.transaction_id = Some(transaction_id.to_string());
        self
    }
    pub fn out_trade_no(mut self, out_trade_no: &str) -> Self {
        self.out_trade_no = Some(out_trade_no.to_string());
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefundAmount {
    /// 退款金额，单位为分。 退款金额，币种的最小单位，只能为整数，不能超过原订单支付金额。
    pub refund: i64,
    /// 原支付交易的订单总金额，币种的最小单位，只能为整数。
    pub total: i64,
    /// 用户实际支付金额，单位为分，只能为整数，详见支付金额
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_total: Option<i64>,
    /// 退款给用户的金额，不包含所有优惠券金额
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_refund: Option<i64>,
    /// 币类型, CNY：人民币，境内商户号仅支持人民币。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefundResponseV3 {
    /// 退款编号
    pub refund_id: String,
    /// 商户订单编号
    pub out_trade_no: String,
    /// 微信交易编号
    pub transaction_id: String,
    /// 退款单号
    pub out_refund_no: String,
    /// 退款渠道 枚举值：
    ///  ORIGINAL—原路退款
    ///  BALANCE—退回到余额
    ///  OTHER_BALANCE—原账户异常退到其他余额账户
    ///  OTHER_BANKCARD—原银行卡异常退到其他银行卡
    pub channel: String,
    ///  退款入账账户
    /// 描述：
    ///  取当前退款单的退款入账方，有以下几种情况：
    ///  1）退回银行卡：{银行名称}{卡类型}{卡尾号}
    ///  2）退回支付用户零钱:支付用户零钱
    ///  3）退还商户:商户基本账户商户结算银行账户
    ///  4）退回支付用户零钱通:支付用户零钱通
    pub user_received_account: String,
    ///  退款成功时间
    pub success_time: Option<String>,
    ///  退款创建时间
    pub create_time: String,
    ///  退款状态
    ///  退款到银行发现用户的卡作废或者冻结了，导致原路退款银行卡失败，可前往商户平台（pay.weixin.qq.com）-交易中心，手动处理此笔退款。
    ///  枚举值：
    ///  SUCCESS：退款成功
    ///  CLOSED：退款关闭
    ///  PROCESSING：退款处理中
    ///  ABNORMAL：退款异常
    pub status: String,
    /// 资金账户 退款所使用资金对应的资金账户类型
    /// 枚举值：
    ///  UNSETTLED : 未结算资金
    ///  AVAILABLE : 可用余额
    ///  UNAVAILABLE : 不可用余额
    ///  OPERATION : 运营户
    ///  BASIC : 基本账户（含可用余额和不可用余额）
    pub funds_account: Option<String>,
    /// 金额信息
    pub amount: RefundAmount,
    /// 优惠退款信息
    pub promotion_detail: Option<Vec<RefundPromotionDetail>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefundQueryResponseV3 {
    /// 微信支付退款号
    pub refund_id: String,
    /// 微信交易编号
    pub transaction_id: String,
    /// 商户系统内部的退款单号，商户系统内部唯一，只能是数字、大小写字母_-|*@ ，同一退款单号多次请求只退一笔。
    pub out_refund_no: String,
    /// 商户订单编号
    pub out_trade_no: String,
    /// 商户订单编号
    /// 描述：退款渠道
    ///  枚举值：
    ///  ORIGINAL：原路退款
    ///  BALANCE：退回到余额
    ///  OTHER_BALANCE：原账户异常退到其他余额账户
    ///  OTHER_BANKCARD：原银行卡异常退到其他银行卡
    pub channel: Option<String>,
    /// 退款入账账户
    /// 描述：
    ///  取当前退款单的退款入账方，有以下几种情况：
    ///  1）退回银行卡：{银行名称}{卡类型}{卡尾号}
    ///  2）退回支付用户零钱:支付用户零钱
    ///  3）退还商户:商户基本账户商户结算银行账户
    ///  4）退回支付用户零钱通:支付用户零钱通
    pub user_received_account: String,
    /// 退款成功时间，当退款状态为退款成功时有返回。
    pub success_time: Option<String>,
    /// 退款受理时间
    pub create_time: String,
    /// 退款状态
    /// 描述：
    ///  退款到银行发现用户的卡作废或者冻结了，导致原路退款银行卡失败，可前往商户平台（pay.weixin.qq.com）-交易中心，手动处理此笔退款。
    ///  枚举值：
    ///  SUCCESS：退款成功
    ///  CLOSED：退款关闭
    ///  PROCESSING：退款处理中
    ///  ABNORMAL：退款异常
    ///  示例值：SUCCESS
    pub status: String,
    ///  退款所使用资金对应的资金账户类型
    /// 枚举值：
    ///  UNSETTLED : 未结算资金
    ///  AVAILABLE : 可用余额
    ///  UNAVAILABLE : 不可用余额
    ///  OPERATION : 运营户
    ///  BASIC : 基本账户（含可用余额和不可用余额）
    ///  示例值：UNSETTLED
    pub funds_account: Option<String>,
    /// 金额信息
    pub amount: RefundAmount,
    /// 优惠退款信息
    pub promotion_detail: Option<Vec<RefundPromotionDetail>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WechatPayNotifyResponseV3 {
    /// 源数据
    pub raw_data: Option<OriginNotifyResponse>,
    /// 解密后的数据
    pub result: Option<DecryptNotifyResult>,
}

impl WechatPayNotifyResponseV3 {
    pub fn decrypt_result(&self) -> DecryptNotifyResult {
        if let Some(res) = self.result.to_owned() {
            res
        } else {
            DecryptNotifyResult {
                appid: "".to_string(),
                mchid: "".to_string(),
                out_trade_no: "".to_string(),
                transaction_id: "".to_string(),
                trade_type: "".to_string(),
                trade_state: "".to_string(),
                trade_state_desc: "".to_string(),
                bank_type: "".to_string(),
                attach: None,
                success_time: "".to_string(),
                payer: Payer {
                    openid: "".to_string(),
                    auth_code: None,
                },
                amount: None,
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OriginNotifyResponse {
    /// 通知ID
    pub id: String,
    /// 通知创建的时间，遵循rfc3339标准格式，格式为YYYY-MM-DDTHH:mm:ss+TIMEZONE，YYYY-MM-DD表示年月日，T出现在字符串中，表示time元素的开头，HH:mm:ss表示时分秒，TIMEZONE表示时区（+08:00表示东八区时间，领先UTC 8小时，即北京时间）。例如：2015-05-20T13:29:35+08:00表示，北京时间2015年5月20日13点29分35秒。
    pub create_time: String,
    /// 通知的类型：
    ///  REFUND.SUCCESS：退款成功通知
    ///  REFUND.ABNORMAL：退款异常通知
    ///  REFUND.CLOSED：退款关闭通知
    ///  示例值：REFUND.SUCCESS
    pub event_type: String,
    ///  通知简要说明
    pub summary: String,
    ///  通知的资源数据类型，支付成功通知为encrypt-resource
    pub resource_type: String,
    /// 通知资源数据
    pub resource: WechatEncryptResponseV3,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WechatRefundNotifyResponseV3 {
    /// 源数据
    pub raw_data: Option<OriginNotifyResponse>,
    /// 解密后的数据
    pub result: Option<DecryptRefundNotifyResult>,
}

impl WechatRefundNotifyResponseV3 {
    pub fn decrypt_result(&self) -> DecryptRefundNotifyResult {
        if let Some(res) = self.result.to_owned() {
            res
        } else {
            DecryptRefundNotifyResult {
                mchid: "".to_string(),
                out_trade_no: "".to_string(),
                transaction_id: "".to_string(),
                out_refund_no: "".to_string(),
                refund_id: "".to_string(),
                success_time: "".to_string(),
                amount: RefundAmount {
                    refund: 0,
                    total: 0,
                    payer_total: None,
                    payer_refund: None,
                    currency: None,
                },
                refund_status: "".to_string(),
                user_received_account: "".to_string(),
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DecryptRefundNotifyResult {
    /// 商户号
    pub mchid: String,
    /// 商户订单号
    pub out_trade_no: String,
    /// 微信支付订单号
    pub transaction_id: String,
    /// 商户退款单号
    pub out_refund_no: String,
    /// 微信支付退款号
    pub refund_id: String,
    /// 退款状态
    ///<pre>
    /// 字段名：退款状态
    /// 变量名：refund_status
    /// 是否必填：是
    /// 类型：string[1,16]
    /// 描述：
    ///  退款状态，枚举值：
    ///  SUCCESS：退款成功
    ///  CLOSE：退款关闭
    ///  ABNORMAL：退款异常，退款到银行发现用户的卡作废或者冻结了，导致原路退款银行卡失败，可前往【商户平台—>交易中心】，手动处理此笔退款
    ///  示例值：SUCCESS
    /// </pre>
    pub refund_status: String,
    /// 描述：
    ///  1、退款成功时间，遵循rfc3339标准格式，格式为YYYY-MM-DDTHH:mm:ss+TIMEZONE，YYYY-MM-DD表示年月日，T出现在字符串中，表示time元素的开头，HH:mm:ss表示时分秒，TIMEZONE表示时区（+08:00表示东八区时间，领先UTC 8小时，即北京时间）。例如：2015-05-20T13:29:35+08:00表示，北京时间2015年5月20日13点29分35秒。
    ///  2、当退款状态为退款成功时返回此参数。
    ///  示例值：2018-06-08T10:34:56+08:00
    pub success_time: String,
    ///<pre>
    /// 字段名：退款入账账户
    /// 变量名：user_received_account
    /// 是否必填：是
    /// 类型：string[1,64]
    /// 描述：
    ///  取当前退款单的退款入账方。
    ///  1、退回银行卡：{银行名称}{卡类型}{卡尾号}
    ///  2、退回支付用户零钱: 支付用户零钱
    ///  3、退还商户: 商户基本账户、商户结算银行账户
    ///  4、退回支付用户零钱通：支付用户零钱通
    ///  示例值：招商银行信用卡0403
    /// </pre>
    pub user_received_account: String,
    /// 订单金额
    pub amount: RefundAmount,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DecryptNotifyResult {
    /// 直连商户申请的公众号或移动应用appid
    pub appid: String,
    /// 商户号
    pub mchid: String,
    /// 商户订单号
    pub out_trade_no: String,
    /// 微信支付订单号
    pub transaction_id: String,
    /// 交易类型
    /// <pre>
    /// 交易类型，枚举值：
    /// JSAPI：公众号支付
    /// NATIVE：扫码支付
    /// APP：APP支付
    /// MICROPAY：付款码支付
    /// MWEB：H5支付
    /// FACEPAY：刷脸支付
    /// 示例值：MICROPAY
    /// </pre>
    pub trade_type: String,
    /// <pre>
    /// 字段名：交易状态
    /// 变量名：trade_state
    /// 是否必填：是
    /// 类型：string[1,32]
    /// 描述：
    ///  交易状态，枚举值：
    ///  SUCCESS：支付成功
    ///  REFUND：转入退款
    ///  NOTPAY：未支付
    ///  CLOSED：已关闭
    ///  REVOKED：已撤销（付款码支付）
    ///  USERPAYING：用户支付中（付款码支付）
    ///  PAYERROR：支付失败(其他原因，如银行返回失败)
    ///  示例值：SUCCESS
    /// </pre>
    pub trade_state: String,
    /// 交易状态描述
    pub trade_state_desc: String,
    /// 银行类型，采用字符串类型的银行标识。
    /// 银行标识请参考[《银行类型对照表》](https://pay.weixin.qq.com/wiki/doc/apiv3/terms_definition/chapter1_1_3.shtml#part-6)
    pub bank_type: String,
    /// 附加数据，在查询API和支付通知中原样返回，可作为自定义参数使用
    pub attach: Option<String>,
    /// 支付完成时间，遵循rfc3339标准格式，格式为YYYY-MM-DDTHH:mm:ss+TIMEZONE，YYYY-MM-DD表示年月日，T出现在字符串中，表示time元素的开头，HH:mm:ss表示时分秒，TIMEZONE表示时区（+08:00表示东八区时间，领先UTC 8小时，即北京时间）。例如：2015-05-20T13:29:35+08:00表示，北京时间2015年5月20日 13点29分35秒。
    pub success_time: String,
    /// 支付者
    pub payer: Payer,
    /// 订单金额
    pub amount: Option<Amount>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FundFlowBillResponseV3 {
    /// 【哈希类型】 哈希类型，固定为SHA1。
    pub hash_type: String,
    /// 【哈希类型】 哈希类型，固定为SHA1。
    pub hash_value: String,
    /// 【下载地址】 供下一步请求账单文件的下载地址，该地址5min内有效。参考下载账单
    pub download_url: String,
}

/// 转换短链接结果对象类
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WechatPayShortUrlResponse {
    /// <pre>
    /// URL链接
    /// short_url
    /// 是
    /// String(64)
    /// weixin：//wxpay/s/XXXXXX
    /// 转换后的URL
    /// </pre>
    pub short_url: String,
    pub return_code: Option<String>,
    pub return_msg: Option<String>,
}

#[allow(unused)]
impl WechatPayShortUrlResponse {
    pub fn from_xml(xml: &str) -> LabradorResult<WechatPayShortUrlResponse> {
        let mut response: Self = XmlSerializer::deserialize(xml)?;
        if response
            .return_code
            .clone()
            .unwrap_or_default()
            .ne(&"SUCCESS")
            && response.return_code.is_none()
        {
            return Err(LabraError::RequestError(format!(
                "微信回调失败: {}",
                response.return_msg.unwrap_or_default()
            )));
        }
        Ok(response)
    }

    /// 检查是否成功
    pub fn is_success(&self) -> bool {
        self.return_code.clone().unwrap_or_default() == "SUCCESS" && self.return_code.is_some()
    }
}

// ==================== V3 撤销订单 ====================

/// 撤销订单请求 V3
#[derive(Debug, Clone, Serialize)]
pub struct OrderReverseRequestV3 {
    /// 商户号
    #[serde(rename = "mchid")]
    pub mch_id: String,
    /// 商户订单号（路径参数）
    #[serde(skip_serializing)]
    pub out_trade_no: String,
}

impl OrderReverseRequestV3 {
    pub fn new(mch_id: &str, out_trade_no: &str) -> Self {
        Self {
            mch_id: mch_id.to_string(),
            out_trade_no: out_trade_no.to_string(),
        }
    }

    pub fn req_path(&self) -> String {
        format!(
            "/v3/pay/transactions/out-trade-no/{}/reverse",
            self.out_trade_no
        )
    }
}

/// 撤销订单响应 V3（成功时返回空内容，状态码204）
/// 这里提供一个通用结构用于解析可能的错误响应
#[derive(Debug, Clone, Deserialize)]
pub struct OrderReverseResponseV3 {
    /// 撤销结果
    #[serde(default)]
    pub result: Option<String>,
}

// ==================== V3 申请交易账单 ====================

/// 申请交易账单请求 V3
#[derive(Debug, Clone)]
pub struct TradeBillRequestV3 {
    /// 账单日期，格式：yyyy-MM-DD
    pub bill_date: String,
    /// 账单类型：ALL, SUCCESS, REFUND
    pub bill_type: Option<BillType>,
    /// 压缩类型：GZIP
    pub tar_type: Option<String>,
}

impl TradeBillRequestV3 {
    pub fn new(bill_date: &str) -> Self {
        Self {
            bill_date: bill_date.to_string(),
            bill_type: None,
            tar_type: None,
        }
    }

    pub fn bill_type(mut self, bill_type: BillType) -> Self {
        self.bill_type = Some(bill_type);
        self
    }

    pub fn tar_type(mut self, tar_type: &str) -> Self {
        self.tar_type = Some(tar_type.to_string());
        self
    }
}

/// 申请交易账单响应 V3
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeBillResponseV3 {
    /// 哈希类型
    pub hash_type: String,
    /// 哈希值
    pub hash_value: String,
    /// 下载地址
    pub download_url: String,
}
