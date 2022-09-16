use serde::{Serialize, Deserialize};

/// 接口许可失效通知
/// 当许可帐号失效（未激活或已过期）的企业成员访问应用或小程序时，企业微信会提示用户联系服务商开通许可帐号，同时企业微信自动推送该通知事件至服务商后台。接收消息的方式参见使用接收消息。
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpUnlicensedNotifyEvent {
    #[serde(rename="FromUserName")]
    pub source: String,
    #[serde(rename="ToUserName")]
    pub target: String,
    #[serde(rename="CreateTime")]
    pub create_time: i64,
    #[serde(rename="Event")]
    pub event: String,
    #[serde(rename="MsgType")]
    pub mst_type: String,
    #[serde(rename="AgentID")]
    pub agent_id: i64,
}
