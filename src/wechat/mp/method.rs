use crate::RequestMethod;

#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum WechatMpMethod {
    AccessToken,
    Oauth2(Oauth2Method),
    /// codesession
    CodeSession,
    /// 客户服务
    CustomService(CustomServiceMethod),
    User(UserMethod),
    Menu(MenuMethod),
    Message(MessageMethod),
    QrCode(QrCodeMethod),
}


#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum Oauth2Method {
    UserInfo,
    AccessToken,
    RefreshToken,
}

#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum CustomServiceMethod {
    AccountAdd,
    AccountUpdate,
    AccountDelete,
    AccountList,
    AccountOnlineList,
}

#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum UserMethod {
    Info,
    UpdateRemark,
    Get,
    GetGroupId,
    GetBatch,
}


#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum MenuMethod {
    Create,
    GetCurrentMenuInfo,
    Get,
    Delete,
}



#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum QrCodeMethod {
    Create,
    GetWxaCodeUnlimit,
    ShowQrCode,
}



#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum MessageMethod {
    Send,
    SendTemplate,
    SendUniform,
    SendSubscribe,
}




#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum PayMethod {
    /// 统一下单
    UnifiedOrder,
    MicroPay,
    /// 关闭订单
    CloseOrder,
    /// 退款
    Refund,
    /// 查询订单
    OrderQuery,
}

impl RequestMethod for WechatMpMethod {
    fn get_method(&self) -> String {
        match self {
            WechatMpMethod::CodeSession => String::from("https://api.weixin.qq.com/sns/jscode2session"),
            WechatMpMethod::AccessToken => String::from("token"),
            WechatMpMethod::Oauth2(v) => v.get_method(),
            WechatMpMethod::CustomService(v) => v.get_method(),
            WechatMpMethod::User(v) => v.get_method(),
            WechatMpMethod::Menu(v) => v.get_method(),
            WechatMpMethod::Message(v) => v.get_method(),
            WechatMpMethod::QrCode(v) => v.get_method(),
        }
    }
}

#[allow(unused)]
impl WechatMpMethod {

    pub fn need_token(&self) -> bool {
        match self {
            WechatMpMethod::CodeSession | WechatMpMethod::AccessToken | WechatMpMethod::Oauth2(_)  => false,
            _ => true,
        }
    }
}


#[allow(unused)]
impl CustomServiceMethod {
    pub fn get_method(&self) -> String {
        match *self {
            CustomServiceMethod::AccountAdd => String::from("https://api.weixin.qq.com/customservice/kfaccount/add"),
            CustomServiceMethod::AccountUpdate => String::from("https://api.weixin.qq.com/customservice/kfaccount/update"),
            CustomServiceMethod::AccountDelete => String::from("https://api.weixin.qq.com/customservice/kfaccount/del"),
            CustomServiceMethod::AccountList => String::from("customservice/getkflist"),
            CustomServiceMethod::AccountOnlineList => String::from("customservice/getonlinekflist"),
        }
    }
}



#[allow(unused)]
impl Oauth2Method {
    pub fn get_method(&self) -> String {
        match *self {
            Oauth2Method::AccessToken => String::from("https://api.weixin.qq.com/sns/oauth2/access_token"),
            Oauth2Method::RefreshToken => String::from("https://api.weixin.qq.com/sns/oauth2/refresh_token"),
            Oauth2Method::UserInfo => String::from("https://api.weixin.qq.com/sns/userinfo"),
        }
    }
}



#[allow(unused)]
impl UserMethod {
    pub fn get_method(&self) -> String {
        match *self {
            UserMethod::Info => String::from("user/info"),
            UserMethod::UpdateRemark => String::from("user/info/updateremark"),
            UserMethod::Get => String::from("user/get"),
            UserMethod::GetGroupId => String::from("groups/getid"),
            UserMethod::GetBatch => String::from("user/info/batchget"),
        }
    }
}


#[allow(unused)]
impl MenuMethod {
    pub fn get_method(&self) -> String {
        match *self {
            MenuMethod::Create => String::from("menu/create"),
            MenuMethod::GetCurrentMenuInfo => String::from("get_current_selfmenu_info"),
            MenuMethod::Get => String::from("menu/get"),
            MenuMethod::Delete => String::from("menu/delete"),
        }
    }
}


#[allow(unused)]
impl MessageMethod {
    pub fn get_method(&self) -> String {
        match *self {
            MessageMethod::Send => String::from("message/custom/send"),
            MessageMethod::SendTemplate => String::from("message/template/send"),
            MessageMethod::SendUniform => String::from("message/wxopen/template/uniform_send"),
            MessageMethod::SendSubscribe => String::from("message/subscribe/send"),
        }
    }
}

#[allow(unused)]
impl QrCodeMethod {
    pub fn get_method(&self) -> String {
        match *self {
            QrCodeMethod::Create => String::from("qrcode/create"),
            QrCodeMethod::GetWxaCodeUnlimit => String::from("https://api.weixin.qq.com/wxa/getwxacodeunlimit"),
            QrCodeMethod::ShowQrCode => String::from("https://mp.weixin.qq.com/cgi-bin/showqrcode"),
        }
    }
}
