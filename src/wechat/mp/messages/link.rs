use chrono::NaiveDateTime;

use crate::wechat::mp::messages::MessageParser;
use crate::xmlutil;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LinkMessage {
    pub source: String,
    pub target: String,
    pub time: i64,
    pub create_time: NaiveDateTime,
    pub id: i64,
    pub title: String,
    pub description: String,
    pub url: String,
    pub raw: String,
}

impl MessageParser for LinkMessage {
    type WechatMessage = LinkMessage;

    #[inline]
    fn from_xml(xml: &str) -> LinkMessage {
        let package = xmlutil::parse(xml);
        let doc = package.as_document();
        let source = xmlutil::evaluate(&doc, "//xml/FromUserName/text()").string();
        let target = xmlutil::evaluate(&doc, "//xml/ToUserName/text()").string();
        let id = xmlutil::evaluate(&doc, "//xml/MsgId/text()").number() as i64;
        let time = xmlutil::evaluate(&doc, "//xml/CreateTime/text()").number() as i64;
        let title = xmlutil::evaluate(&doc, "//xml/Title/text()").string();
        let description = xmlutil::evaluate(&doc, "//xml/Description/text()").string();
        let url = xmlutil::evaluate(&doc, "//xml/Url/text()").string();
        LinkMessage {
            source: source,
            target: target,
            id: id,
            time: time,
            create_time: NaiveDateTime::from_timestamp(time, 0),
            title: title,
            description: description,
            url: url,
            raw: xml.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::wechat::{messages::MessageParser};
    use crate::wechat::mp::messages::MessageParser;
    use super::LinkMessage;

    #[test]
    fn test_from_xml() {
        let xml = "<xml>\
        <ToUserName><![CDATA[toUser]]></ToUserName>\
        <FromUserName><![CDATA[fromUser]]></FromUserName>\
        <CreateTime>1348831860</CreateTime>\
        <MsgType><![CDATA[link]]></MsgType>\
        <Title><![CDATA[公众平台官网链接]]></Title>\
        <Description><![CDATA[公众平台官网链接]]></Description>\
        <Url><![CDATA[url]]></Url>\
        <MsgId>1234567890123456</MsgId>\
        </xml>";
        let msg = LinkMessage::from_xml(xml);

        assert_eq!("fromUser", &msg.source);
        assert_eq!("toUser", &msg.target);
        assert_eq!(1234567890123456, msg.id);
        assert_eq!(1348831860, msg.time);
        assert_eq!("公众平台官网链接", &msg.title);
        assert_eq!("公众平台官网链接", &msg.description);
        assert_eq!("url", &msg.url);
    }
}
