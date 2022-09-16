
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpLinkMessage {
    #[serde(rename="FromUserName")]
    pub source: String,
    #[serde(rename="ToUserName")]
    pub target: String,
    #[serde(rename="CreateTime")]
    pub create_time: i64,
    #[serde(rename="MsgId")]
    pub id: i64,
    #[serde(rename="Title")]
    pub title: String,
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="Url")]
    pub url: String,
    #[serde(rename="AgentID")]
    pub agent_id: i64,
}


#[cfg(test)]
mod tests {
    use crate::XmlMessageParser;
    use super::CpLinkMessage;

    #[test]
    fn test_from_xml() {
        let xml = "<xml>\
        <ToUserName><![CDATA[toUser]]></ToUserName>\
        <FromUserName><![CDATA[fromUser]]></FromUserName>\
        <CreateTime>1348831860</CreateTime>\
        <MsgType><![CDATA[link]]></MsgType>\
        <Title><![CDATA[公众平台官网链接]]></Title>\
        <Description><![CDATA[公众平台官网链接]]></Description>\
        <Url><![CDATA[url]]></Url>\
        <MsgId>1234567890123456</MsgId>\
        </xml>";
        let msg = CpLinkMessage::from_xml(xml).unwrap();

        assert_eq!("fromUser", &msg.source);
        assert_eq!("toUser", &msg.target);
        assert_eq!(1234567890123456, msg.id);
        assert_eq!("公众平台官网链接", &msg.title);
        assert_eq!("公众平台官网链接", &msg.description);
        assert_eq!("url", &msg.url);
    }
}
