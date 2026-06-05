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
use std::any::Any;
use std::collections::{BTreeMap, HashMap};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use crate::client::certificate::Certificate;
use crate::errors::{LabraError, LabradorResult};
use crate::request::{Request, RequestBody};
use crate::{CryptoUtils, HashType, RsaEncryptor, RsaKeyFormat};
use crate::client::RequestSigner;
use crate::response::Response;
use crate::signer::SignMethod;
use crate::utils::encryption::base64_decode;
use crate::utils::string::random_string;
use crate::utils::time::timestamp_millis;
use crate::wechat::constants::{ACCEPT, AUTHORIZATION, CONTENT_TYPE_JSON, PAY_SIGN_SCHEMA_V3};
use crate::wechat::pay::config::{WechatPayApiVersion, WechatPayConfig};
use crate::wechat::pay::types::WechatSignatureHeader;


/// 微信支付签名器
#[derive(Debug, Clone)]
pub struct WechatPaySigner {
    /// 商户号
    mch_id: String,
    /// API密钥
    api_key: Option<String>,
    /// API密钥V3
    api_key_v3: Option<String>,
    api_version: WechatPayApiVersion,
    /// 签名类型
    sign_type: String,
    /// API证书序列号
    serial_no: Option<String>,
    /// API商户证书秘钥
    private_key: Option<String>,
    root_certificates: HashMap<String, Certificate>,
}

impl WechatPaySigner {
    /// 创建新的微信支付签名器
    pub fn new(mch_id: impl Into<String>) -> Self {
        Self {
            mch_id: mch_id.into(),
            api_key: None,
            api_key_v3: None,
            api_version: WechatPayApiVersion::V3,
            sign_type: "HMAC-SHA256".to_string(),
            serial_no: None,
            private_key: None,
            root_certificates: HashMap::new(),
        }
    }

    pub fn from_config(config: &WechatPayConfig) -> Self {
        let root_certificates = config.root_certificates.clone().map(|certs| certs.into_iter().map(|cert| (cert.serial_number.clone(), cert.clone())).collect()).unwrap_or(HashMap::new());
        Self {
            mch_id: config.mch_id.clone(),
            api_key: config.api_key.clone(),
            api_key_v3: config.api_key_v3.clone(),
            api_version: config.api_version.clone(),
            sign_type: "HMAC-SHA256".to_string(),
            serial_no: config.serial_no.clone(),
            private_key: config.private_key.clone(),
            root_certificates,
        }
    }

    pub fn is_v3(&self) -> bool {
        self.api_version == WechatPayApiVersion::V3
    }

    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn with_serial_no(mut self, serial_no: impl Into<String>) -> Self {
        self.serial_no = Some(serial_no.into());
        self
    }

    pub fn with_api_key_v3(mut self, api_key_v3: impl Into<String>) -> Self {
        self.api_key_v3 = Some(api_key_v3.into());
        self
    }

    pub fn with_private_key(mut self, private_key: impl Into<String>) -> Self {
        self.private_key = Some(private_key.into());
        self
    }

    pub fn with_root_certificates(mut self, root_certificates: Vec<Certificate>) -> Self {
        self.root_certificates = root_certificates.into_iter().map(|cert| (cert.serial_number(), cert)).collect();
        self
    }

    /// 设置签名类型
    pub fn with_sign_type(mut self, sign_type: impl Into<String>) -> Self {
        self.sign_type = sign_type.into();
        self
    }

    /// 微信支付特定的参数排序和格式化
    fn format_wechat_params(&self, params: &BTreeMap<String, String>) -> String {
        let mut sorted_keys: Vec<&String> = params.keys().collect();
        sorted_keys.sort();

        sorted_keys
            .iter()
            .filter(|&&k| !k.is_empty() && !params[k].is_empty())
            .map(|&k| format!("{}={}", k, params[k]))
            .collect::<Vec<String>>()
            .join("&")
    }

    /// 微信支付HMAC签名
    fn wechat_hmac_sign(&self, sign_string: &str) -> LabradorResult<String> {
        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(self.api_key.to_owned().unwrap_or_default().as_bytes())
            .map_err(|e| LabraError::Sign(format!("微信支付HMAC key error: {}", e)))?;
        mac.update(sign_string.as_bytes());
        let result = mac.finalize();
        let bytes = result.into_bytes();

        // 微信支付要求签名大写
        Ok(hex::encode(bytes).to_uppercase())
    }

    pub fn build_authorization_v3(&self, req: &Request) -> LabradorResult<String> {
        // 获取请求中的参数
        let url = req.path();
        let method = req.method().to_string();
        let body = req.body().as_text().map(ToString::to_string).unwrap_or_default();
        let mut format_url = url.to_string();


        for (index, (key, value)) in req.query_params().iter().enumerate() {
            if index == 0 {
                format_url.push('?');
            }
            if index > 0 {
                format_url.push('&');
            }
            format_url.push_str(key);
            format_url.push('=');
            format_url.push_str(value);
        }

        let mch_id = self.mch_id.to_string();
        let private_key = self.private_key.to_owned().unwrap_or_default();
        let serial_no = self.serial_no.to_owned().unwrap_or_default();

        if mch_id.is_empty() || serial_no.is_empty()  || private_key.is_empty() {
            return Err(LabraError::Sign("缺少必要参数".to_string()))
        }
        let nonce_str = random_string(32).to_uppercase();

        let timestamp = timestamp_millis() / 1000;
        let signatures = [method, format_url.to_string(), timestamp.to_string(), nonce_str.to_string(), body];
        let signature_str = signatures.iter().map(|item| item.to_string()).collect::<Vec<_>>().join("\n") + "\n";
        let encryptor = RsaEncryptor::with_private_key(private_key.as_bytes(), RsaKeyFormat::Pem);
        let signature = encryptor.sign(signature_str.as_bytes(), HashType::Sha256).map_err(|e| LabraError::Sign(format!("RSA加密错误: {}", e)))?;
        let authorization = format!("{} mchid=\"{}\",nonce_str=\"{}\",signature=\"{}\",timestamp=\"{}\",serial_no=\"{}\"",
                                    PAY_SIGN_SCHEMA_V3 , mch_id, nonce_str, CryptoUtils::base64_encode(&signature), timestamp, serial_no);
        tracing::debug!("wechat pay authorization built successfully");
        Ok(authorization)
    }

    /// V3  验证签名
    pub fn verify_sign_v3(&self, serial_number: &str, timestamp: &str, nonce: &str, message: &str, signature: &str) -> LabradorResult<bool> {
        let signatures = vec![timestamp, nonce, message];
        let signature_str = signatures.iter().map(|item| item.to_string()).collect::<Vec<_>>().join("\n") + "\n";
        // let signature_str = "1722850421\nd824f2e086d3c1df967785d13fcd22ef\n{\"code_url\":\"weixin://wxpay/bizpayurl?pr=JyC91EIz1\"}\n";
        if let Some(cert) = self.root_certificates.get(serial_number) {
            let signature = base64_decode(signature)?;
            let encryptor = RsaEncryptor::with_public_key(&cert.public_key, RsaKeyFormat::Pkcs8);
            let verify = encryptor.verify(signature_str.as_bytes(), &signature, HashType::Sha256)?;
            tracing::debug!("wechat pay v3 signature verify result: {}", verify);
            Ok(verify)
        } else {
            Ok(false)
        }
    }

    /// 验证签名
    pub fn verify_header_sign_v3(&self, message: &str, signature_header: &WechatSignatureHeader) -> LabradorResult<bool> {
        let signature = signature_header.signature.to_string();
        let serial_number = signature_header.serial.to_string();
        let timestamp = signature_header.time_stamp.to_string();
        let nonce = signature_header.nonce.to_string();
        let signatures = vec![timestamp, nonce, message.to_string()];
        let signature_str = signatures.iter().map(|item| item.to_string()).collect::<Vec<_>>().join("\n") + "\n";
        if let Some(cert) = self.root_certificates.get(&serial_number) {
            let signature = base64_decode(&signature)?;
            let encryptor = RsaEncryptor::with_public_key(&cert.public_key, RsaKeyFormat::Pkcs8);
            let verify = encryptor.verify(signature_str.as_bytes(), &signature, HashType::Sha256)?;
            Ok(verify)
        } else {
            Ok(false)
        }
    }

    /// 从XML中提取参数
    fn extract_xml_params(&self, value: &serde_json::Value, prefix: &str, params: &mut BTreeMap<String, String>) {
        match value {
            serde_json::Value::Object(obj) => {
                for (key, val) in obj {
                    let new_prefix = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", prefix, key)
                    };
                    self.extract_xml_params(val, &new_prefix, params);
                }
            }
            serde_json::Value::String(s) => {
                params.insert(prefix.to_string(), s.clone());
            }
            serde_json::Value::Number(n) => {
                params.insert(prefix.to_string(), n.to_string());
            }
            serde_json::Value::Bool(b) => {
                params.insert(prefix.to_string(), b.to_string());
            }
            _ => {}
        }
    }
}

impl RequestSigner for WechatPaySigner {
    fn sign_request(&self, request: &mut Request) -> LabradorResult<()> {
        use std::collections::BTreeMap;

        if self.is_v3() {
            let authorization = self.build_authorization_v3(request)?;
            request.set_header(AUTHORIZATION, authorization)?;
            request.set_header(ACCEPT, CONTENT_TYPE_JSON)?;
        } else {
            // 收集所有参数
            let mut params = BTreeMap::new();

            // 添加查询参数
            for (key, value) in &request.query_params {
                params.insert(key.clone(), value.clone());
            }

            // 添加请求体参数（如果是XML格式）
            if let RequestBody::Xml(xml) = &request.body {
                // 解析XML获取参数
                // TODO: 这里简化处理，实际需要解析XML
                if let Ok(value) = quick_xml::de::from_str::<serde_json::Value>(xml) {
                    self.extract_xml_params(&value, "", &mut params);
                }
            }

            // 添加商户号
            params.insert("mch_id".to_string(), self.mch_id.clone());

            // 微信支付需要将参数按ASCII码排序
            let sign_string = self.format_wechat_params(&params);

            // 计算HMAC-SHA256签名
            let signature = self.wechat_hmac_sign(&sign_string)?;

            // 添加签名到参数中
            params.insert("sign".to_string(), signature.clone());

            // 更新请求的查询参数
            request.query_params = params.into_iter().collect();
        }

        Ok(())
    }

    fn sign_method(&self) -> SignMethod {
        SignMethod::HmacSha256
    }

    fn verify_signature(&self, response: &Response) -> LabradorResult<bool> {
        if self.is_v3() {
            // 收集参数（排除签名字段）
            let headers = response.headers();
            let signature_header = WechatSignatureHeader::from_header(headers);
            let signature = signature_header.signature;
            let serial_number = signature_header.serial;
            let timestamp = signature_header.time_stamp;
            let nonce = signature_header.nonce;
            let message = response.text()?;
            if !signature.is_empty() {
                self.verify_sign_v3(&serial_number, &timestamp, &nonce, &message, &signature)
            } else {
                Ok(true)
            }

        } else {
            // TODO: 收集参数（排除签名字段）
            // let mut params = BTreeMap::new();
            // 
            // for (key, value) in &response.query_params {
            //     if key != "sign" {
            //         params.insert(key.clone(), value.clone());
            //     }
            // }
            // 
            // // 生成签名字符串
            // let sign_string = self.format_wechat_params(&params);
            // 
            // // 计算签名
            // let calculated_signature = self.wechat_hmac_sign(&sign_string)?;
            // 
            // Ok(calculated_signature == signature)
            Ok(true)
        }

    }

    fn secret_key(&self) -> &str {
        self.api_key.as_ref().map(|x| x.as_str()).unwrap_or("")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}