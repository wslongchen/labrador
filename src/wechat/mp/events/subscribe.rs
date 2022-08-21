use chrono::NaiveDateTime;

use crate::wechat::mp::messages::MessageParser;
use crate::xmlutil;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SubscribeEvent {
    pub source: String,
    pub target: String,
    pub time: i64,
    pub create_time: NaiveDateTime,
    pub id: i64,
    pub event: String,
    pub raw: String,
}

impl MessageParser for SubscribeEvent {
    type WeChatMessage = SubscribeEvent;

    #[inline]
    fn from_xml(xml: &str) -> SubscribeEvent {
        let package = xmlutil::parse(xml);
        let doc = package.as_document();
        let source = xmlutil::evaluate(&doc, "//xml/FromUserName/text()").string();
        let target = xmlutil::evaluate(&doc, "//xml/ToUserName/text()").string();
        let id = xmlutil::evaluate(&doc, "//xml/MsgId/text()").number() as i64;
        let time = xmlutil::evaluate(&doc, "//xml/CreateTime/text()").number() as i64;
        SubscribeEvent {
            source: source,
            target: target,
            id: id,
            time: time,
            create_time: NaiveDateTime::from_timestamp(time, 0),
            event: "subscribe".to_owned(),
            raw: xml.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::wechat::{messages::MessageParser};
    use super::SubscribeEvent;

    #[test]
    fn test_from_xml() {
        let xml = "<xml>\
        <ToUserName><![CDATA[toUser]]></ToUserName>\
        <FromUserName><![CDATA[fromUser]]></FromUserName>\
        <CreateTime>123456789</CreateTime>\
        <MsgType><![CDATA[event]]></MsgType>\
        <Event><![CDATA[subscribe]]></Event>\
        </xml>";
        let msg = SubscribeEvent::from_xml(xml);

        assert_eq!("fromUser", &msg.source);
        assert_eq!("toUser", &msg.target);
        assert_eq!("subscribe", &msg.event);
        assert_eq!(123456789, msg.time);
    }
}