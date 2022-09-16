use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationEvent {
    #[serde(rename="FromUserName")]
    pub source: String,
    #[serde(rename="ToUserName")]
    pub target: String,
    #[serde(rename="CreateTime")]
    pub create_time: i64,
    #[serde(rename="MsgId")]
    pub id: i64,
    #[serde(rename="Event")]
    pub event: String,
    #[serde(rename="Latitude")]
    pub latitude: f64,
    #[serde(rename="Longitude")]
    pub longitude: f64,
    #[serde(rename="Precision")]
    pub precision: f64,
}

#[cfg(test)]
mod tests {
    use crate::XmlMessageParser;
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
        let msg = LocationEvent::from_xml(xml).unwrap();

        assert_eq!("fromUser", &msg.source);
        assert_eq!("toUser", &msg.target);
        assert_eq!("location", &msg.event);
        assert_eq!(23, msg.latitude as usize);
        assert_eq!(113, msg.longitude as usize);
        assert_eq!(119, msg.precision as usize);
    }
}