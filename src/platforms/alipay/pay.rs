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

//! 支付宝支付模块
//!
//! 提供支付宝各种支付方式的接口实现

use crate::errors::{LabradorResult};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use crate::alipay::{constants, AlipayBizRequest};
use crate::alipay::method::AlipayMethod;
use crate::platforms::alipay::AlipayResponse;
use super::client::{AlipayClient};

/// 支付服务
pub struct AlipayPayService<'a> {
    client: &'a AlipayClient,
}

impl <'a> AlipayPayService<'a> {
    /// 创建新的支付服务
    pub fn new(client: &'a AlipayClient) -> Self {
        Self { client }
    }


    /// 电脑网站支付
    pub async fn page_pay(
        &self,
        mut request: AlipayBizRequest<AlipayTradePagePayModel>,
    ) -> LabradorResult<String> {
        request.prod_code = Some(constants::pay::PRODUCT_CODE_PAGE_PAY.to_string());
        request.method = AlipayMethod::TradePagePay;
        self.client.execute_page_request(
            request,
            None,
        )
    }

    /// JSAPI支付（小程序/生活号支付）
    /// 使用 alipay.trade.create 接口创建订单并获取 trade_no，
    /// 然后通过 alipay.trade.pay 发起支付。
    pub async fn jsapi_pay(&self, mut request: AlipayBizRequest<AlipayTradeJsapiPayModel>) -> LabradorResult<JsapiPayResponse> {
        request.prod_code = Some(constants::pay::PRODUCT_CODE_JSAPI_PAY.to_string());
        request.method = AlipayMethod::TradePay;
        let response: AlipayResponse<JsapiPayResponse> = self.client
            .request(request, None, None, None)
            .await?;
        response.into_result()
    }

    /// 统一收单交易创建 (alipay.trade.create)
    /// 商户通过该接口创建订单，获取支付宝交易号 trade_no。
    /// 适用于小程序支付等需要先创建订单再发起支付的场景。
    pub async fn trade_create(
        &self,
        mut request: AlipayBizRequest<AlipayTradeCreateModel>,
    ) -> LabradorResult<TradeCreateResponse> {
        request.method = AlipayMethod::TradeCreate;
        let response: AlipayResponse<TradeCreateResponse> = self.client
            .request(request, None, None, None)
            .await?;
        response.into_result()
    }

    /// 查询账单下载地址 (alipay.data.dataservice.bill.downloadurl.query)
    pub async fn bill_download(
        &self,
        bill_type: &str,
        bill_date: &str,
    ) -> LabradorResult<BillDownloadResponse> {
        let mut request = AlipayBizRequest::new();
        let mut biz_content = BTreeMap::new();
        biz_content.insert("bill_type".to_string(), bill_type.to_string());
        biz_content.insert("bill_date".to_string(), bill_date.to_string());
        request.biz_model = Some(biz_content);
        request.method = AlipayMethod::BillDownloadUrlQuery;
        let response: AlipayResponse<BillDownloadResponse> = self.client
            .request(request, None, None, None)
            .await?;
        response.into_result()
    }

    /// 移动网页支付

    /// 手机网站支付
    pub async fn wap_pay(
        &self,
        mut request: AlipayBizRequest<AlipayTradeWapPayModel>,
    ) -> LabradorResult<String> {
        request.prod_code = Some(constants::pay::PRODUCT_CODE_WAP_PAY.to_string());
        request.method = AlipayMethod::TradeWapPay;

        self.client.execute_page_request(
            request,
            None,
        )
    }

    /// App支付
    pub async fn app_pay(
        &self,
        mut request: AlipayBizRequest<AlipayTradeAppPayModel>,
    ) -> LabradorResult<String> {
        request.prod_code = Some(constants::pay::PRODUCT_CODE_APP_PAY.to_string());
        request.method = AlipayMethod::TradeAppPay;

        self.client.execute_page_request(
            request,
            None,
        )
    }

    /// 当面付（条码支付）
    pub async fn face_to_face_pay(
        &self,
        mut request: AlipayBizRequest<AlipayFaceOrderPayModel>,
    ) -> LabradorResult<FaceToFacePayResponse> {
        request.prod_code = Some(constants::pay::PRODUCT_CODE_FACE_TO_FACE.to_string());
        request.method = AlipayMethod::TradePay;
        let response: AlipayResponse<FaceToFacePayResponse> = self.client
            .request(request, None, None, None)
            .await?;

        response.into_result()
    }


    /// 周期扣款
    pub async fn cycle_pay(&self, mut request: AlipayBizRequest<AlipayCycleOrderPayModel>) -> LabradorResult<CycleOrderPayResponse> {
        request.prod_code = Some(constants::pay::PRODUCT_CODE_CYCLE_PAY.to_string());
        request.method = AlipayMethod::TradePay;
        let response: AlipayResponse<CycleOrderPayResponse> = self.client
            .request(request, None, None, None)
            .await?;
        response.into_result()
    }

    /// 商家扣款
    pub async fn deduct_pay(&self, mut request: AlipayBizRequest<AlipayCycleOrderPayModel>) -> LabradorResult<DeductPayResponse> {
        request.prod_code = Some(constants::pay::PRODUCT_CODE_CYCLE_PAY.to_string());
        request.method = AlipayMethod::TradePay;
        let response: AlipayResponse<DeductPayResponse> = self.client
            .request(request, None, None, None)
            .await?;
        response.into_result()
    }

    /// 预下单（生成二维码）
    pub async fn precreate(
        &self,
        mut request: AlipayBizRequest<AlipayTradePreCreateModel>
    ) -> LabradorResult<PrecreateResponse> {
        request.prod_code = Some(constants::pay::PRODUCT_CODE_QR_CODE_OFFLINE.to_string());
        request.method = AlipayMethod::TradePrecreate;

        let response: AlipayResponse<PrecreateResponse> = self.client
            .request(request, None, None, None)
            .await?;

        response.into_result()
    }

    /// 查询订单
    pub async fn query(
        &self,
        mut request: AlipayBizRequest<AlipayTradeQueryModel>,
    ) -> LabradorResult<OrderQueryResponse> {
        request.method = AlipayMethod::TradeQuery;
        let response: AlipayResponse<OrderQueryResponse> = self.client
            .request(request, None, None, None)
            .await?;
        response.into_result()
    }

    /// 关闭订单
    pub async fn close(
        &self,
        mut request: AlipayBizRequest<AlipayTradeCloseModel>,
    ) -> LabradorResult<OrderCloseResponse> {
        request.method = AlipayMethod::TradeClose;
        let response: AlipayResponse<OrderCloseResponse> = self.client
            .request(request, None, None, None)
            .await?;

        response.into_result()
    }

    /// 申请退款
    pub async fn refund(
        &self,
        mut request: AlipayBizRequest<AlipayTradeRefundModel>,
    ) -> LabradorResult<RefundResponse> {
        request.method = AlipayMethod::TradeRefund;
        let response: AlipayResponse<RefundResponse> = self.client
            .request(request, None, None, None)
            .await?;

        response.into_result()
    }

    /// 查询退款
    pub async fn query_refund(
        &self,
        out_trade_no: Option<String>,
        trade_no: Option<String>,
        out_request_no: String,
    ) -> LabradorResult<RefundQueryResponse> {
        let mut request = AlipayBizRequest::new();
        let mut biz_content = BTreeMap::new();
        biz_content.insert("out_request_no".to_string(), out_request_no);

        if let Some(no) = out_trade_no {
            biz_content.insert("out_trade_no".to_string(), no);
        }

        if let Some(no) = trade_no {
            biz_content.insert("trade_no".to_string(), no);
        }
        request.biz_model = Some(biz_content);
        request.method = AlipayMethod::TradeFastpayRefundQuery;
        let response: AlipayResponse<RefundQueryResponse> = self.client
            .request(request, None, None, None)
            .await?;

        response.into_result()
    }

    /// 撤销订单
    pub async fn cancel(
        &self,
        mut request: AlipayBizRequest<AlipayTradeCancelModel>,
    ) -> LabradorResult<OrderCancelResponse> {
        request.method = AlipayMethod::TradeCancel;
        let response: AlipayResponse<OrderCancelResponse> = self.client
            .request(request, None, None, None)
            .await?;
        response.into_result()
    }

    /// # 统一收单交易结算 (alipay.trade.order.settle)
    /// 详见 [文档](https://opendocs.alipay.com/apis/api_1/alipay.trade.order.settle)
    ///
    /// 用于在线下场景，交易完成后进行资金结算。
    pub async fn settle(
        &self,
        mut request: AlipayBizRequest<AlipayTradeOrderSettleModel>,
    ) -> LabradorResult<TradeOrderSettleResponse> {
        request.method = AlipayMethod::TradeOrderSettle;
        let response: AlipayResponse<TradeOrderSettleResponse> = self.client
            .request(request, None, None, None)
            .await?;
        response.into_result()
    }

}

//----------------------------------------------------------------------------------------------------------------------------

/// 统一收单交易创建 (alipay.trade.create)
#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayTradeCreateModel {
    /// 商户订单号
    pub out_trade_no: String,
    /// 订单总金额，单位元
    pub total_amount: f64,
    /// 订单标题
    pub subject: String,
    /// 买家支付宝用户ID（2088开头）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_id: Option<String>,
    /// 买家支付宝用户唯一标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_open_id: Option<String>,
    /// 卖家支付宝用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_id: Option<String>,
    /// 订单附加信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// 产品码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    /// 订单包含的商品列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_detail: Option<Vec<GoodsDetail>>,
    /// 业务扩展参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extend_params: Option<ExtendParams>,
    /// 订单绝对超时时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_expire: Option<String>,
    /// 订单相对超时时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_express: Option<String>,
    /// 可打折金额
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discountable_amount: Option<f64>,
    /// 不可打折金额
    #[serde(skip_serializing_if = "Option::is_none")]
    pub undiscountable_amount: Option<f64>,
    /// 商户门店编号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    /// 商户操作员编号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    /// 商户机具终端编号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal_id: Option<String>,
    /// 业务扩展参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_params: Option<String>,
    /// 公用回传参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passback_params: Option<String>,
    /// 商户原始订单号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_order_no: Option<String>,
    /// 查询选项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_options: Option<Vec<String>>,
}

/// 交易创建响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeCreateResponse {
    /// 商户订单号
    pub out_trade_no: Option<String>,
    /// 支付宝交易号
    pub trade_no: Option<String>,
}

/// 账单下载响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillDownloadResponse {
    /// 账单下载地址（30秒有效）
    pub bill_download_url: Option<String>,
    /// 账单文件大小，单位字节
    pub bill_file_size: Option<String>,
}

/// 统一收单交易结算接口 (alipay.trade.order.settle)
#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayTradeOrderSettleModel {
    /// 结算请求流水号，由商家自定义，需保证唯一
    pub out_request_no: String,
    /// 支付宝交易号，和商户订单号不能同时为空
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_no: Option<String>,
    /// 结算的金额明细列表
    pub royalty_parameters: Vec<OpenApiRoyaltyDetailInfoPojo>,
    /// 操作员ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    /// 扩展参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extend_params: Option<RoyaltySettleExtendParams>,
}

/// 分账扩展参数
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RoyaltySettleExtendParams {
    /// 是否解冻剩余资金
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfreeze_default: Option<String>,
}

/// 交易结算响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeOrderSettleResponse {
    /// 支付宝交易号
    pub trade_no: Option<String>,
    /// 结算请求流水号
    pub settle_no: Option<String>,
}

/// 统一收单交易撤销接口
#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayTradeCancelModel {
    /// 商户订单号。
    /// 订单支付时传入的商户订单号，商家自定义且保证商家系统中唯一。与支付宝交易号 trade_no 不能同时为空。
    pub out_trade_no: Option<String>,
    /// 支付宝交易号。
    /// 和商户订单号 out_trade_no 不能同时为空。
    pub trade_no: Option<String>,
}


#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayTradeCloseModel {
    /// 订单支付时传入的商户订单号,和支付宝交易号不能同时为空。 trade_no,out_trade_no如果同时存在优先取trade_no
    pub out_trade_no: Option<String>,
    /// 该交易在支付宝系统中的交易流水号。最短 16 位，最长 64 位。和out_trade_no不能同时为空，如果同时传了 out_trade_no和 trade_no，则以 trade_no为准
    pub trade_no: Option<String>,
    /// 商家操作员编号 id，由商家自定义。
    pub operator_id: Option<String>,
}


/// 当面付响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceToFacePayResponse {
    /// 商户订单号
    pub out_trade_no: Option<String>,
    /// 订单金额
    pub total_amount: Option<String>,
    /// 实收金额
    pub receipt_amount: Option<String>,
    /// 支付时间
    pub gmt_payment: Option<String>,
    /// 资金渠道
    pub fund_bill_list: Option<Vec<FundBill>>,
    /// 买家在支付宝的用户id
    pub buyer_user_id: Option<String>,
    /// 买家支付宝用户唯一标识
    pub buyer_open_id: Option<String>,
    /// 平台优惠金额
    pub discount_amount: Option<String>,
    /// 商家优惠金额
    pub mdiscount_amount: Option<String>,
    /// 支付宝交易号
    pub trade_no: Option<String>,
    /// 买家支付宝账号
    pub buyer_logon_id: Option<String>,
    /// 买家实付金额
    pub buyer_pay_amount: Option<String>,
    /// 使用集分宝付款的金额
    pub point_amount: Option<String>,
    /// 交易中可给用户开具发票的金额
    pub invoice_amount: Option<String>,
    /// 发生支付交易的商户门店名称
    pub store_name: Option<String>,
    ///本次交易支付所使用的单品券优惠的商品优惠信息。
    /// 只有在query_options中指定时才返回该字段信息。
    pub discount_goods_detail: Option<String>,
    /// 本交易支付时使用的所有优惠券信息。
    pub voucher_detail_list: Option<Vec<VoucherDetail>>,





}

/// 预下单响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrecreateResponse {
    /// 商户订单号
    pub out_trade_no: Option<String>,
    /// 二维码内容
    pub qr_code: Option<String>,
}

/// 订单查询响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderQueryResponse {
    /// 商户订单号
    pub out_trade_no: Option<String>,
    /// 支付宝交易号
    pub trade_no: Option<String>,
    /// 交易状态
    pub trade_status: Option<String>,
    /// 订单金额
    pub total_amount: Option<String>,
    /// 资金渠道
    pub fund_bill_list: Option<Vec<FundBill>>,
    /// 买家在支付宝的用户id
    pub buyer_user_id: Option<String>,
    pub buyer_open_id: Option<String>,
    /// 发送付款时间
    pub send_pay_date: Option<String>,
    /// 实收金额
    pub receipt_amount: Option<String>,
    /// 商户门店编号
    pub store_id: Option<String>,
    /// 商户机具终端编号
    pub terminal_id: Option<String>,
    /// 请求交易支付中的商户店铺的名称
    pub store_name: Option<String>,
    /// 平台优惠金额
    pub discount_amount: Option<String>,
    /// 交易额外信息，特殊场景下与支付宝约定返回。
    /// json格式。
    pub ext_infos: Option<String>,
    /// 买家用户类型。CORPORATE:企业用户；PRIVATE:个人用户。
    /// 【枚举值】
    /// 企业用户: CORPORATE
    /// 个人用户: PRIVATE
    pub buyer_user_type: Option<String>,
    /// 商家优惠金额
    pub mdiscount_amount: Option<String>,
    /// 买家支付宝账号
    pub buyer_logon_id: Option<String>,
    /// 买家实付金额
    pub buyer_pay_amount: Option<String>,
    /// 开票金额
    pub invoice_amount: Option<String>,
    /// 积分支付的金额，单位为元，两位小数。
    /// 该金额代表该笔交易中用户使用积分支付的金额，比如集分宝或者支付宝实时优惠等
    pub point_amount: Option<String>,




}

/// 订单关闭响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCloseResponse {
    /// 商户订单号
    pub out_trade_no: Option<String>,
    /// 支付宝交易号
    pub trade_no: Option<String>,
}

/// 退款响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundResponse {
    /// 商户订单号
    pub out_trade_no: Option<String>,
    /// 支付宝交易号
    pub trade_no: Option<String>,
    /// 买家支付宝账号
    pub buyer_logon_id: Option<String>,
    /// 资金变化标志
    pub fund_change: Option<String>,
    /// 退款金额
    pub refund_fee: Option<String>,
    /// 退款使用的资金渠道。 只有在签约中指定需要返回资金明细，
    /// 或者入参的query_options中指定时才返回该字段信息。
    pub refund_detail_item_list: Option<Vec<FundBill>>,
    /// 交易在支付时候的门店名称
    pub store_name: Option<String>,
    /// 买家在支付宝的用户id
    pub buyer_user_id: Option<String>,
    /// 买家支付宝用户唯一标识
    pub buyer_open_id: Option<String>,
    /// 本次商户实际退回金额
    pub send_back_fee: Option<String>,
    /// 当用户使用芝麻信用先享后付时，且当前的操作为预授权撤销动作时，会返回该字段，代表当前撤销的预授权金额，单位元。
    /// 【必选条件】当用户使用芝麻信用先享后付时，且当前的操作为预授权撤销动作时，会返回该字段。
    pub pre_auth_cancel_fee: Option<String>,
    /// 本次请求退惠营宝金额。单位：元。
    pub refund_hyb_amount: Option<String>,
    /// 退费信息
    pub refund_charge_info_list: Option<Vec<RefundChargeInfo>>,
    pub refund_voucher_detail_list: Option<Vec<VoucherDetail>>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundChargeInfo {
    /// 实退费用。单位：元。
    pub refund_charge_fee: Option<String>,
    /// 签约费率
    pub switch_fee_rate: Option<String>,
    /// 收单手续费trade，花呗分期手续hbfq，其他手续费charge
    pub charge_type: Option<String>,
    /// 组合支付退费明细
    pub refund_sub_fee_detail_list: Option<Vec<RefundSubFee>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundSubFee {
    /// 实退费用。单位：元。
    pub refund_charge_fee: Option<String>,
    /// 签约费率
    pub switch_fee_rate: Option<String>,

}

/// 退款查询响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundQueryResponse {
    /// 商户订单号
    pub out_trade_no: Option<String>,
    /// 支付宝交易号
    pub trade_no: Option<String>,
    /// 退款请求号
    pub out_request_no: Option<String>,
    /// 订单金额
    pub total_amount: Option<String>,
    /// 退款金额
    pub refund_amount: Option<String>,
    /// 退款状态
    pub refund_status: Option<String>,
    /// 退款时间
    pub gmt_refund_pay: Option<String>,
    /// 分账明细信息，当前仅在直付通产品中返回。
    pub refund_royaltys: Option<Vec<RefundRoyaltyResult>>,
    /// 本次退款使用的资金渠道； 默认不返回该信息，需要在入参的query_options中指定"refund_detail_item_list"值时才返回该字段信息。
    pub refund_detail_item_list: Option<Vec<FundBill>>,
    /// 本次商户实际退回金额；单位：元。 默认不返回该信息，需要在入参的query_options中指定"refund_detail_item_list"值时才返回该字段信息。
    pub send_back_fee: Option<String>,
    /// 银行卡冲退信息； 默认不返回该信息，需要在入参的query_options中指定"deposit_back_info"值时才返回该字段信息。
    pub deposit_back_info: Option<DepositBackInfo>,
    /// 本交易支付时使用的所有优惠券信息。 只有在query_options中指定refund_voucher_detail_list时才返回该字段信息。
    /// 【必选条件】query_options中包含refund_voucher_detail_list时，才会返回券信息列表
    pub refund_voucher_detail_list: Option<Vec<VoucherDetail>>,
    /// 当用户使用芝麻信用先享后付时，且当前的操作为预授权撤销动作时，会返回该字段，代表当前撤销的预授权金额，单位元。
    /// 【必选条件】当用户使用芝麻信用先享后付时，且当前的操作为预授权撤销动作时，会返回该字段。
    pub pre_auth_cancel_fee: Option<String>,
    /// 本次退款金额中退惠营宝的金额。单位：元。
    pub refund_hyb_amount: Option<String>,
    /// 退费信息
    pub refund_charge_info_list: Option<Vec<RefundChargeInfo>>,
    /// 银行卡冲退信息列表。
    /// 默认不返回该信息，需要在入参的query_options中指定"deposit_back_info_list"值时才返回该字段信息。
    pub deposit_back_info_list: Option<Vec<DepositBackInfo>>,
}


#[derive(Debug, Clone, Deserialize,Serialize)]
pub struct DepositBackInfo {
    /// 是否存在银行卡冲退信息。
    pub has_deposit_back: Option<String>,
    /// 银行卡冲退状态。S-成功，F-失败，P-处理中。银行卡冲退失败，资金自动转入用户支付宝余额。
    pub dback_status: Option<String>,
    /// 银行卡冲退金额
    pub dback_amount: Option<String>,
    /// 银行响应时间，格式为yyyy-MM-dd HH:mm:ss
    pub bank_ack_time: Option<String>,
    /// 预估银行到账时间，格式为yyyy-MM-dd HH:mm:ss
    pub est_bank_receipt_time: Option<String>,
    /// 是否包含因公付资产
    pub is_use_enterprise_pay: Option<String>,
}

#[derive(Debug, Clone, Deserialize,Serialize)]
pub struct RefundRoyaltyResult {
    /// 退分账金额
    pub refund_amount: Option<String>,
    /// 分账类型.
    /// 普通分账为：transfer;
    /// 补差为：replenish;
    /// 为空默认为分账transfer;
    pub royalty_type: Option<String>,
    /// 退分账结果码
    pub result_code: Option<String>,
    /// 转出人支付宝账号对应用户ID
    pub trans_out: Option<String>,
    /// 转出人支付宝账号
    pub trans_out_email: Option<String>,
    /// 转入人支付宝账号对应用户ID
    pub trans_in: Option<String>,
    /// 转入人支付宝账号
    pub trans_in_email: Option<String>,
    /// 商户请求的转入账号
    pub ori_trans_in: Option<String>,
    /// 商户请求的转出账号
    pub ori_trans_out: Option<String>,
}

/// 订单撤销响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCancelResponse {
    /// 商户订单号
    pub out_trade_no: Option<String>,
    /// 支付宝交易号
    pub trade_no: Option<String>,
    /// 重试标志
    pub retry_flag: Option<String>,
    /// 执行动作
    pub action: Option<String>,
}

/// 资金渠道
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundBill {
    /// 资金渠道
    pub fund_channel: Option<String>,
    /// 资金类型
    pub fund_type: Option<String>,
    /// 金额
    pub amount: Option<String>,
    /// 实际金额
    pub real_amount: Option<String>,
}


#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayTradePagePayModel {
    /// 商户网站唯一订单号
    pub out_trade_no: String,
    /// 订单总金额。
    /// 单位为元，精确到小数点后两位，取值范围：[0.01,100000000] 。
    pub total_amount: f64,
    /// 订单标题。
    /// 注意：不可使用特殊字符，如 /，=，& 等。
    pub subject: String,
    /// 订单附加信息。
    /// 如果请求时传递了该参数，将在异步通知、对账单中原样返回，同时会在商户和用户的pc账单详情中作为交易描述展示
    pub body: Option<String>,
    /// 销售产品码，与支付宝签约的产品码名称。注：目前电脑支付场景下仅支持FAST_INSTANT_TRADE_PAY
    pub product_code: String,
    /// PC扫码支付的方式。
    /// <pre>
    /// 支持前置模式和跳转模式。
    /// 前置模式是将二维码前置到商户的订单确认页的模式。需要商户在自己的页面中以 iframe 方式请求支付宝页面。具体支持的枚举值有以下几种：
    /// 0：订单码-简约前置模式，对应 iframe 宽度不能小于600px，高度不能小于300px；
    /// 1：订单码-前置模式，对应iframe 宽度不能小于 300px，高度不能小于600px；
    /// 3：订单码-迷你前置模式，对应 iframe 宽度不能小于 75px，高度不能小于75px；
    /// 4：订单码-可定义宽度的嵌入式二维码，商户可根据需要设定二维码的大小。
    ///
    /// 跳转模式下，用户的扫码界面是由支付宝生成的，不在商户的域名下。支持传入的枚举值有：
    /// 2：订单码-跳转模式
    /// </pre>
    pub qr_pay_mode: Option<String>,
    /// 商户自定义二维码宽度。
    /// 注：qr_pay_mode=4时该参数有效
    pub qrcode_width: Option<String>,
    /// 用户付款中途退出返回商户网站的地址
    pub quit_url: Option<String>,
    /// 订单包含的商品列表信息，json格式，其它说明详见商品明细说明
    pub goods_detail: Option<GoodsDetail>,
    /// 业务扩展参数
    pub extend_params: Option<ExtendParams>,
    /// 订单绝对超时时间。
    /// 格式为yyyy-MM-dd HH:mm:ss。
    /// 注：time_expire和timeout_express两者只需传入一个或者都不传，两者均传入时，优先使用time_expire。
    pub time_expire: Option<String>,
    /// 商户传入业务信息，具体值要和支付宝约定，应用于安全，营销等参数直传场景，格式为json格式
    pub business_params: Option<String>,
    /// 优惠参数。为 JSON 格式。注：仅与支付宝协商后可用
    pub promo_params: Option<String>,
    /// 请求后页面的集成方式。
    /// <pre>
    /// 枚举值：
    /// ALIAPP：支付宝钱包内
    /// PCWEB：PC端访问
    /// 默认值为PCWEB。
    /// </pre>
    pub integration_type: Option<String>,
    /// 请求来源地址。如果使用ALIAPP的集成方式，用户中途取消支付会返回该地址。
    pub request_from_url: Option<String>,
    /// 商户门店编号。
    /// 指商户创建门店时输入的门店编号。
    pub store_id: Option<String>,
    /// 二级商户信息。
    /// 直付通模式和机构间连模式下必传，其它场景下不需要传入。。
    pub sub_merchant: Option<SubMerchantInfo>,
    /// 开票信息
    pub invoice_info: Option<AlipayInvoiceInfo>,
    /// 商户原始订单号，最大长度限制32位
    pub merchant_order_no: Option<String>,
}


/// 开票信息
#[derive(Debug, Serialize, Deserialize)]
pub struct AlipayInvoiceInfo {
    /// 商品的编号
    pub key_info: Option<InvoiceKeyInfo>,
    /// 开票内容
    /// 注：json数组格式
    pub details: String,
}

/// 开票信息
#[derive(Debug, Serialize, Deserialize)]
pub struct SubMerchantInfo {
    /// 间连受理商户的支付宝商户编号，通过间连商户入驻后得到。间连业务下必传，并且需要按规范传递受理商户编号。
    pub merchant_id: String,
    /// 二级商户编号类型。
    /// 枚举值：
    /// alipay:支付宝分配的间联商户编号；
    /// 目前仅支持alipay，默认可以不传。
    pub merchant_type: Option<String>,
}

/// 开票关键信息
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceKeyInfo {
    /// 该交易是否支持开票
    pub is_support_invoice: bool,
    /// 开票商户名称：商户品牌简称|商户门店简称
    pub invoice_merchant_name: String,
    /// 税号
    pub tax_num: String,
}

/// 商品详情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoodsDetail {
    /// 商品编号
    pub goods_id: String,
    /// 支付宝定义的统一商品编号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay_goods_id: Option<String>,
    /// 商品名称
    pub goods_name: String,
    /// 商品数量
    pub quantity: i32,
    /// 商品单价，单位元
    pub price: f64,
    /// 商品类目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_category: Option<String>,
    /// 商品描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// 商品展示地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_url: Option<String>,
}

/// 业务扩展参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendParams {
    /// 系统商编号
    pub sys_service_provider_id: Option<String>,
    /// 花呗分期数
    pub hb_fq_num: Option<String>,
    /// 花呗分期卖家承担手续费比例
    pub hb_fq_seller_percent: Option<String>,
    /// 行业数据回流信息
    pub industry_reflux_info: Option<String>,
    /// 卡类型
    pub card_type: Option<String>,
}



/// 周期扣款
/// 用户与商户签署周期扣款协议后，商户可通过本接口做后续免密代扣操作
#[derive(Debug, Serialize, Default, Deserialize)]
pub struct CyclePayRequest {
    /// 商户网站唯一订单号
    pub out_trade_no: String,
    /// 订单总金额。
    /// 单位为元，精确到小数点后两位，取值范围：[0.01,100000000] 。
    pub total_amount: f64,
    /// 订单标题。
    /// 注意：不可使用特殊字符，如 /，=，& 等。
    pub subject: String,
    /// <pre>
    /// 产品码。
    /// 商家和支付宝签约的产品码。 枚举值：CYCLE_PAY_AUTH：周期扣款产品；GENERAL_WITHHOLDING：代扣产品；注意：非当面付产品使用本接口时，本参数必填。请传入对应产品码。
    /// </pre>
    pub product_code: Option<String>,
    /// 代扣信息。
    /// 代扣业务需要传入的协议相关信息，使用本参数传入协议号后scene和auth_code不需要再传值。
    pub agreement_params: Option<AgreementParams>,
    /// 卖家支付宝用户ID。
    /// <pre>
    /// 当需要指定收款账号时，通过该参数传入，如果该值为空，则默认为商户签约账号对应的支付宝用户ID。
    /// 收款账号优先级规则：门店绑定的收款账户>请求传入的seller_id>商户签约账号对应的支付宝用户ID；
    /// 注：直付通和机构间联场景下seller_id无需传入或者保持跟pid一致；
    /// 如果传入的seller_id与pid不一致，需要联系支付宝小二配置收款关系；
    /// 支付宝预授权和新当面资金授权场景下必填。
    /// </pre>
    pub seller_id: Option<String>,
    /// 订单包含的商品列表信息，json格式，其它说明详见商品明细说明
    pub goods_detail: Option<GoodsDetail>,
    /// 业务扩展参数
    pub extend_params: Option<ExtendParams>,
    /// 优惠明细参数，通过此属性补充营销参数
    pub promo_params: Option<PromoParam>,
    /// 支付相关参数
    pub pay_params: Option<PayParams>,
    /// 返回参数选项。
    /// 商户通过传递该参数来定制同步需要额外返回的信息字段，数组格式。如：["fund_bill_list","voucher_detail_list","discount_goods_detail"]
    pub query_options: Option<Vec<String>>,
    /// 回调地址
    pub notify_url: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AgreementParams {
    /// 支付宝系统中用以唯一标识用户签约记录的编号（用户签约成功后的协议号 ）
    pub agreement_no: Option<String>,
    /// 鉴权确认码，在需要做支付鉴权校验时，该参数不能为空
    pub auth_confirm_no: Option<String>,
    /// 鉴权申请token，其格式和内容，由支付宝定义。在需要做支付鉴权校验时，该参数不能为空。
    pub apply_token: Option<String>,
    /// 商户代扣扣款许可
    pub deduct_permission: Option<String>,
}


/// 外部指定买家
#[derive(Debug, Serialize, Deserialize)]
pub struct PromoParam {
    /// 存在延迟扣款这一类的场景，用这个时间表明用户发生交易的时间，比如说，在公交地铁场景，用户刷码出站的时间，和商户上送交易的时间是不一样的。
    pub actual_order_time: Option<String>,
}


/// 支付相关参数
#[derive(Debug, Serialize, Deserialize)]
pub struct PayParams {
    /// 普通异步支付, 传入该参数时，如果满足受理条件，会先同步受理支付，然后在异步调度推进支付
    /// <pre>
    /// NORMAL_ASYNC: 普通异步，受理成功之后，会在交易关单之前通过一定的策略重试
    /// NEAR_REAL_TIME_ASYNC: 准实时异步，受理成功之后，会准实时发起1次调度
    /// </pre>
    pub async_type: Option<String>,
    /// 重试类型，当async_type传入NORMAL_ASYNC时，可以设置该参数，选择是否要重试，retry_type 可选，不设置时，可重试。
    /// <pre>
    /// ● NONE_AND_CLOSETRADE：不重试，支付请求只会被执行1次，执行完成后如果交易未成功，会关闭交易
    /// ● NONE：不重试，支付请求只会被执行1次，执行完成后，不做任何处理。交易到达了timeout_express指定的时间后，关闭交易。
    /// ● RETY: 重试，支付请求在超时关单前，会按照策略重试
    /// </pre>
    pub retry_type: Option<String>,
    /// 是否异步支付，传入true时，表明本次期望走异步支付，会先将支付请求受理下来，再异步推进。商户可以通过交易的异步通知或者轮询交易的状态来确定最终的交易结果。
    /// 只在代扣场景下有效，其它场景无需传入。
    pub is_async_pay: Option<bool>,
    /// 可打折金额。
    /// <pre>
    /// 参与优惠计算的金额，单位为元，精确到小数点后两位，取值范围[0.01,100000000]。
    /// 如果同时传入了【可打折金额】、【不可打折金额】和【订单总金额】，则必须满足如下条件：【订单总金额】=【可打折金额】+【不可打折金额】。
    /// 如果订单金额全部参与优惠计算，则【可打折金额】和【不可打折金额】都无需传入。
    /// </pre>
    pub discountable_amount: Option<f64>,
    /// 不可打折金额。
    /// <pre>
    /// 不参与优惠计算的金额，单位为元，精确到小数点后两位，取值范围[0.01,100000000]。
    /// 如果同时传入了【可打折金额】、【不可打折金额】和【订单总金额】，则必须满足如下条件：【订单总金额】=【可打折金额】+【不可打折金额】。
    /// 如果订单金额全部参与优惠计算，则【可打折金额】和【不可打折金额】都无需传入。
    /// </pre>
    pub undiscountable_amount: Option<f64>,
}



/// 周期付响应
#[derive(Debug, Deserialize,Serialize)]
pub struct CyclePayResponse {
    /// 商家订单号
    pub out_trade_no: String,
    /// 支付宝交易号
    pub trade_no: String,
    /// 买家支付宝账号
    pub buyer_logon_id: String,
    /// 交易的订单金额，单位为元，两位小数。该参数的值为支付时传入的total_amount
    pub total_amount: f64,
    /// 实收金额，单位为元，两位小数。该金额为本笔交易，商户账户能够实际收到的金额
    pub receipt_amount: Option<String>,
    /// 买家实付金额，单位为元，两位小数。该金额代表该笔交易买家实际支付的金额，不包含商户折扣等金额
    pub buyer_pay_amount: Option<f64>,
    /// 积分支付的金额，单位为元，两位小数。该金额代表该笔交易中用户使用积分支付的金额，比如集分宝或者支付宝实时优惠等
    pub point_amount: Option<f64>,
    /// 交易中用户支付的可开具发票的金额，单位为元，两位小数。该金额代表该笔交易中可以给用户开具发票的金额
    pub invoice_amount: Option<f64>,
    /// 交易支付时间
    pub gmt_payment: String,
    /// 交易支付使用的资金渠道。
    /// 只有在签约中指定需要返回资金明细，或者入参的query_options中指定时才返回该字段信息。
    pub fund_bill_list: Option<Vec<FundBill>>,
    /// 请求交易支付中的商户店铺的名称
    pub store_name: Option<String>,
    /// 买家在支付宝的用户id
    pub buyer_user_id: String,
    /// 本次交易支付所使用的单品券优惠的商品优惠信息。
    /// 只有在query_options中指定时才返回该字段信息。
    pub discount_goods_detail: Option<String>,
    /// 异步支付模式，目前有五种值：
    /// <pre>
    /// ASYNC_DELAY_PAY(异步延时付款);
    /// ASYNC_REALTIME_PAY(异步准实时付款);
    /// SYNC_DIRECT_PAY(同步直接扣款);
    /// NORMAL_ASYNC_PAY(纯异步付款);
    /// QUOTA_OCCUPYIED_ASYNC_PAY(异步支付并且预占了先享后付额度);
    /// </pre>
    pub async_payment_mode: Option<String>,
    /// 本交易支付时使用的所有优惠券信息。
    /// 只有在query_options中指定时才返回该字段信息。
    pub voucher_detail_list: Option<VoucherDetail>,
    /// 先享后付2.0垫资金额,不返回表示没有走垫资，非空表示垫资支付的金额
    pub advance_amount: Option<String>,
    /// 费率活动标识。
    /// <pre>
    /// 费率活动标识，当交易享受活动优惠费率时，返回该活动的标识；
    /// 只在机构间联模式下返回，其它场景下不返回该字段；
    /// 可能的返回值列表：
    /// bluesea_1：蓝海活动标识;
    /// industry_special_00：行业特殊费率0；
    /// industry_special_01：行业特殊费率1；
    /// </pre>
    pub charge_flags: Option<String>,
    /// 商家优惠金额
    pub mdiscount_amount: Option<String>,
    /// 平台优惠金额
    pub discount_amount: Option<String>,
}


#[derive(Debug, Clone, Deserialize,Serialize)]
pub struct VoucherDetail {
    /// 券id
    pub id: String,
    /// 券名称
    pub name: String,
    /// 优惠券面额，它应该会等于商家出资加上其他出资方出资
    pub amount: f64,
    /// 券类型，如：
    /// <pre>
    /// ALIPAY_FIX_VOUCHER - 全场代金券
    /// ALIPAY_DISCOUNT_VOUCHER - 折扣券
    /// ALIPAY_ITEM_VOUCHER - 单品优惠券
    /// ALIPAY_CASH_VOUCHER - 现金抵价券
    /// ALIPAY_BIZ_VOUCHER - 商家全场券
    /// 注：不排除将来新增其他类型的可能，商家接入时注意兼容性避免硬编码
    /// </pre>
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// 商家出资（特指发起交易的商家出资金额）
    pub merchant_contribute: Option<f64>,
    /// 其他出资方出资金额，可能是支付宝，可能是品牌商，或者其他方，也可能是他们的一起出资
    pub other_contribute: Option<f64>,
    /// 优惠券备注信息
    pub memo: Option<String>,
    /// 券模板id
    pub template_id: Option<String>,
    /// 如果使用的这张券是用户购买的，则该字段代表用户在购买这张券时用户实际付款的金额
    pub purchase_buyer_contribute: Option<f64>,
    /// 如果使用的这张券是用户购买的，则该字段代表用户在购买这张券时商户优惠的金额
    pub purchase_merchant_contribute: Option<f64>,
    /// 如果使用的这张券是用户购买的，则该字段代表用户在购买这张券时平台优惠的金额
    pub purchase_ant_contribute: Option<f64>,
}


#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayTradeWapPayModel {
    /// 商户网站唯一订单号
    pub out_trade_no: String,
    /// 订单总金额。
    /// 单位为元，精确到小数点后两位，取值范围：[0.01,100000000] 。
    pub total_amount: f64,
    /// 订单标题。
    /// 注意：不可使用特殊字符，如 /，=，& 等。
    pub subject: String,
    /// 订单附加信息。
    /// 如果请求时传递了该参数，将在异步通知、对账单中原样返回，同时会在商户和用户的pc账单详情中作为交易描述展示
    pub body: Option<String>,
    /// 销售产品码，商家和支付宝签约的产品码。手机网站支付为：QUICK_WAP_WAY
    pub product_code: String,
    /// 针对用户授权接口，获取用户相关数据时，用于标识用户授权关系
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    /// 用户付款中途退出返回商户网站的地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quit_url: Option<String>,
    /// 订单包含的商品列表信息，json格式，其它说明详见商品明细说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_detail: Option<GoodsDetail>,
    /// 业务扩展参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extend_params: Option<ExtendParams>,
    /// 订单绝对超时时间。
    /// 格式为yyyy-MM-dd HH:mm:ss。
    /// 注：time_expire和timeout_express两者只需传入一个或者都不传，两者均传入时，优先使用time_expire。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_expire: Option<String>,
    /// 商户传入业务信息，具体值要和支付宝约定，应用于安全，营销等参数直传场景，格式为json格式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_params: Option<String>,
    /// 公用回传参数，如果请求时传递了该参数，则返回给商户时会回传该参数。支付宝只会在同步返回（包括跳转回商户网站）和异步通知时将该参数原样返回。本参数必须进行UrlEncode之后才可以发送给支付宝。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passback_params: Option<String>,
    /// 商户原始订单号，最大长度限制32位
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_order_no: Option<String>,
}



#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayTradeJsapiPayModel {
    /// 商户网站唯一订单号
    pub out_trade_no: String,
    /// 订单总金额。
    /// 单位为元，精确到小数点后两位，取值范围：[0.01,100000000] 。
    pub total_amount: f64,
    /// 订单标题。
    /// 注意：不可使用特殊字符，如 /，=，& 等。
    pub subject: String,
    /// 订单附加信息。
    /// 如果请求时传递了该参数，将在异步通知、对账单中原样返回，同时会在商户和用户的pc账单详情中作为交易描述展示
    pub body: Option<String>,
    /// 销售产品码，商家和支付宝签约的产品码。手机网站支付为：QUICK_WAP_WAY
    pub product_code: String,
    /// 小程序支付中，商户实际经营主体的小程序应用的appid，也即最终唤起收银台支付所在的小程序的应用id
    pub op_app_id: String,
    /// 买家支付宝用户ID。 2088开头的16位纯数字，与openid二选一
    pub buyer_id: Option<String>,
    /// 买家支付宝用户唯一标识，与支付宝用户ID二选一
    pub buyer_open_id: Option<String>,
    /// 买家支付宝用户唯一标识（商户实际经营主体的小程序应用关联的买家open_id）
    pub op_buyer_open_id: Option<String>,
    /// 卖家支付宝用户ID。
    /// 当需要指定收款账号时，通过该参数传入，如果该值为空，则默认为商户签约账号对应的支付宝用户ID。
    /// 收款账号优先级规则：门店绑定的收款账户>请求传入的seller_id>商户签约账号对应的支付宝用户ID；
    /// 注：直付通和机构间联场景下seller_id无需传入或者保持跟pid一致；
    /// 如果传入的seller_id与pid不一致，需要联系支付宝小二配置收款关系；
    pub seller_id: Option<String>,
    /// 订单包含的商品列表信息，json格式，其它说明详见商品明细说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_detail: Option<GoodsDetail>,
    /// 业务扩展参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extend_params: Option<ExtendParams>,
    /// 订单绝对超时时间。
    /// 格式为yyyy-MM-dd HH:mm:ss。
    /// 注：time_expire和timeout_express两者只需传入一个或者都不传，两者均传入时，优先使用time_expire。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_expire: Option<String>,
    /// 订单相对超时时间。从交易创建时间开始计算。
    /// 该笔订单允许的最晚付款时间，逾期将关闭交易。取值范围：1m～15d。m-分钟，h-小时，d-天，1c-当天（1c-当天的情况下，无论交易何时创建，都在0点关闭）。 该参数数值不接受小数点， 如 1.5h，可转换为 90m。
    /// 当面付场景默认值为3h。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_express: Option<String>,
    /// 公用回传参数，如果请求时传递了该参数，则返回给商户时会回传该参数。支付宝只会在同步返回（包括跳转回商户网站）和异步通知时将该参数原样返回。本参数必须进行UrlEncode之后才可以发送给支付宝。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passback_params: Option<String>,
    /// 商户传入业务信息，具体值要和支付宝约定，应用于安全，营销等参数直传场景，格式为json格式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_params: Option<String>,
    /// 可打折金额。
    /// <pre>
    /// 参与优惠计算的金额，单位为元，精确到小数点后两位，取值范围[0.01,100000000]。
    /// 如果同时传入了【可打折金额】、【不可打折金额】和【订单总金额】，则必须满足如下条件：【订单总金额】=【可打折金额】+【不可打折金额】。
    /// 如果订单金额全部参与优惠计算，则【可打折金额】和【不可打折金额】都无需传入。
    /// </pre>
    pub discountable_amount: Option<f64>,
    /// 不可打折金额。
    /// <pre>
    /// 不参与优惠计算的金额，单位为元，精确到小数点后两位，取值范围[0.01,100000000]。
    /// 如果同时传入了【可打折金额】、【不可打折金额】和【订单总金额】，则必须满足如下条件：【订单总金额】=【可打折金额】+【不可打折金额】。
    /// 如果订单金额全部参与优惠计算，则【可打折金额】和【不可打折金额】都无需传入。
    /// </pre>
    pub undiscountable_amount: Option<f64>,
    /// 商户门店编号。
    /// 指商户创建门店时输入的门店编号。
    pub store_id: Option<String>,
    /// 支付宝店铺编号
    pub alipay_store_id: Option<String>,
    /// 指定支付渠道。
    /// 用户只能使用指定的渠道进行支付，多个渠道以逗号分割。
    /// 与disable_pay_channels互斥，支持传入的值：渠道列表。
    /// 注：如果传入了指定支付渠道，则用户只能用指定内的渠道支付，包括营销渠道也要指定才能使用。该参数可能导致用户支付受限，慎用。
    pub enable_pay_channels: Option<String>,
    /// 禁用渠道,用户不可用指定渠道支付，多个渠道以逗号分割
    /// 注，与enable_pay_channels互斥
    pub disable_pay_channels: Option<String>,
    /// 返回参数选项。 商户通过传递该参数来定制同步需要额外返回的信息字段，数组格式。包括但不限于：["fund_bill_list","voucher_detail_list","discount_goods_detail","discount_amount","mdiscount_amount"]
    pub query_options: Option<Vec<String>>,
    pub agreement_sign_params: Option<SignParams>,
}



#[derive(Debug, Deserialize,Serialize)]
pub struct JsapiPayResponse {
    /// 商家订单号
    pub out_trade_no: String,
    /// 支付宝交易号
    pub trade_no: String,
}


#[derive(Debug, Serialize, Default, Deserialize)]
pub struct SignParams {
    /// 商家和支付宝签约的产品码。 商家扣款产品传入固定值：GENERAL_WITHHOLDING
    pub product_code: String,
    /// 个人签约产品码，商户和支付宝签约时确定。
    pub personal_product_code: String,
    /// 协议签约场景，商户和支付宝签约时确定，商户可咨询技术支持。
    pub sign_scene: String,
    /// 请按当前接入的方式进行填充，且输入值必须为文档中的参数取值范围。
    pub access_params: AccessParams,
    /// 商户签约号，代扣协议中标示用户的唯一签约号（确保在商户系统中唯一）。
    /// 格式规则：支持大写小写字母和数字，最长32位。 商户系统按需传入，
    /// 如果同一用户在同一产品码、同一签约场景下，签订了多份代扣协议，那么需要指定并传入该值。
    pub external_agreement_no: Option<String>,
    /// 用户在商户网站的登录账号，用于在签约页面展示，如果为空，则不展示
    pub external_logon_id: Option<String>,
    /// 此参数用于传递子商户信息，无特殊需求时不用关注。目前商户代扣、海外代扣、淘旅行信用住产品支持传入该参数（在销售方案中“是否允许自定义子商户信息”需要选是）。
    pub sub_merchant: Option<SignMerchantParams>,
    /// 周期管控规则参数period_rule_params，在签约周期扣款产品（如CYCLE_PAY_AUTH_P）时必传，在签约其他产品时无需传入。 周期扣款产品，会按照这里传入的参数提示用户，并对发起扣款的时间、金额、次数等做相应限制。
    pub period_rule_params: Option<PeriodRuleParams>,
    /// 签约成功后商户用于接收异步通知的地址。如果不传入，签约与支付的异步通知都会发到外层notify_url参数传入的地址；如果外层也未传入，签约与支付的异步通知都会发到商户appid配置的网关地址。
    pub sign_notify_url: Option<String>,

}

#[derive(Debug, Serialize, Default, Deserialize)]
pub struct PeriodRuleParams {
    /// 周期类型period_type是周期扣款产品必填，枚举值为DAY和MONTH。
    /// DAY即扣款周期按天计，MONTH代表扣款周期按自然月。
    /// 与另一参数period组合使用确定扣款周期，例如period_type为DAY，period=30，则扣款周期为30天；period_type为MONTH，period=3，则扣款周期为3个自然月。
    /// 自然月是指，不论这个月有多少天，周期都计算到月份中的同一日期。例如1月3日到2月3日为一个自然月，1月3日到4月3日为三个自然月。注意周期类型使用MONTH的时候，计划扣款时间execute_time不允许传28日之后的日期（可以传28日），以此避免有些月份可能不存在对应日期的情况。
    /// 【枚举值】
    /// 自然日: DAY
    /// 自然月: MONTH
    pub period_type: String,
    /// 周期数period是周期扣款产品必填。与另一参数period_type组合使用确定扣款周期，例如period_type为DAY，period=90，则扣款周期为90天
    pub period: i64,
    /// 首次执行时间execute_time是周期扣款产品必填，即商户发起首次扣款的时间。精确到日，格式为yyyy-MM-dd
    /// 结合其他必填的扣款周期参数，会确定商户以后的扣款计划。发起扣款的时间需符合这里的扣款计划。
    pub execute_time: String,
    /// 单次扣款最大金额single_amount是周期扣款产品必填，即每次发起扣款时限制的最大金额，单位为元。商户每次发起扣款都不允许大于此金额。
    pub single_amount: f64,
    /// 总金额限制，单位为元。如果传入此参数，商户多次扣款的累计金额不允许超过此金额。
    pub total_amount: Option<f64>,
    /// 总扣款次数。如果传入此参数，则商户成功扣款的次数不能超过此次数限制（扣款失败不计入）。
    pub total_payments: Option<i64>,
}

#[derive(Debug, Serialize, Default, Deserialize)]
pub struct SignMerchantParams {
    /// 子商户的商户id
    pub sub_merchant_id: Option<String>,
    /// 子商户的商户名称
    pub sub_merchant_name: Option<String>,
    /// 子商户的服务名称
    pub sub_merchant_service_name: Option<String>,
    /// 子商户的服务描述
    pub sub_merchant_service_description: Option<String>,
}

#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AccessParams {
    /// 目前支持以下值：
    /// 1. ALIPAYAPP （钱包h5页面签约）
    /// 2. QRCODE(扫码签约)
    /// 3. QRCODEORSMS(扫码签约或者短信签约)
    pub channel: String,
}


#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayTradeAppPayModel {
    /// 商户网站唯一订单号
    pub out_trade_no: String,
    /// 订单总金额。
    /// 单位为元，精确到小数点后两位，取值范围：[0.01,100000000] 。
    pub total_amount: f64,
    /// 订单标题。
    /// 注意：不可使用特殊字符，如 /，=，& 等。
    pub subject: String,
    /// 如果请求时传递了该参数，将在异步通知、对账单中原样返回，同时会在商户和用户的pc账单详情中作为交易描述展示
    pub body: Option<String>,
    /// 销售产品码，商家和支付宝签约的产品码。手机网站支付为：QUICK_WAP_WAY
    pub product_code: String,
    /// 订单包含的商品列表信息，json格式，其它说明详见商品明细说明
    pub goods_detail: Option<GoodsDetail>,
    /// 业务扩展参数
    pub extend_params: Option<ExtendParams>,
    /// 订单绝对超时时间。
    /// 格式为yyyy-MM-dd HH:mm:ss。
    /// 注：time_expire和timeout_express两者只需传入一个或者都不传，两者均传入时，优先使用time_expire。
    pub time_expire: Option<String>,
    /// 公用回传参数，如果请求时传递了该参数，则返回给商户时会回传该参数。支付宝只会在同步返回（包括跳转回商户网站）和异步通知时将该参数原样返回。本参数必须进行UrlEncode之后才可以发送给支付宝。
    pub passback_params: Option<String>,
    /// 商户原始订单号，最大长度限制32位
    pub merchant_order_no: Option<String>,
    /// 外部指定买家
    pub ext_user_info: Option<ExtUserInfo>,
    /// 返回参数选项。 商户通过传递该参数来定制同步需要额外返回的信息字段，数组格式。包括但不限于：["fund_bill_list","voucher_detail_list","discount_goods_detail","discount_amount","mdiscount_amount"]
    pub query_options: Option<Vec<String>>,
}


/// 外部指定买家
#[derive(Debug, Serialize, Deserialize)]
pub struct ExtUserInfo {
    /// 指定买家姓名。
    /// 注： need_check_info=T时该参数才有效
    pub name: Option<String>,
    /// 指定买家手机号。
    /// 注：该参数暂不校验
    pub mobile: Option<String>,
    /// 指定买家证件类型。
    /// <pre>
    /// 枚举值：
    /// IDENTITY_CARD：身份证；
    /// PASSPORT：护照；
    /// OFFICER_CARD：军官证；
    /// SOLDIER_CARD：士兵证；
    /// HOKOU：户口本。如有其它类型需要支持，请与支付宝工作人员联系。
    /// 注： need_check_info=T时该参数才有效，支付宝会比较买家在支付宝留存的证件类型与该参数传入的值是否匹配。
    /// </pre>
    pub cert_type: Option<String>,
    /// 买家证件号。
    /// 注：need_check_info=T时该参数才有效，支付宝会比较买家在支付宝留存的证件号码与该参数传入的值是否匹配。
    pub cert_no: Option<String>,
    /// 允许的最小买家年龄。
    /// <pre>
    /// 买家年龄必须大于等于所传数值
    /// 注：
    /// 1. need_check_info=T时该参数才有效
    /// 2. min_age为整数，必须大于等于0
    /// </pre>
    pub min_age: Option<String>,
    /// 是否强制校验买家信息；
    /// <pre>
    /// 需要强制校验传：T;
    /// 不需要强制校验传：F或者不传；
    /// 当传T时，支付宝会校验支付买家的信息与接口上传递的cert_type、cert_no、name或age是否匹配，只有接口传递了信息才会进行对应项的校验；只要有任何一项信息校验不匹配交易都会失败。如果传递了need_check_info，但是没有传任何校验项，则不进行任何校验。
    /// 默认为不校验。
    /// </pre>
    pub need_check_info: Option<String>,
}


/// 当面付
#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayFaceOrderPayModel {
    /// 商户网站唯一订单号
    pub out_trade_no: String,
    /// 订单总金额。
    /// 单位为元，精确到小数点后两位，取值范围：[0.01,100000000] 。
    pub total_amount: f64,
    /// 订单标题。
    /// 注意：不可使用特殊字符，如 /，=，& 等。
    pub subject: String,
    /// 支付场景。枚举值：
    /// <pre>
    /// 枚举值：
    /// bar_code：当面付条码支付场景；
    /// security_code：当面付刷脸支付场景，对应的auth_code为fp开头的刷脸标识串；
    /// 默认值为bar_code。
    /// </pre>
    pub scene: String,
    /// <pre>
    /// 支付授权码。
    /// 当面付场景传买家的付款码（25~30开头的长度为16~24位的数字，实际字符串长度以开发者获取的付款码长度为准）或者刷脸标识串（fp开头的35位字符串）。
    /// </pre>
    pub auth_code: String,
    /// <pre>
    /// 产品码。
    /// 商家和支付宝签约的产品码。
    /// 当面付场景下，如果签约的是当面付快捷版，则传 OFFLINE_PAYMENT;
    /// 其它支付宝当面付产品传 FACE_TO_FACE_PAYMENT；
    /// 不传则默认使用FACE_TO_FACE_PAYMENT。
    /// </pre>
    pub product_code: Option<String>,
    /// 卖家支付宝用户ID。
    /// <pre>
    /// 当需要指定收款账号时，通过该参数传入，如果该值为空，则默认为商户签约账号对应的支付宝用户ID。
    /// 收款账号优先级规则：门店绑定的收款账户>请求传入的seller_id>商户签约账号对应的支付宝用户ID；
    /// 注：直付通和机构间联场景下seller_id无需传入或者保持跟pid一致；
    /// 如果传入的seller_id与pid不一致，需要联系支付宝小二配置收款关系；
    /// 支付宝预授权和新当面资金授权场景下必填。
    /// </pre>
    pub seller_id: Option<String>,
    /// 订单包含的商品列表信息，json格式，其它说明详见商品明细说明
    pub goods_detail: Option<GoodsDetail>,
    /// 业务扩展参数
    pub extend_params: Option<ExtendParams>,
    /// 优惠明细参数，通过此属性补充营销参数
    pub promo_params: Option<PromoParam>,
    /// 商户门店编号。
    /// 指商户创建门店时输入的门店编号。
    pub store_id: Option<String>,
    /// 商户操作员编号。
    pub operator_id: Option<String>,
    /// 商户机具终端编号。
    pub terminal_id: Option<String>,
    /// 返回参数选项。
    /// 商户通过传递该参数来定制同步需要额外返回的信息字段，数组格式。包括但不限于：["fund_bill_list","voucher_detail_list","enterprise_pay_info","discount_goods_detail","discount_amount","mdiscount_amount"]
    pub query_options: Option<Vec<String>>,
}



/// 周期扣款
/// 用户与商户签署周期扣款协议后，商户可通过本接口做后续免密代扣操作
#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayCycleOrderPayModel {
    /// 商户网站唯一订单号
    pub out_trade_no: String,
    /// 订单总金额。
    /// 单位为元，精确到小数点后两位，取值范围：[0.01,100000000] 。
    pub total_amount: f64,
    /// 订单标题。
    /// 注意：不可使用特殊字符，如 /，=，& 等。
    pub subject: String,
    /// <pre>
    /// 产品码。
    /// 商家和支付宝签约的产品码。 枚举值：CYCLE_PAY_AUTH：周期扣款产品；GENERAL_WITHHOLDING：代扣产品；注意：非当面付产品使用本接口时，本参数必填。请传入对应产品码。
    /// </pre>
    pub product_code: Option<String>,
    /// 代扣信息。
    /// 代扣业务需要传入的协议相关信息，使用本参数传入协议号后scene和auth_code不需要再传值。
    pub agreement_params: Option<AgreementParams>,
    /// 卖家支付宝用户ID。
    /// <pre>
    /// 当需要指定收款账号时，通过该参数传入，如果该值为空，则默认为商户签约账号对应的支付宝用户ID。
    /// 收款账号优先级规则：门店绑定的收款账户>请求传入的seller_id>商户签约账号对应的支付宝用户ID；
    /// 注：直付通和机构间联场景下seller_id无需传入或者保持跟pid一致；
    /// 如果传入的seller_id与pid不一致，需要联系支付宝小二配置收款关系；
    /// 支付宝预授权和新当面资金授权场景下必填。
    /// </pre>
    pub seller_id: Option<String>,
    /// 订单包含的商品列表信息，json格式，其它说明详见商品明细说明
    pub goods_detail: Option<GoodsDetail>,
    /// 业务扩展参数
    pub extend_params: Option<ExtendParams>,
    /// 优惠明细参数，通过此属性补充营销参数
    pub promo_params: Option<PromoParam>,
    /// 支付相关参数
    pub pay_params: Option<PayParams>,
    /// 返回参数选项。
    /// 商户通过传递该参数来定制同步需要额外返回的信息字段，数组格式。如：["fund_bill_list","voucher_detail_list","discount_goods_detail"]
    pub query_options: Option<Vec<String>>,
}



/// 周期付响应
#[derive(Debug, Deserialize,Serialize)]
pub struct CycleOrderPayResponse {
    /// 商家订单号
    pub out_trade_no: Option<String>,
    /// 支付宝交易号
    pub trade_no: Option<String>,
    /// 买家支付宝账号
    pub buyer_logon_id: Option<String>,
    /// 交易的订单金额，单位为元，两位小数。该参数的值为支付时传入的total_amount
    pub total_amount: Option<f64>,
    /// 实收金额，单位为元，两位小数。该金额为本笔交易，商户账户能够实际收到的金额
    pub receipt_amount: Option<String>,
    /// 买家实付金额，单位为元，两位小数。该金额代表该笔交易买家实际支付的金额，不包含商户折扣等金额
    pub buyer_pay_amount: Option<f64>,
    /// 积分支付的金额，单位为元，两位小数。该金额代表该笔交易中用户使用积分支付的金额，比如集分宝或者支付宝实时优惠等
    pub point_amount: Option<f64>,
    /// 交易中用户支付的可开具发票的金额，单位为元，两位小数。该金额代表该笔交易中可以给用户开具发票的金额
    pub invoice_amount: Option<f64>,
    /// 交易支付时间
    pub gmt_payment: Option<String>,
    /// 交易支付使用的资金渠道。
    /// 只有在签约中指定需要返回资金明细，或者入参的query_options中指定时才返回该字段信息。
    pub fund_bill_list: Option<Vec<FundBill>>,
    /// 请求交易支付中的商户店铺的名称
    pub store_name: Option<String>,
    /// 买家在支付宝的用户id
    pub buyer_user_id: Option<String>,
    /// 买家支付宝用户唯一标识
    pub buyer_open_id: Option<String>,
    /// 本次交易支付所使用的单品券优惠的商品优惠信息。
    /// 只有在query_options中指定时才返回该字段信息。
    pub discount_goods_detail: Option<String>,
    /// 异步支付模式，目前有五种值：
    /// <pre>
    /// ASYNC_DELAY_PAY(异步延时付款);
    /// ASYNC_REALTIME_PAY(异步准实时付款);
    /// SYNC_DIRECT_PAY(同步直接扣款);
    /// NORMAL_ASYNC_PAY(纯异步付款);
    /// QUOTA_OCCUPYIED_ASYNC_PAY(异步支付并且预占了先享后付额度);
    /// </pre>
    pub async_payment_mode: Option<String>,
    /// 本交易支付时使用的所有优惠券信息。
    /// 只有在query_options中指定时才返回该字段信息。
    pub voucher_detail_list: Option<Vec<VoucherDetail>>,
    /// 先享后付2.0垫资金额,不返回表示没有走垫资，非空表示垫资支付的金额
    pub advance_amount: Option<String>,
    /// 费率活动标识。
    /// <pre>
    /// 费率活动标识，当交易享受活动优惠费率时，返回该活动的标识；
    /// 只在机构间联模式下返回，其它场景下不返回该字段；
    /// 可能的返回值列表：
    /// bluesea_1：蓝海活动标识;
    /// industry_special_00：行业特殊费率0；
    /// industry_special_01：行业特殊费率1；
    /// </pre>
    pub charge_flags: Option<String>,
    /// 商家优惠金额
    pub mdiscount_amount: Option<String>,
    /// 平台优惠金额
    pub discount_amount: Option<String>,
}


/// 商家扣款
/// 用户与商户签署商家扣款协议后，商户可通过本接口做后续免密代扣操作
#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayDeductPayModel {
    /// 商户网站唯一订单号
    pub out_trade_no: String,
    /// 订单总金额。
    /// 单位为元，精确到小数点后两位，取值范围：[0.01,100000000] 。
    pub total_amount: f64,
    /// 订单标题。
    /// 注意：不可使用特殊字符，如 /，=，& 等。
    pub subject: String,
    /// <pre>
    /// 产品码。
    /// 商家和支付宝签约的产品码。 枚举值：CYCLE_PAY_AUTH：周期扣款产品；GENERAL_WITHHOLDING：代扣产品；注意：非当面付产品使用本接口时，本参数必填。请传入对应产品码。
    /// </pre>
    pub product_code: Option<String>,
    /// 代扣信息。
    /// 代扣业务需要传入的协议相关信息，使用本参数传入协议号后scene和auth_code不需要再传值。
    pub agreement_params: Option<AgreementParams>,
    /// 卖家支付宝用户ID。
    /// <pre>
    /// 当需要指定收款账号时，通过该参数传入，如果该值为空，则默认为商户签约账号对应的支付宝用户ID。
    /// 收款账号优先级规则：门店绑定的收款账户>请求传入的seller_id>商户签约账号对应的支付宝用户ID；
    /// 注：直付通和机构间联场景下seller_id无需传入或者保持跟pid一致；
    /// 如果传入的seller_id与pid不一致，需要联系支付宝小二配置收款关系；
    /// 支付宝预授权和新当面资金授权场景下必填。
    /// </pre>
    pub seller_id: Option<String>,
    /// 订单包含的商品列表信息，json格式，其它说明详见商品明细说明
    pub goods_detail: Option<GoodsDetail>,
    /// 业务扩展参数
    pub extend_params: Option<ExtendParams>,
    /// 优惠明细参数，通过此属性补充营销参数
    pub promo_params: Option<PromoParam>,
    /// 支付相关参数
    pub pay_params: Option<PayParams>,
    /// 返回参数选项。
    /// 商户通过传递该参数来定制同步需要额外返回的信息字段，数组格式。如：["fund_bill_list","voucher_detail_list","discount_goods_detail"]
    pub query_options: Option<Vec<String>>,
}



/// 商家扣款响应
#[derive(Debug, Deserialize,Serialize)]
pub struct DeductPayResponse {
    /// 商家订单号
    pub out_trade_no: Option<String>,
    /// 支付宝交易号
    pub trade_no: Option<String>,
    /// 买家支付宝账号
    pub buyer_logon_id: Option<String>,
    /// 交易的订单金额，单位为元，两位小数。该参数的值为支付时传入的total_amount
    pub total_amount: Option<f64>,
    /// 实收金额，单位为元，两位小数。该金额为本笔交易，商户账户能够实际收到的金额
    pub receipt_amount: Option<String>,
    /// 买家实付金额，单位为元，两位小数。该金额代表该笔交易买家实际支付的金额，不包含商户折扣等金额
    pub buyer_pay_amount: Option<f64>,
    /// 积分支付的金额，单位为元，两位小数。该金额代表该笔交易中用户使用积分支付的金额，比如集分宝或者支付宝实时优惠等
    pub point_amount: Option<f64>,
    /// 交易中用户支付的可开具发票的金额，单位为元，两位小数。该金额代表该笔交易中可以给用户开具发票的金额
    pub invoice_amount: Option<f64>,
    /// 交易支付时间
    pub gmt_payment: Option<String>,
    /// 交易支付使用的资金渠道。
    /// 只有在签约中指定需要返回资金明细，或者入参的query_options中指定时才返回该字段信息。
    pub fund_bill_list: Option<Vec<FundBill>>,
    /// 请求交易支付中的商户店铺的名称
    pub store_name: Option<String>,
    /// 买家在支付宝的用户id
    pub buyer_user_id: Option<String>,
    /// 买家支付宝用户唯一标识
    pub buyer_open_id: Option<String>,
    /// 本次交易支付所使用的单品券优惠的商品优惠信息。
    /// 只有在query_options中指定时才返回该字段信息。
    pub discount_goods_detail: Option<String>,
    /// 异步支付模式，目前有五种值：
    /// <pre>
    /// ASYNC_DELAY_PAY(异步延时付款);
    /// ASYNC_REALTIME_PAY(异步准实时付款);
    /// SYNC_DIRECT_PAY(同步直接扣款);
    /// NORMAL_ASYNC_PAY(纯异步付款);
    /// QUOTA_OCCUPYIED_ASYNC_PAY(异步支付并且预占了先享后付额度);
    /// </pre>
    pub async_payment_mode: Option<String>,
    /// 本交易支付时使用的所有优惠券信息。
    /// 只有在query_options中指定时才返回该字段信息。
    pub voucher_detail_list: Option<Vec<VoucherDetail>>,
    /// 先享后付2.0垫资金额,不返回表示没有走垫资，非空表示垫资支付的金额
    pub advance_amount: Option<String>,
    /// 费率活动标识。
    /// <pre>
    /// 费率活动标识，当交易享受活动优惠费率时，返回该活动的标识；
    /// 只在机构间联模式下返回，其它场景下不返回该字段；
    /// 可能的返回值列表：
    /// bluesea_1：蓝海活动标识;
    /// industry_special_00：行业特殊费率0；
    /// industry_special_01：行业特殊费率1；
    /// </pre>
    pub charge_flags: Option<String>,
    /// 商家优惠金额
    pub mdiscount_amount: Option<String>,
    /// 平台优惠金额
    pub discount_amount: Option<String>,
}




/// 统一收单线下交易预创建
/// 收银员通过收银台或商户后台调用支付宝接口，生成二维码后，展示给用户，由用户扫描二维码完成订单支付。
#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayTradePreCreateModel {
    /// 商户网站唯一订单号
    pub out_trade_no: String,
    /// 订单总金额。
    /// 单位为元，精确到小数点后两位，取值范围：[0.01,100000000] 。
    pub total_amount: f64,
    /// 订单标题。
    /// 注意：不可使用特殊字符，如 /，=，& 等。
    pub subject: String,
    /// <pre>
    /// 产品码。
    /// 商家和支付宝签约的产品码。 枚举值（点击查看签约情况）：
    /// FACE_TO_FACE_PAYMENT：当面付产品；
    /// 默认值为FACE_TO_FACE_PAYMENT。
    /// </pre>
    pub product_code: String,
    /// 卖家支付宝用户ID。
    /// <pre>
    /// 当需要指定收款账号时，通过该参数传入，如果该值为空，则默认为商户签约账号对应的支付宝用户ID。
    /// 收款账号优先级规则：门店绑定的收款账户>请求传入的seller_id>商户签约账号对应的支付宝用户ID；
    /// 注：直付通和机构间联场景下seller_id无需传入或者保持跟pid一致；
    /// 如果传入的seller_id与pid不一致，需要联系支付宝小二配置收款关系；
    /// 支付宝预授权和新当面资金授权场景下必填。
    /// </pre>
    pub seller_id: Option<String>,
    /// 订单附加信息。
    /// 如果请求时传递了该参数，将在异步通知、对账单中原样返回，同时会在商户和用户的pc账单详情中作为交易描述展示
    pub body: Option<String>,
    /// 订单包含的商品列表信息，json格式，其它说明详见商品明细说明
    pub goods_detail: Option<Vec<GoodsDetail>>,
    /// 业务扩展参数
    pub extend_params: Option<ExtendParams>,
    /// 商户传入业务信息，具体值要和支付宝约定，应用于安全，营销等参数直传场景，格式为json格式
    pub business_params: Option<BusinessParams>,
    /// 可打折金额。
    /// <pre>
    /// 参与优惠计算的金额，单位为元，精确到小数点后两位，取值范围[0.01,100000000]。
    /// 如果同时传入了【可打折金额】、【不可打折金额】和【订单总金额】，则必须满足如下条件：【订单总金额】=【可打折金额】+【不可打折金额】。
    /// 如果订单金额全部参与优惠计算，则【可打折金额】和【不可打折金额】都无需传入。
    /// </pre>
    pub discountable_amount: Option<f64>,
    /// 不可打折金额。
    /// <pre>
    /// 不参与优惠计算的金额，单位为元，精确到小数点后两位，取值范围[0.01,100000000]。
    /// 如果同时传入了【可打折金额】、【不可打折金额】和【订单总金额】，则必须满足如下条件：【订单总金额】=【可打折金额】+【不可打折金额】。
    /// 如果订单金额全部参与优惠计算，则【可打折金额】和【不可打折金额】都无需传入。
    /// </pre>
    pub undiscountable_amount: Option<f64>,
    /// 商户门店编号。
    /// 指商户创建门店时输入的门店编号。
    pub store_id: Option<String>,
    /// 商户机具终端编号。
    pub terminal_id: Option<String>,
    /// 商家操作员编号 id，由商家自定义。
    pub operator_id: Option<String>,
    /// 商户原始订单号，最大长度限制32位
    pub merchant_order_no: Option<String>,
}

#[derive(Debug, Serialize, Default, Deserialize)]
pub struct BusinessParams {
    /// 商户端创建订单的 IP，须上传正确的用户端外网 IP，支持 ipv4/ipv6 格式；
    pub mc_create_trade_ip: Option<String>,
}



#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayTradeQueryModel {
    /// 订单支付时传入的商户订单号,和支付宝交易号不能同时为空。
    /// trade_no,out_trade_no如果同时存在优先取trade_no
    pub out_trade_no: Option<String>,
    /// 支付宝交易号，和商户订单号不能同时为空
    pub trade_no: Option<String>,
    /// 银行间联模式下有用，其它场景请不要使用；
    /// 双联通过该参数指定需要查询的交易所属收单机构的pid;
    pub org_pid: Option<String>,
    /// 查询选项，商户传入该参数可定制本接口同步响应额外返回的信息字段，数组格式。支持枚举如下：trade_settle_info：返回的交易结算信息，包含分账、补差等信息；
    /// <pre>
    /// fund_bill_list：交易支付使用的资金渠道；
    /// voucher_detail_list：交易支付时使用的所有优惠券信息；
    /// discount_goods_detail：交易支付所使用的单品券优惠的商品优惠信息；
    /// mdiscount_amount：商家优惠金额；
    /// </pre>
    pub query_options: Option<Vec<String>>,
}


#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayTradeRefundModel {
    /// 商户订单号。
    /// 订单支付时传入的商户订单号，商家自定义且保证商家系统中唯一。与支付宝交易号 trade_no 不能同时为空。
    pub out_trade_no: Option<String>,
    /// 支付宝交易号。
    /// 和商户订单号 out_trade_no 不能同时为空。
    pub trade_no: Option<String>,
    /// 退款金额。
    /// <pre>
    /// 需要退款的金额，该金额不能大于订单金额，单位为元，支持两位小数。
    /// 注：如果正向交易使用了营销，该退款金额包含营销金额，支付宝会按业务规则分配营销和买家自有资金分别退多少，默认优先退买家的自有资金。
    /// 如交易总金额100元，用户支付时使用了80元自有资金和20元无资金流的营销券，商家实际收款80元。如果首次请求退款60元，则60元全部从商家收款资金扣除退回给用户自有资产；如果再请求退款40元，
    /// 则从商家收款资金扣除20元退回用户资产以及把20元的营销券退回给用户（券是否可再使用取决于券的规则配置）。
    /// </pre>
    pub refund_amount: Option<f64>,
    /// 退款原因说明。
    /// 商家自定义，将在会在商户和用户的pc退款账单详情中展示
    pub refund_reason: Option<String>,
    /// 退款请求号。
    /// <pre>
    /// 标识一次退款请求，需要保证在交易号下唯一，如需部分退款，则此参数必传。
    /// 注：针对同一次退款请求，如果调用接口失败或异常了，重试时需要保证退款请求号不能变更，防止该笔交易重复退款。支付宝会保证同样的退款请求号多次请求只会退一次。
    /// </pre>
    pub out_request_no: Option<String>,
    /// <pre>
    /// 退分账明细信息。
    /// 注： 1.当面付且非直付通模式无需传入退分账明细，系统自动按退款金额与订单金额的比率，从收款方和分账收入方退款，不支持指定退款金额与退款方。
    /// 2.直付通模式，电脑网站支付，手机 APP 支付，手机网站支付产品，须在退款请求中明确是否退分账，从哪个分账收入方退，退多少分账金额；如不明确，默认从收款方退款，收款方余额不足退款失败。不支持系统按比率退款。
    /// </pre>
    pub refund_royalty_parameters: Option<OpenApiRoyaltyDetailInfoPojo>,
    /// 查询选项。
    /// 商户通过上送该参数来定制同步需要额外返回的信息字段，数组格式。支持：refund_detail_item_list：退款使用的资金渠道；deposit_back_info：触发银行卡冲退信息通知；
    pub query_options: Option<Vec<String>>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct OpenApiRoyaltyDetailInfoPojo {
    /// 分账类型.
    /// <pre>
    /// 普通分账为：transfer;
    /// 补差为：replenish;
    /// 为空默认为分账transfer;
    /// </pre>
    pub royalty_type: Option<String>,
    /// 支出方账户。如果支出方账户类型为userId，本参数为支出方的支付宝账号对应的支付宝唯一用户号，以2088开头的纯16位数字；如果支出方类型为loginName，本参数为支出方的支付宝登录号。 泛金融类商户分账时，该字段不要上送。
    pub trans_out: Option<String>,
    /// 支出方账户类型。userId表示是支付宝账号对应的支付宝唯一用户号;loginName表示是支付宝登录号； 泛金融类商户分账时，该字段不要上送。
    pub trans_out_type: Option<String>,
    /// 收入方账户类型。userId表示是支付宝账号对应的支付宝唯一用户号;cardAliasNo表示是卡编号;loginName表示是支付宝登录号；
    pub trans_in_type: Option<String>,
    /// 收入方账户。如果收入方账户类型为userId，本参数为收入方的支付宝账号对应的支付宝唯一用户号，以2088开头的纯16位数字；如果收入方类型为cardAliasNo，本参数为收入方在支付宝绑定的卡编号；如果收入方类型为loginName，本参数为收入方的支付宝登录号；
    pub trans_in: Option<String>,
    /// 分账的金额，单位为元
    pub amount: Option<f64>,
    /// 分账描述
    pub desc: Option<String>,
    /// 可选值：达人佣金、平台服务费、技术服务费、其他
    pub royalty_scene: Option<String>,
    /// 分账收款方姓名，上送则进行姓名与支付宝账号的一致性校验，校验不一致则分账失败。不上送则不进行姓名校验
    pub trans_in_name: Option<String>,
}



#[cfg(test)]
mod test {
    use crate::alipay::builder::AlipayClientBuilder;
    use super::*;
    fn create_client() -> AlipayClient {
        AlipayClientBuilder::new()
            .app_id("2021003142611544")
            .alipay_public_key("MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEApDLNM4aHZUe0eCQp8/GfH5sR3ogbI+H1bClPTxL+Qg1pW2qI6p4SFTR2NZIjB1Y8pXnx8OIYor2OIeOP6UvfkRjfuhGQvazGsQfgHpAqzfaqyveBnMdRK5HEx/UKHK4LrvdL6C0+DHcDy/5WMH55y33ZqdZ40YTe15Akw5+HC6qvTGEc8GRk5cDhCOy0eVRBMUalQHjH3/NnId0m4oS5IKS/f7WxgayoRDutrLeFxsIFcGS8paIV+4EYzrqDE87XyV5xOyKBTo7+409yY7xcOhp1FO6seSpBkLFemI1UQDcrmpvMbGIF9KQxncmZX8vd5PL8LPErx2Gq8ruEALRUHQIDAQAB")
            .alipay_root_cert_path("http://img.dailyguitar.cn/16/0cfd3d67596c46dfb7648b13389cbd6f_apk.txt")
            .app_private_key("MIIEowIBAAKCAQEAlrUunTdVDXv+5usLSugxpQIn4k6FCLXup/LsWX6iqNccNs3ZCFrPqJs6vTUg9nCYN4JqernaxSpTgiXDb2ilJpqXFTWs/g4CBYWTSXVjvqDmXsx5W6QyiZB5MM1mya8kGbXhMOweZBAy48QI30rJ529ZDGLpYdjV0FlEz+KKs8tcQacjw/7XQP6euEA5GfTr8JG6tvHiOYruI0UppGUIWngSmxsLTlco3P2Q0OO0sOZcJNR3sZcqlTURXnldZ1yQyUAQiibUuUpqlEbim91yI0jNGlGq5UThlaj+QlYKlnakzGJ7bRZQJymyAdhtsAptPfUwr3NPU8OqoQuIS9oh8QIDAQABAoIBAHiyvJVvBjAZeWlpHw8r7O6RTm4Boqv6QRvDAyUdtZnxRYhizgnR6PPI9n8oOLEGNfJnAW4uwRYdMupN4jqsnW/QaWW0KksB3v6bHv27BVpbtISB0EPjuhHQYi8Coequ2QO6VZL/18sd6hPeIZjrZc5zr+aqpO0XYw5NID452gnpffe6+fCTz/ZO8K6APp37rjq8vvci+iJ3ZBWkZfBBNro7Bimv8m7qPueyUMWo2GnB/qXNh7pDA6M66Uq0GgfbroMZ55OOiZNYb5MvrDo7kZICCIACFHwNvu3Ua8k/lyCPCufo9xhD0uxam0O0kea/JSJeplWqBbsg61XxRpe8sgECgYEA1CpcE8Mp4YmlIUE1YUCMexU6oabcqVpozOtMZLlmSTdJUjOUZNB5H1daQJX2fYTkVbZPPPwOHQAm8rKkgbddAZx6UNxHAKEFTFW/2VOV9qyhZ9rj0DON8pUR4UaH3DGS2BDbOtfDjnB0segG0TqHKfatzMMh8/6Vr69a51zZI+ECgYEAtdhDS1+EnKAEPjP3/w67KlUcBjjVdqbFvUkUTd4CvwXo4YS59wIomA80MbjgpqbS3RPGi3wXM1AHDMcl2oCioN8D0zEArBiIh2/bNVhgKbIrelPjEMTuL1KXn1GuDo3AebCvvpH7jsI3EV2F0/yWUcME/ukhkOuDMiWs11dGwBECgYBp9UXK8NsHslBvzTs1eKEwDyga91SYl3hjbtfWLelxg1Tw4qWnu53iEyQVwU863JWUYNot4MvDnAHXj3Qs/EBMv7MukvQ60i/tMZ4AkYgBi7GIRn8jybuIfP5S/YB4baGimriCKKOXjX4aI4DUGWDLilC+RG2+h1SGSxMhHPOswQKBgQCiAuSpwZcvyC0fkkOQPXqpk5xdOsOMa7dfFj39xh/iOwyp6AubM5QhLeKNri6Qq67Qgu7wlQVClTlCvCdQjizWcPtCbLUxnsX9DL5bv7olS/UKjrSN2zZueQJXUnllxAtJIA7kFcHvBb/0O8OhA7iVbdAEoIZkTD/eLMzaKosBUQKBgH+Pq8Q3ZpMSSYG8MvizoqF05Y7O58P3fghaRJcnfHgTjHfKnlXfVMi4SR8pvg7uOHMHcDIBknZ5iXP/H/B2xKZRXhNmJNnr4omLwKATaif21ojvZh92Bz+m7w41WD/k+mzBEWNutbVGLmLiBQhKYYC7VRhgtWw/o3wilC/f/rPY")
            .sign_type("RSA2")
            .format("JSON")
            .build().unwrap()
    }

    #[tokio::test]
    async fn test_pay() {
        let pay_request = AlipayTradePreCreateModel {
            out_trade_no: "20201212121212".to_string(),
            total_amount: 0.01,
            subject: "测试".to_string(),
            product_code: "".to_string(),
            seller_id: None,
            body: Some("测试".to_string()),
            goods_detail: Some(vec![GoodsDetail{
                goods_id: "1".to_string(),
                alipay_goods_id: None,
                goods_name: "测试商品".to_string(),
                quantity: 1,
                price: 0.01,
                goods_category: None,
                body: None,
                show_url: None,
            }]),
            store_id: None,
            terminal_id: None,
            operator_id: None,
            extend_params: None,
            business_params: None,
            discountable_amount: None,
            undiscountable_amount: None,
            merchant_order_no: None,
        };
        let client = create_client();
        let srv = client.alipay_service();
        let mut request = AlipayBizRequest::new();
        request.set_biz_model(pay_request);
        let pay_response = srv.precreate(request).await.unwrap();
        println!("{:?}", pay_response);
    }

    #[tokio::test]
    async fn test_query() {
        let client = create_client();
        let srv = client.alipay_service();
        let request = AlipayTradeQueryModel {
            out_trade_no: Some("20201212121212".to_string()),
            trade_no: Some("20201212121212".to_string()),
            org_pid: None,
            query_options: None,
        };
        let response = srv.query(request.into()).await.unwrap();
        println!("{:?}", response);
    }

    #[tokio::test]
    async fn test_refund() {
        let client = create_client();
        let srv = client.alipay_service();
        let request = AlipayTradeRefundModel {
            out_trade_no: Some("20201212121212".to_string()),
            trade_no: Some("20201212121212".to_string()),
            refund_amount: Some(0.01),
            refund_reason: None,
            out_request_no: None,
            refund_royalty_parameters: None,
            query_options: None,
        };
        let response = srv.refund(request.into()).await.unwrap();
        println!("{:?}", response);
    }

    #[tokio::test]
    async fn test_cancel() {
        let client = create_client();
        let srv = client.alipay_service();
        let request = AlipayTradeCancelModel {
            out_trade_no: Some("20201212121212".to_string()),
            trade_no: Some("20201212121212".to_string()),
        };
        let response = srv.cancel(request.into()).await.unwrap();
        println!("{:?}", response);
    }

    #[tokio::test]
    async fn test_close() {
        let client = create_client();
        let srv = client.alipay_service();
        let request = AlipayTradeCloseModel {
            out_trade_no: Some("20201212121212".to_string()),
            trade_no: Some("20201212121212".to_string()),
            operator_id: None,
        };
        let response = srv.close(request.into()).await.unwrap();
    }
}