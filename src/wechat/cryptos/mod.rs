use base64;
use openssl::sha::Sha1;
use openssl::symm;
use reqwest::header::HeaderMap;
use rustc_serialize::hex::{FromHex, ToHex};

use crate::{errors::LabraError, LabradorResult, util::md5};
use serde::{Deserialize, Serialize};
use crate::prp::PrpCrypto;

#[derive(Debug, Eq, PartialEq)]
pub struct WeChatCrypto {
    token: String,
    key: Vec<u8>,
    _id: String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct WeChatCryptoV3 {
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
impl WeChatCrypto {
    pub fn new(token: &str, encoding_aes_key: &str, _id: &str) -> WeChatCrypto {
        let mut aes_key = encoding_aes_key.to_owned();
        aes_key.push('=');
        let key = base64::decode(&aes_key).unwrap_or_default();
        WeChatCrypto {
            token: token.to_owned(),
            key: key,
            _id: _id.to_owned(),
        }
    }

    /// #获取签名
    ///
    /// timestamp 时间戳
    /// nonce 随机字符串
    /// encrypted 加密数据
    fn get_signature(&self, timestamp: i64, nonce: &str, encrypted: &str) -> LabradorResult<String> {
        let mut data = vec![
            self.token.to_owned(),
            timestamp.to_string(),
            nonce.to_owned(),
            encrypted.to_owned(),
        ];
        data.sort();
        let data_str = data.join("");
        // create a Sha1 object
        let mut hasher = Sha1::new();
        // write input message
        hasher.update( data_str.as_bytes());
        // read hash digest
        let signature = hasher.finish();
        // let signature = hash::hash(MessageDigest::sha1(), data_str.as_bytes())?;
        Ok(String::from_utf8_lossy(&signature).to_string())
    }

    /// #数据解密
    ///
    /// session_key key
    /// iv 偏移量
    /// encrypted_data 加密数据
    pub fn decrypt_data(session_key: &str, encrypted_data: &str, iv: &str) -> LabradorResult<String> {
        let key = base64::decode(&session_key)?;
        let prp = PrpCrypto::new(key);
        let msg = prp.aes_128_cbc_decrypt_data(encrypted_data, iv)?;
        Ok(msg)
    }

    /// #检查签名
    ///
    /// timestamp 时间戳
    /// nonce 随机字符串
    /// echo_str 加密数据
    pub fn check_signature(&self, signature: &str, timestamp: i64, nonce: &str, echo_str: &str) -> LabradorResult<String> {
        let real_signature = self.get_signature(timestamp, nonce, echo_str)?;
        if signature != &real_signature {
            return Err(LabraError::InvalidSignature("Unmatched signature.".to_string()));
        }
        let prp = PrpCrypto::new(self.key.to_owned());
        let msg = prp.aes_128_cbc_decrypt_msg(echo_str, &self._id)?;
        Ok(msg)
    }

    /// #加密消息
    ///
    /// timestamp 时间戳
    /// nonce 随机字符串
    /// msg 加密数据
    pub fn encrypt_message(&self, msg: &str, timestamp: i64, nonce: &str) -> LabradorResult<String> {
        let prp = PrpCrypto::new(self.key.to_owned());
        let encrypted_msg = prp.aes_128_cbc_encrypt_msg(msg, &self._id)?;
        let signature = self.get_signature(timestamp, nonce, &encrypted_msg)?;
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

    /// #解密消息
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
        let real_signature = self.get_signature(timestamp, nonce, &encrypted_msg)?;
        if signature != &real_signature {
            return Err(LabraError::InvalidSignature("unmatched signature.".to_string()));
        }
        let prp = PrpCrypto::new(self.key.to_owned());
        let msg = prp.aes_128_cbc_decrypt_msg(&encrypted_msg, &self._id)?;
        Ok(msg)
    }

    /// #解密退款消息
    ///
    /// app_key 应用key
    /// ciphertext 加密数据
    pub fn decrypt_data_refund(app_key: &str, ciphertext: &str) -> LabradorResult<String> {
        let b64decoded = base64::decode(ciphertext)?;
        let md5_key = md5::md5(app_key);
        let text = symm::decrypt(symm::Cipher::aes_256_ecb(), md5_key.as_bytes(), None, &b64decoded).unwrap_or_default();
        let content_string = String::from_utf8(text).unwrap_or_default();
        Ok(content_string)
    }
}


#[allow(unused)]
impl WeChatCryptoV3 {
    pub fn new(v3_key: &str) -> Self {
        let v3_key = v3_key.as_bytes().to_vec();
        WeChatCryptoV3 {
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
        PrpCrypto::rsa_sha256_sign(&sign, private_key)
    }

    /// # V3  SHA256withRSA 签名.
    /// sign                签名
    /// private_key         私钥
    pub fn sign(sign: &String, private_key: &String) -> LabradorResult<String> {
        PrpCrypto::rsa_sha256_sign(&sign, private_key)
    }

    /// # V3  验证签名
    /// signature     签名
    /// public_key    公钥
    pub fn verify(message: &str, signature: &str, public_key: &String) -> LabradorResult<bool> {
        PrpCrypto::rsa_sha256_verify(public_key, message, signature)
    }

    /// # V3 消息解密 - 使用V3密钥
    /// decrypt     微信返回的待解密的数据体
    pub fn decrypt_data_v3(&self, decrypt: &EncryptV3) -> LabradorResult<Vec<u8>> {
        let associated_data = decrypt.associated_data.to_owned().unwrap_or_default();
        let nonce = decrypt.nonce.to_owned();
        let ciphertext = decrypt.ciphertext.to_owned().unwrap_or_default();
        let cipher_text = base64::decode(ciphertext)?;
        let base64_cipher = cipher_text.to_hex();
        let cipher_text = base64_cipher.from_hex()?;
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


#[cfg(test)]
#[allow(unused, non_snake_case)]
mod tests {
    use super::WeChatCrypto;

    #[test]
    fn test_get_signature() {
        let crypto = WeChatCrypto::new("test", "kWxPEV2UEDyxWpmPdKC3F4dgPDmOvfKX1HGnEUDS1aR", "test");
        let signature = crypto.get_signature(123456i64, "test", "rust").unwrap();
        assert_eq!("d6056f2bb3ad3e30f4afa5ef90cc9ddcdc7b7b27", &signature);
    }

    #[test]
    fn test_check_signature_should_ok() {
        let signature = "dd6b9c95b495b3f7e2901bfbc76c664930ffdb96";
        let timestamp = 1411443780;
        let nonce = "437374425";
        let echo_str = "4ByGGj+sVCYcvGeQYhaKIk1o0pQRNbRjxybjTGblXrBaXlTXeOo1+bXFXDQQb1o6co6Yh9Bv41n7hOchLF6p+Q==";
        let crypto = WeChatCrypto::new("123456", "kWxPEV2UEDyxWpmPdKC3F4dgPDmOvfKX1HGnEUDS1aR", "wx49f0ab532d5d035a");
        match crypto.check_signature(signature, timestamp, nonce, echo_str) {
            Ok(_) => {},
            Err(_) => panic!("Check signature failed"),
        }
    }

    #[test]
    fn test_check_decrypted_data_should_ok() {
        let appId = "wx4f4bc4dec97d474b";
        let sessionKey = "tiihtNczf5v6AKRyjwEUhQ==";
        let encryptedData = "CiyLU1Aw2KjvrjMdj8YKliAjtP4gsMZMQmRzooG2xrDcvSnxIMXFufNstNGTyaGS9uT5geRa0W4oTOb1WT7fJlAC+oNPdbB+3hVbJSRgv+4lGOETKUQz6OYStslQ142dNCuabNPGBzlooOmB231qMM85d2/fV6ChevvXvQP8Hkue1poOFtnEtpyxVLW1zAo6/1Xx1COxFvrc2d7UL/lmHInNlxuacJXwu0fjpXfz/YqYzBIBzD6WUfTIF9GRHpOn/Hz7saL8xz+W//FRAUid1OksQaQx4CMs8LOddcQhULW4ucetDf96JcR3g0gfRK4PC7E/r7Z6xNrXd2UIeorGj5Ef7b1pJAYB6Y5anaHqZ9J6nKEBvB4DnNLIVWSgARns/8wR2SiRS7MNACwTyrGvt9ts8p12PKFdlqYTopNHR1Vf7XjfhQlVsAJdNiKdYmYVoKlaRv85IfVunYzO0IKXsyl7JCUjCpoG20f0a04COwfneQAGGwd5oa+T8yO5hzuyDb/XcxxmK01EpqOyuxINew==";
        let iv = "r7BXXKkLb8qrSNn05n0qiA==";
        match WeChatCrypto::decrypt_data(sessionKey, encryptedData, iv) {
            Ok(data) => {
                println!("success to decrypted data.{}", data);
            },
            Err(_) => panic!("Check signature failed"),
        }
    }

    #[test]
    #[should_panic]
    fn test_check_signature_should_fail() {
        let signature = "dd6b9c95b495b3f7e2901bfbc76c664930ffdb96";
        let timestamp = 1411443780;
        let nonce = "437374424";
        let echo_str = "4ByGGj+sVCYcvGeQYhaKIk1o0pQRNbRjxybjTGblXrBaXlTXeOo1+bXFXDQQb1o6co6Yh9Bv41n7hOchLF6p+Q==";
        let crypto = WeChatCrypto::new("123456", "kWxPEV2UEDyxWpmPdKC3F4dgPDmOvfKX1HGnEUDS1aR", "wx49f0ab532d5d035a");
        match crypto.check_signature(signature, timestamp, nonce, echo_str) {
            Ok(_) => {},
            Err(_) => panic!("Check signature failed"),
        }
    }

    #[test]
    fn test_encrypt_message() {
        let timestamp = 1411525903;
        let nonce = "461056294";
        let msg = "<xml>\n\
            <MsgType><![CDATA[text]]></MsgType>\n\
            <Content><![CDATA[test]]></Content>\n\
            <FromUserName><![CDATA[wx49f0ab532d5d035a]]></FromUserName>\n\
            <ToUserName><![CDATA[messense]]></ToUserName>\n\
            <AgentID>1</AgentID>\n\
            <CreateTime>1411525903</CreateTime>\n\
            </xml>";
        let expected = "<xml>\n\
            <Encrypt><![CDATA[9s4gMv99m88kKTh/H8IdkOiMg6bisoy3ypwy9H4hvSPe9nsGaqyw5hhSjdYbcrKk+j3nba4HMOTzHrluLBYqxgNcBqGsL8GqxlhZgURnAtObvesEl5nZ+uBE8bviY0LWke8Zy9V/QYKxNV2FqllNXcfmstttyIkMKCCmVbCFM2JTF5wY0nFhHZSjPUL2Q1qvSUCUld+/WIXrx0oyKQmpB6o8NRrrNrsDf03oxI1p9FxUgMnwKKZeOA/uu+2IEvEBtb7muXsVbwbgX05UPPJvFurDXafG0RQyPR+mf1nDnAtQmmNOuiR5MIkdQ39xn1vWwi1O5oazPoQJz0nTYjxxEE8kv3kFxtAGVRe3ypD3WeK2XeFYFMNMpatF9XiKzHo3]]></Encrypt>\n\
            <MsgSignature><![CDATA[407518b7649e86ef23978113f92d27afa9296533]]></MsgSignature>\n\
            <TimeStamp>1411525903</TimeStamp>\n\
            <Nonce><![CDATA[461056294]]></Nonce>\n\
            </xml>";
        let crypto = WeChatCrypto::new("123456", "kWxPEV2UEDyxWpmPdKC3F4dgPDmOvfKX1HGnEUDS1aR", "wx49f0ab532d5d035a");
        let encrypted = crypto.encrypt_message(msg, timestamp, nonce).unwrap();
        assert_eq!(expected, &encrypted);
    }

    #[test]
    fn test_decrypt_message() {
        let xml = "<xml><ToUserName><![CDATA[wx49f0ab532d5d035a]]></ToUserName>\n\
            <Encrypt><![CDATA[RgqEoJj5A4EMYlLvWO1F86ioRjZfaex/gePD0gOXTxpsq5Yj4GNglrBb8I2BAJVODGajiFnXBu7mCPatfjsu6IHCrsTyeDXzF6Bv283dGymzxh6ydJRvZsryDyZbLTE7rhnus50qGPMfp2wASFlzEgMW9z1ef/RD8XzaFYgm7iTdaXpXaG4+BiYyolBug/gYNx410cvkKR2/nPwBiT+P4hIiOAQqGp/TywZBtDh1yCF2KOd0gpiMZ5jSw3e29mTvmUHzkVQiMS6td7vXUaWOMZnYZlF3So2SjHnwh4jYFxdgpkHHqIrH/54SNdshoQgWYEvccTKe7FS709/5t6NMxuGhcUGAPOQipvWTT4dShyqio7mlsl5noTrb++x6En749zCpQVhDpbV6GDnTbcX2e8K9QaNWHp91eBdCRxthuL0=]]></Encrypt>\n\
            <AgentID><![CDATA[1]]></AgentID>\n\
            </xml>";
        let expected = "<xml><ToUserName><![CDATA[wx49f0ab532d5d035a]]></ToUserName>\n\
            <FromUserName><![CDATA[messense]]></FromUserName>\n\
            <CreateTime>1411525903</CreateTime>\n\
            <MsgType><![CDATA[text]]></MsgType>\n\
            <Content><![CDATA[test]]></Content>\n\
            <MsgId>4363689963896700987</MsgId>\n\
            <AgentID>1</AgentID>\n\
            </xml>";

        let signature = "74d92dfeb87ba7c714f89d98870ae5eb62dff26d";
        let timestamp = 1411525903;
        let nonce = "461056294";
        let crypto = WeChatCrypto::new("123456", "kWxPEV2UEDyxWpmPdKC3F4dgPDmOvfKX1HGnEUDS1aR", "wx49f0ab532d5d035a");
        let decrypted = crypto.decrypt_message(xml, signature, timestamp, nonce).unwrap();
        assert_eq!(expected, &decrypted);
    }
}
