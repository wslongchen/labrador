use crate::current_timestamp;
use super::ReplyRenderer;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ImageReply {
    pub source: String,
    pub target: String,
    pub time: i64,
    pub media_id: String,
}

#[allow(unused)]
impl ImageReply {
    #[inline]
    pub fn new<S: Into<String>>(source: S, target: S, media_id: S) -> ImageReply {
        ImageReply {
            source: source.into(),
            target: target.into(),
            time: current_timestamp(),
            media_id: media_id.into(),
        }
    }
}

impl ReplyRenderer for ImageReply {
    #[inline]
    fn render(&self) -> String {
        format!("<xml>\n\
            <ToUserName><![CDATA[{target}]]></ToUserName>\n\
            <FromUserName><![CDATA[{source}]]></FromUserName>\n\
            <CreateTime>{time}</CreateTime>\n\
            <MsgType><![CDATA[image]]></MsgType>\n\
            <Image>\n\
            <MediaId><![CDATA[{media_id}]]></MediaId>\n\
            </Image>\n\
            </xml>",
            target=self.target,
            source=self.source,
            time=self.time,
            media_id=self.media_id
        )
    }
}

#[cfg(test)]
mod tests {
    use super::ReplyRenderer;
    use super::ImageReply;

    #[test]
    fn test_render_image_reply() {
        let reply = ImageReply::new("test1", "test2", "test");
        let rendered = reply.render();
        assert!(rendered.contains("test1"));
        assert!(rendered.contains("test2"));
        assert!(rendered.contains("test"));
    }
}
