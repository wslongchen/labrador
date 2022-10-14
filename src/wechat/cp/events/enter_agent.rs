use serde::{Serialize, Deserialize};

/// 进入应用
/// 本事件在成员进入企业微信的应用时触发
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpEnterAgentEvent {
    #[serde(rename="FromUserName")]
    pub source: String,
    #[serde(rename="ToUserName")]
    pub target: String,
    #[serde(rename="CreateTime")]
    pub create_time: i64,
    #[serde(rename="MsgId")]
    pub id: Option<i64>,
    #[serde(rename="Event")]
    pub event: String,
    #[serde(rename="AgentID")]
    pub agent_id: i64,
}

