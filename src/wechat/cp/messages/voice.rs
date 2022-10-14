

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpVoiceMessage {
    #[serde(rename="FromUserName")]
    pub source: String,
    #[serde(rename="ToUserName")]
    pub target: String,
    #[serde(rename="CreateTime")]
    pub create_time: i64,
    #[serde(rename="MsgId")]
    pub id: i64,
    #[serde(rename="MediaId")]
    pub media_id: String,
    #[serde(rename="Format")]
    pub format: String,
    #[serde(rename="AgentID")]
    pub agent_id: i64,
}

