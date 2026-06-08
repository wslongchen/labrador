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
use serde::{Deserialize, Serialize};

use crate::errors::{LabraError, LabradorResult};
use crate::wechat::client::WechatApiResponse;
use crate::wechat::cp::WechatCpClient;

/// 企业微信菜单管理模块
#[derive(Debug, Clone)]
pub struct WechatCpMenu<'a> {
    client: &'a WechatCpClient,
}

#[allow(unused)]
impl<'a> WechatCpMenu<'a> {
    #[inline]
    pub fn new(client: &'a WechatCpClient) -> Self {
        Self { client }
    }

    /// 创建菜单（使用默认应用ID）
    ///
    /// 使用客户端配置的agent_id创建菜单。
    pub async fn create(&self, menu: MenuInfo) -> LabradorResult<WechatApiResponse> {
        let agent_id = self
            .client
            .agent_id()
            .ok_or_else(|| LabraError::Config("agent_id未配置".to_string()))?;
        self.create_with_agentid(agent_id, menu).await
    }

    /// 创建菜单（指定应用ID）
    ///
    /// 为指定应用创建自定义菜单。
    pub async fn create_with_agentid(
        &self,
        agent_id: i32,
        menu: MenuInfo,
    ) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .post(&format!("/cgi-bin/menu/create?agentid={}", agent_id), menu)
            .await?;
        Ok(response)
    }

    /// 删除菜单（使用默认应用ID）
    ///
    /// 使用客户端配置的agent_id删除菜单。
    pub async fn delete(&self) -> LabradorResult<WechatApiResponse> {
        let agent_id = self
            .client
            .agent_id()
            .ok_or_else(|| LabraError::Config("agent_id未配置".to_string()))?;
        self.delete_with_agentid(agent_id).await
    }

    /// 删除菜单（指定应用ID）
    ///
    /// 删除指定应用的自定义菜单。
    pub async fn delete_with_agentid(&self, agent_id: i32) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .get(&format!("/cgi-bin/menu/delete?agentid={}", agent_id))
            .await?;
        Ok(response)
    }

    /// 获取菜单（使用默认应用ID）
    ///
    /// 使用客户端配置的agent_id获取菜单配置。
    pub async fn get(&self) -> LabradorResult<MenuInfo> {
        let agent_id = self
            .client
            .agent_id()
            .ok_or_else(|| LabraError::Config("agent_id未配置".to_string()))?;
        self.get_with_agentid(agent_id).await
    }

    /// 获取菜单（指定应用ID）
    ///
    /// 获取指定应用的菜单配置。
    pub async fn get_with_agentid(&self, agent_id: i32) -> LabradorResult<MenuInfo> {
        let response: WechatApiResponse<MenuInfo> = self
            .client
            .get(&format!("/cgi-bin/menu/get?agentid={}", agent_id))
            .await?;
        response.into_result()
    }
}

// ============================================================================
// 请求与响应结构体
// ============================================================================

/// 菜单信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuInfo {
    /// 菜单按钮列表
    pub button: Vec<MenuButton>,
    /// 菜单匹配规则（个性化菜单时使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_rule: Option<MenuRule>,
}

/// 菜单按钮
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuButton {
    /// 菜单的响应动作类型：
    /// - view：网页类型
    /// - click：点击类型
    /// - miniprogram：小程序类型
    /// - media_id：弹出素材
    /// - article_id：弹出图文
    /// - article_view_limited：查看图文
    #[serde(rename = "type")]
    pub button_type: String,
    /// 菜单标题，不超过16个字节，子菜单不超过60个字节
    pub name: String,
    /// 菜单KEY值，用于消息接口推送，不超过128字节（click等点击类型必须）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 网页链接，不超过1024字节（view、miniprogram类型必须）
    /// type为miniprogram时，不支持小程序的老版本客户端将打开本url
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 永久素材的media_id（media_id和view_limited类型必须）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_id: Option<String>,
    /// 发布图文获得的article_id（article_id和article_view_limited类型必须）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub article_id: Option<String>,
    /// 小程序的appid（miniprogram类型必须）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,
    /// 小程序的页面路径（miniprogram类型必须）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagepath: Option<String>,
    /// 二级菜单列表
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sub_button: Vec<MenuButton>,
}

/// 菜单匹配规则（用于个性化菜单）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuRule {
    /// 用户标签ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<String>,
    /// 性别：0-不限，1-男，2-女
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<String>,
    /// 省份
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    /// 城市
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// 客户端平台类型：1-PC，2-移动端
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_platform_type: Option<String>,
    /// 语言
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}
