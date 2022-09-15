use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplateSendJobFinishEvent {
    #[serde(rename="FromUserName")]
    pub source: String,
    #[serde(rename="ToUserName")]
    pub target: String,
    #[serde(rename="CreateTime")]
    pub create_time: i64,
    #[serde(rename="MsgId")]
    pub id: i64,
    #[serde(rename="Event")]
    pub event: String
}

#[cfg(test)]
mod tests {
    use crate::events::TemplateSendJobFinishEvent;
    use crate::XmlMessageParser;

    #[test]
    fn test_from_xml() {
        let xml = "<xml><ToUserName><![CDATA[ToUserName]]></ToUserName>
        <FromUserName><![CDATA[FromUserName]]></FromUserName>
        <CreateTime>1661061510</CreateTime>
        <MsgType><![CDATA[event]]></MsgType>
        <Event><![CDATA[TEMPLATESENDJOBFINISH]]></Event>
        <MsgID>MsgID</MsgID>
        <Status><![CDATA[success]]></Status>
        </xml>";
        let msg = TemplateSendJobFinishEvent::from_xml(xml).unwrap();

        assert_eq!("fromUser", &msg.source);
        assert_eq!("toUser", &msg.target);
        assert_eq!("unsubscribe", &msg.event);
    }
}