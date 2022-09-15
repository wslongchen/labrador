use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubscribeEvent {
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
}

#[cfg(test)]
mod tests {
    use crate::XmlMessageParser;
    use super::SubscribeEvent;

    #[test]
    fn test_from_xml() {
        let xml = "<xml>\
        <ToUserName><![CDATA[toUser]]></ToUserName>\
        <FromUserName><![CDATA[fromUser]]></FromUserName>\
        <CreateTime>123456789</CreateTime>\
        <MsgType><![CDATA[event]]></MsgType>\
        <Event><![CDATA[subscribe]]></Event>\
        </xml>";
        let _msg = SubscribeEvent::from_xml(xml);
    }
}