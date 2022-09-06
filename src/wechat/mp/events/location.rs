use chrono::NaiveDateTime;

use crate::wechat::mp::messages::MessageParser;
use crate::xmlutil;

#[derive(Debug, Clone)]
pub struct LocationEvent {
    pub source: String,
    pub target: String,
    pub time: i64,
    pub create_time: NaiveDateTime,
    pub id: i64,
    pub latitude: f64,
    pub longitude: f64,
    pub precision: f64,
    pub event: String,
    pub raw: String,
}

impl MessageParser for LocationEvent {
    type WechatMessage = LocationEvent;

    #[inline]
    fn from_xml(xml: &str) -> LocationEvent {
        let package = xmlutil::parse(xml);
        let doc = package.as_document();
        let source = xmlutil::evaluate(&doc, "//xml/FromUserName/text()").string();
        let target = xmlutil::evaluate(&doc, "//xml/ToUserName/text()").string();
        let id = xmlutil::evaluate(&doc, "//xml/MsgId/text()").number() as i64;
        let time = xmlutil::evaluate(&doc, "//xml/CreateTime/text()").number() as i64;
        let latitude = xmlutil::evaluate(&doc, "//xml/Latitude/text()").number() as f64;
        let longitude = xmlutil::evaluate(&doc, "//xml/Longitude/text()").number() as f64;
        let precision = xmlutil::evaluate(&doc, "//xml/Precision/text()").number() as f64;
        LocationEvent {
            source: source,
            target: target,
            id: id,
            time: time,
            create_time: NaiveDateTime::from_timestamp(time, 0),
            latitude: latitude,
            longitude: longitude,
            precision: precision,
            event: "location".to_owned(),
            raw: xml.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::wechat::{messages::MessageParser};
    use super::LocationEvent;

    #[test]
    fn test_from_xml() {
        let xml = "<xml>\
        <ToUserName><![CDATA[toUser]]></ToUserName>\
        <FromUserName><![CDATA[fromUser]]></FromUserName>\
        <CreateTime>123456789</CreateTime>\
        <MsgType><![CDATA[event]]></MsgType>\
        <Event><![CDATA[LOCATION]]></Event>\
        <Latitude>23.137466</Latitude>\
        <Longitude>113.352425</Longitude>\
        <Precision>119.385040</Precision>\
        </xml>";
        let msg = LocationEvent::from_xml(xml);

        assert_eq!("fromUser", &msg.source);
        assert_eq!("toUser", &msg.target);
        assert_eq!("location", &msg.event);
        assert_eq!(123456789, msg.time);
        assert_eq!(23, msg.latitude as usize);
        assert_eq!(113, msg.longitude as usize);
        assert_eq!(119, msg.precision as usize);
    }
}