use serde::{Serialize, Deserialize};

/// 自动激活回调通知
/// 企业成员满足自动激活条件并触发自动激活后，企业微信后台会推送“自动激活通知”到服务商的系统事件接收URL（应用管理 -通用开发参数-系统事件接收URL）。
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpAutoActivateEvent {
    #[serde(rename="AuthCorpId")]
    pub auth_corp_id: String,
    #[serde(rename="ServiceCorpId")]
    pub service_corp_id: String,
    #[serde(rename="TimeStamp")]
    pub time_stamp: i64,
    #[serde(rename="InfoType")]
    pub into_type: String,
    #[serde(rename="OrderId")]
    pub order_id: String,
    /// 许可自动激活的时机，1:企业成员主动访问应用，2:服务商调用消息推送接口，3:服务商调用互通接口
    #[serde(rename="Scene")]
    pub scene: i64,
    #[serde(rename="AccountList")]
    pub account_list: Option<AccountList>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountList {
    /// 自动激活的许可帐号激活码
    #[serde(rename="ActiveCode")]
    pub active_code: String,
    /// 自动激活的许可的类型，1:基础许可，2:互通许可
    #[serde(rename="Type")]
    pub account_type: u8,
    /// 自动激活后，该许可的到期时间
    #[serde(rename="ExpireTime")]
    pub expire_time: i64,
    /// 许可自动激活的成员的UserID
    #[serde(rename="UserId")]
    pub user_id: String,
    /// 激活成员自动激活前的许可状态，1:未激活许可，2:已激活许可且许可未过期（即许可的剩余时长小于等于7天），3:已激活许可且许可已过期
    #[serde(rename="PreviousStatus")]
    pub previous_status: Option<u8>,
    /// 仅针对已激活的成员进行自动激活时返回，返回该成员之前激活的旧的激活码
    #[serde(rename="PreviousActiveCode")]
    pub previous_active_code: Option<String>,
}
