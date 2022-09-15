

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpVoiceMessage {
    #[serde(rename="FromUserName")]
    pub source: String,
    #[serde(rename="ToUserName")]
    pub target: String,
    #[serde(rename="CreateTime")]
    pub create_time: i64,
    #[serde(rename="MsgId")]
    pub id: i64,
    #[serde(rename="MediaId")]
    pub media_id: String,
    #[serde(rename="Format")]
    pub format: String,
    #[serde(rename="AgentID")]
    pub agent_id: i64,
}


#[cfg(test)]
mod tests {
    use crate::XmlMessageParser;
    use super::CpVoiceMessage;

    #[test]
    fn test_from_xml() {
        let xml = "<xml>\
        <ToUserName><![CDATA[toUser]]></ToUserName>\
        <FromUserName><![CDATA[fromUser]]></FromUserName>\
        <CreateTime>1348831860</CreateTime>\
        <MsgType><![CDATA[voice]]></MsgType>\
        <MediaId><![CDATA[media_id]]></MediaId>\
        <Format><![CDATA[Format]]></Format>\
        <MsgId>1234567890123456</MsgId>\
        </xml>";
        let msg = CpVoiceMessage::from_xml(xml).unwrap();

        assert_eq!("fromUser", &msg.source);
        assert_eq!("toUser", &msg.target);
        assert_eq!(1234567890123456, msg.id);
        assert_eq!("media_id", &msg.media_id);
        assert_eq!("Format", &msg.format);
    }
}
