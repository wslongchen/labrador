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
    /// 卡券已过期
    CardReceiveExpired = 45053,
    /// 卡券已失效
    CardReceiveInvalid = 45054,
    /// 卡券已删除
    CardReceiveDeleted = 45055,
    /// 卡券已使用
    CardReceiveUsed = 45056,
    /// 卡券已转赠
    CardReceiveTransferred = 45057,
    /// 卡券已核销
    CardReceiveConsumed = 45058,
    /// 卡券已过期
    CardReceiveExpired2 = 45059,
    /// 卡券已失效
    CardReceiveInvalid2 = 45060,
    /// 卡券已删除
    CardReceiveDeleted2 = 45061,
    /// 卡券已使用
    CardReceiveUsed2 = 45062,
    /// 卡券已转赠
    CardReceiveTransferred2 = 45063,
    /// 卡券已核销
    CardReceiveConsumed2 = 45064,
    /// 卡券已过期
    CardReceiveExpired3 = 45065,
    /// 卡券已失效
    CardReceiveInvalid3 = 45066,
    /// 卡券已删除
    CardReceiveDeleted3 = 45067,
    /// 卡券已使用
    CardReceiveUsed3 = 45068,
    /// 卡券已转赠
    CardReceiveTransferred3 = 45069,
    /// 卡券已核销
    CardReceiveConsumed3 = 45070,
    /// 卡券已过期
    CardReceiveExpired4 = 45071,
    /// 卡券已失效
    CardReceiveInvalid4 = 45072,
    /// 卡券已删除
    CardReceiveDeleted4 = 45073,
    /// 卡券已使用
    CardReceiveUsed4 = 45074,
    /// 卡券已转赠
    CardReceiveTransferred4 = 45075,
    /// 卡券已核销
    CardReceiveConsumed4 = 45076,
    /// 卡券已过期
    CardReceiveExpired5 = 45077,
    /// 卡券已失效
    CardReceiveInvalid5 = 45078,
    /// 卡券已删除
    CardReceiveDeleted5 = 45079,
    /// 卡券已使用
    CardReceiveUsed5 = 45080,
    /// 卡券已转赠
    CardReceiveTransferred5 = 45081,
    /// 卡券已核销
    CardReceiveConsumed5 = 45082,
    /// 卡券已过期
    CardReceiveExpired6 = 45083,
    /// 卡券已失效
    CardReceiveInvalid6 = 45084,
    /// 卡券已删除
    CardReceiveDeleted6 = 45085,
    /// 卡券已使用
    CardReceiveUsed6 = 45086,
    /// 卡券已转赠
    CardReceiveTransferred6 = 45087,
    /// 卡券已核销
    CardReceiveConsumed6 = 45088,
    /// 卡券已过期
    CardReceiveExpired7 = 45089,
    /// 卡券已失效
    CardReceiveInvalid7 = 45090,
    /// 卡券已删除
    CardReceiveDeleted7 = 45091,
    /// 卡券已使用
    CardReceiveUsed7 = 45092,
    /// 卡券已转赠
    CardReceiveTransferred7 = 45093,
    /// 卡券已核销
    CardReceiveConsumed7 = 45094,
    /// 卡券已过期
    CardReceiveExpired8 = 45095,
    /// 卡券已失效
    CardReceiveInvalid8 = 45096,
    /// 卡券已删除
    CardReceiveDeleted8 = 45097,
    /// 卡券已使用
    CardReceiveUsed8 = 45098,
    /// 卡券已转赠
    CardReceiveTransferred8 = 45099,
    /// 卡券已核销
    CardReceiveConsumed8 = 45100,
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
            45053 => Self::CardReceiveExpired,
            45054 => Self::CardReceiveInvalid,
            45055 => Self::CardReceiveDeleted,
            45056 => Self::CardReceiveUsed,
            45057 => Self::CardReceiveTransferred,
            45058 => Self::CardReceiveConsumed,
            45059 => Self::CardReceiveExpired2,
            45060 => Self::CardReceiveInvalid2,
            45061 => Self::CardReceiveDeleted2,
            45062 => Self::CardReceiveUsed2,
            45063 => Self::CardReceiveTransferred2,
            45064 => Self::CardReceiveConsumed2,
            45065 => Self::CardReceiveExpired3,
            45066 => Self::CardReceiveInvalid3,
            45067 => Self::CardReceiveDeleted3,
            45068 => Self::CardReceiveUsed3,
            45069 => Self::CardReceiveTransferred3,
            45070 => Self::CardReceiveConsumed3,
            45071 => Self::CardReceiveExpired4,
            45072 => Self::CardReceiveInvalid4,
            45073 => Self::CardReceiveDeleted4,
            45074 => Self::CardReceiveUsed4,
            45075 => Self::CardReceiveTransferred4,
            45076 => Self::CardReceiveConsumed4,
            45077 => Self::CardReceiveExpired5,
            45078 => Self::CardReceiveInvalid5,
            45079 => Self::CardReceiveDeleted5,
            45080 => Self::CardReceiveUsed5,
            45081 => Self::CardReceiveTransferred5,
            45082 => Self::CardReceiveConsumed5,
            45083 => Self::CardReceiveExpired6,
            45084 => Self::CardReceiveInvalid6,
            45085 => Self::CardReceiveDeleted6,
            45086 => Self::CardReceiveUsed6,
            45087 => Self::CardReceiveTransferred6,
            45088 => Self::CardReceiveConsumed6,
            45089 => Self::CardReceiveExpired7,
            45090 => Self::CardReceiveInvalid7,
            45091 => Self::CardReceiveDeleted7,
            45092 => Self::CardReceiveUsed7,
            45093 => Self::CardReceiveTransferred7,
            45094 => Self::CardReceiveConsumed7,
            45095 => Self::CardReceiveExpired8,
            45096 => Self::CardReceiveInvalid8,
            45097 => Self::CardReceiveDeleted8,
            45098 => Self::CardReceiveUsed8,
            45099 => Self::CardReceiveTransferred8,
            45100 => Self::CardReceiveConsumed8,
            _ => Self::Success,
        }
    }
}
