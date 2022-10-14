
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpLinkMessage {
    #[serde(rename="FromUserName")]
    pub source: String,
    #[serde(rename="ToUserName")]
    pub target: String,
    #[serde(rename="CreateTime")]
    pub create_time: i64,
    #[serde(rename="MsgId")]
    pub id: i64,
    #[serde(rename="Title")]
    pub title: String,
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="Url")]
    pub url: String,
    #[serde(rename="AgentID")]
    pub agent_id: i64,
}


