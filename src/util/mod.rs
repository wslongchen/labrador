use std::{collections::BTreeMap, time::{SystemTime, UNIX_EPOCH}};
use uuid::Uuid;
use crate::prp::PrpCrypto;

pub mod md5;
pub mod prp;


/// 请求参数
#[derive(Debug, Clone)]
pub struct RequestParametersHolder {
    pub protocal_must_params: BTreeMap<String, String>,
    pub protocal_opt_params: BTreeMap<String, String>,
    pub application_params: BTreeMap<String, String>,
}

impl RequestParametersHolder {
    pub fn new() -> Self {
        Self {
            protocal_must_params: BTreeMap::new(),
            protocal_opt_params: BTreeMap::new(),
            application_params: BTreeMap::new()
        }
    }
    pub fn set_application_params(&mut self, application_params: BTreeMap<String, String>) -> &mut Self {
        self.application_params = application_params;
        self
    }
    pub fn set_protocal_must_params(&mut self, protocal_must_params: BTreeMap<String, String>) -> &mut Self {
        self.protocal_must_params = protocal_must_params;
        self
    }
    pub fn set_protocal_opt_params(&mut self, protocal_opt_params: BTreeMap<String, String>) -> &mut Self {
        self.protocal_opt_params = protocal_opt_params;
        self
    }

    pub fn get_sorted_map(&self) -> BTreeMap<&String, &String> {
        let mut sorted_params = BTreeMap::new();
        if self.application_params.len() > 0 {
            for (k, v) in self.application_params.iter() {
                sorted_params.insert(k, v);
            }
        }
        if self.protocal_must_params.len() > 0 {
            for (k, v) in self.protocal_must_params.iter() {
                sorted_params.insert(k, v);
            }
        }
        if self.protocal_opt_params.len() > 0 {
            for (k, v) in self.protocal_opt_params.iter() {
                sorted_params.insert(k, v);
            }
        }
        sorted_params
    }

    pub fn get_signature_content(&self) -> String {
        let pairs = self.get_sorted_map();
        let signature_content = pairs.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<String>>().join("&");
        signature_content
    }
}

pub trait StringExtern <T> {
    fn add_form_param(&mut self, field_name: &str, field_value: T);
    fn add_form_params(&mut self, field_name: &str, field_value: T);
}

impl <T: Sized + std::fmt::Display> StringExtern<T> for String {

    fn add_form_param(&mut self, field_name: &str, field_value: T) {
        self.push_str(format!("{}{}", field_name, field_value).as_str());
    }

    fn add_form_params(&mut self, field_name: &str, field_value: T) {
        if !self.is_empty() {
            self.push_str("&");
        }
        self.push_str(format!("{}={}", field_name, field_value).as_str());
    }
}

/// Sign Method (build request paramaters sign format)
///
/// See: ThirdPlatform(Wechat/Alibaba/PDD) > Security
#[allow(unused)]
pub fn get_sign(pairs: &BTreeMap<String, String>, secret: &str) -> String {
    // filter null params
    let keys = pairs
        .iter()
        .filter(|pair| pair.0.ne("key") && pair.0.ne("sign") && !pair.1.is_empty())
        .map(|pair| pair.0.to_string())
        .collect::<Vec<String>>();
    // Build ASCII Sort（DIC）
    let mut params = String::default();
    for key in keys {
        params.add_form_param(key.as_str(), &pairs[&key].as_str());
        
    }
    params = format!("{}{}{}",secret,params,secret);
    // generate MD5 string
    md5::md5(params).to_uppercase()
}

/// Sign Method (build request paramaters sign format)
#[allow(unused)]
pub fn get_sign_with_rsa(pairs: &BTreeMap<String, String>, private_key: &str) -> String {
    // filter null params
    let keys = pairs
        .iter()
        .filter(|pair| pair.0.ne("sign") && !pair.1.is_empty())
        .map(|pair| pair.0.to_string())
        .collect::<Vec<String>>();
    // Build ASCII Sort（DIC）
    let mut params = String::default();
    for key in keys {
        params.add_form_params(key.as_str(), &pairs[&key].as_str());
    }
    // generate RSA string
    let sign = PrpCrypto::rsa_sha256_sign_pkcs8(&params, base64::decode(private_key).unwrap()).unwrap();
    sign
}

/// Sign Method (build request paramaters sign format)
#[allow(unused)]
pub fn get_sign_params(pairs: &BTreeMap<String, String>, private_key: &str) -> String {
    // filter null params
    let keys = pairs
        .iter()
        .filter(|pair| pair.0.ne("key") && pair.0.ne("sign") && !pair.1.is_empty())
        .map(|pair| pair.0.to_string())
        .collect::<Vec<String>>();
    // Build ASCII Sort（DIC）
    let mut params = String::default();
    for key in keys {
        params.add_form_params(key.as_str(), &pairs[&key].as_str());

    }
    params
}

/// Get TimeStamp
#[allow(unused)]
pub fn get_timestamp() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let ms = since_the_epoch.as_secs() as i64 * 1000i64 + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64;
    ms
}

pub fn merge_properties(mut source: serde_json::Value, target: serde_json::Value) -> serde_json::Value {
    if let Some(data) = target.as_object() {
        for (k, v) in data.into_iter() {
            source[k] = v.to_owned();
        }
    }
    source
}

pub fn remove_properties(mut source: serde_json::Value, props: Vec<&str>) -> serde_json::Value {
    if let Some(data) = source.as_object_mut() {
        for prop in props.iter() {
            data.remove(&prop.to_string());
        }
    }
    source
}

/// 生成随机数算法
///
/// 微信支付API接口协议中包含字段nonce_str，主要保证签名不可预测。
#[allow(unused)]
pub fn get_nonce_str() -> String {
    Uuid::new_v4().to_simple().to_string()
}

#[allow(unused)]
pub fn current_timestamp() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
}


// pub fn to_raw<V: Serialize>(value: &V) -> Vec<u8> {
//     let mut raw = Vec::new();
//     let mut zstd_writer = zstd::Encoder::new(&mut raw, 0).unwrap();
//     bincode::serialize_into(&mut zstd_writer, value).unwrap();
//     zstd_writer.finish().unwrap();
//     raw
// }

// pub fn from_raw<V: DeserializeOwned>(raw: &[u8]) -> Option<V> {
//     bincode::deserialize(&zstd::decode_all(raw).unwrap()).ok()
// }

#[macro_export]
macro_rules! cfg_if {
    // match if/else chains with a final `else`
    (
        $(
            if #[cfg( $i_meta:meta )] { $( $i_tokens:tt )* }
        ) else+
        else { $( $e_tokens:tt )* }
    ) => {
        $crate::cfg_if! {
            @__items () ;
            $(
                (( $i_meta ) ( $( $i_tokens )* )) ,
            )+
            (() ( $( $e_tokens )* )) ,
        }
    };

    // match if/else chains lacking a final `else`
    (
        if #[cfg( $i_meta:meta )] { $( $i_tokens:tt )* }
        $(
            else if #[cfg( $e_meta:meta )] { $( $e_tokens:tt )* }
        )*
    ) => {
        $crate::cfg_if! {
            @__items () ;
            (( $i_meta ) ( $( $i_tokens )* )) ,
            $(
                (( $e_meta ) ( $( $e_tokens )* )) ,
            )*
        }
    };

    // Internal and recursive macro to emit all the items
    //
    // Collects all the previous cfgs in a list at the beginning, so they can be
    // negated. After the semicolon is all the remaining items.
    (@__items ( $( $_:meta , )* ) ; ) => {};
    (
        @__items ( $( $no:meta , )* ) ;
        (( $( $yes:meta )? ) ( $( $tokens:tt )* )) ,
        $( $rest:tt , )*
    ) => {
        // Emit all items within one block, applying an appropriate #[cfg]. The
        // #[cfg] will require all `$yes` matchers specified and must also negate
        // all previous matchers.
        #[cfg(all(
            $( $yes , )?
            not(any( $( $no ),* ))
        ))]
        $crate::cfg_if! { @__identity $( $tokens )* }

        // Recurse to emit all other items in `$rest`, and when we do so add all
        // our `$yes` matchers to the list of `$no` matchers as future emissions
        // will have to negate everything we just matched as well.
        $crate::cfg_if! {
            @__items ( $( $no , )* $( $yes , )? ) ;
            $( $rest , )*
        }
    };

    // Internal macro to make __apply work out right for different match types,
    // because of how macros match/expand stuff.
    (@__identity $( $tokens:tt )* ) => {
        $( $tokens )*
    };
}


cfg_if! {if #[cfg(feature = "wechat")]{
    pub mod xmlutil;
}}


#[test]
fn test() {
    println!("{}", get_nonce_str());
}