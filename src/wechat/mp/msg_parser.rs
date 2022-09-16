use crate::messages::{ImageMessage, LinkMessage, LocationMessage, MpMessage, ShortVideoMessage, TextMessage, UnknownMessage, VideoMessage, VoiceMessage};
use crate::{LabradorResult, XmlMessageParser, xmlutil};
use crate::events::{ClickEvent, LocationEvent, QualificationVerifySuccessEvent, ScanEvent, SubscribeEvent, SubscribeScanEvent, UnsubscribeEvent, ViewEvent};

pub fn parse_message<S: AsRef<str>>(xml: S) -> LabradorResult<MpMessage> {
    let xml = xml.as_ref();
    let package = xmlutil::parse(xml);
    let doc = package.as_document();
    let msg_type_str = xmlutil::evaluate(&doc, "//xml/MsgType/text()").string().to_lowercase();
    let msg_type = &msg_type_str[..];
    let msg = match msg_type {
        "text" => MpMessage::TextMessage(TextMessage::from_xml(xml)?),
        "image" => MpMessage::ImageMessage(ImageMessage::from_xml(xml)?),
        "voice" => MpMessage::VoiceMessage(VoiceMessage::from_xml(xml)?),
        "shortvideo" => MpMessage::ShortVideoMessage(ShortVideoMessage::from_xml(xml)?),
        "video" => MpMessage::VideoMessage(VideoMessage::from_xml(xml)?),
        "location" => MpMessage::LocationMessage(LocationMessage::from_xml(xml)?),
        "link" => MpMessage::LinkMessage(LinkMessage::from_xml(xml)?),
        "event" => {
            let event_str = xmlutil::evaluate(&doc, "//xml/Event/text()").string().to_lowercase();
            if &event_str == "subscribe" {
                let event_key = xmlutil::evaluate(&doc, "//xml/EventKey/text()").string();
                if &event_key != "" {
                    // special SubscribeScanEvent
                    return Ok(MpMessage::SubscribeScanEvent(SubscribeScanEvent::from_xml(xml)?));
                }
            }
            parse_event(&event_str[..], xml)?
        },
        _ => MpMessage::UnknownMessage(UnknownMessage::from_xml(xml)?),
    };
    Ok(msg)
}

fn parse_event(event: &str, xml: &str) -> LabradorResult<MpMessage> {
    let msg =match event {
        "subscribe" => MpMessage::SubscribeEvent(SubscribeEvent::from_xml(xml)?),
        "unsubscribe" => MpMessage::UnsubscribeEvent(UnsubscribeEvent::from_xml(xml)?),
        "templatesendjobfinish" => MpMessage::UnsubscribeEvent(UnsubscribeEvent::from_xml(xml)?),
        "scan" => MpMessage::ScanEvent(ScanEvent::from_xml(xml)?),
        "location" => MpMessage::LocationEvent(LocationEvent::from_xml(xml)?),
        "click" => MpMessage::ClickEvent(ClickEvent::from_xml(xml)?),
        "view" => MpMessage::ViewEvent(ViewEvent::from_xml(xml)?),
        "qualification_verify_success" => MpMessage::QualificationVerifySuccessEvent(QualificationVerifySuccessEvent::from_xml(xml)?),
        _ => MpMessage::UnknownMessage(UnknownMessage::from_xml(xml)?),
    };
    Ok(msg)
}
