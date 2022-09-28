mod text;
mod image;
mod voice;
mod video;
mod articles;
mod update_button;
mod template_card;

use crate::ReplyRenderer;
pub use self::text::CpTextReply;
pub use self::image::CpImageReply;
pub use self::voice::CpVoiceReply;
pub use self::video::CpVideoReply;
pub use self::articles::CpArticlesReply;
pub use self::update_button::*;
pub use self::template_card::*;


#[allow(unused)]
#[derive(Debug, Clone)]
pub enum CpReply {
    TextReply(CpTextReply),
    ImageReply(CpImageReply),
    VoiceReply(CpVoiceReply),
    VideoReply(CpVideoReply),
    ArticlesReply(CpArticlesReply),
    UpdateButtonReply(CpUpdateButtonReply),
    TemplateCard(CpTemplateCardTextReply),
}

#[allow(unused)]
impl CpReply {
    pub fn render(&self) -> String {
        let reply = match *self {
            CpReply::TextReply(ref r) => r.render(),
            CpReply::ImageReply(ref r) => r.render(),
            CpReply::VoiceReply(ref r) => r.render(),
            CpReply::VideoReply(ref r) => r.render(),
            CpReply::ArticlesReply(ref r) => r.render(),
            CpReply::TemplateCard(ref r) => r.render(),
            CpReply::UpdateButtonReply(ref r) => r.render(),
        };
        reply
    }
}
