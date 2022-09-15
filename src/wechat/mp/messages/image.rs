use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageMessage {
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
    #[serde(rename="PicUrl")]
    pub image: String,
}


#[cfg(test)]
mod tests {
    use crate::XmlMessageParser;
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
        let msg = ImageMessage::from_xml(xml).unwrap();

        assert_eq!("fromUser", &msg.source);
        assert_eq!("toUser", &msg.target);
        assert_eq!(1234567890123456, msg.id);
        assert_eq!("media_id", &msg.media_id);
        assert_eq!("this is a url", &msg.image);
    }
}
