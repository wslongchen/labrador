mod text;
mod image;
mod voice;
mod video;
mod music;
mod articles;
mod transfer_customer_service;

use crate::ReplyRenderer;
pub use self::text::TextReply;
pub use self::image::ImageReply;
pub use self::voice::VoiceReply;
pub use self::video::VideoReply;
pub use self::music::MusicReply;
pub use self::articles::ArticlesReply;
pub use self::transfer_customer_service::TransferCustomerServiceReply;


#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Reply {
    TextReply(TextReply),
    ImageReply(ImageReply),
    VoiceReply(VoiceReply),
    VideoReply(VideoReply),
    MusicReply(MusicReply),
    ArticlesReply(ArticlesReply),
    TransferCustomerServiceReply(TransferCustomerServiceReply),
}

#[allow(unused)]
impl Reply {
    pub fn render(&self) -> String {
        let reply = match *self {
            Reply::TextReply(ref r) => r.render(),
            Reply::ImageReply(ref r) => r.render(),
            Reply::VoiceReply(ref r) => r.render(),
            Reply::VideoReply(ref r) => r.render(),
            Reply::MusicReply(ref r) => r.render(),
            Reply::ArticlesReply(ref r) => r.render(),
            Reply::TransferCustomerServiceReply(ref r) => r.render(),
        };
        reply
    }
}
