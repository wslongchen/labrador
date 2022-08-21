use chrono::NaiveDateTime;

use crate::wechat::mp::messages::MessageParser;
use crate::xmlutil;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ImageMessage {
    pub source: String,
    pub target: String,
    pub time: i64,
    pub create_time: NaiveDateTime,
    pub id: i64,
    pub media_id: String,
    pub image: String,
    pub raw: String,
}

impl MessageParser for ImageMessage {
    type WeChatMessage = ImageMessage;

    #[inline]
    fn from_xml(xml: &str) -> ImageMessage {
        let package = xmlutil::parse(xml);
        let doc = package.as_document();
        let source = xmlutil::evaluate(&doc, "//xml/FromUserName/text()").string();
        let target = xmlutil::evaluate(&doc, "//xml/ToUserName/text()").string();
        let id = xmlutil::evaluate(&doc, "//xml/MsgId/text()").number() as i64;
        let time = xmlutil::evaluate(&doc, "//xml/CreateTime/text()").number() as i64;
        let media_id = xmlutil::evaluate(&doc, "//xml/MediaId/text()").string();
        let image = xmlutil::evaluate(&doc, "//xml/PicUrl/text()").string();
        ImageMessage {
            source: source,
            target: target,
            id: id,
            time: time,
            create_time: NaiveDateTime::from_timestamp(time, 0),
            media_id: media_id,
            image: image,
            raw: xml.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::wechat::{messages::MessageParser};
    use crate::wechat::mp::messages::MessageParser;
    use super::ImageMessage;

    #[test]
    fn test_from_xml() {
        let xml = "<xml>\
        <ToUserName><![CDATA[toUser]]></ToUserName>\
        <FromUserName><![CDATA[fromUser]]></FromUserName>\
        <CreateTime>1348831860</CreateTime>\
        <MsgType><![CDATA[image]]></MsgType>\
        <PicUrl><![CDATA[this is a url]]></PicUrl>\
        <MediaId><![CDATA[media_id]]></MediaId>\
        <MsgId>1234567890123456</MsgId>\
        </xml>";
        let msg = ImageMessage::from_xml(xml);

        assert_eq!("fromUser", &msg.source);
        assert_eq!("toUser", &msg.target);
        assert_eq!(1234567890123456, msg.id);
        assert_eq!(1348831860, msg.time);
        assert_eq!("media_id", &msg.media_id);
        assert_eq!("this is a url", &msg.image);
    }
}
