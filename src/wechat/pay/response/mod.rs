use serde::{Deserialize, Serialize};
use serde_json::{Value};

use crate::{Amount, errors::LabraError, GoodsDetail, LabradorResult, Payer, RefundAmount, SceneInfo, TradeType};
use crate::util::{get_nonce_str, get_timestamp, xmlutil};
use crate::wechat::cryptos::{EncryptV3, WeChatCrypto, WeChatCryptoV3};


//----------------------------------------------------------------------------------------------------------------------------

// 微信支付 ↓


#[derive(Debug, Serialize, Deserialize)]
pub struct WeChatPayResponse {
    pub appid: Option<String>,
    /// 交易类型
    pub trade_type: String,
    /// 商户号
    pub mch_id: String,
    /// 用户号
    pub openid: String,
    /// 签名
    pub sign: String,
    /// 加密字符串
    pub nonce_str: Option<String>,
    /// 预支付交易会话标识
    pub prepay_id: Option<String>,
    /// 业务结果
    pub result_code: String,
    /// trade_type为NATIVE时有返回，用于生成二维码，展示给用户进行扫码支付
    pub code_url: Option<String>,
    /// 错误代码
    pub err_code: Option<String>,
    /// 错误代码描述
    pub err_code_des: Option<String>,
    /// 支付跳转链接
    pub mweb_url: Option<String>,
    pub transaction_id: Option<String>,
}


#[allow(unused)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformCertificateResponse {
    /// 加密前的对象类型
    pub effective_time: String,
    /// 加密前的对象类型
    pub expire_time: String,
    /// 加密算法
    pub encrypt_certificate: EncryptV3,
    /// Base64编码后的密文
    pub serial_no: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeChatPayResponseV3 {
    pub prepay_id: Option<String>,
    /// 支付跳转链接（H5支付 会返回）
    pub h5_url: Option<String>,
    /// 二维码链接（NATIVE支付 会返回）
    pub code_url: Option<String>,
}

impl WeChatPayResponseV3 {
    pub fn get_pay_info(&self, trade_type: TradeType, appid: Option<String>, mchid: String, private_key: Option<String>) -> LabradorResult<Value> {
        let timpstamp = get_timestamp() / 1000;
        let nonce_str = get_nonce_str();
        let private_key= private_key.unwrap_or_default();
        let appid = appid.unwrap_or_default();
        match trade_type {
            TradeType::MWeb => {
                Ok(Value::String(self.h5_url.to_owned().unwrap_or_default()))
            }
            TradeType::Jsapi => {
                let mut result = JsapiResult {
                    app_id: appid.to_owned(),
                    time_stamp: timpstamp.to_string(),
                    nonce_str,
                    prepay_id: self.prepay_id.to_owned().unwrap_or_default(),
                    package: format!("prepay_id={}", self.prepay_id.to_owned().unwrap_or_default()),
                    sign_type: "RSA".to_string(), //签名类型，默认为RSA，仅支持RSA。
                    pay_sign: String::default()
                };
                result.pay_sign = WeChatCryptoV3::sign(&result.get_sign_str(), &private_key)?;
                Ok(serde_json::to_value(result)?)
            }
            TradeType::Native => {
                Ok(Value::String(self.code_url.to_owned().unwrap_or_default()))
            }
            TradeType::App => {
                let mut result = AppResult {
                    partner_id: mchid,
                    appid: appid.to_owned(),
                    time_stamp: timpstamp.to_string(),
                    nonce_str,
                    package_value: format!("Sign=WXPay"),
                    prepay_id: self.prepay_id.to_owned().unwrap_or_default(),
                    sign: "".to_string()
                };
                result.sign = WeChatCryptoV3::sign(&result.get_sign_str(), &private_key)?;
                Ok(serde_json::to_value(result)?)
            }
            _ => Err(LabraError::RequestError("不支持的支付类型".to_string()))
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
        format!("{}\n{}\n{}\n{}\n", self.app_id, self.time_stamp, self.nonce_str, self.package)
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
        format!("{}\n{}\n{}\n{}\n", self.appid, self.time_stamp, self.nonce_str, self.prepay_id)
    }
}


#[allow(unused)]
impl WeChatPayResponse {
    pub(crate) fn parse_xml(xml: String) -> LabradorResult<Self> {

        let package = xmlutil::parse(xml.to_owned());
        let doc = package.as_document();
        let return_code = xmlutil::evaluate(&doc, "//xml/return_code/text()").string();
        let return_msg = xmlutil::evaluate(&doc, "//xml/return_msg/text()").string();
        if return_code.eq(&"SUCCESS") {
            let appid = xmlutil::evaluate(&doc, "//xml/appid/text()").string();
            let trade_type = xmlutil::evaluate(&doc, "//xml/trade_type/text()").string();
            let mch_id = xmlutil::evaluate(&doc, "//xml/mch_id/text()").string();
            let openid = xmlutil::evaluate(&doc, "//xml/openid/text()").string();
            let nonce_str = xmlutil::evaluate(&doc, "//xml/nonce_str/text()").string();
            let prepay_id = xmlutil::evaluate(&doc, "//xml/prepay_id/text()").string();
            let result_code = xmlutil::evaluate(&doc, "//xml/result_code/text()").string();
            let err_code = xmlutil::evaluate(&doc, "//xml/err_code/text()").string();
            let err_code_des = xmlutil::evaluate(&doc, "//xml/err_code_des/text()").string();
            let code_url = xmlutil::evaluate(&doc, "//xml/code_url/text()").string();
            let mweb_url = xmlutil::evaluate(&doc, "//xml/mweb_url/text()").string();
            let transaction_id = xmlutil::evaluate(&doc, "//xml/transaction_id/text()").string();
            let sign = xmlutil::evaluate(&doc, "//xml/sign/text()").string();
            Ok(WeChatPayResponse {
                appid: appid.into(),
                trade_type,
                mch_id,
                openid,
                sign: sign.into(),
                nonce_str: nonce_str.into(),
                prepay_id: prepay_id.into(),
                result_code,
                code_url: code_url.into(),
                err_code: err_code.into(),
                err_code_des: err_code_des.into(),
                mweb_url: mweb_url.into(),
                transaction_id: transaction_id.into(),
            })
        } else {
            Err(LabraError::ClientError{ errcode: "-1".to_string(), errmsg: return_msg})
        }


    }
}



#[derive(Debug, Serialize, Deserialize)]
pub struct WeChatCloseOrderResponse {
    pub appid: Option<String>,
    /// 商户号
    pub mch_id: String,
    /// 签名
    pub sign: String,
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


///
/// 关闭订单返回
/// <xml>
/// <return_code><![CDATA[SUCCESS]]></return_code>
/// <return_msg><![CDATA[OK]]></return_msg>
/// <appid><![CDATA[wx2421b1c4370ec43b]]></appid>
/// <mch_id><![CDATA[10000100]]></mch_id>
/// <nonce_str><![CDATA[BFK89FC6rxKCOjLX]]></nonce_str>
/// <sign><![CDATA[72B321D92A7BFA0B2509F3D13C7B1631]]></sign>
/// <result_code><![CDATA[SUCCESS]]></result_code>
/// <result_msg><![CDATA[OK]]></result_msg>
/// </xml>
///
impl WeChatCloseOrderResponse {

    pub fn parse_xml(xml: String) -> LabradorResult<Self> {
        let package = xmlutil::parse(xml.to_owned());
        let doc = package.as_document();
        let return_code = xmlutil::evaluate(&doc, "//xml/return_code/text()").string();
        let return_msg = xmlutil::evaluate(&doc, "//xml/return_msg/text()").string();
        if return_code.eq(&"SUCCESS") {
            let appid = xmlutil::evaluate(&doc, "//xml/appid/text()").string();
            let mch_id = xmlutil::evaluate(&doc, "//xml/mch_id/text()").string();
            let nonce_str = xmlutil::evaluate(&doc, "//xml/nonce_str/text()").string();
            let result_code = xmlutil::evaluate(&doc, "//xml/result_code/text()").string();
            let sign = xmlutil::evaluate(&doc, "//xml/sign/text()").string();
            let err_code = xmlutil::evaluate(&doc, "//xml/err_code/text()").string();
            let err_code_des = xmlutil::evaluate(&doc, "//xml/err_code_des/text()").string();
            Ok(WeChatCloseOrderResponse {
                appid: appid.into(),
                nonce_str: nonce_str.into(),
                mch_id,
                return_code,
                sign,
                err_code: err_code.into(),
                err_code_des: err_code_des.into(),
                return_msg,
                result_code,
            })
        } else {
            Err(LabraError::ClientError{ errcode: "-1".to_string(), errmsg: return_msg})
        }

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
    #[serde(rename="type")]
    pub r#type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefundPromotionDetail {
    /// 券ID
    pub promotion_id: String,
    /// 优惠范围 GLOBAL：全场代金券 SINGLE：单品优惠
    pub scope: Option<String>,
    /// COUPON：代金券，需要走结算资金的充值型代金券 *  DISCOUNT：优惠券，不走结算资金的免充值型优惠券
    #[serde(rename="type")]
    pub r#type: Option<String>,
    /// 优惠券面额
    pub amount: i64,
    /// 优惠退款金额<=退款金额，退款金额-代金券或立减优惠退款金额为用户支付的现金，说明详见代金券或立减优惠，单位为分
    pub refund_amount: i64,
    /// 单品列表
    pub goods_detail: Option<GoodsDetail>,

}


#[derive(Debug, Serialize, Deserialize)]
pub struct WeChatQueryOrderResponseV3 {
    pub appid: String,
    /// 商户号
    pub mch_id: String,
    /// 商户系统的订单号，与请求一致。
    pub out_trade_no: String,
    /// 交易类型 调用接口提交的交易类型，取值如下：JSAPI，NATIVE，APP，MICROPAY，详细说明见参数规定
    pub trade_type: String,
    /// 微信支付订单号
    pub transaction_id: String,
    /// SUCCESS—支付成功,REFUND—转入退款,NOTPAY—未支付,CLOSED—已关闭,REVOKED—已撤销（刷卡支付）,USERPAYING--用户支付中,PAYERROR--支付失败(其他原因，如银行返回失败)
    pub trade_state: String,
    /// 交易状态描述
    pub trade_state_desc: String,
    /// 付款银行
    pub bank_type: String,
    /// 附加数据，原样返回
    pub attach: Option<String>,
    /// 支付完成时间，遵循rfc3339标准格式，格式为YYYY-MM-DDTHH:mm:ss+TIMEZONE，YYYY-MM-DD表示年月日，T出现在字符串中，表示time元素的开头，HH:mm:ss表示时分秒，TIMEZONE表示时区（+08:00表示东八区时间，领先UTC 8小时，即北京时间）。例如：2015-05-20T13:29:35+08:00表示，北京时间2015年5月20日 13点29分35秒。
    pub success_time: String,
    /// 支付者
    pub payer: Payer,
    /// 订单金额信息，当支付成功时返回该字段。
    pub amount: Option<Amount>,
    /// 场景信息
    pub scene_info: Option<SceneInfo>,
    /// 优惠功能，享受优惠时返回该字段。
    pub promotion_detail: Option<PromotionDetail>,



    /// 货币类型，符合ISO 4217标准的三位字母代码，默认人民币：CNY，其他值列表详见货币类型
    pub fee_type: Option<String>,
    /// 订单金额
    pub total_fee: i64,
    /// 应结订单金额=订单金额-非充值代金券金额，应结订单金额<=订单金额。
    pub settlement_total_fee: Option<i64>,
    /// “代金券”金额<=订单金额，订单金额-“代金券”金额=现金支付金额，详见支付金额
    pub coupon_fee: Option<i64>,
    /// 代金券使用数量
    pub coupon_count: Option<i64>,
    /// 现金支付金额订单现金支付金额，详见支付金额
    pub cash_fee: i64,
    /// 货币类型，符合ISO 4217标准的三位字母代码，默认人民币：CNY，其他值列表详见货币类型
    pub cash_fee_type: Option<String>,
}




#[derive(Debug, Serialize, Deserialize)]
pub struct WeChatQueryOrderResponse {
    pub appid: Option<String>,
    /// 商户号
    pub mch_id: String,
    /// 签名
    pub sign: String,
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
    /// 营销详情
    pub promotion_detail: Option<String>,
    /// 设备号
    pub device_info: Option<String>,
    /// 用户标识
    pub openid: String,
    /// 用户是否关注公众账号，Y-关注，N-未关注，仅在公众账号类型支付有效
    pub is_subscribe: String,
    /// 用户子标识
    pub sub_openid: Option<String>,
    /// 交易类型 调用接口提交的交易类型，取值如下：JSAPI，NATIVE，APP，MICROPAY，详细说明见参数规定
    pub trade_type: String,
    /// SUCCESS—支付成功,REFUND—转入退款,NOTPAY—未支付,CLOSED—已关闭,REVOKED—已撤销（刷卡支付）,USERPAYING--用户支付中,PAYERROR--支付失败(其他原因，如银行返回失败)
    pub trade_state: String,
    /// 付款银行
    pub bank_type: String,
    /// 微信支付订单号
    pub transaction_id: String,
    /// 商户系统的订单号，与请求一致。
    pub out_trade_no: String,
    /// 订单支付时间，格式为yyyyMMddHHmmss，如2009年12月25日9点10分10秒表示为20091225091010。其他详见时间规则
    pub time_end: String,
    /// 支付失败，请重新下单支付 * 对当前查询订单状态的描述和下一步操作的指引
    pub trade_state_desc: String,
    /// 商品详情
    pub detail: Option<String>,
    /// 附加数据，原样返回
    pub attach: Option<String>,
    /// 货币类型，符合ISO 4217标准的三位字母代码，默认人民币：CNY，其他值列表详见货币类型
    pub fee_type: Option<String>,
    /// 订单金额
    pub total_fee: i64,
    /// 应结订单金额=订单金额-非充值代金券金额，应结订单金额<=订单金额。
    pub settlement_total_fee: Option<i64>,
    /// “代金券”金额<=订单金额，订单金额-“代金券”金额=现金支付金额，详见支付金额
    pub coupon_fee: Option<i64>,
    /// 代金券使用数量
    pub coupon_count: Option<i64>,
    /// 现金支付金额订单现金支付金额，详见支付金额
    pub cash_fee: i64,
    /// 货币类型，符合ISO 4217标准的三位字母代码，默认人民币：CNY，其他值列表详见货币类型
    pub cash_fee_type: Option<String>,
}


///
/// 查询订单返回
///
impl WeChatQueryOrderResponse {
    pub fn parse_xml(xml: String) -> LabradorResult<Self> {
        let package = xmlutil::parse(xml.to_owned());
        let doc = package.as_document();
        let return_code = xmlutil::evaluate(&doc, "//xml/return_code/text()").string();
        let return_msg = xmlutil::evaluate(&doc, "//xml/return_msg/text()").string();
        if return_code.eq(&"SUCCESS") {
            let appid = xmlutil::evaluate(&doc, "//xml/appid/text()").string();
            let mch_id = xmlutil::evaluate(&doc, "//xml/mch_id/text()").string();
            let nonce_str = xmlutil::evaluate(&doc, "//xml/nonce_str/text()").string();
            let result_code = xmlutil::evaluate(&doc, "//xml/result_code/text()").string();
            let sign = xmlutil::evaluate(&doc, "//xml/sign/text()").string();
            let err_code = xmlutil::evaluate(&doc, "//xml/err_code/text()").string();
            let err_code_des = xmlutil::evaluate(&doc, "//xml/err_code_des/text()").string();
            let promotion_detail = xmlutil::evaluate(&doc, "//xml/promotion_detail/text()").string();
            let device_info = xmlutil::evaluate(&doc, "//xml/device_info/text()").string();
            let openid = xmlutil::evaluate(&doc, "//xml/openid/text()").string();
            let is_subscribe = xmlutil::evaluate(&doc, "//xml/is_subscribe/text()").string();
            let sub_openid = xmlutil::evaluate(&doc, "//xml/sub_openid/text()").string();
            let trade_type = xmlutil::evaluate(&doc, "//xml/trade_type/text()").string();
            let trade_state = xmlutil::evaluate(&doc, "//xml/trade_state/text()").string();
            let bank_type = xmlutil::evaluate(&doc, "//xml/bank_type/text()").string();
            let transaction_id = xmlutil::evaluate(&doc, "//xml/transaction_id/text()").string();
            let out_trade_no = xmlutil::evaluate(&doc, "//xml/out_trade_no/text()").string();
            let time_end = xmlutil::evaluate(&doc, "//xml/time_end/text()").string();
            let trade_state_desc = xmlutil::evaluate(&doc, "//xml/trade_state_desc/text()").string();
            let detail = xmlutil::evaluate(&doc, "//xml/detail/text()").string();
            let attach = xmlutil::evaluate(&doc, "//xml/attach/text()").string();
            let fee_type = xmlutil::evaluate(&doc, "//xml/fee_type/text()").string();
            let total_fee = xmlutil::evaluate(&doc, "//xml/total_fee/text()").string();
            let settlement_total_fee = xmlutil::evaluate(&doc, "//xml/settlement_total_fee/text()").string();
            let coupon_fee = xmlutil::evaluate(&doc, "//xml/coupon_fee/text()").string();
            let coupon_count = xmlutil::evaluate(&doc, "//xml/coupon_count/text()").string();
            let cash_fee = xmlutil::evaluate(&doc, "//xml/cash_fee/text()").string();
            let cash_fee_type = xmlutil::evaluate(&doc, "//xml/cash_fee_type/text()").string();
            Ok(WeChatQueryOrderResponse {
                appid: appid.into(),
                nonce_str: nonce_str.into(),
                mch_id,
                return_code,
                sign,
                err_code: err_code.into(),
                err_code_des: err_code_des.into(),
                promotion_detail: promotion_detail.into(),
                device_info: device_info.into(),
                openid,
                is_subscribe,
                sub_openid: sub_openid.into(),
                trade_type,
                trade_state,
                bank_type,
                transaction_id,
                out_trade_no,
                time_end,
                trade_state_desc,
                detail: detail.into(),
                attach: attach.into(),
                fee_type: fee_type.into(),
                total_fee: total_fee.parse::<i64>().unwrap_or_default(),
                settlement_total_fee: settlement_total_fee.parse::<i64>().unwrap_or_default().into(),
                coupon_fee: coupon_fee.parse::<i64>().unwrap_or_default().into(),
                coupon_count: coupon_count.parse::<i64>().unwrap_or_default().into(),
                cash_fee: cash_fee.parse::<i64>().unwrap_or_default(),
                return_msg,
                result_code,
                cash_fee_type: cash_fee_type.into()
            })
        } else {
            Err(LabraError::ClientError{ errcode: "-1".to_string(), errmsg: return_msg})
        }

    }
}






#[derive(Debug, Serialize, Deserialize)]
pub struct WeChatRefundResponseV3 {
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
    pub amount : RefundAmount,
    /// 优惠退款信息
    pub promotion_detail : Option<Vec<RefundPromotionDetail>>,
}




#[derive(Debug, Serialize, Deserialize)]
pub struct WeChatRefundResponse {
    pub appid: Option<String>,
    /// 商户号
    pub mch_id: String,
    /// 签名
    pub sign: String,
    /// 微信交易编号
    pub transaction_id: String,
    /// 商户订单编号
    pub out_trade_no: String,
    /// 退款单号
    pub out_refund_no: String,
    /// 退款编号
    pub refund_id: String,
    /// 加密字符串
    pub nonce_str: Option<String>,
    /// 退款渠道
    pub refund_channel: Option<String>,
    /// 业务结果
    pub result_code: String,
    pub return_code: String,
    /// 返回结果
    pub return_msg: String,
    /// 退款金额
    pub refund_fee: String,
    /// 错误码
    pub err_code: Option<String>,
    pub err_code_des: Option<String>,
}




#[derive(Debug, Serialize, Deserialize)]
pub struct WeChatOrderReverseResponse {
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


impl WeChatOrderReverseResponse {

    pub fn parse_xml(xml: String) -> LabradorResult<Self> {
        let package = xmlutil::parse(xml.to_owned());
        let doc = package.as_document();
        let return_code = xmlutil::evaluate(&doc, "//xml/return_code/text()").string();
        let return_msg = xmlutil::evaluate(&doc, "//xml/return_msg/text()").string();
        if return_code.eq(&"SUCCESS") {
            let appid = xmlutil::evaluate(&doc, "//xml/appid/text()").string();
            let recall = xmlutil::evaluate(&doc, "//xml/recall/text()").string();
            let mch_id = xmlutil::evaluate(&doc, "//xml/mch_id/text()").string();
            let nonce_str = xmlutil::evaluate(&doc, "//xml/nonce_str/text()").string();
            let result_code = xmlutil::evaluate(&doc, "//xml/result_code/text()").string();
            let sign = xmlutil::evaluate(&doc, "//xml/sign/text()").string();
            let err_code = xmlutil::evaluate(&doc, "//xml/err_code/text()").string();
            let err_code_des = xmlutil::evaluate(&doc, "//xml/err_code_des/text()").string();
            if result_code.eq("SUCCESS") {
                Ok(WeChatOrderReverseResponse {
                    appid: appid.into(),
                    nonce_str: nonce_str.into(),
                    mch_id,
                    return_code,
                    sign,
                    err_code: err_code.into(),
                    err_code_des: err_code_des.into(),
                    return_msg,
                    result_code,
                    recall
                })
            } else {
                Err(LabraError::ClientError{ errcode: "-1".to_string(), errmsg: err_code_des})
            }

        } else {
            Err(LabraError::ClientError{ errcode: "-1".to_string(), errmsg: return_msg})
        }
    }
}

///
/// 退款返回
/// <xml>
///    <return_code><![CDATA[SUCCESS]]></return_code>
///    <return_msg><![CDATA[OK]]></return_msg>
///    <appid><![CDATA[wx2421b1c4370ec43b]]></appid>
///    <mch_id><![CDATA[10000100]]></mch_id>
///    <nonce_str><![CDATA[NfsMFbUFpdbEhPXP]]></nonce_str>
///    <sign><![CDATA[B7274EB9F8925EB93100DD2085FA56C0]]></sign>
///    <result_code><![CDATA[SUCCESS]]></result_code>
///    <transaction_id><![CDATA[1008450740201411110005820873]]></transaction_id>
///    <out_trade_no><![CDATA[1415757673]]></out_trade_no>
///    <out_refund_no><![CDATA[1415701182]]></out_refund_no>
///    <refund_id><![CDATA[2008450740201411110000174436]]></refund_id>
///    <refund_channel><![CDATA[]]></refund_channel>
///    <refund_fee>1</refund_fee>
/// </xml>
///
impl WeChatRefundResponse {
    pub fn parse_xml(xml: String) -> LabradorResult<Self> {
        let package = xmlutil::parse(xml.to_owned());
        let doc = package.as_document();
        let return_code = xmlutil::evaluate(&doc, "//xml/return_code/text()").string();
        let return_msg = xmlutil::evaluate(&doc, "//xml/return_msg/text()").string();
        if return_code.eq(&"SUCCESS") {
            let appid = xmlutil::evaluate(&doc, "//xml/appid/text()").string();
            let transaction_id = xmlutil::evaluate(&doc, "//xml/transaction_id/text()").string();
            let mch_id = xmlutil::evaluate(&doc, "//xml/mch_id/text()").string();
            let nonce_str = xmlutil::evaluate(&doc, "//xml/nonce_str/text()").string();
            let result_code = xmlutil::evaluate(&doc, "//xml/result_code/text()").string();
            let out_trade_no = xmlutil::evaluate(&doc, "//xml/out_trade_no/text()").string();
            let out_refund_no = xmlutil::evaluate(&doc, "//xml/out_refund_no/text()").string();
            let refund_id = xmlutil::evaluate(&doc, "//xml/refund_id/text()").string();
            let refund_channel = xmlutil::evaluate(&doc, "//xml/refund_channel/text()").string();
            let refund_fee = xmlutil::evaluate(&doc, "//xml/refund_fee/text()").string();
            let sign = xmlutil::evaluate(&doc, "//xml/sign/text()").string();
            let err_code = xmlutil::evaluate(&doc, "//xml/err_code/text()").string();
            let err_code_des = xmlutil::evaluate(&doc, "//xml/err_code_des/text()").string();
            if result_code.eq("SUCCESS") {
                Ok(WeChatRefundResponse {
                    appid: appid.into(),
                    nonce_str: nonce_str.into(),
                    mch_id,
                    return_code,
                    sign,
                    err_code: err_code.into(),
                    err_code_des: err_code_des.into(),
                    return_msg,
                    result_code,
                    transaction_id,
                    out_trade_no,
                    out_refund_no,
                    refund_id,
                    refund_channel: refund_channel.into(),
                    refund_fee,

                })
            } else {
                Err(LabraError::ClientError{ errcode: "-1".to_string(), errmsg: err_code_des})
            }

        } else {
            Err(LabraError::ClientError{ errcode: "-1".to_string(), errmsg: return_msg})
        }

    }
}




#[derive(Debug, Serialize, Deserialize)]
pub struct WeChatQueryRefundResponse {
    pub appid: Option<String>,
    /// 商户号
    pub mch_id: String,
    /// 签名
    pub sign: String,
    /// 微信交易编号
    pub transaction_id: String,
    /// 商户订单编号
    pub out_trade_no: String,
    /// 订单总金额，单位为分，只能为整数，详见支付金额
    pub total_fee: i64,
    /// 应结订单金额=订单金额-非充值代金券金额，应结订单金额<=订单金额。
    pub settlement_total_fee: Option<i64>,
    /// 订单金额货币类型，符合ISO 4217标准的三位字母代码，默认人民币：CNY，其他值列表详见货币类型
    pub fee_type: Option<String>,
    /// 现金支付金额，单位为分，只能为整数，详见支付金额
    pub cash_fee: i64,
    /// 退款笔数
    pub refund_count: i64,
    /// 营销详情
    pub promotion_detail: Option<String>,
    /// 加密字符串
    pub nonce_str: Option<String>,
    /// 业务结果
    pub result_code: String,
    pub return_code: String,
    /// 返回结果
    pub return_msg: String,
    /// 退款金额
    pub refund_fee: String,
    /// 错误码
    pub err_code: Option<String>,
    pub err_code_des: Option<String>,
}


impl WeChatQueryRefundResponse {
    pub fn parse_xml(xml: String) -> LabradorResult<Self> {
        let package = xmlutil::parse(xml.to_owned());
        let doc = package.as_document();
        let return_code = xmlutil::evaluate(&doc, "//xml/return_code/text()").string();
        let return_msg = xmlutil::evaluate(&doc, "//xml/return_msg/text()").string();
        if return_code.eq(&"SUCCESS") {
            let appid = xmlutil::evaluate(&doc, "//xml/appid/text()").string();
            let transaction_id = xmlutil::evaluate(&doc, "//xml/transaction_id/text()").string();
            let mch_id = xmlutil::evaluate(&doc, "//xml/mch_id/text()").string();
            let nonce_str = xmlutil::evaluate(&doc, "//xml/nonce_str/text()").string();
            let result_code = xmlutil::evaluate(&doc, "//xml/result_code/text()").string();
            let out_trade_no = xmlutil::evaluate(&doc, "//xml/out_trade_no/text()").string();
            let refund_fee = xmlutil::evaluate(&doc, "//xml/refund_fee/text()").string();
            let sign = xmlutil::evaluate(&doc, "//xml/sign/text()").string();
            let err_code = xmlutil::evaluate(&doc, "//xml/err_code/text()").string();
            let err_code_des = xmlutil::evaluate(&doc, "//xml/err_code_des/text()").string();
            let total_fee = xmlutil::evaluate(&doc, "//xml/total_fee/text()").string();
            let settlement_total_fee = xmlutil::evaluate(&doc, "//xml/settlement_total_fee/text()").string();
            let fee_type = xmlutil::evaluate(&doc, "//xml/fee_type/text()").string();
            let cash_fee = xmlutil::evaluate(&doc, "//xml/cash_fee/text()").string();
            if result_code.eq("SUCCESS") {
                Ok(WeChatQueryRefundResponse {
                    appid: appid.into(),
                    nonce_str: nonce_str.into(),
                    mch_id,
                    return_code,
                    sign,
                    err_code: err_code.into(),
                    err_code_des: err_code_des.into(),
                    return_msg,
                    result_code,
                    transaction_id,
                    out_trade_no,
                    total_fee: total_fee.parse::<i64>().unwrap_or_default(),
                    settlement_total_fee: settlement_total_fee.parse::<i64>().unwrap_or_default().into(),
                    fee_type: fee_type.into(),
                    cash_fee: cash_fee.parse::<i64>().unwrap_or_default(),
                    refund_count: 0,
                    refund_fee,

                    promotion_detail: None
                })
            } else {
                Err(LabraError::ClientError{ errcode: "-1".to_string(), errmsg: err_code_des})
            }

        } else {
            Err(LabraError::ClientError{ errcode: "-1".to_string(), errmsg: return_msg})
        }

    }
}






#[derive(Debug, Serialize, Deserialize)]
pub struct WeChatQueryRefundResponseV3 {
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
pub struct WeChatPayNotifyResponse {
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


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WeChatPayNotifyResponseV3 {
    /// 源数据
    pub raw_data: Option<OriginNotifyResponse>,
    /// 解密后的数据
    pub result: Option<DecryptNotifyResult>,
}

impl WeChatPayNotifyResponseV3 {
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
                payer: Payer { openid: "".to_string() },
                amount: None
            }
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WeChatRefundNotifyResponseV3 {
    /// 源数据
    pub raw_data: Option<OriginNotifyResponse>,
    /// 解密后的数据
    pub result: Option<DecryptRefundNotifyResult>,
}

impl WeChatRefundNotifyResponseV3 {
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
                    currency: None
                },
                refund_status: "".to_string(),
                user_received_account: "".to_string()
            }
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WxScanPayNotifyResponse {
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
}

impl WxScanPayNotifyResponse {
    pub fn parse_xml(xml: String) -> LabradorResult<Self> {
        let package = xmlutil::parse(xml.to_owned());
        let doc = package.as_document();
        let return_code = xmlutil::evaluate(&doc, "//xml/return_code/text()").string();
        let return_msg = xmlutil::evaluate(&doc, "//xml/return_msg/text()").string();
        if return_code.eq(&"SUCCESS") {
            let openid = xmlutil::evaluate(&doc, "//xml/openid/text()").string();
            let is_subscribe = xmlutil::evaluate(&doc, "//xml/is_subscribe/text()").string();
            let product_id = xmlutil::evaluate(&doc, "//xml/product_id/text()").string();
            let result_code = xmlutil::evaluate(&doc, "//xml/result_code/text()").string();
            let err_code_des = xmlutil::evaluate(&doc, "//xml/err_code_des/text()").string();
            let _sign = xmlutil::evaluate(&doc, "//xml/sign/text()").string();
            let _err_code = xmlutil::evaluate(&doc, "//xml/err_code/text()").string();
            if result_code.eq("SUCCESS") {
                Ok(WxScanPayNotifyResponse {
                    openid,
                    is_subscribe,
                    product_id
                })
            } else {
                Err(LabraError::ClientError{ errcode: "-1".to_string(), errmsg: err_code_des})
            }

        } else {
            Err(LabraError::ClientError{ errcode: "-1".to_string(), errmsg: return_msg})
        }

    }
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
    pub resource: EncryptV3,
}

#[allow(unused)]
impl WeChatPayNotifyResponse {
    pub fn parse_xml(xml: String) -> LabradorResult<Self> {
        let package = xmlutil::parse(xml.to_owned());
        let doc = package.as_document();
        let return_code = xmlutil::evaluate(&doc, "//xml/return_code/text()").string();
        let return_msg = xmlutil::evaluate(&doc, "//xml/return_msg/text()").string();
        if return_code.eq(&"SUCCESS") {
            let appid = xmlutil::evaluate(&doc, "//xml/appid/text()").string();
            let trade_type = xmlutil::evaluate(&doc, "//xml/trade_type/text()").string();
            let mch_id = xmlutil::evaluate(&doc, "//xml/mch_id/text()").string();
            let openid = xmlutil::evaluate(&doc, "//xml/openid/text()").string();
            let nonce_str = xmlutil::evaluate(&doc, "//xml/nonce_str/text()").string();
            let result_code = xmlutil::evaluate(&doc, "//xml/result_code/text()").string();
            let bank_type = xmlutil::evaluate(&doc, "//xml/bank_type/text()").string();
            let fee_type = xmlutil::evaluate(&doc, "//xml/fee_type/text()").string();
            let sign = xmlutil::evaluate(&doc, "//xml/sign/text()").string();
            let is_subscribe = xmlutil::evaluate(&doc, "//xml/is_subscribe/text()").string();
            let out_trade_no = xmlutil::evaluate(&doc, "//xml/out_trade_no/text()").string();
            let time_end = xmlutil::evaluate(&doc, "//xml/time_end/text()").string();
            let total_fee = xmlutil::evaluate(&doc, "//xml/total_fee/text()").string();
            let cash_fee = xmlutil::evaluate(&doc, "//xml/cash_fee/text()").string();
            let coupon_fee = xmlutil::evaluate(&doc, "//xml/coupon_fee/text()").string();
            let coupon_count = xmlutil::evaluate(&doc, "//xml/coupon_count/text()").string();
            let coupon_id = xmlutil::evaluate(&doc, "//xml/coupon_id/text()").string();
            let coupon_type = xmlutil::evaluate(&doc, "//xml/coupon_type/text()").string();
            let transaction_id = xmlutil::evaluate(&doc, "//xml/transaction_id/text()").string();
            let attach = xmlutil::evaluate(&doc, "//xml/attach/text()").string();
            let err_code = xmlutil::evaluate(&doc, "//xml/err_code/text()").string();
            let err_code_des = xmlutil::evaluate(&doc, "//xml/err_code_des/text()").string();
            Ok(WeChatPayNotifyResponse {
                appid: appid.into(),
                trade_type,
                bank_type: bank_type.into(),
                fee_type: fee_type.into(),
                is_subscribe: is_subscribe.into(),
                nonce_str: nonce_str.into(),
                mch_id,
                openid,
                out_trade_no,
                return_code,
                sign,
                err_code: err_code.into(),
                err_code_des: err_code_des.into(),
                time_end,
                total_fee,
                return_msg,
                coupon_fee: coupon_fee.into(),
                coupon_count: coupon_count.into(),
                coupon_type: coupon_type.into(),
                coupon_id: coupon_id.into(),
                transaction_id,
                attach: attach.into(),
                result_code,
                cash_fee,
            })
        } else {
            Err(LabraError::ClientError{ errcode: "-1".to_string(), errmsg: return_msg})
        }

    }

}



///
/// <xml>
/// <return_code>SUCCESS</return_code>
/// <appid><![CDATA[wx2421b1c4370ec43b]]></appid>
/// <mch_id><![CDATA[10000100]]></mch_id>
/// <nonce_str><![CDATA[TeqClE3i0mvn3DrK]]></nonce_str>
/// <req_info><![CDATA[T87GAHG17TGAHG1TGHAHAHA1Y1CIOA9UGJH1GAHV871HAGAGQYQQPOOJMXNBCXBVNMNMAJAA]]></req_info>
/// </xml>
///
#[derive(Debug, Serialize, Deserialize)]
pub struct WeChatRefundNotifyResponse {
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



///
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
///
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct WeChatDecryptRefundNotifyResponse {
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

#[allow(unused)]
impl WeChatRefundNotifyResponse {
    pub fn parse_xml(xml: String, appkey: &str) -> LabradorResult<WeChatDecryptRefundNotifyResponse> {
        let package = xmlutil::parse(xml.to_owned());
        let doc = package.as_document();
        let return_code = xmlutil::evaluate(&doc, "//xml/return_code/text()").string();
        let return_msg = xmlutil::evaluate(&doc, "//xml/return_msg/text()").string();
        if return_code.eq(&"SUCCESS") {
            let _appid = xmlutil::evaluate(&doc, "//xml/appid/text()").string();
            let _result_code = xmlutil::evaluate(&doc, "//xml/result_code/text()").string();
            let _return_msg = xmlutil::evaluate(&doc, "//xml/return_msg/text()").string();
            let req_info = xmlutil::evaluate(&doc, "//xml/req_info/text()").string();
            let _refund_msg = WeChatCrypto::decrypt_data_refund(appkey,req_info.as_str())?;
            let refund_package = xmlutil::parse(_refund_msg.to_owned());
            let refund_doc = refund_package.as_document();
            let out_refund_no = xmlutil::evaluate(&refund_doc, "//root/out_refund_no/text()").string();
            let out_trade_no = xmlutil::evaluate(&refund_doc, "//root/out_trade_no/text()").string();
            let refund_account = xmlutil::evaluate(&refund_doc, "//root/refund_account/text()").string();
            let refund_fee = xmlutil::evaluate(&refund_doc, "//root/refund_fee/text()").string();
            let refund_id = xmlutil::evaluate(&refund_doc, "//root/refund_id/text()").string();
            let refund_recv_accout = xmlutil::evaluate(&refund_doc, "//root/refund_recv_accout/text()").string();
            let refund_request_source = xmlutil::evaluate(&refund_doc, "//root/refund_request_source/text()").string();
            let refund_status = xmlutil::evaluate(&refund_doc, "//root/refund_status/text()").string();
            let settlement_refund_fee = xmlutil::evaluate(&refund_doc, "//root/settlement_refund_fee/text()").string();
            let settlement_total_fee = xmlutil::evaluate(&refund_doc, "//root/settlement_total_fee/text()").string();
            let success_time = xmlutil::evaluate(&refund_doc, "//root/success_time/text()").string();
            let total_fee = xmlutil::evaluate(&refund_doc, "//root/total_fee/text()").string();
            let transaction_id = xmlutil::evaluate(&refund_doc, "//root/transaction_id/text()").string();
            Ok(WeChatDecryptRefundNotifyResponse {
                out_refund_no,
                out_trade_no,
                refund_account,
                refund_fee,
                refund_id,
                refund_recv_accout,
                refund_request_source,
                refund_status,
                settlement_refund_fee,
                settlement_total_fee,
                success_time,
                total_fee,
                transaction_id,
            })
        } else {
            Err(LabraError::ClientError{ errcode: "-1".to_string(), errmsg: return_msg})
        }

    }
}


/// 转换短链接结果对象类
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct WxPayShortUrlResponse {
    /// <pre>
    /// URL链接
    /// short_url
    /// 是
    /// String(64)
    /// weixin：//wxpay/s/XXXXXX
    /// 转换后的URL
    /// </pre>
    pub short_url: String,
}

#[allow(unused)]
impl WxPayShortUrlResponse {
    pub fn parse_xml(xml: String) -> LabradorResult<WxPayShortUrlResponse> {
        let package = xmlutil::parse(xml.to_owned());
        let doc = package.as_document();
        let return_code = xmlutil::evaluate(&doc, "//xml/return_code/text()").string();
        let return_msg = xmlutil::evaluate(&doc, "//xml/return_msg/text()").string();
        if return_code.eq(&"SUCCESS") {
            let short_url = xmlutil::evaluate(&doc, "//xml/short_url/text()").string();
            Ok(WxPayShortUrlResponse {
                short_url,
            })
        } else {
            Err(LabraError::ClientError{ errcode: "-1".to_string(), errmsg: return_msg})
        }

    }
}