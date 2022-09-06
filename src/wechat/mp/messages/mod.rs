pub trait MessageParser {
    type WechatMessage;

    fn from_xml(xml: &str) -> Self::WechatMessage;
}

mod text;
mod image;
mod voice;
mod shortvideo;
mod video;
mod location;
mod link;
mod unknown;

use crate::parse_message;
// export Message types
pub use self::text::TextMessage;
pub use self::image::ImageMessage;
pub use self::voice::VoiceMessage;
pub use self::shortvideo::ShortVideoMessage;
pub use self::video::VideoMessage;
pub use self::location::LocationMessage;
pub use self::link::LinkMessage;
pub use self::unknown::UnknownMessage;

// export Event types
pub use super::events::SubscribeEvent;
pub use super::events::UnsubscribeEvent;
pub use super::events::ScanEvent;
pub use super::events::SubscribeScanEvent;
pub use super::events::LocationEvent;
pub use super::events::ClickEvent;
pub use super::events::ViewEvent;
pub use super::events::QualificationVerifySuccessEvent;
pub use super::events::TemplateSendJobFinishEvent;

// an enum or messages and events
#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Message {
    TextMessage(TextMessage),
    ImageMessage(ImageMessage),
    VoiceMessage(VoiceMessage),
    ShortVideoMessage(ShortVideoMessage),
    VideoMessage(VideoMessage),
    LocationMessage(LocationMessage),
    LinkMessage(LinkMessage),
    UnknownMessage(UnknownMessage),
    SubscribeEvent(SubscribeEvent),
    UnsubscribeEvent(UnsubscribeEvent),
    TemplateSendJobFinishEvent(TemplateSendJobFinishEvent),
    ScanEvent(ScanEvent),
    SubscribeScanEvent(SubscribeScanEvent),
    LocationEvent(LocationEvent),
    ClickEvent(ClickEvent),
    ViewEvent(ViewEvent),
    QualificationVerifySuccessEvent(QualificationVerifySuccessEvent),
}

#[allow(unused)]
impl Message {
    pub fn parse<S: AsRef<str>>(xml: S) -> Message {
        parse_message(xml.as_ref())
    }

    pub fn get_source(&self) -> String {
        match *self {
            Message::TextMessage(ref msg) => msg.source.to_owned(),
            Message::ImageMessage(ref msg) => msg.source.to_owned(),
            Message::VoiceMessage(ref msg) => msg.source.to_owned(),
            Message::ShortVideoMessage(ref msg) => msg.source.to_owned(),
            Message::VideoMessage(ref msg) => msg.source.to_owned(),
            Message::LocationMessage(ref msg) => msg.source.to_owned(),
            Message::LinkMessage(ref msg) => msg.source.to_owned(),
            Message::UnknownMessage(ref msg) => msg.source.to_owned(),
            Message::SubscribeEvent(ref msg) => msg.source.to_owned(),
            Message::UnsubscribeEvent(ref msg) => msg.source.to_owned(),
            Message::SubscribeScanEvent(ref msg) => msg.source.to_owned(),
            Message::ScanEvent(ref msg) => msg.source.to_owned(),
            Message::LocationEvent(ref msg) => msg.source.to_owned(),
            Message::ClickEvent(ref msg) => msg.source.to_owned(),
            Message::ViewEvent(ref msg) => msg.source.to_owned(),
            Message::TemplateSendJobFinishEvent(ref msg) => msg.source.to_owned(),
            Message::QualificationVerifySuccessEvent(ref msg) => msg.source.to_owned(),
        }
    }

    pub fn get_target(&self) -> String {
        match *self {
            Message::TextMessage(ref msg) => msg.target.to_owned(),
            Message::ImageMessage(ref msg) => msg.target.to_owned(),
            Message::VoiceMessage(ref msg) => msg.target.to_owned(),
            Message::ShortVideoMessage(ref msg) => msg.target.to_owned(),
            Message::VideoMessage(ref msg) => msg.target.to_owned(),
            Message::LocationMessage(ref msg) => msg.target.to_owned(),
            Message::LinkMessage(ref msg) => msg.target.to_owned(),
            Message::UnknownMessage(ref msg) => msg.target.to_owned(),
            Message::SubscribeEvent(ref msg) => msg.target.to_owned(),
            Message::UnsubscribeEvent(ref msg) => msg.target.to_owned(),
            Message::SubscribeScanEvent(ref msg) => msg.target.to_owned(),
            Message::TemplateSendJobFinishEvent(ref msg) => msg.target.to_owned(),
            Message::ScanEvent(ref msg) => msg.target.to_owned(),
            Message::LocationEvent(ref msg) => msg.target.to_owned(),
            Message::ClickEvent(ref msg) => msg.target.to_owned(),
            Message::ViewEvent(ref msg) => msg.target.to_owned(),
            Message::QualificationVerifySuccessEvent(ref msg) => msg.target.to_owned(),
        }
    }
}
