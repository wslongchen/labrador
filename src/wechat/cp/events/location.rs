use serde::{Serialize, Deserialize};

/// 上报地理位置
/// 成员同意上报地理位置后，每次在进入应用会话时都会上报一次地理位置。
/// 企业可以在管理端修改应用是否需要获取地理位置权限。
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpLocationEvent {
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
    #[serde(rename="Latitude")]
    pub latitude: f64,
    #[serde(rename="Longitude")]
    pub longitude: f64,
    #[serde(rename="Precision")]
    pub precision: f64,
    #[serde(rename="AgentID")]
    pub agent_id: i64,
}

#[cfg(test)]
mod tests {
    use crate::XmlMessageParser;
    use super::CpLocationEvent;

    #[test]
    fn test_from_xml() {
        let xml = "<xml>\
        <ToUserName><![CDATA[toUser]]></ToUserName>\
        <FromUserName><![CDATA[fromUser]]></FromUserName>\
        <CreateTime>123456789</CreateTime>\
        <MsgType><![CDATA[event]]></MsgType>\
        <Event><![CDATA[LOCATION]]></Event>\
        <Latitude>23.137466</Latitude>\
        <Longitude>113.352425</Longitude>\
        <Precision>119.385040</Precision>\
        </xml>";
        let msg = CpLocationEvent::from_xml(xml).unwrap();

        assert_eq!("fromUser", &msg.source);
        assert_eq!("toUser", &msg.target);
        assert_eq!("location", &msg.event);
        assert_eq!(23, msg.latitude as usize);
        assert_eq!(113, msg.longitude as usize);
        assert_eq!(119, msg.precision as usize);
    }
}