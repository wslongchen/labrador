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

//! 微信开放平台实现
//!
//! 提供微信开放平台（open.weixin.qq.com）相关接口，包括：
//! - **第三方平台(component)**: 代开发小程序/公众号，管理 component_access_token、预授权码、授权方令牌等
//! - **网站应用(web)**: 网站应用扫码登录 OAuth2 流程
//!
//! ## 使用示例
//!
//! ```rust,ignore
//! use labrador::wechat::open::builder::WechatOpenBuilder;
//!
//! let client = WechatOpenBuilder::new("app_id", "app_secret")
//!     .encoding_aes_key("your_aes_key")
//!     .token("your_token")
//!     .build()
//!     .unwrap();
//!
//! // 网站应用扫码登录
//! let web = client.web();
//! let login_url = web.generate_login_url("https://example.com/callback", "random_state");
//!
//! // 第三方平台代开发
//! let component = client.component();
//! let token = component.get_component_access_token("verify_ticket").await?;
//! ```

pub mod api;
pub mod builder;
pub mod config;
pub mod types;

use super::client::{WechatClient, WechatClientConfig};
use crate::errors::LabradorResult;
use crate::platforms::wechat::open::api::component::WechatOpenComponent;
use crate::platforms::wechat::open::api::web::WechatOpenWeb;
use crate::platforms::wechat::open::config::WechatOpenConfig;

/// 微信开放平台客户端
///
/// 提供微信开放平台（open.weixin.qq.com）的统一客户端，
/// 支持第三方平台代开发和网站应用扫码登录。
pub struct WechatOpenClient {
    /// 微信客户端
    wechat_client: WechatClient,
    /// 配置
    config: WechatOpenConfig,
}

impl std::fmt::Debug for WechatOpenClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WechatOpenClient")
            .field("config", &self.config)
            .finish()
    }
}

impl WechatOpenClient {
    /// 创建新的微信开放平台客户端
    pub fn new(config: WechatOpenConfig) -> LabradorResult<Self> {
        let wechat_client_config = WechatClientConfig {
            app_id: config.app_id.clone(),
            app_secret: config.app_secret.clone(),
            sandbox: config.sandbox,
            ..WechatClientConfig::default()
        };

        let wechat_client = WechatClient::new(wechat_client_config)?;

        Ok(Self {
            wechat_client,
            config,
        })
    }

    /// 获取微信客户端
    pub fn wechat_client(&self) -> &WechatClient {
        &self.wechat_client
    }

    /// 获取第三方平台服务
    ///
    /// 提供第三方平台代开发相关接口：
    /// - component_access_token 管理
    /// - 预授权码获取
    /// - 授权方令牌管理
    /// - 授权方信息查询
    pub fn component(&self) -> WechatOpenComponent<'_> {
        WechatOpenComponent::new(self)
    }

    /// 获取网站应用服务
    ///
    /// 提供网站应用扫码登录相关接口：
    /// - 生成扫码登录 URL
    /// - OAuth2 授权流程
    /// - 获取用户信息
    /// - 刷新/校验令牌
    pub fn web(&self) -> WechatOpenWeb<'_> {
        WechatOpenWeb::new(self)
    }

    /// 获取配置
    pub fn config(&self) -> &WechatOpenConfig {
        &self.config
    }
}
