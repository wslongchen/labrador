use std::borrow::Cow;
use chrono::Local;

use bytes::Bytes;
use log::{debug, info};
use reqwest::header::HeaderMap;
use reqwest::multipart::Part;
use serde_json::json;
use crate::{LabradorResult, LabraError};
use crate::prp::PrpCrypto;


#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Qiniu<'a> {
    access_key: Cow<'a, str>,
    secret_key: Cow<'a, str>,
    endpoint: Cow<'a, str>,
    bucket: Cow<'a, str>,

    pub(crate) http_client: reqwest::Client,
}


/// Qiniu
/// 
/// 
/// # Example
/// 
/// ```no_run
/// use labrador::Qiniu;
/// async fn main() {
///     let client = Qiniu::new("appKey", "secret");
///     // Do Some Thing You Want
///     // ...
/// }
/// ```
/// 

impl<'a> Qiniu<'a> {
    pub fn new<S>(access_key: S, secret_key: S, endpoint: S, bucket: S) -> Self
        where
            S: Into<Cow<'a, str>>,
    {
        let http_client = reqwest::Client::new();

        Qiniu {
            access_key: access_key.into(),
            secret_key: secret_key.into(),
            endpoint: endpoint.into(),
            bucket: bucket.into(),
            http_client,
        }
    }

    pub fn bucket(&self) -> &str {
        &self.bucket
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn access_key(&self) -> &str {
        &self.access_key
    }

    pub fn secret_key(&self) -> &str {
        &self.secret_key
    }


    pub fn get_access_token(&self, filename: &str) -> String {
        // 1.构造上传策略
        // let setting = &SETTINGS;
        let bucket_name = self.bucket.to_owned();
        let secret_key = self.secret_key.to_owned();
        let access_key =  self.access_key.to_owned();

        let scope = format!("{}:{}", bucket_name, filename);
        let put_policy = json!({
            "scope":scope,
            "deadline": Local::now().timestamp() + 172000,
            "returnBody":"{\"name\":$(fname),\"size\":$(fsize),\"w\":$(imageInfo.width),\"h\":$(imageInfo.height),\"hash\":$(etag),\"key\":$(key)}"
        });
        debug!("上传七牛云参数policy: {}", &put_policy.to_string());
        // 3.对 JSON 编码的上传策略进行URL 安全的 Base64 编码，得到待签名字符串
        let encoded_put_policy = safe_base64(put_policy.to_string().as_bytes());//base64::encode(put_policy.to_string().as_bytes());
        debug!("上传七牛云参数encodedPutPolicy: {}", &encoded_put_policy.to_string());
        // 4.使用访问密钥（AK/SK）对上一步生成的待签名字符串计算HMAC-SHA1签名
        let prp = PrpCrypto::new(secret_key.as_bytes().to_vec());
        let result = prp.hmac_sha1_sign(&encoded_put_policy).unwrap_or_default();
        // 5.对签名进行URL安全的Base64编码
        let encoded_sign = safe_base64(result);//base64::encode(result.code());
        let data = format!("{}:{}:{}",access_key, encoded_sign, encoded_put_policy);
        debug!("上传七牛云参数token: {}", &data);
        data
    }

    pub fn host(&self, bucket: &str, object: &str, resources_str: &str) -> String {
        if self.endpoint.starts_with("https") {
            format!(
                "https://{}.{}/{}?{}",
                bucket,
                self.endpoint.replacen("https://", "", 1),
                object,
                resources_str
            )
        } else {
            format!(
                "http://{}.{}/{}?{}",
                bucket,
                self.endpoint.replacen("http://", "", 1),
                object,
                resources_str
            )
        }
    }

    pub async fn upload(&self, file: Bytes, filename: String) -> LabradorResult<String> {
        let upload_token = self.get_access_token(filename.as_str());
        let mut headers = HeaderMap::new();
        headers.insert("Host", "up-z2.qiniup.com".parse().unwrap());
        let client = reqwest::Client::new();
        let part = Part::stream(file).file_name(filename.to_owned());
        let form =reqwest::multipart::Form::new().part("file", part)
            .part("key", Part::text(filename.to_owned()))
            .part("token", Part::text(upload_token.to_owned()))
            .part("fileName", Part::text(filename.to_owned()))
            .part("resource_key",  Part::text(filename.to_owned()));
        info!("七牛云上传参数：url:{}, upload_token: {}, filename: {}", self.endpoint(), &upload_token, &filename);
        let result = client
            .post(self.endpoint())
            .multipart(form)
            .headers(headers.to_owned()).send().await.map_err(|err| LabraError::ApiError(err.to_string()))?
            .text().await.map_err(|err| LabraError::ApiError(err.to_string()))?;
        info!("请求七牛云返回结果：{}", result);
        Ok(result)
    }

}


pub fn safe_base64<T: AsRef<[u8]>>(encode_data: T) -> String {
    base64::encode(encode_data).replace("+", "-").replace("/", "_")
}