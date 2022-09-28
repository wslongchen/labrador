//! 消息格式
//! 开启接收消息模式后，企业成员在企业微信应用里发送消息时，企业微信会将消息同步到企业应用的后台。
//! 如何接收消息已经在使用接收消息说明，本小节是对普通消息结构体的说明。
//! 消息类型支持：文本、图片、语音、视频、位置以及链接信息。
//! 注：以下出现的xml包仅是接收的消息包中的<a href="https://developer.work.weixin.qq.com/document/path/90240#12977/%E4%BD%BF%E7%94%A8%E6%8E%A5%E6%94%B6%E6%B6%88%E6%81%AF">Encrypt参数</a>解密后的内容说明
mod text;
mod image;
mod voice;
mod video;
mod location;
mod link;
mod unknown;

use crate::{CpAppAdminChangeEvent, CpAuthCancelEvent, CpAuthChangeEvent, CpAuthCreateEvent, CpBatchJobResultEvent, CpChangeExternalContactEvent, CpContactCreatePartyEvent, CpContactCreateUserEvent, CpContactDeletePartyEvent, CpContactDeleteUserEvent, CpContactUpdatePartyEvent, CpContactUpdateTagEvent, CpContactUpdateUserEvent, CpEnterAgentEvent, CpLicensePaySuccessEvent, CpLicenseRefundEvent, CpLocationEvent, CpMenuClickEvent, CpMenuLocationSelectEvent, CpMenuPicPhotoOrAlbumEvent, CpMenuPicSysPhotoEvent, CpMenuPicWeixinEvent, CpMenuScanCodePushEvent, CpMenuScanCodeWaitMsgEvent, CpMenuViewEvent, CpOpenApprovalChangeEvent, CpPermanentCodeEvent, CpShareAgentChangeEvent, CpShareChainChangeEvent, CpSubscribeEvent, CpTemplateCardEvent, CpTemplateCardMenuEvent, CpTicketEvent, CpTpContactCreatePartyEvent, CpTpContactCreateUserEvent, CpTpContactDeletePartyEvent, CpTpContactDeleteUserEvent, CpTpContactUpdatePartyEvent, CpTpContactUpdateTagEvent, CpTpContactUpdateUserEvent, CpUnlicensedNotifyEvent, CpUnsubscribeEvent, LabradorResult, parse_cp_message};
// export Message types
pub use self::text::CpTextMessage;
pub use self::image::CpImageMessage;
pub use self::voice::CpVoiceMessage;
pub use self::video::CpVideoMessage;
pub use self::location::CpLocationMessage;
pub use self::link::CpLinkMessage;
pub use self::unknown::CpUnknownMessage;

// an enum or messages and events
#[allow(unused)]
#[derive(Debug, Clone)]
pub enum CpMessage {
    TextMessage(CpTextMessage),
    ImageMessage(CpImageMessage),
    VoiceMessage(CpVoiceMessage),
    VideoMessage(CpVideoMessage),
    LocationMessage(CpLocationMessage),
    LinkMessage(CpLinkMessage),
    TicketEvent(CpTicketEvent),
    AuthChangeEvent(CpAuthChangeEvent),
    AuthCreateEvent(CpAuthCreateEvent),
    AuthCancelEvent(CpAuthCancelEvent),
    PermanentCodeEvent(CpPermanentCodeEvent),
    AppAdminChangeEvent(CpAppAdminChangeEvent),
    UnknownMessage(CpUnknownMessage),
    LocationEvent(CpLocationEvent),
    OpenApprovalChangeEvent(CpOpenApprovalChangeEvent),
    BatchJobResultEvent(CpBatchJobResultEvent),
    ContactCreateUserEvent(CpContactCreateUserEvent),
    ContactUpdateUserEvent(CpContactUpdateUserEvent),
    ContactDeleteUserEvent(CpContactDeleteUserEvent),
    ContactCreatePartyEvent(CpContactCreatePartyEvent),
    ContactUpdatePartyEvent(CpContactUpdatePartyEvent),
    ContactDeletePartyEvent(CpContactDeletePartyEvent),
    ContactUpdateTagEvent(CpContactUpdateTagEvent),
    TpContactCreateUserEvent(CpTpContactCreateUserEvent),
    TpContactUpdateUserEvent(CpTpContactUpdateUserEvent),
    TpContactDeleteUserEvent(CpTpContactDeleteUserEvent),
    TpContactCreatePartyEvent(CpTpContactCreatePartyEvent),
    TpContactUpdatePartyEvent(CpTpContactUpdatePartyEvent),
    TpContactDeletePartyEvent(CpTpContactDeletePartyEvent),
    TpContactUpdateTagEvent(CpTpContactUpdateTagEvent),
    EnterAgentEvent(CpEnterAgentEvent),
    LicensePaySuccessEvent(CpLicensePaySuccessEvent),
    LicenseRefundEvent(CpLicenseRefundEvent),
    UnlicensedNotifyEvent(CpUnlicensedNotifyEvent),
    MenuClickEvent(CpMenuClickEvent),
    MenuViewEvent(CpMenuViewEvent),
    MenuPicWeixinEvent(CpMenuPicWeixinEvent),
    MenuLocationSelectEvent(CpMenuLocationSelectEvent),
    MenuPicSysPhotoEvent(CpMenuPicSysPhotoEvent),
    MenuScanCodePushEvent(CpMenuScanCodePushEvent),
    MenuPicPhotoOrAlbumEvent(CpMenuPicPhotoOrAlbumEvent),
    MenuScanCodeWaitMsgEvent(CpMenuScanCodeWaitMsgEvent),
    ShareAgentChangeEvent(CpShareAgentChangeEvent),
    ShareChainChangeEvent(CpShareChainChangeEvent),
    SubscribeEvent(CpSubscribeEvent),
    UnsubscribeEvent(CpUnsubscribeEvent),
    TemplateCardEvent(CpTemplateCardEvent),
    TemplateCardMenuEvent(CpTemplateCardMenuEvent),
    AddExternalContactEvent(CpChangeExternalContactEvent),
    EditExternalContact(CpChangeExternalContactEvent),
    AddHalfExternalContact(CpChangeExternalContactEvent),
    DelExternalContact(CpChangeExternalContactEvent),
    DelFollowUser(CpChangeExternalContactEvent),
}

#[allow(unused)]
impl CpMessage {
    pub fn parse<S: AsRef<str>>(xml: S) -> LabradorResult<CpMessage> {
        parse_cp_message(xml.as_ref())
    }

    pub fn get_source(&self) -> String {
        match *self {
            CpMessage::TextMessage(ref msg) => msg.source.to_string(),
            CpMessage::ImageMessage(ref msg) => msg.source.to_string(),
            CpMessage::VoiceMessage(ref msg) => msg.source.to_string(),
            CpMessage::VideoMessage(ref msg) => msg.source.to_string(),
            CpMessage::LocationMessage(ref msg) => msg.source.to_string(),
            CpMessage::LinkMessage(ref msg) => msg.source.to_string(),
            CpMessage::UnknownMessage(ref msg) => msg.source.to_string(),
            CpMessage::LocationEvent(ref msg) => msg.source.to_string(),
            CpMessage::OpenApprovalChangeEvent(ref msg) => msg.source.to_string(),
            CpMessage::BatchJobResultEvent(ref msg) => msg.source.to_string(),
            CpMessage::ContactCreateUserEvent(ref msg) => msg.source.to_string(),
            CpMessage::ContactUpdateUserEvent(ref msg) => msg.source.to_string(),
            CpMessage::ContactDeleteUserEvent(ref msg) => msg.source.to_string(),
            CpMessage::ContactCreatePartyEvent(ref msg) => msg.source.to_string(),
            CpMessage::ContactUpdatePartyEvent(ref msg) => msg.source.to_string(),
            CpMessage::ContactDeletePartyEvent(ref msg) => msg.source.to_string(),
            CpMessage::ContactUpdateTagEvent(ref msg) => msg.source.to_string(),
            CpMessage::EnterAgentEvent(ref msg) => msg.source.to_string(),
            CpMessage::MenuClickEvent(ref msg) => msg.source.to_string(),
            CpMessage::MenuViewEvent(ref msg) => msg.source.to_string(),
            CpMessage::ShareAgentChangeEvent(ref msg) => msg.source.to_string(),
            CpMessage::ShareChainChangeEvent(ref msg) => msg.source.to_string(),
            CpMessage::SubscribeEvent(ref msg) => msg.source.to_string(),
            CpMessage::UnsubscribeEvent(ref msg) => msg.source.to_string(),
            CpMessage::TemplateCardEvent(ref msg) => msg.source.to_string(),
            CpMessage::TemplateCardMenuEvent(ref msg) => msg.source.to_string(),
            CpMessage::AddExternalContactEvent(ref msg) => msg.source.to_string(),
            CpMessage::EditExternalContact(ref msg) => msg.source.to_string(),
            CpMessage::AddHalfExternalContact(ref msg) => msg.source.to_string(),
            CpMessage::DelExternalContact(ref msg) => msg.source.to_string(),
            CpMessage::DelFollowUser(ref msg) => msg.source.to_string(),
            CpMessage::MenuPicWeixinEvent(ref msg) => msg.source.to_string(),
            CpMessage::MenuLocationSelectEvent(ref msg) => msg.source.to_string(),
            CpMessage::MenuPicSysPhotoEvent(ref msg) => msg.source.to_string(),
            CpMessage::MenuScanCodePushEvent(ref msg) => msg.source.to_string(),
            CpMessage::MenuPicPhotoOrAlbumEvent(ref msg) => msg.source.to_string(),
            CpMessage::MenuScanCodeWaitMsgEvent(ref msg) => msg.source.to_string(),
            _ => "".to_string()
        }
    }

    pub fn get_target(&self) -> String {
        match *self {
            CpMessage::TextMessage(ref msg) => msg.target.to_string(),
            CpMessage::ImageMessage(ref msg) => msg.target.to_string(),
            CpMessage::VoiceMessage(ref msg) => msg.target.to_string(),
            CpMessage::VideoMessage(ref msg) => msg.target.to_string(),
            CpMessage::LocationMessage(ref msg) => msg.target.to_string(),
            CpMessage::LinkMessage(ref msg) => msg.target.to_string(),
            CpMessage::UnknownMessage(ref msg) => msg.target.to_string(),
            CpMessage::LocationEvent(ref msg) => msg.target.to_string(),
            CpMessage::OpenApprovalChangeEvent(ref msg) => msg.target.to_string(),
            CpMessage::BatchJobResultEvent(ref msg) => msg.target.to_string(),
            CpMessage::ContactCreateUserEvent(ref msg) => msg.target.to_string(),
            CpMessage::ContactUpdateUserEvent(ref msg) => msg.target.to_string(),
            CpMessage::ContactDeleteUserEvent(ref msg) => msg.target.to_string(),
            CpMessage::ContactCreatePartyEvent(ref msg) => msg.target.to_string(),
            CpMessage::ContactUpdatePartyEvent(ref msg) => msg.target.to_string(),
            CpMessage::ContactDeletePartyEvent(ref msg) => msg.target.to_string(),
            CpMessage::ContactUpdateTagEvent(ref msg) => msg.target.to_string(),
            CpMessage::EnterAgentEvent(ref msg) => msg.target.to_string(),
            CpMessage::MenuClickEvent(ref msg) => msg.target.to_string(),
            CpMessage::MenuViewEvent(ref msg) => msg.target.to_string(),
            CpMessage::ShareAgentChangeEvent(ref msg) => msg.target.to_string(),
            CpMessage::ShareChainChangeEvent(ref msg) => msg.target.to_string(),
            CpMessage::SubscribeEvent(ref msg) => msg.target.to_string(),
            CpMessage::UnsubscribeEvent(ref msg) => msg.target.to_string(),
            CpMessage::TemplateCardEvent(ref msg) => msg.target.to_string(),
            CpMessage::TemplateCardMenuEvent(ref msg) => msg.target.to_string(),
            CpMessage::AddExternalContactEvent(ref msg) => msg.target.to_string(),
            CpMessage::EditExternalContact(ref msg) => msg.target.to_string(),
            CpMessage::AddHalfExternalContact(ref msg) => msg.target.to_string(),
            CpMessage::DelExternalContact(ref msg) => msg.target.to_string(),
            CpMessage::DelFollowUser(ref msg) => msg.target.to_string(),
            CpMessage::MenuPicWeixinEvent(ref msg) => msg.target.to_string(),
            CpMessage::MenuLocationSelectEvent(ref msg) => msg.target.to_string(),
            CpMessage::MenuPicSysPhotoEvent(ref msg) => msg.target.to_string(),
            CpMessage::MenuScanCodePushEvent(ref msg) => msg.target.to_string(),
            CpMessage::MenuPicPhotoOrAlbumEvent(ref msg) => msg.target.to_string(),
            CpMessage::MenuScanCodeWaitMsgEvent(ref msg) => msg.target.to_string(),
            _ => "".to_string()
        }
    }

    pub fn get_agent_id(&self) -> i64 {
        match *self {
            CpMessage::TextMessage(ref msg) => msg.agent_id,
            CpMessage::ImageMessage(ref msg) => msg.agent_id,
            CpMessage::VoiceMessage(ref msg) => msg.agent_id,
            CpMessage::VideoMessage(ref msg) => msg.agent_id,
            CpMessage::LocationMessage(ref msg) => msg.agent_id,
            CpMessage::LinkMessage(ref msg) => msg.agent_id,
            CpMessage::UnknownMessage(ref msg) => msg.agent_id.unwrap_or_default(),
            CpMessage::LocationEvent(ref msg) => msg.agent_id,
            CpMessage::OpenApprovalChangeEvent(ref msg) => msg.agent_id,
            CpMessage::BatchJobResultEvent(ref msg) => msg.agent_id,
            CpMessage::EnterAgentEvent(ref msg) => msg.agent_id,
            CpMessage::MenuClickEvent(ref msg) => msg.agent_id,
            CpMessage::MenuViewEvent(ref msg) => msg.agent_id,
            CpMessage::ShareAgentChangeEvent(ref msg) => msg.agent_id,
            CpMessage::ShareChainChangeEvent(ref msg) => msg.agent_id,
            CpMessage::SubscribeEvent(ref msg) => msg.agent_id,
            CpMessage::UnsubscribeEvent(ref msg) => msg.agent_id,
            CpMessage::TemplateCardEvent(ref msg) => msg.agent_id,
            CpMessage::TemplateCardMenuEvent(ref msg) => msg.agent_id,
            // CpMessage::AddExternalContactEvent(ref msg) => msg.agent_id,
            CpMessage::MenuPicWeixinEvent(ref msg) => msg.agent_id,
            CpMessage::MenuLocationSelectEvent(ref msg) => msg.agent_id,
            CpMessage::MenuPicSysPhotoEvent(ref msg) => msg.agent_id,
            CpMessage::MenuScanCodePushEvent(ref msg) => msg.agent_id,
            CpMessage::MenuPicPhotoOrAlbumEvent(ref msg) => msg.agent_id,
            CpMessage::MenuScanCodeWaitMsgEvent(ref msg) => msg.agent_id,
            CpMessage::AppAdminChangeEvent(ref msg) => msg.agent_id,
            CpMessage::UnlicensedNotifyEvent(ref msg) => msg.agent_id,
            _ => 0
        }
    }

    pub fn get_content(&self) -> String {
        match *self {
            CpMessage::TextMessage(ref msg) => msg.content.to_string(),
            _ => "".to_string()
        }
    }

    pub fn get_event_key(&self) -> String {
        match *self {
            CpMessage::LocationMessage(ref msg) => msg.agent_id.to_string(),
            CpMessage::LinkMessage(ref msg) => msg.agent_id.to_string(),
            CpMessage::UnknownMessage(ref msg) => msg.agent_id.unwrap_or_default().to_string(),
            CpMessage::MenuClickEvent(ref msg) => msg.event_key.to_string(),
            CpMessage::MenuViewEvent(ref msg) => msg.event_key.to_string(),
            CpMessage::MenuPicWeixinEvent(ref msg) => msg.event_key.to_string(),
            CpMessage::MenuLocationSelectEvent(ref msg) => msg.event_key.to_string(),
            CpMessage::MenuPicSysPhotoEvent(ref msg) => msg.event_key.to_string(),
            CpMessage::MenuScanCodePushEvent(ref msg) => msg.event_key.to_string(),
            CpMessage::MenuPicPhotoOrAlbumEvent(ref msg) => msg.event_key.to_string(),
            CpMessage::MenuScanCodeWaitMsgEvent(ref msg) => msg.event_key.to_string(),
            _ => "".to_string()
        }
    }

    pub fn get_event(&self) -> String {
        match *self {
            CpMessage::LocationEvent(ref msg) => msg.event.to_string(),
            CpMessage::OpenApprovalChangeEvent(ref msg) => msg.event.to_string(),
            CpMessage::BatchJobResultEvent(ref msg) => msg.event.to_string(),
            CpMessage::ContactCreateUserEvent(ref msg) => msg.event.to_string(),
            CpMessage::ContactUpdateUserEvent(ref msg) => msg.event.to_string(),
            CpMessage::ContactDeleteUserEvent(ref msg) => msg.event.to_string(),
            CpMessage::ContactCreatePartyEvent(ref msg) => msg.event.to_string(),
            CpMessage::ContactUpdatePartyEvent(ref msg) => msg.event.to_string(),
            CpMessage::ContactDeletePartyEvent(ref msg) => msg.event.to_string(),
            CpMessage::ContactUpdateTagEvent(ref msg) => msg.event.to_string(),
            CpMessage::EnterAgentEvent(ref msg) => msg.event.to_string(),
            CpMessage::MenuClickEvent(ref msg) => msg.event.to_string(),
            CpMessage::MenuViewEvent(ref msg) => msg.event.to_string(),
            CpMessage::ShareAgentChangeEvent(ref msg) => msg.event.to_string(),
            CpMessage::ShareChainChangeEvent(ref msg) => msg.event.to_string(),
            CpMessage::SubscribeEvent(ref msg) => msg.event.to_string(),
            CpMessage::UnsubscribeEvent(ref msg) => msg.event.to_string(),
            CpMessage::TemplateCardEvent(ref msg) => msg.event.to_string(),
            CpMessage::TemplateCardMenuEvent(ref msg) => msg.event.to_string(),
            CpMessage::AddExternalContactEvent(ref msg) => msg.event.to_string(),
            CpMessage::EditExternalContact(ref msg) => msg.event.to_string(),
            CpMessage::AddHalfExternalContact(ref msg) => msg.event.to_string(),
            CpMessage::DelExternalContact(ref msg) => msg.event.to_string(),
            CpMessage::DelFollowUser(ref msg) => msg.event.to_string(),
            CpMessage::MenuPicWeixinEvent(ref msg) => msg.event.to_string(),
            CpMessage::MenuLocationSelectEvent(ref msg) => msg.event.to_string(),
            CpMessage::MenuPicSysPhotoEvent(ref msg) => msg.event.to_string(),
            CpMessage::MenuScanCodePushEvent(ref msg) => msg.event.to_string(),
            CpMessage::MenuPicPhotoOrAlbumEvent(ref msg) => msg.event.to_string(),
            CpMessage::MenuScanCodeWaitMsgEvent(ref msg) => msg.event.to_string(),
            CpMessage::TicketEvent(ref msg) => msg.info_type.to_string(),
            CpMessage::AuthChangeEvent(ref msg) => msg.info_type.to_string(),
            CpMessage::AuthCreateEvent(ref msg) => msg.info_type.to_string(),
            CpMessage::AuthCancelEvent(ref msg) => msg.info_type.to_string(),
            CpMessage::PermanentCodeEvent(ref msg) => msg.info_type.to_string(),
            CpMessage::AppAdminChangeEvent(ref msg) => msg.event.to_string(),
            CpMessage::TpContactCreateUserEvent(ref msg) => msg.info_type.to_string(),
            CpMessage::TpContactUpdateUserEvent(ref msg) => msg.info_type.to_string(),
            CpMessage::TpContactDeleteUserEvent(ref msg) => msg.info_type.to_string(),
            CpMessage::TpContactCreatePartyEvent(ref msg) => msg.info_type.to_string(),
            CpMessage::TpContactUpdatePartyEvent(ref msg) => msg.info_type.to_string(),
            CpMessage::TpContactDeletePartyEvent(ref msg) => msg.info_type.to_string(),
            CpMessage::TpContactUpdateTagEvent(ref msg) => msg.info_type.to_string(),
            CpMessage::LicensePaySuccessEvent(ref msg) => msg.into_type.to_string(),
            CpMessage::LicenseRefundEvent(ref msg) => msg.into_type.to_string(),
            CpMessage::UnlicensedNotifyEvent(ref msg) => msg.event.to_string(),
            _ => "".to_string()
        }
    }
}
