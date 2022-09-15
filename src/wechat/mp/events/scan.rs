use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScanEvent {
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
    use super::ScanEvent;

    #[test]
    fn test_from_xml() {
        let xml = "<xml>\
        <ToUserName><![CDATA[toUser]]></ToUserName>\
        <FromUserName><![CDATA[fromUser]]></FromUserName>\
        <CreateTime>123456789</CreateTime>\
        <MsgType><![CDATA[event]]></MsgType>\
        <Event><![CDATA[SCAN]]></Event>\
        <EventKey><![CDATA[SCENE_VALUE]]></EventKey>\
        <Ticket><![CDATA[TICKET]]></Ticket>\
        </xml>";
        let _msg = ScanEvent::from_xml(xml);

    }
}