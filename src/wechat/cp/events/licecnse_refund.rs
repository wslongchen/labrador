use serde::{Serialize, Deserialize};

/// 支付成功通知
/// 当服务商购买接口调用许可帐号并完成付款后，企业微信后台会推送“支付成功通知”到服务商的系统事件接收URL（应用管理 -通用开发参数-系统事件接收URL）。
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpLicenseRefundEvent {
    #[serde(rename="AuthCorpId")]
    pub auth_corp_id: String,
    #[serde(rename="ServiceCorpId")]
    pub service_corp_id: String,
    #[serde(rename="InfoType")]
    pub into_type: String,
    #[serde(rename="OrderId")]
    pub order_id: String,
    /// 订单状态，1:退款成功，2:退款被拒绝。
    #[serde(rename="OrderStatus")]
    pub order_status: u8,
    #[serde(rename="TimeStamp")]
    pub timestamp: i64,
}
