use chrono::NaiveDateTime;

use crate::wechat::mp::messages::MessageParser;
use crate::xmlutil;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SubscribeScanEvent {
    pub source: String,
    pub target: String,
    pub time: i64,
    pub create_time: NaiveDateTime,
    pub id: i64,
    pub scene_id: String,
    pub ticket: String,
    pub event: String,
    pub raw: String,
}

impl MessageParser for SubscribeScanEvent {
    type WechatMessage = SubscribeScanEvent;

    #[inline]
    fn from_xml(xml: &str) -> SubscribeScanEvent {
        let package = xmlutil::parse(xml);
        let doc = package.as_document();
        let source = xmlutil::evaluate(&doc, "//xml/FromUserName/text()").string();
        let target = xmlutil::evaluate(&doc, "//xml/ToUserName/text()").string();
        let id = xmlutil::evaluate(&doc, "//xml/MsgId/text()").number() as i64;
        let time = xmlutil::evaluate(&doc, "//xml/CreateTime/text()").number() as i64;
        let scene_id = xmlutil::evaluate(&doc, "//xml/EventKey/text()").string();
        let ticket = xmlutil::evaluate(&doc, "//xml/Ticket/text()").string();
        SubscribeScanEvent {
            source: source,
            target: target,
            id: id,
            time: time,
            create_time: NaiveDateTime::from_timestamp(time, 0),
            scene_id: scene_id.replace("qrscene_", ""),
            ticket: ticket,
            event: "subscribe_scan".to_owned(),
            raw: xml.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::wechat::{messages::MessageParser};
    use super::SubscribeScanEvent;

    #[test]
    fn test_from_xml() {
        let xml = "<xml>\
        <ToUserName><![CDATA[toUser]]></ToUserName>\
        <FromUserName><![CDATA[fromUser]]></FromUserName>\
        <CreateTime>123456789</CreateTime>\
        <MsgType><![CDATA[event]]></MsgType>\
        <Event><![CDATA[subscribe]]></Event>\
        <EventKey><![CDATA[qrscene_123123]]></EventKey>\
        <Ticket><![CDATA[TICKET]]></Ticket>\
        </xml>";
        let msg = SubscribeScanEvent::from_xml(xml);

        assert_eq!("fromUser", &msg.source);
        assert_eq!("toUser", &msg.target);
        assert_eq!("subscribe_scan", &msg.event);
        assert_eq!(123456789, msg.time);
        assert_eq!("123123", &msg.scene_id);
        assert_eq!("TICKET", &msg.ticket);
    }
}