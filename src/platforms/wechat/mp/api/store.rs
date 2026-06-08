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
use serde_json::json;

use crate::errors::LabradorResult;
use crate::wechat::client::WechatApiResponse;
use crate::wechat::mp::WechatMpClient;

/// 微信门店模块
#[derive(Debug, Clone)]
pub struct WechatMpStore<'a> {
    client: &'a WechatMpClient,
}

#[allow(unused)]
impl<'a> WechatMpStore<'a> {
    #[inline]
    pub fn new(client: &'a WechatMpClient) -> Self {
        Self { client }
    }

    /// 获取门店小程序类目列表
    ///
    /// 本接口用于获取门店小程序可选的类目列表。
    pub async fn get_cate_list(&self) -> LabradorResult<Vec<StoreCategory>> {
        let response: WechatApiResponse<GetCateListResponse> = self
            .client
            .wechat_client()
            .get("/wxa/get_store_category")
            .await?;
        Ok(response.into_result()?.data)
    }

    /// 申请门店小程序
    ///
    /// 本接口用于申请开通门店小程序。
    pub async fn apply_store(
        &self,
        request: &ApplyStoreRequest,
    ) -> LabradorResult<ApplyStoreResponse> {
        let response: WechatApiResponse<ApplyStoreResponse> = self
            .client
            .wechat_client()
            .post("/wxa/apply_store_wxa", request)
            .await?;
        response.into_result()
    }

    /// 查询门店小程序审核结果
    ///
    /// 本接口用于查询门店小程序的审核结果。
    pub async fn get_audit_info(&self, apply_id: i64) -> LabradorResult<AuditInfo> {
        let request = json!({ "apply_id": apply_id });
        let response: WechatApiResponse<AuditInfo> = self
            .client
            .wechat_client()
            .post("/wxa/get_merchant_audit_info", request)
            .await?;
        response.into_result()
    }

    /// 修改门店小程序信息
    ///
    /// 本接口用于修改门店小程序信息。
    pub async fn modify_store(
        &self,
        request: &ModifyStoreRequest,
    ) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/wxa/modify_merchant", request)
            .await?;
        Ok(response)
    }

    /// 获取省市区列表
    ///
    /// 本接口用于获取腾讯地图的省市区列表。
    pub async fn get_district_list(&self) -> LabradorResult<Vec<DistrictInfo>> {
        let response: WechatApiResponse<GetDistrictListResponse> =
            self.client.wechat_client().get("/wxa/get_district").await?;
        Ok(response.into_result()?.data)
    }

    /// 搜索周边门店
    ///
    /// 本接口用于搜索周边门店。
    pub async fn poi_list_search(
        &self,
        request: &PoiSearchRequest,
    ) -> LabradorResult<PoiSearchResponse> {
        let response: WechatApiResponse<PoiSearchResponse> = self
            .client
            .wechat_client()
            .post("/wxa/search_map_poi", request)
            .await?;
        response.into_result()
    }

    /// 添加门店
    ///
    /// 本接口用于添加门店（包括门店小程序的门店）。
    pub async fn add_entity_shop(
        &self,
        request: &AddEntityShopRequest,
    ) -> LabradorResult<AddEntityShopResponse> {
        let response: WechatApiResponse<AddEntityShopResponse> = self
            .client
            .wechat_client()
            .post("/wxa/add_store_entity", request)
            .await?;
        response.into_result()
    }

    /// 获取单个门店信息
    ///
    /// 本接口用于获取单个门店的详细信息。
    pub async fn get_poi(&self, poi_id: &str) -> LabradorResult<StoreInfo> {
        let request = json!({ "poi_id": poi_id });
        let response: WechatApiResponse<StoreInfo> = self
            .client
            .wechat_client()
            .post("/wxa/get_store_info", request)
            .await?;
        response.into_result()
    }

    /// 获取门店列表
    ///
    /// 本接口用于获取门店列表。
    pub async fn get_poi_list(&self, offset: u32, limit: u32) -> LabradorResult<StoreListResponse> {
        let request = json!({
            "offset": offset,
            "limit": limit
        });
        let response: WechatApiResponse<StoreListResponse> = self
            .client
            .wechat_client()
            .post("/wxa/get_store_list", request)
            .await?;
        response.into_result()
    }

    /// 删除门店
    ///
    /// 本接口用于删除门店。
    pub async fn delete_poi(&self, poi_id: &str) -> LabradorResult<WechatApiResponse> {
        let request = json!({ "poi_id": poi_id });
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/wxa/del_store", request)
            .await?;
        Ok(response)
    }

    /// 更新门店信息
    ///
    /// 本接口用于更新门店信息。
    pub async fn update_poi(
        &self,
        request: &UpdateStoreRequest,
    ) -> LabradorResult<UpdateStoreResponse> {
        let response: WechatApiResponse<UpdateStoreResponse> = self
            .client
            .wechat_client()
            .post("/wxa/update_store", request)
            .await?;
        response.into_result()
    }

    /// 在地图中创建门店
    ///
    /// 本接口用于在腾讯地图中创建门店。
    pub async fn create_map_poi(
        &self,
        request: &CreateMapPoiRequest,
    ) -> LabradorResult<CreateMapPoiResponse> {
        let response: WechatApiResponse<CreateMapPoiResponse> = self
            .client
            .wechat_client()
            .post("/wxa/create_map_poi", request)
            .await?;
        response.into_result()
    }
}

// -------------------- 门店相关结构体 --------------------

/// 门店类目
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreCategory {
    /// 类目ID
    pub id: String,
    /// 类目名称
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
struct GetCateListResponse {
    data: Vec<StoreCategory>,
}

/// 申请门店请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyStoreRequest {
    /// 门店小程序的头像
    pub headimg_url: String,
    /// 门店小程序名称
    pub nick_name: String,
    /// 营业执照
    pub qualification_list: String,
    /// 其他证明材料
    pub qualification_prove: Option<String>,
    /// 行业分类
    pub category: Option<String>,
    /// 门店小程序名称来源
    pub name_source: Option<String>,
    /// 经营年限
    pub business_years: Option<String>,
}

/// 申请门店响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyStoreResponse {
    pub apply_id: i64,
    pub audit_id: i64,
}

/// 审核信息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditInfo {
    pub audit_id: i64,
    pub status: i32,
    pub reason: Option<String>,
    pub nick_name: Option<String>,
    pub headimg_url: Option<String>,
}

/// 修改门店请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyStoreRequest {
    pub headimg_url: Option<String>,
    pub nick_name: Option<String>,
    pub qualification_list: Option<String>,
    pub category: Option<String>,
}

/// 区划信息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DistrictInfo {
    pub id: String,
    pub name: String,
    pub level: i32,
    pub fullname: String,
    pub lng: f64,
    pub lat: f64,
    pub pinyin: Vec<String>,
    pub children: Option<Vec<DistrictInfo>>,
}

#[derive(Debug, Clone, Deserialize)]
struct GetDistrictListResponse {
    data: Vec<DistrictInfo>,
}

/// 门店搜索请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PoiSearchRequest {
    pub keyword: String,
    pub longitude: f64,
    pub latitude: f64,
    pub offset: u32,
    pub limit: u32,
}

/// 门店搜索结果
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoiSearchResponse {
    pub data: Vec<PoiInfo>,
    pub count: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoiInfo {
    pub poi_id: String,
    pub title: String,
    pub address: String,
    pub province: String,
    pub city: String,
    pub district: String,
    pub longitude: f64,
    pub latitude: f64,
    pub phone: String,
    pub category: String,
    pub shop_hours: String,
}

/// 添加门店请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddEntityShopRequest {
    pub poi_id: String,
    pub store_name: String,
    pub longitude: f64,
    pub latitude: f64,
    pub province: String,
    pub city: String,
    pub district: String,
    pub address: String,
    pub category: String,
    pub phone: String,
    pub photo_list: Vec<PhotoItem>,
    pub hour: String,
    pub credential_list: Vec<CredentialItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhotoItem {
    pub photo_url: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialItem {
    pub name: String,
    pub number: String,
    pub url: String,
}

/// 添加门店响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddEntityShopResponse {
    pub audit_id: String,
}

/// 门店信息
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreInfo {
    pub poi_id: String,
    pub business_name: String,
    pub longitude: f64,
    pub latitude: f64,
    pub province: String,
    pub city: String,
    pub district: String,
    pub address: String,
    pub telephone: String,
    pub photo_list: Vec<PhotoItem>,
    pub qualification_name: String,
    pub qualification_num: String,
    pub open_time: String,
    pub status: i32,
}

/// 门店列表响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreListResponse {
    pub business_list: Vec<StoreListItem>,
    pub total_count: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreListItem {
    #[serde(flatten)]
    pub base_info: StoreInfo,
}

/// 更新门店请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStoreRequest {
    pub poi_id: String,
    pub hour: Option<String>,
    pub contract_phone: Option<String>,
    pub pic_list: Option<String>,
    pub card_id: Option<String>,
}

/// 更新门店响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStoreResponse {
    pub data: UpdateStoreData,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStoreData {
    pub has_audit_id: i32,
    pub audit_id: i64,
}

/// 创建地图门店请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMapPoiRequest {
    pub name: String,
    pub longitude: String,
    pub latitude: String,
    pub province: String,
    pub city: String,
    pub district: String,
    pub address: String,
    pub category: String,
    pub telephone: String,
    pub photo: String,
    pub license: String,
    pub introduct: String,
    pub districtid: String,
    pub poi_id: Option<String>,
}

/// 创建地图门店响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMapPoiResponse {
    pub data: CreateMapPoiData,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMapPoiData {
    pub base_id: i64,
    pub rich_id: i64,
}
