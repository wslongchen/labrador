use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use crate::{AlipayRequest};
use crate::alipay::constants::BIZ_CONTENT_KEY;
use crate::alipay::method::AlipayMethod;

//----------------------------------------------------------------------------------------------------------------------------

// 支付宝 ↓`

#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayTradeWapPayRequest<T: Serialize> {
    /// API版本
    pub api_version: String,
    /// 回调地址
    pub notify_url: Option<String>,
    /// 跳转地址
    pub return_url: Option<String>,
    /// 业务内容
    pub biz_content: Option<String>,
    /// 终端类型
    pub terminal_type: Option<String>,
    /// 终端信息
    pub terminal_info: Option<String>,
    /// 产品编码
    pub prod_code: Option<String>,
    /// 是否需要加密
    pub need_encrypt: bool,
    /// 参数
    pub udf_params: BTreeMap<String, String>,
    /// 业务实体
    pub biz_model: Option<T>
}

impl <T> AlipayTradeWapPayRequest<T> where T: Serialize {
    pub fn new() -> Self {
        Self {
            api_version: "1.0".to_string(),
            notify_url: None,
            return_url: None,
            biz_content: None,
            terminal_type: None,
            terminal_info: None,
            prod_code: None,
            need_encrypt: false,
            udf_params: BTreeMap::new(),
            biz_model: None
        }
    }

    pub fn put_other_text_param(&mut self, key: String, value: String) {
        self.udf_params.insert(key, value);
    }
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
    pub goods_detail: Option<AlipayGoodsDetail>,
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


impl <T> AlipayRequest<T> for AlipayTradeWapPayRequest<T> where T: Serialize {
    fn get_api_method_name(&self) -> AlipayMethod {
        AlipayMethod::WapPay
    }

    fn get_text_params(&self) -> BTreeMap<String, String> {
        let mut txt_params = BTreeMap::new();
        txt_params.insert(BIZ_CONTENT_KEY.to_string(), serde_json::to_string(&self.get_biz_model()).unwrap_or_default());
        txt_params
    }

    fn get_api_version(&self) -> String {
        if self.api_version.is_empty() {
            "1.0".to_string()
        } else {
            self.api_version.to_string()
        }
    }

    fn get_terminal_type(&self) -> String {
        self.terminal_type.to_owned().unwrap_or_default()
    }

    fn get_terminal_info(&self) -> String {
        self.terminal_info.to_owned().unwrap_or_default()
    }

    fn get_prod_code(&self) -> String {
        self.prod_code.to_owned().unwrap_or_default()
    }

    fn get_notify_url(&self) -> String {
        self.notify_url.to_owned().unwrap_or_default()
    }

    fn get_return_url(&self) -> String {
        self.return_url.to_owned().unwrap_or_default()
    }

    fn is_need_encrypt(&self) -> bool {
        self.need_encrypt
    }

    fn get_biz_content(self) -> String {
        self.biz_content.to_owned().unwrap_or_default()
    }

    fn get_biz_model(&self) -> Option<&T> {
        self.biz_model.as_ref()
    }
}

//----------------------------------------------------------------------------------------------------------------------------


#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayTradePagePayRequest<T: Serialize> {
    /// API版本
    pub api_version: String,
    /// 回调地址
    pub notify_url: Option<String>,
    /// 跳转地址
    pub return_url: Option<String>,
    /// 业务内容
    pub biz_content: Option<String>,
    /// 终端类型
    pub terminal_type: Option<String>,
    /// 终端信息
    pub terminal_info: Option<String>,
    /// 产品编码
    pub prod_code: Option<String>,
    /// 是否需要加密
    pub need_encrypt: bool,
    /// 参数
    pub udf_params: BTreeMap<String, String>,
    /// 业务实体
    pub biz_model: Option<T>
}

impl <T> AlipayTradePagePayRequest<T> where T: Serialize {
    pub fn new() -> Self {
        Self {
            api_version: "1.0".to_string(),
            notify_url: None,
            return_url: None,
            biz_content: None,
            terminal_type: None,
            terminal_info: None,
            prod_code: None,
            need_encrypt: false,
            udf_params: BTreeMap::new(),
            biz_model: None
        }
    }

    pub fn put_other_text_param(&mut self, key: String, value: String) {
        self.udf_params.insert(key, value);
    }
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
    pub goods_detail: Option<AlipayGoodsDetail>,
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


impl <T> AlipayRequest<T> for AlipayTradePagePayRequest<T> where T: Serialize {
    fn get_api_method_name(&self) -> AlipayMethod {
        AlipayMethod::PCPay
    }

    fn get_text_params(&self) -> BTreeMap<String, String> {
        let mut txt_params = BTreeMap::new();
        txt_params.insert(BIZ_CONTENT_KEY.to_string(), serde_json::to_string(&self.get_biz_model()).unwrap_or_default());
        if !self.udf_params.is_empty() {
            for (k, v) in &self.udf_params {
                txt_params.insert(k.to_string(), v.to_string());
            }
        }
        txt_params
    }

    fn get_api_version(&self) -> String {
        if self.api_version.is_empty() {
            "1.0".to_string()
        } else {
            self.api_version.to_string()
        }
    }

    fn get_terminal_type(&self) -> String {
        self.terminal_type.to_owned().unwrap_or_default()
    }

    fn get_terminal_info(&self) -> String {
        self.terminal_info.to_owned().unwrap_or_default()
    }

    fn get_prod_code(&self) -> String {
        self.prod_code.to_owned().unwrap_or_default()
    }

    fn get_notify_url(&self) -> String {
        self.notify_url.to_owned().unwrap_or_default()
    }

    fn get_return_url(&self) -> String {
        self.return_url.to_owned().unwrap_or_default()
    }

    fn is_need_encrypt(&self) -> bool {
        self.need_encrypt
    }

    fn get_biz_content(self) -> String {
        self.biz_content.to_owned().unwrap_or_default()
    }

    fn get_biz_model(&self) -> Option<&T> {
        self.biz_model.as_ref()
    }
}

//----------------------------------------------------------------------------------------------------------------------------



#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayTradeAppPayRequest<T: Serialize> {
    /// API版本
    pub api_version: String,
    /// 回调地址
    pub notify_url: Option<String>,
    /// 跳转地址
    pub return_url: Option<String>,
    /// 业务内容
    pub biz_content: Option<String>,
    /// 终端类型
    pub terminal_type: Option<String>,
    /// 终端信息
    pub terminal_info: Option<String>,
    /// 产品编码
    pub prod_code: Option<String>,
    /// 是否需要加密
    pub need_encrypt: bool,
    /// 参数
    pub udf_params: BTreeMap<String, String>,
    /// 业务实体
    pub biz_model: Option<T>
}

impl <T> AlipayTradeAppPayRequest<T> where T: Serialize {
    pub fn new() -> Self {
        Self {
            api_version: "1.0".to_string(),
            notify_url: None,
            return_url: None,
            biz_content: None,
            terminal_type: None,
            terminal_info: None,
            prod_code: None,
            need_encrypt: false,
            udf_params: BTreeMap::new(),
            biz_model: None
        }
    }

    pub fn put_other_text_param(&mut self, key: String, value: String) {
        self.udf_params.insert(key, value);
    }
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
    pub goods_detail: Option<AlipayGoodsDetail>,
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



impl <T> AlipayRequest<T> for AlipayTradeAppPayRequest<T> where T: Serialize {
    fn get_api_method_name(&self) -> AlipayMethod {
        AlipayMethod::AppPay
    }

    fn get_text_params(&self) -> BTreeMap<String, String> {
        let mut txt_params = BTreeMap::new();
        txt_params.insert(BIZ_CONTENT_KEY.to_string(), serde_json::to_string(&self.get_biz_model()).unwrap_or_default());
        if !self.udf_params.is_empty() {
            for (k, v) in &self.udf_params {
                txt_params.insert(k.to_string(), v.to_string());
            }
        }
        txt_params
    }

    fn get_api_version(&self) -> String {
        if self.api_version.is_empty() {
            "1.0".to_string()
        } else {
            self.api_version.to_string()
        }
    }

    fn get_terminal_type(&self) -> String {
        self.terminal_type.to_owned().unwrap_or_default()
    }

    fn get_terminal_info(&self) -> String {
        self.terminal_info.to_owned().unwrap_or_default()
    }

    fn get_prod_code(&self) -> String {
        self.prod_code.to_owned().unwrap_or_default()
    }

    fn get_notify_url(&self) -> String {
        self.notify_url.to_owned().unwrap_or_default()
    }

    fn get_return_url(&self) -> String {
        self.return_url.to_owned().unwrap_or_default()
    }

    fn is_need_encrypt(&self) -> bool {
        self.need_encrypt
    }

    fn get_biz_content(self) -> String {
        self.biz_content.to_owned().unwrap_or_default()
    }

    fn get_biz_model(&self) -> Option<&T> {
        self.biz_model.as_ref()
    }
}

//----------------------------------------------------------------------------------------------------------------------------


#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayTradePayRequest<T: Serialize> {
    /// API版本
    pub api_version: String,
    /// 回调地址
    pub notify_url: Option<String>,
    /// 跳转地址
    pub return_url: Option<String>,
    /// 业务内容
    pub biz_content: Option<String>,
    /// 终端类型
    pub terminal_type: Option<String>,
    /// 终端信息
    pub terminal_info: Option<String>,
    /// 产品编码
    pub prod_code: Option<String>,
    /// 是否需要加密
    pub need_encrypt: bool,
    /// 参数
    pub udf_params: BTreeMap<String, String>,
    /// 业务实体
    pub biz_model: Option<T>
}

impl <T> AlipayTradePayRequest<T> where T: Serialize {
    pub fn new() -> Self {
        Self {
            api_version: "1.0".to_string(),
            notify_url: None,
            return_url: None,
            biz_content: None,
            terminal_type: None,
            terminal_info: None,
            prod_code: None,
            need_encrypt: false,
            udf_params: BTreeMap::new(),
            biz_model: None
        }
    }

    pub fn put_other_text_param(&mut self, key: String, value: String) {
        self.udf_params.insert(key, value);
    }
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

#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayTradePayModel {
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
    /// bar_code：当面付条码支付场景；
    /// security_code：当面付刷脸支付场景，对应的auth_code为fp开头的刷脸标识串；
    /// 周期扣款或代扣场景无需传入，协议号通过agreement_params参数传递；
    /// 支付宝预授权和新当面资金授权场景无需传入，授权订单号通过 auth_no字段传入。
    /// 默认值为bar_code。
    /// </pre>
    pub scene: String,
    /// <pre>
    /// 支付授权码。
    /// 当面付场景传买家的付款码（25~30开头的长度为16~24位的数字，实际字符串长度以开发者获取的付款码长度为准）或者刷脸标识串（fp开头的35位字符串）；
    /// 周期扣款或代扣场景无需传入，协议号通过agreement_params参数传递；
    /// 支付宝预授权和新当面资金授权场景无需传入，授权订单号通过 auth_no字段传入。
    /// 注：交易的买家与卖家不能相同。
    /// </pre>
    pub auth_code: String,
    /// <pre>
    /// 产品码。
    /// 商家和支付宝签约的产品码。 枚举值（点击查看签约情况）：
    /// FACE_TO_FACE_PAYMENT：当面付产品；
    /// CYCLE_PAY_AUTH：周期扣款产品；
    /// GENERAL_WITHHOLDING：代扣产品；
    /// PRE_AUTH_ONLINE：支付宝预授权产品；
    /// PRE_AUTH：新当面资金授权产品；
    /// 默认值为FACE_TO_FACE_PAYMENT。
    /// 注意：非当面付产品使用本接口时，本参数必填。请传入对应产品码。
    /// </pre>
    pub product_code: Option<String>,
    /// 资金预授权单号。
    /// 支付宝预授权和新当面资金授权场景下必填。
    pub auth_no: Option<String>,
    /// 预授权确认模式。
    /// <pre>
    /// 适用于支付宝预授权和新当面资金授权场景。枚举值：
    /// COMPLETE：转交易完成后解冻剩余冻结金额；
    /// NOT_COMPLETE：转交易完成后不解冻剩余冻结金额；
    /// 默认值为NOT_COMPLETE。
    /// </pre>
    pub auth_confirm_mode: String,
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
    /// 买家支付宝用户ID。
    /// 支付宝预授权和新当面资金授权场景下必填，其它场景不需要传入。
    pub buyer_id: Option<String>,
    /// 订单附加信息。
    /// 如果请求时传递了该参数，将在异步通知、对账单中原样返回，同时会在商户和用户的pc账单详情中作为交易描述展示
    pub body: Option<String>,
    /// 订单包含的商品列表信息，json格式，其它说明详见商品明细说明
    pub goods_detail: Option<AlipayGoodsDetail>,
    /// 订单绝对超时时间。
    /// 格式为yyyy-MM-dd HH:mm:ss。
    /// 注：time_expire和timeout_express两者只需传入一个或者都不传，两者均传入时，优先使用time_expire。
    pub time_expire: Option<String>,
    /// 订单相对超时时间。从交易创建时间开始计算。
    /// 该笔订单允许的最晚付款时间，逾期将关闭交易。取值范围：1m～15d。m-分钟，h-小时，d-天，1c-当天（1c-当天的情况下，无论交易何时创建，都在0点关闭）。 该参数数值不接受小数点， 如 1.5h，可转换为 90m。
    /// 当面付场景默认值为3h；
    /// 其它场景默认值为15d;
    pub timeout_express: Option<String>,
    /// 结算信息、
    /// json格式，详见结算参数说明。
    /// 直付通模式下必传。
    pub settle_info: Option<SettleInfo>,
    /// 二级商户信息。
    /// 直付通模式和机构间连模式下必传，其它场景下不需要传入。。
    pub sub_merchant: Option<SubMerchantInfo>,
    /// 业务扩展参数
    pub extend_params: Option<ExtendParams>,
    /// 优惠明细参数，通过此属性补充营销参数
    pub promo_params: Option<PromoParam>,
    /// 支付模式类型,若值为ENJOY_PAY_V2表示当前交易允许走先享后付2.0垫资
    pub advance_payment_type: Option<String>,
    /// 支付相关参数
    pub pay_params: Option<PayParams>,
    /// 商户门店编号。
    /// 指商户创建门店时输入的门店编号。
    pub store_id: Option<String>,
    /// 商户操作员编号。
    pub operator_id: Option<String>,
    /// 商户机具终端编号。
    pub terminal_id: Option<String>,
    /// 收单机构(例如银行）的标识，填写该机构在支付宝的pid。只在机构间联场景下传递该值。
    pub request_org_pid: Option<String>,
    /// 返回参数选项。
    /// 商户通过传递该参数来定制同步需要额外返回的信息字段，数组格式。包括但不限于：["fund_bill_list","voucher_detail_list","enterprise_pay_info","discount_goods_detail","discount_amount","mdiscount_amount"]
    pub query_options: Option<Vec<String>>,
}



impl <T> AlipayRequest<T> for AlipayTradePayRequest<T> where T: Serialize {
    fn get_api_method_name(&self) -> AlipayMethod {
        AlipayMethod::UnifiedPay
    }

    fn get_text_params(&self) -> BTreeMap<String, String> {
        let mut txt_params = BTreeMap::new();
        txt_params.insert(BIZ_CONTENT_KEY.to_string(), serde_json::to_string(&self.get_biz_model()).unwrap_or_default());
        if !self.udf_params.is_empty() {
            for (k, v) in &self.udf_params {
                txt_params.insert(k.to_string(), v.to_string());
            }
        }
        txt_params
    }

    fn get_api_version(&self) -> String {
        if self.api_version.is_empty() {
            "1.0".to_string()
        } else {
            self.api_version.to_string()
        }
    }

    fn get_terminal_type(&self) -> String {
        self.terminal_type.to_owned().unwrap_or_default()
    }

    fn get_terminal_info(&self) -> String {
        self.terminal_info.to_owned().unwrap_or_default()
    }

    fn get_prod_code(&self) -> String {
        self.prod_code.to_owned().unwrap_or_default()
    }

    fn get_notify_url(&self) -> String {
        self.notify_url.to_owned().unwrap_or_default()
    }

    fn get_return_url(&self) -> String {
        self.return_url.to_owned().unwrap_or_default()
    }

    fn is_need_encrypt(&self) -> bool {
        self.need_encrypt
    }

    fn get_biz_content(self) -> String {
        self.biz_content.to_owned().unwrap_or_default()
    }

    fn get_biz_model(&self) -> Option<&T> {
        self.biz_model.as_ref()
    }
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
    pub goods_detail: Option<AlipayGoodsDetail>,
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
    /// 回调地址
    pub notify_url: Option<String>,
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
    pub goods_detail: Option<AlipayGoodsDetail>,
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


/// 预授权
/// 用户在商户侧授权冻结并享受服务后，商户使用授权单号通过本接口对用户已授权金额发起扣款
#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayPreAuthOnlinePayModel {
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
    /// 商家和支付宝签约的产品码。 支付宝预授权场景传：PRE_AUTH_ONLINE；新当面资金授权场景传：PRE_AUTH；
    /// </pre>
    pub product_code: String,
    /// 资金预授权单号。
    /// 支付宝预授权和新当面资金授权场景下必填。
    pub auth_no: Option<String>,
    /// 预授权确认模式。
    /// <pre>
    /// 适用于支付宝预授权和新当面资金授权场景。枚举值：
    /// COMPLETE：转交易完成后解冻剩余冻结金额；
    /// NOT_COMPLETE：转交易完成后不解冻剩余冻结金额；
    /// 默认值为NOT_COMPLETE。
    /// </pre>
    pub auth_confirm_mode: String,
    /// 订单包含的商品列表信息，json格式，其它说明详见商品明细说明
    pub goods_detail: Option<AlipayGoodsDetail>,
    /// 业务扩展参数
    pub extend_params: Option<ExtendParams>,
    /// 优惠明细参数，通过此属性补充营销参数
    pub promo_params: Option<PromoParam>,
    /// 商户门店编号。
    /// 指商户创建门店时输入的门店编号。
    pub store_id: Option<String>,
    /// 商户机具终端编号。
    pub terminal_id: Option<String>,
    /// 返回参数选项。
    /// 商户通过传递该参数来定制同步需要额外返回的信息字段，数组格式。如：["fund_bill_list","voucher_detail_list","discount_goods_detail"]
    pub query_options: Option<Vec<String>>,
}

//----------------------------------------------------------------------------------------------------------------------------


#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayTradePrecreateRequest<T: Serialize> {
    /// API版本
    pub api_version: String,
    /// 回调地址
    pub notify_url: Option<String>,
    /// 跳转地址
    pub return_url: Option<String>,
    /// 业务内容
    pub biz_content: Option<String>,
    /// 终端类型
    pub terminal_type: Option<String>,
    /// 终端信息
    pub terminal_info: Option<String>,
    /// 产品编码
    pub prod_code: Option<String>,
    /// 是否需要加密
    pub need_encrypt: bool,
    /// 参数
    pub udf_params: BTreeMap<String, String>,
    /// 业务实体
    pub biz_model: Option<T>
}

impl <T> AlipayTradePrecreateRequest<T> where T: Serialize {
    pub fn new() -> Self {
        Self {
            api_version: "1.0".to_string(),
            notify_url: None,
            return_url: None,
            biz_content: None,
            terminal_type: None,
            terminal_info: None,
            prod_code: None,
            need_encrypt: false,
            udf_params: BTreeMap::new(),
            biz_model: None
        }
    }

    pub fn put_other_text_param(&mut self, key: String, value: String) {
        self.udf_params.insert(key, value);
    }
}


/// 统一收单线下交易预创建
/// 收银员通过收银台或商户后台调用支付宝接口，生成二维码后，展示给用户，由用户扫描二维码完成订单支付。
#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayTradePrecreateModel {
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
    /// 订单绝对超时时间。
    /// 格式为yyyy-MM-dd HH:mm:ss。
    /// 注：time_expire和timeout_express两者只需传入一个或者都不传，两者均传入时，优先使用time_expire。
    pub time_expire: Option<String>,
    /// 订单相对超时时间。从交易创建时间开始计算。
    /// 该笔订单允许的最晚付款时间，逾期将关闭交易。取值范围：1m～15d。m-分钟，h-小时，d-天，1c-当天（1c-当天的情况下，无论交易何时创建，都在0点关闭）。 该参数数值不接受小数点， 如 1.5h，可转换为 90m。
    /// 当面付场景默认值为3h；
    /// 其它场景默认值为15d;
    pub timeout_express: Option<String>,
    /// 结算信息、
    /// json格式，详见结算参数说明。
    /// 直付通模式下必传。
    pub settle_info: Option<SettleInfo>,
    /// 订单包含的商品列表信息，json格式，其它说明详见商品明细说明
    pub goods_detail: Option<AlipayGoodsDetail>,
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
    /// 禁用渠道,用户不可用指定渠道支付，多个渠道以逗号分割
    /// 注，与enable_pay_channels互斥
    /// [渠道列表](https://opendocs.alipay.com/open/common/wifww7)
    pub disable_pay_channels: Option<String>,
    /// 指定支付渠道。
    /// 用户只能使用指定的渠道进行支付，多个渠道以逗号分割。
    /// 与disable_pay_channels互斥，支持传入的值：[渠道列表](https://opendocs.alipay.com/open/common/wifww7)。
    /// 注：如果传入了指定支付渠道，则用户只能用指定内的渠道支付，包括营销渠道也要指定才能使用。该参数可能导致用户支付受限，慎用。
    pub enable_pay_channels: Option<String>,
    /// 商户原始订单号，最大长度限制32位
    pub merchant_order_no: Option<String>,
    /// 回调地址
    pub notify_url: Option<String>,
}



impl <T> AlipayRequest<T> for AlipayTradePrecreateRequest<T> where T: Serialize {
    fn get_api_method_name(&self) -> AlipayMethod {
        AlipayMethod::PreUnifiedOrder
    }

    fn get_text_params(&self) -> BTreeMap<String, String> {
        let mut txt_params = BTreeMap::new();
        txt_params.insert(BIZ_CONTENT_KEY.to_string(), serde_json::to_string(&self.get_biz_model()).unwrap_or_default());
        if !self.udf_params.is_empty() {
            for (k, v) in &self.udf_params {
                txt_params.insert(k.to_string(), v.to_string());
            }
        }
        txt_params
    }

    fn get_api_version(&self) -> String {
        if self.api_version.is_empty() {
            "1.0".to_string()
        } else {
            self.api_version.to_string()
        }
    }

    fn get_terminal_type(&self) -> String {
        self.terminal_type.to_owned().unwrap_or_default()
    }

    fn get_terminal_info(&self) -> String {
        self.terminal_info.to_owned().unwrap_or_default()
    }

    fn get_prod_code(&self) -> String {
        self.prod_code.to_owned().unwrap_or_default()
    }

    fn get_notify_url(&self) -> String {
        self.notify_url.to_owned().unwrap_or_default()
    }

    fn get_return_url(&self) -> String {
        self.return_url.to_owned().unwrap_or_default()
    }

    fn is_need_encrypt(&self) -> bool {
        self.need_encrypt
    }

    fn get_biz_content(self) -> String {
        self.biz_content.to_owned().unwrap_or_default()
    }

    fn get_biz_model(&self) -> Option<&T> {
        self.biz_model.as_ref()
    }
}

//----------------------------------------------------------------------------------------------------------------------------




#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayTradeCreateRequest<T: Serialize> {
    /// API版本
    pub api_version: String,
    /// 回调地址
    pub notify_url: Option<String>,
    /// 跳转地址
    pub return_url: Option<String>,
    /// 业务内容
    pub biz_content: Option<String>,
    /// 终端类型
    pub terminal_type: Option<String>,
    /// 终端信息
    pub terminal_info: Option<String>,
    /// 产品编码
    pub prod_code: Option<String>,
    /// 是否需要加密
    pub need_encrypt: bool,
    /// 参数
    pub udf_params: BTreeMap<String, String>,
    /// 业务实体
    pub biz_model: Option<T>
}

impl <T> AlipayTradeCreateRequest<T> where T: Serialize {
    pub fn new() -> Self {
        Self {
            api_version: "1.0".to_string(),
            notify_url: None,
            return_url: None,
            biz_content: None,
            terminal_type: None,
            terminal_info: None,
            prod_code: None,
            need_encrypt: false,
            udf_params: BTreeMap::new(),
            biz_model: None
        }
    }

    pub fn put_other_text_param(&mut self, key: String, value: String) {
        self.udf_params.insert(key, value);
    }
}

/// 统一收单交易创建接口
/// 商户通过该接口进行交易的创建下单
#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayTradeCreateModel {
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
    /// 买家支付宝用户ID。
    /// 2088开头的16位纯数字，小程序场景下获取用户ID请参考：[用户授权](https://opendocs.alipay.com/mini/introduce/authcode);
    /// 其它场景下获取用户ID请参考：[网页授权获取用户信息](https://opendocs.alipay.com/open/284/106001/#s4);
    /// 注：交易的买家与卖家不能相同。
    pub buyer_id: Option<String>,
    /// 订单附加信息。
    /// 如果请求时传递了该参数，将在异步通知、对账单中原样返回，同时会在商户和用户的pc账单详情中作为交易描述展示
    pub body: Option<String>,
    /// 订单绝对超时时间。
    /// 格式为yyyy-MM-dd HH:mm:ss。
    /// 注：time_expire和timeout_express两者只需传入一个或者都不传，两者均传入时，优先使用time_expire。
    pub time_expire: Option<String>,
    /// 订单相对超时时间。从交易创建时间开始计算。
    /// 该笔订单允许的最晚付款时间，逾期将关闭交易。取值范围：1m～15d。m-分钟，h-小时，d-天，1c-当天（1c-当天的情况下，无论交易何时创建，都在0点关闭）。 该参数数值不接受小数点， 如 1.5h，可转换为 90m。
    /// 当面付场景默认值为3h；
    /// 其它场景默认值为15d;
    pub timeout_express: Option<String>,
    /// 结算信息、
    /// json格式，详见结算参数说明。
    /// 直付通模式下必传。
    pub settle_info: Option<SettleInfo>,
    /// 订单包含的商品列表信息，json格式，其它说明详见商品明细说明
    pub goods_detail: Option<AlipayGoodsDetail>,
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
    /// 物流信息
    pub logistics_detail: Option<LogisticsDetail>,
    /// 收货人及地址信息
    pub receiver_address_info: Option<ReceiverAddressInfo>,
}



impl <T> AlipayRequest<T> for AlipayTradeCreateRequest<T> where T: Serialize {
    fn get_api_method_name(&self) -> AlipayMethod {
        AlipayMethod::CreateUnifiedOrder
    }

    fn get_text_params(&self) -> BTreeMap<String, String> {
        let mut txt_params = BTreeMap::new();
        txt_params.insert(BIZ_CONTENT_KEY.to_string(), serde_json::to_string(&self.get_biz_model()).unwrap_or_default());
        if !self.udf_params.is_empty() {
            for (k, v) in &self.udf_params {
                txt_params.insert(k.to_string(), v.to_string());
            }
        }
        txt_params
    }

    fn get_api_version(&self) -> String {
        if self.api_version.is_empty() {
            "1.0".to_string()
        } else {
            self.api_version.to_string()
        }
    }

    fn get_terminal_type(&self) -> String {
        self.terminal_type.to_owned().unwrap_or_default()
    }

    fn get_terminal_info(&self) -> String {
        self.terminal_info.to_owned().unwrap_or_default()
    }

    fn get_prod_code(&self) -> String {
        self.prod_code.to_owned().unwrap_or_default()
    }

    fn get_notify_url(&self) -> String {
        self.notify_url.to_owned().unwrap_or_default()
    }

    fn get_return_url(&self) -> String {
        self.return_url.to_owned().unwrap_or_default()
    }

    fn is_need_encrypt(&self) -> bool {
        self.need_encrypt
    }

    fn get_biz_content(self) -> String {
        self.biz_content.to_owned().unwrap_or_default()
    }

    fn get_biz_model(&self) -> Option<&T> {
        self.biz_model.as_ref()
    }
}

//----------------------------------------------------------------------------------------------------------------------------



#[derive(Debug, Serialize, Deserialize)]
pub struct ReceiverAddressInfo {
    /// 收货人的姓名
    pub name: Option<String>,
    /// 收货地址
    pub address: Option<String>,
    /// 收货人手机号
    pub mobile: Option<String>,
    /// 收货地址邮编
    pub zip: Option<String>,
    /// 中国标准城市区域码
    pub division_code: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct LogisticsDetail {
    /// <pre>
    /// 物流类型,
    /// POST 平邮,
    /// EXPRESS 其他快递,
    /// VIRTUAL 虚拟物品,
    /// EMS EMS,
    /// DIRECT 无需物流。
    /// </pre>
    pub logistics_type: Option<String>
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

/// 外部指定买家
#[derive(Debug, Serialize, Deserialize)]
pub struct PromoParam {
    /// 存在延迟扣款这一类的场景，用这个时间表明用户发生交易的时间，比如说，在公交地铁场景，用户刷码出站的时间，和商户上送交易的时间是不一样的。
    pub actual_order_time: Option<String>,
}

/// 外部指定买家
#[derive(Debug, Serialize, Deserialize)]
pub struct SettleInfo {
    /// 结算详细信息，json数组，目前只支持一条。
    pub settle_detail_infos: Option<Vec<SettleDetailInfo>>,
    /// 该笔订单的超期自动确认结算时间，到达期限后，将自动确认结算。此字段只在签约账期结算模式时有效。取值范围：1d～365d。d-天。 该参数数值不接受小数点。
    pub settle_period_time: Option<String>,
}

/// 外部指定买家
#[derive(Debug, Serialize, Deserialize)]
pub struct SettleDetailInfo {
    /// 结算收款方的账户类型。
    /// <pre>
    /// cardAliasNo：结算收款方的银行卡编号;
    /// userId：表示是支付宝账号对应的支付宝唯一用户号;
    /// loginName：表示是支付宝登录号；
    /// defaultSettle：表示结算到商户进件时设置的默认结算账号，结算主体为门店时不支持传defaultSettle；
    /// </pre>
    pub trans_in_type: String,
    /// 结算收款方。当结算收款方类型是cardAliasNo时，本参数为用户在支付宝绑定的卡编号；结算收款方类型是userId时，本参数为用户的支付宝账号对应的支付宝唯一用户号，以2088开头的纯16位数字；当结算收款方类型是loginName时，本参数为用户的支付宝登录号；当结算收款方类型是defaultSettle时，本参数不能传值，保持为空。
    pub trans_in: String,
    /// 结算汇总维度，按照这个维度汇总成批次结算，由商户指定。
    /// 目前需要和结算收款方账户类型为cardAliasNo配合使用
    pub summary_dimension: Option<String>,
    /// 结算主体标识。当结算主体类型为SecondMerchant时，为二级商户的SecondMerchantID；当结算主体类型为Store时，为门店的外标。
    pub settle_entity_id: Option<String>,
    /// 结算主体类型。
    /// 二级商户:SecondMerchant;商户或者直连商户门店:Store
    pub settle_entity_type: Option<String>,
    /// 结算的金额，单位为元。在创建订单和支付接口时必须和交易金额相同。在结算确认接口时必须等于交易金额减去已退款金额。
    pub amount: Option<f64>,
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
pub struct ExtendParams {
    /// 系统商编号
    /// 该参数作为系统商返佣数据提取的依据，请填写系统商签约协议的PID
    pub sys_service_provider_id: Option<String>,
    /// 使用花呗分期要进行的分期数
    pub hb_fq_num: Option<String>,
    /// 使用花呗分期需要卖家承担的手续费比例的百分值，传入100代表100%
    pub hb_fq_seller_percent: Option<String>,
    /// 行业数据回流信息, 详见：地铁支付接口参数补充说明
    pub industry_reflux_info: Option<String>,
    /// 卡类型
    pub card_type: Option<String>,
    /// 特殊场景下，允许商户指定交易展示的卖家名称
    pub specified_seller_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessParams {
    /// 校园卡编号
    pub campus_card: Option<String>,
    /// 虚拟卡卡类型
    pub card_type: Option<String>,
    /// 实际订单时间，在乘车码场景，传入的是用户刷码乘车时间
    pub actual_order_time: Option<String>,
    /// 商户传入的交易税费。需要落地风控使用
    pub good_taxes: Option<String>,
    /// 因公付业务信息
    pub enterprise_pay_info: Option<String>,
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


/// 订单包含的商品列表信息
#[derive(Debug, Serialize, Deserialize)]
pub struct AlipayGoodsDetail {
    /// 商品的编号
    pub goods_id: String,
    /// 支付宝定义的统一商品编号
    pub alipay_goods_id: String,
    /// 商品名称
    pub goods_name: String,
    /// 商品数量
    pub quantity: String,
    /// 商品单价，单位为元
    pub price: f64,
    /// 商品类目
    pub goods_category: Option<String>,
    /// 商品类目树，从商品类目根节点到叶子节点的类目id组成，类目id值使用|分割
    pub categories_tree: Option<String>,
    /// 商品描述信息
    pub body: Option<String>,
    /// 商品的展示地址
    pub show_url: Option<String>,
}
//----------------------------------------------------------------------------------------------------------------------------



#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayTradeQueryRequest<T: Serialize> {
    /// API版本
    pub api_version: String,
    /// 回调地址
    pub notify_url: Option<String>,
    /// 跳转地址
    pub return_url: Option<String>,
    /// 业务内容
    pub biz_content: Option<String>,
    /// 终端类型
    pub terminal_type: Option<String>,
    /// 终端信息
    pub terminal_info: Option<String>,
    /// 产品编码
    pub prod_code: Option<String>,
    /// 是否需要加密
    pub need_encrypt: bool,
    /// 参数
    pub udf_params: BTreeMap<String, String>,
    /// 业务实体
    pub biz_model: Option<T>
}

impl <T> AlipayTradeQueryRequest<T> where T: Serialize {
    pub fn new() -> Self {
        Self {
            api_version: "1.0".to_string(),
            notify_url: None,
            return_url: None,
            biz_content: None,
            terminal_type: None,
            terminal_info: None,
            prod_code: None,
            need_encrypt: false,
            udf_params: BTreeMap::new(),
            biz_model: None
        }
    }

    pub fn put_other_text_param(&mut self, key: String, value: String) {
        self.udf_params.insert(key, value);
    }
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



impl <T> AlipayRequest<T> for AlipayTradeQueryRequest<T> where T: Serialize {
    fn get_api_method_name(&self) -> AlipayMethod {
        AlipayMethod::QueryOrder
    }

    fn get_text_params(&self) -> BTreeMap<String, String> {
        let mut txt_params = BTreeMap::new();
        txt_params.insert(BIZ_CONTENT_KEY.to_string(), serde_json::to_string(&self.get_biz_model()).unwrap_or_default());
        if !self.udf_params.is_empty() {
            for (k, v) in &self.udf_params {
                txt_params.insert(k.to_string(), v.to_string());
            }
        }
        txt_params
    }

    fn get_api_version(&self) -> String {
        if self.api_version.is_empty() {
            "1.0".to_string()
        } else {
            self.api_version.to_string()
        }
    }

    fn get_terminal_type(&self) -> String {
        self.terminal_type.to_owned().unwrap_or_default()
    }

    fn get_terminal_info(&self) -> String {
        self.terminal_info.to_owned().unwrap_or_default()
    }

    fn get_prod_code(&self) -> String {
        self.prod_code.to_owned().unwrap_or_default()
    }

    fn get_notify_url(&self) -> String {
        self.notify_url.to_owned().unwrap_or_default()
    }

    fn get_return_url(&self) -> String {
        self.return_url.to_owned().unwrap_or_default()
    }

    fn is_need_encrypt(&self) -> bool {
        self.need_encrypt
    }

    fn get_biz_content(self) -> String {
        self.biz_content.to_owned().unwrap_or_default()
    }

    fn get_biz_model(&self) -> Option<&T> {
        self.biz_model.as_ref()
    }
}


//----------------------------------------------------------------------------------------------------------------------------




#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayTradeCloseRequest<T: Serialize> {
    /// API版本
    pub api_version: String,
    /// 回调地址
    pub notify_url: Option<String>,
    /// 跳转地址
    pub return_url: Option<String>,
    /// 业务内容
    pub biz_content: Option<String>,
    /// 终端类型
    pub terminal_type: Option<String>,
    /// 终端信息
    pub terminal_info: Option<String>,
    /// 产品编码
    pub prod_code: Option<String>,
    /// 是否需要加密
    pub need_encrypt: bool,
    /// 参数
    pub udf_params: BTreeMap<String, String>,
    /// 业务实体
    pub biz_model: Option<T>
}

impl <T> AlipayTradeCloseRequest<T> where T: Serialize {
    pub fn new() -> Self {
        Self {
            api_version: "1.0".to_string(),
            notify_url: None,
            return_url: None,
            biz_content: None,
            terminal_type: None,
            terminal_info: None,
            prod_code: None,
            need_encrypt: false,
            udf_params: BTreeMap::new(),
            biz_model: None
        }
    }

    pub fn put_other_text_param(&mut self, key: String, value: String) {
        self.udf_params.insert(key, value);
    }
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



impl <T> AlipayRequest<T> for AlipayTradeCloseRequest<T> where T: Serialize {
    fn get_api_method_name(&self) -> AlipayMethod {
        AlipayMethod::CloseOrder
    }

    fn get_text_params(&self) -> BTreeMap<String, String> {
        let mut txt_params = BTreeMap::new();
        txt_params.insert(BIZ_CONTENT_KEY.to_string(), serde_json::to_string(&self.get_biz_model()).unwrap_or_default());
        if !self.udf_params.is_empty() {
            for (k, v) in &self.udf_params {
                txt_params.insert(k.to_string(), v.to_string());
            }
        }
        txt_params
    }

    fn get_api_version(&self) -> String {
        if self.api_version.is_empty() {
            "1.0".to_string()
        } else {
            self.api_version.to_string()
        }
    }

    fn get_terminal_type(&self) -> String {
        self.terminal_type.to_owned().unwrap_or_default()
    }

    fn get_terminal_info(&self) -> String {
        self.terminal_info.to_owned().unwrap_or_default()
    }

    fn get_prod_code(&self) -> String {
        self.prod_code.to_owned().unwrap_or_default()
    }

    fn get_notify_url(&self) -> String {
        self.notify_url.to_owned().unwrap_or_default()
    }

    fn get_return_url(&self) -> String {
        self.return_url.to_owned().unwrap_or_default()
    }

    fn is_need_encrypt(&self) -> bool {
        self.need_encrypt
    }

    fn get_biz_content(self) -> String {
        self.biz_content.to_owned().unwrap_or_default()
    }

    fn get_biz_model(&self) -> Option<&T> {
        self.biz_model.as_ref()
    }
}


//----------------------------------------------------------------------------------------------------------------------------



#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayTradeRefundRequest<T: Serialize> {
    /// API版本
    pub api_version: String,
    /// 回调地址
    pub notify_url: Option<String>,
    /// 跳转地址
    pub return_url: Option<String>,
    /// 业务内容
    pub biz_content: Option<String>,
    /// 终端类型
    pub terminal_type: Option<String>,
    /// 终端信息
    pub terminal_info: Option<String>,
    /// 产品编码
    pub prod_code: Option<String>,
    /// 是否需要加密
    pub need_encrypt: bool,
    /// 参数
    pub udf_params: BTreeMap<String, String>,
    /// 业务实体
    pub biz_model: Option<T>
}

impl <T> AlipayTradeRefundRequest<T> where T: Serialize {
    pub fn new() -> Self {
        Self {
            api_version: "1.0".to_string(),
            notify_url: None,
            return_url: None,
            biz_content: None,
            terminal_type: None,
            terminal_info: None,
            prod_code: None,
            need_encrypt: false,
            udf_params: BTreeMap::new(),
            biz_model: None
        }
    }

    pub fn put_other_text_param(&mut self, key: String, value: String) {
        self.udf_params.insert(key, value);
    }
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


impl <T> AlipayRequest<T> for AlipayTradeRefundRequest<T> where T: Serialize {
    fn get_api_method_name(&self) -> AlipayMethod {
        AlipayMethod::Refund
    }

    fn get_text_params(&self) -> BTreeMap<String, String> {
        let mut txt_params = BTreeMap::new();
        txt_params.insert(BIZ_CONTENT_KEY.to_string(), serde_json::to_string(&self.get_biz_model()).unwrap_or_default());
        if !self.udf_params.is_empty() {
            for (k, v) in &self.udf_params {
                txt_params.insert(k.to_string(), v.to_string());
            }
        }
        txt_params
    }

    fn get_api_version(&self) -> String {
        if self.api_version.is_empty() {
            "1.0".to_string()
        } else {
            self.api_version.to_string()
        }
    }

    fn get_terminal_type(&self) -> String {
        self.terminal_type.to_owned().unwrap_or_default()
    }

    fn get_terminal_info(&self) -> String {
        self.terminal_info.to_owned().unwrap_or_default()
    }

    fn get_prod_code(&self) -> String {
        self.prod_code.to_owned().unwrap_or_default()
    }

    fn get_notify_url(&self) -> String {
        self.notify_url.to_owned().unwrap_or_default()
    }

    fn get_return_url(&self) -> String {
        self.return_url.to_owned().unwrap_or_default()
    }

    fn is_need_encrypt(&self) -> bool {
        self.need_encrypt
    }

    fn get_biz_content(self) -> String {
        self.biz_content.to_owned().unwrap_or_default()
    }

    fn get_biz_model(&self) -> Option<&T> {
        self.biz_model.as_ref()
    }
}

//----------------------------------------------------------------------------------------------------------------------------



#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayTradeFastpayRefundQueryRequest<T: Serialize> {
    /// API版本
    pub api_version: String,
    /// 回调地址
    pub notify_url: Option<String>,
    /// 跳转地址
    pub return_url: Option<String>,
    /// 业务内容
    pub biz_content: Option<String>,
    /// 终端类型
    pub terminal_type: Option<String>,
    /// 终端信息
    pub terminal_info: Option<String>,
    /// 产品编码
    pub prod_code: Option<String>,
    /// 是否需要加密
    pub need_encrypt: bool,
    /// 参数
    pub udf_params: BTreeMap<String, String>,
    /// 业务实体
    pub biz_model: Option<T>
}

impl <T> AlipayTradeFastpayRefundQueryRequest<T> where T: Serialize {
    pub fn new() -> Self {
        Self {
            api_version: "1.0".to_string(),
            notify_url: None,
            return_url: None,
            biz_content: None,
            terminal_type: None,
            terminal_info: None,
            prod_code: None,
            need_encrypt: false,
            udf_params: BTreeMap::new(),
            biz_model: None
        }
    }

    pub fn put_other_text_param(&mut self, key: String, value: String) {
        self.udf_params.insert(key, value);
    }
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



/// 统一收单交易退款查询
#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayTradeFastpayRefundQueryModel {
    /// 商户订单号。
    /// 订单支付时传入的商户订单号，商家自定义且保证商家系统中唯一。与支付宝交易号 trade_no 不能同时为空。
    pub out_trade_no: Option<String>,
    /// 支付宝交易号。
    /// 和商户订单号 out_trade_no 不能同时为空。
    pub trade_no: Option<String>,
    /// 退款请求号。
    /// <pre>
    /// 标识一次退款请求，需要保证在交易号下唯一，如需部分退款，则此参数必传。
    /// 注：针对同一次退款请求，如果调用接口失败或异常了，重试时需要保证退款请求号不能变更，防止该笔交易重复退款。支付宝会保证同样的退款请求号多次请求只会退一次。
    /// </pre>
    pub out_request_no: Option<String>,
    /// 查询选项。
    /// <pre>
    /// 商户通过上送该参数来定制同步需要额外返回的信息字段，数组格式。
    /// refund_detail_item_list：本次退款使用的资金渠道；
    /// gmt_refund_pay：退款执行成功的时间；
    /// deposit_back_info：银行卡冲退信息；
    /// </pre>
    pub query_options: Option<Vec<String>>,
}


impl <T> AlipayRequest<T> for AlipayTradeFastpayRefundQueryRequest<T> where T: Serialize {
    fn get_api_method_name(&self) -> AlipayMethod {
        AlipayMethod::QueryRefund
    }

    fn get_text_params(&self) -> BTreeMap<String, String> {
        let mut txt_params = BTreeMap::new();
        txt_params.insert(BIZ_CONTENT_KEY.to_string(), serde_json::to_string(&self.get_biz_model()).unwrap_or_default());
        if !self.udf_params.is_empty() {
            for (k, v) in &self.udf_params {
                txt_params.insert(k.to_string(), v.to_string());
            }
        }
        txt_params
    }

    fn get_api_version(&self) -> String {
        if self.api_version.is_empty() {
            "1.0".to_string()
        } else {
            self.api_version.to_string()
        }
    }

    fn get_terminal_type(&self) -> String {
        self.terminal_type.to_owned().unwrap_or_default()
    }

    fn get_terminal_info(&self) -> String {
        self.terminal_info.to_owned().unwrap_or_default()
    }

    fn get_prod_code(&self) -> String {
        self.prod_code.to_owned().unwrap_or_default()
    }

    fn get_notify_url(&self) -> String {
        self.notify_url.to_owned().unwrap_or_default()
    }

    fn get_return_url(&self) -> String {
        self.return_url.to_owned().unwrap_or_default()
    }

    fn is_need_encrypt(&self) -> bool {
        self.need_encrypt
    }

    fn get_biz_content(self) -> String {
        self.biz_content.to_owned().unwrap_or_default()
    }

    fn get_biz_model(&self) -> Option<&T> {
        self.biz_model.as_ref()
    }
}


//----------------------------------------------------------------------------------------------------------------------------


#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayTradeCancelRequest<T: Serialize> {
    /// API版本
    pub api_version: String,
    /// 回调地址
    pub notify_url: Option<String>,
    /// 跳转地址
    pub return_url: Option<String>,
    /// 业务内容
    pub biz_content: Option<String>,
    /// 终端类型
    pub terminal_type: Option<String>,
    /// 终端信息
    pub terminal_info: Option<String>,
    /// 产品编码
    pub prod_code: Option<String>,
    /// 是否需要加密
    pub need_encrypt: bool,
    /// 参数
    pub udf_params: BTreeMap<String, String>,
    /// 业务实体
    pub biz_model: Option<T>
}

impl <T> AlipayTradeCancelRequest<T> where T: Serialize {
    pub fn new() -> Self {
        Self {
            api_version: "1.0".to_string(),
            notify_url: None,
            return_url: None,
            biz_content: None,
            terminal_type: None,
            terminal_info: None,
            prod_code: None,
            need_encrypt: false,
            udf_params: BTreeMap::new(),
            biz_model: None
        }
    }

    pub fn put_other_text_param(&mut self, key: String, value: String) {
        self.udf_params.insert(key, value);
    }
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


impl <T> AlipayRequest<T> for AlipayTradeCancelRequest<T> where T: Serialize {
    fn get_api_method_name(&self) -> AlipayMethod {
        AlipayMethod::CancelOrder
    }

    fn get_text_params(&self) -> BTreeMap<String, String> {
        let mut txt_params = BTreeMap::new();
        txt_params.insert(BIZ_CONTENT_KEY.to_string(), serde_json::to_string(&self.get_biz_model()).unwrap_or_default());
        if !self.udf_params.is_empty() {
            for (k, v) in &self.udf_params {
                txt_params.insert(k.to_string(), v.to_string());
            }
        }
        txt_params
    }

    fn get_api_version(&self) -> String {
        if self.api_version.is_empty() {
            "1.0".to_string()
        } else {
            if self.api_version.is_empty() {
            "1.0".to_string()
        } else {
            self.api_version.to_string()
        }
        }
    }

    fn get_terminal_type(&self) -> String {
        self.terminal_type.to_owned().unwrap_or_default()
    }

    fn get_terminal_info(&self) -> String {
        self.terminal_info.to_owned().unwrap_or_default()
    }

    fn get_prod_code(&self) -> String {
        self.prod_code.to_owned().unwrap_or_default()
    }

    fn get_notify_url(&self) -> String {
        self.notify_url.to_owned().unwrap_or_default()
    }

    fn get_return_url(&self) -> String {
        self.return_url.to_owned().unwrap_or_default()
    }

    fn is_need_encrypt(&self) -> bool {
        self.need_encrypt
    }

    fn get_biz_content(self) -> String {
        self.biz_content.to_owned().unwrap_or_default()
    }

    fn get_biz_model(&self) -> Option<&T> {
        self.biz_model.as_ref()
    }
}


//----------------------------------------------------------------------------------------------------------------------------



/// 获取用户认证token
#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipaySystemOauthTokenRequest {
    /// API版本
    pub api_version: String,
    /// 回调地址
    pub notify_url: Option<String>,
    /// 跳转地址
    pub return_url: Option<String>,
    /// 业务内容
    pub biz_content: Option<String>,
    /// 终端类型
    pub terminal_type: Option<String>,
    /// 终端信息
    pub terminal_info: Option<String>,
    /// 产品编码
    pub prod_code: Option<String>,
    /// 是否需要加密
    pub need_encrypt: bool,
    /// 参数
    pub udf_params: BTreeMap<String, String>,
    /// 授权方式。支持：
    /// <pre>
    /// 1.authorization_code，表示换取使用用户授权码code换取授权令牌access_token。
    /// 2.refresh_token，表示使用refresh_token刷新获取新授权令牌。
    /// </pre>
    pub grant_type: String,
    /// 授权码，用户对应用授权后得到。
    /// 本参数在 grant_type 为 authorization_code 时必填；为 refresh_token 时不填。
    pub code: Option<String>,
    /// 刷新令牌，上次换取访问令牌时得到。
    /// 本参数在 grant_type 为 authorization_code 时不填；
    /// 为 refresh_token 时必填，且该值来源于此接口的返回值 app_refresh_token（即至少需要通过 grant_type=authorization_code 调用此接口一次才能获取）。
    pub refresh_token: Option<String>,
}

impl AlipaySystemOauthTokenRequest {
    pub fn new() -> Self {
        Self {
            api_version: "1.0".to_string(),
            notify_url: None,
            return_url: None,
            biz_content: None,
            terminal_type: None,
            terminal_info: None,
            prod_code: None,
            need_encrypt: false,
            udf_params: BTreeMap::new(),
            grant_type: "".to_string(),
            code: None,
            refresh_token: None
        }
    }

    pub fn put_other_text_param(&mut self, key: String, value: String) {
        self.udf_params.insert(key, value);
    }
}

impl <T> AlipayRequest<T> for AlipaySystemOauthTokenRequest where T: Serialize {
    fn get_api_method_name(&self) -> AlipayMethod {
        AlipayMethod::SystemOauthToken
    }

    fn get_text_params(&self) -> BTreeMap<String, String> {
        let mut txt_params = BTreeMap::new();
        txt_params.insert("grant_type".to_string(), self.grant_type.to_string());
        if let Some(code) = &self.code {
            txt_params.insert("code".to_string(), code.to_string());
        }
        if let Some(refresh_token) = &self.refresh_token {
            txt_params.insert("refresh_token".to_string(), refresh_token.to_string());
        }
        if !self.udf_params.is_empty() {
            for (k, v) in &self.udf_params {
                txt_params.insert(k.to_string(), v.to_string());
            }
        }
        txt_params
    }

    fn get_api_version(&self) -> String {
        if self.api_version.is_empty() {
            "1.0".to_string()
        } else {
            self.api_version.to_string()
        }
    }

    fn get_terminal_type(&self) -> String {
        self.terminal_type.to_owned().unwrap_or_default()
    }

    fn get_terminal_info(&self) -> String {
        self.terminal_info.to_owned().unwrap_or_default()
    }

    fn get_prod_code(&self) -> String {
        self.prod_code.to_owned().unwrap_or_default()
    }

    fn get_notify_url(&self) -> String {
        self.notify_url.to_owned().unwrap_or_default()
    }

    fn get_return_url(&self) -> String {
        self.return_url.to_owned().unwrap_or_default()
    }

    fn is_need_encrypt(&self) -> bool {
        self.need_encrypt
    }

    fn get_biz_content(self) -> String {
        self.biz_content.to_owned().unwrap_or_default()
    }
}

//----------------------------------------------------------------------------------------------------------------------------

//----------------------------------------------------------------------------------------------------------------------------



/// 获取用户认证token
#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayOpenAuthTokenAppRequest<T: Serialize> {
    /// API版本
    pub api_version: String,
    /// 回调地址
    pub notify_url: Option<String>,
    /// 跳转地址
    pub return_url: Option<String>,
    /// 业务内容
    pub biz_content: Option<String>,
    /// 终端类型
    pub terminal_type: Option<String>,
    /// 终端信息
    pub terminal_info: Option<String>,
    /// 产品编码
    pub prod_code: Option<String>,
    /// 是否需要加密
    pub need_encrypt: bool,
    /// 参数
    pub udf_params: BTreeMap<String, String>,
    /// 业务实体
    pub biz_model: Option<T>
}


impl <T> AlipayOpenAuthTokenAppRequest<T> where T: Serialize {
    pub fn new() -> Self {
        Self {
            api_version: "1.0".to_string(),
            notify_url: None,
            return_url: None,
            biz_content: None,
            terminal_type: None,
            terminal_info: None,
            prod_code: None,
            need_encrypt: false,
            udf_params: BTreeMap::new(),
            biz_model: None,
        }
    }

    pub fn put_other_text_param(&mut self, key: String, value: String) {
        self.udf_params.insert(key, value);
    }
}



#[derive(Debug, Serialize, Default, Deserialize)]
pub struct AlipayOpenAuthTokenAppModel {
    /// 授权方式。支持：
    /// <pre>
    /// authorization_code：使用应用授权码换取应用授权令牌app_auth_token。
    /// refresh_token：使用app_refresh_token刷新获取新授权令牌。
    /// </pre>
    pub grant_type: String,
    /// 应用授权码，传入应用授权后得到的 app_auth_code。
    /// 说明：
    /// <pre>
    /// grant_type 为 authorization_code 时，本参数必填；
    /// grant_type 为 refresh_token 时不填。
    /// </pre>
    pub code: Option<String>,
    /// 刷新令牌，上次换取访问令牌时得到。
    /// 本参数在 grant_type 为 authorization_code 时不填；
    /// 为 refresh_token 时必填，且该值来源于此接口的返回值 app_refresh_token（即至少需要通过 grant_type=authorization_code 调用此接口一次才能获取）。
    pub refresh_token: Option<String>,
}

impl <T> AlipayRequest<T> for AlipayOpenAuthTokenAppRequest<T> where T: Serialize {
    fn get_api_method_name(&self) -> AlipayMethod {
        AlipayMethod::OpenAuthTokenApp
    }

    fn get_text_params(&self) -> BTreeMap<String, String> {
        let mut txt_params = BTreeMap::new();
        txt_params.insert(BIZ_CONTENT_KEY.to_string(), serde_json::to_string(&self.get_biz_model()).unwrap_or_default());
        if !self.udf_params.is_empty() {
            for (k, v) in &self.udf_params {
                txt_params.insert(k.to_string(), v.to_string());
            }
        }
        txt_params
    }

    fn get_api_version(&self) -> String {
        if self.api_version.is_empty() {
            "1.0".to_string()
        } else {
            self.api_version.to_string()
        }
    }

    fn get_terminal_type(&self) -> String {
        self.terminal_type.to_owned().unwrap_or_default()
    }

    fn get_terminal_info(&self) -> String {
        self.terminal_info.to_owned().unwrap_or_default()
    }

    fn get_prod_code(&self) -> String {
        self.prod_code.to_owned().unwrap_or_default()
    }

    fn get_notify_url(&self) -> String {
        self.notify_url.to_owned().unwrap_or_default()
    }

    fn get_return_url(&self) -> String {
        self.return_url.to_owned().unwrap_or_default()
    }

    fn is_need_encrypt(&self) -> bool {
        self.need_encrypt
    }

    fn get_biz_content(self) -> String {
        self.biz_content.to_owned().unwrap_or_default()
    }

    fn get_biz_model(&self) -> Option<&T> {
        self.biz_model.as_ref()
    }
}

//----------------------------------------------------------------------------------------------------------------------------
