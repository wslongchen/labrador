use chrono::NaiveDateTime;

use crate::wechat::mp::messages::MessageParser;
use crate::xmlutil;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct QualificationVerifySuccessEvent {
    pub source: String,
    pub target: String,
    pub time: i64,
    pub create_time: NaiveDateTime,
    pub id: i64,
    pub expired_time: i64,
    pub event: String,
    pub raw: String,
}

impl MessageParser for QualificationVerifySuccessEvent {
    type WechatMessage = QualificationVerifySuccessEvent;

    #[inline]
    fn from_xml(xml: &str) -> QualificationVerifySuccessEvent {
        let package = xmlutil::parse(xml);
        let doc = package.as_document();
        let source = xmlutil::evaluate(&doc, "//xml/FromUserName/text()").string();
        let target = xmlutil::evaluate(&doc, "//xml/ToUserName/text()").string();
        let id = xmlutil::evaluate(&doc, "//xml/MsgId/text()").number() as i64;
        let time = xmlutil::evaluate(&doc, "//xml/CreateTime/text()").number() as i64;
        let expired_time = xmlutil::evaluate(&doc, "//xml/ExpiredTime/text()").number() as i64;
        QualificationVerifySuccessEvent {
            source: source,
            target: target,
            id: id,
            time: time,
            create_time: NaiveDateTime::from_timestamp(time, 0),
            expired_time: expired_time,
            event: "qualification_verify_success".to_owned(),
            raw: xml.to_owned(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::wechat::{messages::MessageParser};
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
        let msg = QualificationVerifySuccessEvent::from_xml(xml);

        assert_eq!("fromUser", &msg.source);
        assert_eq!("toUser", &msg.target);
        assert_eq!("qualification_verify_success", &msg.event);
        assert_eq!(123456789, msg.time);
        assert_eq!(987654321, msg.expired_time);
    }
}
