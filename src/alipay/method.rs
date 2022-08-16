use crate::RequestMethod;

#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum AlipayMethod {
    /// 手机网站支付
    WapPay,
    /// PC场景下单并支付
    PCPay,
    /// PC场景下单并支付
    AppPay,
    /// 统一收单交易支付接口
    UnifiedPay,
    /// 统一收单线下交易预创建
    PreUnifiedOrder,
    /// 统一收单交易创建接口
    CreateUnifiedOrder,
    /// 统一收单线下交易查询
    QueryOrder,
    /// 统一收单交易关闭接口
    CloseOrder,
    /// 统一收单交易退款接口
    Refund,
    /// 统一收单交易退款查询
    QueryRefund,
    /// 统一收单交易撤销接口
    CancelOrder,
    /// 换取授权访问令牌
    SystemOauthToken,
    /// 换取应用授权令牌
    OpenAuthTokenApp,
    /// 自定义方法
    Custom { method: String, response_key: String }
}

#[allow(unused)]
impl RequestMethod for AlipayMethod {
    fn get_method(&self) -> String {
        match *self {
            AlipayMethod::WapPay => String::from("alipay.trade.wap.pay"),
            AlipayMethod::PCPay => String::from("alipay.trade.page.pay"),
            AlipayMethod::AppPay => String::from("alipay.trade.app.pay"),
            AlipayMethod::UnifiedPay => String::from("alipay.trade.pay"),
            AlipayMethod::PreUnifiedOrder => String::from("alipay.trade.precreate"),
            AlipayMethod::CreateUnifiedOrder => String::from("alipay.trade.create"),
            AlipayMethod::QueryOrder => String::from("alipay.trade.query"),
            AlipayMethod::CloseOrder => String::from("alipay.trade.close"),
            AlipayMethod::Refund => String::from("alipay.trade.refund"),
            AlipayMethod::QueryRefund => String::from("alipay.trade.fastpay.refund.query"),
            AlipayMethod::CancelOrder => String::from("alipay.trade.cancel"),
            AlipayMethod::SystemOauthToken => String::from("alipay.system.oauth.token"),
            AlipayMethod::OpenAuthTokenApp => String::from("alipay.open.auth.token.app"),
            AlipayMethod::Custom{ ref method, .. } => method.to_string()
        }
    }

    fn get_response_key(&self) -> String {
        match *self {
            AlipayMethod::WapPay => String::from("alipay_trade_wap_pay_response"),
            AlipayMethod::PCPay => String::from("alipay_trade_page_pay_response"),
            AlipayMethod::AppPay => String::from("alipay_trade_app_pay_response"),
            AlipayMethod::UnifiedPay => String::from("alipay_trade_pay_response"),
            AlipayMethod::PreUnifiedOrder => String::from("alipay_trade_precreate_response"),
            AlipayMethod::CreateUnifiedOrder => String::from("alipay_trade_create_response"),
            AlipayMethod::QueryOrder => String::from("alipay_trade_query_response"),
            AlipayMethod::CloseOrder => String::from("alipay_trade_close_response"),
            AlipayMethod::Refund => String::from("alipay_trade_refund_response"),
            AlipayMethod::QueryRefund => String::from("alipay_trade_fastpay_refund_query_response"),
            AlipayMethod::CancelOrder => String::from("alipay_trade_cancel_response"),
            AlipayMethod::SystemOauthToken => String::from("alipay_system_oauth_token_response"),
            AlipayMethod::OpenAuthTokenApp => String::from("alipay_open_auth_token_app_response"),
            AlipayMethod::Custom{ ref response_key, .. } => response_key.to_string()
        }
    }
}
