pub trait ReplyRenderer {
    fn render(&self) -> String;
}

mod text;
mod image;
mod voice;
mod video;
mod articles;
mod update_button;
mod template_card;

pub use self::text::TextReply;
pub use self::image::ImageReply;
pub use self::voice::VoiceReply;
pub use self::video::VideoReply;
pub use self::articles::ArticlesReply;
pub use self::update_button::*;
pub use self::template_card::*;


#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Reply {
    TextReply(TextReply),
    ImageReply(ImageReply),
    VoiceReply(VoiceReply),
    VideoReply(VideoReply),
    ArticlesReply(ArticlesReply),
    UpdateButtonReply(UpdateButtonReply),
    TemplateCard(TemplateCardTextReply),
}

#[allow(unused)]
impl Reply {
    pub fn render(&self) -> String {
        let reply = match *self {
            Reply::TextReply(ref r) => r.render(),
            Reply::ImageReply(ref r) => r.render(),
            Reply::VoiceReply(ref r) => r.render(),
            Reply::VideoReply(ref r) => r.render(),
            Reply::ArticlesReply(ref r) => r.render(),
            Reply::TemplateCard(ref r) => r.render(),
            Reply::UpdateButtonReply(ref r) => r.render(),
        };
        reply
    }
}
