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
//! 支付宝客户端实现

use super::{constants, AlipayResponse};
use crate::alipay::config::AlipayClientConfig;
use crate::alipay::method::{AlipayMethod, RequestParametersHolder};
use crate::alipay::types::AlipayNotifyResponse;
use crate::client::builder::ClientBuilder;
use crate::client::ApiClient;
use crate::{errors::{LabraError, LabradorResult}, request::{HttpMethod, Request}, AesEncryptor, AesMode, CryptoUtils, HashType, RsaEncryptor, RsaKeyFormat};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use serde_json::json;
use tracing::warn;
use crate::alipay::{AlipayBizRequest, AlipayRequest};
use crate::alipay::miniapp::AlipayMiniappService;
use crate::alipay::open::AlipayOpenService;
use crate::alipay::pay::{AlipayPayService};
use crate::request::RequestBody;
use crate::utils::encryption::{base64_decode, base64_encode};

/// 支付宝客户端
pub struct AlipayClient {
    /// HTTP客户端
    http_client: ApiClient,
    /// 配置
    config: AlipayClientConfig,
    /// 证书缓存
    cache_certs: Arc<DashMap<String, String>>,
    /// 应用证书序列号
    app_cert_sn: Option<String>,
    /// 支付宝根证书序列号
    alipay_root_cert_sn: Option<String>,
}

/// 支付宝客户端实现
#[allow(unused)]
impl AlipayClient {
    /// 创建新的支付宝客户端
    pub fn new(config: AlipayClientConfig) -> LabradorResult<Self> {
        let base_url = if config.sandbox {
            constants::SANDBOX_API_BASE_URL
        } else {
            constants::API_BASE_URL
        };

        let http_client = ClientBuilder::new()
            .api_base_url(base_url)
            .timeout(config.timeout.unwrap_or(Duration::from_secs(30)))
            .connect_timeout(Duration::from_secs(10))
            .enable_auto_retry()
            .max_retries(3)
            .build()?;

        let mut client = Self {
            http_client,
            config,
            cache_certs: Arc::new(DashMap::new()),
            app_cert_sn: None,
            alipay_root_cert_sn: None,
        };

        // 初始化证书
        if client.config.use_cert {
            client.init_certs()?;
        }

        Ok(client)
    }

    /// 初始化证书
    fn init_certs(&mut self) -> LabradorResult<()> {
        // 初始化应用证书序列号
        if let Some(app_cert_path) = &self.config.app_cert_path {
            self.app_cert_sn = Some(self.get_app_cert_sn(app_cert_path)?);
        }

        // 初始化支付宝根证书序列号
        if let Some(root_cert_path) = &self.config.alipay_root_cert_path {
            self.alipay_root_cert_sn = Some(self.get_root_cert_sn(root_cert_path)?);
        }

        // 初始化支付宝公钥证书
        if let Some(public_cert_path) = &self.config.alipay_public_cert_path {
            self.init_alipay_public_cert(public_cert_path)?;
        }

        Ok(())
    }

    /// 获取应用证书序列号
    fn get_app_cert_sn(&self, cert_path: &str) -> LabradorResult<String> {
        use std::fs;

        let pem_content = fs::read_to_string(cert_path)
            .map_err(|e| LabraError::Certificate(format!("读取应用证书失败: {}", e)))?;

        self.extract_cert_sn(&pem_content)
    }

    /// 获取根证书序列号
    fn get_root_cert_sn(&self, cert_path: &str) -> LabradorResult<String> {
        use std::fs;

        let pem_content = fs::read_to_string(cert_path)
            .map_err(|e| LabraError::Certificate(format!("读取根证书失败: {}", e)))?;

        self.extract_root_cert_sn(&pem_content)
    }

    /// 提取证书序列号
    fn extract_cert_sn(&self, pem_content: &str) -> LabradorResult<String> {
        use x509_parser::prelude::*;

        let pem = parse_x509_pem(pem_content.as_bytes())
            .map_err(|e| LabraError::Certificate(format!("解析PEM失败: {}", e)))?
            .1;
        let cert = pem.parse_x509()
            .map_err(|e| LabraError::Certificate(format!("解析证书失败: {}", e)))?;

        let issuer = cert.issuer();
        let issuer_str = issuer.iter()
            .map(|rdn| {
                rdn.iter()
                    .map(|attr| {
                        format!("{:?}={:?}", attr.attr_type(), attr.attr_value())
                    })
                    .collect::<Vec<String>>()
                    .join(",")
            })
            .collect::<Vec<String>>()
            .join(",");

        let serial = cert.serial.to_string();
        let data = format!("{},{}", issuer_str, serial);

        let md5_digest = CryptoUtils::md5(data.as_bytes());
        Ok(md5_digest)
    }

    /// 提取根证书序列号
    fn extract_root_cert_sn(&self, pem_content: &str) -> LabradorResult<String> {
        use x509_parser::prelude::*;

        let mut sns = Vec::new();
        let mut remaining = pem_content.as_bytes();

        while !remaining.is_empty() {
            match parse_x509_pem(remaining) {
                Ok((next, pem)) => {
                    remaining = next;

                    match pem.parse_x509() {
                        Ok(cert) => {
                            // 过滤掉RSA签名算法的证书
                            let algorithm = cert.signature_algorithm.oid().to_string();
                            if algorithm.starts_with("1.2.840.113549.1.1") {
                                continue;
                            }

                            let issuer = cert.issuer();
                            let issuer_str = issuer.iter()
                                .map(|rdn| {
                                    rdn.iter()
                                        .map(|attr| {
                                            format!("{:?}={:?}", attr.attr_type(), attr.attr_value())
                                        })
                                        .collect::<Vec<String>>()
                                        .join(",")
                                })
                                .collect::<Vec<String>>()
                                .join(",");

                            let serial = cert.serial.to_string();
                            let data = format!("{},{}", issuer_str, serial);

                            let md5_digest = CryptoUtils::md5(data.as_bytes());
                            sns.push(md5_digest);
                        }
                        Err(_) => continue,
                    }
                }
                Err(_) => break,
            }
        }

        Ok(sns.join("_"))
    }

    /// 初始化支付宝公钥证书
    fn init_alipay_public_cert(&self, cert_path: &str) -> LabradorResult<()> {
        use std::fs;
        use x509_parser::prelude::*;

        let pem_content = fs::read_to_string(cert_path)
            .map_err(|e| LabraError::Certificate(format!("读取支付宝公钥证书失败: {}", e)))?;

        let mut remaining = pem_content.as_bytes();

        while !remaining.is_empty() {
            match parse_x509_pem(remaining) {
                Ok((next, pem)) => {
                    remaining = next;

                    match pem.parse_x509() {
                        Ok(cert) => {
                            let issuer = cert.issuer();
                            let issuer_str = issuer.iter()
                                .map(|rdn| {
                                    rdn.iter()
                                        .map(|attr| {
                                            format!("{:?}={:?}", attr.attr_type(), attr.attr_value())
                                        })
                                        .collect::<Vec<String>>()
                                        .join(",")
                                })
                                .collect::<Vec<String>>()
                                .join(",");

                            let serial = cert.serial.to_string();
                            let data = format!("{},{}", issuer_str, serial);
                            let md5_digest = CryptoUtils::md5(data.as_bytes());
                            let sn = md5_digest;

                            // 提取公钥
                            let public_key = cert.public_key();
                            let public_key_b64 = STANDARD.encode(public_key.raw);

                            self.cache_certs.insert(sn, public_key_b64);
                        }
                        Err(_) => continue,
                    }
                }
                Err(_) => break,
            }
        }

        Ok(())
    }

    /// 获取支付宝公钥
    fn get_alipay_public_key(&self, alipay_cert_sn: &str) -> LabradorResult<Option<String>> {
        if let Some(key) = self.cache_certs.get(alipay_cert_sn) {
            Ok(Some(key.clone()))
        } else {
            Ok(None)
        }
    }

    /// 自动加载证书（异步）
    fn auto_load_cert(&self, alipay_cert_sn: String) -> Pin<Box<dyn Future<Output = LabradorResult<Option<String>>> + Send + '_>> {
        Box::pin(async move {
        if !self.config.use_cert {
            return Ok(None);
        }

        // 构建下载证书请求
        let params = json!({
            "alipay_cert_sn": alipay_cert_sn.to_string(),
        });
        let mut request = AlipayBizRequest::new();
        request.set_biz_model(params);
        request.set_method(AlipayMethod::OpenAppAlipaycertDownload);
        let response: AlipayResponse = self.request_internal(
            request,
            None,
            None,
            None
        ).await?;

        if response.is_success() {
            if let Some(data) = response.data {
                if let Some(content) = data.get("alipay_cert_content") {
                    let content_str = content.as_str()
                        .ok_or_else(|| LabraError::Certificate("证书内容格式错误".to_string()))?;

                    self.cache_certs.insert(alipay_cert_sn.to_string(), content_str.to_string());
                    return Ok(Some(content_str.to_string()));
                }
            }
        }

        Ok(None)
        })
    }

    /// 加密数据
    fn encrypt_data(&self, plaintext: &str) -> LabradorResult<String> {
        let encrypt_key = self.config.encrypt_key
            .as_ref()
            .ok_or_else(|| LabraError::Crypto("加密密钥未配置".to_string()))?;

        // 解码AES密钥
        let aes_key = STANDARD.decode(encrypt_key)
            .map_err(|e| LabraError::Crypto(format!("解码AES密钥失败: {}", e)))?;

        // 使用AES-CBC加密
        let iv = &aes_key[..16]; // 使用AES密钥的前16字节作为IV
        let encryptor = AesEncryptor::new(AesMode::Cbc, &aes_key, iv)
            .map_err(|e| LabraError::Crypto(format!("创建AES加密器失败: {}", e)))?;

        let encrypted = encryptor.encrypt(plaintext.as_bytes())
            .map_err(|e| LabraError::Crypto(format!("加密失败: {}", e)))?;

        Ok(STANDARD.encode(encrypted))
    }

    /// 解密数据
    fn decrypt_data(&self, ciphertext: &str) -> LabradorResult<String> {
        let encrypt_key = self.config.encrypt_key
            .as_ref()
            .ok_or_else(|| LabraError::Crypto("加密密钥未配置".to_string()))?;

        // 解码AES密钥
        let aes_key = STANDARD.decode(encrypt_key)
            .map_err(|e| LabraError::Crypto(format!("解码AES密钥失败: {}", e)))?;

        // 解码加密数据
        let encrypted_data = STANDARD.decode(ciphertext)
            .map_err(|e| LabraError::Crypto(format!("解码加密数据失败: {}", e)))?;

        // 使用AES-CBC解密
        let iv = &aes_key[..16];
        let encryptor = AesEncryptor::new(AesMode::Cbc, &aes_key, iv)
            .map_err(|e| LabraError::Crypto(format!("创建AES加密器失败: {}", e)))?;

        let decrypted = encryptor.decrypt(&encrypted_data)
            .map_err(|e| LabraError::Crypto(format!("解密失败: {}", e)))?;

        String::from_utf8(decrypted)
            .map_err(|e| LabraError::Utf8(e))
    }

    /// 生成签名
    fn sign(&self, content: &str) -> LabradorResult<String> {
        tracing::debug!("开始生成签名...{}", content);
        let rsa_encryptor = RsaEncryptor::with_private_key(&base64_decode(&self.config.app_private_key)?, RsaKeyFormat::Pkcs1);
        match self.config.sign_type.as_str() {
            constants::SIGN_TYPE_RSA2 => {
                rsa_encryptor.sign(content.as_bytes(), HashType::Sha256).map(|res| base64_encode(&res))
            }
            constants::SIGN_TYPE_RSA => {
                rsa_encryptor.sign(content.as_bytes(), HashType::Sha1).map(|res| base64_encode(&res))
            }
            _ => Err(LabraError::Sign("不支持的签名类型".to_string())),
        }
    }

    /// 验证签名
    fn verify(&self, content: &str, signature: &str, alipay_cert_sn: Option<&str>) -> LabradorResult<bool> {
        let public_key = if let Some(cert_sn) = alipay_cert_sn {
            // 使用证书模式
            self.get_alipay_public_key(cert_sn)?
                .ok_or_else(|| LabraError::Sign("获取支付宝公钥失败".to_string()))?
        } else {
            // 使用普通公钥模式
            self.config.alipay_public_key
                .clone()
                .ok_or_else(|| LabraError::Sign("支付宝公钥未配置".to_string()))?
        };
        let rsa_encryptor = RsaEncryptor::with_public_key(&base64_decode(&public_key)?, RsaKeyFormat::Pkcs1);
        match self.config.sign_type.as_str() {
            constants::SIGN_TYPE_RSA2 => {
                rsa_encryptor.verify(content.as_bytes(), &base64_decode(signature)?, HashType::Sha256)
            }
            constants::SIGN_TYPE_RSA => {
                rsa_encryptor.verify(content.as_bytes(), &base64_decode(signature)?, HashType::Sha1)
            }
            _ => Err(LabraError::Sign("不支持的签名类型".to_string())),
        }
    }

    /// 发送请求（内部实现）
    async fn request_internal<T, B, R>(
        &self,
        request: T,
        access_token: Option<String>,
        app_auth_token: Option<String>,
        target_app_id: Option<String>,
    ) -> LabradorResult<AlipayResponse<R>>
    where
        T: AlipayRequest<B>,
        B: Serialize,
        R: for<'de> Deserialize<'de>,

    {
        let method = request.get_api_method();
        // 构建请求参数
        let mut holder = RequestParametersHolder::new();

        // 设置应用参数
        let mut app_params = request.get_text_params();
        // 处理加密
        if let Some(biz_content_str) = app_params.get(constants::BIZ_CONTENT_KEY) {
            if !biz_content_str.is_empty() && self.config.encrypt_key.is_some() && request.is_need_encrypt() {
                let encrypted = self.encrypt_data(biz_content_str)?;
                app_params.insert(constants::BIZ_CONTENT_KEY.to_string(), encrypted);
            }
        }

        // 添加应用授权令牌
        if let Some(auth_token) = app_auth_token {
            app_params.insert(constants::APP_AUTH_TOKEN.to_string(), auth_token);
        }

        holder.set_application_params(app_params);

        // 设置协议必须参数
        let mut protocol_must_params = BTreeMap::new();
        protocol_must_params.insert(constants::METHOD.to_string(), method.as_str().to_string());
        protocol_must_params.insert(constants::VERSION.to_string(), request.get_api_version());
        protocol_must_params.insert(constants::APP_ID.to_string(), self.config.app_id.clone());
        protocol_must_params.insert(constants::SIGN_TYPE.to_string(), self.config.sign_type.clone());
        protocol_must_params.insert(constants::CHARSET.to_string(), self.config.charset.clone());
        protocol_must_params.insert(constants::TIMESTAMP.to_string(),
                                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());

        // 添加证书序列号
        if self.config.use_cert {
            if let Some(ref app_cert_sn) = self.app_cert_sn {
                protocol_must_params.insert(constants::APP_CERT_SN.to_string(), app_cert_sn.clone());
            }
            if let Some(ref root_cert_sn) = self.alipay_root_cert_sn {
                protocol_must_params.insert(constants::ALIPAY_ROOT_CERT_SN.to_string(), root_cert_sn.clone());
            }
        }

        // 添加加密类型
        if self.config.encrypt_key.is_some() {
            protocol_must_params.insert(constants::ENCRYPT_TYPE.to_string(), self.config.encrypt_type.clone());
        }

        holder.set_protocal_must_params(protocol_must_params);

        // 设置协议可选参数
        let mut protocal_opt_params = BTreeMap::new();
        protocal_opt_params.insert(constants::FORMAT.to_string(), self.config.format.clone());

        if let Some(token) = access_token {
            protocal_opt_params.insert(constants::ACCESS_TOKEN.to_string(), token);
        }

        if let Some(target_id) = target_app_id {
            protocal_opt_params.insert(constants::TARGET_APP_ID.to_string(), target_id);
        }

        holder.set_protocal_opt_params(protocal_opt_params);

        // 生成签名
        let sign_content = holder.get_signature_content();
        let sign = self.sign(&sign_content)?;
        holder.protocol_must_params.insert(constants::SIGN.to_string(), sign);

        // 构建请求URL
        let url = self.build_request_url(&holder);
        // 发送请求
        let request = Request::builder()
            .method(HttpMethod::Post)
            .path(&url)
            .body(RequestBody::form(holder.application_params))
            .build();

        let response = self.http_client.request(request).await?;
        let response_text = response.text()?;

        // 解析响应
        self.parse_response::<R>(&response_text, method).await
    }

    /// 发送请求
    pub async fn request<R, T, B>(
        &self,
        request: T,
        access_token: Option<String>,
        app_auth_token: Option<String>,
        target_app_id: Option<String>,
    ) -> LabradorResult<AlipayResponse<R>>
    where
        R: for<'de> Deserialize<'de>,
        T: AlipayRequest<B>,
        B: Serialize,
    {
        self.request_internal(request, access_token, app_auth_token, target_app_id).await
    }

    /// 构建请求URL
    fn build_request_url(&self, holder: &RequestParametersHolder) -> String {
        let mut params = BTreeMap::new();
        params.extend(holder.protocol_must_params.clone());
        params.extend(holder.protocol_opt_params.clone());

        let query_string = params.iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<String>>()
            .join("&");

        format!("{}?{}", self.http_client.config().api_base_url, query_string)
    }

    /// 解析响应
    async fn parse_response<T>(
        &self,
        response_text: &str,
        method: &AlipayMethod,
    ) -> LabradorResult<AlipayResponse<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        use serde_json::Value;

        let json_value: Value = serde_json::from_str(response_text)
            .map_err(|e| LabraError::Json(e))?;

        // 检查错误响应
        if let Some(error_response) = json_value.get("error_response") {
            let error: AlipayResponse<Value> = serde_json::from_value(error_response.clone())
                .map_err(|e| LabraError::Json(e))?;

            return Err(LabraError::business(
                error.sub_code.unwrap_or(error.code.clone()),
                error.sub_msg.unwrap_or(error.msg.clone()),
            ));
        }

        // 获取正常响应
        let response_key = method.response_key();
        if let Some(response_value) = json_value.get(&response_key) {
            let response: AlipayResponse<T> = serde_json::from_value(response_value.clone())
                .map_err(|e| LabraError::Json(e))?;
            // 获取签名和证书序列号
            let sign = json_value.get("sign").and_then(|v| v.as_str()).map(|s| s.to_string());
            let alipay_cert_sn = json_value.get("alipay_cert_sn").and_then(|v| v.as_str()).map(|s| s.to_string());

            // 如果是证书模式且需要下载证书
            if self.config.use_cert && alipay_cert_sn.is_some() && sign.is_some() {
                if let Some(cert_sn) = &alipay_cert_sn {
                    if self.get_alipay_public_key(cert_sn)?.is_none() {
                        // 尝试自动下载证书并重新验证
                        let cert_future = self.auto_load_cert(cert_sn.to_string());
                        if let Some(_public_key) = cert_future.await? {
                        }
                    }
                }
            }

            // 验证签名
            if let Some(sign) = &sign {
                let verify_result = self.verify(
                    &self.extract_response_raw_json(response_text, &response_key)?,
                    sign,
                    alipay_cert_sn.as_deref(),
                )?;

                if !verify_result {
                    return Err(LabraError::Sign("签名验证失败".to_string()));
                }
            }

            Ok(response)
        } else {
            Err(LabraError::Other(format!("未找到响应键: {}", response_key)))
        }
    }

    fn extract_response_raw_json(&self, response_text: &str, key: &str) -> LabradorResult<String> {
        // 使用非贪婪匹配，并假设 JSON 值是第一个完整的对象
        let pattern = format!(r#""{}"\s*:\s*(\{{.*?\}})"#, regex::escape(key));
        let re = regex::Regex::new(&pattern)
            .map_err(|e| LabraError::Other(format!("正则构建失败: {}", e)))?;

        // 尝试查找所有匹配
        let captures: Vec<regex::Captures> = re.captures_iter(response_text).collect();

        if !captures.is_empty() {
            // 取第一个匹配（通常就是我们要的）
            if let Some(matched) = captures[0].get(1) {
                let extracted = matched.as_str();

                // 验证提取的内容是否是有效的 JSON
                if serde_json::from_str::<serde_json::Value>(extracted).is_ok() {
                    return Ok(extracted.to_string());
                } else {
                    warn!("警告: 提取的内容不是有效 JSON，尝试其他匹配");
                }
            }
        }

        // 如果正则提取失败或无效，回退到原始方法
        warn!("警告: 无法从原始响应提取 {}，使用重新序列化的值，验签可能失败", key);
        let json_value: serde_json::Value = serde_json::from_str(response_text)?;
        let response_value = json_value.get(key)
            .ok_or_else(|| LabraError::Other(format!("找不到 key: {}", key)))?;
        serde_json::to_string(response_value)
            .map_err(|e| LabraError::Json(e).into())
    }

    /// 执行页面支付请求（同步）
    pub fn execute_page_request<T: AlipayRequest<B>, B: Serialize>(
        &self,
        request: T,
        http_method: Option<&str>,
    ) -> LabradorResult<String> {
        let method = request.get_api_method();
        let mut holder = RequestParametersHolder::new();
        let biz_content = request.get_text_params();
        // 设置应用参数
        holder.set_application_params(biz_content);

        // 设置协议必须参数
        let mut protocal_must_params = BTreeMap::new();
        protocal_must_params.insert(constants::METHOD.to_string(), method.as_str().to_string());
        protocal_must_params.insert(constants::VERSION.to_string(), "1.0".to_string());
        protocal_must_params.insert(constants::APP_ID.to_string(), self.config.app_id.clone());
        protocal_must_params.insert(constants::SIGN_TYPE.to_string(), self.config.sign_type.clone());
        protocal_must_params.insert(constants::CHARSET.to_string(), self.config.charset.clone());
        protocal_must_params.insert(constants::TIMESTAMP.to_string(),
                                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());

        holder.set_protocal_must_params(protocal_must_params);

        // 设置协议可选参数
        let mut protocal_opt_params = BTreeMap::new();
        protocal_opt_params.insert(constants::FORMAT.to_string(), self.config.format.clone());
        holder.set_protocal_opt_params(protocal_opt_params);

        // 生成签名
        let sign_content = holder.get_signature_content();
        let sign = self.sign(&sign_content)?;
        holder.protocol_must_params.insert(constants::SIGN.to_string(), sign);

        // 构建URL
        let url = if http_method.unwrap_or("POST").to_uppercase() == "GET" {
            self.build_redirect_url(&holder)
        } else {
            self.build_form_url(&holder)
        };

        Ok(url)
    }

    /// 构建重定向URL
    fn build_redirect_url(&self, holder: &RequestParametersHolder) -> String {
        let sorted_params = holder.get_sorted_map();
        let query_string = sorted_params.iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<String>>()
            .join("&");

        format!("{}?{}", self.http_client.config().api_base_url, query_string)
    }

    /// 构建表单URL
    fn build_form_url(&self, holder: &RequestParametersHolder) -> String {
        let url = self.build_request_url(holder);
        let params = holder.application_params.clone();

        let hidden_fields = params.iter()
            .filter(|(k, v)| !k.is_empty() && !v.is_empty())
            .map(|(k, v)| {
                let escaped = v.replace("\"", "&quot;");
                format!("<input type=\"hidden\" name=\"{}\" value=\"{}\">", k, escaped)
            })
            .collect::<Vec<String>>()
            .join("\n");

        format!(
            r#"<form name="punchout_form" method="post" action="{}">
{}
<input type="submit" value="立即支付" style="display:none">
</form>
<script>document.forms[0].submit();</script>"#,
            url, hidden_fields
        )
    }

    /// 解析支付通知
    pub fn parse_payment_notify(&self, notify_data: &str) -> LabradorResult<AlipayNotifyResponse> {
        use url::form_urlencoded;

        let params: BTreeMap<String, String> = form_urlencoded::parse(notify_data.as_bytes())
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect();

        // 获取签名
        let sign = params.get("sign")
            .ok_or_else(|| LabraError::Sign("缺少签名参数".to_string()))?;
        let _sign_type = params.get("sign_type")
            .unwrap_or(&self.config.sign_type)
            .clone();

        // 构建验签字符串
        let mut sign_params = params.clone();
        sign_params.remove("sign");
        sign_params.remove("sign_type");

        let sign_content = sign_params.iter()
            .filter(|(_, v)| !v.is_empty())
            .map(|(k, v)| format!("{}={}", k, urlencoding::decode(v).unwrap_or_default().to_string()))
            .collect::<Vec<String>>()
            .join("&");

        // 验证签名
        let verify_result = self.verify(&sign_content, sign, None)?;
        if !verify_result {
            return Err(LabraError::Sign("签名验证失败".to_string()));
        }

        // 解析通知数据
        let notify: AlipayNotifyResponse = serde_json::from_value(
            serde_json::to_value(&params).map_err(|e| LabraError::Json(e))?
        ).map_err(|e| LabraError::Json(e))?;

        Ok(notify)
    }

    /// 获取配置
    pub fn config(&self) -> &AlipayClientConfig {
        &self.config
    }

    /// 获取HTTP客户端
    pub fn http_client(&self) -> &ApiClient {
        &self.http_client
    }

    /// 获取支付宝支付服务
    pub fn alipay_service(&self) -> AlipayPayService<'_> {
        AlipayPayService::new(&self)
    }

    /// 获取支付宝小程序服务
    pub fn alipay_mining_service(&self) -> AlipayMiniappService<'_> {
        AlipayMiniappService::new(&self)
    }

    /// 获取支付宝开放平台服务
    pub fn alipay_open_service(&self) -> AlipayOpenService<'_> {
        AlipayOpenService::new(&self)
    }
}