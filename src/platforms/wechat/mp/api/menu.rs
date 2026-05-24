/*
 *
 *  *
 *  *      Copyright (c) 2018-2025, SnackCloud All rights reserved.
 *  *
 *  *   Redistribution and use in source and binary forms, with or without
 *  *   modification, are permitted provided that the following conditions are met:
 *  *
 *  *   Redistributions of source code must retain the above copyright notice,
 *  *   this list of conditions and the following disclaimer.
 *  *   Redistributions in binary form must reproduce the above copyright
 *  *   notice, this list of conditions and the following disclaimer in the
 *  *   documentation and/or other materials provided with the distribution.
 *  *   Neither the name of the www.snackcloud.cn developer nor the names of its
 *  *   contributors may be used to endorse or promote products derived from
 *  *   this software without specific prior written permission.
 *  *   Author: SnackCloud
 *  *
 *
 */

//!
//! 请注意：

//! 自定义菜单最多包括3个一级菜单，每个一级菜单最多包含5个二级菜单。
//! 一级菜单最多4个汉字，二级菜单最多7个汉字，多出来的部分将会以“...”代替。
//! 创建自定义菜单后，菜单的刷新策略是，在用户进入公众号会话页或公众号profile页时，如果发现上一次拉取菜单的请求在5分钟以前，就会拉取一下菜单，如果菜单有更新，就会刷新客户端的菜单。测试时可以尝试取消关注公众账号后再次关注，则可以看到创建后的效果。​
//! 自定义菜单接口可实现多种类型按钮，如下：

//! click：点击推事件用户点击click类型按钮后，微信服务器会通过消息接口推送消息类型为event的结构给开发者（参考消息接口指南），并且带上按钮中开发者填写的key值，开发者可以通过自定义的key值与用户进行交互；
//! view：跳转URL用户点击view类型按钮后，微信客户端将会打开开发者在按钮中填写的网页URL，可与网页授权获取用户基本信息接口结合，获得用户基本信息。
//! scancode_push：扫码推事件用户点击按钮后，微信客户端将调起扫一扫工具，完成扫码操作后显示扫描结果（如果是URL，将进入URL），且会将扫码的结果传给开发者，开发者可以下发消息。
//! scancode_waitmsg：扫码推事件且弹出“消息接收中”提示框用户点击按钮后，微信客户端将调起扫一扫工具，完成扫码操作后，将扫码的结果传给开发者，同时收起扫一扫工具，然后弹出“消息接收中”提示框，随后可能会收到开发者下发的消息。
//! pic_sysphoto：弹出系统拍照发图用户点击按钮后，微信客户端将调起系统相机，完成拍照操作后，会将拍摄的相片发送给开发者，并推送事件给开发者，同时收起系统相机，随后可能会收到开发者下发的消息。
//! pic_photo_or_album：弹出拍照或者相册发图用户点击按钮后，微信客户端将弹出选择器供用户选择“拍照”或者“从手机相册选择”。用户选择后即走其他两种流程。
//! pic_weixin：弹出微信相册发图器用户点击按钮后，微信客户端将调起微信相册，完成选择操作后，将选择的相片发送给开发者的服务器，并推送事件给开发者，同时收起相册，随后可能会收到开发者下发的消息。
//! location_select：弹出地理位置选择器用户点击按钮后，微信客户端将调起地理位置选择工具，完成选择操作后，将选择的地理位置发送给开发者的服务器，同时收起位置选择工具，随后可能会收到开发者下发的消息。
//! media_id：下发消息（除文本消息）用户点击media_id类型按钮后，微信服务器会将开发者填写的永久素材id对应的素材下发给用户，永久素材类型可以是图片、音频、视频、图文消息。请注意：永久素材id必须是在“素材管理/新增永久素材”接口上传后获得的合法id。
//! view_limited：跳转图文消息URL用户点击view_limited类型按钮后，微信客户端将打开开发者在按钮中填写的永久素材id对应的图文消息URL，永久素材类型只支持图文消息。请注意：永久素材id必须是在“素材管理/新增永久素材”接口上传后获得的合法id。​
//! 请注意，3到8的所有事件，仅支持微信iPhone5.4.1以上版本，和Android5.4以上版本的微信用户，旧版本微信用户点击后将没有回应，开发者也不能正常接收到事件推送。9和10，是专门给第三方平台旗下未微信认证（具体而言，是资质认证未通过）的订阅号准备的事件类型，它们是没有事件推送的，能力相对受限，其他类型的公众号不必使用。
//! 
//! 
use serde::{Deserialize, Serialize};
use crate::errors::LabradorResult;
use crate::wechat::client::WechatApiResponse;
use crate::wechat::mp::WechatMpClient;

#[derive(Debug, Clone)]
pub struct WechatMpMenu<'a> {
    client: &'a WechatMpClient,
}

#[allow(unused)]
impl<'a> WechatMpMenu<'a> {

    #[inline]
    pub fn new(client: &'a WechatMpClient) -> WechatMpMenu<'a> {
        WechatMpMenu {
            client,
        }
    }

    /// <pre>
    /// 自定义菜单创建接口
    /// 详情请见：https://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1421141013&token=&lang=zh_CN
    /// 如果要创建个性化菜单，请设置matchrule属性
    /// 详情请见：https://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1455782296&token=&lang=zh_CN
    /// </pre>
    pub async fn create_menu(&self, menu: &Menu) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self.client.wechat_client()
            .post("/cgi-bin/menu/create", serde_json::to_value(menu).unwrap())
            .await?;
        Ok(response)
    }

    /// 自定义菜单查询接口
    /// 详情[请见](https://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1421141014&token=&lang=zh_CN)
    /// 获取菜单信息
    pub async fn get_menu(&self) -> LabradorResult<MenuResponse> {
        let response: WechatApiResponse<MenuResponse> = self.client.wechat_client()
            .get("/cgi-bin/menu/get")
            .await?;

        response.into_result()
    }

    /// <pre>
    /// 删除个性化菜单接口
    /// 详情[请见](https://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1455782296&token=&lang=zh_CN)
    /// </pre>
    pub async fn delete_menu(&self) -> LabradorResult<()> {
        let response: WechatApiResponse<WechatApiResponse> = self.client.wechat_client()
            .get("/cgi-bin/menu/delete")
            .await?;

        response.into_result().map(|_| ())
    }

    /// <pre>
    /// 自定义菜单创建接口
    /// 详情请见：https://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1421141013&token=&lang=zh_CN
    /// 如果要创建个性化菜单，请设置matchrule属性
    /// 详情请见：https://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1455782296&token=&lang=zh_CN
    /// </pre>
    pub async fn create_custom_menu(&self, buttons: Menu) -> LabradorResult<WechatApiResponse> {
        self.create_menu(&buttons).await
    }

    /// <pre>
    /// 获取自定义菜单配置接口
    /// 本接口将会提供公众号当前使用的自定义菜单的配置，如果公众号是通过API调用设置的菜单，则返回菜单的开发配置，而如果公众号是在公众平台官网通过网站功能发布菜单，则本接口返回运营者设置的菜单配置。
    /// 请注意：
    /// 1、第三方平台开发者可以通过本接口，在旗下公众号将业务授权给你后，立即通过本接口检测公众号的自定义菜单配置，并通过接口再次给公众号设置好自动回复规则，以提升公众号运营者的业务体验。
    /// 2、本接口与自定义菜单查询接口的不同之处在于，本接口无论公众号的接口是如何设置的，都能查询到接口，而自定义菜单查询接口则仅能查询到使用API设置的菜单配置。
    /// 3、认证/未认证的服务号/订阅号，以及接口测试号，均拥有该接口权限。
    /// 4、从第三方平台的公众号登录授权机制上来说，该接口从属于消息与菜单权限集。
    /// 5、本接口中返回的图片/语音/视频为临时素材（临时素材每次获取都不同，3天内有效，通过素材管理-获取临时素材接口来获取这些素材），本接口返回的图文消息为永久素材素材（通过素材管理-获取永久素材接口来获取这些素材）。
    ///  接口调用请求说明:
    /// http请求方式: GET（请使用https协议）
    /// https://api.weixin.qq.com/cgi-bin/get_current_selfmenu_info?access_token=ACCESS_TOKEN
    /// </pre>
    pub async fn get_current_selfmenu_info(&self) -> LabradorResult<SelfMenuInfoResponse> {
        let response: WechatApiResponse<SelfMenuInfoResponse> = self.client.wechat_client()
            .get("/cgi-bin/get_current_selfmenu_info")
            .await?;
        response.into_result()
    }

    /// <pre>
    /// 删除自定义菜单
    /// 详情[请见](https://developers.weixin.qq.com/doc/subscription/api/custommenu/api_deletemenu.html#HTTPS-%E8%B0%83%E7%94%A8)
    /// </pre>
    pub async fn delete(&self) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self.client.wechat_client()
            .get("/cgi-bin/menu/delete")
            .await?;
        Ok(response)
    }
}

//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct SelfMenuInfoResponse {
    /// 菜单是否开启，0代表未开启，1代表开启
    pub is_menu_open: Option<u8>,
    /// 菜单标题，不超过16个字节，子菜单不超过60个字节
    pub selfmenu_info: Option<SelfMenuInfo>,
}

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct SelfMenuInfo {
    /// 菜单是否开启，0代表未开启，1代表开启
    pub button: Option<Vec<SelfMenuButton>>,
}


#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct SelfMenuButton {
    #[serde(rename = "type")]
    pub button_type: Option<String>,
    /// 菜单标题，不超过16个字节，子菜单不超过60个字节
    pub name: String,
    /// view、miniprogram类型必须
    /// 网页 链接，用户点击菜单可打开链接，不超过1024字节。 
    /// type为miniprogram时，不支持小程序的老版本客户端将打开本url。
    pub url: Option<String>,
    /// 菜单KEY值，用于消息接口推送，不超过128字节
    pub key: Option<String>,
    /// 调用新增永久素材接口返回的合法media_id
    pub media_id: Option<String>,
    /// 小程序的appid（仅认证公众号可配置）
    pub appid: Option<String>,
    /// 小程序的页面路径
    pub pagepath: Option<String>,
    /// 图文消息的信息
    pub news_info: Option<SelfMenuNewsButton>,
    /// 二级菜单数组，个数应为1~5个
    pub sub_button: Option<SelfMenuSubButton>,

}

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct SelfMenuSubButton {
    pub list: Option<Vec<SelfMenuButton>>,
}

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct SelfMenuNewsButton {
    pub list: Option<Vec<SelfMenuNewsInfo>>,
}

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct SelfMenuNewsInfo {
    /// 图文消息的标题
    pub title: Option<String>,
    /// 摘要
    pub digest: Option<String>,
    /// 作者
    pub author: Option<String>,
    /// 是否显示封面，0为不显示，1为显示
    pub show_cover: Option<u8>,
    /// 封面图片的URL
    pub cover_url: Option<String>,
    /// 正文的URL
    pub content_url: Option<String>,
    /// 原文的URL，若置空则无查看原文入口
    pub source_url: Option<String>,
}



/// 菜单按钮类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ButtonType {
    /// 点击推事件
    Click,
    /// 跳转URL
    View,
    /// 扫码推事件
    ScanCodePush,
    /// 扫码推事件且弹出"消息接收中"提示框
    ScanCodeWaitMsg,
    /// 弹出系统拍照发图
    PicSysPhoto,
    /// 弹出拍照或者相册发图
    PicPhotoOrAlbum,
    /// 弹出微信相册发图器
    PicWeixin,
    /// 弹出地理位置选择器
    LocationSelect,
    /// 下发消息（除文本消息）
    MediaId,
    /// 跳转图文消息URL
    ViewLimited,
    /// 小程序
    Miniprogram,
}

impl ButtonType {
    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonType::Click => "click",
            ButtonType::View => "view",
            ButtonType::ScanCodePush => "scancode_push",
            ButtonType::ScanCodeWaitMsg => "scancode_waitmsg",
            ButtonType::PicSysPhoto => "pic_sysphoto",
            ButtonType::PicPhotoOrAlbum => "pic_photo_or_album",
            ButtonType::PicWeixin => "pic_weixin",
            ButtonType::LocationSelect => "location_select",
            ButtonType::MediaId => "media_id",
            ButtonType::ViewLimited => "view_limited",
            ButtonType::Miniprogram => "miniprogram",
        }
    }
}

/// 菜单按钮
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Button {
    /// 菜单的响应动作类型
    #[serde(rename = "type")]
    pub button_type: Option<ButtonType>,
    /// 菜单标题，不超过16个字节
    pub name: String,
    /// 菜单KEY值，用于消息接口推送，不超过128字节
    pub key: Option<String>,
    /// 网页链接，用户点击菜单可打开链接，不超过1024字节
    pub url: Option<String>,
    /// 调用新增永久素材接口返回的合法media_id
    pub media_id: Option<String>,
    /// 小程序的appid
    pub appid: Option<String>,
    /// 小程序的页面路径
    pub pagepath: Option<String>,
    /// 二级菜单数组，个数应为1~5个
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_button: Option<Vec<Button>>,
}

impl Button {
    /// 创建点击按钮
    pub fn click(name: &str, key: &str) -> Self {
        Self {
            button_type: Some(ButtonType::Click),
            name: name.to_string(),
            key: Some(key.to_string()),
            url: None,
            media_id: None,
            appid: None,
            pagepath: None,
            sub_button: None,
        }
    }

    /// 创建跳转按钮
    pub fn view(name: &str, url: &str) -> Self {
        Self {
            button_type: Some(ButtonType::View),
            name: name.to_string(),
            key: None,
            url: Some(url.to_string()),
            media_id: None,
            appid: None,
            pagepath: None,
            sub_button: None,
        }
    }

    /// 创建小程序按钮
    pub fn miniprogram(name: &str, url: &str, appid: &str, pagepath: &str) -> Self {
        Self {
            button_type: Some(ButtonType::Miniprogram),
            name: name.to_string(),
            key: None,
            url: Some(url.to_string()),
            media_id: None,
            appid: Some(appid.to_string()),
            pagepath: Some(pagepath.to_string()),
            sub_button: None,
        }
    }

    /// 创建父菜单按钮
    pub fn parent(name: &str, sub_buttons: Vec<Button>) -> Self {
        Self {
            button_type: None,
            name: name.to_string(),
            key: None,
            url: None,
            media_id: None,
            appid: None,
            pagepath: None,
            sub_button: Some(sub_buttons),
        }
    }
}

/// 菜单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Menu {
    /// 菜单按钮
    pub button: Vec<Button>,
}

impl Menu {
    /// 创建新的菜单
    pub fn new(buttons: Vec<Button>) -> Self {
        Self { button: buttons }
    }
}

/// 菜单响应
#[derive(Debug, Clone, Deserialize)]
pub struct MenuResponse {
    /// 菜单
    pub menu: Menu,
    /// 个性化菜单
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditionalmenu: Option<Vec<ConditionalMenu>>,
}

/// 个性化菜单
#[derive(Debug, Clone, Deserialize)]
pub struct ConditionalMenu {
    /// 菜单按钮
    pub button: Vec<Button>,
    /// 菜单匹配规则
    pub matchrule: MatchRule,
    /// 菜单ID
    pub menuid: i64,
}

/// 菜单匹配规则
#[derive(Debug, Clone, Deserialize)]
pub struct MatchRule {
    /// 用户标签的id，可通过用户标签管理接口获取
    pub tag_id: Option<String>,
    /// 性别：男（1）女（2），不填则不做匹配
    pub sex: Option<String>,
    /// 国家信息，是用户在微信中设置的地区
    pub country: Option<String>,
    /// 省份信息，是用户在微信中设置的地区
    pub province: Option<String>,
    /// 城市信息，是用户在微信中设置的地区
    pub city: Option<String>,
    /// 客户端版本，当前只具体到系统型号：IOS(1), Android(2),Others(3)，不填则不做匹配
    pub client_platform_type: Option<String>,
    /// 语言信息，是用户在微信中设置的语言
    pub language: Option<String>,
}
