use crate::RequestMethod;

#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum WechatCpMethod {
    AccessToken,
    JsCode2Session,
    GetJsapiTicket,
    GetAgentConfigTicket,
    GetCallbackIp,
    Media(CpMediaMethod),
    Oauth2(CpOauth2Method),
    ExternalContact(CpExternalContactMethod),
    /// 自定义方法
    Custom(String)
}

impl RequestMethod for WechatCpMethod {
    fn get_method(&self) -> String {
        match self {
            WechatCpMethod::AccessToken => String::from("/cgi-bin/gettoken"),
            WechatCpMethod::GetJsapiTicket => String::from("/cgi-bin/get_jsapi_ticket"),
            WechatCpMethod::JsCode2Session => String::from("/cgi-bin/miniprogram/jscode2session"),
            WechatCpMethod::GetCallbackIp => String::from("/cgi-bin/getcallbackip"),
            WechatCpMethod::GetAgentConfigTicket => String::from("/cgi-bin/ticket/get?&type=agent_config"),
            WechatCpMethod::Media(v) => v.get_method(),
            WechatCpMethod::ExternalContact(v) => v.get_method(),
            WechatCpMethod::Oauth2(v) => v.get_method(),
            WechatCpMethod::Custom(v) => v.to_string(),
        }
    }
}

#[allow(unused)]
impl WechatCpMethod {

    pub fn need_token(&self) -> bool {
        match self {
            WechatCpMethod::AccessToken => false,
            _ => true,
        }
    }
}


#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum CpMediaMethod {
    /// 上传素材
    UploadMedia(String),
    /// 上传图片
    UploadImage,
    /// 上传附件
    UploadAttachment,
    /// 获取临时素材
    GetMedia,
    /// 获取素材JSSDK
    GetMediaJssdk,
}

#[allow(unused)]
impl CpMediaMethod {
    pub fn get_method(&self) -> String {
        match self {
            CpMediaMethod::UploadMedia(v) => format!("/cgi-bin/media/upload?type={}", v),
            CpMediaMethod::UploadImage => String::from("/cgi-bin/media/uploadimg"),
            CpMediaMethod::UploadAttachment => String::from("/cgi-bin/media/upload_attachment"),
            CpMediaMethod::GetMedia => String::from("/cgi-bin/media/get"),
            CpMediaMethod::GetMediaJssdk => String::from("/cgi-bin/media/get/jssdk"),
        }
    }
}


#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum CpOauth2Method {
    Oauth2Authorize,
    GetUserDetail,
    GetUserInfo,
}

#[allow(unused)]
impl CpOauth2Method {
    pub fn get_method(&self) -> String {
        match self {
            CpOauth2Method::Oauth2Authorize => String::from("https://open.weixin.qq.com/connect/oauth2/authorize"),
            CpOauth2Method::GetUserDetail => String::from("/cgi-bin/user/getuserdetail"),
            CpOauth2Method::GetUserInfo => String::from("/cgi-bin/user/getuserinfo"),
        }
    }
}
#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum CpExternalContactMethod {
    AddContactWay,
    GetContactWay,
    GetContactWayDetail,
    UpdateContactWay,
    DeleteContactWay,
    CloseTmpChat,
    ConvertToOpenid,
    UnionidToExternalUserid,
    BatchGetByUser,
    Remark,
    List,
    GetFollowUserList,
    GetUnassignedList,
    TransferCustomer,
    TransferResult,
    ResignedTransferCustomer,
    GetUserBehaviorData,
    ResignedTransferResult,
    GroupChatAddJoinWay,
    GroupChatGetJoinWay,
    GroupChatUpdateJoinWay,
    GroupChatDeleteJoinWay,
    GroupChatList,
    GroupChatGet,
    GroupChatTransfer,
    GroupChatStatistic,
    AddMsgTemplate,
    SendWelcomeMsg,
    GetCorpTagList,
    AddCorpTag,
    EditCorpTag,
    DeleteCorpTag,
    MarkTag,
    GetGroupMsgListV2,
    GetGroupMsgSendResult,
    GetGroupMsgResult,
    GetGroupMsgTask,
    AddGroupWelcomeTemplate,
    EditGroupWelcomeTemplate,
    GetGroupWelcomeTemplate,
    DeleteGroupWelcomeTemplate,
}

#[allow(unused)]
impl CpExternalContactMethod {
    pub fn get_method(&self) -> String {
        match self {
            CpExternalContactMethod::AddContactWay => String::from("/cgi-bin/externalcontact/add_contact_way"),
            CpExternalContactMethod::GetContactWay => String::from("/cgi-bin/externalcontact/get_contact_way"),
            CpExternalContactMethod::GetContactWayDetail => String::from("/cgi-bin/externalcontact/get"),
            CpExternalContactMethod::UpdateContactWay => String::from("/cgi-bin/externalcontact/update_contact_way"),
            CpExternalContactMethod::DeleteContactWay => String::from("/cgi-bin/externalcontact/del_contact_way"),
            CpExternalContactMethod::CloseTmpChat => String::from("/cgi-bin/externalcontact/close_temp_chat"),
            CpExternalContactMethod::UnionidToExternalUserid => String::from("/cgi-bin/externalcontact/unionid_to_external_userid"),
            CpExternalContactMethod::ConvertToOpenid => String::from("/cgi-bin/externalcontact/convert_to_openid"),
            CpExternalContactMethod::BatchGetByUser => String::from("/cgi-bin/externalcontact/batch/get_by_user"),
            CpExternalContactMethod::Remark => String::from("/cgi-bin/externalcontact/remark"),
            CpExternalContactMethod::List => String::from("/cgi-bin/externalcontact/list"),
            CpExternalContactMethod::GetFollowUserList => String::from("/cgi-bin/externalcontact/get_follow_user_list"),
            CpExternalContactMethod::GetUnassignedList => String::from("/cgi-bin/externalcontact/get_unassigned_list"),
            CpExternalContactMethod::TransferCustomer => String::from("/cgi-bin/externalcontact/transfer_customer"),
            CpExternalContactMethod::TransferResult => String::from("/cgi-bin/externalcontact/transfer_result"),
            CpExternalContactMethod::ResignedTransferCustomer => String::from("/cgi-bin/externalcontact/resigned/transfer_customer"),
            CpExternalContactMethod::GetUserBehaviorData => String::from("/cgi-bin/externalcontact/get_user_behavior_data"),
            CpExternalContactMethod::ResignedTransferResult => String::from("/cgi-bin/externalcontact/resigned/transfer_result"),
            CpExternalContactMethod::GroupChatAddJoinWay => String::from("/cgi-bin/externalcontact/groupchat/add_join_way"),
            CpExternalContactMethod::GroupChatGetJoinWay => String::from("/cgi-bin/externalcontact/groupchat/get_join_way"),
            CpExternalContactMethod::GroupChatUpdateJoinWay => String::from("/cgi-bin/externalcontact/groupchat/update_join_way"),
            CpExternalContactMethod::GroupChatDeleteJoinWay => String::from("/cgi-bin/externalcontact/groupchat/del_join_way"),
            CpExternalContactMethod::GroupChatList => String::from("/cgi-bin/externalcontact/groupchat/list"),
            CpExternalContactMethod::GroupChatGet => String::from("/cgi-bin/externalcontact/groupchat/get"),
            CpExternalContactMethod::GroupChatTransfer => String::from("/cgi-bin/externalcontact/groupchat/transfer"),
            CpExternalContactMethod::GroupChatStatistic => String::from("/cgi-bin/externalcontact/groupchat/statistic"),
            CpExternalContactMethod::AddMsgTemplate => String::from("/cgi-bin/externalcontact/add_msg_template"),
            CpExternalContactMethod::SendWelcomeMsg => String::from("/cgi-bin/externalcontact/send_welcome_msg"),
            CpExternalContactMethod::GetCorpTagList => String::from("/cgi-bin/externalcontact/get_corp_tag_list"),
            CpExternalContactMethod::AddCorpTag => String::from("/cgi-bin/externalcontact/add_corp_tag"),
            CpExternalContactMethod::EditCorpTag => String::from("/cgi-bin/externalcontact/edit_corp_tag"),
            CpExternalContactMethod::DeleteCorpTag => String::from("/cgi-bin/externalcontact/del_corp_tag"),
            CpExternalContactMethod::MarkTag => String::from("/cgi-bin/externalcontact/mark_tag"),
            CpExternalContactMethod::GetGroupMsgListV2 => String::from("/cgi-bin/externalcontact/get_groupmsg_list_v2"),
            CpExternalContactMethod::GetGroupMsgSendResult => String::from("/cgi-bin/externalcontact/get_groupmsg_send_result"),
            CpExternalContactMethod::GetGroupMsgResult => String::from("/cgi-bin/externalcontact/get_group_msg_result"),
            CpExternalContactMethod::GetGroupMsgTask => String::from("/cgi-bin/externalcontact/get_groupmsg_task"),
            CpExternalContactMethod::AddGroupWelcomeTemplate => String::from("/cgi-bin/externalcontact/group_welcome_template/add"),
            CpExternalContactMethod::EditGroupWelcomeTemplate => String::from("/cgi-bin/externalcontact/group_welcome_template/edit"),
            CpExternalContactMethod::GetGroupWelcomeTemplate => String::from("/cgi-bin/externalcontact/group_welcome_template/get"),
            CpExternalContactMethod::DeleteGroupWelcomeTemplate => String::from("/cgi-bin/externalcontact/group_welcome_template/del"),
        }
    }
}