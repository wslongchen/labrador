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
    Message(MpMessageMethod),
    QrCode(QrCodeMethod),
    Media(MpMediaMethod),
    /// 自定义方法
    Custom(String)
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
pub enum MpMediaMethod {
    /// 上传临时永久
    UploadMedia(String),
    /// 添加永久素材
    AddMaterial(String),
    /// 上传图片
    UploadImage,
    /// 获取永久素材
    GetMaterial,
    /// 删除永久素材
    DeleteMaterial,
    /// 获取永久素材数量
    GetMaterialCount,
    /// 获取永久素材列表
    GetMaterialList,
    /// 获取临时素材
    GetMedia,
}

#[allow(unused)]
impl MpMediaMethod {
    pub fn get_method(&self) -> String {
        match self {
            MpMediaMethod::UploadMedia(v) => format!("/cgi-bin/media/upload?type={}", v),
            MpMediaMethod::AddMaterial(v) => format!("/cgi-bin/material/add_material?type={}", v),
            MpMediaMethod::GetMaterial => String::from("/cgi-bin/material/get_material"),
            MpMediaMethod::DeleteMaterial => String::from("/cgi-bin/material/del_material"),
            MpMediaMethod::GetMaterialCount => String::from("/cgi-bin/material/get_materialcount"),
            MpMediaMethod::GetMaterialList => String::from("/cgi-bin/material/batchget_material"),
            MpMediaMethod::UploadImage => String::from("/cgi-bin/media/uploadimg"),
            MpMediaMethod::GetMedia => String::from("/cgi-bin/media/get"),
        }
    }
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
pub enum MpMessageMethod {
    /// 客服消息
    CustomSend,
    SendTemplate,
    /// 订阅模板消息
    SubscribeMessage,
    /// 设置行业
    SetIndustry,
    /// 获取设置行业
    GetIndustry,
    /// 获取模版ID
    GetTemplateId,
    /// 获取模版列表
    GetTemplateList,
    /// 删除模版列表
    DeleteTemplate,
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
            WechatMpMethod::CodeSession => String::from("/sns/jscode2session"),
            WechatMpMethod::AccessToken => String::from("/cgi-bin/token"),
            WechatMpMethod::Oauth2(v) => v.get_method(),
            WechatMpMethod::CustomService(v) => v.get_method(),
            WechatMpMethod::User(v) => v.get_method(),
            WechatMpMethod::Menu(v) => v.get_method(),
            WechatMpMethod::Message(v) => v.get_method(),
            WechatMpMethod::QrCode(v) => v.get_method(),
            WechatMpMethod::Media(v) => v.get_method(),
            WechatMpMethod::Custom(v) => v.to_string(),
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
            CustomServiceMethod::AccountAdd => String::from("/cgi-bin/customservice/kfaccount/add"),
            CustomServiceMethod::AccountUpdate => String::from("/cgi-bin/customservice/kfaccount/update"),
            CustomServiceMethod::AccountDelete => String::from("/cgi-bin/customservice/kfaccount/del"),
            CustomServiceMethod::AccountList => String::from("/cgi-bin/customservice/getkflist"),
            CustomServiceMethod::AccountOnlineList => String::from("/cgi-bin/customservice/getonlinekflist"),
        }
    }
}



#[allow(unused)]
impl Oauth2Method {
    pub fn get_method(&self) -> String {
        match *self {
            Oauth2Method::AccessToken => String::from("/sns/oauth2/access_token"),
            Oauth2Method::RefreshToken => String::from("/sns/oauth2/refresh_token"),
            Oauth2Method::UserInfo => String::from("/sns/userinfo"),
        }
    }
}



#[allow(unused)]
impl UserMethod {
    pub fn get_method(&self) -> String {
        match *self {
            UserMethod::Info => String::from("/cgi-bin/user/info"),
            UserMethod::UpdateRemark => String::from("/cgi-bin/user/info/updateremark"),
            UserMethod::Get => String::from("/cgi-bin/user/get"),
            UserMethod::GetGroupId => String::from("/cgi-bin/groups/getid"),
            UserMethod::GetBatch => String::from("/cgi-bin/user/info/batchget"),
        }
    }
}


#[allow(unused)]
impl MenuMethod {
    pub fn get_method(&self) -> String {
        match *self {
            MenuMethod::Create => String::from("/cgi-bin/menu/create"),
            MenuMethod::GetCurrentMenuInfo => String::from("/cgi-bin/get_current_selfmenu_info"),
            MenuMethod::Get => String::from("/cgi-bin/menu/get"),
            MenuMethod::Delete => String::from("/cgi-bin/menu/delete"),
        }
    }
}


#[allow(unused)]
impl MpMessageMethod {
    pub fn get_method(&self) -> String {
        match *self {
            MpMessageMethod::CustomSend => String::from("/cgi-bin/message/custom/send"),
            MpMessageMethod::SendTemplate => String::from("/cgi-bin/message/template/send"),
            MpMessageMethod::SubscribeMessage => String::from("/cgi-bin/message/template/subscribe"),
            MpMessageMethod::SetIndustry => String::from("/cgi-bin/template/api_set_industry"),
            MpMessageMethod::GetIndustry => String::from("/cgi-bin/template/get_industry"),
            MpMessageMethod::GetTemplateId => String::from("/cgi-bin/template/api_add_template"),
            MpMessageMethod::GetTemplateList => String::from("/cgi-bin/template/get_all_private_template"),
            MpMessageMethod::DeleteTemplate => String::from("/cgi-bin/template/del_private_template"),
        }
    }
}

#[allow(unused)]
impl QrCodeMethod {
    pub fn get_method(&self) -> String {
        match *self {
            QrCodeMethod::Create => String::from("/cgi-bin/qrcode/create"),
            QrCodeMethod::GetWxaCodeUnlimit => String::from("/cgi-bin/wxa/getwxacodeunlimit"),
            QrCodeMethod::ShowQrCode => String::from("https://mp.weixin.qq.com/cgi-bin/showqrcode"),
        }
    }
}
