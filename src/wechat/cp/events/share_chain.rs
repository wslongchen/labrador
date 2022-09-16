use serde::{Serialize, Deserialize};

/// 上下游共享应用事件回调
/// 本事件触发时机为：
/// 1. 上级企业把自建应用共享给下级企业使用
/// 2. 上级企业把下级企业从共享应用中移除
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpShareChainChangeEvent {
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
