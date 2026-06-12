/*
 *
 *  *
 *  *      Copyright (c) 2018-2025, WoofCloud All rights reserved.
 *  *
 *  *   Redistribution and use in source and binary forms, with or without
 *  *   modification, are permitted provided that the following conditions are met:
 *  *
 *  *   Redistributions of source code must retain the above copyright notice,
 *  *   this list of conditions and the following disclaimer.
 *  *   Redistributions in binary form must reproduce the above copyright
 *  *   notice, this list of conditions and the following disclaimer in the
 *  *   documentation and/or other materials provided with the distribution.
 *  *   Neither the name of the www.woofcloud.com developer nor the names of its
 *  *   contributors may be used to endorse or promote products derived from
 *  *   this software without specific prior written permission.
 *  *   Author: WoofCloud
 *  *
 *
 */

/// ticket类型
pub static TICKET_TYPE_JSAPI: &str = "jsapi";
pub static TICKET_TYPE: &str = "type";
pub static TICKET_TYPE_SDK: &str = "2";
pub static TICKET_TYPE_WXCARD: &str = "wx_card";
pub static MEMBER_CARD: &str = "MEMBER_CARD";
pub static QR_SCENE: &str = "QR_SCENE";
pub static QR_STR_SCENE: &str = "QR_STR_SCENE";
pub static QR_CODE: &str = "QR_CODE";
pub static QR_LIMIT_SCENE: &str = "QR_LIMIT_SCENE";
pub static QR_LIMIT_STR_SCENE: &str = "QR_LIMIT_STR_SCENE";
pub static IMG_URL: &str = "img_url";

pub static MATERIAL_TYPE_NEWS: &str = "news";
pub static MATERIAL_TYPE_VOICE: &str = "voice";
pub static MATERIAL_TYPE_IMAGE: &str = "image";
pub static MATERIAL_TYPE_VIDEO: &str = "video";
pub static GRANT_TYPE: &str = "grant_type";
pub static CODE: &str = "code";
pub static CLIENT_CREDENTIAL: &str = "client_credential";
pub static APPID: &str = "appid";
pub static OPENID: &str = "openid";
pub static LANG: &str = "lang";
pub static ZH_CN: &str = "zh_CN";
pub static SECRET: &str = "secret";
pub static ACCESS_TOKEN: &str = "access_token";
pub static REFRESH_TOKEN: &str = "refresh_token";

pub enum TicketType {
    /// jsapi
    JSAPI,
    /// sdk
    SDK,
    /// 微信卡券
    WxCard,
}

impl std::fmt::Display for TicketType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TicketType::JSAPI => write!(f, "{}", TICKET_TYPE_JSAPI),
            TicketType::SDK => write!(f, "{}", TICKET_TYPE_SDK),
            TicketType::WxCard => write!(f, "{}", TICKET_TYPE_WXCARD),
        }
    }
}
