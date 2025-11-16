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


    pub fn get_upload_token(&self, filename: &str) -> String {
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


    pub fn get_access_token(&self, method: &str, req_url: &str, req_content_type: Option<&str>, req_keys: Option<Vec<(String, String)>>, req_body: Option<&str>) -> String {
        let secret_key = self.secret_key.to_owned();
        let access_key =  self.access_key.to_owned();

        let mut signing_str = format!("{} {}\nHost: ", method.to_uppercase(), req_url);
        if let Some(content_type) = req_content_type {
            signing_str.push_str("\n");
            signing_str.push_str("Content-Type: ");
            signing_str.push_str(content_type);
        }
        if let Some(req_keys) = req_keys {
            for (k, v) in req_keys.iter() {
                signing_str.push_str("\n");
                signing_str.push_str(k);
                signing_str.push_str(": ");
                signing_str.push_str(v);
            }
        }

        signing_str.push_str("\n");
        signing_str.push_str("\n");

        if let Some(body) = req_body {
            let content_type =  req_content_type.unwrap_or_default();
            if !content_type.contains("application/octet-stream") {
                signing_str.push_str(body);
            }

        }
        debug!("七牛云管理凭证待签名字符串: {}", &signing_str);
        // 使用访问密钥（AK/SK）对上一步生成的待签名字符串计算HMAC-SHA1签名
        let prp = PrpCrypto::new(secret_key.as_bytes().to_vec());
        let result = prp.hmac_sha1_sign(&signing_str).unwrap_or_default();
        // 对签名进行URL安全的Base64编码
        let encoded_sign = safe_base64(result);//base64::encode(result.code());
        let data = format!("{}:{}",access_key, encoded_sign);
        debug!("七牛云管理凭证: {}", &data);
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
        let upload_token = self.get_upload_token(filename.as_str());
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

    pub async fn delete_obj(&self, obj_name: String) -> LabradorResult<String> {
        let mut headers = HeaderMap::new();
        headers.insert("Host", "rs.qiniup.com".parse().unwrap());
        headers.insert("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());
        let client = reqwest::Client::new();
        let entry = format!("{}:{}", self.bucket(), &obj_name);
        let encoded_entry_uri = safe_base64(entry);
        let url = format!("/delete/{}", encoded_entry_uri);
        let token = self.get_access_token("POST", &url, Some("application/x-www-form-urlencoded"), None, None);
        headers.insert("Authorization", format!("Qiniu {}", token).parse().unwrap());
        let result = client
            .post(format!("{}{}", self.endpoint(), url))
            .form(&serde_json::Value::Null)
            .headers(headers.to_owned()).send().await.map_err(|err| LabraError::ApiError(err.to_string()))?
            .text().await.map_err(|err| LabraError::ApiError(err.to_string()))?;
        Ok(result)
    }

    pub async fn get_obj(&self, obj_name: &str) -> LabradorResult<Bytes> {
        let domains = self.domains().await?;
        let domain = domains.first().map(ToString::to_string).unwrap_or_default();
        let client = reqwest::Client::new();
        let result = client
            .get(format!("{}/{}", domain, obj_name)).send().await.map_err(|err| LabraError::ApiError(err.to_string()))?
            .bytes().await.map_err(|err| LabraError::ApiError(err.to_string()))?;
        Ok(result)
    }

    pub async fn domains(&self) -> LabradorResult<Vec<String>> {
        let bucket = self.bucket.to_string();
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());
        headers.insert("Host", "rs.qiniup.com".parse().unwrap());
        let client = reqwest::Client::new();
        let url = format!("/v2/domains?tbl={}", bucket);
        let token = self.get_access_token("POST", &url, Some("application/x-www-form-urlencoded"), None, None);
        headers.insert("Authorization", format!("Qiniu {}", token).parse().unwrap());
        let result = client
            .post(format!("{}{}", self.endpoint(), url))
            .form(&serde_json::Value::Null)
            .headers(headers.to_owned()).send().await.map_err(|err| LabraError::ApiError(err.to_string()))?
            .json::<Vec<String>>().await.map_err(|err| LabraError::ApiError(err.to_string()))?;
        Ok(result)
    }

}


pub fn safe_base64<T: AsRef<[u8]>>(encode_data: T) -> String {
    base64::encode(encode_data).replace("+", "-").replace("/", "_")
}