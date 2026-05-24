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
use serde_json::Value;

use crate::errors::LabradorResult;
use crate::wechat::client::WechatApiResponse;
use crate::wechat::miniapp::WechatMiniAppClient;

/// 云开发模块
///
/// 提供云函数、数据库、存储以及短信等云开发相关接口。
#[derive(Debug, Clone)]
pub struct WechatMxaCloudBase<'a> {
    client: &'a WechatMiniAppClient,
}

#[allow(unused)]
impl<'a> WechatMxaCloudBase<'a> {
    #[inline]
    pub fn new(client: &'a WechatMiniAppClient) -> Self {
        Self { client }
    }

    // ========================= 云函数 =========================

    /// 触发云函数
    ///
    /// 通过本接口可以触发云函数。
    ///
    /// # 参数说明
    /// * `env` - 云环境ID
    /// * `name` - 云函数名称
    /// * `data` - 传递给云函数的参数
    pub async fn invoke_cloud_function(
        &self,
        env: &str,
        name: &str,
        data: Option<Value>,
    ) -> LabradorResult<InvokeFunctionResponse> {
        let request = serde_json::json!({
            "env": env,
            "name": name,
            "data": data,
        });
        let response: WechatApiResponse<InvokeFunctionResponse> = self
            .client
            .wechat_client()
            .post("/tcb/invokecloudfunction", request)
            .await?;
        response.into_result()
    }

    /// 延时调用云函数
    ///
    /// 该接口用于延时调用云函数。
    ///
    /// # 参数说明
    /// * `env` - 云环境ID
    /// * `function_name` - 云函数名称
    /// * `delay` - 延迟时间，单位秒
    /// * `data` - 传递给云函数的参数
    pub async fn add_delayed_function_task(
        &self,
        env: &str,
        function_name: &str,
        delay: i32,
        data: Option<Value>,
    ) -> LabradorResult<DelayedTaskResponse> {
        let request = serde_json::json!({
            "env": env,
            "function_name": function_name,
            "delay": delay,
            "data": data,
        });
        let response: WechatApiResponse<DelayedTaskResponse> = self
            .client
            .wechat_client()
            .post("/tcb/adddelayedfunctiontask", request)
            .await?;
        response.into_result()
    }

    // ========================= 数据库 =========================

    /// 数据库插入记录
    ///
    /// 通过本接口可以数据库插入记录。
    ///
    /// # 参数说明
    /// * `request` - 插入记录请求参数
    pub async fn database_add(&self, request: &DatabaseAddRequest) -> LabradorResult<DatabaseAddResponse> {
        let response: WechatApiResponse<DatabaseAddResponse> = self
            .client
            .wechat_client()
            .post("/tcb/databaseadd", request)
            .await?;
        response.into_result()
    }

    /// 数据库聚合
    ///
    /// 通过本接口可以数据库聚合查询。
    ///
    /// # 参数说明
    /// * `request` - 聚合查询请求参数
    pub async fn database_aggregate(&self, request: &DatabaseAggregateRequest) -> LabradorResult<DatabaseAggregateResponse> {
        let response: WechatApiResponse<DatabaseAggregateResponse> = self
            .client
            .wechat_client()
            .post("/tcb/databaseaggregate", request)
            .await?;
        response.into_result()
    }

    /// 新增集合
    ///
    /// 通过本接口可以新增数据库集合。
    ///
    /// # 参数说明
    /// * `env` - 云环境ID
    /// * `collection_name` - 集合名称
    pub async fn database_collection_add(&self, env: &str, collection_name: &str) -> LabradorResult<WechatApiResponse> {
        let request = serde_json::json!({
            "env": env,
            "collection_name": collection_name,
        });
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/tcb/databasecollectionadd", request)
            .await?;
        Ok(response)
    }

    /// 删除集合
    ///
    /// 通过本接口可以删除数据库集合。
    ///
    /// # 参数说明
    /// * `env` - 云环境ID
    /// * `collection_name` - 集合名称
    pub async fn database_collection_delete(&self, env: &str, collection_name: &str) -> LabradorResult<WechatApiResponse> {
        let request = serde_json::json!({
            "env": env,
            "collection_name": collection_name,
        });
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/tcb/databasecollectiondelete", request)
            .await?;
        Ok(response)
    }

    /// 获取集合信息
    ///
    /// 通过本接口可以获取特定云环境下集合信息。
    ///
    /// # 参数说明
    /// * `env` - 云环境ID
    /// * `limit` - 返回集合数量，最大100
    /// * `offset` - 偏移量
    pub async fn database_collection_get(&self, env: &str, limit: i32, offset: i32) -> LabradorResult<CollectionInfoResponse> {
        let request = serde_json::json!({
            "env": env,
            "limit": limit,
            "offset": offset,
        });
        let response: WechatApiResponse<CollectionInfoResponse> = self
            .client
            .wechat_client()
            .post("/tcb/databasecollectionget", request)
            .await?;
        response.into_result()
    }

    /// 统计集合记录数
    ///
    /// 通过本接口可以统计集合记录数或统计查询语句对应的结果记录数。
    ///
    /// # 参数说明
    /// * `request` - 统计请求参数
    pub async fn database_count(&self, request: &DatabaseCountRequest) -> LabradorResult<DatabaseCountResponse> {
        let response: WechatApiResponse<DatabaseCountResponse> = self
            .client
            .wechat_client()
            .post("/tcb/databasecount", request)
            .await?;
        response.into_result()
    }

    /// 数据库删除记录
    ///
    /// 通过本接口可以数据库删除记录。
    ///
    /// # 参数说明
    /// * `request` - 删除记录请求参数
    pub async fn database_delete(&self, request: &DatabaseDeleteRequest) -> LabradorResult<DatabaseDeleteResponse> {
        let response: WechatApiResponse<DatabaseDeleteResponse> = self
            .client
            .wechat_client()
            .post("/tcb/databasedelete", request)
            .await?;
        response.into_result()
    }

    /// 数据库导出
    ///
    /// 通过本接口可以进行数据库导出。
    ///
    /// # 参数说明
    /// * `request` - 导出请求参数
    pub async fn database_migrate_export(&self, request: &MigrateExportRequest) -> LabradorResult<MigrateExportResponse> {
        let response: WechatApiResponse<MigrateExportResponse> = self
            .client
            .wechat_client()
            .post("/tcb/databasemigrateexport", request)
            .await?;
        response.into_result()
    }

    /// 数据库导入
    ///
    /// 通过本接口可以进行数据库导入。
    ///
    /// # 参数说明
    /// * `request` - 导入请求参数
    pub async fn database_migrate_import(&self, request: &MigrateImportRequest) -> LabradorResult<MigrateImportResponse> {
        let response: WechatApiResponse<MigrateImportResponse> = self
            .client
            .wechat_client()
            .post("/tcb/databasemigrateimport", request)
            .await?;
        response.into_result()
    }

    /// 数据库迁移状态查询
    ///
    /// 通过本接口可以数据库迁移状态查询。
    ///
    /// # 参数说明
    /// * `env` - 云环境ID
    /// * `job_id` - 迁移任务ID
    pub async fn database_migrate_query_info(&self, env: &str, job_id: i32) -> LabradorResult<MigrateQueryInfoResponse> {
        let request = serde_json::json!({
            "env": env,
            "job_id": job_id,
        });
        let response: WechatApiResponse<MigrateQueryInfoResponse> = self
            .client
            .wechat_client()
            .post("/tcb/databasemigratequeryinfo", request)
            .await?;
        response.into_result()
    }

    /// 数据库查询记录
    ///
    /// 通过本接口可以数据库查询记录。
    ///
    /// # 参数说明
    /// * `request` - 查询请求参数
    pub async fn database_query(&self, request: &DatabaseQueryRequest) -> LabradorResult<DatabaseQueryResponse> {
        let response: WechatApiResponse<DatabaseQueryResponse> = self
            .client
            .wechat_client()
            .post("/tcb/databasequery", request)
            .await?;
        response.into_result()
    }

    /// 数据库更新记录
    ///
    /// 通过本接口可以数据库更新记录。
    ///
    /// # 参数说明
    /// * `request` - 更新请求参数
    pub async fn database_update(&self, request: &DatabaseUpdateRequest) -> LabradorResult<DatabaseUpdateResponse> {
        let response: WechatApiResponse<DatabaseUpdateResponse> = self
            .client
            .wechat_client()
            .post("/tcb/databaseupdate", request)
            .await?;
        response.into_result()
    }

    /// 更新数据库索引
    ///
    /// 通过本接口可以变更数据库索引。
    ///
    /// # 参数说明
    /// * `request` - 更新索引请求参数
    pub async fn update_index(&self, request: &UpdateIndexRequest) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/tcb/updateindex", request)
            .await?;
        Ok(response)
    }

    // ========================= 存储 =========================

    /// 获取文件上传链接
    ///
    /// 通过本接口可以获取文件上传链接。
    ///
    /// # 参数说明
    /// * `request` - 获取上传链接请求参数
    pub async fn upload_file(&self, request: &UploadFileRequest) -> LabradorResult<UploadFileResponse> {
        let response: WechatApiResponse<UploadFileResponse> = self
            .client
            .wechat_client()
            .post("/tcb/uploadfile", request)
            .await?;
        response.into_result()
    }

    /// 获取文件下载链接
    ///
    /// 通过本接口可以获取文件下载链接。
    ///
    /// # 参数说明
    /// * `request` - 获取下载链接请求参数
    pub async fn batch_download_file(&self, request: &BatchDownloadFileRequest) -> LabradorResult<BatchDownloadFileResponse> {
        let response: WechatApiResponse<BatchDownloadFileResponse> = self
            .client
            .wechat_client()
            .post("/tcb/batchdownloadfile", request)
            .await?;
        response.into_result()
    }

    /// 删除文件
    ///
    /// 通过本接口可以删除云存储中的文件。
    ///
    /// # 参数说明
    /// * `request` - 删除文件请求参数
    pub async fn batch_delete_file(&self, request: &BatchDeleteFileRequest) -> LabradorResult<BatchDeleteFileResponse> {
        let response: WechatApiResponse<BatchDeleteFileResponse> = self
            .client
            .wechat_client()
            .post("/tcb/batchdeletefile", request)
            .await?;
        response.into_result()
    }

    // ========================= 其他 =========================

    /// 发送短信v2
    ///
    /// 发送携带 URL Link 的短信。
    ///
    /// # 参数说明
    /// * `request` - 发送短信请求参数
    pub async fn send_sms_v2(&self, request: &SendSmsV2Request) -> LabradorResult<SendSmsResponse> {
        let response: WechatApiResponse<SendSmsResponse> = self
            .client
            .wechat_client()
            .post("/tcb/sendsmsv2", request)
            .await?;
        response.into_result()
    }

    /// 发送短信
    ///
    /// 发送支持打开云开发静态网站的短信，该 H5 可以打开小程序。
    ///
    /// # 参数说明
    /// * `request` - 发送短信请求参数
    pub async fn send_sms(&self, request: &SendSmsRequest) -> LabradorResult<SendSmsResponse> {
        let response: WechatApiResponse<SendSmsResponse> = self
            .client
            .wechat_client()
            .post("/tcb/sendsms", request)
            .await?;
        response.into_result()
    }

    /// 创建发短信任务
    ///
    /// 该接口用于创建发短信任务。
    ///
    /// # 参数说明
    /// * `request` - 创建任务请求参数
    pub async fn create_send_sms_task(&self, request: &CreateSendSmsTaskRequest) -> LabradorResult<CreateSendSmsTaskResponse> {
        let response: WechatApiResponse<CreateSendSmsTaskResponse> = self
            .client
            .wechat_client()
            .post("/tcb/createsendsmstask", request)
            .await?;
        response.into_result()
    }

    /// 云开发上报接口
    ///
    /// 该接口为云开发通用上报接口。
    ///
    /// # 参数说明
    /// * `request` - 上报数据
    pub async fn cloud_base_report(&self, request: Value) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/tcb/cloudbasereport", request)
            .await?;
        Ok(response)
    }

    /// 查询短信记录
    ///
    /// 该接口用于查询 2 个月内的短信记录。
    ///
    /// # 参数说明
    /// * `request` - 查询请求参数
    pub async fn describe_sms_records(&self, request: &DescribeSmsRecordsRequest) -> LabradorResult<DescribeSmsRecordsResponse> {
        let response: WechatApiResponse<DescribeSmsRecordsResponse> = self
            .client
            .wechat_client()
            .post("/tcb/describesmsrecords", request)
            .await?;
        response.into_result()
    }

    /// 描述扩展上传文件信息
    ///
    /// 该接口用于描述扩展上传文件信息。
    ///
    /// # 参数说明
    /// * `request` - 描述请求参数
    pub async fn describe_extension_upload_info(&self, request: &DescribeExtensionUploadInfoRequest) -> LabradorResult<DescribeExtensionUploadInfoResponse> {
        let response: WechatApiResponse<DescribeExtensionUploadInfoResponse> = self
            .client
            .wechat_client()
            .post("/tcb/describeextensionuploadinfo", request)
            .await?;
        response.into_result()
    }

    /// 获取云开发数据
    ///
    /// 该接口用于获取云开发数据。
    ///
    /// # 参数说明
    /// * `request` - 获取数据请求参数
    pub async fn get_statistics(&self, request: &GetStatisticsRequest) -> LabradorResult<GetStatisticsResponse> {
        let response: WechatApiResponse<GetStatisticsResponse> = self
            .client
            .wechat_client()
            .post("/tcb/getstatistics", request)
            .await?;
        response.into_result()
    }

    /// 获取cloudID对应的数据
    ///
    /// 该接口用于换取 cloudID 对应的开放数据。
    ///
    /// # 参数说明
    /// * `cloudid_list` - cloudID列表，最多20个
    pub async fn get_open_data(&self, cloudid_list: Vec<String>) -> LabradorResult<Vec<OpenDataItem>> {
        let request = serde_json::json!({
            "cloudid_list": cloudid_list,
        });
        let response: WechatApiResponse<GetOpenDataResponse> = self
            .client
            .wechat_client()
            .post("/wxa/getopendata", request)
            .await?;
        response.into_result().map(|r| r.data)
    }

    /// 获取实时语音签名
    ///
    /// 该接口用于获取实时语音签名。
    ///
    /// # 参数说明
    /// * `request` - 获取签名请求参数
    pub async fn get_voip_sign(&self, request: &GetVoipSignRequest) -> LabradorResult<GetVoipSignResponse> {
        let response: WechatApiResponse<GetVoipSignResponse> = self
            .client
            .wechat_client()
            .post("/wxa/getvoipsign", request)
            .await?;
        response.into_result()
    }

    /// 获取腾讯云API调用凭证
    ///
    /// 通过本接口可以获取腾讯云API调用凭证，用于调用腾讯云可用 API。
    ///
    /// # 参数说明
    /// * `request` - 获取凭证请求参数
    pub async fn get_qcloud_token(&self, request: &GetQcloudTokenRequest) -> LabradorResult<GetQcloudTokenResponse> {
        let response: WechatApiResponse<GetQcloudTokenResponse> = self
            .client
            .wechat_client()
            .post("/tcb/getqcloudtoken", request)
            .await?;
        response.into_result()
    }
}

// ============================================================================
// 请求与响应结构体（按功能分组）
// 注意：使用 #[serde(rename_all = "camelCase")] 处理驼峰字段。
// ============================================================================

// -------------------- 云函数 结构体 --------------------

/// 触发云函数响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvokeFunctionResponse {
    /// 云函数返回的数据
    pub resp_data: String,
}

/// 延时调用云函数响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelayedTaskResponse {
    /// 任务ID
    pub task_id: String,
}

// -------------------- 数据库 结构体 --------------------

/// 数据库添加记录请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseAddRequest {
    /// 云环境ID
    pub env: String,
    /// 集合名称
    pub collection_name: String,
    /// 要添加的数据，可以是对象或数组
    pub data: Value,
}

/// 数据库添加记录响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseAddResponse {
    /// 插入成功的记录ID列表
    pub id_list: Vec<String>,
}

/// 数据库聚合请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseAggregateRequest {
    /// 云环境ID
    pub env: String,
    /// 集合名称
    pub collection_name: String,
    /// 聚合管道
    pub pipeline: Value,
}

/// 数据库聚合响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseAggregateResponse {
    /// 查询结果列表
    pub data: Vec<Value>,
    /// 请求ID
    pub request_id: String,
}

/// 集合信息响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionInfoResponse {
    /// 集合列表
    pub collections: Vec<CollectionInfo>,
    /// 请求ID
    pub request_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionInfo {
    /// 集合名称
    pub name: String,
    /// 集合中文名
    pub chinese_name: String,
    /// 集合类型
    pub collection_type: String,
}

/// 数据库计数请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseCountRequest {
    /// 云环境ID
    pub env: String,
    /// 集合名称
    pub collection_name: String,
    /// 查询条件
    pub query: Value,
}

/// 数据库计数响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseCountResponse {
    /// 记录总数
    pub total: i32,
    /// 请求ID
    pub request_id: String,
}

/// 数据库删除请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseDeleteRequest {
    /// 云环境ID
    pub env: String,
    /// 集合名称
    pub collection_name: String,
    /// 删除条件
    pub query: Value,
}

/// 数据库删除响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseDeleteResponse {
    /// 删除的记录数
    pub deleted: i32,
    /// 请求ID
    pub request_id: String,
}

/// 数据库导出请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MigrateExportRequest {
    /// 云环境ID
    pub env: String,
    /// 集合名称
    pub collection_name: String,
    /// 导出文件路径
    pub file_path: String,
    /// 导出字段列表，逗号分隔
    pub fields: Option<String>,
    /// 查询条件
    pub query: Option<Value>,
}

/// 数据库导出响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MigrateExportResponse {
    /// 任务ID
    pub job_id: i32,
    /// 请求ID
    pub request_id: String,
}

/// 数据库导入请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MigrateImportRequest {
    /// 云环境ID
    pub env: String,
    /// 集合名称
    pub collection_name: String,
    /// 导入文件路径
    pub file_path: String,
    /// 导入冲突时处理方式：UPSERT-插入或更新，INSERT-插入
    pub conflict_mode: Option<String>,
    /// 是否强制导入，跳过格式检查
    pub force: Option<bool>,
    /// 文件类型：json或csv
    pub file_type: Option<String>,
    /// 上传文件大小
    pub file_size: Option<i32>,
}

/// 数据库导入响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MigrateImportResponse {
    /// 任务ID
    pub job_id: i32,
    /// 请求ID
    pub request_id: String,
}

/// 数据库迁移状态查询响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MigrateQueryInfoResponse {
    /// 任务状态：0-等待中，1-进行中，2-成功，3-失败，4-取消
    pub status: i32,
    /// 任务成功时的下载链接
    pub file_url: Option<String>,
    /// 记录总数
    pub record_success: Option<i32>,
    /// 失败记录数
    pub record_fail: Option<i32>,
    /// 错误信息
    pub err_msg: Option<String>,
    /// 请求ID
    pub request_id: String,
}

/// 数据库查询请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseQueryRequest {
    /// 云环境ID
    pub env: String,
    /// 集合名称
    pub collection_name: String,
    /// 查询条件
    pub query: Value,
    /// 排序条件
    pub order_by: Option<Value>,
    /// 限制返回数量
    pub limit: Option<i32>,
    /// 偏移量
    pub offset: Option<i32>,
}

/// 数据库查询响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseQueryResponse {
    /// 查询结果列表
    pub data: Vec<Value>,
    /// 请求ID
    pub request_id: String,
}

/// 数据库更新请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseUpdateRequest {
    /// 云环境ID
    pub env: String,
    /// 集合名称
    pub collection_name: String,
    /// 更新条件
    pub query: Value,
    /// 更新操作
    pub update: Value,
    /// 是否允许全量更新（无查询条件时）
    pub multi: Option<bool>,
    /// 是否允许merge模式
    pub merge: Option<bool>,
    /// 是否允许upsert
    pub upsert: Option<bool>,
}

/// 数据库更新响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseUpdateResponse {
    /// 更新的记录数
    pub updated: i32,
    /// 插入的记录ID（当upsert时）
    pub upsert_id: Option<String>,
    /// 请求ID
    pub request_id: String,
}

/// 更新索引请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateIndexRequest {
    /// 云环境ID
    pub env: String,
    /// 集合名称
    pub collection_name: String,
    /// 创建/删除索引：create-创建，drop-删除
    pub action: String,
    /// 索引名称
    pub index_name: String,
    /// 索引配置
    pub index_config: IndexConfig,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexConfig {
    /// 索引字段列表
    pub keys: Vec<IndexKey>,
    /// 索引选项
    pub options: IndexOptions,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexKey {
    /// 字段名
    pub name: String,
    /// 排序方向：1-升序，-1-降序
    pub direction: i32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexOptions {
    /// 是否唯一索引
    pub unique: Option<bool>,
    /// 是否稀疏索引
    pub sparse: Option<bool>,
}

// -------------------- 存储 结构体 --------------------

/// 获取上传链接请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadFileRequest {
    /// 云环境ID
    pub env: String,
    /// 上传文件路径
    pub path: String,
}

/// 获取上传链接响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadFileResponse {
    /// 文件ID
    pub file_id: String,
    /// 上传链接
    pub url: String,
    /// 授权信息
    pub token: String,
    /// 授权信息
    pub authorization: String,
    /// 文件存储路径
    pub cos_file_id: String,
}

/// 批量获取下载链接请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchDownloadFileRequest {
    /// 云环境ID
    pub env: String,
    /// 文件ID列表，最多50个
    pub file_id_list: Vec<String>,
    /// 下载链接有效期，单位秒，默认7200
    pub max_age: Option<i32>,
}

/// 批量获取下载链接响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchDownloadFileResponse {
    /// 文件下载信息列表
    pub file_list: Vec<DownloadFileInfo>,
    /// 请求ID
    pub request_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadFileInfo {
    /// 文件ID
    pub file_id: String,
    /// 下载链接
    pub download_url: String,
    /// 文件状态
    pub status: i32,
    /// 错误信息
    pub err_msg: Option<String>,
}

/// 批量删除文件请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchDeleteFileRequest {
    /// 云环境ID
    pub env: String,
    /// 文件ID列表，最多50个
    pub file_id_list: Vec<String>,
}

/// 批量删除文件响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchDeleteFileResponse {
    /// 删除结果列表
    pub delete_list: Vec<DeleteFileInfo>,
    /// 请求ID
    pub request_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteFileInfo {
    /// 文件ID
    pub file_id: String,
    /// 删除状态码
    pub code: i32,
    /// 删除信息
    pub msg: String,
}

// -------------------- 其他 结构体 --------------------

/// 发送短信v2请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendSmsV2Request {
    /// 云环境ID
    pub env: String,
    /// 短信模板ID
    pub template_id: String,
    /// 短信模板参数
    pub template_param_list: Vec<TemplateParam>,
    /// 接收短信的手机号列表，最多100个
    pub phone_number_list: Vec<String>,
    /// 资源类型
    pub resource_type: String,
    /// 资源ID
    pub resource_id: String,
    /// 资源子类型
    pub resource_sub_type: Option<String>,
    /// 失效时间戳，单位秒
    pub expire_time: Option<i64>,
}

/// 发送短信请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendSmsRequest {
    /// 云环境ID
    pub env: String,
    /// 短信模板ID
    pub template_id: String,
    /// 短信模板参数
    pub template_param_list: Vec<TemplateParam>,
    /// 接收短信的手机号列表，最多100个
    pub phone_number_list: Vec<String>,
    /// 资源类型
    pub resource_type: String,
    /// 资源ID
    pub resource_id: String,
    /// 资源子类型
    pub resource_sub_type: Option<String>,
    /// 失效时间戳，单位秒
    pub expire_time: Option<i64>,
}

/// 短信模板参数
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateParam {
    /// 参数名称
    pub name: String,
    /// 参数值
    pub value: String,
}

/// 发送短信响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendSmsResponse {
    /// 任务ID
    pub task_id: String,
    /// 请求ID
    pub request_id: String,
}

/// 创建发送短信任务请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSendSmsTaskRequest {
    /// 云环境ID
    pub env: String,
    /// 任务名称
    pub task_name: String,
    /// 短信模板ID
    pub template_id: String,
    /// 短信模板参数
    pub template_param_list: Vec<TemplateParam>,
    /// 接收短信的手机号列表，最多100个
    pub phone_number_list: Vec<String>,
    /// 资源类型
    pub resource_type: String,
    /// 资源ID
    pub resource_id: String,
    /// 任务执行时间戳
    pub execute_time: Option<i64>,
}

/// 创建发送短信任务响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSendSmsTaskResponse {
    /// 任务ID
    pub task_id: String,
    /// 请求ID
    pub request_id: String,
}

/// 查询短信记录请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeSmsRecordsRequest {
    /// 云环境ID
    pub env: String,
    /// 开始时间戳，单位秒
    pub start_time: i64,
    /// 结束时间戳，单位秒
    pub end_time: i64,
    /// 分页偏移量
    pub offset: Option<i32>,
    /// 分页大小，最大100
    pub limit: Option<i32>,
    /// 任务ID
    pub task_id: Option<String>,
    /// 手机号
    pub phone_number: Option<String>,
    /// 模板ID
    pub template_id: Option<String>,
}

/// 查询短信记录响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeSmsRecordsResponse {
    /// 总记录数
    pub total: i32,
    /// 短信记录列表
    pub records: Vec<SmsRecord>,
    /// 请求ID
    pub request_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmsRecord {
    /// 任务ID
    pub task_id: String,
    /// 手机号
    pub phone_number: String,
    /// 模板ID
    pub template_id: String,
    /// 发送状态
    pub status: i32,
    /// 发送时间戳
    pub send_time: i64,
    /// 错误信息
    pub err_msg: Option<String>,
}

/// 描述扩展上传信息请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeExtensionUploadInfoRequest {
    /// 云环境ID
    pub env: String,
    /// 扩展模块
    pub extension: String,
}

/// 描述扩展上传信息响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeExtensionUploadInfoResponse {
    /// 上传信息列表
    pub files: Vec<ExtensionUploadInfo>,
    /// 请求ID
    pub request_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionUploadInfo {
    /// 文件ID
    pub file_id: String,
    /// 上传链接
    pub upload_url: String,
    /// 授权信息
    pub authorization: String,
    /// 文件存储路径
    pub cos_file_id: String,
}

/// 获取统计数据请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStatisticsRequest {
    /// 云环境ID
    pub env: String,
    /// 统计类型
    pub statistics_type: String,
    /// 开始时间戳，单位秒
    pub start_time: i64,
    /// 结束时间戳，单位秒
    pub end_time: i64,
}

/// 获取统计数据响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStatisticsResponse {
    /// 统计数据
    pub statistics: Value,
    /// 请求ID
    pub request_id: String,
}

/// 获取开放数据响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetOpenDataResponse {
    data: Vec<OpenDataItem>,
}

/// 开放数据项
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenDataItem {
    /// cloudID
    pub cloud_id: String,
    /// 开放数据JSON字符串
    pub data: String,
}

/// 获取实时语音签名请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetVoipSignRequest {
    /// 云环境ID
    pub env: String,
    /// 用户openid列表
    pub openid_list: Vec<String>,
    /// 签名有效期，单位秒
    pub expire_time: Option<i32>,
}

/// 获取实时语音签名响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetVoipSignResponse {
    /// 签名
    pub signature: String,
    /// 请求ID
    pub request_id: String,
}

/// 获取腾讯云API调用凭证请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetQcloudTokenRequest {
    /// 云环境ID
    pub env: String,
    /// 有效期，单位秒
    pub lifespan: Option<i32>,
}

/// 获取腾讯云API调用凭证响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetQcloudTokenResponse {
    /// 临时SecretId
    pub secret_id: String,
    /// 临时SecretKey
    pub secret_key: String,
    /// 临时Token
    pub token: String,
    /// 过期时间戳，单位秒
    pub expired_time: i64,
    /// 请求ID
    pub request_id: String,
}