
use serde::{Serialize, Deserialize};

/// 授权成功通知
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpAuthCreateEvent {
    #[serde(rename="SuiteId")]
    pub suite_id: String,
    #[serde(rename="InfoType")]
    pub info_type: String,
    #[serde(rename="TimeStamp")]
    pub time: i64,
    #[serde(rename="AuthCode")]
    pub auth_code: String,
    #[serde(rename="State")]
    pub state: Option<String>,
}
/// 变更授权通知
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpAuthChangeEvent {
    #[serde(rename="SuiteId")]
    pub suite_id: String,
    #[serde(rename="InfoType")]
    pub info_type: String,
    #[serde(rename="TimeStamp")]
    pub time: i64,
    #[serde(rename="AuthCorpId")]
    pub auth_corp_id: String,
    #[serde(rename="State")]
    pub state: Option<String>,
}
/// 取消授权通知
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpAuthCancelEvent {
    #[serde(rename="SuiteId")]
    pub suite_id: String,
    #[serde(rename="InfoType")]
    pub info_type: String,
    #[serde(rename="TimeStamp")]
    pub time: i64,
    #[serde(rename="AuthCorpId")]
    pub auth_corp_id: String,
}