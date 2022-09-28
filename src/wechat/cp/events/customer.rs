use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpChangeExternalContactEvent {
    #[serde(rename = "FromUserName")]
    pub source: String,
    #[serde(rename = "ToUserName")]
    pub target: String,
    #[serde(rename = "SuiteId")]
    pub suite_id: Option<String>,
    #[serde(rename = "AuthCorpId")]
    pub auth_corp_id: Option<String>,
    #[serde(rename = "InfoType")]
    pub info_type: Option<String>,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: Option<i64>,
    #[serde(rename = "CreateTime")]
    pub create_time: Option<i64>,
    #[serde(rename = "ChangeType")]
    pub change_type: String,
    #[serde(rename = "UserID")]
    pub user_id: String,
    #[serde(rename = "ExternalUserID")]
    pub external_user_id: String,
    #[serde(rename = "State")]
    pub state: Option<String>,
    #[serde(rename = "WelcomeCode")]
    pub welcome_code: Option<String>,
    /// 事件类型：click
    #[serde(rename = "Event")]
    pub event: String,
}