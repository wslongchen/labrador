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

//! 微信支付实现

pub mod types;
pub mod builder;
pub mod config;

use super::constants;
use crate::utils::xml::XmlSerializer;
use crate::{errors::{LabraError, LabradorResult}, request::{HttpMethod, Request, RequestBody}, AesEncryptor, AesMode, ApiClient, ClientBuilder};
use serde::{Deserialize};
use std::collections::{BTreeMap};
use std::fs;
use base64::Engine;
use base64::engine::general_purpose;
use bytes::Bytes;
use serde_json::{json, Value};
use crate::client::certificate::Certificate;
use crate::client::identity::Identity;
use crate::platforms::wechat::pay::config::WechatPayConfig;
use crate::platforms::wechat::pay::types::{BillType, CodepayOrderRequestV3, CombineCloseOrderRequestV3, CombineOrderQueryRequestV3, CombineOrderRequestV3, CombineOrderResponseV3, OrderQueryRequest, OrderQueryResponse, PlatformCertificateResponse, RefundQueryRequest, RefundQueryResponse, RefundRequest, RefundResponse, TradeType, UnifiedOrderRequest, UnifiedOrderRequestV3, UnifiedOrderResponse, WechatEncryptResponseV3, WechatPayCommonResponse, WechatPayResponseV3};
use crate::utils::string::random_string;
use crate::wechat::pay::types::{AccountType, DecryptNotifyResult, DecryptRefundNotifyResult, FundFlowBillResponseV3, OrderQueryRequestV3, OrderQueryResponseV3, OrderReverseRequestV3, OriginNotifyResponse, RefundQueryResponseV3, RefundRequestV3, RefundResponseV3, TradeBillRequestV3, TradeBillResponseV3, WechatDecryptRefundNotifyResponse, WechatEncryptResponse, WechatOrderReverseRequest, WechatOrderReverseResponse, WechatPayNotifyResponse, WechatPayShortUrlResponse, WechatPayShorturlRequest, WechatScanPayNotifyResponse, WechatSignatureHeader};
use crate::wechat::signer::WechatPaySigner;

/// 微信支付客户端
pub struct WechatPayClient {
    /// HTTP客户端
    http_client: ApiClient,
    /// 配置
    config: WechatPayConfig,
}


impl WechatPayClient {
    
    /// 创建新的微信支付客户端
    pub fn new(config: WechatPayConfig) -> LabradorResult<Self> {
        let http_client = Self::build_client(&config)?;
        Ok(Self {
            http_client,
            config,
        })
    }
    
    fn build_client(config: &WechatPayConfig) -> LabradorResult<ApiClient> {
        let base_url = if config.sandbox {
            constants::PAY_SANDBOX_API_BASE_URL
        } else {
            constants::PAY_API_BASE_URL
        };

        let mut client_builder = ClientBuilder::new()
            .api_base_url(base_url)
            .timeout(std::time::Duration::from_secs(30))
            .connect_timeout(std::time::Duration::from_secs(10));
        let mut signer = WechatPaySigner::from_config(&config);
        // 处理证书配置
        if let Some(identity) = Self::load_identity(&config)? {
            // 添加私钥
            if let Some(private_key) = identity.private_key_pem() {
                signer = signer.with_private_key(private_key);
            }
            if let Some(serial_no) = identity.serial_number() {
                signer = signer.with_serial_no(serial_no);
            }
            client_builder = client_builder.identity(identity);
        }

        if let Some(root_certs) = config.root_certificates.clone() {
            for root_cert in root_certs.into_iter() {
                client_builder = client_builder.add_root_certificate(root_cert);
            }
        }

        let mut http_client = client_builder.build()?;

        http_client.set_signer(signer);
        Ok(http_client)
    } 

    /// 自动加载证书
    pub async fn get_certificates(config: &WechatPayConfig) -> LabradorResult<Vec<Certificate>> {
        let http_client = Self::build_client(config)?;
        let request = Request::builder()
            .method(HttpMethod::Get)
            .path("/v3/certificates")
            .build();
        let response = http_client.request(request).await?;
        let mut wechat_certs = Vec::new();
        if response.is_success() {
            let certs_response = response.json::<PlatformCertificateResponse>()?;
            tracing::info!("获取平台证书:{}", serde_json::to_string(&certs_response).unwrap_or_default());
            if let Some(certs) = certs_response.data {
                for cert in certs.into_iter() {
                    let data = cert.encrypt_certificate;
                    let res = WechatPayClient::decrypt_data_v3(&config.api_key_v3.clone().unwrap_or_default(), data)?;
                    let cert = Certificate::from_pem(&res)?;
                    wechat_certs.push(cert);
                }
            }
        }
        Ok(wechat_certs)
    }
    
    fn decrypt_data_v3(api_key_v3: &str, data: WechatEncryptResponseV3) -> LabradorResult<Vec<u8>> {
        let key = api_key_v3.as_bytes();
        let associated_data = data.associated_data.to_owned().unwrap_or_default();
        let nonce = data.nonce.to_owned();
        let ciphertext = data.ciphertext.to_owned().unwrap_or_default();
        let cipher_text = general_purpose::STANDARD
            .decode(ciphertext)?;
        let base64_cipher = hex::encode(cipher_text);
        let cipher_text = hex::decode(base64_cipher)?;
        let aad= associated_data.as_bytes();
        let nonce = nonce.as_bytes();
        let ciphertext_length = cipher_text.len() - 16;
        let ciphertext_bytes = &cipher_text[0..ciphertext_length];
        let tag = &cipher_text[ciphertext_length..cipher_text.len()];
        let encryptor = AesEncryptor::new(AesMode::Gcm, key,  nonce)?;
        let decrypted_data = encryptor.gcm_decrypt(nonce, aad, ciphertext_bytes, tag)?;
        Ok(decrypted_data)
    }

    fn decrypt_data(api_key: &str, data: WechatEncryptResponse) -> LabradorResult<Vec<u8>> {
        let key = api_key.as_bytes();
        let nonce = data.nonce_str.to_owned().unwrap_or_default();
        let ciphertext = data.req_info;
        let cipher_text = general_purpose::STANDARD
            .decode(ciphertext)?;
        let base64_cipher = hex::encode(cipher_text);
        let cipher_text = hex::decode(base64_cipher)?;
        let nonce = nonce.as_bytes();
        let ciphertext_length = cipher_text.len() - 16;
        let ciphertext_bytes = &cipher_text[0..ciphertext_length];
        let encryptor = AesEncryptor::new(AesMode::Ecb, key,  nonce)?;
        let decrypted_data = encryptor.decrypt(ciphertext_bytes)?;
        Ok(decrypted_data)
    }


    /// 加载证书身份信息
    fn load_identity(config: &WechatPayConfig) -> LabradorResult<Option<Identity>> {
        // 情况1：同时有 cert_path 和 key_path（PEM格式）
        if let (Some(cert_path), Some(key_path)) = (&config.cert_path, &config.key_path) {
            let cert_data = fs::read(cert_path)
                .map_err(|e| LabraError::Certificate(format!("读取证书失败: {}", e)))?;
            let key_data = fs::read(key_path)
                .map_err(|e| LabraError::Certificate(format!("读取密钥失败: {}", e)))?;

            let mut identity_data = cert_data;
            identity_data.extend_from_slice(&key_data);

            let identity = Identity::from_pem(&identity_data)
                .map_err(|e| LabraError::Certificate(format!("创建PEM身份失败: {}", e)))?;

            return Ok(Some(identity));
        }

        // 情况2：有 p12 文件（PKCS#12格式）
        if let Some(p12_path) = &config.p12_path {
            let p12_data = fs::read(p12_path)
                .map_err(|e| LabraError::Certificate(format!("读取P12文件失败: {}", e)))?;

            // 获取P12密码，默认为商户号
            let password = config.p12_password.as_deref()
                .unwrap_or(&config.mch_id);

            let identity = Identity::from_pkcs12(&p12_data, password)
                .map_err(|e| LabraError::Certificate(format!("解析P12文件失败: {}", e)))?;
            
            return Ok(Some(identity));
        }

        Ok(None)
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
    ///#[tokio::main]
    ///async fn main() {
    ///     use labrador::wechat::pay::builder::WechatPayBuilder;
    ///     use labrador::wechat::pay::types::{TradeType, UnifiedOrderRequest, UnifiedOrderResponse};
    ///
    ///     let client = WechatPayBuilder::new("appid", "mchid", "secret", "http://api.woofcloud.com/callback")
    ///         .p12_path("/apiclient_cert.p12", None)
    ///         .api_key("apikey")
    ///         .build().await.unwrap();
    ///     let request = UnifiedOrderRequest::new(
    ///         "测试商品",
    ///         "16029202235sdfsdfas32234234",
    ///         1,
    ///         "ip",
    ///         "test",
    ///         TradeType::Jsapi);
    ///     let response: UnifiedOrderResponse = client.unified_order(request).await.unwrap();
    /// }
    /// ```
    ///
    pub async fn unified_order(&self, request: UnifiedOrderRequest) -> LabradorResult<UnifiedOrderResponse> {
        let xml = request.to_xml(&self.config)?;
        let req_path = if request.trade_type == TradeType::Micropay {
            "/pay/micropay"
        } else {
            "/pay/unifiedorder"
        };
        let http_request = Request::builder()
            .method(HttpMethod::Post)
            .path(req_path)
            .body(RequestBody::xml(xml))
            .build();

        let response = self.http_client.request(http_request).await?;
        let response_text = response.text()?;

        let result = UnifiedOrderResponse::from_xml(&response_text)?;

        if !result.is_success() {
            return Err(LabraError::business(
                result.err_code.unwrap_or_default(),
                result.err_code_des.unwrap_or_default(),
            ));
        } else {
            // 验证签名
            if !result.verify_signature(&self.config.api_key.clone().unwrap_or_default())? {
                return Err(LabraError::Sign("响应签名验证失败".to_string()));
            }
        }

        Ok(result)
    }

    ///
    /// # 统一下单 - V3版本
    /// <pre>
    /// 详见:[文档](https://pay.weixin.qq.com/docs/merchant/apis/jsapi-payment/direct-jsons/jsapi-prepay.html)
    ///
    /// 在发起微信支付前，需要调用统一下单接口，获取"预支付交易会话标识"
    /// 接口地址：POST /v3/pay/transactions/{jsapi|app|h5|native}
    /// </pre>
    /// # 示例
    ///
    /// ```no_run
    ///
    /// async fn main() {
    ///     use labrador::wechat::pay::builder::WechatPayBuilder;
    ///     use labrador::wechat::pay::types::{Amount, Detail, GoodsDetail, Payer, TradeType, UnifiedOrderRequestV3, WechatPayResponseV3};
    ///     let client = WechatPayBuilder::new("appid", "mchid", "secret", "http://api.woofcloud.com/callback")
    ///         .p12_path("/apiclient_cert.p12", None)
    ///         .api_key_v3("apikey")
    ///         .build().await.unwrap();
    ///     let mut request = UnifiedOrderRequestV3::new(
    ///             TradeType::Jsapi,
    ///             "16029202235sdfsdfas32234234",
    ///             "测试商品",
    ///             Amount::new(1),
    ///             Detail::new(vec![GoodsDetail::new("1001".to_string(), "测试商品".to_string(), 1, 1)]),
    ///             "https://api.woofcloud.com/shop/callback",
    ///         );
    ///         request.payer(Payer::new("oY"));
    ///         let response: WechatPayResponseV3 = client.unified_order_v3(request).await.unwrap();
    ///
    /// }
    //// ```
    ///
    pub async fn unified_order_v3(&self, mut request: UnifiedOrderRequestV3) -> LabradorResult<WechatPayResponseV3> {
        request.mch_id(&self.config.mch_id);
        request.appid(&self.config.app_id);
        let req_path = request.trade_type.req_path_v3();
        let http_request = Request::builder()
            .method(HttpMethod::Post)
            .path(req_path)
            .body(request)
            .build();

        let response = self.http_client.request(http_request).await?;
        // 验证签名
        if let Some(signer) = self.http_client.signer() {
            if !signer.verify_signature(&response)? {
                return Err(LabraError::Sign("响应签名验证失败".to_string()));
            }
        }
        let result = response.json::<WechatPayCommonResponse<WechatPayResponseV3>>()?;

        if !result.is_success() {
            return Err(LabraError::business(
                result.code.unwrap_or_default(),
                result.message.unwrap_or_default(),
            ));
        }

        Ok(result.data)
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
    ///async fn main() {
    ///     use labrador::wechat::pay::builder::WechatPayBuilder;
    ///     use labrador::wechat::pay::types::{OrderQueryRequest, OrderQueryResponse};
    ///     let client = WechatPayBuilder::new("appid", "mchid", "secret", "http://api.woofcloud.com/callback")
    ///         .p12_path("/apiclient_cert.p12", None)
    ///         .api_key("apikey")
    ///         .build().await.unwrap();
    ///     let request = OrderQueryRequest::by_out_trade_no("out_trade_no");
    ///     let response: OrderQueryResponse = client.order_query(&request).await.unwrap();
    /// }
    ///
    /// ```
    ///
    pub async fn order_query(&self, request: &OrderQueryRequest) -> LabradorResult<OrderQueryResponse> {
        let xml = request.to_xml(&self.config)?;

        let http_request = Request::builder()
            .method(HttpMethod::Post)
            .path("/pay/orderquery")
            .body(RequestBody::xml(xml))
            .build();

        let response = self.http_client.request(http_request).await?;
        let response_text = response.text()?;

        let result = OrderQueryResponse::from_xml(&response_text)?;

        if !result.is_success() {
            return Err(LabraError::business(
                result.err_code.unwrap_or_default(),
                result.err_code_des.unwrap_or_default(),
            ));
        } else {
            // 验证签名
            if !result.verify_signature(&self.config.api_key.clone().unwrap_or_default())? {
                return Err(LabraError::Sign("响应签名验证失败".to_string()));
            }
        }

        Ok(result)
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
    /// async fn main() {
    ///     use labrador::wechat::pay::builder::WechatPayBuilder;
    ///     use labrador::wechat::pay::types::{OrderQueryRequestV3, OrderQueryResponseV3};
    ///     let client = WechatPayBuilder::new("appid", "mchid", "secret", "http://api.woofcloud.com/callback")
    ///         .p12_path("/apiclient_cert.p12", None)
    ///         .api_key("apikey")
    ///         .build().await.unwrap();
    ///     let request = OrderQueryRequestV3::by_out_trade_no("P20260111202648d99df5");
    ///     let response: OrderQueryResponseV3 = client.order_query_v3(request).await.unwrap();
    ///
    /// }
    ///
    /// ```
    ///
    pub async fn order_query_v3(&self, request: OrderQueryRequestV3) -> LabradorResult<OrderQueryResponseV3> {
        let http_request = Request::builder()
            .method(HttpMethod::Get)
            .query_param("mchid", &self.config.mch_id)
            .path(request.req_path())
            .build();
        let response = self.http_client.request(http_request).await?;
        // 验证签名
        if let Some(signer) = self.http_client.signer() {
            if !signer.verify_signature(&response)? {
                return Err(LabraError::Sign("响应签名验证失败".to_string()));
            }
        }
        let result = response.json::<WechatPayCommonResponse<OrderQueryResponseV3>>()?;

        if !result.is_success() {
            return Err(LabraError::business(
                result.code.unwrap_or_default(),
                result.message.unwrap_or_default(),
            ));
        }

        Ok(result.data)
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
    /// # async fn main() {
    ///     use labrador::wechat::pay::builder::WechatPayBuilder;
    ///     use labrador::wechat::pay::types::{OrderQueryRequestV3, OrderQueryResponseV3};
    ///     let client = WechatPayBuilder::new("appid", "mchid", "secret", "http://api.woofcloud.com/callback")
    ///         .p12_path("/apiclient_cert.p12", None)
    ///         .api_key("apikey")
    ///         .build().await.unwrap();
    ///     match client.close_order("").await {
    ///         Ok(res) => {}
    ///         Err(err) => {}
    ///     }
    /// }
    ///
    /// ```
    ///
    pub async fn close_order(&self, out_trade_no: &str) -> LabradorResult<()> {
        let mut params = BTreeMap::new();
        params.insert("appid".to_string(), self.config.app_id.clone());
        params.insert("mch_id".to_string(), self.config.mch_id.clone());
        params.insert("out_trade_no".to_string(), out_trade_no.to_string());
        params.insert("nonce_str".to_string(), random_string(32));

        let sign = UnifiedOrderRequest::generate_sign(&params, &self.config.api_key.clone().unwrap_or_default())?;
        params.insert("sign".to_string(), sign);

        let xml = UnifiedOrderRequest::map_to_xml(&params)?;

        let http_request = Request::builder()
            .method(HttpMethod::Post)
            .path("/pay/closeorder")
            .body(RequestBody::xml(xml))
            .build();

        let response = self.http_client.request(http_request).await?;
        let response_text = response.text()?;

        #[derive(Debug, Deserialize)]
        struct CloseOrderResponse {
            return_code: String,
            return_msg: String,
            result_code: Option<String>,
            err_code: Option<String>,
            err_code_des: Option<String>,
        }

        let result: CloseOrderResponse = XmlSerializer::deserialize(&response_text)?;

        if result.return_code != "SUCCESS" || result.result_code.as_deref() != Some("SUCCESS") {
            return Err(LabraError::business(
                result.err_code.unwrap_or_default(),
                result.err_code_des.unwrap_or(result.return_msg),
            ));
        }

        Ok(())
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
    /// async fn main() {
    ///     use labrador::wechat::pay::builder::WechatPayBuilder;
    ///     use labrador::wechat::pay::types::{OrderQueryRequestV3, OrderQueryResponseV3};
    ///     let client = WechatPayBuilder::new("appid", "mchid", "secret", "http://api.woofcloud.com/callback")
    ///         .p12_path("/apiclient_cert.p12", None)
    ///         .api_key("apikey")
    ///         .build().await.unwrap();
    ///     let response = client.close_order_v3("P2025110922333212aee8").await.unwrap();
    ///
    /// }
    ///
    /// ```
    ///
    pub async fn close_order_v3(&self, out_trade_no: &str) -> LabradorResult<()> {
        let http_request = Request::builder()
            .method(HttpMethod::Post)
            .path(format!("/v3/pay/transactions/out-trade-no/{}/close", out_trade_no))
            .body(json!({
                "mchid": &self.config.mch_id
            }));
        let response = self.http_client.request(http_request).await?;
        // 验证签名
        if let Some(signer) = self.http_client.signer() {
            if !signer.verify_signature(&response)? {
                return Err(LabraError::Sign("响应签名验证失败".to_string()));
            }
        }
        let result = response.json::<WechatPayCommonResponse<Value>>()?;
        if !result.is_success() {
            return Err(LabraError::business(
                result.code.unwrap_or_default(),
                result.message.unwrap_or_default(),
            ));
        }

        Ok(())

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
        request: &RefundRequest
    ) -> LabradorResult<RefundResponse> {
        let mut params = BTreeMap::new();
        params.insert("appid".to_string(), self.config.app_id.clone());
        params.insert("mch_id".to_string(), self.config.mch_id.clone());
        params.insert("nonce_str".to_string(), random_string(32));
        params.insert("out_trade_no".to_string(), request.out_trade_no.to_string());
        params.insert("out_refund_no".to_string(), request.out_refund_no.to_string());
        params.insert("total_fee".to_string(), request.total_fee.to_string());
        params.insert("refund_fee".to_string(), request.refund_fee.to_string());

        if let Some(desc) = request.refund_desc.as_ref() {
            params.insert("refund_desc".to_string(), desc.to_string());
        }

        if let Some(ref url) = self.config.refund_notify_url {
            params.insert("notify_url".to_string(), url.clone());
        }

        let sign = UnifiedOrderRequest::generate_sign(&params, &self.config.api_key.clone().unwrap_or_default())?;
        params.insert("sign".to_string(), sign);

        let xml = UnifiedOrderRequest::map_to_xml(&params)?;

        let http_request = Request::builder()
            .method(HttpMethod::Post)
            .path("/pay/refund")
            .body(RequestBody::xml(xml))
            .build();

        let response = self.http_client.request(http_request).await?;
        let response_text = response.text()?;

        let result = RefundResponse::from_xml(&response_text)?;

        if !result.is_success() {
            return Err(LabraError::business(
                result.err_code.unwrap_or_default(),
                result.err_code_des.unwrap_or_default(),
            ));
        } else {
            // 验证签名
            if !result.verify_signature(&self.config.api_key.clone().unwrap_or_default())? {
                return Err(LabraError::Sign("响应签名验证失败".to_string()));
            }
        }

        Ok(result)
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
        request: RefundRequestV3
    ) -> LabradorResult<RefundResponseV3> {
        let http_request = Request::builder()
            .method(HttpMethod::Post)
            .path("/v3/refund/domestic/refunds")
            .body(request)
            .build();

        let response = self.http_client.request(http_request).await?;
        // 验证签名
        if let Some(signer) = self.http_client.signer() {
            if !signer.verify_signature(&response)? {
                return Err(LabraError::Sign("响应签名验证失败".to_string()));
            }
        }
        let result = response.json::<WechatPayCommonResponse<RefundResponseV3>>()?;

        if !result.is_success() {
            return Err(LabraError::business(
                result.code.unwrap_or_default(),
                result.message.unwrap_or_default(),
            ));
        }

        Ok(result.data)
    }

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
    pub async fn refund_query(
        &self,
        request: &RefundQueryRequest,
    ) -> LabradorResult<RefundQueryResponse> {
        let mut params = BTreeMap::new();
        params.insert("appid".to_string(), self.config.app_id.clone());
        params.insert("mch_id".to_string(), self.config.mch_id.clone());
        params.insert("nonce_str".to_string(), random_string(32));

        if let Some(id) = request.transaction_id.as_ref() {
            params.insert("transaction_id".to_string(), id.to_string());
        }
        if let Some(no) = request.out_trade_no.as_ref() {
            params.insert("out_trade_no".to_string(), no.to_string());
        }
        if let Some(no) = request.out_refund_no.as_ref() {
            params.insert("out_refund_no".to_string(), no.to_string());
        }
        if let Some(id) = request.refund_id.as_ref() {
            params.insert("refund_id".to_string(), id.to_string());
        }

        let sign = UnifiedOrderRequest::generate_sign(&params, &self.config.api_key.clone().unwrap_or_default())?;
        params.insert("sign".to_string(), sign);

        let xml = UnifiedOrderRequest::map_to_xml(&params)?;

        let http_request = Request::builder()
            .method(HttpMethod::Post)
            .path("/pay/refundquery")
            .body(RequestBody::xml(xml))
            .build();

        let response = self.http_client.request(http_request).await?;
        let response_text = response.text()?;

        let result = RefundQueryResponse::from_xml(&response_text)?;

        if !result.is_success() {
            return Err(LabraError::business(
                result.err_code.unwrap_or_default(),
                result.err_code_des.unwrap_or_default(),
            ));
        } else {
            // 验证签名
            if !result.verify_signature(&self.config.api_key.clone().unwrap_or_default())? {
                return Err(LabraError::Sign("响应签名验证失败".to_string()));
            }
        }

        Ok(result)
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
    pub async fn refund_query_v3(
        &self,
        out_refund_no: &str
    ) -> LabradorResult<RefundQueryResponseV3> {
        let http_request = Request::builder()
            .method(HttpMethod::Get)
            .path(format!("/v3/refund/domestic/refunds/{}", out_refund_no))
            .build();

        let response = self.http_client.request(http_request).await?;
        // 验证签名
        if let Some(signer) = self.http_client.signer() {
            if !signer.verify_signature(&response)? {
                return Err(LabraError::Sign("响应签名验证失败".to_string()));
            }
        }
        let result = response.json::<WechatPayCommonResponse<RefundQueryResponseV3>>()?;

        if !result.is_success() {
            return Err(LabraError::business(
                result.code.unwrap_or_default(),
                result.message.unwrap_or_default(),
            ));
        }

        Ok(result.data)
    }



    /// 下载交易账单
    pub async fn download_bill(
        &self,
        bill_date: &str, // 格式：20140603
        bill_type: BillType,
    ) -> LabradorResult<String> {
        let mut params = BTreeMap::new();
        params.insert("appid".to_string(), self.config.app_id.clone());
        params.insert("mch_id".to_string(), self.config.mch_id.clone());
        params.insert("nonce_str".to_string(), random_string(32));
        params.insert("bill_date".to_string(), bill_date.to_string());
        params.insert("bill_type".to_string(), bill_type.as_str().to_string());

        let sign = UnifiedOrderRequest::generate_sign(&params, &self.config.api_key.clone().unwrap_or_default())?;
        params.insert("sign".to_string(), sign);

        let xml = UnifiedOrderRequest::map_to_xml(&params)?;

        let http_request = Request::builder()
            .method(HttpMethod::Post)
            .path("/pay/downloadbill")
            .body(RequestBody::xml(xml))
            .build();

        let response = self.http_client.request(http_request).await?;
        let response_text = response.text()?;

        // 检查是否是错误响应
        if response_text.starts_with("<xml>") {
            #[derive(Debug, Deserialize)]
            struct ErrorResponse {
                return_code: String,
                return_msg: String,
            }

            let error: ErrorResponse = XmlSerializer::deserialize(&response_text)?;
            return Err(LabraError::business(
                error.return_code,
                error.return_msg,
            ));
        }

        Ok(response_text)
    }

    /// # 申请资金账单
    /// 下载接口说明
    /// 微信支付按天提供商户各账户的资金流水账单文件，商户可以通过该接口获取账单文件的下载地址。账单文件详细记录了账户资金操作的相关信息，包括业务单号、收支金额及记账时间等，以便商户进行核对与确认。详细介绍参考：下载账单-产品介绍。
    ///
    /// 注意：
    ///
    /// 资金账单中的数据反映的是商户微信账户资金变动情况；
    ///
    /// 当日账单将在次日上午9点开始生成，建议商户在次日上午10点以后获取；
    ///
    /// 资金账单中所有涉及金额的字段均以“元”为单位。
    ///
    /// 以商户号维度频率限制为3QPS。
    ///
    /// 文件格式说明
    /// 账单文件主要由明细数据和汇总数据两大部分构成，每部分均包含一行表头以及多行详细数据。
    ///
    /// 明细数据的每一行都代表一笔具体的资金操作。为防止数据在Excel中被自动转换为科学计数法，每项数据前均添加了字符`。若需汇总计算金额等数据，可以批量移除该字符。
    pub async fn download_bill_v3(
        &self,
        bill_date: &str, // 账单日期
        acct_type: Option<AccountType>, // 账户类型
        tar_type: Option<&str>, // 压缩类型
    ) -> LabradorResult<FundFlowBillResponseV3> {
        let mut http_request = Request::builder()
            .method(HttpMethod::Get)
            .path("/v3/bill/fundflowbill")
            .query_param("bill_date", bill_date);
        if let Some(acct_type) = acct_type {
            http_request = http_request.query_param("account_type", acct_type.as_str());
        }
        if let Some(tar_type) = tar_type {
            http_request = http_request.query_param("tar_type", tar_type);
        }
        let http_request = http_request.build();
        let response = self.http_client.request(http_request).await?;
        // 验证签名
        if let Some(signer) = self.http_client.signer() {
            if !signer.verify_signature(&response)? {
                return Err(LabraError::Sign("响应签名验证失败".to_string()));
            }
        }
        let result = response.json::<WechatPayCommonResponse<FundFlowBillResponseV3>>()?;

        if !result.is_success() {
            return Err(LabraError::business(
                result.code.unwrap_or_default(),
                result.message.unwrap_or_default(),
            ));
        }

        Ok(result.data)
    }
    
    pub async fn download_bill_v3_by_url(
        &self,
        url: &str,
    ) -> LabradorResult<Bytes> {
        let http_request = Request::builder()
            .method(HttpMethod::Get)
            .path(url)
            .build();
        let response = self.http_client.request(http_request).await?;
        let response_bytes = response.bytes();
        Ok(response_bytes)
    }

    /// # 付款码支付 V3
    /// 收银员使用扫码设备读取微信用户付款码后，调用该接口发起支付。
    ///
    /// [接口文档](https://pay.weixin.qq.com/docs/merchant/apis/code-payment-v3/direct/code-pay.html)
    pub async fn codepay_v3(&self, mut request: CodepayOrderRequestV3) -> LabradorResult<WechatPayResponseV3> {
        request = request.mch_id(&self.config.mch_id);
        request = request.appid(&self.config.app_id);
        let http_request = Request::builder()
            .method(HttpMethod::Post)
            .path("/v3/pay/transactions/codepay")
            .body(request)
            .build();

        let response = self.http_client.request(http_request).await?;
        // 验证签名
        if let Some(signer) = self.http_client.signer() {
            if !signer.verify_signature(&response)? {
                return Err(LabraError::Sign("响应签名验证失败".to_string()));
            }
        }
        let result = response.json::<WechatPayCommonResponse<WechatPayResponseV3>>()?;

        if !result.is_success() {
            return Err(LabraError::business(
                result.code.unwrap_or_default(),
                result.message.unwrap_or_default(),
            ));
        }

        Ok(result.data)
    }

    /// # 合单支付-统一下单 V3
    /// 使用合单支付接口，用户只输入一次密码即可完成多个订单的支付。目前最多支持50笔。
    ///
    /// [接口文档](https://pay.weixin.qq.com/docs/merchant/apis/combine-payment/orders/jsapi-prepay.html)
    pub async fn combine_order_v3(&self, mut request: CombineOrderRequestV3) -> LabradorResult<CombineOrderResponseV3> {
        request.combine_mch_id = self.config.mch_id.clone();
        if request.combine_appid.is_none() {
            request = request.combine_appid(&self.config.app_id);
        }
        let req_path = request.req_path();
        let http_request = Request::builder()
            .method(HttpMethod::Post)
            .path(req_path)
            .body(request)
            .build();

        let response = self.http_client.request(http_request).await?;
        // 验证签名
        if let Some(signer) = self.http_client.signer() {
            if !signer.verify_signature(&response)? {
                return Err(LabraError::Sign("响应签名验证失败".to_string()));
            }
        }
        let result = response.json::<WechatPayCommonResponse<CombineOrderResponseV3>>()?;

        if !result.is_success() {
            return Err(LabraError::business(
                result.code.unwrap_or_default(),
                result.message.unwrap_or_default(),
            ));
        }

        Ok(result.data)
    }

    /// # 合单支付-查询订单 V3
    ///
    /// [接口文档](https://pay.weixin.qq.com/docs/merchant/apis/combine-payment/orders/query-order.html)
    pub async fn combine_order_query_v3(&self, request: &CombineOrderQueryRequestV3) -> LabradorResult<CombineOrderResponseV3> {
        let http_request = Request::builder()
            .method(HttpMethod::Get)
            .path(request.req_path())
            .build();

        let response = self.http_client.request(http_request).await?;
        // 验证签名
        if let Some(signer) = self.http_client.signer() {
            if !signer.verify_signature(&response)? {
                return Err(LabraError::Sign("响应签名验证失败".to_string()));
            }
        }
        let result = response.json::<WechatPayCommonResponse<CombineOrderResponseV3>>()?;

        if !result.is_success() {
            return Err(LabraError::business(
                result.code.unwrap_or_default(),
                result.message.unwrap_or_default(),
            ));
        }

        Ok(result.data)
    }

    /// # 合单支付-关闭订单 V3
    ///
    /// [接口文档](https://pay.weixin.qq.com/docs/merchant/apis/combine-payment/orders/close-order.html)
    pub async fn combine_close_order_v3(&self, request: &CombineCloseOrderRequestV3) -> LabradorResult<()> {
        let body = json!({
            "combine_appid": &self.config.app_id,
            "sub_orders": request.sub_orders,
        });
        let http_request = Request::builder()
            .method(HttpMethod::Post)
            .path(request.req_path())
            .body(body)
            .build();

        let response = self.http_client.request(http_request).await?;
        // 验证签名
        if let Some(signer) = self.http_client.signer() {
            if !signer.verify_signature(&response)? {
                return Err(LabraError::Sign("响应签名验证失败".to_string()));
            }
        }
        let result = response.json::<WechatPayCommonResponse<Value>>()?;
        if !result.is_success() {
            return Err(LabraError::business(
                result.code.unwrap_or_default(),
                result.message.unwrap_or_default(),
            ));
        }

        Ok(())
    }

    ///
    /// # 撤销订单 V3
    /// 详见 [文档](https://pay.weixin.qq.com/docs/merchant/apis/in-person-payment/reverse-order.html)
    /// <pre>
    /// 支付交易返回失败或支付系统超时，调用该接口撤销交易。
    /// 接口地址：POST /v3/pay/transactions/out-trade-no/{out_trade_no}/reverse
    /// </pre>
    pub async fn reverse_order_v3(&self, request: OrderReverseRequestV3) -> LabradorResult<()> {
        let http_request = Request::builder()
            .method(HttpMethod::Post)
            .path(request.req_path())
            .body(json!({
                "mchid": &self.config.mch_id
            }))
            .build();

        let response = self.http_client.request(http_request).await?;
        // 验证签名
        if let Some(signer) = self.http_client.signer() {
            if !signer.verify_signature(&response)? {
                return Err(LabraError::Sign("响应签名验证失败".to_string()));
            }
        }
        let result = response.json::<WechatPayCommonResponse<Value>>()?;
        if !result.is_success() {
            return Err(LabraError::business(
                result.code.unwrap_or_default(),
                result.message.unwrap_or_default(),
            ));
        }

        Ok(())
    }

    ///
    /// # 申请交易账单 V3
    /// 详见 [文档](https://pay.weixin.qq.com/docs/merchant/apis/bill-download/trade-bill.html)
    /// <pre>
    /// 微信支付按天提供交易账单文件，商户可以通过该接口获取账单文件的下载地址。
    /// 接口地址：GET /v3/bill/tradebill
    /// </pre>
    pub async fn trade_bill_v3(&self, request: &TradeBillRequestV3) -> LabradorResult<TradeBillResponseV3> {
        let mut http_request = Request::builder()
            .method(HttpMethod::Get)
            .path("/v3/bill/tradebill")
            .query_param("bill_date", &request.bill_date);
        if let Some(bill_type) = &request.bill_type {
            http_request = http_request.query_param("bill_type", bill_type.as_str());
        }
        if let Some(tar_type) = &request.tar_type {
            http_request = http_request.query_param("tar_type", tar_type.as_str());
        }
        let http_request = http_request.build();
        let response = self.http_client.request(http_request).await?;
        // 验证签名
        if let Some(signer) = self.http_client.signer() {
            if !signer.verify_signature(&response)? {
                return Err(LabraError::Sign("响应签名验证失败".to_string()));
            }
        }
        let result = response.json::<WechatPayCommonResponse<TradeBillResponseV3>>()?;

        if !result.is_success() {
            return Err(LabraError::business(
                result.code.unwrap_or_default(),
                result.message.unwrap_or_default(),
            ));
        }

        Ok(result.data)
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
        mut request: WechatOrderReverseRequest
    ) -> LabradorResult<WechatOrderReverseResponse> {
        request.appid = self.config.app_id.to_string().into();
        request.sign = request.generate_sign(&self.config.api_key.clone().unwrap_or_default())?;
        let xml= request.parse_xml();
        let http_request = Request::builder()
            .method(HttpMethod::Post)
            .path("/secapi/pay/reverse")
            .body(RequestBody::xml(xml))
            .build();

        let response = self.http_client.request(http_request).await?;
        let response_text = response.text()?;

        let result = WechatOrderReverseResponse::from_xml(&response_text)?;
        if !result.is_success() {
            return Err(LabraError::business(
                result.err_code.unwrap_or_default(),
                result.err_code_des.unwrap_or_default(),
            ));
        } else {
            // 验证签名
            if !result.verify_signature(&self.config.api_key.clone().unwrap_or_default())? {
                return Err(LabraError::Sign("响应签名验证失败".to_string()));
            }
        }

        Ok(result)
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
        mut request: WechatPayShorturlRequest
    ) -> LabradorResult<WechatPayShortUrlResponse> {
        request.appid = self.config.app_id.to_owned().into();
        request.mch_id = self.config.mch_id.to_owned().into();
        request.sign = request.generate_sign(&self.config.api_key.clone().unwrap_or_default())?;
        let xml= request.parse_xml();
        let http_request = Request::builder()
            .method(HttpMethod::Post)
            .path("/tools/shorturl")
            .body(RequestBody::xml(xml))
            .build();

        let response = self.http_client.request(http_request).await?;
        let response_text = response.text()?;

        let result = WechatPayShortUrlResponse::from_xml(&response_text)?;
        if !result.is_success() {
            return Err(LabraError::business(
                result.return_code.unwrap_or_default(),
                result.return_msg.unwrap_or_default(),
            ));
        }
        Ok(result)
    }

    /// 解析支付回调
    /// 详见 [文档](https://pay.weixin.qq.com/wiki/doc/api/jsapi.php?chapter=9_7)
    pub fn parse_payment_notify(&self, xml: &str) -> LabradorResult<WechatPayNotifyResponse> {
        let response = WechatPayNotifyResponse::from_xml(xml)?;
        let value = serde_json::to_value(&response)?;
        let mut sign_params = BTreeMap::new();
        if let Value::Object(map) = value {
            for (key, val) in map {
                if key != "sign" {
                    // 将 Value 转换为字符串（根据微信支付的格式要求）
                    let value_str = match val {
                        Value::String(s) => s,
                        Value::Number(n) => n.to_string(),
                        Value::Bool(b) => b.to_string(),
                        Value::Null => String::new(),
                        _ => val.to_string(),
                    };
                    sign_params.insert(key, value_str);
                }
            }
        }
        let calculated_sign = UnifiedOrderRequest::generate_sign(&sign_params, &self.config.api_key.clone().unwrap_or_default())?;
        let sign = response.sign.to_string();
        if !(calculated_sign == *sign) {
            return Err(LabraError::Sign("签名验证失败".to_string()));
        }
        Ok(response)
    }

    /// # 解析支付结果通知. - v3
    /// 详见 [文档](https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter3_1_5.shtml)
    pub async fn parse_payment_notify_v3(&self, notify_data: &str, header: Option<WechatSignatureHeader>) -> LabradorResult<DecryptNotifyResult> {
        if header.is_none() {
            return Err(LabraError::RequestError("非法请求，头部信息为空".to_string()));
        }
        let header = header.unwrap();
        if let Some(signer) = self.http_client.downcast_signer::<WechatPaySigner>() {
            if !signer.verify_header_sign_v3(notify_data, &header)? {
                return Err(LabraError::Sign("签名验证失败".to_string()));
            }
        } else {
            return Err(LabraError::Sign("签名验证失败".to_string()));
        }

        let origin = serde_json::from_str::<OriginNotifyResponse>(notify_data)?;
        let resource = origin.resource.to_owned();
        let decrypted = WechatPayClient::decrypt_data_v3(&self.config.api_key_v3.clone().unwrap_or_default(), resource)?;
        let decrypt_notify_result = serde_json::from_slice::<DecryptNotifyResult>(&decrypted)?;
        Ok(decrypt_notify_result)
    }

    /// # 解析退款结果通知.
    /// 详见 [文档](https://pay.weixin.qq.com/wiki/doc/api/jsapi.php?chapter=9_16&index=9)
    pub fn parse_refund_notify(&self, xml: &str) -> LabradorResult<WechatDecryptRefundNotifyResponse> {
        let response = WechatEncryptResponse::from_xml(xml)?;
        let decrypted = WechatPayClient::decrypt_data(&self.config.api_key.clone().unwrap_or_default(), response)?;
        let decrypt_notify_result = quick_xml::de::from_str::<WechatDecryptRefundNotifyResponse>(&String::from_utf8(decrypted)?)?;
        Ok(decrypt_notify_result)


    }

    /// # 解析退款结果通知 - V3.
    /// 详见 [文档](https://pay.weixin.qq.com/wiki/doc/api/jsapi.php?chapter=9_16&index=9)
    pub async fn parse_refund_notify_v3(&self, notify_data: &str, header: Option<WechatSignatureHeader>) -> LabradorResult<DecryptRefundNotifyResult> {
        if header.is_none() {
            return Err(LabraError::RequestError("非法请求，头部信息验证为空".to_string()));
        }
        let header = header.unwrap();
        if let Some(signer) = self.http_client.downcast_signer::<WechatPaySigner>() {
            if !signer.verify_header_sign_v3(notify_data, &header)? {
                return Err(LabraError::Sign("签名验证失败".to_string()));
            }
        } else {
            return Err(LabraError::Sign("签名验证失败".to_string()));
        }

        let origin = serde_json::from_str::<OriginNotifyResponse>(notify_data)?;
        let resource = origin.resource.to_owned();
        let decrypted = WechatPayClient::decrypt_data_v3(&self.config.api_key_v3.clone().unwrap_or_default(), resource)?;
        let decrypt_notify_result = serde_json::from_slice::<DecryptRefundNotifyResult>(&decrypted)?;
        Ok(decrypt_notify_result)
    }

    /// # 解析扫码支付回调通知
    /// 详见 [文档](https://pay.weixin.qq.com/wiki/doc/api/native.php?chapter=6_4)
    pub fn parse_scan_pay_notify(&self, xml: &str) -> LabradorResult<WechatScanPayNotifyResponse> {
        WechatScanPayNotifyResponse::from_xml(xml)
    }

    /// 获取配置
    pub fn config(&self) -> &WechatPayConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use crate::platforms::wechat::pay::builder::WechatPayBuilder;
    use crate::platforms::wechat::pay::types::{Amount, Detail, GoodsDetail, Payer, TradeType, UnifiedOrderRequestV3};
    use super::*;

    async fn create_client() -> WechatPayClient {
        let client = WechatPayBuilder::new("wxd17fc52706acfe11", "1602920235", "cbc9ae18a1d87ecac3bdb10976230546", "http://api.woofcloud.com/callback")
            .p12_path("/Users/mrpan/Documents/cert/1602920235_20251128_cert/apiclient_cert.p12", None)
            .api_key_v3("364ae33e57cf4989b8aefaa66ddc7ca7")
            .build().await.unwrap();
        client
    }
    
    #[tokio::test]
    async fn test_unified_order() {
        let client = create_client().await;
        let request = UnifiedOrderRequest::new(
            "测试商品",
            "16029202235sdfsdfas32234234",
            1,
            "ip",
            "test",
            TradeType::Jsapi,
        );
        let response: UnifiedOrderResponse = client.unified_order(request).await.unwrap();
        println!("{:?}", response);
    }
    
    #[tokio::test]
    async fn test_unified_order_v3() {
        let client = create_client().await;
        let mut request = UnifiedOrderRequestV3::new(
            TradeType::Jsapi,
            "16029202235sdfsdfas32234234",
            "测试商品",
            Amount::new(1),
            Detail::new(vec![GoodsDetail::new("1001".to_string(), "测试商品".to_string(), 1, 1)]),
            "https://api.woofcloud.com/shop/callback",
        );
        request.payer(Payer::new("oY0lJ47M7AoNI-0Q8R5-Pt0Iok_A"));
        let response: WechatPayResponseV3 = client.unified_order_v3(request).await.unwrap();
        println!("{:?}", response);
    }
    
    #[tokio::test]
    async fn test_order_query() {
        let client = create_client().await;
        let request = OrderQueryRequest::by_out_trade_no("out_trade_no");
        let response: OrderQueryResponse = client.order_query(&request).await.unwrap();
        println!("{:?}", response);
    }

    #[tokio::test]
    async fn test_order_query_v3() {
        let client = create_client().await;
        let request = OrderQueryRequestV3::by_out_trade_no("P2025110922333212aee8");
        let response: OrderQueryResponseV3 = client.order_query_v3(request).await.unwrap();
        println!("{:?}", response);
    }
    
    #[tokio::test]
    async fn test_refund() {
        let config = WechatPayConfig::new(
            "wx7c5c0f5f5f5f5f5f",
            "test",
            "MchId",
            "ApiKey",
        );
        let client = WechatPayClient::new(config).unwrap();
        let request = RefundRequest::new(
            "transaction_id",
            "out_trade_no",
            1,
            1,
        ).refund_desc("test");
        let response: RefundResponse = client.refund(&request).await.unwrap();
        println!("{:?}", response);
    }
    
    #[tokio::test]
    async fn test_refund_query() {
        let config = WechatPayConfig::new(
            "wx7c5c0f5f5f5f5f5f",
            "test",
            "MchId",
            "ApiKey",
        );
        let client = WechatPayClient::new(config).unwrap();
        let request = RefundQueryRequest::new(Some("transaction_id".to_string()), Some("out_trade_no".to_string()), Some("refund_id".to_string()), None);
        let response: RefundQueryResponse = client.refund_query(&request).await.unwrap();
        println!("{:?}", response);
    }
    
    #[tokio::test]
    async fn test_close() {
        let client = create_client().await;
        client.close_order("out_trade_no").await.unwrap();
        println!("close order success");
    }

    #[tokio::test]
    async fn test_close_v3() {
        let client = create_client().await;
        client.close_order_v3("P2025110922333212aee8").await.unwrap();
        println!("close order success");
    }
}