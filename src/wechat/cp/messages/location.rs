use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpLocationMessage {
    #[serde(rename="FromUserName")]
    pub source: String,
    #[serde(rename="ToUserName")]
    pub target: String,
    #[serde(rename="CreateTime")]
    pub create_time: i64,
    #[serde(rename="MsgId")]
    pub id: i64,
    #[serde(rename="Location_X")]
    pub location_x: f64,
    #[serde(rename="Location_Y")]
    pub location_y: f64,
    #[serde(rename="Scale")]
    pub scale: usize,
    #[serde(rename="Label")]
    pub label: String,
    #[serde(rename="AgentID")]
    pub agent_id: i64,
}
