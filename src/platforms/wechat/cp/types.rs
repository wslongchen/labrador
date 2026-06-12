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
use serde::{Deserialize, Serialize};

/**
 * 不弹出授权页面，直接跳转，只能获取用户openid.
 */
pub static SNSAPI_BASE: &str = "snsapi_base";

/**
 * 弹出授权页面，可通过openid拿到昵称、性别、所在地。并且，即使在未关注的情况下，只要用户授权，也能获取其信息.
 */
pub static SNSAPI_USERINFO: &str = "snsapi_userinfo";

/**
 * 手动授权,可获取成员的详细信息,包含手机、邮箱。只适用于企业微信或企业号.
 */
pub static SNSAPI_PRIVATEINFO: &str = "snsapi_privateinfo";

/**
 * 图片消息.
 */
pub static WELCOME_MSG_TYPE_IMAGE: &str = "image";
/**
 * 图文消息.
 */
pub static WELCOME_MSG_TYPE_LINK: &str = "link";
/**
 * 视频消息.
 */
pub static WELCOME_MSG_TYPE_VIDEO: &str = "video";
/**
 * 小程序消息.
 */
pub static WELCOME_MSG_TYPE_MINIPROGRAM: &str = "miniprogram";

/**
 * 文件消息.
 */
pub static WELCOME_MSG_TYPE_FILE: &str = "file";

/**
 * 文本消息.
 */
pub static GROUP_ROBOT_MSG_TEXT: &str = "text";

/**
 * 图片消息.
 */
pub static GROUP_ROBOT_MSG_IMAGE: &str = "image";

/**
 * markdown消息.
 */
pub static GROUP_ROBOT_MSG_MARKDOWN: &str = "markdown";

/**
 * 图文消息（点击跳转到外链）.
 */
pub static GROUP_ROBOT_MSG_NEWS: &str = "news";

/**
 * 文件类型消息.
 */
pub static GROUP_ROBOT_MSG_FILE: &str = "file";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WechatCpJsCodeSession {
    /// 企业编号
    pub corpid: String,
    /// 会话密钥
    pub session_key: String,
    pub userid: Option<String>,
}

#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct WechatCpProviderToken {
    /// 服务商的access_token，最长为512字节。
    pub provider_access_token: String,
    /// provider_access_token有效期（秒）
    pub expires_in: i64,
}
