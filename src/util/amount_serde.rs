use serde::{Serializer, Deserializer, Deserialize};
use serde::de::Error;

pub fn serialize<S>(value: &Option<f64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(v) => {
            // 序列化时格式化为2位小数
            serializer.serialize_str(&format!("{:.2}", v))
        },
        None => serializer.serialize_none(),
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(str_val) => {
            // 解析为f64
            str_val.parse::<f64>()
                .map(Some)
                .map_err(D::Error::custom)
        },
        None => Ok(None),
    }
}