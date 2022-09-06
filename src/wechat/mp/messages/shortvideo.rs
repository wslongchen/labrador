use chrono::NaiveDateTime;


use crate::wechat::mp::messages::MessageParser;
use crate::xmlutil;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ShortVideoMessage {
    pub source: String,
    pub target: String,
    pub time: i64,
    pub create_time: NaiveDateTime,
    pub id: i64,
    pub media_id: String,
    pub thumb_media_id: String,
    pub raw: String,
}

impl MessageParser for ShortVideoMessage {
    type WechatMessage = ShortVideoMessage;

    #[inline]
    fn from_xml(xml: &str) -> ShortVideoMessage {
        let package = xmlutil::parse(xml);
        let doc = package.as_document();
        let source = xmlutil::evaluate(&doc, "//xml/FromUserName/text()").string();
        let target = xmlutil::evaluate(&doc, "//xml/ToUserName/text()").string();
        let id = xmlutil::evaluate(&doc, "//xml/MsgId/text()").number() as i64;
        let time = xmlutil::evaluate(&doc, "//xml/CreateTime/text()").number() as i64;
        let media_id = xmlutil::evaluate(&doc, "//xml/MediaId/text()").string();
        let thumb_media_id = xmlutil::evaluate(&doc, "//xml/ThumbMediaId/text()").string();
        ShortVideoMessage {
            source: source,
            target: target,
            id: id,
            time: time,
            create_time: NaiveDateTime::from_timestamp(time, 0),
            media_id: media_id,
            thumb_media_id: thumb_media_id,
            raw: xml.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::wechat::{messages::MessageParser};
    use crate::wechat::mp::messages::MessageParser;
    use super::ShortVideoMessage;

    #[test]
    fn test_from_xml() {
        let xml = "<xml>\
        <ToUserName><![CDATA[toUser]]></ToUserName>\
        <FromUserName><![CDATA[fromUser]]></FromUserName>\
        <CreateTime>1348831860</CreateTime>\
        <MsgType><![CDATA[shortvideo]]></MsgType>\
        <MediaId><![CDATA[media_id]]></MediaId>\
        <ThumbMediaId><![CDATA[thumb_media_id]]></ThumbMediaId>\
        <MsgId>1234567890123456</MsgId>\
        </xml>";
        let msg = ShortVideoMessage::from_xml(xml);

        assert_eq!("fromUser", &msg.source);
        assert_eq!("toUser", &msg.target);
        assert_eq!(1234567890123456, msg.id);
        assert_eq!(1348831860, msg.time);
        assert_eq!("media_id", &msg.media_id);
        assert_eq!("thumb_media_id", &msg.thumb_media_id);
    }
}
