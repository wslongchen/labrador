use serde::{Serialize, Deserialize};

/// 成员关注及取消关注事件
/// 成员已经加入企业，管理员添加成员到应用可见范围(或移除可见范围)时
/// 成员已经在应用可见范围，成员加入(或退出)企业时


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpSubscribeEvent {
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

