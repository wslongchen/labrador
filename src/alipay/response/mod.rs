use std::collections::{BTreeMap};
use json::JsonValue;
use serde::{Deserialize, Serialize};
use serde::de::{DeserializeOwned};

use crate::{errors::LabraError, AlipayResponse, LabradorResult, RequestMethod};
use crate::alipay::constants::{ERROR_RESPONSE_KEY, SIGN};

//----------------------------------------------------------------------------------------------------------------------------
#[derive(Debug, Deserialize,Serialize)]
pub struct DescList {
    string: Option<Vec<String>>
}


// 支付宝 ↓
#[derive(Debug, Serialize, Deserialize)]
pub struct AlipayBaseResponse {
    pub code: Option<String>,
    pub msg: Option<String>,
    pub sub_code: Option<String>,
    pub sub_msg: Option<String>,
    pub body: Option<String>,
    pub sign: Option<String>,
    pub params: Option<BTreeMap<String, String>>,
}



#[derive(Debug, Deserialize,Serialize)]
pub struct ResultlList<T> {
    /// 数据
    pub map_data: Option<Vec<T>>,
}

#[derive(Debug, Deserialize,Serialize, Default)]
pub struct AlipayCommonResponse {
    pub out_html: Option<String>,
    /// 商户网站唯一订单号
    pub out_trade_no: String,
    /// 该交易在支付宝系统中的交易流水号。最长64位。
    pub trade_no: String,
    /// 该笔订单的资金总额，单位为人民币（元），取值范围为 0.01~100000000.00，精确到小数点后两位。
    pub total_amount: f64,
    /// 收款支付宝账号对应的支付宝唯一用户号。
    /// 以2088开头的纯16位数字
    pub seller_id: String,
    /// 商户原始订单号，最大长度限制32位
    pub merchant_order_no: String,
}

impl AlipayBaseResponse {
    pub fn new() -> Self {
        Self {
            code: None,
            msg: None,
            sub_code: None,
            sub_msg: None,
            body: None,
            sign: None,
            params: None,
        }
    }

    pub fn parse(str: &str,method: impl RequestMethod) -> LabradorResult<Self> {
        let v= json::parse(str).unwrap_or(JsonValue::Null);
        let sign = v[SIGN].as_str().unwrap_or_default();
        // 判断是否异常
        let err= &v[ERROR_RESPONSE_KEY];
        if !err.is_empty() && !err.is_null() {
            let resp = serde_json::from_str::<Self>(&err.to_string()).unwrap_or(AlipayBaseResponse::new());
            Err(LabraError::ClientError {errcode: resp.code.to_owned().unwrap_or_default(), errmsg: resp.sub_msg.to_owned().unwrap_or_default()})
        } else {
            let response = &v[&method.get_response_key()];
            if !response.is_empty() && !response.is_null() {
                let mut resp = serde_json::from_str::<Self>(&response.to_string()).unwrap_or(AlipayBaseResponse::new());
                if resp.code.is_none() {
                    resp.code = "10000".to_string().into();
                }
                resp.sign = sign.to_string().into();
                resp.body = response.to_string().into();
                Ok(resp)
            } else {
                Err(LabraError::MissingField(format!("无法获取解析返回结果：【{}】", str)))
            }
        }

    }

    pub fn is_success(&self) -> bool {
        self.code.to_owned().unwrap_or_default().eq("10000")
    }

    pub fn get_biz_model<T: DeserializeOwned>(&self) -> LabradorResult<T> {
        if self.is_success() {
            serde_json::from_str::<T>(&self.body.to_owned().unwrap_or_default()).map_err(LabraError::from)
        } else {
            Err(LabraError::ClientError { errcode: self.code.to_owned().unwrap_or_default(), errmsg: self.sub_msg.to_owned().unwrap_or_default() })
        }
    }

}

impl AlipayResponse for AlipayBaseResponse {
    fn set_sub_code(&mut self, sub_code: String) {
        self.sub_code = sub_code.into();
    }

    fn set_code(&mut self, code: String) {
        self.code = code.into();
    }

    fn get_body(&self) -> String {
        self.body.to_owned().unwrap_or_default()
    }

    fn set_body(&mut self, body: String) {
        self.body = body.into();
    }

    fn get_sub_code(&self) -> String {
        self.sub_code.to_owned().unwrap_or_default()
    }

    fn get_code(&self) -> String {
        self.code.to_owned().unwrap_or_default()
    }

    fn get_sign(&self) -> String {
        self.sign.to_owned().unwrap_or_default()
    }

    fn set_msg(&mut self, msg: String) {
        self.msg = msg.into();
    }

    fn set_sub_msg(&mut self, sub_msg: String) {
        self.sub_msg = sub_msg.into();
    }

    fn get_sub_msg(&self) -> String {
        self.sub_msg.to_owned().unwrap_or_default()
    }

    fn get_msg(&self) -> String {
        self.msg.to_owned().unwrap_or_default()
    }

}

#[derive(Debug, Deserialize,Serialize)]
pub struct AlipayQueryOrderResponse {
    /// 商家订单号
    pub out_trade_no: String,
    /// 支付宝交易号
    pub trade_no: String,
    /// 买家支付宝账号
    pub buyer_logon_id: String,
    /// 交易状态：WAIT_BUYER_PAY（交易创建，等待买家付款）、TRADE_CLOSED（未付款交易超时关闭，或支付完成后全额退款）、TRADE_SUCCESS（交易支付成功）、TRADE_FINISHED（交易结束，不可退款）
    pub trade_status: String,
    /// 交易的订单金额，单位为元，两位小数。该参数的值为支付时传入的total_amount
    pub total_amount: f64,
    /// 标价币种，该参数的值为支付时传入的trans_currency，支持英镑：GBP、港币：HKD、美元：USD、新加坡元：SGD、日元：JPY、加拿大元：CAD、澳元：AUD、欧元：EUR、新西兰元：NZD、韩元：KRW、泰铢：THB、瑞士法郎：CHF、瑞典克朗：SEK、丹麦克朗：DKK、挪威克朗：NOK、马来西亚林吉特：MYR、印尼卢比：IDR、菲律宾比索：PHP、毛里求斯卢比：MUR、以色列新谢克尔：ILS、斯里兰卡卢比：LKR、俄罗斯卢布：RUB、阿联酋迪拉姆：AED、捷克克朗：CZK、南非兰特：ZAR、人民币：CNY、新台币：TWD。当trans_currency 和 settle_currency 不一致时，trans_currency支持人民币：CNY、新台币：TWD
    pub trans_currency: Option<String>,
    /// 订单结算币种，对应支付接口传入的settle_currency，支持英镑：GBP、港币：HKD、美元：USD、新加坡元：SGD、日元：JPY、加拿大元：CAD、澳元：AUD、欧元：EUR、新西兰元：NZD、韩元：KRW、泰铢：THB、瑞士法郎：CHF、瑞典克朗：SEK、丹麦克朗：DKK、挪威克朗：NOK、马来西亚林吉特：MYR、印尼卢比：IDR、菲律宾比索：PHP、毛里求斯卢比：MUR、以色列新谢克尔：ILS、斯里兰卡卢比：LKR、俄罗斯卢布：RUB、阿联酋迪拉姆：AED、捷克克朗：CZK、南非兰特：ZAR
    pub settle_currency: Option<String>,
    /// 结算币种订单金额
    pub settle_amount: Option<f64>,
    /// 订单支付币种 -- 可能类型有问题
    pub pay_currency: Option<String>,
    /// 支付币种订单金额
    pub pay_amount: Option<String>,
    /// 结算币种兑换标价币种汇率
    pub settle_trans_rate: Option<String>,
    /// 标价币种兑换支付币种汇率
    pub trans_pay_rate: Option<String>,
    /// 买家实付金额，单位为元，两位小数。该金额代表该笔交易买家实际支付的金额，不包含商户折扣等金额
    pub buyer_pay_amount: Option<f64>,
    /// 积分支付的金额，单位为元，两位小数。该金额代表该笔交易中用户使用积分支付的金额，比如集分宝或者支付宝实时优惠等
    pub point_amount: Option<f64>,
    /// 交易中用户支付的可开具发票的金额，单位为元，两位小数。该金额代表该笔交易中可以给用户开具发票的金额
    pub invoice_amount: Option<f64>,
    /// 本次交易打款给卖家的时间
    pub send_pay_date: Option<String>,
    /// 实收金额，单位为元，两位小数。该金额为本笔交易，商户账户能够实际收到的金额
    pub receipt_amount: Option<String>,
    /// 商户门店编号
    pub store_id: Option<String>,
    /// 商户机具终端编号
    pub terminal_id: Option<String>,
    /// 交易支付使用的资金渠道。
    /// 只有在签约中指定需要返回资金明细，或者入参的query_options中指定时才返回该字段信息。
    pub fund_bill_list: Option<Vec<TradeFundBill>>,
    /// 请求交易支付中的商户店铺的名称
    pub store_name: Option<String>,
    /// 买家在支付宝的用户id
    pub buyer_user_id: String,
    /// 行业特殊信息-统筹相关
    pub industry_sepc_detail_gov: Option<String>,
    /// 行业特殊信息-个账相关
    pub industry_sepc_detail_acc: Option<String>,
    /// 该笔交易针对收款方的收费金额；
    /// 只在银行间联交易场景下返回该信息；
    pub charge_amount: Option<String>,
    /// 费率活动标识。
    /// <pre>
    /// 当交易享受特殊行业或活动费率时，返回该场景的标识。具体场景如下：
    /// trade_special_00：订单优惠费率；
    /// industry_special_on_00：线上行业特殊费率0；
    /// industry_special_on_01：线上行业特殊费率1；
    /// industry_special_00：线下行业特殊费率0；
    /// industry_special_01：线下行业特殊费率1；
    /// bluesea_1：蓝海活动优惠费率标签；
    /// 注：只在机构间联模式下返回，其它场景下不返回该字段；
    /// </pre>
    pub charge_flags: Option<String>,
    /// 支付清算编号，用于清算对账使用；
    /// 只在银行间联交易场景下返回该信息；
    pub settlement_id: Option<String>,
    /// 返回的交易结算信息，包含分账、补差等信息。
    /// 只有在query_options中指定时才返回该字段信息。
    pub trade_settle_info: Option<TradeSettleInfo>,
    /// 预授权支付模式，该参数仅在信用预授权支付场景下返回。信用预授权支付：CREDIT_PREAUTH_PAY
    pub auth_trade_pay_mode: Option<String>,
    /// 买家用户类型。CORPORATE:企业用户；PRIVATE:个人用户。
    pub buyer_user_type: Option<String>,
    /// 商家优惠金额
    pub mdiscount_amount: Option<String>,
    /// 平台优惠金额
    pub discount_amount: Option<String>,
    /// 订单标题；
    /// 只在银行间联交易场景下返回该信息；
    pub subject: Option<String>,
    /// 订单描述；
    /// 只在银行间联交易场景下返回该信息
    pub body: Option<String>,
    /// 间连商户在支付宝端的商户编号；
    /// 只在银行间联交易场景下返回该信息；
    pub alipay_sub_merchant_id: Option<String>,
    /// 交易额外信息，特殊场景下与支付宝约定返回。
    /// json格式。
    pub ext_infos: Option<String>,
    /// 公用回传参数。
    /// 返回支付时传入的passback_params参数信息
    pub passback_params: Option<String>,
    /// 若用户使用花呗分期支付，且商家开通返回此通知参数，则会返回花呗分期信息。json格式其它说明详见花呗分期信息说明。
    /// 注意：商家需与支付宝约定后才返回本参数。
    pub hb_fq_pay_info: Option<HbFqPayInfo>,
    /// 信用支付模式。表示订单是采用信用支付方式（支付时买家没有出资，需要后续履约）。"creditAdvanceV2"表示芝麻先用后付模式，用户后续需要履约扣款。 此字段只有信用支付场景才有值，商户需要根据字段值单独处理。此字段以后可能扩展其他值，建议商户使用白名单方式识别，对于未识别的值做失败处理，并联系支付宝技术支持人员。
    pub credit_pay_mode: String,
    /// 信用支付模式。表示订单是采用信用支付方式（支付时买家没有出资，需要后续履约）。"creditAdvanceV2"表示芝麻先用后付模式，用户后续需要履约扣款。 此字段只有信用支付场景才有值，商户需要根据字段值单独处理。此字段以后可能扩展其他值，建议商户使用白名单方式识别，对于未识别的值做失败处理，并联系支付宝技术支持人员。
    pub credit_biz_order_id: String,
}



#[derive(Debug, Deserialize,Serialize)]
pub struct AlipayUnifiedOrderPayResponse {
    /// 商家订单号
    pub out_trade_no: String,
    /// 支付宝交易号
    pub trade_no: String,
    /// 买家支付宝账号
    pub buyer_logon_id: String,
    /// 交易的订单金额，单位为元，两位小数。该参数的值为支付时传入的total_amount
    pub total_amount: f64,
    /// 交易支付时间
    pub gmt_payment: String,
    /// 交易支付使用的资金渠道。
    /// 只有在签约中指定需要返回资金明细，或者入参的query_options中指定时才返回该字段信息。
    pub fund_bill_list: Option<Vec<TradeFundBill>>,
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
    /// 预授权支付模式，该参数仅在信用预授权支付场景下返回。信用预授权支付：CREDIT_PREAUTH_PAY
    pub auth_trade_pay_mode: Option<String>,
    /// 商家优惠金额
    pub mdiscount_amount: Option<String>,
    /// 平台优惠金额
    pub discount_amount: Option<String>,
    /// 信用支付模式。表示订单是采用信用支付方式（支付时买家没有出资，需要后续履约）。"creditAdvanceV2"表示芝麻先用后付模式，用户后续需要履约扣款。 此字段只有信用支付场景才有值，商户需要根据字段值单独处理。此字段以后可能扩展其他值，建议商户使用白名单方式识别，对于未识别的值做失败处理，并联系支付宝技术支持人员。
    pub credit_pay_mode: String,
    /// 信用支付模式。表示订单是采用信用支付方式（支付时买家没有出资，需要后续履约）。"creditAdvanceV2"表示芝麻先用后付模式，用户后续需要履约扣款。 此字段只有信用支付场景才有值，商户需要根据字段值单独处理。此字段以后可能扩展其他值，建议商户使用白名单方式识别，对于未识别的值做失败处理，并联系支付宝技术支持人员。
    pub credit_biz_order_id: String,
    /// 因公付支付信息，只有入参的query_options中指定时才返回该字段信息
    pub enterprise_pay_info: Option<EnterprisePayInfo>,
    /// 是否可以转为app支付，仅当商户代扣失败场景才会返回该字段信息
    pub can_turn_to_app_pay: Option<String>,
    /// 买家实付金额，单位为元，两位小数。该金额代表该笔交易买家实际支付的金额，不包含商户折扣等金额
    pub buyer_pay_amount: Option<f64>,
    /// 积分支付的金额，单位为元，两位小数。该金额代表该笔交易中用户使用积分支付的金额，比如集分宝或者支付宝实时优惠等
    pub point_amount: Option<f64>,
    /// 交易中用户支付的可开具发票的金额，单位为元，两位小数。该金额代表该笔交易中可以给用户开具发票的金额
    pub invoice_amount: Option<f64>,
    /// 实收金额，单位为元，两位小数。该金额为本笔交易，商户账户能够实际收到的金额
    pub receipt_amount: Option<String>,
}



/// 当面付响应
#[derive(Debug, Deserialize,Serialize)]
pub struct AlipayFaceOrderPayResponse {
    /// 商家订单号
    pub out_trade_no: String,
    /// 支付宝交易号
    pub trade_no: String,
    /// 买家支付宝账号
    pub buyer_logon_id: String,
    /// 交易的订单金额，单位为元，两位小数。该参数的值为支付时传入的total_amount
    pub total_amount: f64,
    /// 交易支付时间
    pub gmt_payment: String,
    /// 交易支付使用的资金渠道。
    /// 只有在签约中指定需要返回资金明细，或者入参的query_options中指定时才返回该字段信息。
    pub fund_bill_list: Option<Vec<TradeFundBill>>,
    /// 请求交易支付中的商户店铺的名称
    pub store_name: Option<String>,
    /// 买家在支付宝的用户id
    pub buyer_user_id: String,
    /// 本次交易支付所使用的单品券优惠的商品优惠信息。
    /// 只有在query_options中指定时才返回该字段信息。
    pub discount_goods_detail: Option<String>,
    /// 本交易支付时使用的所有优惠券信息。
    /// 只有在query_options中指定时才返回该字段信息。
    pub voucher_detail_list: Option<VoucherDetail>,
    /// 商家优惠金额
    pub mdiscount_amount: Option<String>,
    /// 平台优惠金额
    pub discount_amount: Option<String>,
    /// 买家实付金额，单位为元，两位小数。该金额代表该笔交易买家实际支付的金额，不包含商户折扣等金额
    pub buyer_pay_amount: Option<f64>,
    /// 积分支付的金额，单位为元，两位小数。该金额代表该笔交易中用户使用积分支付的金额，比如集分宝或者支付宝实时优惠等
    pub point_amount: Option<f64>,
    /// 交易中用户支付的可开具发票的金额，单位为元，两位小数。该金额代表该笔交易中可以给用户开具发票的金额
    pub invoice_amount: Option<f64>,
    /// 实收金额，单位为元，两位小数。该金额为本笔交易，商户账户能够实际收到的金额
    pub receipt_amount: Option<String>,
}

/// 周期付响应
#[derive(Debug, Deserialize,Serialize)]
pub struct AlipayCycleOrderPayResponse {
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
    pub fund_bill_list: Option<Vec<TradeFundBill>>,
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


/// 周期付响应
#[derive(Debug, Deserialize,Serialize)]
pub struct AlipayPreAuthOnlinePayResponse {
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
    pub fund_bill_list: Option<Vec<TradeFundBill>>,
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
    /// 预授权支付模式，该参数仅在信用预授权支付场景下返回。信用预授权支付：CREDIT_PREAUTH_PAY
    pub auth_trade_pay_mode: Option<String>,
    /// 商家优惠金额
    pub mdiscount_amount: Option<String>,
    /// 平台优惠金额
    pub discount_amount: Option<String>,
}

/// 统一收单线下交易预创建响应
#[derive(Debug, Deserialize,Serialize)]
pub struct AlipayPreOrderResponse {
    /// 商家订单号
    pub out_trade_no: String,
    /// 当前预下单请求生成的二维码码串，有效时间2小时，可以用二维码生成工具根据该码串值生成对应的二维码
    pub qr_code: String,
}

/// 统一收单交易创建接口响应
#[derive(Debug, Deserialize,Serialize)]
pub struct AlipayCreateUnifiedOrderResponse {
    /// 商家订单号
    pub out_trade_no: String,
    /// 支付宝交易号
    pub trade_no: String,
}


#[derive(Debug, Deserialize,Serialize)]
pub struct EnterprisePayInfo {
    /// 是否包含因公付资产
    pub is_use_enterprise_pay: Option<bool>,
    /// 开票金额
    pub invoice_amount: Option<f64>,
    /// 因公付业务信息
    pub biz_info: Option<String>,
}

#[derive(Debug, Deserialize,Serialize)]
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



#[derive(Debug, Deserialize,Serialize)]
pub struct TradeFundBill {
    /// 交易使用的资金渠道，详见 支付渠道列表
    pub fund_channel: Option<String>,
    /// 渠道所使用的资金类型,目前只在资金渠道(fund_channel)是银行卡渠道(BANKCARD)的情况下才返回该信息(DEBIT_CARD:借记卡,CREDIT_CARD:信用卡,MIXED_CARD:借贷合一卡)
    pub fund_type: Option<String>,
    /// 该支付工具类型所使用的金额
    pub amount: Option<String>,
    /// 渠道实际付款金额
    pub real_amount: Option<f64>,
}


#[derive(Debug, Deserialize,Serialize)]
pub struct TradeSettleInfo {
    /// 交易结算明细信息
    pub trade_settle_detail_list: Option<Vec<TradeSettleDetail>>,
}

/// 交易结算明细信息
#[derive(Debug, Deserialize,Serialize)]
pub struct TradeSettleDetail {
    /// 结算操作类型。有以下几种类型：
    /// replenish(补差)、replenish_refund(退补差)、transfer(分账)、transfer_refund(退分账)、settle(结算)、settle_refund(退结算)、on_settle(待结算)。
    pub operation_type: String,
    /// 商户操作序列号。商户发起请求的外部请求号。
    pub operation_serial_no: Option<String>,
    /// 操作日期
    pub operation_dt: Option<String>,
    /// 转出账号
    pub trans_out: Option<String>,
    /// 商户请求的转出账号
    pub ori_trans_out: Option<String>,
    /// 商户请求的转入账号
    pub ori_trans_in: Option<String>,
    /// 转入账号
    pub trans_in: Option<String>,
    /// 实际操作金额，单位为元，两位小数。该参数的值为分账或补差或结算时传入
    pub amount: f64,
}


#[derive(Debug, Deserialize,Serialize)]
pub struct HbFqPayInfo {
    /// 用户使用花呗分期支付的分期数
    pub user_install_num: Option<String>,
}


#[derive(Debug, Deserialize,Serialize)]
pub struct AlipayCloseOrderResponse {
    /// 商户网站唯一订单号
    pub out_trade_no: Option<String>,
    /// 该交易在支付宝系统中的交易流水号。最长64位。
    pub trade_no: Option<String>,
}


//----------------------------------------------------------------------------------------------------------------------------


#[derive(Debug, Deserialize,Serialize)]
pub struct AlipayRefundOrderResponse {
    /// 商户网站唯一订单号
    pub out_trade_no: String,
    /// 该交易在支付宝系统中的交易流水号。最长64位。
    pub trade_no: String,
    /// 用户的登录id
    pub buyer_logon_id: String,
    /// 本次退款是否发生了资金变化
    pub fund_change: String,
    /// 退款总金额。
    /// 指该笔交易累计已经退款成功的金额。
    pub refund_fee: Option<String>,
    /// 交易在支付时候的门店名称
    pub store_name: Option<String>,
    /// 退款使用的资金渠道。
    /// 只有在签约中指定需要返回资金明细，或者入参的query_options中指定时才返回该字段信息。
    pub refund_detail_item_list: Option<Vec<TradeFundBill>>,
    /// 本次商户实际退回金额。
    /// 说明：如需获取该值，需在入参query_options中传入 refund_detail_item_list。
    pub send_back_fee: Option<String>,
    /// 买家在支付宝的用户id
    pub buyer_user_id: String,
}

//----------------------------------------------------------------------------------------------------------------------------


#[derive(Debug, Deserialize,Serialize)]
pub struct RefundRoyaltyResult {
    /// 退分账金额
    pub refund_amount: f64,
    /// 分账类型.
    /// 普通分账为：transfer;
    /// 补差为：replenish;
    /// 为空默认为分账transfer;
    pub royalty_type: Option<String>,
    /// 退分账结果码
    pub result_code: String,
    /// 转出人支付宝账号对应用户ID
    pub trans_out: Option<String>,
    /// 转出人支付宝账号
    pub trans_out_email: Option<String>,
    /// 转入人支付宝账号对应用户ID
    pub trans_in: Option<String>,
    /// 转入人支付宝账号
    pub trans_in_email: Option<String>,
}


#[derive(Debug, Deserialize,Serialize)]
pub struct DepositBackInfo {
    /// 是否存在银行卡冲退信息。
    pub has_deposit_back: Option<String>,
    /// 银行卡冲退状态。S-成功，F-失败，P-处理中。银行卡冲退失败，资金自动转入用户支付宝余额。
    pub dback_status: Option<String>,
    /// 银行卡冲退金额
    pub dback_amount: Option<f64>,
    /// 银行响应时间，格式为yyyy-MM-dd HH:mm:ss
    pub bank_ack_time: Option<String>,
    /// 预估银行到账时间，格式为yyyy-MM-dd HH:mm:ss
    pub est_bank_receipt_time: Option<String>,
    /// 是否包含因公付资产
    pub is_use_enterprise_pay: Option<bool>,
}



#[derive(Debug, Deserialize,Serialize)]
pub struct AlipayRefundQueryResponse {
    /// 商户网站唯一订单号
    pub out_trade_no: Option<String>,
    /// 该交易在支付宝系统中的交易流水号。最长64位。
    pub trade_no: Option<String>,
    /// 本笔退款对应的退款请求号
    pub out_request_no: Option<String>,
    /// 该笔退款所对应的交易的订单金额
    pub total_amount: Option<f64>,
    /// 本次退款请求，对应的退款金额
    pub refund_amount: Option<f64>,
    /// 退款状态。枚举值：
    /// <pre>
    /// REFUND_SUCCESS 退款处理成功；
    /// 未返回该字段表示退款请求未收到或者退款失败；
    /// 注：如果退款查询发起时间早于退款时间，或者间隔退款发起时间太短，可能出现退款查询时还没处理成功，后面又处理成功的情况，建议商户在退款发起后间隔10秒以上再发起退款查询请求。
    /// </pre>
    pub refund_status: Option<String>,
    /// 退分账明细信息	。
    pub refund_royaltys: Option<Vec<RefundRoyaltyResult>>,
    /// 退款时间。默认不返回该信息，需要在入参的query_options中指定"gmt_refund_pay"值时才返回该字段信息。
    pub gmt_refund_pay: Option<String>,
    /// 退款使用的资金渠道。
    /// 只有在签约中指定需要返回资金明细，或者入参的query_options中指定时才返回该字段信息。
    pub refund_detail_item_list: Option<Vec<TradeFundBill>>,
    /// 本次商户实际退回金额。
    /// 说明：如需获取该值，需在入参query_options中传入 refund_detail_item_list。
    pub send_back_fee: Option<String>,
    /// 银行卡冲退信息；
    /// 默认不返回该信息，需要在入参的query_options中指定"deposit_back_info"值时才返回该字段信息。
    pub deposit_back_info: Option<DepositBackInfo>,
}

//----------------------------------------------------------------------------------------------------------------------------


#[derive(Debug, Deserialize,Serialize)]
pub struct AlipayCancelOrderResponse {
    /// 商户网站唯一订单号
    pub out_trade_no: String,
    /// 该交易在支付宝系统中的交易流水号。最长64位。
    pub trade_no: String,
    /// 是否需要重试
    pub retry_flag: String,
    /// 本次撤销触发的交易动作,接口调用成功且交易存在时返回。可能的返回值：
    /// <pre>
    /// close：交易未支付，触发关闭交易动作，无退款；
    /// refund：交易已支付，触发交易退款动作；
    /// 未返回：未查询到交易，或接口调用失败；
    /// </pre>
    pub action: String,
}

//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Deserialize,Serialize)]
pub struct AlipayNotifyResponse {
    /// 通知的发送时间。格式为 yyyy-MM-dd HH:mm:ss
    pub notify_time: String,
    /// 通知类型 trade_status_sync
    pub notify_type: String,
    /// 通知校验 ID
    pub notify_id: String,
    /// 编码格式。如 utf-8、gbk、gb312等。
    pub charset: String,
    /// 调用的接口版本。固定为1.0
    pub version: String,
    /// 签名类型。签名算法类型，目前支持RSA2和RSA，推荐使用 RSA2
    pub sign_type: String,
    /// 签名。详情可查看 [异步返回结果的验签](https://opendocs.alipay.com/open/270/105902#%E5%BC%82%E6%AD%A5%E8%BF%94%E5%9B%9E%E7%BB%93%E6%9E%9C%E7%9A%84%E9%AA%8C%E7%AD%BE)
    pub sign: String,
    /// 授权方的APPID。由于本接口暂不开放第三方应用授权，因此 auth_app_id=app_id
    pub auth_app_id: String,

    // 以下为业务参数
    /// 支付宝交易号，支付宝交易凭证号。
    pub trade_no: String,
    /// 支付宝应用的APPID。支付宝分配给开发者的应用 ID
    pub app_id: String,
    /// 商家订单号。原支付请求的商家订单号
    pub out_trade_no: String,
    /// 商家业务号。商家业务ID，通常是退款通知中返回的退款申请流水号
    pub out_biz_no: Option<String>,
    /// 买家支付宝账号 ID。以 2088 开头的纯 16 位数字
    pub buyer_id: Option<String>,
    /// 卖家支付宝账号 ID。以 2088 开头的纯 16 位数字
    pub seller_id: Option<String>,
    /// 交易状态。交易目前所处状态，详情可查看下表 交易状态说明
    /// WAIT_BUYER_PAY
    /// <pre>
    /// 交易创建，等待买家付款。
    /// TRADE_CLOSED
    /// 未付款交易超时关闭，或支付完成后全额退款。
    /// TRADE_SUCCESS
    /// 交易支付成功。
    /// TRADE_FINISHED
    /// 交易结束，不可退款。
    /// </pre>
    pub trade_status: Option<String>,
    /// 订单金额。本次交易支付订单金额，单位为人民币（元），精确到小数点后 2 位
    pub total_amount: Option<f64>,
    /// 实收金额。商家在交易中实际收到的款项，单位为人民币（元），精确到小数点后 2 位
    pub receipt_amount: Option<f64>,
    /// 开票金额。用户在交易中支付的可开发票的金额，单位为人民币（元），精确到小数点后 2 位
    pub invoice_amount: Option<f64>,
    /// 用户在交易中支付的金额，单位为人民币（元），精确到小数点后 2 位
    pub buyer_pay_amount: Option<f64>,
    /// 使用集分宝支付金额，单位为人民币（元），精确到小数点后 2 位
    pub point_amount: Option<f64>,
    /// 总退款金额。退款通知中，返回总退款金额，单位为人民币（元），精确到小数点后 2 位
    pub refund_fee: Option<f64>,
    /// 订单标题/商品标题/交易标题/订单关键字等，是请求时对应参数，会在通知中原样传回
    pub subject: Option<String>,
    /// 商品描述。该订单的备注、描述、明细等。对应请求时的 body 参数，会在通知中原样传回
    pub body: Option<String>,
    /// 交易创建时间。格式为 yyyy-MM-dd HH:mm:ss
    pub gmt_create: Option<String>,
    /// 交易付款时间。格式为 yyyy-MM-dd HH:mm:ss
    pub gmt_payment: Option<String>,
    /// 交易退款时间。格式为 yyyy-MM-dd HH:mm:ss.S
    pub gmt_refund: Option<String>,
    /// 交易结束时间。格式为 yyyy-MM-dd HH:mm:ss
    pub gmt_close: Option<String>,
    /// 支付金额信息。支付成功的各个渠道金额信息。详情可查看下文 资金明细信息说明
    pub fund_bill_list: Option<String>,
    /// 优惠券信息。本交易支付时所使用的所有优惠券信息。详情可查看下表 优惠券信息说明
    pub vocher_detail_list: Option<String>,
    /// 回传参数，公共回传参数，如果请求时传递了该参数，则返回的异步通知会原样传回。本参数必须进行 UrlEncode 之后才可传入。
    pub passback_params: Option<String>,
}

//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Deserialize,Serialize)]
pub struct AlipaySystemOauthTokenResponse {
    /// 已废弃，请勿使用
    pub alipay_user_id: Option<String>,
    /// 支付宝用户的唯一标识。以2088开头的16位数字。
    pub user_id: String,
    /// 访问令牌。通过该令牌调用需要授权类接口
    pub access_token: String,
    /// 访问令牌的有效时间，单位是秒。
    pub expires_in: i64,
    /// 刷新令牌。通过该令牌可以刷新access_token
    pub refresh_token: String,
    /// 刷新令牌的有效时间，单位是秒。
    pub re_expires_in: i64,
    /// 授权token开始时间，作为有效期计算的起点
    pub auth_start: Option<String>,
}

//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Deserialize,Serialize)]
pub struct AlipayOpenAuthTokenAppResponse {
    /// 应用授权令牌
    pub app_auth_token: Option<String>,
    /// 刷新令牌
    pub app_refresh_token: Option<String>,
    /// 授权商户的appid
    pub auth_app_id: Option<String>,
    /// 该字段已作废，应用令牌长期有效，接入方不需要消费该字段
    pub expires_in: Option<String>,
    /// 刷新令牌的有效时间（从接口调用时间作为起始时间），单位到秒
    pub re_expires_in: Option<String>,
    /// 授权商户的user_id
    pub user_id: Option<String>,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct AppTokenExchangeSubElement {
    /// 已废弃，请勿使用
    pub app_auth_token: Option<String>,
    /// 支付宝用户的唯一标识。以2088开头的16位数字。
    pub app_refresh_token: String,
    /// 访问令牌。通过该令牌调用需要授权类接口
    pub auth_app_id: String,
    /// 刷新令牌。通过该令牌可以刷新access_token
    pub re_expires_in: String,
    /// 刷新令牌的有效时间，单位是秒。
    pub app_token_exchange_sub_element: i64,
    /// 授权token开始时间，作为有效期计算的起点
    pub user_id: Option<String>,
}

//----------------------------------------------------------------------------------------------------------------------------
