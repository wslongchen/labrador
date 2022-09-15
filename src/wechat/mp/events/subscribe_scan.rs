use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubscribeScanEvent {
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
    #[serde(rename="EventKey")]
    pub scene_id: String,
    #[serde(rename="Ticket")]
    pub ticket: String,
}

#[cfg(test)]
mod tests {
    use crate::XmlMessageParser;
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
        let msg = SubscribeScanEvent::from_xml(xml).unwrap();

        assert_eq!("fromUser", &msg.source);
        assert_eq!("toUser", &msg.target);
        assert_eq!("subscribe_scan", &msg.event);
        assert_eq!("123123", &msg.scene_id);
        assert_eq!("TICKET", &msg.ticket);
    }
}