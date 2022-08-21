use chrono::NaiveDateTime;


use crate::wechat::mp::messages::MessageParser;
use crate::xmlutil;

#[derive(Debug, Clone)]
pub struct LocationMessage {
    pub source: String,
    pub target: String,
    pub time: i64,
    pub create_time: NaiveDateTime,
    pub id: i64,
    pub location_x: f64,
    pub location_y: f64,
    pub location: (f64, f64),
    pub scale: usize,
    pub label: String,
    pub raw: String,
}

impl MessageParser for LocationMessage {
    type WeChatMessage = LocationMessage;

    #[inline]
    fn from_xml(xml: &str) -> LocationMessage {
        let package = xmlutil::parse(xml);
        let doc = package.as_document();
        let source = xmlutil::evaluate(&doc, "//xml/FromUserName/text()").string();
        let target = xmlutil::evaluate(&doc, "//xml/ToUserName/text()").string();
        let id = xmlutil::evaluate(&doc, "//xml/MsgId/text()").number() as i64;
        let time = xmlutil::evaluate(&doc, "//xml/CreateTime/text()").number() as i64;
        let location_x = xmlutil::evaluate(&doc, "//xml/Location_X/text()").number();
        let location_y = xmlutil::evaluate(&doc, "//xml/Location_Y/text()").number();
        let scale = xmlutil::evaluate(&doc, "//xml/Scale/text()").number() as usize;
        let label = xmlutil::evaluate(&doc, "//xml/Label/text()").string();
        LocationMessage {
            source: source,
            target: target,
            id: id,
            time: time,
            create_time: NaiveDateTime::from_timestamp(time, 0),
            location_x: location_x,
            location_y: location_y,
            location: (location_x, location_y),
            scale: scale,
            label: label,
            raw: xml.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::wechat::{messages::MessageParser};
    use crate::wechat::mp::messages::MessageParser;
    use super::LocationMessage;

    #[test]
    fn test_from_xml() {
        let xml = "<xml>\
        <ToUserName><![CDATA[toUser]]></ToUserName>\
        <FromUserName><![CDATA[fromUser]]></FromUserName>\
        <CreateTime>1348831860</CreateTime>\
        <MsgType><![CDATA[location]]></MsgType>\
        <Location_X>23.134521</Location_X>\
        <Location_Y>113.358803</Location_Y>
        <Scale>20</Scale>\
        <Label><![CDATA[位置信息]]></Label>
        <MsgId>1234567890123456</MsgId>\
        </xml>";
        let msg = LocationMessage::from_xml(xml);

        assert_eq!("fromUser", &msg.source);
        assert_eq!("toUser", &msg.target);
        assert_eq!(1234567890123456, msg.id);
        assert_eq!(1348831860, msg.time);
        assert_eq!(23, msg.location_x as usize);
        assert_eq!(113, msg.location_y as usize);
        assert_eq!(20, msg.scale);
        assert_eq!("位置信息", &msg.label);
    }
}
