use serde_json::Value;
use crate::{DecryptNotifyResult, DecryptRefundNotifyResult, IsvWechatPayRequestV3, LabradorResult, LabraError, OriginNotifyResponse, RequestType, SessionStore, WechatCloseOrderRequest, WechatCloseOrderRequestV3, WechatCloseOrderResponse, WechatDecryptRefundNotifyResponse, WechatOrderReverseRequest, WechatOrderReverseResponse, WechatPayClient, WechatPayNotifyResponse, WechatPayNotifyResponseV3, WechatPayRequestV3, WechatPayResponse, WechatPayResponseV3, WechatQueryOrderRequest, WechatQueryOrderRequestV3, WechatQueryOrderResponse, WechatQueryOrderResponseV3, WechatQueryRefundOrderRequest, WechatQueryRefundResponse, WechatQueryRefundResponseV3, WechatRefundNotifyResponse, WechatRefundNotifyResponseV3, WechatRefundRequest, WechatRefundRequestV3, WechatRefundResponse, WechatRefundResponseV3, WxPayShorturlRequest, WxPayShortUrlResponse, WxScanPayNotifyResponse};
use crate::wechat::cryptos::{SignatureHeader, WechatCryptoV3};
use crate::wechat::pay::method::{WechatPayMethod, WxPayMethod};
use crate::wechat::pay::{TradeType};
use crate::wechat::pay::request::WechatPayRequest;

#[derive(Debug, Clone)]
pub struct WxPay<'a, T: SessionStore> {
    client: &'a WechatPayClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WxPay<'a, T> {

    #[inline]
    pub fn new(client: &WechatPayClient<T>) -> WxPay<T> {
        WxPay {
            client,
        }
    }

    ///
    /// # 统一下单
    /// <pre>
    /// 详见:[文档](https://pay.weixin.qq.com/wiki/doc/api/app/app.php?chapter=9_1)
    ///
    /// 在发起微信支付前，需要调用统一下单接口，获取"预支付交易会话标识"
    /// [接口地址](https://api.mch.weixin.qq.com/pay/unifiedorder)
    /// </pre>
    ///
    /// # 示例
    ///
    /// ```no_run
    ///
    /// # use labrador::SimpleStorage;
    /// # use labrador::WechatPayClient;
    /// # use labrador::WechatPayRequest;
    /// # use labrador::TradeType;
    /// # async fn main() {
    /// let client = WechatPayClient::new("appid","secret").wxpay();
    /// let param = WechatPayRequest {
    ///     appid: None,
    ///     trade_type: TradeType::Micro,
    ///     mch_id: "".to_string(),
    ///     openid: "".to_string(),
    ///     notify_url: None,
    ///     body: "".to_string(),
    ///     detail: "".to_string(),
    ///     attach: "".to_string(),
    ///     out_trade_no: "".to_string(),
    ///     total_fee: "".to_string(),
    ///     spbill_create_ip: "".to_string(),
    ///     sign: "".to_string(),
    ///     nonce_str: None,
    ///     device_info: "".to_string(),
    ///     product_id: "".to_string(),
    ///     auth_code: "".to_string()
    /// };
    /// match client.unified_order(param).await {
    ///     Ok(res) => {}
    ///     Err(err) => {}
    /// }
    /// # }
    ///
    /// ```
    ///
    pub async fn unified_order(&self, mut params: WechatPayRequest) -> LabradorResult<WechatPayResponse> {
        params.check_params()?;
        let method = if params.trade_type == TradeType::Micro { WxPayMethod::MicroPay } else { WxPayMethod::UnifiedOrder };
        params.appid = self.client.appid.to_owned().into();
        if params.trade_type == TradeType::Micro {
            // 將通知url置空
            params.notify_url = None;
        }
        params.get_sign(&self.client.secret);
        let res = self.client.post(WechatPayMethod::WxPay(method), &params.parse_xml(), RequestType::Xml).await?.text()?;
        WechatPayResponse::parse_xml(res)
    }

    ///
    /// # 统一下单 - V3版本
    /// <pre>
    /// 详见:[文档](https://pay.weixin.qq.com/wiki/doc/api/app/app.php?chapter=9_1)
    ///
    /// 在发起微信支付前，需要调用统一下单接口，获取"预支付交易会话标识"
    /// [接口地址](https://api.mch.weixin.qq.com/pay/unifiedorder)
    /// </pre>
    /// # 示例
    ///
    /// ```no_run
    ///
    /// # use labrador::SimpleStorage;
    /// # use labrador::WechatPayClient;
    /// # use labrador::WechatPayRequestV3;
    /// # use labrador::TradeType;
    /// # use labrador::Amount;
    /// # use labrador::Payer;
    /// # use chrono::NaiveDateTime;
    /// # async fn main() {
    /// let client = WechatPayClient::new("appid","secret").wxpay();
    /// let param = WechatPayRequestV3 {
    ///     appid: None,
    ///     mch_id: "".to_string(),
    ///     notify_url: "".to_string(),
    ///     amount: Amount { total: 0,currency: None,payer_total: None,payer_currency: None},
    ///     payer: Payer { openid: "".to_string()}.into(),
    ///     detail: None,
    ///     scene_info: None,attach: None,
    ///     out_trade_no: "".to_string(),
    ///     description: "".to_string(),
    ///     time_expire: "".to_string(),
    ///     settle_info: None
    /// };
    /// match client.unified_order_v3(TradeType::App, param).await {
    ///     Ok(res) => {}
    ///     Err(err) => {}
    /// }
    /// # }
    ///
    /// ```
    ///
    pub async fn unified_order_v3(&self, trade_type: TradeType, mut params: WechatPayRequestV3) -> LabradorResult<WechatPayResponseV3> {
        if params.mch_id.is_empty() {
            params.mch_id = self.client.mch_id.to_owned().unwrap_or_default();
        }
        if params.appid.is_none() {
            params.appid = self.client.appid.to_owned().into();
        }
        let res = self.client.post_v3(params.mch_id.to_owned().into(), WechatPayMethod::WxPay(WxPayMethod::UnifiedOrderV3(trade_type)), vec![],&params, RequestType::Json).await?.json::<serde_json::Value>()?;
        serde_json::from_value::<WechatPayResponseV3>(res).map_err(LabraError::from)
    }

    pub async fn isv_unified_order_v3(&self, trade_type: TradeType, mut params: IsvWechatPayRequestV3) -> LabradorResult<WechatPayResponseV3> {
        let res = self.client.post_v3(None, WechatPayMethod::WxPay(WxPayMethod::IsvUnifiedOrderV3(trade_type)), vec![],&params, RequestType::Json).await?.json::<serde_json::Value>()?;
        serde_json::from_value::<WechatPayResponseV3>(res).map_err(LabraError::from)
    }

    /// 调用统一下单接口，并组装生成支付所需参数对象.
    pub async fn create_order_v3(&self, trade_type: TradeType, params: WechatPayRequestV3) -> LabradorResult<Value> {
        let result = self.unified_order_v3(trade_type.to_owned(), params.to_owned()).await?;
        result.get_pay_info(trade_type, params.appid, params.mch_id, self.client.private_key.to_owned())
    }

    /// 服务商调用统一下单接口，并组装生成支付所需参数对象.
    pub async fn isv_create_order_v3(&self, trade_type: TradeType, params: IsvWechatPayRequestV3) -> LabradorResult<Value> {
        let result = self.isv_unified_order_v3(trade_type.to_owned(), params.to_owned()).await?;
        result.get_pay_info(trade_type, params.sub_appid.to_owned(), params.sub_mchid.to_owned().unwrap_or_default(), self.client.private_key.to_owned())
    }

    ///
    /// # 关闭订单
    /// <pre>
    /// 应用场景
    /// 以下情况需要调用关单接口：
    /// 1、商户订单支付失败需要生成新单号重新发起支付，要对原订单号调用关单，避免重复支付；
    /// 2、系统下单后，用户支付超时，系统退出不再受理，避免用户继续，请调用关单接口。
    /// 注意：关单没有时间限制，建议在订单生成后间隔几分钟（最短5分钟）再调用关单接口，避免出现订单状态同步不及时导致关单失败。
    ///
    /// [接口地址](https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter3_1_3.shtml)
    /// </pre>
    /// # 示例
    ///
    /// ```no_run
    ///
    /// # use labrador::SimpleStorage;
    /// # use labrador::WechatPayClient;
    /// # use labrador::WechatCloseOrderRequest;
    /// # async fn main() {
    /// let client = WechatPayClient::new("appid","secret").wxpay();
    /// let param = WechatCloseOrderRequest {
    ///     appid: None,
    ///     mch_id: "".to_string(),
    ///     out_trade_no: "".to_string(),
    ///     sign: "".to_string(),
    ///     nonce_str: None,
    /// };
    /// match client.close_order(param).await {
    ///     Ok(res) => {}
    ///     Err(err) => {}
    /// }
    /// # }
    ///
    /// ```
    ///
    pub async fn close_order(&self,
                             mut params: WechatCloseOrderRequest) -> LabradorResult<WechatCloseOrderResponse> {
        params.appid = self.client.appid.to_owned().into();
        params.get_sign(&self.client.api_key.to_owned().unwrap_or_default());
        let res = self.client.post(WechatPayMethod::WxPay(WxPayMethod::CloseOrder), &params.parse_xml(), RequestType::Xml).await?.text()?;
        WechatCloseOrderResponse::parse_xml(res)
    }

    ///
    /// # 关闭订单
    /// <pre>
    /// 应用场景
    /// 以下情况需要调用关单接口：
    /// 1、商户订单支付失败需要生成新单号重新发起支付，要对原订单号调用关单，避免重复支付；
    /// 2、系统下单后，用户支付超时，系统退出不再受理，避免用户继续，请调用关单接口。
    /// 注意：关单没有时间限制，建议在订单生成后间隔几分钟（最短5分钟）再调用关单接口，避免出现订单状态同步不及时导致关单失败。
    ///
    /// [接口地址](https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter3_1_3.shtml)
    /// </pre>
    /// # 示例
    ///
    /// ```no_run
    ///
    /// # use labrador::SimpleStorage;
    /// # use labrador::WechatPayClient;
    /// # use labrador::WechatCloseOrderRequestV3;
    /// # async fn main() {
    /// let client = WechatPayClient::new("appid","secret").wxpay();
    /// let param = WechatCloseOrderRequestV3 {
    ///     mchid: "".to_string(),
    ///     out_trade_no: None,
    /// };
    /// match client.close_order_v3(param).await {
    ///     Ok(res) => {}
    ///     Err(err) => {}
    /// }
    /// # }
    ///
    /// ```
    ///
    pub async fn close_order_v3(&self, mut params: WechatCloseOrderRequestV3) -> LabradorResult<()> {
        let out_trade_no = params.out_trade_no.to_owned().unwrap_or_default();
        params.out_trade_no = None;
        let res = self.client.post_v3(params.mchid.to_owned().into(), WechatPayMethod::WxPay(WxPayMethod::CloseOrderV3(out_trade_no)), vec![], &params, RequestType::Json).await?;
        let _ = res.text()?;
        // let s = res.json::<serde_json::Value>().await?;
        Ok(())
    }

    ///
    /// # 查询订单（适合于需要自定义子商户号和子商户appid的情形）.
    /// 详见:[文档](https://pay.weixin.qq.com/wiki/doc/api/jsapi.php?chapter=9_2)
    /// <pre>
    /// 该接口提供所有微信支付订单的查询，商户可以通过查询订单接口主动查询订单状态，完成下一步的业务逻辑。
    /// 需要调用查询接口的情况：
    ///    ◆ 当商户后台、网络、服务器等出现异常，商户系统最终未接收到支付通知；
    ///    ◆ 调用支付接口后，返回系统错误或未知交易状态情况；
    ///    ◆ 调用被扫支付API，返回USERPAYING的状态；
    ///    ◆ 调用关单或撤销接口API之前，需确认支付状态；
    ///
    /// 接口地址：
    /// https://api.mch.weixin.qq.com/pay/orderquery
    /// </pre>
    /// # 示例
    ///
    /// ```no_run
    ///
    /// # use labrador::SimpleStorage;
    /// # use labrador::WechatPayClient;
    /// # use labrador::WechatQueryOrderRequest;
    /// # async fn main() {
    /// let client = WechatPayClient::new("appid","secret").wxpay();
    /// let param = WechatQueryOrderRequest {
    ///     transaction_id: None,
    ///     out_trade_no: None,
    ///     appid: None,
    ///     mch_id: "".to_string(),
    ///     sign: "".to_string(),
    ///     nonce_str: None
    /// };
    /// match client.query_order(param).await {
    ///     Ok(res) => {}
    ///     Err(err) => {}
    /// }
    /// # }
    ///
    /// ```
    ///
    pub async fn query_order(&self, mut params: WechatQueryOrderRequest) -> LabradorResult<WechatQueryOrderResponse> {
        let res = self.client.post(WechatPayMethod::WxPay(WxPayMethod::QueryOrder), &params, RequestType::Xml).await?;
        let result = res.text()?;
        WechatQueryOrderResponse::parse_xml(result)
    }

    ///
    /// # 查询订单
    /// 详见:[文档](https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter3_1_2.shtml)
    /// <pre>
    /// 商户可以通过查询订单接口主动查询订单状态，完成下一步的业务逻辑。查询订单状态可通过微信支付订单号或商户订单号两种方式查询
    /// 注意：
    /// 查询订单可通过微信支付订单号和商户订单号两种方式查询，两种查询方式返回结果相同
    /// 需要调用查询接口的情况：
    ///   ◆ 当商户后台、网络、服务器等出现异常，商户系统最终未接收到支付通知。
    ///   ◆ 调用支付接口后，返回系统错误或未知交易状态情况。
    ///   ◆ 调用付款码支付API，返回USERPAYING的状态。
    ///   ◆ 调用关单或撤销接口API之前，需确认支付状态。
    ///
    /// 接口地址：
    /// https://api.mch.weixin.qq.com/v3/pay/transactions/id/{transaction_id}
    /// https://api.mch.weixin.qq.com/v3/pay/transactions/out-trade-no/{out_trade_no}
    /// </pre>
    /// # 示例
    ///
    /// ```no_run
    ///
    /// # use labrador::SimpleStorage;
    /// # use labrador::WechatPayClient;
    /// # use labrador::WechatQueryOrderRequestV3;
    /// # async fn main() {
    /// let client = WechatPayClient::new("appid","secret").wxpay();
    /// let param = WechatQueryOrderRequestV3 {
    ///     mchid: "".to_string(),
    ///     transaction_id: None,
    ///     out_trade_no: None,
    /// };
    /// match client.query_order_v3(param).await {
    ///     Ok(res) => {}
    ///     Err(err) => {}
    /// }
    /// # }
    ///
    /// ```
    ///
    pub async fn query_order_v3(&self, params: WechatQueryOrderRequestV3) -> LabradorResult<WechatQueryOrderResponseV3> {
        self.client.post_v3(params.mchid.to_owned().into(), WechatPayMethod::WxPay(WxPayMethod::QueryOrderV3((params.out_trade_no.to_owned(), params.out_trade_no.to_owned()))), vec![], "", RequestType::Json)
            .await?.json::<WechatQueryOrderResponseV3>()
    }




    ///
    ///
    /// # 微信支付-查询退款（适合于需要自定义子商户号和子商户appid的情形）.
    /// 详见 [文档](https://pay.weixin.qq.com/wiki/doc/api/jsapi.php?chapter=9_5)
    /// <pre>
    /// 应用场景：
    ///  提交退款申请后，通过调用该接口查询退款状态。退款有一定延时，用零钱支付的退款20分钟内到账，
    ///  银行卡支付的退款3个工作日后重新查询退款状态。
    ///
    /// 接口链接：https://api.mch.weixin.qq.com/pay/refundquery
    /// </pre>
    pub async fn query_refund_order(&self, mut params: WechatQueryRefundOrderRequest) -> LabradorResult<WechatQueryRefundResponse> {
        params.appid = self.client.appid.to_owned().into();
        params.get_sign(&self.client.api_key.to_owned().unwrap_or_default());
        let res = self.client.post(WechatPayMethod::WxPay(WxPayMethod::QueryRefundOrder), params, RequestType::Xml)
            .await?.text()?;
        WechatQueryRefundResponse::parse_xml(res)
    }


    ///
    ///
    /// # 微信支付-查询退款API（支持单品）.
    /// 详见 [文档](https://pay.weixin.qq.com/wiki/doc/api/danpin.php?chapter=9_104&index=4)
    /// <pre>
    /// 应用场景
    ///    提交退款申请后，通过调用该接口查询退款状态。退款有一定延时，用零钱支付的退款20分钟内到账，银行卡支付的退款3个工作日后重新查询退款状态。
    /// 注意：
    /// 1、本接口支持查询单品优惠相关退款信息，且仅支持按微信退款单号或商户退款单号查询，若继续调用老查询退款接口，
    ///    请见https://pay.weixin.qq.com/wiki/doc/api/jsapi_sl.php?chapter=9_5
    /// 2、请求频率限制：300qps，即每秒钟正常的退款查询请求次数不超过300次
    /// 3、错误或无效请求频率限制：6qps，即每秒钟异常或错误的退款查询请求不超过6次
    ///
    /// 接口地址
    /// https://api.mch.weixin.qq.com/pay/refundqueryv2
    /// https://api2.mch.weixin.qq.com/pay/refundqueryv2(备用域名)见跨城冗灾方案
    ///
    /// </pre>
    pub async fn query_refund_order_v2(&self, params: WechatQueryRefundOrderRequest) -> LabradorResult<WechatQueryRefundResponse> {
        let res = self.client.post(WechatPayMethod::WxPay(WxPayMethod::QueryRefundOrderV2), params, RequestType::Json)
            .await?.text()?;
        WechatQueryRefundResponse::parse_xml(res)
    }

    /// 
    /// # 微信支付-查询退款
    /// 详见 [文档](https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter3_1_10.shtml)
    /// <pre>
    /// 
    /// 应用场景：
    ///  提交退款申请后，通过调用该接口查询退款状态。退款有一定延时，建议在提交退款申请后1分钟发起查询退款状态，一般来说零钱支付的退款5分钟内到账，银行卡支付的退款1-3个工作日到账。
    ///
    /// 接口链接：https://api.mch.weixin.qq.com/v3/refund/domestic/refunds/{out_refund_no}
    /// </pre>
    pub async fn query_refund_order_v3(&self, out_refund_no: String) -> LabradorResult<WechatQueryRefundResponseV3> {
        self.client.post_v3(None, WechatPayMethod::WxPay(WxPayMethod::QueryRefundOrderV3(out_refund_no)), vec![], "", RequestType::Json)
            .await?.json::<WechatQueryRefundResponseV3>()
    }

    pub async fn isv_query_refund_order_v3(&self, out_refund_no: String, sub_mch_id: String) -> LabradorResult<WechatQueryRefundResponseV3> {
        self.client.post_v3(None, WechatPayMethod::WxPay(WxPayMethod::QueryRefundOrderV3(out_refund_no)), vec![("sub_mchid".to_string(), sub_mch_id)], "", RequestType::Json)
            .await?.json::<WechatQueryRefundResponseV3>()
    }

    /// # 解析支付结果通知.
    /// 详见 [文档](https://pay.weixin.qq.com/wiki/doc/api/jsapi.php?chapter=9_7)
    pub fn parse_order_notify(&self, xml: &str) -> LabradorResult<WechatPayNotifyResponse> {
        WechatPayNotifyResponse::parse_xml(xml.to_string())
    }

    /// # 解析支付结果通知. - v3
    /// 详见 [文档](https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter3_1_5.shtml)
    pub async fn parse_order_notify_v3(&self, notify_data: &str, header: Option<SignatureHeader>) -> LabradorResult<WechatPayNotifyResponseV3> {

        if header.is_none() {
            return Err(LabraError::RequestError("非法请求，头部信息为空".to_string()));
        }
        let header = header.unwrap();
        if !self.client.verify_notify_sign(&header, notify_data).await {
            return Err(LabraError::RequestError("非法请求，头部信息验证失败".to_string()));
        }
        let origin = serde_json::from_str::<OriginNotifyResponse>(notify_data)?;
        let resource = origin.resource.to_owned();
        let v3_key = self.client.api_key_v3.to_owned().unwrap_or_default();
        let crypto = WechatCryptoV3::new(&v3_key);
        let decrypted = crypto.decrypt_data_v3(&resource)?;
        let decrypt_notify_result = serde_json::from_slice::<DecryptNotifyResult>(&decrypted)?;
        Ok(WechatPayNotifyResponseV3 {
            raw_data: origin.into(),
            result: decrypt_notify_result.into()
        })
    }

    /// # 解析退款结果通知.
    /// 详见 [文档](https://pay.weixin.qq.com/wiki/doc/api/jsapi.php?chapter=9_16&index=9)
    pub fn parse_refund_notify(&self, xml: &str) -> LabradorResult<WechatDecryptRefundNotifyResponse> {
        WechatRefundNotifyResponse::parse_xml(xml.to_string(), &self.client.appid)
    }

    /// # 解析退款结果通知 - V3.
    /// 详见 [文档](https://pay.weixin.qq.com/wiki/doc/api/jsapi.php?chapter=9_16&index=9)
    pub async fn parse_refund_notify_v3(&self, notify_data: &str, header: &Option<SignatureHeader>) -> LabradorResult<WechatRefundNotifyResponseV3> {
        if header.is_none() {
            return Err(LabraError::RequestError("非法请求，头部信息验证为空".to_string()));
        }
        let header = header.to_owned().unwrap();
        if !self.client.verify_notify_sign(&header, notify_data).await {
            return Err(LabraError::RequestError("非法请求，头部信息验证失败".to_string()));
        }
        let origin = serde_json::from_str::<OriginNotifyResponse>(notify_data)?;
        let resource = origin.resource.to_owned();
        let v3_key = self.client.api_key_v3.to_owned().unwrap_or_default();
        let crypto = WechatCryptoV3::new(&v3_key);
        let decrypted = crypto.decrypt_data_v3(&resource)?;
        let decrypt_notify_result = serde_json::from_slice::<DecryptRefundNotifyResult>(&decrypted)?;
        Ok(WechatRefundNotifyResponseV3 {
            raw_data: origin.into(),
            result: decrypt_notify_result.into()
        })
    }

    /// # 解析扫码支付回调通知
    /// 详见 [文档](https://pay.weixin.qq.com/wiki/doc/api/native.php?chapter=6_4)
    pub fn parse_scan_pay_notify(&self, xml: &str) -> LabradorResult<WxScanPayNotifyResponse> {
        WxScanPayNotifyResponse::parse_xml(xml.to_string())
    }


    /// 支付
    pub async fn micro_pay(
        &self,
        mut pay_params: WechatPayRequest
    ) -> LabradorResult<WechatPayResponse> {
        pay_params.trade_type = TradeType::Micro;
        self.unified_order(pay_params).await
    }

    /// JSAPI支付
    pub async fn jsapi_pay(
        &self,
        mut pay_params: WechatPayRequest
    ) -> LabradorResult<WechatPayResponse> {
        pay_params.trade_type = TradeType::Jsapi;
        self.unified_order(pay_params).await
    }

    /// APP支付
    pub async fn app_pay(
        &self,
        mut pay_params: WechatPayRequest
    ) -> LabradorResult<WechatPayResponse> {
        pay_params.trade_type = TradeType::App;
        self.unified_order(pay_params).await
    }

    ///
    ///
    /// # 申请退款API（支持单品）.
    /// 详见 [文档](https://pay.weixin.qq.com/wiki/doc/api/danpin.php?chapter=9_103&index=3)
    /// <pre>
    /// 应用场景
    /// 当交易发生之后一段时间内，由于买家或者卖家的原因需要退款时，卖家可以通过退款接口将支付款退还给买家，微信支付将在收到退款请求并且验证成功之后，按照退款规则将支付款按原路退到买家帐号上。
    ///
    /// 注意：
    /// 1、交易时间超过一年的订单无法提交退款；
    /// 2、微信支付退款支持单笔交易分多次退款，多次退款需要提交原支付订单的商户订单号和设置不同的退款单号。申请退款总金额不能超过订单金额。 一笔退款失败后重新提交，请不要更换退款单号，请使用原商户退款单号。
    /// 3、请求频率限制：150qps，即每秒钟正常的申请退款请求次数不超过150次
    ///     错误或无效请求频率限制：6qps，即每秒钟异常或错误的退款申请请求不超过6次
    /// 4、每个支付订单的部分退款次数不能超过50次
    /// 5、本接口支持单品优惠订单全额退款和单品优惠订单部分退款，推荐使用本接口，如果使用不支持单品优惠部分退款的历史接口，请看https://pay.weixin.qq.com/wiki/doc/api/jsapi_sl.php?chapter=9_4
    ///
    /// 接口地址
    /// https://api.mch.weixin.qq.com/secapi/pay/refundv2
    /// https://api2.mch.weixin.qq.com/secapi/pay/refundv2(备用域名)见跨城冗灾方案
    /// </pre>
    ///
    pub async fn refund(
        &self,
        mut params: WechatRefundRequest
    ) -> LabradorResult<WechatRefundResponse> {
        params.appid = self.client.appid.to_owned().into();
        let mch_id = params.mch_id.as_str();
        params.get_sign(&self.client.api_key.to_owned().unwrap_or_default());
        let res = self.client.post(WechatPayMethod::WxPay(WxPayMethod::Refund), &params.parse_xml(), RequestType::Xml).await?.text()?;
        WechatRefundResponse::parse_xml(res)
    }

    ///
    ///
    /// # 撤销订单API.
    /// 详见 [文档](https://pay.weixin.qq.com/wiki/doc/api/micropay.php?chapter=9_11&index=3)
    /// <pre>
    /// 应用场景：
    ///  支付交易返回失败或支付系统超时，调用该接口撤销交易。如果此订单用户支付失败，微信支付系统会将此订单关闭；
    ///  如果用户支付成功，微信支付系统会将此订单资金退还给用户。
    ///  注意：7天以内的交易单可调用撤销，其他正常支付的单如需实现相同功能请调用申请退款API。
    ///  提交支付交易后调用【查询订单API】，没有明确的支付结果再调用【撤销订单API】。
    ///  调用支付接口后请勿立即调用撤销订单API，建议支付后至少15s后再调用撤销订单接口。
    ///  接口链接 ：https://api.mch.weixin.qq.com/secapi/pay/reverse
    ///  是否需要证书：请求需要双向证书。
    /// </pre>
    ///
    pub async fn reverse_order(
        &self,
        mut params: WechatOrderReverseRequest
    ) -> LabradorResult<WechatOrderReverseResponse> {
        params.appid = self.client.appid.to_owned().into();
        let mch_id = params.mch_id.as_str();
        let res = self.client.post(WechatPayMethod::WxPay(WxPayMethod::ReverseOrder), &params.parse_xml(), RequestType::Xml).await?.text()?;
        WechatOrderReverseResponse::parse_xml(res)
    }

    ///
    ///
    /// # 转换短链接.
    /// 详见 [文档](https://pay.weixin.qq.com/wiki/doc/api/micropay.php?chapter=9_9&index=8)
    /// <pre>
    ///  应用场景：
    ///     该接口主要用于扫码原生支付模式一中的二维码链接转成短链接(weixin://wxpay/s/XXXXXX)，减小二维码数据量，提升扫描速度和精确度。
    ///  接口地址：<a href="https://api.mch.weixin.qq.com/tools/shorturl">https://api.mch.weixin.qq.com/tools/shorturl</a>
    ///  是否需要证书：否
    /// </pre>
    ///
    pub async fn short_url(
        &self,
        mut params: WxPayShorturlRequest
    ) -> LabradorResult<WxPayShortUrlResponse> {
        params.appid = self.client.appid.to_owned().into();
        let mch_id = params.mch_id.as_str();
        let res = self.client.post(WechatPayMethod::WxPay(WxPayMethod::ShortUrl), &params.parse_xml(), RequestType::Xml).await?.text()?;
        WxPayShortUrlResponse::parse_xml(res)
    }

    ///
    ///
    /// # 申请退款API（支持单品）.
    /// 详见 [文档](https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter3_1_9.shtml)
    /// <pre>
    /// 应用场景
    /// 当交易发生之后一年内，由于买家或者卖家的原因需要退款时，卖家可以通过退款接口将支付金额退还给买家，微信支付将在收到退款请求并且验证成功之后，将支付款按原路退还至买家账号上。
    ///
    /// 注意：
    /// 1、交易时间超过一年的订单无法提交退款
    /// 2、微信支付退款支持单笔交易分多次退款（不超50次），多次退款需要提交原支付订单的商户订单号和设置不同的退款单号。申请退款总金额不能超过订单金额。 一笔退款失败后重新提交，请不要更换退款单号，请使用原商户退款单号
    /// 3、错误或无效请求频率限制：6qps，即每秒钟异常或错误的退款申请请求不超过6次
    /// 4、每个支付订单的部分退款次数不能超过50次
    /// 5、如果同一个用户有多笔退款，建议分不同批次进行退款，避免并发退款导致退款失败
    /// 6、申请退款接口的返回仅代表业务的受理情况，具体退款是否成功，需要通过退款查询接口获取结果
    /// 7、一个月之前的订单申请退款频率限制为：5000/min
    ///
    /// 接口地址
    /// https://api.mch.weixin.qq.com/v3/refund/domestic/refunds
    /// </pre>
    ///
    pub async fn refund_v3(
        &self,
        mut params: WechatRefundRequestV3
    ) -> LabradorResult<WechatRefundResponseV3> {
       self.client.post_v3(None, WechatPayMethod::WxPay(WxPayMethod::RefundV3), vec![],params, RequestType::Json).await?
            .json::<WechatRefundResponseV3>()
    }
}



#[cfg(test)]
#[allow(unused, non_snake_case)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use std::ops::Add;
    use chrono::{DateTime, Local, NaiveDateTime, SecondsFormat};
    use crate::{Amount, Payer, request, SimpleStorage, TradeType, WechatCloseOrderRequestV3, WechatPayClient, WechatPayRequestV3};

    #[test]
    fn test_close_order_v3() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let mut private_key = Vec::new();
        let s = File::open("src/wechat/pay/sec/apiclient_key.pem");
        if s.is_err() {
            return;
        }
        s.unwrap().read_to_end(&mut private_key);
        let r = rt.spawn(async {
            let c =  WechatPayClient::<SimpleStorage>::new("appid", "secret");
            let mut client =c.wxpay();
            let result = client.close_order_v3(WechatCloseOrderRequestV3 {
                mchid: "mchid".to_string(),
                out_trade_no: "23234234234".to_string().into()
            });
            match result.await {
                Ok(res) => {
                    println!("请求成功",);
                }
                Err(err) => {
                    println!("err:{:?}", err);
                }
            }
        });
        rt.block_on(r);
    }



    #[test]
    fn test_callback_v3() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let mut private_key = Vec::new();
        let s = File::open("src/wechat/pay/sec/apiclient_key.pem");
        if s.is_err() {
            return;
        }
        s.unwrap().read_to_end(&mut private_key);
        let r = rt.spawn(async {
            let c =  WechatPayClient::<SimpleStorage>::new("appid", "secret");
            let mut client =c.wxpay();
            // .cert(MchCert {
            //     mch_id: "1602920235".to_string().into(),
            //     serial_no: "71785E680339B8B4B056BD61A41A1AD2020AE33E".to_string().into(),
            //     private_key_path: String::from("src/wechat/pay/sec/apiclient_key.pem").into(),
            //     private_key: String::from_utf8(private_key).unwrap().into(),
            //     private_cert_path:  String::from("src/wechat/pay/sec/apiclient_cert.pem").into(),
            //     pkcs12_path: None
            // }).key_v3("364ae33e57cf4989b8aefaa66ddc7ca7".to_string())
            let date = Local::now().to_rfc3339_opts(SecondsFormat::Secs, false);
            /*let result = client.unified_order_v3(TradeType::Jsapi, WechatPayRequestV3 {
                appid: "wx7959501b424a9e93".to_string().into(),
                mch_id: "1602920235".to_string(),
                description: "测试商品支付".to_string(),
                out_trade_no: "1602920235sdfsdfas32234234".to_string(),
                time_expire: date,
                attach: None,
                notify_url: "https://api.snackcloud.cn/trade/notify".to_string(),
                amount: Amount {
                    total: 1,
                    currency: String::from("CNY").into()
                },
                payer: Payer {
                    openid: "oUVZc6S_uGx3bsNPUA-davo4Dt7U".to_string()
                },
                detail: None,
                scene_info: None,
                settle_info: None
            });*/
            let result = client.close_order_v3(WechatCloseOrderRequestV3 {
                mchid: "mchid".to_string(),
                out_trade_no: "23234234234".to_string().into()
            });
            match result.await {
                Ok(res) => {
                    println!("请求成功",);
                }
                Err(err) => {
                    println!("err:{:?}", err);
                }
            }
        });
        rt.block_on(r);
    }

    #[test]
    fn test_create_order_v3() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let mut private_key = Vec::new();
        let s = File::open("src/wechat/pay/sec/apiclient_key.pem");
        if s.is_err() {
            return;
        }
        s.unwrap().read_to_end(&mut private_key);
        let r = rt.spawn(async {
            let c =  WechatPayClient::<SimpleStorage>::new("appid", "secret");
            let mut client =c.wxpay();
            let date = Local::now().to_rfc3339_opts(SecondsFormat::Secs, false);
            let result = client.unified_order_v3(TradeType::Jsapi, WechatPayRequestV3 {
                appid: "appid".to_string().into(),
                mch_id: "mchid".to_string(),
                description: "测试商品支付".to_string(),
                out_trade_no: "1602920235sdfsdfas32234234".to_string(),
                time_expire: date,
                attach: None,
                notify_url: "https://xxx.cn/trade/notify".to_string(),
                amount: Amount {
                    total: 1,
                    currency: String::from("CNY").into(),
                    payer_total: None,
                    payer_currency: None
                },
                payer: Payer {
                    openid: "oUVZc6S_uGx3bsNPUA-davo4Dt7Us".to_string()
                }.into(),
                detail: None,
                scene_info: None,
                settle_info: None
            });
            match result.await {
                Ok(res) => {
                    println!("请求成功",);
                }
                Err(err) => {
                    println!("err:{:?}", err);
                }
            }
        });
        rt.block_on(r);
    }

}