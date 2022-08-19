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
pub enum MaUserMethod {
    SetUserStorage
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
            WechatMaMethod::QrCode(v) => v.get_method()
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