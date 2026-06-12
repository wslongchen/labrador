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
//! 支付宝常量定义

/// 支付宝API基础URL
pub const API_BASE_URL: &str = "https://openapi.alipay.com/gateway.do";

/// 支付宝沙箱API基础URL
pub const SANDBOX_API_BASE_URL: &str = "https://openapi.alipaydev.com/gateway.do";

/// 签名类型参数名
pub static SIGN_TYPE: &str = "sign_type";

/// 错误响应键
pub static ERROR_RESPONSE_KEY: &str = "error_response";

/// RSA签名类型
pub const SIGN_TYPE_RSA: &str = "RSA";

/// RSA2签名类型
pub const SIGN_TYPE_RSA2: &str = "RSA2";

/// SM2签名类型
pub static SIGN_TYPE_SM2: &str = "SM2";

/// SHA1算法
pub static SHA_TYPE: &str = "SHA1";

/// SHA256算法
pub static SHA_TYPE256: &str = "SHA256";

/// 签名算法
pub static SIGN_ALGORITHMS: &str = "SHA1WithRSA";

/// SHA256RSA签名算法
pub static SIGN_SHA256RSA_ALGORITHMS: &str = "SHA256WithRSA";

/// AES加密类型
pub const ENCRYPT_TYPE_AES: &str = "AES";

/// 应用ID参数名
pub static APP_ID: &str = "app_id";

/// 目标应用ID参数名
pub static TARGET_APP_ID: &str = "target_app_id";

/// 格式参数名
pub static FORMAT: &str = "format";

/// 方法参数名
pub static METHOD: &str = "method";

/// 时间戳参数名
pub static TIMESTAMP: &str = "timestamp";

/// 版本参数名
pub static VERSION: &str = "version";

/// 签名参数名
pub static SIGN: &str = "sign";

/// 支付宝SDK参数名
pub static ALIPAY_SDK: &str = "alipay_sdk";

/// 访问令牌参数名
pub static ACCESS_TOKEN: &str = "auth_token";

/// 应用授权令牌参数名
pub static APP_AUTH_TOKEN: &str = "app_auth_token";

/// 终端类型参数名
pub static TERMINAL_TYPE: &str = "terminal_type";

/// 终端信息参数名
pub static TERMINAL_INFO: &str = "terminal_info";

/// 字符集参数名
pub static CHARSET: &str = "charset";

/// 通知地址参数名
pub static NOTIFY_URL: &str = "notify_url";

/// 返回地址参数名
pub static RETURN_URL: &str = "return_url";

/// 加密类型参数名
pub static ENCRYPT_TYPE: &str = "encrypt_type";

/// 应用证书序列号参数名
pub static APP_CERT_SN: &str = "app_cert_sn";

/// 支付宝证书序列号参数名
pub static ALIPAY_CERT_SN: &str = "alipay_cert_sn";

/// 支付宝根证书序列号参数名
pub static ALIPAY_ROOT_CERT_SN: &str = "alipay_root_cert_sn";

/// 业务内容参数名
pub static BIZ_CONTENT_KEY: &str = "biz_content";

/// UTF-8字符集
pub const CHARSET_UTF8: &str = "UTF-8";

/// GBK字符集
pub static CHARSET_GBK: &str = "GBK";

/// JSON格式
pub const FORMAT_JSON: &str = "json";

/// XML格式
pub static FORMAT_XML: &str = "xml";

/// 时间格式
pub static FORMAT_TIME: &str = "%Y-%m-%d %H:%M:%S";

/// 产品码参数名
pub static PROD_CODE: &str = "prod_code";

/// 支付相关常量
pub mod pay {
    /// JSAPI支付产品码
    pub static PRODUCT_CODE_JSAPI_PAY: &str = "JSAPI_PAY";
    /// 周期扣款产品码
    pub static PRODUCT_CODE_CYCLE_PAY: &str = "CYCLE_PAY_AUTH";
    /// 商家扣款产品码
    pub static PRODUCT_CODE_DEDUCT_PAY: &str = "GENERAL_WITHHOLDING";
    /// 电脑网站支付产品码
    pub const PRODUCT_CODE_PAGE_PAY: &str = "FAST_INSTANT_TRADE_PAY";
    /// 扫码支付产品码
    pub const PRODUCT_CODE_QR_CODE_OFFLINE: &str = "QR_CODE_OFFLINE";

    /// 手机网站支付产品码
    pub const PRODUCT_CODE_WAP_PAY: &str = "QUICK_WAP_WAY";

    /// App支付产品码
    pub const PRODUCT_CODE_APP_PAY: &str = "QUICK_MSECURITY_PAY";

    /// 当面付产品码
    pub const PRODUCT_CODE_FACE_TO_FACE: &str = "FACE_TO_FACE_PAYMENT";

    /// 交易状态：等待买家付款
    pub const TRADE_STATUS_WAIT_BUYER_PAY: &str = "WAIT_BUYER_PAY";

    /// 交易状态：交易关闭
    pub const TRADE_STATUS_CLOSED: &str = "TRADE_CLOSED";

    /// 交易状态：交易成功
    pub const TRADE_STATUS_SUCCESS: &str = "TRADE_SUCCESS";

    /// 交易状态：交易结束
    pub const TRADE_STATUS_FINISHED: &str = "TRADE_FINISHED";
}

/// 开放平台相关常量
pub mod open {
    /// 授权类型：授权码
    pub const GRANT_TYPE_AUTHORIZATION_CODE: &str = "authorization_code";

    /// 授权类型：刷新令牌
    pub const GRANT_TYPE_REFRESH_TOKEN: &str = "refresh_token";

    /// 授权范围：用户基础信息
    pub const SCOPE_USER_BASE: &str = "auth_base";

    /// 授权范围：用户信息
    pub const SCOPE_USER_INFO: &str = "auth_user";
}

/// 小程序相关常量
pub mod miniapp {
    /// 小程序模板消息状态：发送成功
    pub const TEMPLATE_MESSAGE_STATUS_SUCCESS: &str = "success";

    /// 小程序模板消息状态：发送失败
    pub const TEMPLATE_MESSAGE_STATUS_FAIL: &str = "fail";

    /// 小程序模板消息状态：用户拒收
    pub const TEMPLATE_MESSAGE_STATUS_USER_BLOCK: &str = "user_block";
}
