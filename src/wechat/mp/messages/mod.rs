mod text;
mod image;
mod voice;
mod shortvideo;
mod video;
mod location;
mod link;
mod unknown;

use crate::{LabradorResult, parse_message};
use crate::events::{ClickEvent, LocationEvent, QualificationVerifySuccessEvent, ScanEvent, SubscribeEvent, SubscribeScanEvent, TemplateSendJobFinishEvent, UnsubscribeEvent, ViewEvent};
// export MpMessage types
pub use self::text::TextMessage;
pub use self::image::ImageMessage;
pub use self::voice::VoiceMessage;
pub use self::shortvideo::ShortVideoMessage;
pub use self::video::VideoMessage;
pub use self::location::LocationMessage;
pub use self::link::LinkMessage;
pub use self::unknown::UnknownMessage;


// an enum or messages and events
#[allow(unused)]
#[derive(Debug, Clone)]
pub enum MpMessage {
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
impl MpMessage {
    pub fn parse<S: AsRef<str>>(xml: S) -> LabradorResult<MpMessage> {
        parse_message(xml.as_ref())
    }

    pub fn get_source(&self) -> String {
        match *self {
            MpMessage::TextMessage(ref msg) => msg.source.to_string(),
            MpMessage::ImageMessage(ref msg) => msg.source.to_string(),
            MpMessage::VoiceMessage(ref msg) => msg.source.to_string(),
            MpMessage::ShortVideoMessage(ref msg) => msg.source.to_string(),
            MpMessage::VideoMessage(ref msg) => msg.source.to_string(),
            MpMessage::LocationMessage(ref msg) => msg.source.to_string(),
            MpMessage::LinkMessage(ref msg) => msg.source.to_string(),
            MpMessage::UnknownMessage(ref msg) => msg.source.to_string(),
            MpMessage::SubscribeEvent(ref msg) => msg.source.to_string(),
            MpMessage::UnsubscribeEvent(ref msg) => msg.source.to_string(),
            MpMessage::SubscribeScanEvent(ref msg) => msg.source.to_string(),
            MpMessage::ScanEvent(ref msg) => msg.source.to_string(),
            MpMessage::LocationEvent(ref msg) => msg.source.to_string(),
            MpMessage::ClickEvent(ref msg) => msg.source.to_string(),
            MpMessage::ViewEvent(ref msg) => msg.source.to_string(),
            MpMessage::TemplateSendJobFinishEvent(ref msg) => msg.source.to_string(),
            MpMessage::QualificationVerifySuccessEvent(ref msg) => msg.source.to_string(),
        }
    }

    pub fn get_target(&self) -> String {
        match *self {
            MpMessage::TextMessage(ref msg) => msg.target.to_string(),
            MpMessage::ImageMessage(ref msg) => msg.target.to_string(),
            MpMessage::VoiceMessage(ref msg) => msg.target.to_string(),
            MpMessage::ShortVideoMessage(ref msg) => msg.target.to_string(),
            MpMessage::VideoMessage(ref msg) => msg.target.to_string(),
            MpMessage::LocationMessage(ref msg) => msg.target.to_string(),
            MpMessage::LinkMessage(ref msg) => msg.target.to_string(),
            MpMessage::UnknownMessage(ref msg) => msg.target.to_string(),
            MpMessage::SubscribeEvent(ref msg) => msg.target.to_string(),
            MpMessage::UnsubscribeEvent(ref msg) => msg.target.to_string(),
            MpMessage::SubscribeScanEvent(ref msg) => msg.target.to_string(),
            MpMessage::TemplateSendJobFinishEvent(ref msg) => msg.target.to_string(),
            MpMessage::ScanEvent(ref msg) => msg.target.to_string(),
            MpMessage::LocationEvent(ref msg) => msg.target.to_string(),
            MpMessage::ClickEvent(ref msg) => msg.target.to_string(),
            MpMessage::ViewEvent(ref msg) => msg.target.to_string(),
            MpMessage::QualificationVerifySuccessEvent(ref msg) => msg.target.to_string(),
        }
    }
}
