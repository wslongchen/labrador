use chrono::NaiveDateTime;


use crate::wechat::mp::messages::MessageParser;
use crate::xmlutil;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TextMessage {
    pub source: String,
    pub target: String,
    pub time: i64,
    pub create_time: NaiveDateTime,
    pub id: i64,
    pub content: String,
    pub raw: String,
}

impl MessageParser for TextMessage {
    type WeChatMessage = TextMessage;

    #[inline]
    fn from_xml(xml: &str) -> TextMessage {
        let package = xmlutil::parse(xml);
        let doc = package.as_document();
        let source = xmlutil::evaluate(&doc, "//xml/FromUserName/text()").string();
        let target = xmlutil::evaluate(&doc, "//xml/ToUserName/text()").string();
        let id = xmlutil::evaluate(&doc, "//xml/MsgId/text()").number() as i64;
        let time = xmlutil::evaluate(&doc, "//xml/CreateTime/text()").number() as i64;
        let content = xmlutil::evaluate(&doc, "//xml/Content/text()").string();
        TextMessage {
            source: source,
            target: target,
            id: id,
            time: time,
            create_time: NaiveDateTime::from_timestamp(time, 0),
            content: content,
            raw: xml.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::wechat::{messages::MessageParser};
    use super::TextMessage;

    #[test]
    fn test_from_xml() {
        let xml = "<xml>\
        <ToUserName><![CDATA[toUser]]></ToUserName>\
        <FromUserName><![CDATA[fromUser]]></FromUserName>\
        <CreateTime>1348831860</CreateTime>\
        <MsgType><![CDATA[text]]></MsgType>\
        <Content><![CDATA[this is a test]]></Content>\
        <MsgId>1234567890123456</MsgId>\
        </xml>";
        let msg = TextMessage::from_xml(xml);

        assert_eq!("fromUser", &msg.source);
        assert_eq!("toUser", &msg.target);
        assert_eq!(1234567890123456, msg.id);
        assert_eq!(1348831860, msg.time);
        assert_eq!("this is a test", &msg.content);
    }
}
