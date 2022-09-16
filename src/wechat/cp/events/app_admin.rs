use serde::{Serialize, Deserialize};

/// 应用管理员变更通知
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpAppAdminChangeEvent {
    #[serde(rename="FromUserName")]
    pub source: String,
    #[serde(rename="ToUserName")]
    pub target: String,
    #[serde(rename="CreateTime")]
    pub create_time: i64,
    #[serde(rename="MsgId")]
    pub id: i64,
    #[serde(rename="Event")]
    pub event: String,
    #[serde(rename="AgentID")]
    pub agent_id: i64,
}
