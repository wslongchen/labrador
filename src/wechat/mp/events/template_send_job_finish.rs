use chrono::NaiveDateTime;

use crate::wechat::mp::messages::MessageParser;
use crate::xmlutil;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TemplateSendJobFinishEvent {
    pub source: String,
    pub target: String,
    pub time: i64,
    pub create_time: NaiveDateTime,
    pub id: i64,
    pub event: String,
    pub raw: String,
}

impl MessageParser for TemplateSendJobFinishEvent {
    type WechatMessage = TemplateSendJobFinishEvent;

    #[inline]
    fn from_xml(xml: &str) -> TemplateSendJobFinishEvent {
        let package = xmlutil::parse(xml);
        let doc = package.as_document();
        let source = xmlutil::evaluate(&doc, "//xml/FromUserName/text()").string();
        let target = xmlutil::evaluate(&doc, "//xml/ToUserName/text()").string();
        let id = xmlutil::evaluate(&doc, "//xml/MsgId/text()").number() as i64;
        let time = xmlutil::evaluate(&doc, "//xml/CreateTime/text()").number() as i64;
        TemplateSendJobFinishEvent {
            source: source,
            target: target,
            id: id,
            time: time,
            create_time: NaiveDateTime::from_timestamp(time, 0),
            event: "templatesendjobfinish".to_owned(),
            raw: xml.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::events::TemplateSendJobFinishEvent;
    use crate::wechat::{messages::MessageParser};
    use super::UnsubscribeEvent;

    #[test]
    fn test_from_xml() {
        let xml = "<xml><ToUserName><![CDATA[ToUserName]]></ToUserName>
        <FromUserName><![CDATA[FromUserName]]></FromUserName>
        <CreateTime>1661061510</CreateTime>
        <MsgType><![CDATA[event]]></MsgType>
        <Event><![CDATA[TEMPLATESENDJOBFINISH]]></Event>
        <MsgID>MsgID</MsgID>
        <Status><![CDATA[success]]></Status>
        </xml>";
        let msg = TemplateSendJobFinishEvent::from_xml(xml);

        assert_eq!("fromUser", &msg.source);
        assert_eq!("toUser", &msg.target);
        assert_eq!("unsubscribe", &msg.event);
        assert_eq!(123456789, msg.time);
    }
}