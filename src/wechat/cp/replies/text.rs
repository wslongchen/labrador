use crate::{current_timestamp, ReplyRenderer};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CpTextReply {
    pub source: String,
    pub target: String,
    pub time: i64,
    pub content: String,
}

#[allow(unused)]
impl CpTextReply {
    #[inline]
    pub fn new<S: Into<String>>(source: S, target: S, content: S) -> CpTextReply {
        CpTextReply {
            source: source.into(),
            target: target.into(),
            time: current_timestamp(),
            content: content.into(),
        }
    }
}

impl ReplyRenderer for CpTextReply {
    #[inline]
    fn render(&self) -> String {
        format!("<xml>\n\
            <ToUserName><![CDATA[{target}]]></ToUserName>\n\
            <FromUserName><![CDATA[{source}]]></FromUserName>\n\
            <CreateTime>{time}</CreateTime>\n\
            <MsgType><![CDATA[text]]></MsgType>\n\
            <Content><![CDATA[{content}]]></Content>\n\
            </xml>",
            target=self.target,
            source=self.source,
            time=self.time,
            content=self.content
        )
    }
}

#[cfg(test)]
mod tests {
    use super::ReplyRenderer;
    use super::CpTextReply;

    #[test]
    fn test_render_text_reply() {
        let reply = CpTextReply::new("test1", "test2", "test");
        let rendered = reply.render();
        assert!(rendered.contains("test1"));
        assert!(rendered.contains("test2"));
        assert!(rendered.contains("test"));
    }
}
