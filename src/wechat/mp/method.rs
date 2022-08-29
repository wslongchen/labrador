use crate::RequestMethod;

#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum WechatMpMethod {
    /// 获取access_token
    AccessToken,
    /// 短key托管(生成短key的url)
    GenShortenUrl,
    GetCallbackIp,
    QrConnectUrl,
    /// 获得各种类型的ticket
    GetTicket,
    /// 短key解析(解析短key的url)
    FetchShortenUrl,
    Oauth2(Oauth2Method),
    /// codesession
    CodeSession,
    /// 客户服务
    CustomService(MpCustomServiceMethod),
    /// ocr
    Ocr(MpOcrMethod),
    /// wifi服务
    Wifi(MpWifiMethod),
    /// 会员卡服务
    MemberCard(MpMemeberCardMethod),
    /// 卡券服务
    Card(MpCardMethod),
    /// 用户服务
    User(MpUserMethod),
    /// 菜单服务
    Menu(MpMenuMethod),
    /// 订阅消息
    SubscribeMessage(MpSubscribeMessageMethod),
    /// 模板消息
    TemplateMessage(MpTemplateMessageMethod),
    /// 二维码
    QrCode(MpQrCodeMethod),
    /// 媒体文件
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
pub enum MpCustomServiceMethod {
    /// 客服消息
    CustomSend,
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
    /// 获取素材JSSDK
    GetMediaJssdk,
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
            MpMediaMethod::GetMediaJssdk => String::from("/cgi-bin/media/get/jssdk"),
        }
    }
}

#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum MpUserMethod {
    Info,
    UpdateRemark,
    Get,
    GetGroupId,
    GetBatch,
}


#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum MpMenuMethod {
    Create,
    GetCurrentMenuInfo,
    Get,
    Delete,
}



#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum MpQrCodeMethod {
    Create,
    GetWxaCodeUnlimit,
    ShowQrCode,
}

#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum MpTemplateMessageMethod {
    SendTemplate,
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
impl MpTemplateMessageMethod {
    pub fn get_method(&self) -> String {
        match *self {
            MpTemplateMessageMethod::SendTemplate => String::from("/cgi-bin/message/template/send"),
            MpTemplateMessageMethod::SetIndustry => String::from("/cgi-bin/template/api_set_industry"),
            MpTemplateMessageMethod::GetIndustry => String::from("/cgi-bin/template/get_industry"),
            MpTemplateMessageMethod::GetTemplateId => String::from("/cgi-bin/template/api_add_template"),
            MpTemplateMessageMethod::GetTemplateList => String::from("/cgi-bin/template/get_all_private_template"),
            MpTemplateMessageMethod::DeleteTemplate => String::from("/cgi-bin/template/del_private_template"),
        }
    }
}


#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum MpWifiMethod {
    ShopList,
    GetShop,
    UpdateShop,
}

#[allow(unused)]
impl MpWifiMethod {
    pub fn get_method(&self) -> String {
        match *self {
            MpWifiMethod::ShopList => String::from("/bizwifi/shop/list"),
            MpWifiMethod::GetShop => String::from("/bizwifi/shop/get"),
            MpWifiMethod::UpdateShop => String::from("/bizwifi/shop/update"),
        }
    }
}





#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum MpMemeberCardMethod {
    Create,
    Activate,
    GetUserInfo,
    UpdateUser,
    /// 会员卡激活之微信开卡接口(wx_activate=true情况调用).
    ActivateSetUser,
    /// 获取会员卡开卡插件参数
    ActivateGetUrl,
    /// 会员卡信息更新
    Update,
    /// 跳转型会员卡开卡字段.
    /// 获取用户提交资料(wx_activate=true情况调用),开发者根据activate_ticket获取到用户填写的信息
    GetActivateTempInfo,
}

#[allow(unused)]
impl MpMemeberCardMethod {
    pub fn get_method(&self) -> String {
        match *self {
            MpMemeberCardMethod::Create => String::from("/card/create"),
            MpMemeberCardMethod::Activate => String::from("/card/membercard/activate"),
            MpMemeberCardMethod::GetUserInfo => String::from("/card/membercard/userinfo/get"),
            MpMemeberCardMethod::UpdateUser => String::from("/card/membercard/updateuser"),
            MpMemeberCardMethod::ActivateSetUser => String::from("/card/membercard/activateuserform/set"),
            MpMemeberCardMethod::ActivateSetUser => String::from("/card/membercard/activateuserform/set"),
            MpMemeberCardMethod::ActivateGetUrl => String::from("/card/membercard/activate/geturl"),
            MpMemeberCardMethod::Update => String::from("/card/update"),
            MpMemeberCardMethod::GetActivateTempInfo => String::from("/card/membercard/activatetempinfo/get"),
        }
    }
}





#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum MpCardMethod {
    Create,
    Get,
    Update,
    CodeDecrypt,
    CodeGet,
    CodeMark,
    SetWhiteList,
    CreateQrcode,
    CodeConsume,
    CreateLandingpage,
    /// 将用户的卡券设置为失效状态
    UnavailabeCode,
    Delete,
    /// 导入code接口
    CodeDeposit,
    /// 查询导入code数目接口
    GetDepositCount,
    /// 核查code接口
    CheckCode,
    /// 图文消息群发卡券
    GetHtml,
    /// 修改库存接口
    ModifyStock,
    /// 更改Code接口
    UpdateCode,
    /// 设置买单接口
    SetPayCell,
    /// 设置自助核销接口
    SetSelfConsumerCell,
    /// 获取用户已领取卡券接口
    GetUserCardList,
    /// 创建子商户
    SubmitSubmerchant,
    /// 卡券开放类目查询接口
    GetApplyProtocol,
    /// 修改子商户
    UpdateSubmerchant,
    /// 拉取单个子商户信息接口
    GetSubmerchant,
    /// 批量拉取子商户信息接口
    BatchGetSubmerchant,
}

#[allow(unused)]
impl MpCardMethod {
    pub fn get_method(&self) -> String {
        match *self {
            MpCardMethod::Create => String::from("/card/create"),
            MpCardMethod::Get => String::from("/card/get"),
            MpCardMethod::Update => String::from("/card/update"),
            MpCardMethod::CodeDecrypt => String::from("/card/code/decrypt"),
            MpCardMethod::CodeGet => String::from("/card/code/get"),
            MpCardMethod::CodeConsume => String::from("/card/code/consume"),
            MpCardMethod::CodeMark => String::from("/card/code/mark"),
            MpCardMethod::SetWhiteList => String::from("/card/testwhitelist/set"),
            MpCardMethod::CreateQrcode => String::from("/card/qrcode/create"),
            MpCardMethod::CreateLandingpage => String::from("/card/landingpage/create"),
            MpCardMethod::UnavailabeCode => String::from("/card/code/unavailable"),
            MpCardMethod::Delete => String::from("/card/delete"),
            MpCardMethod::CodeDeposit => String::from("/card/code/deposit"),
            MpCardMethod::GetDepositCount => String::from("/card/code/getdepositcount"),
            MpCardMethod::CheckCode => String::from("/card/code/checkcode"),
            MpCardMethod::GetHtml => String::from("/card/mpnews/gethtml"),
            MpCardMethod::ModifyStock => String::from("/card/modifystock"),
            MpCardMethod::UpdateCode => String::from("/card/code/update"),
            MpCardMethod::SetPayCell => String::from("/card/paycell/set"),
            MpCardMethod::SetSelfConsumerCell => String::from("/card/selfconsumecell/set"),
            MpCardMethod::GetUserCardList => String::from("/card/user/getcardlist"),
            MpCardMethod::SubmitSubmerchant => String::from("/card/submerchant/submit"),
            MpCardMethod::UpdateSubmerchant => String::from("/card/submerchant/update"),
            MpCardMethod::GetApplyProtocol => String::from("/card/getapplyprotocol"),
            MpCardMethod::GetSubmerchant => String::from("/card/submerchant/get"),
            MpCardMethod::BatchGetSubmerchant => String::from("/card/submerchant/batchget"),
            MpCardMethod::BatchGetSubmerchant => String::from("/card/submerchant/batchget"),
        }
    }
}






#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum MpOcrMethod {
    IdCard,
    BankCard,
    Driving,
    DrivingLicense,
    BizLicense,
    Comm
}

#[allow(unused)]
impl MpOcrMethod {
    pub fn get_method(&self) -> String {
        match *self {
            MpOcrMethod::IdCard => String::from("/cv/ocr/idcard"),
            MpOcrMethod::BankCard => String::from("/cv/ocr/bankcard"),
            MpOcrMethod::Driving => String::from("/cv/ocr/driving"),
            MpOcrMethod::DrivingLicense => String::from("/cv/ocr/drivinglicense"),
            MpOcrMethod::BizLicense => String::from("/cv/ocr/bizlicense"),
            MpOcrMethod::Comm => String::from("/cv/ocr/comm"),
        }
    }
}





#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum MpSubscribeMessageMethod {
    /// 一次性订阅模板消息
    SubscribeMessageOnce,
    /// 订阅通知消息发送
    SendSubscribeMessage,
    /// 订阅授权url
    SubscribeAuthorizeUrl,
    /// 获取模板标题下的关键词列表
    GetPubTemplateKeywords,
    /// 获取模板标题下的关键词列表
    GetPubTemplateTitles,
    /// 组合模板并添加至帐号下的个人模板库
    AddTemplate,
    /// 获取当前帐号下的个人模板列表
    GetTemplate,
    /// 删除帐号下的某个模板
    DeleteTemplate,
    /// 获取小程序账号的类目
    GetCategory,
}

#[allow(unused)]
impl MpSubscribeMessageMethod {
    pub fn get_method(&self) -> String {
        match *self {
            MpSubscribeMessageMethod::SubscribeMessageOnce => String::from("/cgi-bin/message/template/subscribe"),
            MpSubscribeMessageMethod::SubscribeAuthorizeUrl => String::from("/mp/subscribemsg?action=get_confirm"),
            MpSubscribeMessageMethod::GetPubTemplateKeywords => String::from("/wxaapi/newtmpl/getpubtemplatekeywords"),
            MpSubscribeMessageMethod::GetPubTemplateTitles => String::from("/wxaapi/newtmpl/getpubtemplatetitles"),
            MpSubscribeMessageMethod::AddTemplate => String::from("/wxaapi/newtmpl/addtemplate"),
            MpSubscribeMessageMethod::GetTemplate => String::from("/wxaapi/newtmpl/gettemplate"),
            MpSubscribeMessageMethod::DeleteTemplate => String::from("/wxaapi/newtmpl/deltemplate"),
            MpSubscribeMessageMethod::GetCategory => String::from("/wxaapi/newtmpl/getcategory"),
            MpSubscribeMessageMethod::SendSubscribeMessage => String::from("/cgi-bin/message/subscribe/bizsend"),
        }
    }
}


impl RequestMethod for WechatMpMethod {
    fn get_method(&self) -> String {
        match self {
            WechatMpMethod::CodeSession => String::from("/sns/jscode2session"),
            WechatMpMethod::AccessToken => String::from("/cgi-bin/token"),
            WechatMpMethod::GenShortenUrl => String::from("/cgi-bin/shorten/gen"),
            WechatMpMethod::FetchShortenUrl => String::from("/cgi-bin/shorten/fetch"),
            WechatMpMethod::GetTicket => String::from("/cgi-bin/ticket/getticket"),
            WechatMpMethod::GetCallbackIp => String::from("/cgi-bin/getcallbackip"),
            WechatMpMethod::QrConnectUrl => String::from("/connect/qrconnect"),
            WechatMpMethod::Oauth2(v) => v.get_method(),
            WechatMpMethod::CustomService(v) => v.get_method(),
            WechatMpMethod::User(v) => v.get_method(),
            WechatMpMethod::Menu(v) => v.get_method(),
            WechatMpMethod::MemberCard(v) => v.get_method(),
            WechatMpMethod::Wifi(v) => v.get_method(),
            WechatMpMethod::TemplateMessage(v) => v.get_method(),
            WechatMpMethod::QrCode(v) => v.get_method(),
            WechatMpMethod::Media(v) => v.get_method(),
            WechatMpMethod::Custom(v) => v.to_string(),
            WechatMpMethod::SubscribeMessage(v) => v.get_method(),
            WechatMpMethod::Ocr(v) => v.get_method(),
            WechatMpMethod::Card(v) => v.get_method(),
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
impl MpCustomServiceMethod {
    pub fn get_method(&self) -> String {
        match *self {
            MpCustomServiceMethod::CustomSend => String::from("/cgi-bin/message/custom/send"),
            MpCustomServiceMethod::AccountAdd => String::from("/cgi-bin/customservice/kfaccount/add"),
            MpCustomServiceMethod::AccountUpdate => String::from("/cgi-bin/customservice/kfaccount/update"),
            MpCustomServiceMethod::AccountDelete => String::from("/cgi-bin/customservice/kfaccount/del"),
            MpCustomServiceMethod::AccountList => String::from("/cgi-bin/customservice/getkflist"),
            MpCustomServiceMethod::AccountOnlineList => String::from("/cgi-bin/customservice/getonlinekflist"),
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
impl MpUserMethod {
    pub fn get_method(&self) -> String {
        match *self {
            MpUserMethod::Info => String::from("/cgi-bin/user/info"),
            MpUserMethod::UpdateRemark => String::from("/cgi-bin/user/info/updateremark"),
            MpUserMethod::Get => String::from("/cgi-bin/user/get"),
            MpUserMethod::GetGroupId => String::from("/cgi-bin/groups/getid"),
            MpUserMethod::GetBatch => String::from("/cgi-bin/user/info/batchget"),
        }
    }
}


#[allow(unused)]
impl MpMenuMethod {
    pub fn get_method(&self) -> String {
        match *self {
            MpMenuMethod::Create => String::from("/cgi-bin/menu/create"),
            MpMenuMethod::GetCurrentMenuInfo => String::from("/cgi-bin/get_current_selfmenu_info"),
            MpMenuMethod::Get => String::from("/cgi-bin/menu/get"),
            MpMenuMethod::Delete => String::from("/cgi-bin/menu/delete"),
        }
    }
}


#[allow(unused)]
impl MpQrCodeMethod {
    pub fn get_method(&self) -> String {
        match *self {
            MpQrCodeMethod::Create => String::from("/cgi-bin/qrcode/create"),
            MpQrCodeMethod::GetWxaCodeUnlimit => String::from("/cgi-bin/wxa/getwxacodeunlimit"),
            MpQrCodeMethod::ShowQrCode => String::from("https://mp.weixin.qq.com/cgi-bin/showqrcode"),
        }
    }
}
