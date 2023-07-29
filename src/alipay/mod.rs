use chrono::Local;
use crate::{client::{APIClient}, request::{RequestType, Method, LabraRequest}, errors::LabraError, session::{SimpleStorage, SessionStore}, RequestMethod, LabradorResult, get_nonce_str, RequestParametersHolder, cfg_if};

use std::collections::{BTreeMap};
use std::fs;
use std::sync::Arc;
use dashmap::{DashMap};
use serde::Serialize;

use crate::alipay::method::AlipayMethod;

cfg_if! {if #[cfg(feature = "openssl-crypto")]{
    use openssl::hash::{hash, MessageDigest};
    use openssl::nid::Nid;
    use openssl::x509::{X509, X509NameEntries};
}}


cfg_if! {if #[cfg(not(feature = "openssl-crypto"))]{
    use x509_parser::parse_x509_certificate;
    use x509_parser::x509::X509Name;
}}

mod request;
mod response;
mod method;
#[allow(unused)]
mod constants;

pub use request::*;
pub use response::*;

use crate::alipay::constants::{ENCRYPT_TYPE_AES, FORMAT_JSON, SIGN_TYPE_RSA2};
use crate::md5::md5;
use crate::prp::PrpCrypto;

#[derive(Debug, Clone)]
pub struct AlipayClient<T: SessionStore> {
    api_client: APIClient<T>,
    /// 加密类型
    encrypt_type: String,
    /// 是否使用证书模式
    use_cert: bool,
    /// 格式类型
    format: String,
    /// 签名类型
    sign_type: String,
    charset: String,
    /// 加密KEY
    encrypt_key: Option<String>,
    /// 应用私钥
    private_key: Option<String>,
    /// 支付宝公钥
    alipay_public_key: Option<String>,
    /// 应用公钥证书路径
    app_cert: Option<String>,
    /// 设置支付宝公钥证书路径
    alipay_public_cert: Option<String>,
    /// 设置支付宝根证书路径
    alipay_root_cert: Option<String>,
    /// 缓存的公钥证书文件
    cache_certs: Arc<DashMap<String, String>>,

}


pub trait AlipayResponse {

    fn set_sub_code(&mut self, sub_code: String);

    fn set_code(&mut self, code: String);

    ///
    /// 获取公共得响应
    ///
    fn get_body(&self) -> String;

    fn set_body(&mut self, body: String);

    /// 返回码
    fn get_sub_code(&self) -> String;

    /// 返回码
    fn get_code(&self) -> String;

    /// 签名
    fn get_sign(&self) -> String;

    /// 支付宝公钥证书
    fn get_alipay_cert_sn(&self) -> String;

    /// 响应是否成功
    fn is_success(&self) -> bool {
        self.get_sub_code().is_empty()
    }

    fn set_msg(&mut self, msg: String);

    fn set_sub_msg(&mut self, sub_msg: String);

    fn get_sub_msg(&self) -> String;

    fn get_msg(&self) -> String;
}


pub trait AlipayRequest<T: Serialize> {
    ///
    /// 获取TOP的API名称。
    ///
    /// @return API名称
    fn get_api_method_name(&self) -> AlipayMethod;

    ///
    /// 获取所有的Key-Value形式的文本请求参数集合。其中：
    /// <ul>
    /// <li>Key: 请求参数名</li>
    /// <li>Value: 请求参数值</li>
    /// </ul>
    ///
    /// @return 文本请求参数集合
    fn get_text_params(&self) -> BTreeMap<String, String> {
        BTreeMap::default()
    }

    ///
    /// 得到当前接口的版本
    ///
    /// @return API版本
    fn get_api_version(&self) -> String;



    ///
    /// 获取终端类型
    ///
    /// @return 终端类型
    fn get_terminal_type(&self) -> String;

    ///
    /// 获取终端信息
    ///
    /// @return 终端信息
    fn get_terminal_info(&self) -> String;

    ///
    /// 获取产品码
    ///
    /// @return 产品码
    fn get_prod_code(&self) -> String;

    ///
    /// 返回通知地址
    ///
    /// @return
    fn get_notify_url(&self) -> String;

    ///
    /// 返回回跳地址
    ///
    /// @return
    fn get_return_url(&self) -> String;

    ///
    /// 判断是否需要加密
    ///
    /// @return
    fn is_need_encrypt(&self) -> bool;

    fn get_biz_content(self) -> String;

    fn get_biz_model(&self) -> Option<&T>{
        None
    }
}

/// # AlipayClient
/// 
/// 
/// # Example
/// 
/// ```no_run
/// # use labrador::AlipayClient;
/// fn main() {
///     let client = AlipayClient::new("appKey", false);
///     // Do Some Thing You Want
///     // ...
/// }
/// ```
/// 
#[allow(unused)]
impl <T: SessionStore> AlipayClient<T> {

    pub fn new<Q: Into<String> + Clone>(app_key: Q, sandbox: bool) -> AlipayClient<SimpleStorage> {
        let url = if sandbox {
            "https://openapi.alipaydev.com/gateway.do"
        } else {
            "https://openapi.alipay.com/gateway.do"
        };
        let api_client = APIClient::<SimpleStorage>::new::<Q, String, String>(app_key.clone(), "".to_string(), url.to_owned());
        AlipayClient {
            api_client,
            encrypt_type: ENCRYPT_TYPE_AES.to_string(),
            use_cert: false,
            format: FORMAT_JSON.to_string(),
            sign_type: SIGN_TYPE_RSA2.to_string(),
            charset: constants::CHARSET_UTF8.to_string(),
            encrypt_key: None,
            private_key: None,
            alipay_public_key: None,
            app_cert: None,
            alipay_public_cert: None,
            alipay_root_cert: None,
            cache_certs: Arc::new(DashMap::new())
        }
    }

    pub fn from_session<Q: Into<String> + Clone>(app_key: Q, session: T, sandbox: bool) -> AlipayClient<T> {
        let url = if sandbox {
            "https://openapi.alipaydev.com/gateway.do"
        } else {
            "https://openapi.alipay.com/gateway.do"
        };
        AlipayClient {
            api_client: APIClient::from_session(app_key.clone(), "", url.to_string(), session),
            encrypt_type: ENCRYPT_TYPE_AES.to_string(),
            charset: constants::CHARSET_UTF8.to_string(),
            sign_type: SIGN_TYPE_RSA2.to_string(),
            format: FORMAT_JSON.to_string(),
            encrypt_key: None,
            private_key: None,
            alipay_public_key: None,
            app_cert: None,
            alipay_public_cert: None,
            alipay_root_cert: None,
            use_cert: false,
            cache_certs: Arc::new(DashMap::new())
        }
    }

    /// 获取应用证书SN
    pub fn get_app_cert_sn(&self) -> LabradorResult<String> {
        let pem = self.app_cert.to_owned().unwrap_or_default();
        #[cfg(not(feature = "openssl-crypto"))]
        fn get_cert_sn(pem: &[u8]) -> LabradorResult<String> {
            let (data, x509) = x509_parser::pem::parse_x509_pem(pem)?;
            let cert = x509.parse_x509()?;
            let issuer = iter2string(cert.issuer())?;
            let serial_number = cert.serial.to_string();
            let data = issuer + &serial_number;
            let app_cert_sn = md5(data);
            Ok(app_cert_sn)
        }

        #[cfg(feature = "openssl-crypto")]
        fn get_cert_sn(pem: &[u8]) -> LabradorResult<String> {
            let x509 = X509::from_pem(pem)?;
            let issuer = iter2string(x509.issuer_name().entries())?;
            let serial_number = x509.serial_number().to_bn()?.to_dec_str()?;
            let data = issuer + &serial_number;
            Ok(md5(data))
        }
        get_cert_sn(pem.as_bytes())
    }

    /// 获取根证书SN
    pub fn get_root_cert_sn(&self) -> LabradorResult<String> {
        let pem = self.alipay_root_cert.to_owned().unwrap_or_default();
        #[cfg(not(feature = "openssl-crypto"))]
        fn get_cert_sn(pem: &[u8]) -> LabradorResult<String> {
            let mut sns = Vec::new();
            for pem in x509_parser::pem::Pem::iter_from_buffer(pem) {
                match pem {
                    Ok(pem) => {
                        let cert = pem.parse_x509()?;
                        let algorithm = cert.signature_algorithm.oid().to_string();
                        if algorithm.starts_with("1.2.840.113549.1.1") {
                            continue;
                        }
                        let issuer = iter2string(cert.issuer())?;
                        let serial_number = cert.serial.to_string();
                        let data = issuer + &serial_number;
                        sns.push(md5(data));
                    }
                    Err(e) => {
                        tracing::error!("Error while decoding PEM entry {:?}", e);
                    }
                }
            }
            Ok(sns.join("_"))
        }

        #[cfg(feature = "openssl-crypto")]
        fn get_cert_sn(pem: &[u8]) -> LabradorResult<String> {
            let x509s = X509::stack_from_pem(pem)?;
            let alipay_root_cert_sn = x509s.iter().filter(|x509| {
                let algorithm = x509.signature_algorithm().object().nid();
                algorithm == Nid::SHA256WITHRSAENCRYPTION || algorithm == Nid::SHA1WITHRSAENCRYPTION
            }).map(|x509| {
                let issuer = iter2string(x509.issuer_name().entries())?;
                let serial_number = x509.serial_number().to_bn()?.to_dec_str()?;
                let data = issuer + &serial_number;
                Ok( hex::encode(hash(MessageDigest::md5(), data.as_ref())?))
            }).map(|cert: LabradorResult<String>| cert.unwrap_or_default()).collect::<Vec<String>>().join("_");
            Ok(alipay_root_cert_sn)
        }

        get_cert_sn(pem.as_bytes())

    }

    /// 初始化支付宝公钥
    pub fn init_alipay_public_cert(&self) -> LabradorResult<()> {
        if !self.cache_certs.is_empty() {
            return Ok(())
        }
        let pem = self.alipay_public_cert.to_owned().unwrap_or_default();
        #[cfg(not(feature = "openssl-crypto"))]
        fn get_cert_sn(pem: &[u8]) -> LabradorResult<Vec<(String, String)>> {
            let mut resp = Vec::new();
            for pem in x509_parser::pem::Pem::iter_from_buffer(pem) {
                match pem {
                    Ok(pem) => {
                        let (_, cert) = parse_x509_certificate(&pem.contents)?;
                        let algorithm = cert.signature_algorithm.oid().to_string();
                        let public_key = cert.public_key();
                        let issuer = iter2string(cert.issuer())?;
                        let serial_number = cert.serial.to_string();
                        let data = issuer + &serial_number;
                        let sn = md5(data);
                        resp.push((sn, base64::encode(public_key.raw)));
                    }
                    Err(e) => {
                        tracing::error!("Error while decoding PEM entry {:?}", e);
                    }
                }
            }
            Ok(resp)
        }

        #[cfg(feature = "openssl-crypto")]
        fn get_cert_sn(pem: &[u8]) -> LabradorResult<Vec<(String, String)>> {
            let x509s = X509::stack_from_pem(pem)?;
            let mut resp = Vec::new();
            for x509 in x509s.iter() {
                let issuer = iter2string(x509.issuer_name().entries())?;

                let serial_number = x509.serial_number().to_bn()?.to_dec_str()?;
                let data = issuer + &serial_number;
                let sn = md5(data);
                let pk = x509.public_key()?;
                let rpk = pk.public_key_to_pem()?;
                resp.push((sn, base64::encode(&rpk)));
            }
            Ok(resp)
        }
        for (sn, cert) in get_cert_sn(pem.as_bytes())? {
            self.cache_certs.insert(sn, cert);
        }
        Ok(())
    }

    /// 初始化支付宝公钥
    pub fn get_alipay_public_key(&self) -> LabradorResult<String> {
        let pem = self.alipay_public_cert.to_owned().unwrap_or_default();
        #[cfg(not(feature = "openssl-crypto"))]
        fn get_cert(pem: &[u8]) -> LabradorResult<(String, String)> {
            let (r, pem) = x509_parser::pem::parse_x509_pem(pem)?;
            let (_, cert) = parse_x509_certificate(&pem.contents)?;
            let public_key = cert.public_key();
            let issuer = iter2string(cert.issuer())?;
            let serial_number = cert.serial.to_string();
            let data = issuer + &serial_number;
            let sn = md5(data);
            Ok((sn, base64::encode(public_key.raw)))
        }

        #[cfg(feature = "openssl-crypto")]
        fn get_cert(pem: &[u8]) -> LabradorResult<(String, String)> {
            let x509 = X509::from_pem(pem)?;
            let pk = x509.public_key()?;
            let issuer = iter2string(x509.issuer_name().entries())?;

            let serial_number = x509.serial_number().to_bn()?.to_dec_str()?;
            let data = issuer + &serial_number;
            let sn = md5(data);
            Ok((sn, base64::encode(pk.public_key_to_pem()?)))
        }
        let (sn, pk) = get_cert(pem.as_bytes())?;
        if let Some(key) = self.cache_certs.get(&sn) {
            let v = key.value();
            Ok(v.to_string())
        } else {
            self.cache_certs.insert(sn, pk.to_string());
            Ok(pk.to_string())
        }
    }

    /// 设置应用私钥
    pub fn set_private_key_path(mut self, private_key: &str) -> LabradorResult<Self> {
        if private_key.is_empty() {
            return Err(LabraError::InvalidSignature("私钥文件有误！".to_string()));
        }
        let content = fs::read_to_string(private_key)?;
        self.private_key = content.into();
        Ok(self)
    }
    pub fn set_private_key(mut self, private_key: &str) -> LabradorResult<Self> {
        if private_key.is_empty() {
            return Err(LabraError::InvalidSignature("私钥内容有误！".to_string()));
        }
        self.private_key = private_key.to_string().into();
        Ok(self)
    }

    /// 设置加密类型
    pub fn set_encrypt_type(mut self, encrypt_type: &str) -> Self {
        self.encrypt_type = encrypt_type.to_string().into();
        self
    }

    /// 设置格式类型
    pub fn set_format(mut self, format: &str) -> Self {
        self.format = format.to_string().into();
        self
    }

    /// 设置是否使用证书
    pub fn use_cert(mut self, use_cert: bool) -> Self {
        self.use_cert = use_cert;
        self
    }

    /// 设置加密KEY
    pub fn set_encrypt_key(mut self, encrypt_key: &str) -> Self {
        self.encrypt_key = encrypt_key.to_string().into();
        self
    }

    pub fn set_alipay_public_key(mut self, alipay_public_key: &str) -> Self {
        self.alipay_public_key = alipay_public_key.to_string().into();
        self
    }

    pub fn set_sign_type(mut self, sign_type: &str) -> Self {
        self.sign_type = sign_type.to_string().into();
        self
    }

    pub fn set_charset(mut self, charset: &str) -> Self {
        self.charset = charset.to_string().into();
        self
    }

    /// 设置APP证书路径
    pub fn set_app_cert_path(mut self, cert_path: &str) -> LabradorResult<Self> {
        if cert_path.is_empty() {
            return Err(LabraError::InvalidSignature("证书文件有误！".to_string()));
        }
        let content = fs::read_to_string(cert_path)?;
        self.app_cert = content.into();
        Ok(self)
    }
    pub fn set_app_cert(mut self, cert: &str) -> Self {
        self.app_cert = cert.to_string().into();
        self
    }

    /// 设置阿里公钥证书路径
    pub fn set_alipay_public_cert_path(mut self, cert_path: &str) -> LabradorResult<Self> {
        if cert_path.is_empty() {
            return Err(LabraError::InvalidSignature("证书文件有误！".to_string()));
        }
        let content = fs::read_to_string(cert_path)?;
        self.alipay_public_cert = content.into();
        Ok(self)
    }
    pub fn set_alipay_public_cert(mut self, cert: &str) -> Self {
        self.alipay_public_cert = cert.to_string().into();
        self
    }

    /// 设置阿里根证书路径
    pub fn set_alipay_root_cert_path(mut self, cert_path: &str) -> LabradorResult<Self> {
        if cert_path.is_empty() {
            return Err(LabraError::InvalidSignature("证书文件有误！".to_string()));
        }
        let content = fs::read_to_string(cert_path)?;
        self.alipay_root_cert = content.into();
        Ok(self)
    }

    pub fn set_alipay_root_cert(mut self, cert: &str) -> Self {
        self.alipay_root_cert = cert.to_string().into();
        self
    }


    /// 签名
    fn sign(&self, params: &str) -> LabradorResult<String> {
        let private_key = self.private_key.to_owned().unwrap_or_default();
        let content = base64::decode(&private_key)?;
        let sign = PrpCrypto::rsa_sha256_sign(params, &private_key)?;
        Ok(sign)
    }

    /// 验签
    fn verify(&self, source: &str, signature: &str, cert: Option<&String>) -> LabradorResult<bool> {
        let mut public_key = cert.map(|v| v.to_string()).unwrap_or_default();
        if public_key.is_empty() {
            if self.alipay_public_cert.is_some() && self.use_cert {
                public_key = self.get_alipay_public_key()?;
            } else {
                public_key = self.alipay_public_key.to_owned().unwrap_or_default();
            }
        }
        let _ = PrpCrypto::rsa_sha256_verify(&public_key, source, signature)?;
        Ok(true)
    }

    fn get_redirect_url<>(&self, holder: &RequestParametersHolder) -> LabradorResult<String> {
        let url_sb = self.api_client.api_path.to_owned();
        let params = holder.get_sorted_map();
        let param = serde_urlencoded::to_string(params)?;
        Ok(format!("{}?{}", url_sb, param))
    }

    fn get_request_url(&self, holder: &RequestParametersHolder) -> LabradorResult<String> {
        let mut url_sb = self.api_client.api_path.to_owned();
        let sys_must = &holder.protocal_must_params;
        let sys_must_query = serde_urlencoded::to_string(sys_must)?;
        let opt_param = &holder.protocal_opt_params;
        let sys_opt_query = serde_urlencoded::to_string(opt_param)?;
        url_sb += "?";
        url_sb += &sys_must_query;
        if !sys_opt_query.is_empty() {
            url_sb += "&";
            url_sb += &sys_opt_query;
        }
        Ok(url_sb)
    }

    /// 拼装sdk调用时所传参数
    fn get_sdk_params(&self, holder: &RequestParametersHolder) -> LabradorResult<String> {
        let mut url_sb = String::default();
        let sorted_params = holder.get_sorted_map();
        let sort_sql = serde_urlencoded::to_string(sorted_params)?;
        url_sb.push_str(&sort_sql);
        Ok(url_sb)
    }

    ///
    /// 组装接口参数，处理加密、签名逻辑
    ///
    /// @param request
    /// @param accessToken
    /// @param appAuthToken
    /// @param appCertSN    应用证书序列号
    fn get_request_holder_with_sign<D, M>(&self, request: D, access_token: Option<String>, app_auth_token: Option<String>, target_app_id: Option<String>) -> LabradorResult<RequestParametersHolder> where D: AlipayRequest<M>, M: Serialize {
        let mut holder = RequestParametersHolder::new();
        let mut app_params = request.get_text_params();
        let empty_str = &"".to_string();
        // 仅当API包含biz_content参数且值为空时，序列化bizModel填充bizContent
        let biz_content = app_params.get(constants::BIZ_CONTENT_KEY).unwrap_or_else(|| empty_str);
        let biz_model = request.get_biz_model();
        if biz_content.is_empty() && biz_model.is_some() {
            let biz_model = biz_model.unwrap();
            app_params.insert(constants::BIZ_CONTENT_KEY.to_string(), serde_json::to_string(&biz_model)?);
        }
        // 只有新接口和设置密钥才能支持加密
        if request.is_need_encrypt() {
            let biz_content = app_params.get(constants::BIZ_CONTENT_KEY).unwrap_or_else(|| empty_str);
            if biz_content.is_empty() {
                return Err(LabraError::ApiError("当前API不支持加密请求".to_string()))
            }

            // 需要加密必须设置密钥和加密算法
            if self.encrypt_type.is_empty() || self.encrypt_key.is_none() {
                return Err(LabraError::ApiError("API请求要求加密，则必须设置密钥类型[encryptType]和加密密钥[encryptKey]".to_string()))
            }
            let key = self.encrypt_key.to_owned().unwrap_or_default();
            let prp = PrpCrypto::new(key.into_bytes());
            let encrypt_content = prp.aes_128_cbc_encrypt_data(biz_content, Some(&get_nonce_str()))?;
            let encrypt_content = String::from_utf8_lossy(&encrypt_content).to_string();
            app_params.insert(constants::BIZ_CONTENT_KEY.to_string(), encrypt_content);
        }

        if let Some(app_auth_token) = app_auth_token {
            app_params.insert(constants::APP_AUTH_TOKEN.to_string(), app_auth_token);
        }

        holder.set_application_params(app_params);

        let mut protocal_must_params = BTreeMap::new();
        protocal_must_params.insert(constants::METHOD.to_string(), request.get_api_method_name().get_method());
        protocal_must_params.insert(constants::VERSION.to_string(), request.get_api_version());
        protocal_must_params.insert(constants::APP_ID.to_string(), self.api_client.app_key.to_owned());
        protocal_must_params.insert(constants::SIGN_TYPE.to_string(), self.sign_type.to_string());
        if !request.get_terminal_type().is_empty() {
            protocal_must_params.insert(constants::TERMINAL_TYPE.to_string(), request.get_terminal_type());
        }
        if !request.get_terminal_info().is_empty() {
            protocal_must_params.insert(constants::TERMINAL_INFO.to_string(), request.get_terminal_info());
        }
        if !request.get_notify_url().is_empty() {
            protocal_must_params.insert(constants::NOTIFY_URL.to_string(), request.get_notify_url());
        }
        if !request.get_return_url().is_empty() {
            protocal_must_params.insert(constants::RETURN_URL.to_string(), request.get_return_url());
        }

        protocal_must_params.insert(constants::CHARSET.to_string(), self.charset.to_string());

        if let Some(target_app_id) = target_app_id {
            protocal_must_params.insert(constants::TARGET_APP_ID.to_string(), target_app_id);
        }

        if request.is_need_encrypt() {
            protocal_must_params.insert(constants::ENCRYPT_TYPE.to_string(), self.encrypt_type.to_string());
        }
        //如果应用证书序列号非空，添加应用证书序列号
        if let Some(_) = &self.app_cert {
            let app_cert_sn = self.get_app_cert_sn()?;
            if !app_cert_sn.is_empty() {
                protocal_must_params.insert(constants::APP_CERT_SN.to_string(), app_cert_sn);
            }
        }
        //如果根证书序列号非空，添加根证书序列号
        if let Some(_) = &self.alipay_root_cert {
            let root_cert_sn = self.get_root_cert_sn()?;
            if !root_cert_sn.is_empty() {
                protocal_must_params.insert(constants::ALIPAY_ROOT_CERT_SN.to_string(), root_cert_sn);
            }
        }
        // TODO: 如果SM2根证书序列号非空，添加SM2根证书序列号
        protocal_must_params.insert(constants::TIMESTAMP.to_string(), Local::now().naive_local().format(constants::FORMAT_TIME).to_string());
        holder.set_protocal_must_params(protocal_must_params.to_owned());

        let mut protocal_opt_params = BTreeMap::new();
        protocal_opt_params.insert(constants::FORMAT.to_string(), self.format.to_string());
        if let Some(access_token) = access_token {
            protocal_opt_params.insert(constants::ACCESS_TOKEN.to_string(), access_token);
        }
        if !request.get_prod_code().is_empty() {
            protocal_opt_params.insert(constants::PROD_CODE.to_string(), request.get_prod_code());
        }
        holder.set_protocal_opt_params(protocal_opt_params);
        if !self.sign_type.is_empty() {
            let sign_content = holder.get_signature_content();
            protocal_must_params.insert(constants::SIGN.to_string(), self.sign_with_type(&sign_content)?);
        } else {
            protocal_must_params.insert(constants::SIGN.to_string(), "".to_string());
        }
        holder.set_protocal_must_params(protocal_must_params);
        Ok(holder)
    }

    /// 签名
    fn sign_with_type(&self, sign_content: &str) -> LabradorResult<String> {
        match self.sign_type.as_str() {
            constants::SIGN_TYPE_RSA2 => {
                let private_key = self.private_key.to_owned().unwrap_or_default();
                let sign = PrpCrypto::rsa_sha256_sign(sign_content, &private_key)?;
                Ok(sign)
            }
            // constants::SIGN_TYPE_RSA => {
            //
            // }
            _ => return Err(LabraError::InvalidSignature("不支持的加密方式".to_string()))
        }
    }

    /// 自动加载对应证书
    pub async fn auto_load_cert(&self, alipay_cert_sn: &str) -> LabradorResult<String> {
        // 如果已经有证书了，则不用自动获取
        if self.cache_certs.is_empty() && self.use_cert {
            self.init_alipay_public_cert();
        }
        if let Some(cert) = self.cache_certs.get(alipay_cert_sn) {
            let data = cert.value();
            tracing::info!("获取内存中平台证书:{}", data);
            Ok(data.to_string())
        } else {
            let mut req = AlipayOpenAppAlipaycertDownloadRequest::new();
            let model = AlipayOpenAppAlipaycertDownloadModel { alipay_cert_sn: alipay_cert_sn.to_string() };
            req.biz_model = model.into();
            let method = req.get_api_method_name();
            let holder = self.get_request_holder_with_sign(req, None, None, None)?;
            let url = self.get_request_url(&holder)?;
            let req = LabraRequest::new().url(url).method(Method::Get).form(&holder.application_params).req_type(RequestType::Form);
            let result = self.api_client.request(req).await?.text()?;
            match AlipayBaseResponse::parse(&result, method) {
                Ok(mut resp) => {
                    let response = resp.get_biz_model::<AlipayOpenAppAlipaycertDownloadResponse>()?;
                    let content = response.alipay_cert_content;
                    tracing::info!("获取下载平台证书:{}", content);
                    self.cache_certs.insert(alipay_cert_sn.to_string(), content.to_string());
                    Ok(content)
                }
                Err(err) => Err(err)
            }
        }
    }

    /// 发送请求数据
    fn page_excute<D, M>(&self, mut http_method: &str, request: D)
        -> LabradorResult<AlipayBaseResponse> where D: AlipayRequest<M>, M: Serialize {
        if http_method.is_empty() {
            http_method = "POST";
        }
        let mut resp = AlipayBaseResponse::new();
        let holder = self.get_request_holder_with_sign(request, None, None, None)?;
        // 获取返回结果
        if http_method.to_uppercase().eq("GET") {
            let body = self.get_redirect_url(&holder)?;
            resp.set_body(body);
        } else {
            let url = self.get_request_url(&holder)?;
            let body = self.build_form(&url, &holder.application_params);
            resp.set_body(body);
        }
        Ok(resp)
    }

    /// 发送请求数据
    fn sdk_excute<D, M>(&self, request: D)
        -> LabradorResult<AlipayBaseResponse> where D: AlipayRequest<M>, M: Serialize {
        let mut resp = AlipayBaseResponse::new();
        let holder = self.get_request_holder_with_sign(request, None, None, None)?;
        // 获取返回结果
        let body = self.get_sdk_params(&holder)?;
        resp.set_body(body);
        Ok(resp)
    }

    /// 发送请求数据
    async fn excute<D, M>(&self, request: D, access_token: Option<String>, app_auth_token: Option<String>, target_app_id: Option<String>) -> LabradorResult<AlipayBaseResponse>
        where D: AlipayRequest<M>, M: Serialize {
        //如果根证书序列号非空，抛异常提示开发者使用certificateExecute
        if self.alipay_root_cert.is_some() || self.use_cert {
            return self.cert_excute(request, access_token, app_auth_token, target_app_id).await
        }
        let method = request.get_api_method_name();
        let holder = self.get_request_holder_with_sign(request, access_token, app_auth_token, target_app_id)?;
        let url = self.get_request_url(&holder)?;
        let req = LabraRequest::new().url(url).method(Method::Post).form(&holder.application_params).req_type(RequestType::Form);
        let result = self.api_client.request(req).await?.text()?;
        match AlipayBaseResponse::parse(&result, method) {
            Ok(mut resp) => {
                let sign = resp.get_sign();
                // 验签请求返回原始串
                if !sign.is_empty() || resp.is_success() {
                    let body = resp.body.to_owned().unwrap_or_default();
                    // 对body进行排序
                    let result = self.verify(&body, &sign, None)?;
                    if !result {
                        return Err(LabraError::InvalidSignature("sign check fail: check Sign and Data Fail!".to_string()))
                    }
                }
                Ok(resp)
            }
            Err(err) => Err(err)
        }
    }

    async fn cert_excute<D, M>(&self, request: D, access_token: Option<String>, app_auth_token: Option<String>, target_app_id: Option<String>) -> LabradorResult<AlipayBaseResponse>
        where D: AlipayRequest<M>, M: Serialize {
        let method = request.get_api_method_name();
        let holder = self.get_request_holder_with_sign(request, access_token, app_auth_token, target_app_id)?;
        let url = self.get_request_url(&holder)?;
        let req = LabraRequest::new().url(url).method(Method::Post).form(&holder.application_params).req_type(RequestType::Form);
        let result = self.api_client.request(req).await?.text()?;
        match AlipayBaseResponse::parse(&result, method) {
            Ok(mut resp) => {
                let sign = resp.get_sign();
                // 验签请求返回原始串
                if !sign.is_empty() || resp.is_success() {
                    let body = resp.body.to_owned().unwrap_or_default();
                    let alipay_cert_sn = resp.get_alipay_cert_sn();
                    if !alipay_cert_sn.is_empty() {
                        let cert: Option<String> = Some(self.auto_load_cert(&alipay_cert_sn).await.unwrap_or_default());
                        let result = self.verify(&body, &sign, cert.as_ref())?;
                        if !result {
                            return Err(LabraError::InvalidSignature("sign check fail: check Sign and Data Fail!".to_string()))
                        }
                    }


                }
                Ok(resp)
            }
            Err(err) => Err(err)
        }
    }

    fn build_form(&self, url: &str, parameters: &BTreeMap<String, String>) -> String {
        let mut form = String::from("<form name=\"punchout_form\" method=\"post\" action=\"");
        form.push_str(url);
        form.push_str("\">\n");
        form.push_str(&self.build_hidden_fields(parameters));

        form.push_str("<input type=\"submit\" value=\"立即支付\" style=\"display:none\" >\n");
        form.push_str("</form>\n");
        form.push_str("<script>document.forms[0].submit();</script>");
        form
    }

    fn build_hidden_fields(&self, parameters: &BTreeMap<String, String>) -> String {
        if parameters.is_empty() || parameters.len() == 0 {
            return "".to_string();
        }
        let mut param = String::default();
        for (k, v) in parameters.into_iter() {
            if k.is_empty() || v.is_empty() {
                continue;
            }
            param.push_str(&self.build_hidden_field(k, v));
        }
        param
    }

    fn build_hidden_field(&self, k: &str, v: &str) -> String {
        let mut param = String::from("<input type=\"hidden\" name=\"");
        param.push_str(k);
        param.push_str("\" value=\"");
        //转义双引号
        let a = v.replace("\"", "&quot;");
        param.push_str(&a);
        param.push_str("\">\n");
        param.to_string()
    }


    /// # 手机网站支付接口2.0
    /// 外部商户创建订单并支付
    /// [接口地址](https://opendocs.alipay.com/open/02ivbs?scene=21)
    /// # 示例
    /// ```no_run
    ///
    ///  # use labrador::AlipayClient;
    ///  # use labrador::AlipayTradeWapPayRequest;
    ///
    ///   # async fn main() {
    ///         let param = AlipayTradeWapPayRequest::default();
    ///         let client = AlipayClient::new("appKey", false);
    ///         match client.wap_pay("POST".into(), param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///   # }
    ///
    /// ```
    /// 
    pub fn wap_pay(&self, http_method: Option<&str>, req: AlipayTradeWapPayRequest<AlipayTradeWapPayModel>) -> LabradorResult<AlipayBaseResponse> {
        self.page_excute(http_method.unwrap_or("POST"), req)
    }

    /// # PC网站统一收单下单并支付页面接口
    /// PC场景下单并支付
    /// [接口地址](https://opendocs.alipay.com/open/028r8t?scene=22)
    /// # 示例
    /// ```no_run
    ///
    ///  # use labrador::AlipayClient;
    ///  # use labrador::AlipayTradePagePayRequest;
    ///
    ///   # async fn main() {
    ///         let param = AlipayTradePagePayRequest::default();
    ///         let client = AlipayClient::new("appKey", false);
    ///         match client.pc_pay("POST".into(), param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///   # }
    ///
    /// ```
    ///
    pub fn pc_pay(&self, http_method: Option<&str>, req: AlipayTradePagePayRequest<AlipayTradePagePayModel>) -> LabradorResult<AlipayBaseResponse> {
        self.page_excute(http_method.unwrap_or("POST"), req)
    }

    /// # app支付接口2.0
    /// 外部商户APP唤起快捷SDK创建订单并支付
    /// [接口地址](https://opendocs.alipay.com/open/02e7gq?scene=20)
    /// # 示例
    /// ```no_run
    ///
    ///  # use labrador::AlipayClient;
    ///  # use labrador::AlipayTradeAppPayRequest;
    ///
    ///   # fn main() {
    ///         let param = AlipayTradeAppPayRequest::default();
    ///         let client = AlipayClient::new("appKey", false);
    ///         match client.app_pay(param) {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///   # }
    ///
    /// ```
    ///
    pub fn app_pay(&self, req: AlipayTradeAppPayRequest<AlipayTradeAppPayModel>) -> LabradorResult<AlipayBaseResponse> {
        self.sdk_excute(req)
    }

    /// # 统一收单交易支付接口
    /// <pre>
    /// 收银员使用扫码设备读取用户手机支付宝“付款码”获取设备（如扫码枪）读取用户手机支付宝的付款码信息后，将二维码或条码信息通过本接口上送至支付宝发起支付。
    ///
    /// 注意：
    /// 1. 请根据接入的具体产品参考对应场景描述和示例代码
    /// 2. 当面付产品对于未获取到明确支付成功结果的交易请务必调用撤销接口
    /// </pre>
    /// [接口地址](https://opendocs.alipay.com/open/02ekfp?scene=common)
    /// # 示例
    /// ```no_run
    ///
    ///  # use labrador::AlipayClient;
    ///  # use labrador::AlipayTradePayRequest;
    ///
    ///   # async fn main() {
    ///         let param = AlipayTradePayRequest::default();
    ///         let client = AlipayClient::new("appKey", false);
    ///         match client.unified_order_pay(param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///   # }
    ///
    /// ```
    ///
    pub async fn unified_order_pay(&self, req: AlipayTradePayRequest<AlipayTradePayModel>) -> LabradorResult<AlipayUnifiedOrderPayResponse> {
        let resp = self.excute(req, None, None, None).await?;
        resp.get_biz_model::<AlipayUnifiedOrderPayResponse>()
    }

    /// # 当面付
    /// <pre>
    /// 收银员使用扫码设备读取用户手机支付宝“付款码”获取设备（如扫码枪）读取用户手机支付宝的付款码信息后，将二维码或条码信息通过本接口上送至支付宝发起支付。
    ///
    /// 注意：
    /// 1. 请根据接入的具体产品参考对应场景描述和示例代码
    /// 2. 当面付产品对于未获取到明确支付成功结果的交易请务必调用撤销接口
    /// </pre>
    /// [接口地址](https://opendocs.alipay.com/open/02ekfp?scene=common)
    /// # 示例
    /// ```no_run
    ///
    ///  # use labrador::AlipayClient;
    ///  # use labrador::AlipayTradePayRequest;
    ///
    ///   # async fn main() {
    ///         let param = AlipayTradePayRequest::default();
    ///         let client = AlipayClient::new("appKey", false);
    ///         match client.face_pay(param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///   # }
    ///
    /// ```
    ///
    pub async fn face_pay(&self, req: AlipayTradePayRequest<AlipayFaceOrderPayModel>) -> LabradorResult<AlipayFaceOrderPayResponse> {
        let resp = self.excute(req, None, None, None).await?;
        resp.get_biz_model::<AlipayFaceOrderPayResponse>()
    }

    /// # 周期扣款
    /// <pre>
    /// 用户与商户签署周期扣款协议后，商户可通过本接口做后续免密代扣操作
    /// </pre>
    /// [接口地址](https://opendocs.alipay.com/open/02ekfp?scene=33)
    /// # 示例
    /// ```no_run
    ///
    ///  # use labrador::AlipayClient;
    ///  # use labrador::AlipayTradePayRequest;
    ///
    ///   # async fn main() {
    ///         let param = AlipayTradePayRequest::default();
    ///         let client = AlipayClient::new("appKey", false);
    ///         match client.cycle_pay(param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///   # }
    ///
    /// ```
    ///
    pub async fn cycle_pay(&self, req: AlipayTradePayRequest<AlipayCycleOrderPayModel>) -> LabradorResult<AlipayCycleOrderPayResponse> {
        let resp = self.excute(req, None, None, None).await?;
        resp.get_biz_model::<AlipayCycleOrderPayResponse>()
    }

    /// # 预授权
    /// <pre>
    /// 用户在商户侧授权冻结并享受服务后，商户使用授权单号通过本接口对用户已授权金额发起扣款
    /// </pre>
    /// [接口地址](https://opendocs.alipay.com/open/02ekfp?scene=34)
    /// # 示例
    /// ```no_run
    ///
    ///  # use labrador::AlipayClient;
    ///  # use labrador::AlipayTradePayRequest;
    ///
    ///   # async fn main() {
    ///         let param = AlipayTradePayRequest::default();
    ///         let client = AlipayClient::new("appKey", false);
    ///         match client.pre_auth_online_pay(param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///   # }
    ///
    /// ```
    ///
    pub async fn pre_auth_online_pay(&self, req: AlipayTradePayRequest<AlipayPreAuthOnlinePayModel>) -> LabradorResult<AlipayPreAuthOnlinePayResponse> {
        let resp = self.excute(req, None, None, None).await?;
        resp.get_biz_model::<AlipayPreAuthOnlinePayResponse>()
    }

    /// # 统一收单线下交易预创建
    /// <pre>
    /// 收银员通过收银台或商户后台调用支付宝接口，生成二维码后，展示给用户，由用户扫描二维码完成订单支付。
    /// </pre>
    /// [接口地址](https://opendocs.alipay.com/open/02ekfg?scene=19)
    /// # 示例
    /// ```no_run
    ///
    ///  # use labrador::AlipayClient;
    ///  # use labrador::AlipayTradePrecreateRequest;
    ///
    ///   # async fn main() {
    ///         let param = AlipayTradePrecreateRequest::default();
    ///         let client = AlipayClient::new("appKey", false);
    ///         match client.create_pre_order(param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///   # }
    ///
    /// ```
    ///
    pub async fn create_pre_order(&self, req: AlipayTradePrecreateRequest<AlipayTradePrecreateModel>) -> LabradorResult<AlipayPreOrderResponse> {
        let resp = self.excute(req, None, None, None).await?;
        resp.get_biz_model::<AlipayPreOrderResponse>()
    }

    /// # 统一收单交易创建接口
    /// <pre>
    /// 商户通过该接口进行交易的创建下单
    /// </pre>
    /// [接口地址](https://opendocs.alipay.com/open/02ekfj)
    /// # 示例
    /// ```no_run
    ///
    ///  # use labrador::AlipayClient;
    ///  # use labrador::AlipayTradeCreateRequest;
    ///
    ///   # async fn main() {
    ///         let param = AlipayTradeCreateRequest::default();
    ///         let client = AlipayClient::new("appKey", false);
    ///         match client.create_unified_order(param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///   # }
    ///
    /// ```
    ///
    pub async fn create_unified_order(&self, req: AlipayTradeCreateRequest<AlipayTradeCreateModel>) -> LabradorResult<AlipayCreateUnifiedOrderResponse> {
        let resp = self.excute(req, None, None, None).await?;
        resp.get_biz_model::<AlipayCreateUnifiedOrderResponse>()
    }

    /// # 统一收单线下交易查询
    /// <pre>
    /// 收银员使用扫码设备读取用户手机支付宝“付款码”获取设备（如扫码枪）读取用户手机支付宝的付款码信息后，将二维码或条码信息通过本接口上送至支付宝发起支付。
    /// </pre>
    /// [接口地址](https://opendocs.alipay.com/open/02ekfp?scene=32)
    /// # 示例
    /// ```no_run
    ///
    ///  # use labrador::AlipayClient;
    ///  # use labrador::AlipayTradeQueryRequest;
    ///
    ///   # async fn main() {
    ///         let param = AlipayTradeQueryRequest::default();
    ///         let client = AlipayClient::new("appKey", false);
    ///         match client.query_order(param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///   # }
    ///
    /// ```
    ///
    pub async fn query_order(&self, req: AlipayTradeQueryRequest<AlipayTradeQueryModel>) -> LabradorResult<AlipayQueryOrderResponse> {
        let resp = self.excute(req, None, None, None).await?;
        resp.get_biz_model::<AlipayQueryOrderResponse>()
    }

    /// # 统一收单交易关闭接口
    /// <pre>
    /// 用于交易创建后，用户在一定时间内未进行支付，可调用该接口直接将未付款的交易进行关闭。
    /// </pre>
    /// [接口地址](https://opendocs.alipay.com/open/02e7gn)
    /// # 示例
    /// ```no_run
    ///
    ///  # use labrador::AlipayClient;
    ///  # use labrador::AlipayTradeCloseRequest;
    ///
    ///   # async fn main() {
    ///         let param = AlipayTradeCloseRequest::default();
    ///         let client = AlipayClient::new("appKey", false);
    ///         match client.close_order(param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///   # }
    ///
    /// ```
    ///
    pub async fn close_order(&self, req: AlipayTradeCloseRequest<AlipayTradeCloseModel>) -> LabradorResult<AlipayCloseOrderResponse> {
        let resp = self.excute(req, None, None, None).await?;
        resp.get_biz_model::<AlipayCloseOrderResponse>()
    }

    /// # 统一收单交易退款接口
    /// <pre>
    /// 当交易发生之后一段时间内，由于买家或者卖家的原因需要退款时，卖家可以通过退款接口将支付款退还给买家，支付宝将在收到退款请求并且验证成功之后，按照退款规则将支付款按原路退到买家帐号上。
    /// 交易超过约定时间（签约时设置的可退款时间）的订单无法进行退款。
    /// 支付宝退款支持单笔交易分多次退款，多次退款需要提交原支付订单的订单号和设置不同的退款请求号。一笔退款失败后重新提交，要保证重试时退款请求号不能变更，防止该笔交易重复退款。
    /// 同一笔交易累计提交的退款金额不能超过原始交易总金额。
    /// 注意：
    /// 1. 同一笔交易的退款至少间隔3s后发起
    /// 2. 请严格按照接口文档中的参数进行接入。若在此接口中传入【非当前接口文档中的参数】会造成【退款失败或重复退款】。
    /// 3. 该接口不可与其他退款产品混用。若商户侧同一笔退款请求已使用了当前接口退款的情况下，【再使用其他退款产品进行退款】可能会造成【重复退款】。
    /// 4. 退款成功判断说明：接口返回fund_change=Y为退款成功，fund_change=N或无此字段值返回时需通过退款查询接口进一步确认退款状态。详见退款成功判断指导。注意，接口中code=10000，仅代表本次退款请求成功，不代表退款成功。
    /// </pre>
    /// [接口地址](https://opendocs.alipay.com/open/02e7go)
    /// # 示例
    /// ```no_run
    ///
    ///  # use labrador::AlipayClient;
    ///  # use labrador::AlipayTradeRefundRequest;
    ///
    ///   # async fn main() {
    ///         let param = AlipayTradeRefundRequest::default();
    ///         let client = AlipayClient::new("appKey", false);
    ///         match client.refund_order(param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///   # }
    ///
    /// ```
    ///
    pub async fn refund_order(&self, req: AlipayTradeRefundRequest<AlipayTradeRefundModel>) -> LabradorResult<AlipayRefundOrderResponse> {
        let resp = self.excute(req, None, None, None).await?;
        resp.get_biz_model::<AlipayRefundOrderResponse>()
    }

    /// # 统一收单交易退款查询
    /// <pre>
    /// 商户可使用该接口查询自已通过alipay.trade.refund提交的退款请求是否执行成功。
    ///
    /// 注意：
    /// 1. 该接口的返回码10000，仅代表本次查询操作成功，不代表退款成功，当接口返回的refund_status值为REFUND_SUCCESS时表示退款成功，否则表示退款没有执行成功。
    /// 2. 如果退款未成功，商户可以调用退款接口重试，重试时请务必保证退款请求号和退款金额一致，防止重复退款。
    /// 3. 发起退款查询接口的时间不能离退款请求时间太短，建议之间间隔10秒以上。
    /// </pre>
    /// [接口地址](https://opendocs.alipay.com/open/02ekfl)
    /// # 示例
    /// ```no_run
    ///
    ///  # use labrador::AlipayClient;
    ///  # use labrador::AlipayTradeFastpayRefundQueryRequest;
    ///
    ///   # async fn main() {
    ///         let param = AlipayTradeFastpayRefundQueryRequest::default();
    ///         let client = AlipayClient::new("appKey", false);
    ///         match client.query_refund_order(param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///   # }
    ///
    /// ```
    ///
    pub async fn query_refund_order(&self, req: AlipayTradeFastpayRefundQueryRequest<AlipayTradeFastpayRefundQueryModel>) -> LabradorResult<AlipayRefundQueryResponse> {
        let resp = self.excute(req, None, None, None).await?;
        resp.get_biz_model::<AlipayRefundQueryResponse>()
    }

    /// # 统一收单交易撤销
    /// <pre>
    /// 支付交易返回失败或支付系统超时，调用该接口撤销交易。如果此订单用户支付失败，支付宝系统会将此订单关闭；如果用户支付成功，支付宝系统会将此订单资金退还给用户。
    /// 注意：只有发生支付系统超时或者支付结果未知时可调用撤销，其他正常支付的单如需实现相同功能请调用申请退款API。
    /// 提交支付交易后调用【查询订单API】，没有明确的支付结果再调用【撤销订单API】。
    /// </pre>
    /// [接口地址](https://opendocs.alipay.com/open/02ekfi)
    /// # 示例
    /// ```no_run
    ///
    ///  # use labrador::AlipayClient;
    ///  # use labrador::AlipayTradeCancelRequest;
    ///
    ///   # async fn main() {
    ///         let param = AlipayTradeCancelRequest::default();
    ///         let client = AlipayClient::new("appKey", false);
    ///         match client.cancel_order(param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///   # }
    ///
    /// ```
    ///
    pub async fn cancel_order(&self, req: AlipayTradeCancelRequest<AlipayTradeCancelModel>) -> LabradorResult<AlipayCancelOrderResponse> {
        let resp = self.excute(req, None, None, None).await?;
        resp.get_biz_model::<AlipayCancelOrderResponse>()
    }

    /// # 异步通知参数
    /// 对于 PC 网站支付的交易，在用户支付完成之后，支付宝会根据 API 中商家传入的 notify_url，通过 POST 请求的形式将支付结果作为参数通知到商家系统。
    /// 详见 [文档](https://opendocs.alipay.com/open/270/105902)
    pub fn parse_order_notify(&self, notify_data: &str) -> LabradorResult<AlipayNotifyResponse> {

        let mut data = serde_urlencoded::from_str::<BTreeMap<String, String>>(notify_data)?;
        let sign_type = data.get(constants::SIGN).map(|v| v.to_owned()).unwrap_or_default();
        let sign = data.get(constants::SIGN).map(|v| urlencoding::decode(v).unwrap_or_default().into_owned()).unwrap_or_default();
        // let alipay_cert_sn = data.get(constants::ALIPAY_CERT_SN).map(|v| urlencoding::decode(v).unwrap_or_default().into_owned());
        let source = data.into_iter().filter(|(k,v)| !k.is_empty() && !v.is_empty() && k.ne(constants::SIGN) && k.ne(constants::SIGN_TYPE)).map(|(k, v)| format!("{}={}", k, urlencoding::decode(&v).unwrap_or_default().replace("+", " "))).collect::<Vec<String>>().join("&");
        let result = self.verify(&source, &sign, None)?;
        if !result {
            return Err(LabraError::InvalidSignature("回调结果验签失败！".to_string()))
        }
        let notify_data = format!("{}&{}={}&{}={}", source, constants::SIGN, sign, constants::SIGN_TYPE, sign_type);
        let notify = serde_urlencoded::from_str::<AlipayNotifyResponse>(&notify_data)?;
        Ok(notify)
    }

    /// # 换取授权访问令牌
    /// 换取授权访问令牌
    /// 详见 [文档](https://opendocs.alipay.com/open/02ailc)
    pub async fn system_oauth_token(&self, req: AlipaySystemOauthTokenRequest) -> LabradorResult<AlipaySystemOauthTokenResponse> {
        let resp = self.excute::<_, String>(req, None, None, None).await?;
        resp.get_biz_model::<AlipaySystemOauthTokenResponse>()
    }

    /// # 换取应用授权令牌
    /// 换取应用授权令牌。在应用授权的场景下，商户把名下应用授权给ISV后，支付宝会给ISV颁发应用授权码app_auth_code，ISV可通过获取到的app_auth_code换取app_auth_token。app_auth_code作为换取app_auth_token的票据，每次用户授权带上的app_auth_code将不一样，app_auth_code只能使用一次，一天（从当前时间算起的24小时）未被使用自动过期。
    /// 刷新应用授权令牌，ISV可通过获取到的refresh_token刷新app_auth_token，刷新后老的refresh_token会在一段时间后失效（失效时间为接口返回的re_expires_in）。
    /// 详见 [文档](https://opendocs.alipay.com/isv/03l9c0)
    pub async fn open_auth_token_app(&self, req: AlipayOpenAuthTokenAppRequest<AlipayOpenAuthTokenAppModel>) -> LabradorResult<AlipayOpenAuthTokenAppResponse> {
        let resp = self.excute(req, None, None, None).await?;
        resp.get_biz_model::<AlipayOpenAuthTokenAppResponse>()
    }
}

#[cfg(feature = "openssl-crypto")]
fn iter2string(iter: X509NameEntries) -> LabradorResult<String> {
    let mut string: String = String::from("");
    for value in iter {
        let data = value.data().as_utf8()?.to_string();
        let key = value.object().nid().short_name()?.to_owned();
        string.insert_str(0, &(key + "=" + &data + ","));
    }
    string.pop();
    Ok(string)
}


#[cfg(not(feature = "openssl-crypto"))]
fn iter2string(iter: &X509Name) -> LabradorResult<String> {
    let mut string: String = String::from("");
    for v in iter.to_string().split(",") {
        string.insert_str(0, &(v.trim().to_string() + ","));
    }
    string.pop();
    Ok(string)
}

