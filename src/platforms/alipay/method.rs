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
use std::collections::BTreeMap;

/// 请求参数持有者
#[derive(Debug, Clone)]
pub struct RequestParametersHolder {
    /// 协议必须参数
    pub protocol_must_params: BTreeMap<String, String>,
    /// 协议可选参数
    pub protocol_opt_params: BTreeMap<String, String>,
    /// 应用参数
    pub application_params: BTreeMap<String, String>,
}

impl Default for RequestParametersHolder {
    fn default() -> Self {
        Self::new()
    }
}

impl RequestParametersHolder {
    /// 创建新的请求参数持有者
    pub fn new() -> Self {
        Self {
            protocol_must_params: BTreeMap::new(),
            protocol_opt_params: BTreeMap::new(),
            application_params: BTreeMap::new(),
        }
    }

    /// 设置协议必须参数
    pub fn set_protocal_must_params(&mut self, params: BTreeMap<String, String>) {
        self.protocol_must_params = params;
    }

    /// 设置协议可选参数
    pub fn set_protocal_opt_params(&mut self, params: BTreeMap<String, String>) {
        self.protocol_opt_params = params;
    }

    /// 设置应用参数
    pub fn set_application_params(&mut self, params: BTreeMap<String, String>) {
        self.application_params = params;
    }

    /// 获取排序后的参数字符串
    pub fn get_signature_content(&self) -> String {
        let mut all_params = BTreeMap::new();
        all_params.extend(self.protocol_must_params.clone());
        all_params.extend(self.protocol_opt_params.clone());
        all_params.extend(self.application_params.clone());

        // 移除签名和空值参数
        all_params.retain(|k, v| k != "sign" && !v.is_empty());

        // 按照字母顺序排序并拼接
        let mut params_vec: Vec<(&String, &String)> = all_params.iter().collect();
        params_vec.sort_by(|a, b| a.0.cmp(b.0));

        params_vec
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("&")
    }

    /// 获取排序后的参数Map
    pub fn get_sorted_map(&self) -> BTreeMap<String, String> {
        let mut all_params = BTreeMap::new();
        all_params.extend(self.protocol_must_params.clone());
        all_params.extend(self.protocol_opt_params.clone());
        all_params.extend(self.application_params.clone());

        // 移除签名和空值参数
        all_params.retain(|k, v| k != "sign" && !v.is_empty());

        // 按照字母顺序排序
        let mut sorted = BTreeMap::new();
        let mut keys: Vec<String> = all_params.keys().cloned().collect();
        keys.sort();

        for key in keys {
            if let Some(value) = all_params.get(&key) {
                sorted.insert(key, value.clone());
            }
        }

        sorted
    }
}

/// 支付宝API方法
#[derive(Debug, Clone)]
pub enum AlipayMethod {
    /// 统一收单下单并支付页面接口
    TradePagePay,
    /// 统一收单手机网站支付接口
    TradeWapPay,
    /// App支付接口
    TradeAppPay,
    /// 统一收单交易支付接口
    TradePay,
    /// 统一收单线下交易预创建
    TradePrecreate,
    /// 统一收单交易创建接口
    TradeCreate,
    /// 统一收单线下交易查询
    TradeQuery,
    /// 统一收单交易关闭接口
    TradeClose,
    /// 统一收单交易退款接口
    TradeRefund,
    /// 统一收单交易退款查询
    TradeFastpayRefundQuery,
    /// 统一收单交易撤销接口
    TradeCancel,
    /// 统一收单交易结算接口
    TradeOrderSettle,
    /// 换取授权访问令牌
    SystemOauthToken,
    /// 换取应用授权令牌
    OpenAuthTokenApp,
    /// 支付宝公钥证书下载
    OpenAppAlipaycertDownload,
    /// 查询账单下载地址
    BillDownloadUrlQuery,
    /// 自定义方法
    Custom(String),
}

impl Default for AlipayMethod {
    fn default() -> Self {
        Self::Custom("unknown".to_string())
    }
}

impl AlipayMethod {
    /// 获取方法字符串
    pub fn as_str(&self) -> &str {
        match self {
            Self::TradePagePay => "alipay.trade.page.pay",
            Self::TradeWapPay => "alipay.trade.wap.pay",
            Self::TradeAppPay => "alipay.trade.app.pay",
            Self::TradePay => "alipay.trade.pay",
            Self::TradePrecreate => "alipay.trade.precreate",
            Self::TradeCreate => "alipay.trade.create",
            Self::TradeQuery => "alipay.trade.query",
            Self::TradeClose => "alipay.trade.close",
            Self::TradeRefund => "alipay.trade.refund",
            Self::TradeFastpayRefundQuery => "alipay.trade.fastpay.refund.query",
            Self::TradeCancel => "alipay.trade.cancel",
            Self::TradeOrderSettle => "alipay.trade.order.settle",
            Self::SystemOauthToken => "alipay.system.oauth.token",
            Self::OpenAuthTokenApp => "alipay.open.auth.token.app",
            Self::OpenAppAlipaycertDownload => "alipay.open.app.alipaycert.download",
            Self::BillDownloadUrlQuery => "alipay.data.dataservice.bill.downloadurl.query",
            Self::Custom(method) => method.as_str(),
        }
    }

    /// 获取响应Key
    pub fn response_key(&self) -> String {
        match self {
            Self::TradePagePay => "alipay_trade_page_pay_response".to_string(),
            Self::TradeWapPay => "alipay_trade_wap_pay_response".to_string(),
            Self::TradeAppPay => "alipay_trade_app_pay_response".to_string(),
            Self::TradePay => "alipay_trade_pay_response".to_string(),
            Self::TradePrecreate => "alipay_trade_precreate_response".to_string(),
            Self::TradeCreate => "alipay_trade_create_response".to_string(),
            Self::TradeQuery => "alipay_trade_query_response".to_string(),
            Self::TradeClose => "alipay_trade_close_response".to_string(),
            Self::TradeRefund => "alipay_trade_refund_response".to_string(),
            Self::TradeFastpayRefundQuery => {
                "alipay_trade_fastpay_refund_query_response".to_string()
            }
            Self::TradeCancel => "alipay_trade_cancel_response".to_string(),
            Self::TradeOrderSettle => "alipay_trade_order_settle_response".to_string(),
            Self::SystemOauthToken => "alipay_system_oauth_token_response".to_string(),
            Self::OpenAuthTokenApp => "alipay_open_auth_token_app_response".to_string(),
            Self::OpenAppAlipaycertDownload => {
                "alipay_open_app_alipaycert_download_response".to_string()
            }
            Self::BillDownloadUrlQuery => {
                "alipay_data_dataservice_bill_downloadurl_query_response".to_string()
            }
            Self::Custom(method) => format!("{}_response", method.replace(".", "_")),
        }
    }
}
