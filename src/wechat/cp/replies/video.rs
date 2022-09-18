use crate::{current_timestamp, ReplyRenderer};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CpVideoReply {
    pub source: String,
    pub target: String,
    pub time: i64,
    pub media_id: String,
    pub title: String,
    pub description: String,
}

#[allow(unused)]
impl CpVideoReply {
    #[inline]
    pub fn new<S: Into<String>>(source: S, target: S, media_id: S) -> CpVideoReply {
        CpVideoReply {
            source: source.into(),
            target: target.into(),
            time: current_timestamp(),
            media_id: media_id.into(),
            title: "".to_owned(),
            description: "".to_owned(),
        }
    }
}

impl ReplyRenderer for CpVideoReply {
    #[inline]
    fn render(&self) -> String {
        format!("<xml>\n\
            <ToUserName><![CDATA[{target}]]></ToUserName>\n\
            <FromUserName><![CDATA[{source}]]></FromUserName>\n\
            <CreateTime>{time}</CreateTime>\n\
            <MsgType><![CDATA[video]]></MsgType>\n\
            <Video>\n\
            <MediaId><![CDATA[{media_id}]]></MediaId>\n\
            <Title><![CDATA[{title}]]></Title>\n\
            <Description><![CDATA[{description}]]></Description>\n\
            </Video>\n\
            </xml>",
            target=self.target,
            source=self.source,
            time=self.time,
            media_id=self.media_id,
            title=self.title,
            description=self.description,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::ReplyRenderer;
    use super::CpVideoReply;

    #[test]
    fn test_render_video_reply() {
        let reply = CpVideoReply::new("test1", "test2", "test");
        let rendered = reply.render();
        assert!(rendered.contains("test1"));
        assert!(rendered.contains("test2"));
        assert!(rendered.contains("test"));
    }
}
