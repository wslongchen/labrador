
use serde::{Serialize, Deserialize};

/// 重置永久授权码通知
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpPermanentCodeEvent {
    #[serde(rename="SuiteId")]
    pub suite_id: String,
    #[serde(rename="InfoType")]
    pub info_type: String,
    #[serde(rename="TimeStamp")]
    pub time: i64,
    #[serde(rename="AuthCode")]
    pub auth_code: String,
}