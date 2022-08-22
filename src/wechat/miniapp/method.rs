use crate::RequestMethod;

#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum WechatMaMethod {
    AccessToken,
    /// codesession
    CodeSession,
    /// codesession
    QrCode(MaQrCodeMethod),
    /// 用户相关
    User(MaUserMethod),
    /// 媒体文件
    Media(MaMediaMethod),
    /// 消息相关
    Message(MaMessageMethod),
    /// 自定义方法
    Custom(String)
}


#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum MaQrCodeMethod {
    CreateWxaQrCode,
    GetWxaQrCode,
    GetWxaCodeUnlimit,
}

#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum MaMediaMethod {
    UploadMedia(String),
    GetMedia,
}


#[allow(unused)]
impl MaMediaMethod {
    pub fn get_method(&self) -> String {
        match self {
            MaMediaMethod::UploadMedia(v) => format!("/cgi-bin/media/upload?type={}", v),
            MaMediaMethod::GetMedia => String::from("/cgi-bin/media/get"),
        }
    }
}
#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum MaMessageMethod {
    SendCustomMsg,
    SendSubscribeMsg,
    SendUniformTemplate,
    CreateActivityId,
    SendUpdatableMsg,
}


#[allow(unused)]
impl MaMessageMethod {
    pub fn get_method(&self) -> String {
        match *self {
            MaMessageMethod::SendCustomMsg => String::from("/cgi-bin/message/custom/send"),
            MaMessageMethod::SendSubscribeMsg => String::from("/cgi-bin/message/subscribe/send"),
            MaMessageMethod::SendUniformTemplate => String::from("/cgi-bin/message/wxopen/template/uniform_send"),
            MaMessageMethod::CreateActivityId => String::from("/cgi-bin/message/wxopen/activityid/create"),
            MaMessageMethod::SendUpdatableMsg => String::from("/cgi-bin/message/wxopen/updatablemsg/send"),
        }
    }
}


#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum MaUserMethod {
    SetUserStorage,
    /// 获取手机号信息,基础库:2.21.2及以上
    GetPhoneNumber,
}


#[allow(unused)]
impl MaQrCodeMethod {
    pub fn get_method(&self) -> String {
        match *self {
            MaQrCodeMethod::CreateWxaQrCode => String::from("/cgi-bin/wxaapp/createwxaqrcode"),
            MaQrCodeMethod::GetWxaCodeUnlimit => String::from("/wxa/getwxacodeunlimit"),
            MaQrCodeMethod::GetWxaQrCode => String::from("/wxa/getwxacode"),
        }
    }
}



#[allow(unused)]
impl MaUserMethod {
    pub fn get_method(&self) -> String {
        match *self {
            MaUserMethod::SetUserStorage => String::from("/wxa/set_user_storage"),
            MaUserMethod::GetPhoneNumber => String::from("/wxa/business/getuserphonenumber"),
        }
    }
}



impl RequestMethod for WechatMaMethod {
    fn get_method(&self) -> String {
        match self {
            WechatMaMethod::CodeSession => String::from("/sns/jscode2session"),
            WechatMaMethod::AccessToken => String::from("/cgi-bin/token"),
            WechatMaMethod::Custom(v) => v.to_string(),
            WechatMaMethod::User(v) => v.get_method(),
            WechatMaMethod::Media(v) => v.get_method(),
            WechatMaMethod::QrCode(v) => v.get_method(),
            WechatMaMethod::Message(v) => v.get_method(),
        }
    }
}

#[allow(unused)]
impl WechatMaMethod {

    pub fn need_token(&self) -> bool {
        match self {
            WechatMaMethod::CodeSession | WechatMaMethod::AccessToken => false,
            _ => true,
        }
    }
}