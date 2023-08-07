use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use crate::{LabradorResult, LabraError};

use crate::util::get_sign;
use crate::wechat::pay::TradeType;

//----------------------------------------------------------------------------------------------------------------------------

// 微信支付 ↓


#[derive(Debug, Serialize, Deserialize)]
pub struct WechatPayRequest {
    pub appid: Option<String>,
    /// 交易类型
    pub trade_type: TradeType,
    /// 商户号
    pub mch_id: String,
    /// 用户号
    pub openid: String,
    /// 通知地址
    pub notify_url: Option<String>,
    /// 签名类型
    // pub sign_type: String,
    /// 商品描述
    pub body: String,
    /// 商品详情
    pub detail: String,
    /// 附加数据
    pub attach: String,
    /// 商户订单号
    pub out_trade_no: String,
    /// 标价金额
    pub total_fee: String,
    /// 终端IP
    pub spbill_create_ip: String,
    /// 签名
    pub sign: String,
    /// 加密字符串
    pub nonce_str: Option<String>,
    /// 设备信息
    pub device_info: String,
    /// 商品ID
    pub product_id: String,
    /// 无
    pub auth_code: String,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WechatPayRequestV3 {
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
    pub time_expire: String,
    /// 附加数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
    /// 通知地址
    pub notify_url: String,
    /// 订单金额
    pub amount: Amount,
    /// 支付者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer: Option<Payer>,
    /// 优惠功能
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<Discount>,
    /// 场景信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_info: Option<SceneInfo>,
    /// 结算信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_info: Option<SettleInfo>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IsvWechatPayRequestV3 {
    /// 由微信生成的应用ID，全局唯一。请求基础下单接口时请注意APPID的应用属性，例如公众号场景下，需使用应用属性为公众号的服务号APPID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp_appid: Option<String>,
    /// 服务商户号，由微信支付生成并下发
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp_mchid: Option<String>,
    /// 子商户申请的应用ID，全局唯一。请求基础下单接口时请注意APPID的应用属性，例如公众号场景下，需使用应用属性为公众号的APPID
    /// 若sub_openid有传的情况下，sub_appid必填，且sub_appid需与sub_openid对应
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_appid: Option<String>,
    /// 子商户的商户号，由微信支付生成并下发。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_mchid: Option<String>,
    /// 商品描述
    pub description: String,
    /// 商户订单号
    pub out_trade_no: String,
    /// 交易结束时间
    pub time_expire: String,
    /// 附加数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
    /// 通知地址
    pub notify_url: String,
    /// 订单金额
    pub amount: Amount,
    /// 支付者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer: Option<Payer>,
    /// 优惠功能
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<Discount>,
    /// 场景信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_info: Option<SceneInfo>,
    /// 结算信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_info: Option<SettleInfo>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SettleInfo {
    /// 是否指定分账
    pub profit_sharing: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SceneInfo {
    /// 用户终端IP 用户的客户端IP，支持IPv4和IPv6两种格式的IP地址。 示例值：14.23.150.211
    pub payer_client_ip: Option<String>,
    /// 商户端设备号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
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
    /// 门店编号
    pub id: String,
    /// 详细地址
    pub address: String,
    /// 门店名称
    pub name: Option<String>,
    /// 地区编码
    pub area_code: Option<String>,
}


#[derive(Default,Debug, Serialize, Deserialize, Clone)]
pub struct Payer {
    /// 用户号,用户在直连商户appid下的唯一标识。
    pub openid: String,
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Discount {
    /// 订单原价
    /// 1、商户侧一张小票订单可能被分多次支付，订单原价用于记录整张小票的交易金额。
    /// 2、当订单原价与支付金额不相等，则不享受优惠。
    /// 3、该字段主要用于防止同一张小票分多次支付，以享受多次优惠的情况，正常支付订单不必上传此参数。
    /// 示例值：608800
    pub cost_price: Option<i32>,
    /// 商品小票ID
    pub invoice_id: Option<i32>,
    /// 单品列表
    pub goods_detail: Option<Vec<GoodsDetail>>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoodsDetail {
    /// 商户侧商品编码
    pub merchant_goods_id: String,
    /// 微信侧商品编码
    pub wechatpay_goods_id: Option<String>,
    /// 商品名称
    pub goods_name: Option<String>,
    /// 商品数量
    pub quantity: i32,
    /// 商品单价
    pub unit_price: i32,
    /// 商品退款金额
    pub refund_amount: Option<i32>,
    /// 商品退货数量
    pub refund_quantity: Option<i32>,
}


#[allow(unused)]
impl WechatPayRequest {
    pub fn parse_xml(&self) -> String {
        let msg = format!(
            "<xml>\n\
                <appid>{appid}</appid>\n\
                <attach>{attach}</attach>\n\
                <body>{body}</body>\n\
                <mch_id>{mch_id}</mch_id>\n\
                <detail><![CDATA[{detail}]]></detail>\n\
                <nonce_str>{nonce_str}</nonce_str>\n\
                <notify_url>{notify_url}</notify_url>\n\
                <openid>{openid}</openid>\n\
                <out_trade_no>{out_trade_no}</out_trade_no>\n\
                <spbill_create_ip>{spbill_create_ip}</spbill_create_ip>\n\
                <total_fee>{total_fee}</total_fee>\n\
                <trade_type>{trade_type}</trade_type>\n\
                <sign>{sign}</sign>\n\
            </xml>",
            appid=self.appid.to_owned().unwrap_or_default(),
            attach=self.attach,
            body=self.body,
            detail= self.detail,
            mch_id=self.mch_id,
            nonce_str=self.nonce_str.to_owned().unwrap_or_default(),
            notify_url=self.notify_url.to_owned().unwrap_or_default(),
            openid=self.openid,
            out_trade_no=self.out_trade_no,
            spbill_create_ip=self.spbill_create_ip,
            total_fee=self.total_fee,
            trade_type=self.trade_type.get_trade_type(),
            sign=self.sign,
        );
        msg
    }

    pub fn get_sign(&mut self, appkey: &str) {
        let mut pairs = BTreeMap::new();
        if let Some(appid) = self.appid.to_owned() {
            pairs.insert("appid".to_string(), appid);
        }
        pairs.insert("attach".to_string(), self.attach.to_owned());
        pairs.insert("auth_code".to_string(), self.auth_code.to_owned());
        pairs.insert("body".to_string(), self.body.to_owned());
        pairs.insert("total_fee".to_string(), self.total_fee.to_owned());
        pairs.insert("detail".to_string(), self.detail.to_owned());
        pairs.insert("device_info".to_string(), self.device_info.to_owned());
        pairs.insert("mch_id".to_string(), self.mch_id.to_owned());
        pairs.insert("openid".to_string(), self.openid.to_owned());
        pairs.insert("out_trade_no".to_string(), self.out_trade_no.to_owned());
        pairs.insert("product_id".to_string(), self.product_id.to_owned());
        pairs.insert("spbill_create_ip".to_string(), self.spbill_create_ip.to_owned());
        pairs.insert("trade_type".to_string(), self.trade_type.get_trade_type().to_string());
        if let Some(nonce_str) = self.nonce_str.to_owned() {
            pairs.insert("nonce_str".to_string(), nonce_str);
        }
        if let Some(notify_url) = self.notify_url.to_owned() {
            pairs.insert("notify_url".to_string(), notify_url);
        }
        self.sign = get_sign(&pairs, appkey);
    }

    pub(crate) fn check_params(&self) -> LabradorResult<()> {
        if self.sign.is_empty() || self.body.is_empty() || self.out_trade_no.is_empty() || self.total_fee.is_empty() || self.spbill_create_ip.is_empty() {
            return Err(LabraError::MissingField("参数不能为空".to_string()));
        }
        match self.trade_type {
            TradeType::Native => {
                if self.product_id.is_empty() {
                    return Err(LabraError::MissingField("商品ID不能为空".to_string()));
                }
            }
            TradeType::Jsapi => {
                if self.openid.is_empty() {
                    return Err(LabraError::MissingField("openid不能为空".to_string()));
                }
            }
            TradeType::Micro => {
                if self.auth_code.is_empty() {
                    return Err(LabraError::MissingField("openid不能为空".to_string()));
                }
            }
            _ => {}
        }
        Ok(())
    }
}



#[derive(Debug, Serialize, Deserialize)]
pub struct WechatCloseOrderRequest {
    pub appid: Option<String>,
    /// 商户号
    pub mch_id: String,
    /// 商户订单号
    pub out_trade_no: String,
    /// 签名
    pub sign: String,
    /// 加密字符串
    pub nonce_str: Option<String>,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct WechatCloseOrderRequestV3 {
    /// 商户号
    pub mchid: String,
    /// 商户订单号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_trade_no: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WechatQueryOrderRequest {
    /// 微信支付订单号 二选一 微信的订单号，优先使用
    pub transaction_id: Option<String>,
    /// 商户订单号。商户系统内部的订单号，当没提供transaction_id时需要传这个。
    pub out_trade_no: Option<String>,
    pub appid: Option<String>,
    /// 商户号
    pub mch_id: String,
    /// 签名
    pub sign: String,
    /// 加密字符串
    pub nonce_str: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct WechatQueryOrderRequestV3 {
    /// 商户号
    pub mchid: String,
    /// 微信支付订单号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// 商户订单号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_trade_no: Option<String>,
}


#[allow(unused)]
impl WechatCloseOrderRequest {
    pub fn parse_xml(&self) -> String {
        let msg = format!(
            "<xml>\n\
                <appid>{appid}</appid>\n\
                <mch_id>{mch_id}</mch_id>\n\
                <nonce_str>{nonce_str}</nonce_str>\n\
                <out_trade_no>{out_trade_no}</out_trade_no>\n\
                <sign>{sign}</sign>\n\
            </xml>",
            appid=self.appid.to_owned().unwrap_or_default(),
            mch_id=self.mch_id,
            nonce_str=self.nonce_str.to_owned().unwrap_or_default(),
            out_trade_no=self.out_trade_no,
            sign=self.sign,
        );
        msg
    }

    pub fn get_sign(&mut self, appkey: &str) {
        let mut pairs = BTreeMap::new();
        if let Some(appid) = self.appid.to_owned() {
            pairs.insert("appid".to_string(), appid);
        }
        pairs.insert("mch_id".to_string(), self.mch_id.to_owned());
        pairs.insert("out_trade_no".to_string(), self.out_trade_no.to_owned());
        if let Some(nonce_str) = self.nonce_str.to_owned() {
            pairs.insert("nonce_str".to_string(), nonce_str);
        }
        self.sign = get_sign(&pairs, appkey);
    }
}

#[allow(unused)]
impl WechatQueryOrderRequest {
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
            appid=self.appid.to_owned().unwrap_or_default(),
            mch_id=self.mch_id.to_owned(),
            nonce_str=self.nonce_str.to_owned().unwrap_or_default(),
            out_trade_no=self.out_trade_no.to_owned().unwrap_or_default(),
            transaction_id=self.transaction_id.to_owned().unwrap_or_default(),
            sign=self.sign,
        );
        msg
    }

    pub fn get_sign(&mut self, appkey: &str) {
        let mut pairs = BTreeMap::new();
        if let Some(appid) = self.appid.to_owned() {
            pairs.insert("appid".to_string(), appid);
        }
        pairs.insert("mch_id".to_string(), self.mch_id.to_owned());
        pairs.insert("out_trade_no".to_string(), self.out_trade_no.to_owned().unwrap_or_default());
        pairs.insert("transaction_id".to_string(), self.transaction_id.to_owned().unwrap_or_default());
        if let Some(nonce_str) = self.nonce_str.to_owned() {
            pairs.insert("nonce_str".to_string(), nonce_str);
        }
        self.sign = get_sign(&pairs, appkey);
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
pub struct WechatRefundRequestV3 {
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



#[derive(Debug, Serialize, Deserialize)]
pub struct WechatRefundRequest {
    pub appid: Option<String>,
    /// 商户号
    pub mch_id: String,
    /// 商户订单号
    pub out_trade_no: String,
    /// 退款订单号
    pub out_refund_no: String,
    /// 交易编号
    pub transaction_id: String,
    /// 回调地址
    pub notify_url: Option<String>,
    /// 退款金额
    pub refund_fee: String,
    /// 总金额
    pub total_fee: String,
    /// 签名
    pub sign: String,
    /// 加密字符串
    pub nonce_str: Option<String>,

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

///
/// 退款参数
/// <xml>
///    <appid>wx2421b1c4370ec43b</appid>
///    <mch_id>10000100</mch_id>
///    <nonce_str>6cefdb308e1e2e8aabd48cf79e546a02</nonce_str>
///    <out_refund_no>1415701182</out_refund_no>
///    <out_trade_no>1415757673</out_trade_no>
///    <refund_fee>1</refund_fee>
///    <total_fee>1</total_fee>
///    <transaction_id></transaction_id>
///    <sign>FE56DD4AA85C0EECA82C35595A69E153</sign>
/// </xml>
///
#[allow(unused)]
impl WechatRefundRequest {
    pub fn parse_xml(&self) -> String {
        let msg = format!(
            "<xml>\n\
                <appid>{appid}</appid>\n\
                <mch_id>{mch_id}</mch_id>\n\
                <nonce_str>{nonce_str}</nonce_str>\n\
                <out_refund_no>{out_refund_no}</out_refund_no>\n\
                <out_trade_no>{out_trade_no}</out_trade_no>\n\
                <refund_fee>{refund_fee}</refund_fee>\n\
                <total_fee>{total_fee}</total_fee>\n\
                <notify_url>{notify_url}</notify_url>\n\
                <transaction_id>{transaction_id}</transaction_id>\n\
                <sign>{sign}</sign>\n\
            </xml>",
            appid=self.appid.to_owned().unwrap_or_default(),
            mch_id=self.mch_id,
            nonce_str=self.nonce_str.to_owned().unwrap_or_default(),
            transaction_id=self.transaction_id,
            out_trade_no=self.out_trade_no,
            out_refund_no=self.out_refund_no,
            refund_fee=self.refund_fee,
            total_fee=self.total_fee,
            notify_url=self.notify_url.to_owned().unwrap_or_default(),
            sign=self.sign,
        );
        msg
    }

    pub fn get_sign(&mut self, appkey: &str) {
        let mut pairs = BTreeMap::new();
        if let Some(appid) = self.appid.to_owned() {
            pairs.insert("appid".to_string(), appid);
        }
        pairs.insert("mch_id".to_string(), self.mch_id.to_owned());
        pairs.insert("out_trade_no".to_string(), self.out_trade_no.to_owned());
        pairs.insert("out_refund_no".to_string(), self.out_refund_no.to_owned());
        pairs.insert("transaction_id".to_string(), self.transaction_id.to_owned());
        pairs.insert("refund_fee".to_string(), self.refund_fee.to_owned());
        pairs.insert("total_fee".to_string(), self.total_fee.to_owned());
        if let Some(notify_url) = self.notify_url.to_owned() {
            pairs.insert("notify_url".to_string(), notify_url);
        }
        if let Some(nonce_str) = self.nonce_str.to_owned() {
            pairs.insert("nonce_str".to_string(), nonce_str);
        }
        // let setting = &SETTINGS;
        self.sign = get_sign(&pairs, appkey);
    }
}


#[allow(unused)]
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
            appid=self.appid.to_owned().unwrap_or_default(),
            mch_id=self.mch_id,
            nonce_str=self.nonce_str.to_owned().unwrap_or_default(),
            transaction_id=self.transaction_id,
            out_trade_no=self.out_trade_no,
            sign=self.sign,
        );
        msg
    }

    pub fn get_sign(&mut self, appkey: &str) {
        let mut pairs = BTreeMap::new();
        if let Some(appid) = self.appid.to_owned() {
            pairs.insert("appid".to_string(), appid);
        }
        pairs.insert("mch_id".to_string(), self.mch_id.to_owned());
        pairs.insert("out_trade_no".to_string(), self.out_trade_no.to_owned());
        pairs.insert("transaction_id".to_string(), self.transaction_id.to_owned());
        self.sign = get_sign(&pairs, appkey);
    }
}




#[derive(Debug, Serialize, Deserialize)]
pub struct WechatQueryRefundOrderRequest {
    pub appid: Option<String>,
    /// 商户号
    pub mch_id: String,
    //************以下四选一************
    /// 退款订单号
    pub out_refund_no: Option<String>,
    /// 微信支付订单号
    pub transaction_id: Option<String>,
    /// 商户订单号
    pub out_trade_no: Option<String>,
    /// 微信退款单号
    pub refund_id: Option<String>,
    /// 签名
    pub sign: String,
    /// 加密字符串
    pub nonce_str: Option<String>,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct WechatQueryRefundOrderRequestV3 {
    /// 退款订单号
    /// 商户系统内部的退款单号，商户系统内部唯一，只能是数字、大小写字母_-|*@ ，同一退款单号多次请求只退一笔。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_refund_no: Option<String>,
}

#[allow(unused)]
impl WechatQueryRefundOrderRequest {
    pub fn parse_xml(&self) -> String {
        let msg = format!(
            "<xml>\n\
                <appid>{appid}</appid>\n\
                <mch_id>{mch_id}</mch_id>\n\
                <nonce_str>{nonce_str}</nonce_str>\n\
                <out_refund_no>{out_refund_no}</out_refund_no>\n\
                <out_trade_no>{out_trade_no}</out_trade_no>\n\
                <refund_id>{refund_id}</refund_id>\n\
                <transaction_id>{transaction_id}</transaction_id>\n\
                <sign>{sign}</sign>\n\
            </xml>",
            appid=self.appid.to_owned().unwrap_or_default(),
            mch_id=self.mch_id,
            nonce_str=self.nonce_str.to_owned().unwrap_or_default(),
            transaction_id=self.transaction_id.to_owned().unwrap_or_default(),
            out_trade_no=self.out_trade_no.to_owned().unwrap_or_default(),
            out_refund_no=self.out_refund_no.to_owned().unwrap_or_default(),
            refund_id=self.refund_id.to_owned().unwrap_or_default(),
            sign=self.sign,
        );
        msg
    }

    pub fn get_sign(&mut self, appkey: &str) {
        let mut pairs = BTreeMap::new();
        if let Some(appid) = self.appid.to_owned() {
            pairs.insert("appid".to_string(), appid);
        }
        pairs.insert("mch_id".to_string(), self.mch_id.to_owned());
        pairs.insert("out_trade_no".to_string(), self.out_trade_no.to_owned().unwrap_or_default());
        pairs.insert("out_refund_no".to_string(), self.out_refund_no.to_owned().unwrap_or_default());
        pairs.insert("transaction_id".to_string(), self.transaction_id.to_owned().unwrap_or_default());
        pairs.insert("refund_id".to_string(), self.refund_id.to_owned().unwrap_or_default());
        if let Some(nonce_str) = self.nonce_str.to_owned() {
            pairs.insert("nonce_str".to_string(), nonce_str);
        }
        // let setting = &SETTINGS;
        self.sign = get_sign(&pairs, appkey);
    }
}



#[derive(Debug, Serialize, Deserialize)]
pub struct WxPayShorturlRequest {
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
    pub mch_id: String,
    /// 签名
    pub sign: String,
    /// 加密字符串
    pub nonce_str: Option<String>,

}


#[allow(unused)]
impl WxPayShorturlRequest {
    pub fn parse_xml(&self) -> String {
        let msg = format!(
            "<xml>\n\
                <appid>{appid}</appid>\n\
                <mch_id>{mch_id}</mch_id>\n\
                <nonce_str>{nonce_str}</nonce_str>\n\
                <long_url>{long_url}</long_url>\n\
                <sign>{sign}</sign>\n\
            </xml>",
            appid=self.appid.to_owned().unwrap_or_default(),
            mch_id=self.mch_id,
            nonce_str=self.nonce_str.to_owned().unwrap_or_default(),
            long_url=self.long_url.to_owned().unwrap_or_default(),
            sign=self.sign,
        );
        msg
    }

    pub fn get_sign(&mut self, appkey: &str) {
        let mut pairs = BTreeMap::new();
        if let Some(appid) = self.appid.to_owned() {
            pairs.insert("appid".to_string(), appid);
        }
        pairs.insert("mch_id".to_string(), self.mch_id.to_owned());
        pairs.insert("long_url".to_string(), self.long_url.to_owned().unwrap_or_default());
        if let Some(nonce_str) = self.nonce_str.to_owned() {
            pairs.insert("nonce_str".to_string(), nonce_str);
        }
        // let setting = &SETTINGS;
        self.sign = get_sign(&pairs, appkey);
    }
}
