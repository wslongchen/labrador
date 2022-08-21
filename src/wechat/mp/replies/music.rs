use crate::current_timestamp;
use super::ReplyRenderer;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MusicReply {
    pub source: String,
    pub target: String,
    pub time: i64,
    pub thumb_media_id: String,
    pub title: String,
    pub description: String,
    pub music_url: String,
    pub hq_music_url: String,
}

#[allow(unused)]
impl MusicReply {
    #[inline]
    pub fn new<S: Into<String>>(source: S, target: S, thumb_media_id: S) -> MusicReply {
        MusicReply {
            source: source.into(),
            target: target.into(),
            time: current_timestamp(),
            thumb_media_id: thumb_media_id.into(),
            title: "".to_owned(),
            description: "".to_owned(),
            music_url: "".to_owned(),
            hq_music_url: "".to_owned(),
        }
    }
}

impl ReplyRenderer for MusicReply {
    #[inline]
    fn render(&self) -> String {
        format!("<xml>\n\
            <ToUserName><![CDATA[{target}]]></ToUserName>\n\
            <FromUserName><![CDATA[{source}]]></FromUserName>\n\
            <CreateTime>{time}</CreateTime>\n\
            <MsgType><![CDATA[music]]></MsgType>\n\
            <Music>\n\
                <ThumbMediaId><![CDATA[{thumb_media_id}]]></ThumbMediaId>\n\
                <Title><![CDATA[{title}]]></Title>\n\
                <Description><![CDATA[{description}]]></Description>\n\
                <MusicUrl><![CDATA[{music_url}]]></MusicUrl>\n\
                <HQMusicUrl><![CDATA[{hq_music_url}]]></HQMusicUrl>\n\
            </Music>\n\
            </xml>",
            target=self.target,
            source=self.source,
            time=self.time,
            thumb_media_id=self.thumb_media_id,
            title=self.title,
            description=self.description,
            music_url=self.music_url,
            hq_music_url=self.hq_music_url,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::ReplyRenderer;
    use super::MusicReply;

    #[test]
    fn test_render_music_reply() {
        let reply = MusicReply::new("test1", "test2", "test");
        let rendered = reply.render();
        assert!(rendered.contains("test1"));
        assert!(rendered.contains("test2"));
        assert!(rendered.contains("test"));
    }
}
