use base64;
use reqwest::header::HeaderMap;

use crate::{errors::LabraError, LabradorResult, util::md5};
use serde::{Deserialize, Serialize};
use crate::prp::PrpCrypto;

#[derive(Debug, Eq, PartialEq)]
pub struct WechatCrypto {
    key: Vec<u8>,
    token: Option<String>,
    s_receive_id : Option<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct WechatCryptoV3 {
    v3_key: Vec<u8>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(rename_all = "camelCase")]
pub struct SignatureHeader {
    /// 时间戳
    pub time_stamp: String,
    /// 随机串
    pub nonce: String,
    /// 已签名字符串
    pub signature: String,
    /// 证书序列号
    pub serial: String,
}

impl SignatureHeader {
    pub fn from_header(header: &HeaderMap) -> Self {
        let timpestamp = header.get("Wechatpay-Timestamp");
        let time_stamp = timpestamp.map(|h| h.to_str().unwrap_or_default().to_string()).unwrap_or_default();
        let nonce = header.get("Wechatpay-Nonce");
        let nonce = nonce.map(|h| h.to_str().unwrap_or_default().to_string()).unwrap_or_default();
        let signature = header.get("Wechatpay-Signature");
        let signature = signature.map(|h| h.to_str().unwrap_or_default().to_string()).unwrap_or_default();
        let serial = header.get("Wechatpay-Serial");
        let serial = serial.map(|h| h.to_str().unwrap_or_default().to_string()).unwrap_or_default();
        SignatureHeader {
            time_stamp,
            nonce,
            signature,
            serial
        }
    }
}


#[allow(unused)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptV3 {
    /// 加密前的对象类型
    pub original_type: Option<String>,
    /// 加密算法
    pub algorithm: String,
    /// Base64编码后的密文
    pub ciphertext: Option<String>,
    /// 加密使用的随机串初始化向量）
    pub nonce: String,
    /// 附加数据包（可能为空）
    pub associated_data: Option<String>,
}

#[allow(unused)]
impl WechatCrypto {
    pub fn new(encoding_aes_key: &str) -> WechatCrypto {
        let mut aes_key = encoding_aes_key.to_owned();
        aes_key += "=";
        let key = base64::decode(&aes_key).unwrap_or_default();
        WechatCrypto {
            key,
            token: None,
            s_receive_id: None
        }
    }

    pub fn token(mut self, token: &str) -> WechatCrypto {
        self.token = token.to_string().into();
        self
    }

    pub fn receive_id(mut self, receive_id: &str) -> WechatCrypto {
        self.s_receive_id = receive_id.to_string().into();
        self
    }


    /// # 获取签名
    ///
    /// timestamp 时间戳
    /// nonce 随机字符串
    /// encrypted 加密数据
    pub fn get_signature(&self, timestamp: i64, nonce: &str, encrypted: &str) -> String {
        let token = self.token.to_owned().unwrap_or_default();
        let mut data = vec![
            token.to_string(),
            timestamp.to_string(),
            nonce.to_string(),
        ];
        if !encrypted.is_empty() {
            data.push(encrypted.to_string());
        }
        data.sort();
        let data_str = data.join("");
        Self::get_sha1_sign(&data_str)
    }

    /// SHA1签名
    pub fn get_sha1_sign(encrypt_str: &str) -> String {

        #[cfg(feature = "openssl-crypto")]
        fn sha1(encrypt_str: &str) -> String {
            // create a Sha1 object
            let mut hasher = openssl::sha::Sha1::new();
            // write input message
            hasher.update( encrypt_str.as_bytes());
            // read hash digest
            let signature = hasher.finish();
            // let signature = hash::hash(MessageDigest::sha1(), data_str.as_bytes())?;
            hex::encode(signature)
        }
        #[cfg(not(feature = "openssl-crypto"))]
        fn sha1(encrypt_str: &str) -> String {
            use sha1::{Sha1, Digest};
            // create a Sha1 object
            let mut hasher = Sha1::new();
            // write input message
            hasher.update(encrypt_str);

            // read hash digest
            let hex = hasher.finalize();
            hex::encode(hex)
        }
        sha1(encrypt_str)
    }

    /// hmac_sha256
    pub fn create_hmac_sha256_sign(key: &str, message: &str) -> LabradorResult<String> {
        let prp = PrpCrypto::new(key.as_bytes().to_vec());
        prp.hmac_sha256_sign( message)
    }

    /// # 数据解密
    ///
    /// session_key key
    /// iv 偏移量
    /// encrypted_data 加密数据
    pub fn decrypt_data(session_key: &str, encrypted_data: &str, iv: &str) -> LabradorResult<String> {
        let key = base64::decode(&session_key)?;
        let prp = PrpCrypto::new(key);
        // let msg = prp.aes_128_cbc_decrypt_msg(encrypted_data, iv.into(), None)?;
        todo!("coding...");
        Ok("".to_string())
    }

    /// # 检查签名
    ///
    /// timestamp 时间戳
    /// nonce 随机字符串
    /// echo_str 加密数据
    pub fn check_signature(&self, signature: &str, timestamp: i64, nonce: &str, echo_str: &str) -> LabradorResult<bool> {
        let real_signature = self.get_signature(timestamp, nonce, echo_str);
        if signature != &real_signature {
            return Err(LabraError::InvalidSignature("Unmatched signature.".to_string()));
        }
        Ok(true)
    }

    /// # 加密消息
    ///
    /// timestamp 时间戳
    /// nonce 随机字符串
    /// msg 加密数据
    pub fn encrypt_message(&self, msg: &str, timestamp: i64, nonce: &str) -> LabradorResult<String> {
        let prp = PrpCrypto::new(self.key.to_owned());
        let encrypted_msg = "".to_string();// prp.aes_128_cbc_encrypt_msg(msg, None, (self.s_receive_id.to_owned().unwrap_or_default().as_str()).into())?;
        todo!("coding...");
        let signature = self.get_signature(timestamp, nonce, &encrypted_msg);
        let msg = format!(
            "<xml>\n\
            <Encrypt><![CDATA[{encrypt}]]></Encrypt>\n\
            <MsgSignature><![CDATA[{signature}]]></MsgSignature>\n\
            <TimeStamp>{timestamp}</TimeStamp>\n\
            <Nonce><![CDATA[{nonce}]]></Nonce>\n\
            </xml>",
            encrypt=encrypted_msg,
            signature=signature,
            timestamp=timestamp,
            nonce=nonce,
        );
        Ok(msg)
    }

    /// # 解密消息
    ///
    /// xml 解密内容
    /// nonce 随机字符串
    /// timestamp 时间戳
    /// signature 签名
    pub fn decrypt_message(&self, xml: &str, signature: &str, timestamp: i64, nonce: &str) -> LabradorResult<String> {
        use crate::util::xmlutil;
        let package = xmlutil::parse(xml);
        let doc = package.as_document();
        let encrypted_msg = xmlutil::evaluate(&doc, "//xml/Encrypt/text()").string();
        let real_signature = self.get_signature(timestamp, nonce, &encrypted_msg);
        if signature != &real_signature {
            return Err(LabraError::InvalidSignature("unmatched signature.".to_string()));
        }
        let prp = PrpCrypto::new(self.key.to_owned());
        let msg = "".to_string(); // prp.aes_128_cbc_decrypt_msg(&encrypted_msg, None, (self.s_receive_id.to_owned().unwrap_or_default().as_str()).into())?;
        todo!("coding...");
        Ok(msg)
    }

    /// # 检验消息的真实性，并且获取解密后的明文.
    /// <ol>
    /// <li>利用收到的密文生成安全签名，进行签名验证</li>
    /// <li>若验证通过，则提取xml中的加密消息</li>
    /// <li>对消息进行解密</li>
    /// </ol>
    pub fn decrypt_content(&self, encrypted_content: &str, signature: &str, timestamp: i64, nonce: &str) -> LabradorResult<String> {
        let real_signature = self.get_signature(timestamp, nonce, &encrypted_content);
        if signature != &real_signature {
            return Err(LabraError::InvalidSignature("unmatched signature.".to_string()));
        }
        let prp = PrpCrypto::new(self.key.to_owned());
        let msg = "".to_string(); //prp.aes_256_cbc_decrypt_msg(&encrypted_content, None, self.s_receive_id.as_ref())?;
        todo!("coding...");
        Ok(msg)
    }

    /// # 检验消息的真实性，并且获取解密后的明文.
    /// <ol>
    /// <li>利用收到的密文生成安全签名，进行签名验证</li>
    /// <li>若验证通过，则提取xml中的加密消息</li>
    /// <li>对消息进行解密</li>
    /// </ol>
    pub fn decrypt_xml(&self, encrypted_xml: &str, signature: &str, timestamp: i64, nonce: &str) -> LabradorResult<String> {
        let doc = serde_xml_rs::from_str::<serde_json::Value>(encrypted_xml).unwrap_or(serde_json::Value::Null);
        let cipher_text = doc["Encrypt"]["$value"].as_str().unwrap_or_default();
        self.decrypt_content(cipher_text, signature, timestamp, nonce)
    }

    /// # 解密退款消息
    ///
    /// app_key 应用key
    /// ciphertext 加密数据
    pub fn decrypt_data_refund(app_key: &str, ciphertext: &str) -> LabradorResult<String> {
        let b64decoded = base64::decode(ciphertext)?;
        let md5_key = md5::md5(app_key);
        let key = md5_key.as_bytes();
        let prp = PrpCrypto::new(key.to_vec());
        prp.aes_256_ecb_decrypt(&b64decoded)
    }
}


#[allow(unused)]
impl WechatCryptoV3 {
    pub fn new(v3_key: &str) -> Self {
        let v3_key = v3_key.as_bytes().to_vec();
        WechatCryptoV3 {
            v3_key
        }
    }

    /// # V3  SHA256withRSA 签名.
    /// method       请求方法  GET  POST PUT DELETE 等
    /// url 例如 [示例](https://api.mch.weixin.qq.com/v3/pay/transactions/app?version=1) ——> /v3/pay/transactions/app?version=1
    /// timestamp    当前时间戳   因为要配置到TOKEN 中所以 签名中的要跟TOKEN 保持一致
    /// nonceStr     随机字符串  要和TOKEN中的保持一致
    /// body         请求体 GET 为 "" POST 为JSON
    /// keyPair      商户API 证书解析的密钥对  实际使用的是其中的私钥
    pub fn signature_v3(method: &String, url: &String, timestamp: i64, nonce_str: &String, body: &String, private_key: &String) -> LabradorResult<String> {
        let signature_str = [method, url, &timestamp.to_string(), nonce_str, body];
        let sign = signature_str.iter().map(|item| item.to_string()).collect::<Vec<_>>().join("\n") + "\n";
        PrpCrypto::rsa_sha256_sign_with_pem(&sign, private_key)
    }

    /// # V3  SHA256withRSA 签名.
    /// sign                签名
    /// private_key         私钥
    pub fn sign(sign: &String, private_key: &String) -> LabradorResult<String> {
        PrpCrypto::rsa_sha256_sign_with_pem(&sign, private_key)
    }

    /// # V3  验证签名
    /// signature     签名
    /// public_key    公钥
    pub fn verify(message: &str, signature: &str, public_key: &String) -> LabradorResult<bool> {
        PrpCrypto::rsa_sha256_verify_with_pem(public_key, message, signature)
    }

    /// # V3 消息解密 - 使用V3密钥
    /// decrypt     微信返回的待解密的数据体
    pub fn decrypt_data_v3(&self, decrypt: &EncryptV3) -> LabradorResult<Vec<u8>> {
        let associated_data = decrypt.associated_data.to_owned().unwrap_or_default();
        let nonce = decrypt.nonce.to_owned();
        let ciphertext = decrypt.ciphertext.to_owned().unwrap_or_default();
        let cipher_text = base64::decode(ciphertext)?;
        let base64_cipher = hex::encode(cipher_text);
        let cipher_text = hex::decode(base64_cipher)?;
        let aad= associated_data.as_bytes();
        let iv = nonce.as_bytes();
        let cipherdata_length = cipher_text.len() - 16;
        let cipherdata_bytes = &cipher_text[0..cipherdata_length];
        let tag = &cipher_text[cipherdata_length..cipher_text.len()];
        let prp = PrpCrypto::new(self.v3_key.to_owned());
        let res = prp.aes_256_gcm_decrypt(aad, iv, cipherdata_bytes, tag)?;
        Ok(res)
    }
}


