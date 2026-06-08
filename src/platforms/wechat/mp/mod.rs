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
//! 微信公众号实现

pub mod api;
pub mod builder;
pub mod callback;
pub mod config;
pub mod types;

use super::client::{JsSdkConfig, JsSdkSigner, WechatClient, WechatClientConfig};
use crate::errors::LabradorResult;
use crate::platforms::wechat::mp::config::WechatMpConfig;
use crate::wechat::mp::api::{
    WechatMpAI, WechatMpApiManage, WechatMpAutoReply, WechatMpCard, WechatMpComment,
    WechatMpCustomService, WechatMpFreePublish, WechatMpImage, WechatMpInvoice,
    WechatMpMassMessage, WechatMpMedia, WechatMpMedicalAssistant, WechatMpMember, WechatMpMenu,
    WechatMpOauth2, WechatMpOcr, WechatMpOneCode, WechatMpStore, WechatMpSubscribeMessage,
    WechatMpTemplateMessage, WechatMpUser,
};

/// 微信公众号客户端
pub struct WechatMpClient {
    /// 微信客户端
    wechat_client: WechatClient,
    /// 配置
    config: WechatMpConfig,
    /// JS-SDK签名器
    js_sdk_signer: JsSdkSigner,
}

impl std::fmt::Debug for WechatMpClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WechatMpClient")
            .field("config", &self.config)
            .field("wechat_client", &self.wechat_client)
            .finish()
    }
}

impl WechatMpClient {
    /// 创建新的微信公众号客户端
    pub fn new(config: WechatMpConfig) -> LabradorResult<Self> {
        let wechat_client_config = WechatClientConfig {
            app_id: config.app_id.clone(),
            app_secret: config.app_secret.clone(),
            sandbox: config.sandbox,
            ..WechatClientConfig::default()
        };

        let wechat_client = WechatClient::new(wechat_client_config)?;
        let js_sdk_signer = JsSdkSigner::new(&config.app_id, &config.app_secret);

        Ok(Self {
            wechat_client,
            config,
            js_sdk_signer,
        })
    }

    /// 获取微信客户端
    pub fn wechat_client(&self) -> &WechatClient {
        &self.wechat_client
    }

    pub fn user(&self) -> WechatMpUser<'_> {
        WechatMpUser::new(self)
    }

    /// 获取客户服务
    pub fn customer_service(&self) -> WechatMpCustomService<'_> {
        WechatMpCustomService::new(self)
    }

    /// 获取会员卡券
    pub fn card(&self) -> WechatMpCard<'_> {
        WechatMpCard::new(self)
    }

    /// 获取媒体
    pub fn media(&self) -> WechatMpMedia<'_> {
        WechatMpMedia::new(self)
    }

    /// 获取会员
    pub fn member(&self) -> WechatMpMember<'_> {
        WechatMpMember::new(self)
    }

    /// 获取菜单
    pub fn menu(&self) -> WechatMpMenu<'_> {
        WechatMpMenu::new(self)
    }

    /// 获取OAuth2
    pub fn oauth2(&self) -> WechatMpOauth2<'_> {
        WechatMpOauth2::new(self)
    }

    /// 获取OCR
    pub fn orc(&self) -> WechatMpOcr<'_> {
        WechatMpOcr::new(self)
    }

    /// 获取订阅消息
    pub fn subscribe_msg(&self) -> WechatMpSubscribeMessage<'_> {
        WechatMpSubscribeMessage::new(self)
    }

    /// 获取模板消息
    pub fn template_msg(&self) -> WechatMpTemplateMessage<'_> {
        WechatMpTemplateMessage::new(self)
    }
    /// 群发消息
    pub fn mass_message(&self) -> WechatMpMassMessage<'_> {
        WechatMpMassMessage::new(self)
    }

    /// 获取自动回复
    pub fn autoreply(&self) -> WechatMpAutoReply<'_> {
        WechatMpAutoReply::new(self)
    }

    /// 获取发布
    pub fn publish(&self) -> WechatMpFreePublish<'_> {
        WechatMpFreePublish::new(self)
    }

    /// 获取门店
    pub fn store(&self) -> WechatMpStore<'_> {
        WechatMpStore::new(self)
    }

    /// 获取发票
    pub fn invoice(&self) -> WechatMpInvoice<'_> {
        WechatMpInvoice::new(self)
    }

    /// 获取评论
    pub fn comment(&self) -> WechatMpComment<'_> {
        WechatMpComment::new(self)
    }

    /// 获取AI
    pub fn ai(&self) -> WechatMpAI<'_> {
        WechatMpAI::new(self)
    }

    /// 获取图片
    pub fn image(&self) -> WechatMpImage<'_> {
        WechatMpImage::new(self)
    }

    /// 获取API管理
    pub fn api_manage(&self) -> WechatMpApiManage<'_> {
        WechatMpApiManage::new(self)
    }

    /// 获取一物一码
    pub fn onecode(&self) -> WechatMpOneCode<'_> {
        WechatMpOneCode::new(self)
    }

    /// 获取医疗助手
    pub fn medical(&self) -> WechatMpMedicalAssistant<'_> {
        WechatMpMedicalAssistant::new(self)
    }

    /// 生成JS-SDK配置
    pub async fn generate_js_sdk_config(
        &self,
        url: &str,
        js_api_list: Vec<String>,
        debug: bool,
    ) -> LabradorResult<JsSdkConfig> {
        self.js_sdk_signer
            .generate_config(&self.wechat_client, url, js_api_list, debug)
            .await
    }

    /// 获取JS-SDK的Ticket
    pub async fn get_jsapi_ticket(&self) -> LabradorResult<String> {
        self.js_sdk_signer
            .get_jsapi_ticket(&self.wechat_client)
            .await
    }

    /// 获取配置
    pub fn config(&self) -> &WechatMpConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use crate::errors::LabradorResult;
    use crate::platforms::wechat::mp::WechatMpClient;
    use crate::wechat::mp::builder::WechatMpBuilder;

    fn create_client() -> WechatMpClient {
        let client = WechatMpBuilder::new("wxd17fc52706acfe11", "c16c016f3415fdec7f9fd32a87d398d3")
            .encoding_aes_key("f72610382c5fb11983d024356396d9d7")
            .token("dogcat")
            .build()
            .unwrap();
        client
    }

    #[tokio::test]
    async fn test_get_jsapi_ticket() {
        let client = create_client();
        let result: LabradorResult<String> = client.get_jsapi_ticket().await;
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_user() {
        let client = create_client();
        let user = client.user();
        let info = user.get("oUVZc6S_uGx3bsNPUA-davo4Dt7U").await.unwrap();
        println!("{:?}", info);
    }
}
