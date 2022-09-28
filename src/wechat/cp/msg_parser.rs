use crate::{CpAuthCancelEvent, CpAuthChangeEvent, CpAuthCreateEvent, CpBatchJobResultEvent, CpChangeExternalContactEvent, CpContactCreatePartyEvent, CpContactCreateUserEvent, CpContactDeletePartyEvent, CpContactDeleteUserEvent, CpContactUpdatePartyEvent, CpContactUpdateTagEvent, CpContactUpdateUserEvent, CpEnterAgentEvent, CpImageMessage, CpLinkMessage, CpLocationEvent, CpLocationMessage, CpMenuClickEvent, CpMenuLocationSelectEvent, CpMenuPicPhotoOrAlbumEvent, CpMenuPicSysPhotoEvent, CpMenuPicWeixinEvent, CpMenuScanCodePushEvent, CpMenuScanCodeWaitMsgEvent, CpMenuViewEvent, CpMessage, CpOpenApprovalChangeEvent, CpShareAgentChangeEvent, CpShareChainChangeEvent, CpSubscribeEvent, CpTemplateCardEvent, CpTemplateCardMenuEvent, CpTextMessage, CpTicketEvent, CpTpContactCreatePartyEvent, CpTpContactCreateUserEvent, CpTpContactDeletePartyEvent, CpTpContactDeleteUserEvent, CpTpContactUpdatePartyEvent, CpTpContactUpdateTagEvent, CpTpContactUpdateUserEvent, CpUnknownMessage, CpVideoMessage, CpVoiceMessage, LabradorResult, XmlMessageParser};

pub fn parse_cp_message<S: AsRef<str>>(xml: S) -> LabradorResult<CpMessage> {
    let doc = serde_xml_rs::from_str::<serde_json::Value>(xml.as_ref())?;
    let msg_type = doc["MsgType"]["$value"].as_str().unwrap_or_default();
    let info_type = doc["InfoType"]["$value"].as_str().unwrap_or_default();
    if !info_type.is_empty() {
        let msg = match info_type {
            "suite_ticket" => CpMessage::TicketEvent(CpTicketEvent::from_xml(xml.as_ref())?),
            "create_auth" => CpMessage::AuthCreateEvent(CpAuthCreateEvent::from_xml(xml.as_ref())?),
            "change_auth" => CpMessage::AuthChangeEvent(CpAuthChangeEvent::from_xml(xml.as_ref())?),
            "cancel_auth" => CpMessage::AuthCancelEvent(CpAuthCancelEvent::from_xml(xml.as_ref())?),
            "change_contact" => {
                let change_type = doc["ChangeType"]["$value"].as_str().unwrap_or_default();
                match change_type {
                    "create_user" => CpMessage::TpContactCreateUserEvent(CpTpContactCreateUserEvent::from_xml(xml.as_ref())?),
                    "update_user" => CpMessage::TpContactUpdateUserEvent(CpTpContactUpdateUserEvent::from_xml(xml.as_ref())?),
                    "delete_user" => CpMessage::TpContactDeleteUserEvent(CpTpContactDeleteUserEvent::from_xml(xml.as_ref())?),
                    "create_party" => CpMessage::TpContactCreatePartyEvent(CpTpContactCreatePartyEvent::from_xml(xml.as_ref())?),
                    "update_party" => CpMessage::TpContactUpdatePartyEvent(CpTpContactUpdatePartyEvent::from_xml(xml.as_ref())?),
                    "delete_party" => CpMessage::TpContactDeletePartyEvent(CpTpContactDeletePartyEvent::from_xml(xml.as_ref())?),
                    "update_tag" => CpMessage::TpContactUpdateTagEvent(CpTpContactUpdateTagEvent::from_xml(xml.as_ref())?),
                    _ => {
                        let mut msg = CpUnknownMessage::from_xml(xml.as_ref())?;
                        msg.raw = xml.as_ref().to_string().into();
                        CpMessage::UnknownMessage(msg)
                    }
                }
            }
            _ => {
                let mut msg = CpUnknownMessage::from_xml(xml.as_ref())?;
                msg.raw = xml.as_ref().to_string().into();
                CpMessage::UnknownMessage(msg)
            }
        };
        return Ok(msg);
    }
    let msg = match msg_type {
        "text" => CpMessage::TextMessage(CpTextMessage::from_xml(xml.as_ref())?),
        "image" => CpMessage::ImageMessage(CpImageMessage::from_xml(xml.as_ref())?),
        "voice" => CpMessage::VoiceMessage(CpVoiceMessage::from_xml(xml.as_ref())?),
        "video" => CpMessage::VideoMessage(CpVideoMessage::from_xml(xml.as_ref())?),
        "location" => CpMessage::LocationMessage(CpLocationMessage::from_xml(xml.as_ref())?),
        "link" => CpMessage::LinkMessage(CpLinkMessage::from_xml(xml.as_ref())?),
        "event" => {
            let event_str = doc["Event"]["$value"].as_str().unwrap_or_default();
            let change_type = doc["ChangeType"]["$value"].as_str().unwrap_or_default();
            parse_event(&event_str.to_lowercase(), change_type, xml.as_ref())?
        }
        _ => CpMessage::UnknownMessage(CpUnknownMessage::from_xml(xml.as_ref())?),
    };
    Ok(msg)
}

fn parse_event(event: &str, change_type: &str, xml: &str) -> LabradorResult<CpMessage> {
    let msg = match event {
        "location" => CpMessage::LocationEvent(CpLocationEvent::from_xml(xml)?),
        "subscribe" => CpMessage::SubscribeEvent(CpSubscribeEvent::from_xml(xml)?),
        "enter_agent" => CpMessage::EnterAgentEvent(CpEnterAgentEvent::from_xml(xml)?),
        "batch_job_result" => CpMessage::BatchJobResultEvent(CpBatchJobResultEvent::from_xml(xml)?),
        "change_contact" => {
            match change_type {
                "create_user" => CpMessage::ContactCreateUserEvent(CpContactCreateUserEvent::from_xml(xml)?),
                "update_user" => CpMessage::ContactUpdateUserEvent(CpContactUpdateUserEvent::from_xml(xml)?),
                "delete_user" => CpMessage::ContactDeleteUserEvent(CpContactDeleteUserEvent::from_xml(xml)?),
                "create_party" => CpMessage::ContactCreatePartyEvent(CpContactCreatePartyEvent::from_xml(xml)?),
                "update_party" => CpMessage::ContactUpdatePartyEvent(CpContactUpdatePartyEvent::from_xml(xml)?),
                "delete_party" => CpMessage::ContactDeletePartyEvent(CpContactDeletePartyEvent::from_xml(xml)?),
                "update_tag" => CpMessage::ContactUpdateTagEvent(CpContactUpdateTagEvent::from_xml(xml)?),
                _ => CpMessage::UnknownMessage(CpUnknownMessage::from_xml(xml)?),
            }
        }
        "click" => CpMessage::MenuClickEvent(CpMenuClickEvent::from_xml(xml)?),
        "view" => CpMessage::MenuViewEvent(CpMenuViewEvent::from_xml(xml)?),
        "scancode_push" => CpMessage::MenuScanCodePushEvent(CpMenuScanCodePushEvent::from_xml(xml)?),
        "scancode_waitmsg" => CpMessage::MenuScanCodeWaitMsgEvent(CpMenuScanCodeWaitMsgEvent::from_xml(xml)?),
        "pic_sysphoto" => CpMessage::MenuPicSysPhotoEvent(CpMenuPicSysPhotoEvent::from_xml(xml)?),
        "pic_photo_or_album" => CpMessage::MenuPicPhotoOrAlbumEvent(CpMenuPicPhotoOrAlbumEvent::from_xml(xml)?),
        "pic_weixin" => CpMessage::MenuPicWeixinEvent(CpMenuPicWeixinEvent::from_xml(xml)?),
        "location_select" => CpMessage::MenuLocationSelectEvent(CpMenuLocationSelectEvent::from_xml(xml)?),
        "open_approval_change" => CpMessage::OpenApprovalChangeEvent(CpOpenApprovalChangeEvent::from_xml(xml)?),
        "share_agent_change" => CpMessage::ShareAgentChangeEvent(CpShareAgentChangeEvent::from_xml(xml)?),
        "share_chain_change" => CpMessage::ShareChainChangeEvent(CpShareChainChangeEvent::from_xml(xml)?),
        "template_card_event" => CpMessage::TemplateCardEvent(CpTemplateCardEvent::from_xml(xml)?),
        "template_card_menu_event" => CpMessage::TemplateCardMenuEvent(CpTemplateCardMenuEvent::from_xml(xml)?),
        "change_external_contact" => {
            match change_type {
                "add_external_contact" => CpMessage::AddExternalContactEvent(CpChangeExternalContactEvent::from_xml(xml.as_ref())?),
                "edit_external_contact" => CpMessage::EditExternalContact(CpChangeExternalContactEvent::from_xml(xml.as_ref())?),
                "add_half_external_contact" => CpMessage::AddHalfExternalContact(CpChangeExternalContactEvent::from_xml(xml.as_ref())?),
                "del_external_contact" => CpMessage::DelExternalContact(CpChangeExternalContactEvent::from_xml(xml.as_ref())?),
                "del_follow_user" => CpMessage::DelFollowUser(CpChangeExternalContactEvent::from_xml(xml.as_ref())?),
                _ => CpMessage::UnknownMessage(CpUnknownMessage::from_xml(xml)?),
            }
        }
        _ => CpMessage::UnknownMessage(CpUnknownMessage::from_xml(xml)?),
    };
    Ok(msg)
}
