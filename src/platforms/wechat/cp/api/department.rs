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

use crate::errors::LabradorResult;
use crate::wechat::client::WechatApiResponse;
use crate::wechat::cp::WechatCpClient;

/// 企业微信部门管理模块
#[derive(Debug, Clone)]
pub struct WechatCpDepartment<'a> {
    client: &'a WechatCpClient,
}

#[allow(unused)]
impl<'a> WechatCpDepartment<'a> {
    #[inline]
    pub fn new(client: &'a WechatCpClient) -> Self {
        Self { client }
    }

    /// 创建部门
    ///
    /// 最多支持创建500个部门。
    ///
    /// # 参数
    /// * `req` - 部门信息，包含部门名称、父部门 id、排序值等
    ///
    /// # 返回
    /// 返回 `LabradorResult<i64>`，包含创建的部门 id
    ///
    /// # 官方文档
    /// <https://developer.work.weixin.qq.com/document/path/90205>
    pub async fn create(&self, req: DepartmentInfo) -> LabradorResult<i64> {
        let response: WechatApiResponse<CreateDepartmentResponse> =
            self.client.post("/cgi-bin/department/create", req).await?;
        Ok(response.into_result()?.id)
    }

    /// 获取子部门ID列表
    ///
    /// 获取指定部门下的子部门ID列表（不包含部门详细信息）。
    ///
    /// # 参数
    /// * `id` - 部门 id，不填默认获取全量组织架构
    ///
    /// # 返回
    /// 返回 `LabradorResult<Vec<DepartmentIdInfo>>`，包含子部门的 id、parentid、order 信息
    ///
    /// # 官方文档
    /// <https://developer.work.weixin.qq.com/document/path/95350>
    pub async fn simple_list(&self, id: Option<i64>) -> LabradorResult<Vec<DepartmentIdInfo>> {
        let mut url = "/cgi-bin/department/simplelist".to_string();
        if let Some(dept_id) = id {
            url.push_str(&format!("?id={}", dept_id));
        }
        let response: WechatApiResponse<DepartmentSimpleListResponse> =
            self.client.get(&url).await?;
        Ok(response.into_result()?.department_id)
    }

    /// 获取部门列表
    ///
    /// 获取指定部门及其下的子部门列表（包含部门详细信息）。
    ///
    /// # 参数
    /// * `id` - 部门 id，不填默认获取全量组织架构
    ///
    /// # 返回
    /// 返回 `LabradorResult<Vec<DepartmentInfo>>`，包含部门名称、英文名、排序值等详细信息
    ///
    /// # 官方文档
    /// <https://developer.work.weixin.qq.com/document/path/90208>
    pub async fn list(&self, id: Option<i64>) -> LabradorResult<Vec<DepartmentInfo>> {
        let mut url = "/cgi-bin/department/list".to_string();
        if let Some(dept_id) = id {
            url.push_str(&format!("?id={}", dept_id));
        }
        let response: WechatApiResponse<DepartmentListResponse> = self.client.get(&url).await?;
        Ok(response.into_result()?.department)
    }

    /// 更新部门
    ///
    /// 更新部门信息。
    /// 注意：如果id为0(未部门)、1(黑名单)、2(星标组)，或者不存在的id，会返回错误。
    ///
    /// # 参数
    /// * `req` - 更新部门请求，包含部门 id 及需要更新的字段（名称、父部门、排序值等）
    ///
    /// # 返回
    /// 返回 `LabradorResult<WechatApiResponse>`
    ///
    /// # 官方文档
    /// <https://developer.work.weixin.qq.com/document/path/90206>
    pub async fn update(&self, req: UpdateDepartmentRequest) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse =
            self.client.post("/cgi-bin/department/update", req).await?;
        Ok(response)
    }

    /// 删除部门
    ///
    /// 删除指定部门。应用须拥有指定部门的管理权限。
    /// 注意：不能删除根部门，不能删除含有子部门、成员的部门。
    ///
    /// # 参数
    /// * `department_id` - 部门 id
    ///
    /// # 返回
    /// 返回 `LabradorResult<WechatApiResponse>`
    ///
    /// # 官方文档
    /// <https://developer.work.weixin.qq.com/document/path/90207>
    pub async fn delete(&self, department_id: i64) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .get(&format!("/cgi-bin/department/delete?id={}", department_id))
            .await?;
        Ok(response)
    }
}

// ============================================================================
// 请求与响应结构体
// ============================================================================

/// 部门信息（用于创建和响应）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepartmentInfo {
    /// 部门id，创建时指定则必须大于0，否则由微信自动分配
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// 部门名称，长度限制为1~32个字符
    pub name: String,
    /// 英文名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_en: Option<String>,
    /// 父部门id，32位整型
    pub parentid: i32,
    /// 在父部门中的次序值，order值大的排序靠前
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
}

/// 更新部门请求
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDepartmentRequest {
    /// 部门id
    pub id: i32,
    /// 部门名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 英文名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_en: Option<String>,
    /// 父部门id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parentid: Option<i32>,
    /// 在父部门中的次序值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
}

/// 创建部门响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateDepartmentResponse {
    /// 创建的部门id
    pub id: i64,
}

/// 部门列表响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DepartmentListResponse {
    /// 部门列表
    pub department: Vec<DepartmentInfo>,
}

/// 部门ID信息（用于简单列表）
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepartmentIdInfo {
    /// 部门id
    pub id: i32,
    /// 父部门id
    pub parentid: i32,
    /// 在父部门中的次序值
    pub order: i32,
}

/// 部门简单列表响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DepartmentSimpleListResponse {
    /// 部门ID列表
    pub department_id: Vec<DepartmentIdInfo>,
}
