use crate::{RequestMethod, TradeType};

#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum WechatPayMethod {
    /// 微信支付
    WxPay(WxPayMethod),
    /// 企业支付
    EntPay(EntPayMethod),
    /// 证书下载
    Certificate,
}

#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum EntPayMethod {

}

#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum WxPayMethod {
    /// 统一下单
    UnifiedOrder,
    /// 统一下单 -- V3
    UnifiedOrderV3(TradeType),
    IsvUnifiedOrderV3(TradeType),
    /// 收款码
    MicroPay,
    /// 关闭订单
    CloseOrder,
    /// 关闭订单 -- V3
    CloseOrderV3(String),
    /// 查询订单
    QueryOrder,
    /// 查询订单 -- V3
    QueryOrderV3((Option<String>, Option<String>)),
    /// 查询退款订单
    QueryRefundOrder,
    /// 查询退款订单 -- V2
    QueryRefundOrderV2,
    /// 查询退款订单 -- V3
    QueryRefundOrderV3(String),
    /// 退款
    Refund,
    /// 退款 - V3
    RefundV3,
    /// 撤销订单 - V3
    ReverseOrder,
    /// 转换短链接
    ShortUrl,
}


#[allow(unused)]
impl RequestMethod for WechatPayMethod {
    fn get_method(&self) -> String {
        match self {
            WechatPayMethod::WxPay(v) => v.get_method(),
            WechatPayMethod::EntPay(_) => {
                String::default()
            }
            WechatPayMethod::Certificate => String::from("/v3/certificates")
        }
    }
}

#[allow(unused)]
impl WxPayMethod {
    pub fn get_method(&self) -> String {
        match self {
            WxPayMethod::UnifiedOrder => String::from("/pay/unifiedorder"),
            WxPayMethod::MicroPay => String::from("/pay/micropay"),
            WxPayMethod::CloseOrder => String::from("/pay/closeorder"),
            WxPayMethod::Refund => String::from("/pay/refund"),
            WxPayMethod::RefundV3 => String::from("/v3/refund/domestic/refunds"),
            WxPayMethod::QueryOrder => String::from("/pay/orderquery"),
            WxPayMethod::ShortUrl => String::from("/tools/shorturl"),
            WxPayMethod::QueryOrderV3((otr, tid)) => {
                if let Some(otr) = otr {
                    format!("/v3/pay/transactions/out-trade-no/{}", otr)
                } else {
                    let tid = tid.to_owned().unwrap_or_default();
                    format!("/v3/pay/transactions/id/{}", tid)
                }
            },
            WxPayMethod::CloseOrderV3(v) => format!("/v3/pay/transactions/out-trade-no/{}/close", v),
            WxPayMethod::UnifiedOrderV3(v) => {
                match v {
                    TradeType::MWeb => String::from("/v3/pay/transactions/h5"),
                    TradeType::Jsapi => String::from("/v3/pay/transactions/jsapi"),
                    TradeType::Native => String::from("/v3/pay/transactions/native"),
                    TradeType::App => String::from("/v3/pay/transactions/app"),
                    _ => String::default()
                }
            }
            WxPayMethod::IsvUnifiedOrderV3(v) => {
                match v {
                    TradeType::MWeb => String::from("/v3/pay/partner/transactions/h5"),
                    TradeType::Jsapi => String::from("/v3/pay/partner/transactions/jsapi"),
                    TradeType::Native => String::from("/v3/pay/partner/transactions/native"),
                    TradeType::App => String::from("/v3/pay/partner/transactions/app"),
                    _ => String::default()
                }
            }
            WxPayMethod::QueryRefundOrder => String::from("/pay/refundquery"),
            WxPayMethod::QueryRefundOrderV2 => String::from("/pay/refundqueryv2"),
            WxPayMethod::QueryRefundOrderV3(v) => format!("/v3/refund/domestic/refunds/{}", v),
            WxPayMethod::ReverseOrder => String::from("/secapi/pay/reverse"),
        }
    }
}
