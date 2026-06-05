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
//! 微信平台实现

/// 微信客户端
pub mod client;
/// 微信支付
pub mod pay;
/// 微信公众号
pub mod mp;
/// 微信小程序
pub mod miniapp;
pub mod constants;
pub mod message_crypto;
/// 企业微信
pub mod cp;
pub mod signer;

/// 微信错误码
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WechatErrorCode {
    /// 成功
    Success = 0,
    /// 系统繁忙
    SystemBusy = -1,
    /// 无效的AppID
    InvalidAppId = 40013,
    /// 无效的AccessToken
    InvalidAccessToken = 40001,
    /// AccessToken过期
    AccessTokenExpired = 42001,
    /// 无效的OpenID
    InvalidOpenId = 40003,
    /// 缺少访问令牌
    MissingAccessToken = 41001,
    /// 无效的凭证类型
    InvalidCredentialType = 40002,
    /// 不支持的图片格式
    UnsupportedImageFormat = 40009,
    /// 音频文件大小超过限制
    AudioFileTooLarge = 40010,
    /// 无效的媒体文件大小
    InvalidMediaFileSize = 40011,
    /// 无效的图片文件大小
    InvalidImageFileSize = 40012,
    /// 无效的消息类型
    InvalidMessageType = 40014,
    /// 无效的按钮个数
    InvalidButtonCount = 40015,
    /// 无效的按钮类型
    InvalidButtonType = 40016,
    /// 无效的按钮名称长度
    InvalidButtonNameLength = 40017,
    /// 无效的按钮KEY长度
    InvalidButtonKeyLength = 40018,
    /// 无效的按钮URL长度
    InvalidButtonUrlLength = 40019,
    /// 无效的子菜单个数
    InvalidSubMenuCount = 40020,
    /// 无效的子菜单类型
    InvalidSubMenuType = 40021,
    /// 无效的子菜单名称长度
    InvalidSubMenuNameLength = 40022,
    /// 无效的子菜单KEY长度
    InvalidSubMenuKeyLength = 40023,
    /// 无效的子菜单URL长度
    InvalidSubMenuUrlLength = 40024,
    /// 无效的自定义菜单版本
    InvalidMenuVersion = 40025,
    /// 无效的OAuth2.0 Code
    InvalidOAuthCode = 40029,
    /// 刷新令牌过期
    RefreshTokenExpired = 40030,
    /// 无效的刷新令牌
    InvalidRefreshToken = 40031,
    /// 无效的OpenID列表
    InvalidOpenIdList = 40032,
    /// 无效的OpenID列表长度
    InvalidOpenIdListLength = 40033,
    /// 无效的请求字符
    InvalidRequestChars = 40035,
    /// 无效的参数
    InvalidParameter = 40038,
    /// 无效的请求格式
    InvalidRequestFormat = 40048,
    /// 无效的URL长度
    InvalidUrlLength = 40049,
    /// 无效的分组ID
    InvalidGroupId = 40050,
    /// 分组名称不合法
    InvalidGroupName = 40051,
    /// 无效的媒体文件ID
    InvalidMediaId = 40007,
    /// 无效的消息ID
    InvalidMessageId = 45015,
    /// 接口调用超过限制
    ApiLimitExceeded = 45016,
    /// 建立菜单被限制
    MenuCreationLimited = 45017,
    /// 修改菜单过于频繁
    MenuModificationTooFrequent = 45018,
    /// 用户拒绝授权
    UserDeniedAuthorization = 45019,
    /// 无效的JSON格式
    InvalidJsonFormat = 45023,
    /// 无效的模板ID
    InvalidTemplateId = 45024,
    /// 模板消息已禁用
    TemplateMessageDisabled = 45026,
    /// 无效的模板参数
    InvalidTemplateParameter = 45027,
    /// 模板消息发送过于频繁
    TemplateMessageTooFrequent = 45028,
    /// 系统错误
    SystemError = 45029,
    /// 无效的日期格式
    InvalidDateFormat = 45030,
    /// 无效的文章数量
    InvalidArticleCount = 45031,
    /// 无效的文章内容
    InvalidArticleContent = 45032,
    /// 无效的标题长度
    InvalidTitleLength = 45033,
    /// 无效的作者长度
    InvalidAuthorLength = 45034,
    /// 无效的摘要长度
    InvalidDigestLength = 45035,
    /// 无效的内容URL
    InvalidContentUrl = 45036,
    /// 无效的图片URL
    InvalidImageUrl = 45037,
    /// 无效的音频URL
    InvalidAudioUrl = 45038,
    /// 无效的缩略图媒体ID
    InvalidThumbMediaId = 45039,
    /// 无效的图文消息内容
    InvalidNewsContent = 45040,
    /// 无效的视频描述信息
    InvalidVideoDescription = 45041,
    /// 无效的音乐描述信息
    InvalidMusicDescription = 45042,
    /// 无效的卡券ID
    InvalidCardId = 45043,
    /// 无效的卡券扩展信息
    InvalidCardExt = 45044,
    /// 无效的卡券状态
    InvalidCardStatus = 45045,
    /// 卡券已过期
    CardExpired = 45046,
    /// 卡券已使用
    CardUsed = 45047,
    /// 卡券已删除
    CardDeleted = 45048,
    /// 卡券已失效
    CardInvalid = 45049,
    /// 卡券库存不足
    CardStockInsufficient = 45050,
    /// 卡券领取次数超过限制
    CardReceiveLimitExceeded = 45051,
    /// 卡券已领取完
    CardReceiveFinished = 45052,
    /// 其他错误码（未知错误码）
    Other(i32),
}

impl From<i32> for WechatErrorCode {
    fn from(code: i32) -> Self {
        match code {
            0 => Self::Success,
            -1 => Self::SystemBusy,
            40013 => Self::InvalidAppId,
            40001 => Self::InvalidAccessToken,
            42001 => Self::AccessTokenExpired,
            40003 => Self::InvalidOpenId,
            41001 => Self::MissingAccessToken,
            40002 => Self::InvalidCredentialType,
            40009 => Self::UnsupportedImageFormat,
            40010 => Self::AudioFileTooLarge,
            40011 => Self::InvalidMediaFileSize,
            40012 => Self::InvalidImageFileSize,
            40014 => Self::InvalidMessageType,
            40015 => Self::InvalidButtonCount,
            40016 => Self::InvalidButtonType,
            40017 => Self::InvalidButtonNameLength,
            40018 => Self::InvalidButtonKeyLength,
            40019 => Self::InvalidButtonUrlLength,
            40020 => Self::InvalidSubMenuCount,
            40021 => Self::InvalidSubMenuType,
            40022 => Self::InvalidSubMenuNameLength,
            40023 => Self::InvalidSubMenuKeyLength,
            40024 => Self::InvalidSubMenuUrlLength,
            40025 => Self::InvalidMenuVersion,
            40029 => Self::InvalidOAuthCode,
            40030 => Self::RefreshTokenExpired,
            40031 => Self::InvalidRefreshToken,
            40032 => Self::InvalidOpenIdList,
            40033 => Self::InvalidOpenIdListLength,
            40035 => Self::InvalidRequestChars,
            40038 => Self::InvalidParameter,
            40048 => Self::InvalidRequestFormat,
            40049 => Self::InvalidUrlLength,
            40050 => Self::InvalidGroupId,
            40051 => Self::InvalidGroupName,
            40007 => Self::InvalidMediaId,
            45015 => Self::InvalidMessageId,
            45016 => Self::ApiLimitExceeded,
            45017 => Self::MenuCreationLimited,
            45018 => Self::MenuModificationTooFrequent,
            45019 => Self::UserDeniedAuthorization,
            45023 => Self::InvalidJsonFormat,
            45024 => Self::InvalidTemplateId,
            45026 => Self::TemplateMessageDisabled,
            45027 => Self::InvalidTemplateParameter,
            45028 => Self::TemplateMessageTooFrequent,
            45029 => Self::SystemError,
            45030 => Self::InvalidDateFormat,
            45031 => Self::InvalidArticleCount,
            45032 => Self::InvalidArticleContent,
            45033 => Self::InvalidTitleLength,
            45034 => Self::InvalidAuthorLength,
            45035 => Self::InvalidDigestLength,
            45036 => Self::InvalidContentUrl,
            45037 => Self::InvalidImageUrl,
            45038 => Self::InvalidAudioUrl,
            45039 => Self::InvalidThumbMediaId,
            45040 => Self::InvalidNewsContent,
            45041 => Self::InvalidVideoDescription,
            45042 => Self::InvalidMusicDescription,
            45043 => Self::InvalidCardId,
            45044 => Self::InvalidCardExt,
            45045 => Self::InvalidCardStatus,
            45046 => Self::CardExpired,
            45047 => Self::CardUsed,
            45048 => Self::CardDeleted,
            45049 => Self::CardInvalid,
            45050 => Self::CardStockInsufficient,
            45051 => Self::CardReceiveLimitExceeded,
            45052 => Self::CardReceiveFinished,
            _ => Self::Other(code),
        }
    }
}
