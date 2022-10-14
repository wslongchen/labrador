use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QualificationVerifySuccessEvent {
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
    #[serde(rename="ExpiredTime")]
    pub expired_time: i64,
}

#[cfg(test)]
mod test {
    use crate::XmlMessageParser;
    use super::QualificationVerifySuccessEvent;

    #[test]
    fn test_from_xml() {
        let xml = "<xml>
        <ToUserName><![CDATA[toUser]]></ToUserName>
        <FromUserName><![CDATA[fromUser]]></FromUserName>
        <CreateTime>123456789</CreateTime>
        <MsgType><![CDATA[event]]></MsgType>
        <Event><![CDATA[qualification_verify_success]]></Event>
        <ExpiredTime>987654321</ExpiredTime>
        </xml>";
        let msg = QualificationVerifySuccessEvent::from_xml(xml).unwrap();

        assert_eq!("fromUser", &msg.source);
        assert_eq!("toUser", &msg.target);
        assert_eq!("qualification_verify_success", &msg.event);
        assert_eq!(987654321, msg.expired_time);
    }
}
