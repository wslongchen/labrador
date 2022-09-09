use crate::RequestMethod;

#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum WechatCpMethod {
    AccessToken,
    GetProviderToken,
    GetSuiteToken,
    GetCorpToken,
    JsCode2Session,
    GetPermanentCode,
    GetPreAuthCode,
    GetJsapiTicket,
    GetAgentConfigTicket,
    GetSuiteJsapiTicket,
    GetCallbackIp,
    GetAuthInfo,
    GetOrder,
    GetOrderList,
    Media(CpMediaMethod),
    Tag(CpTagMethod),
    Agent(CpAgentMethod),
    License(CpLicenseMethod),
    Oauth2(CpOauth2Method),
    Menu(CpMenuMethod),
    User(CpUserMethod),
    Department(CpDepartmentMethod),
    Message(CpMessageMethod),
    ExternalContact(CpExternalContactMethod),
    /// 自定义方法
    Custom{ need_token: bool, method_url: String }
}

impl RequestMethod for WechatCpMethod {
    fn get_method(&self) -> String {
        match self {
            WechatCpMethod::AccessToken => String::from("/cgi-bin/gettoken"),
            WechatCpMethod::GetJsapiTicket => String::from("/cgi-bin/get_jsapi_ticket"),
            WechatCpMethod::GetSuiteJsapiTicket => String::from("/cgi-bin/ticket/get"),
            WechatCpMethod::GetOrder => String::from("/cgi-bin/service/get_order"),
            WechatCpMethod::GetOrderList => String::from("/cgi-bin/service/get_order_list"),
            WechatCpMethod::GetPreAuthCode => String::from("/cgi-bin/service/get_pre_auth_code"),
            WechatCpMethod::GetAuthInfo => String::from("/cgi-bin/service/get_auth_info"),
            WechatCpMethod::GetPermanentCode => String::from("/cgi-bin/service/get_permanent_code"),
            WechatCpMethod::GetProviderToken => String::from("/cgi-bin/service/get_provider_token"),
            WechatCpMethod::GetCorpToken => String::from("/cgi-bin/service/get_corp_token"),
            WechatCpMethod::GetSuiteToken => String::from("/cgi-bin/service/get_suite_token"),
            WechatCpMethod::JsCode2Session => String::from("/cgi-bin/miniprogram/jscode2session"),
            WechatCpMethod::GetCallbackIp => String::from("/cgi-bin/getcallbackip"),
            WechatCpMethod::GetAgentConfigTicket => String::from("/cgi-bin/ticket/get?&type=agent_config"),
            WechatCpMethod::Media(v) => v.get_method(),
            WechatCpMethod::ExternalContact(v) => v.get_method(),
            WechatCpMethod::Oauth2(v) => v.get_method(),
            WechatCpMethod::Custom{ method_url, .. } => method_url.to_string(),
            WechatCpMethod::Menu(v) => v.get_method(),
            WechatCpMethod::Message(v) => v.get_method(),
            WechatCpMethod::Tag(v) => v.get_method(),
            WechatCpMethod::License(v) => v.get_method(),
            WechatCpMethod::Department(v) => v.get_method(),
            WechatCpMethod::User(v) => v.get_method(),
            WechatCpMethod::Agent(v) => v.get_method(),
        }
    }
}

#[allow(unused)]
impl WechatCpMethod {

    pub fn need_token(&self) -> bool {
        match self {
            WechatCpMethod::Custom{ need_token, .. } => *need_token,
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
pub enum CpTagMethod {
    Create,
    Update,
    List,
    AddTagUsers,
    DeleteTagUsers,
    Delete(String),
    Get(String),
}

#[allow(unused)]
impl CpTagMethod {
    pub fn get_method(&self) -> String {
        match self {
            CpTagMethod::Create => String::from("/cgi-bin/tag/create"),
            CpTagMethod::Update => String::from("/cgi-bin/tag/update"),
            CpTagMethod::List => String::from("/cgi-bin/tag/list"),
            CpTagMethod::AddTagUsers => String::from("/cgi-bin/tag/addtagusers"),
            CpTagMethod::DeleteTagUsers => String::from("/cgi-bin/tag/deltagusers"),
            CpTagMethod::Delete(v) => format!("/cgi-bin/tag/delete?tagid={}", v),
            CpTagMethod::Get(v) => format!("/cgi-bin/tag/get?tagid={}", v),
        }
    }
}



#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum CpAgentMethod {
    Get(i32),
    Set,
    List,
}

#[allow(unused)]
impl CpAgentMethod {
    pub fn get_method(&self) -> String {
        match self {
            CpAgentMethod::Get(v) => format!("/cgi-bin/agent/get?agentid={}", v),
            CpAgentMethod::Set => String::from("/cgi-bin/agent/set"),
            CpAgentMethod::List => String::from("/cgi-bin/agent/list"),
        }
    }
}




#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum CpLicenseMethod {
    CreateOrder,
    CreateRenewOrderJob,
    SubmitOrderJob,
    ListOrder,
    GetOrder,
    ListOrderCount,
    ActiveAccount,
    BatchActiveAccount,
    GetActiveInfoByCode,
    BatchGetActiveInfoByCode,
    ListActivedAccount,
    GetActiveInfoByUser,
    BatchTransferLicense,
}

#[allow(unused)]
impl CpLicenseMethod {
    pub fn get_method(&self) -> String {
        match self {
            CpLicenseMethod::CreateOrder => String::from("/cgi-bin/license/create_new_order"),
            CpLicenseMethod::CreateRenewOrderJob => String::from("/cgi-bin/license/create_renew_order_job"),
            CpLicenseMethod::SubmitOrderJob => String::from("/cgi-bin/license/submit_order_job"),
            CpLicenseMethod::ListOrder => String::from("/cgi-bin/license/list_order"),
            CpLicenseMethod::GetOrder => String::from("/cgi-bin/license/get_order"),
            CpLicenseMethod::ListOrderCount => String::from("/cgi-bin/license/list_order_account"),
            CpLicenseMethod::ActiveAccount => String::from("/cgi-bin/license/active_account"),
            CpLicenseMethod::BatchActiveAccount => String::from("/cgi-bin/license/batch_active_account"),
            CpLicenseMethod::GetActiveInfoByCode => String::from("/cgi-bin/license/get_active_info_by_code"),
            CpLicenseMethod::BatchGetActiveInfoByCode => String::from("/cgi-bin/license/batch_get_active_info_by_code"),
            CpLicenseMethod::ListActivedAccount => String::from("/cgi-bin/license/list_actived_account"),
            CpLicenseMethod::GetActiveInfoByUser => String::from("/cgi-bin/license/get_active_info_by_user"),
            CpLicenseMethod::BatchTransferLicense => String::from("/cgi-bin/license/batch_transfer_license"),
        }
    }
}


#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum CpMenuMethod {
    Create(i32),
    Delete(i32),
    Get(i32),
}

#[allow(unused)]
impl CpMenuMethod {
    pub fn get_method(&self) -> String {
        match self {
            CpMenuMethod::Create(v) => format!("/cgi-bin/menu/create?agentid={}", v),
            CpMenuMethod::Delete(v) => format!("/cgi-bin/menu/delete?agentid={}", v),
            CpMenuMethod::Get(v) => format!("/cgi-bin/menu/get?agentid={}", v),
        }
    }
}




#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum CpUserMethod {
    AuthSuccess(String),
    Create,
    Update,
    BatchDelete,
    Invite,
    ConvertToOpenid,
    ConvertToUserid,
    GetUserid,
    GetActiveStat,
    Delete(String),
    Get(String),
    GetExternalContact(String),
    GetJoinQrcode(i32),
    List(i64),
    SimpleList(i64),
}

#[allow(unused)]
impl CpUserMethod {
    pub fn get_method(&self) -> String {
        match self {
            CpUserMethod::AuthSuccess(v) => format!("/cgi-bin/user/authsucc?userid={}", v),
            CpUserMethod::Create => String::from("/cgi-bin/user/create"),
            CpUserMethod::Update => String::from("/cgi-bin/user/update"),
            CpUserMethod::BatchDelete => String::from("/cgi-bin/user/batchdelete"),
            CpUserMethod::ConvertToOpenid => String::from("/cgi-bin/user/convert_to_openid"),
            CpUserMethod::ConvertToUserid => String::from("/cgi-bin/user/convert_to_userid"),
            CpUserMethod::GetUserid => String::from("/cgi-bin/user/getuserid"),
            CpUserMethod::Invite => String::from("/cgi-bin/batch/invite"),
            CpUserMethod::GetActiveStat => String::from("/cgi-bin/user/get_active_stat"),
            CpUserMethod::Delete(v) => format!("/cgi-bin/user/delete?userid={}", v),
            CpUserMethod::Get(v) => format!("/cgi-bin/user/get?userid={}", v),
            CpUserMethod::GetJoinQrcode(v) => format!("/cgi-bin/corp/get_join_qrcode?size_type={}", v),
            CpUserMethod::GetExternalContact(v) => format!("/cgi-bin/crm/get_external_contact?external_userid={}", v),
            CpUserMethod::List(v) => format!("/cgi-bin/user/list?department_id={}", v),
            CpUserMethod::SimpleList(v) => format!("/cgi-bin/user/simplelist?department_id={}", v),
        }
    }
}




#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum CpDepartmentMethod {
    Create,
    List,
    Update,
    SimpleList,
    Get(i64),
    Delete(i64),
}

#[allow(unused)]
impl CpDepartmentMethod {
    pub fn get_method(&self) -> String {
        match self {
            CpDepartmentMethod::Create => String::from("/cgi-bin/department/create"),
            CpDepartmentMethod::Update => String::from("/cgi-bin/department/update"),
            CpDepartmentMethod::Get(v) => format!("/cgi-bin/department/get?id={}", v),
            CpDepartmentMethod::Delete(v) => format!("/cgi-bin/department/delete?id={}", v),
            CpDepartmentMethod::List => String::from("/cgi-bin/department/list"),
            CpDepartmentMethod::SimpleList => String::from("/cgi-bin/department/simplelist"),
        }
    }
}




#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum CpMessageMethod {
    /// 发送应用消息
    Send,
    /// 查询应用消息发送统计
    Statistics,
    /// 互联企业发送应用消息
    /// https://developer.work.weixin.qq.com/document/path/90250
    LinkedCorpSend,
    /// 发送学校通知
    /// https://developer.work.weixin.qq.com/document/path/92321
    ExternalContactSend,
}

#[allow(unused)]
impl CpMessageMethod {
    pub fn get_method(&self) -> String {
        match self {
            CpMessageMethod::Send => String::from("/cgi-bin/message/send"),
            CpMessageMethod::Statistics => String::from("/cgi-bin/message/get_statistics"),
            CpMessageMethod::LinkedCorpSend => String::from("/cgi-bin/linkedcorp/message/send"),
            CpMessageMethod::ExternalContactSend => String::from("/cgi-bin/externalcontact/message/send"),
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