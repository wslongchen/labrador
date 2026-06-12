/*
 *
 *  *
 *  *      Copyright (c) 2018-2025, WoofCloud All rights reserved.
 *  *
 *  *   Redistribution and use in source and binary forms, with or without
 *  *   modification, are permitted provided that the following conditions are met:
 *  *
 *  *   Redistributions of source code must retain the above copyright notice,
 *  *   this list of conditions and the following disclaimer.
 *  *   Redistributions in binary form must reproduce the above copyright
 *  *   notice, this list of conditions and the following disclaimer in the
 *  *   documentation and/or other materials provided with the distribution.
 *  *   Neither the name of the www.woofcloud.com developer nor the names of its
 *  *   contributors may be used to endorse or promote products derived from
 *  *   this software without specific prior written permission.
 *  *   Author: WoofCloud
 *  *
 *
 */

//! 微信支付服务商（合作伙伴）模式
//!
//! 提供微信支付服务商模式下的各类接口，包括统一下单、查询、关单、退款等。

use crate::client::certificate::Certificate;
use crate::client::identity::Identity;
use crate::errors::{LabraError, LabradorResult};
use crate::platforms::wechat::constants;
use crate::platforms::wechat::pay::config::WechatPayConfig;
use crate::platforms::wechat::pay::types::{
    Amount, DecryptNotifyResult, DecryptRefundNotifyResult, OrderQueryResponseV3,
    OriginNotifyResponse, Payer, PlatformCertificateResponse, RefundAmount, RefundQueryResponseV3,
    RefundResponseV3, SceneInfo, TradeType, WechatEncryptResponseV3, WechatPayCommonResponse,
    WechatPayResponseV3, WechatSignatureHeader,
};
use crate::platforms::wechat::signer::WechatPaySigner;
use crate::request::{HttpMethod, Request};
use crate::{AesEncryptor, AesMode, ApiClient, ClientBuilder};
use base64::engine::general_purpose;
use base64::Engine;
use serde::Serialize;
use serde_json::{json, Value};
use std::fs;

/// 服务商统一下单请求 V3
#[derive(Debug, Clone, Serialize)]
pub struct PartnerUnifiedOrderRequestV3 {
    /// 服务商应用ID
    #[serde(rename = "sp_appid")]
    pub sp_appid: String,
    /// 服务商户号
    #[serde(rename = "sp_mchid")]
    pub sp_mchid: String,
    /// 子商户应用ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sub_appid")]
    pub sub_appid: Option<String>,
    /// 子商户号
    #[serde(rename = "sub_mchid")]
    pub sub_mchid: String,
    /// 商品描述
    pub description: String,
    /// 商户订单号
    pub out_trade_no: String,
    /// 交易结束时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_expire: Option<String>,
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
    /// 场景信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_info: Option<SceneInfo>,
    /// 结算信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_info: Option<crate::platforms::wechat::pay::types::WechatSettleInfo>,
    /// 交易类型
    #[serde(skip_serializing)]
    pub trade_type: TradeType,
}

impl PartnerUnifiedOrderRequestV3 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        sp_appid: &str,
        sp_mchid: &str,
        sub_mchid: &str,
        out_trade_no: &str,
        description: &str,
        amount: Amount,
        notify_url: &str,
        trade_type: TradeType,
    ) -> Self {
        Self {
            sp_appid: sp_appid.to_string(),
            sp_mchid: sp_mchid.to_string(),
            sub_appid: None,
            sub_mchid: sub_mchid.to_string(),
            description: description.to_string(),
            out_trade_no: out_trade_no.to_string(),
            time_expire: None,
            attach: None,
            notify_url: notify_url.to_string(),
            amount,
            payer: None,
            scene_info: None,
            settle_info: None,
            trade_type,
        }
    }

    pub fn sub_appid(mut self, appid: &str) -> Self {
        self.sub_appid = Some(appid.to_string());
        self
    }

    pub fn payer(mut self, payer: Payer) -> Self {
        self.payer = Some(payer);
        self
    }

    pub fn scene_info(mut self, info: SceneInfo) -> Self {
        self.scene_info = Some(info);
        self
    }

    pub fn attach(mut self, attach: &str) -> Self {
        self.attach = Some(attach.to_string());
        self
    }

    pub fn time_expire(mut self, time_expire: &str) -> Self {
        self.time_expire = Some(time_expire.to_string());
        self
    }

    pub fn settle_info(
        mut self,
        info: crate::platforms::wechat::pay::types::WechatSettleInfo,
    ) -> Self {
        self.settle_info = Some(info);
        self
    }
}

/// 服务商退款请求 V3
#[derive(Debug, Clone, Serialize)]
pub struct PartnerRefundRequestV3 {
    /// 子商户号
    #[serde(rename = "sub_mchid")]
    pub sub_mchid: String,
    /// 交易编号（与out_trade_no二选一）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// 商户订单号（与transaction_id二选一）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_trade_no: Option<String>,
    /// 退款单号
    pub out_refund_no: String,
    /// 退款原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 回调地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_url: Option<String>,
    /// 金额信息
    pub amount: RefundAmount,
}

impl PartnerRefundRequestV3 {
    pub fn new(sub_mchid: &str, out_refund_no: &str, amount: RefundAmount) -> Self {
        Self {
            sub_mchid: sub_mchid.to_string(),
            transaction_id: None,
            out_trade_no: None,
            out_refund_no: out_refund_no.to_string(),
            reason: None,
            notify_url: None,
            amount,
        }
    }

    pub fn transaction_id(mut self, id: &str) -> Self {
        self.transaction_id = Some(id.to_string());
        self
    }

    pub fn out_trade_no(mut self, no: &str) -> Self {
        self.out_trade_no = Some(no.to_string());
        self
    }

    pub fn reason(mut self, reason: &str) -> Self {
        self.reason = Some(reason.to_string());
        self
    }

    pub fn notify_url(mut self, url: &str) -> Self {
        self.notify_url = Some(url.to_string());
        self
    }
}

/// 微信支付服务商客户端
pub struct PartnerPayClient {
    http_client: ApiClient,
    config: WechatPayConfig,
}

impl PartnerPayClient {
    /// 创建服务商客户端
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
        let mut signer = WechatPaySigner::from_config(config);

        if let Some(identity) = Self::load_identity(config)? {
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
                if root_cert.is_x509() {
                    client_builder = client_builder.add_root_certificate(root_cert);
                }
            }
        }

        let mut http_client = client_builder.build()?;
        http_client.set_signer(signer);
        Ok(http_client)
    }

    fn load_identity(config: &WechatPayConfig) -> LabradorResult<Option<Identity>> {
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
        if let Some(p12_path) = &config.p12_path {
            let p12_data = fs::read(p12_path)
                .map_err(|e| LabraError::Certificate(format!("读取P12文件失败: {}", e)))?;
            let password = config.p12_password.as_deref().unwrap_or(&config.mch_id);
            let identity = Identity::from_pkcs12(&p12_data, password)
                .map_err(|e| LabraError::Certificate(format!("解析P12文件失败: {}", e)))?;
            return Ok(Some(identity));
        }
        Ok(None)
    }

    /// 自动获取微信支付平台证书列表
    ///
    /// 从 `/v3/certificates` 接口下载平台证书，并解密返回可用于签名验证的证书对象。
    /// 该接口为静态方法，不需要实例化客户端。
    ///
    /// # 参数
    /// * `config` - 微信支付配置，需包含 APIv3 密钥 (`api_key_v3`) 用于解密证书密文
    ///
    /// # 返回
    /// 返回 `LabradorResult<Vec<Certificate>>`，成功时包含解密后的平台证书列表
    ///
    /// # 官方文档
    /// <https://pay.weixin.qq.com/docs/merchant/apis/platform-certificate/api-v3-get-certificates.html>
    pub async fn get_certificates(config: &WechatPayConfig) -> LabradorResult<Vec<Certificate>> {
        let http_client = Self::build_client(config)?;
        let request = Request::builder()
            .method(HttpMethod::Get)
            .path("/v3/certificates")
            .build();
        let response = http_client.request(request).await?;
        let mut certs = Vec::new();
        if response.is_success() {
            let certs_response = response.json::<PlatformCertificateResponse>()?;
            if let Some(cert_list) = certs_response.data {
                for cert in cert_list.into_iter() {
                    let data = cert.encrypt_certificate;
                    let res = Self::decrypt_data_v3(
                        &config.api_key_v3.clone().unwrap_or_default(),
                        data,
                    )?;
                    let cert = Certificate::from_pem(&res)?;
                    certs.push(cert);
                }
            }
        }
        Ok(certs)
    }

    fn decrypt_data_v3(api_key_v3: &str, data: WechatEncryptResponseV3) -> LabradorResult<Vec<u8>> {
        let key = api_key_v3.as_bytes();
        let associated_data = data.associated_data.to_owned().unwrap_or_default();
        let nonce = data.nonce.to_owned();
        let ciphertext = data.ciphertext.to_owned().unwrap_or_default();
        let cipher_text = general_purpose::STANDARD.decode(ciphertext)?;
        let base64_cipher = hex::encode(cipher_text);
        let cipher_text = hex::decode(base64_cipher)?;
        let aad = associated_data.as_bytes();
        let nonce = nonce.as_bytes();
        let ciphertext_length = cipher_text.len() - 16;
        let ciphertext_bytes = &cipher_text[0..ciphertext_length];
        let tag = &cipher_text[ciphertext_length..];
        let encryptor = AesEncryptor::new(AesMode::Gcm, key, nonce)?;
        let decrypted_data = encryptor.gcm_decrypt(nonce, aad, ciphertext_bytes, tag)?;
        Ok(decrypted_data)
    }

    // ==================== 服务商统一下单 ====================

    /// 服务商统一下单 V3
    ///
    /// 通过服务商模式发起统一下单，支持 JSAPI、Native、App、H5、小程序等多种交易类型。
    /// 请求中的 `trade_type` 决定了实际调用的微信支付子路径。
    ///
    /// # 参数
    /// * `request` - 服务商统一下单请求，包含服务商/子商户信息、商品描述、金额、交易类型等字段
    ///
    /// # 返回
    /// 返回 `LabradorResult<WechatPayResponseV3>`，成功时包含 `prepay_id` 等预支付信息
    ///
    /// # 官方文档
    /// <https://pay.weixin.qq.com/docs/partner/apis/partner-payment/jsapi-prepay.html>
    pub async fn partner_unified_order_v3(
        &self,
        request: PartnerUnifiedOrderRequestV3,
    ) -> LabradorResult<WechatPayResponseV3> {
        let req_path = request.trade_type.isv_req_path_v3();
        let http_request = Request::builder()
            .method(HttpMethod::Post)
            .path(req_path)
            .body(request)
            .build();

        let response = self.http_client.request(http_request).await?;
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

    // ==================== 服务商查询订单 ====================

    /// 服务商查询订单 V3（按商户订单号）
    ///
    /// 通过商户订单号查询服务商模式下的订单交易状态。使用 GET 请求，参数拼接在 URL 查询字符串中。
    ///
    /// # 参数
    /// * `out_trade_no` - 商户订单号，下单时传入的商户侧唯一订单编号
    /// * `sp_mchid` - 服务商的商户号
    /// * `sub_mchid` - 子商户的商户号
    ///
    /// # 返回
    /// 返回 `LabradorResult<OrderQueryResponseV3>`，成功时包含订单状态、支付信息、金额等详细信息
    ///
    /// # 官方文档
    /// <https://pay.weixin.qq.com/docs/partner/apis/partner-payment/query-by-out-trade-no.html>
    pub async fn partner_order_query_by_out_trade_no(
        &self,
        out_trade_no: &str,
        sp_mchid: &str,
        sub_mchid: &str,
    ) -> LabradorResult<OrderQueryResponseV3> {
        let path = format!(
            "/v3/pay/partner/transactions/out-trade-no/{}?sp_mchid={}&sub_mchid={}",
            out_trade_no, sp_mchid, sub_mchid
        );
        let http_request = Request::builder()
            .method(HttpMethod::Get)
            .path(path)
            .build();

        let response = self.http_client.request(http_request).await?;
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

    // ==================== 服务商关闭订单 ====================

    /// 服务商关闭订单 V3
    ///
    /// 关闭服务商模式下已创建但未支付的订单。关单后用户将无法继续支付。
    ///
    /// # 参数
    /// * `out_trade_no` - 商户订单号，需要关闭的订单编号
    /// * `sp_mchid` - 服务商的商户号
    /// * `sub_mchid` - 子商户的商户号
    ///
    /// # 返回
    /// 返回 `LabradorResult<()>`，成功时返回空元组。若订单已支付或不存在将返回业务错误
    ///
    /// # 官方文档
    /// <https://pay.weixin.qq.com/docs/partner/apis/partner-payment/close-order.html>
    pub async fn partner_close_order_v3(
        &self,
        out_trade_no: &str,
        sp_mchid: &str,
        sub_mchid: &str,
    ) -> LabradorResult<()> {
        let path = format!(
            "/v3/pay/partner/transactions/out-trade-no/{}/close",
            out_trade_no
        );
        let http_request = Request::builder()
            .method(HttpMethod::Post)
            .path(path)
            .body(json!({
                "sp_mchid": sp_mchid,
                "sub_mchid": sub_mchid,
            }));

        let response = self.http_client.request(http_request).await?;
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

    // ==================== 服务商申请退款 ====================

    /// 服务商申请退款 V3
    ///
    /// 通过服务商模式为已支付的订单发起退款申请。支持通过微信交易号或商户订单号指定退款订单，
    /// 退款金额由 `RefundAmount` 结构控制。
    ///
    /// # 参数
    /// * `request` - 服务商退款请求，包含子商户号、退款单号、退款金额、退款原因及回调地址等
    ///
    /// # 返回
    /// 返回 `LabradorResult<RefundResponseV3>`，成功时包含退款单号、退款状态等信息
    ///
    /// # 官方文档
    /// <https://pay.weixin.qq.com/docs/partner/apis/refund/refunds/create.html>
    pub async fn partner_refund_v3(
        &self,
        request: PartnerRefundRequestV3,
    ) -> LabradorResult<RefundResponseV3> {
        let http_request = Request::builder()
            .method(HttpMethod::Post)
            .path("/v3/refund/domestic/refunds")
            .body(request)
            .build();

        let response = self.http_client.request(http_request).await?;
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

    // ==================== 服务商退款查询 ====================

    /// 服务商退款查询 V3
    ///
    /// 通过商户退款单号查询服务商模式下退款申请的当前状态。
    ///
    /// # 参数
    /// * `out_refund_no` - 商户退款单号，发起退款时传入的唯一退款编号
    /// * `sub_mchid` - 子商户的商户号
    ///
    /// # 返回
    /// 返回 `LabradorResult<RefundQueryResponseV3>`，成功时包含退款状态、退款金额、退款渠道等信息
    ///
    /// # 官方文档
    /// <https://pay.weixin.qq.com/docs/partner/apis/refund/refunds/query-by-out-refund-no.html>
    pub async fn partner_refund_query_v3(
        &self,
        out_refund_no: &str,
        sub_mchid: &str,
    ) -> LabradorResult<RefundQueryResponseV3> {
        let path = format!(
            "/v3/refund/domestic/refunds/{}?sub_mchid={}",
            out_refund_no, sub_mchid
        );
        let http_request = Request::builder()
            .method(HttpMethod::Get)
            .path(path)
            .build();

        let response = self.http_client.request(http_request).await?;
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

    // ==================== 服务商支付通知解析 ====================

    /// 服务商支付通知解析 V3
    ///
    /// 解析并验证微信支付回调通知。首先验证 HTTP 头部签名，再使用 APIv3 密钥解密通知体，
    /// 返回解密后的支付结果数据。该接口用于接收微信支付异步通知。
    ///
    /// # 参数
    /// * `notify_data` - 微信支付回调的原始 JSON 字符串（请求体）
    /// * `header` - 回调请求的签名头部信息（WechatSignatureHeader），用于验证通知来源
    ///
    /// # 返回
    /// 返回 `LabradorResult<DecryptNotifyResult>`，成功时包含解密后的支付通知详情
    ///
    /// # 官方文档
    /// <https://pay.weixin.qq.com/docs/partner/apis/partner-payment/payment-notice.html>
    pub async fn partner_parse_payment_notify_v3(
        &self,
        notify_data: &str,
        header: Option<WechatSignatureHeader>,
    ) -> LabradorResult<DecryptNotifyResult> {
        let header =
            header.ok_or_else(|| LabraError::RequestError("非法请求，头部信息为空".to_string()))?;

        if let Some(signer) = self.http_client.downcast_signer::<WechatPaySigner>() {
            if !signer.verify_header_sign_v3(notify_data, &header)? {
                return Err(LabraError::Sign("签名验证失败".to_string()));
            }
        } else {
            return Err(LabraError::Sign("签名验证失败".to_string()));
        }

        let origin = serde_json::from_str::<OriginNotifyResponse>(notify_data)?;
        let resource = origin.resource.to_owned();
        let decrypted = Self::decrypt_data_v3(
            &self.config.api_key_v3.clone().unwrap_or_default(),
            resource,
        )?;
        let result = serde_json::from_slice::<DecryptNotifyResult>(&decrypted)?;
        Ok(result)
    }

    /// 服务商退款通知解析 V3
    ///
    /// 解析并验证微信支付退款回调通知。与支付通知类似，先验证签名，再解密通知体，
    /// 返回解密后的退款结果数据。
    ///
    /// # 参数
    /// * `notify_data` - 微信退款回调的原始 JSON 字符串（请求体）
    /// * `header` - 回调请求的签名头部信息（WechatSignatureHeader），用于验证通知来源
    ///
    /// # 返回
    /// 返回 `LabradorResult<DecryptRefundNotifyResult>`，成功时包含解密后的退款通知详情
    ///
    /// # 官方文档
    /// <https://pay.weixin.qq.com/docs/partner/apis/refund/refund-notice.html>
    pub async fn partner_parse_refund_notify_v3(
        &self,
        notify_data: &str,
        header: Option<WechatSignatureHeader>,
    ) -> LabradorResult<DecryptRefundNotifyResult> {
        let header =
            header.ok_or_else(|| LabraError::RequestError("非法请求，头部信息为空".to_string()))?;

        if let Some(signer) = self.http_client.downcast_signer::<WechatPaySigner>() {
            if !signer.verify_header_sign_v3(notify_data, &header)? {
                return Err(LabraError::Sign("签名验证失败".to_string()));
            }
        } else {
            return Err(LabraError::Sign("签名验证失败".to_string()));
        }

        let origin = serde_json::from_str::<OriginNotifyResponse>(notify_data)?;
        let resource = origin.resource.to_owned();
        let decrypted = Self::decrypt_data_v3(
            &self.config.api_key_v3.clone().unwrap_or_default(),
            resource,
        )?;
        let result = serde_json::from_slice::<DecryptRefundNotifyResult>(&decrypted)?;
        Ok(result)
    }

    /// 获取配置
    pub fn config(&self) -> &WechatPayConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partner_unified_order_request_serialization() {
        let request = PartnerUnifiedOrderRequestV3::new(
            "sp_appid_xxx",
            "1900000100",
            "1900000101",
            "ORDER_20240101_001",
            "测试商品",
            Amount::new(100),
            "https://example.com/notify",
            TradeType::Jsapi,
        )
        .sub_appid("sub_appid_xxx")
        .attach("custom_data");

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("sp_appid"));
        assert!(json.contains("sp_mchid"));
        assert!(json.contains("sub_mchid"));
        assert!(json.contains("sub_appid"));
        assert!(json.contains("ORDER_20240101_001"));
    }

    #[test]
    fn test_partner_refund_request_serialization() {
        let amount = RefundAmount {
            refund: 100,
            total: 200,
            payer_total: None,
            payer_refund: None,
            currency: None,
        };
        let request = PartnerRefundRequestV3::new("1900000101", "REFUND_001", amount)
            .transaction_id("4200001234567890")
            .reason("测试退款");

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("sub_mchid"));
        assert!(json.contains("REFUND_001"));
        assert!(json.contains("4200001234567890"));
    }
}
