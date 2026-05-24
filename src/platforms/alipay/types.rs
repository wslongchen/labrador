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
use serde::{Deserialize, Serialize};

/// 支付宝支付通知响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AlipayNotifyResponse {
    /// 通知时间
    pub notify_time: String,
    /// 通知类型
    pub notify_type: String,
    /// 通知ID
    pub notify_id: String,
    /// 编码格式
    pub charset: String,
    /// 接口版本
    pub version: String,
    /// 签名类型
    pub sign_type: String,
    /// 签名
    pub sign: String,
    /// 授权应用ID
    pub auth_app_id: String,
    /// 支付宝交易号
    pub trade_no: String,
    /// 应用ID
    pub app_id: String,
    /// 商户订单号
    pub out_trade_no: String,
    /// 商户业务号
    pub out_biz_no: Option<String>,
    /// 买家支付宝账号ID
    pub buyer_id: Option<String>,
    /// 卖家支付宝账号ID
    pub seller_id: Option<String>,
    /// 交易状态
    pub trade_status: Option<String>,
    /// 订单金额
    #[serde(default)]
    pub total_amount: Option<f64>,
    /// 实收金额
    #[serde(default)]
    pub receipt_amount: Option<f64>,
    /// 开票金额
    #[serde(default)]
    pub invoice_amount: Option<f64>,
    /// 买家实付金额
    #[serde(default)]
    pub buyer_pay_amount: Option<f64>,
    /// 集分宝支付金额
    #[serde(default)]
    pub point_amount: Option<f64>,
    /// 总退款金额
    #[serde(default)]
    pub refund_fee: Option<f64>,
    /// 订单标题
    pub subject: Option<String>,
    /// 商品描述
    pub body: Option<String>,
    /// 交易创建时间
    pub gmt_create: Option<String>,
    /// 交易付款时间
    pub gmt_payment: Option<String>,
    /// 交易退款时间
    pub gmt_refund: Option<String>,
    /// 交易结束时间
    pub gmt_close: Option<String>,
    /// 支付金额信息
    pub fund_bill_list: Option<String>,
    /// 优惠券信息
    pub voucher_detail_list: Option<String>,
    /// 回传参数
    pub passback_params: Option<String>,
}