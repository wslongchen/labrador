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

use crate::errors::LabradorResult;
use crate::wechat::client::WechatApiResponse;
use crate::wechat::miniapp::WechatMiniAppClient;

/// 小程序直播模块
///
/// 提供直播间管理、商品管理、成员角色管理及长期订阅等相关接口。
#[derive(Debug, Clone)]
pub struct WechatMxaLiveBroadcast<'a> {
    client: &'a WechatMiniAppClient,
}

#[allow(unused)]
impl<'a> WechatMxaLiveBroadcast<'a> {
    #[inline]
    pub fn new(client: &'a WechatMiniAppClient) -> Self {
        Self { client }
    }

    // ========================= 直播间管理 =========================

    /// 创建直播间
    ///
    /// 调用此接口创建直播间，创建成功后将在直播间列表展示。
    /// # 参数说明
    /// * `request` - 创建直播间所需的参数，详见 [`CreateRoomRequest`]
    ///
    /// # 返回
    /// 成功返回包含 `room_id` 的响应。
    pub async fn create_room(&self, request: &CreateRoomRequest) -> LabradorResult<CreateRoomResponse> {
        let response: WechatApiResponse<CreateRoomResponse> = self
            .client
            .wechat_client()
            .post("/wxaapi/broadcast/room/create", request)
            .await?;
        response.into_result()
    }

    /// 获取直播间列表和回放
    ///
    /// 该接口用于获取直播间列表及直播间信息。
    /// # 参数说明
    /// * `start` - 起始偏移量，从0开始
    /// * `limit` - 获取数量，最大30
    ///
    /// # 返回
    /// 包含直播间信息列表和总数的响应。
    pub async fn get_live_info(&self, start: i32, limit: i32) -> LabradorResult<LiveInfoResponse> {
        let response: WechatApiResponse<LiveInfoResponse> = self
            .client
            .wechat_client()
            .get(&format!("/wxa/business/getliveinfo?start={}&limit={}", start, limit))
            .await?;
        response.into_result()
    }

    /// 删除直播间
    ///
    /// 该接口用于删除直播间。
    /// # 参数说明
    /// * `room_id` - 直播间ID
    pub async fn delete_room(&self, room_id: i32) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/wxaapi/broadcast/room/deleteroom", serde_json::json!({ "roomId": room_id }))
            .await?;
        Ok(response)
    }

    /// 导入商品到直播间
    ///
    /// 该接口用于往指定直播间导入商品。
    /// # 参数说明
    /// * `request` - 导入商品参数，包含直播间ID和商品ID列表等。
    pub async fn add_goods_to_room(&self, request: &AddGoodsToRoomRequest) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/wxaapi/broadcast/room/addgoods", request)
            .await?;
        Ok(response)
    }

    /// 编辑直播间
    ///
    /// 该接口用于编辑直播间信息。
    /// # 参数说明
    /// * `request` - 编辑直播间参数，包含直播间ID及需修改的字段。
    pub async fn edit_room(&self, request: &EditRoomRequest) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/wxaapi/broadcast/room/editroom", request)
            .await?;
        Ok(response)
    }

    /// 获取直播间推流地址
    ///
    /// 该接口用于获取直播间推流地址。
    /// # 参数说明
    /// * `room_id` - 直播间ID
    ///
    /// # 返回
    /// 包含推流地址的响应。
    pub async fn get_push_url(&self, room_id: i32) -> LabradorResult<PushUrlResponse> {
        let response: WechatApiResponse<PushUrlResponse> = self
            .client
            .wechat_client()
            .get(&format!("/wxaapi/broadcast/room/getpushurl?roomId={}", room_id))
            .await?;
        response.into_result()
    }

    /// 获取直播间分享二维码
    ///
    /// 该接口用于获取直播间分享二维码。
    /// # 参数说明
    /// * `room_id` - 直播间ID
    /// * `params` - 自定义参数，可传入页面路径参数。
    ///
    /// # 返回
    /// 包含二维码图片二进制数据的响应（此处返回Bytes，需要进一步处理）。
    /// 注意：此接口返回的是图片流，而非JSON，需要特殊处理。
    /// 此处为简化，返回 `WechatApiResponse<Vec<u8>>` 可能不合适，实际使用时可能需要调整。
    /// 建议在 `wechat_client` 层增加一个直接返回 `Bytes` 的方法。
    pub async fn get_shared_code(&self, room_id: i32, params: Option<&str>) -> LabradorResult<Vec<u8>> {
        // 假设 wechat_client 有一个 get_bytes 方法
        let url = format!("/wxaapi/broadcast/room/getsharedcode?roomId={}", room_id);
        let url = if let Some(p) = params {
            format!("{}&params={}", url, p)
        } else {
            url
        };
        self.client.wechat_client().get_bytes(&url).await.map(|b| b.to_vec())
    }

    /// 获取主播副号
    ///
    /// 该接口用于获取主播副号。
    /// # 参数说明
    /// * `room_id` - 直播间ID
    pub async fn get_sub_anchor(&self, room_id: i32) -> LabradorResult<SubAnchorResponse> {
        let response: WechatApiResponse<SubAnchorResponse> = self
            .client
            .wechat_client()
            .get(&format!("/wxaapi/broadcast/room/getsubanchor?roomId={}", room_id))
            .await?;
        response.into_result()
    }

    /// 修改主播副号
    ///
    /// 该接口用于修改主播副号。
    /// # 参数说明
    /// * `room_id` - 直播间ID
    /// * `username` - 副号微信号
    pub async fn modify_sub_anchor(&self, room_id: i32, username: &str) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/wxaapi/broadcast/room/modifysubanchor", serde_json::json!({
                "roomId": room_id,
                "username": username
            }))
            .await?;
        Ok(response)
    }

    /// 删除主播副号
    ///
    /// 该接口用于删除主播副号。
    /// # 参数说明
    /// * `room_id` - 直播间ID
    pub async fn delete_sub_anchor(&self, room_id: i32) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/wxaapi/broadcast/room/deletesubanchor", serde_json::json!({ "roomId": room_id }))
            .await?;
        Ok(response)
    }

    /// 添加主播副号
    ///
    /// 该接口用于添加主播副号。
    /// # 参数说明
    /// * `room_id` - 直播间ID
    /// * `username` - 副号微信号
    pub async fn add_sub_anchor(&self, room_id: i32, username: &str) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/wxaapi/broadcast/room/addsubanchor", serde_json::json!({
                "roomId": room_id,
                "username": username
            }))
            .await?;
        Ok(response)
    }

    /// 删除直播间商品
    ///
    /// 该接口用于删除直播间内已导入的商品。
    /// # 参数说明
    /// * `room_id` - 直播间ID
    /// * `goods_id` - 商品ID
    pub async fn delete_goods_from_room(&self, room_id: i32, goods_id: i32) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/wxaapi/broadcast/goods/deleteInRoom", serde_json::json!({
                "roomId": room_id,
                "goodsId": goods_id
            }))
            .await?;
        Ok(response)
    }

    /// 推送商品到直播间
    ///
    /// 该接口用于将商品推送至直播间，观众端会收到推送消息。
    /// # 参数说明
    /// * `room_id` - 直播间ID
    /// * `goods_id` - 商品ID
    pub async fn push_goods(&self, room_id: i32, goods_id: i32) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/wxaapi/broadcast/goods/push", serde_json::json!({
                "roomId": room_id,
                "goodsId": goods_id
            }))
            .await?;
        Ok(response)
    }

    /// 上下架商品
    ///
    /// 该接口用于在直播间内上架或下架商品。
    /// # 参数说明
    /// * `room_id` - 直播间ID
    /// * `goods_id` - 商品ID
    /// * `on_sale` - 上架状态：1-上架，0-下架
    pub async fn set_goods_on_sale(&self, room_id: i32, goods_id: i32, on_sale: i32) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/wxaapi/broadcast/goods/onsale", serde_json::json!({
                "roomId": room_id,
                "goodsId": goods_id,
                "onSale": on_sale
            }))
            .await?;
        Ok(response)
    }

    /// 直播间商品排序
    ///
    /// 该接口用于调整直播间内商品的展示顺序。
    /// # 参数说明
    /// * `request` - 排序参数，包含直播间ID和排序后的商品ID列表。
    pub async fn sort_goods(&self, request: &SortGoodsRequest) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/wxaapi/broadcast/goods/sort", request)
            .await?;
        Ok(response)
    }

    /// 修改直播间小助手
    ///
    /// 该接口用于修改直播间小助手信息。
    /// # 参数说明
    /// * `request` - 修改小助手参数。
    pub async fn modify_assistant(&self, request: &ModifyAssistantRequest) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/wxaapi/broadcast/room/modifyassistant", request)
            .await?;
        Ok(response)
    }

    /// 查询直播间小助手列表
    ///
    /// 该接口用于查询直播间小助手列表。
    /// # 参数说明
    /// * `room_id` - 直播间ID
    pub async fn get_assistant_list(&self, room_id: i32) -> LabradorResult<AssistantListResponse> {
        let response: WechatApiResponse<AssistantListResponse> = self
            .client
            .wechat_client()
            .get(&format!("/wxaapi/broadcast/room/getassistantlist?roomId={}", room_id))
            .await?;
        response.into_result()
    }

    /// 删除直播间小助手
    ///
    /// 该接口用于删除直播间指定小助手。
    /// # 参数说明
    /// * `room_id` - 直播间ID
    /// * `username` - 小助手微信号
    pub async fn remove_assistant(&self, room_id: i32, username: &str) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/wxaapi/broadcast/room/removeassistant", serde_json::json!({
                "roomId": room_id,
                "username": username
            }))
            .await?;
        Ok(response)
    }

    /// 添加直播间小助手
    ///
    /// 该接口用于添加直播间小助手。
    /// # 参数说明
    /// * `room_id` - 直播间ID
    /// * `username` - 小助手微信号
    pub async fn add_assistant(&self, room_id: i32, username: &str) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/wxaapi/broadcast/room/addassistant", serde_json::json!({
                "roomId": room_id,
                "username": username
            }))
            .await?;
        Ok(response)
    }

    /// 开启/关闭直播间全局禁言
    ///
    /// # 参数说明
    /// * `room_id` - 直播间ID
    /// * `ban_comment` - 0-开启禁言，1-关闭禁言（或相反，请以官方文档为准）
    pub async fn update_comment(&self, room_id: i32, ban_comment: i32) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/wxaapi/broadcast/room/updatecomment", serde_json::json!({
                "roomId": room_id,
                "banComment": ban_comment
            }))
            .await?;
        Ok(response)
    }

    /// 开启/关闭直播间官方收录
    ///
    /// # 参数说明
    /// * `room_id` - 直播间ID
    /// * `is_feed_public` - 0-关闭，1-开启
    pub async fn update_feed_public(&self, room_id: i32, is_feed_public: i32) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/wxaapi/broadcast/room/updatefeedpublic", serde_json::json!({
                "roomId": room_id,
                "isFeedPublic": is_feed_public
            }))
            .await?;
        Ok(response)
    }

    /// 开启/关闭客服功能
    ///
    /// # 参数说明
    /// * `room_id` - 直播间ID
    /// * `close_kf` - 0-关闭，1-开启（或相反，请以官方文档为准）
    pub async fn update_kf(&self, room_id: i32, close_kf: i32) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/wxaapi/broadcast/room/updatekf", serde_json::json!({
                "roomId": room_id,
                "closeKf": close_kf
            }))
            .await?;
        Ok(response)
    }

    /// 开启/关闭回放功能
    ///
    /// # 参数说明
    /// * `room_id` - 直播间ID
    /// * `close_replay` - 0-关闭，1-开启（或相反，请以官方文档为准）
    pub async fn update_replay(&self, room_id: i32, close_replay: i32) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/wxaapi/broadcast/room/updatereplay", serde_json::json!({
                "roomId": room_id,
                "closeReplay": close_replay
            }))
            .await?;
        Ok(response)
    }

    /// 下载商品讲解视频
    ///
    /// # 参数说明
    /// * `room_id` - 直播间ID
    /// * `goods_id` - 商品ID
    ///
    /// # 返回
    /// 包含视频下载地址等信息的响应。
    pub async fn get_goods_video(&self, room_id: i32, goods_id: i32) -> LabradorResult<GoodsVideoResponse> {
        let response: WechatApiResponse<GoodsVideoResponse> = self
            .client
            .wechat_client()
            .get(&format!("/wxaapi/broadcast/goods/getVideo?roomId={}&goodsId={}", room_id, goods_id))
            .await?;
        response.into_result()
    }

    // ========================= 商品管理 =========================

    /// 添加并提审商品
    ///
    /// 调用此接口上传并提审需要直播的商品信息，审核通过后商品录入【小程序直播】商品库。
    pub async fn add_goods(&self, request: &AddGoodsRequest) -> LabradorResult<AddGoodsResponse> {
        let response: WechatApiResponse<AddGoodsResponse> = self
            .client
            .wechat_client()
            .post("/wxaapi/broadcast/goods/add", request)
            .await?;
        response.into_result()
    }

    /// 重新提交商品审核
    ///
    /// 调用此接口可以对已撤回提审的商品再次发起提审申请。
    /// # 参数说明
    /// * `goods_id` - 商品ID
    pub async fn audit_goods(&self, goods_id: i32) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/wxaapi/broadcast/goods/audit", serde_json::json!({ "goodsId": goods_id }))
            .await?;
        Ok(response)
    }

    /// 获取商品的信息与审核状态
    ///
    /// 该接口用于获取商品的信息与审核状态。
    /// # 参数说明
    /// * `goods_id` - 商品ID数组，最多10个
    pub async fn get_goods_warehouse(&self, goods_ids: &[i32]) -> LabradorResult<Vec<GoodsInfo>> {
        let response: WechatApiResponse<GoodsWarehouseResponse> = self
            .client
            .wechat_client()
            .post("/wxa/business/getgoodswarehouse", serde_json::json!({ "goodsIds": goods_ids }))
            .await?;
        response.into_result().map(|r| r.goods)
    }

    /// 撤回商品审核
    ///
    /// 该接口用于撤回商品审核，消耗的提审次数不返还。
    /// # 参数说明
    /// * `goods_id` - 商品ID
    pub async fn reset_audit_goods(&self, goods_id: i32) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/wxaapi/broadcast/goods/resetaudit", serde_json::json!({ "goodsId": goods_id }))
            .await?;
        Ok(response)
    }

    /// 更新商品信息
    ///
    /// 调用此接口可以更新商品信息。
    /// 审核通过的商品仅允许更新价格类型与价格，审核中的商品不允许更新，未审核的商品允许更新所有字段。
    /// # 注意
    /// 只传入需要更新的字段。
    pub async fn update_goods(&self, request: &UpdateGoodsRequest) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/wxaapi/broadcast/goods/update", request)
            .await?;
        Ok(response)
    }

    /// 获取商品列表
    ///
    /// 该接口用于获取不同审核状态的商品信息。
    /// # 参数说明
    /// * `status` - 商品状态，0：未审核，1：审核中，2：审核通过，3：审核驳回
    /// * `offset` - 起始偏移量
    /// * `limit` - 获取数量，最大30
    pub async fn get_approved_goods(&self, status: i32, offset: i32, limit: i32) -> LabradorResult<ApprovedGoodsResponse> {
        let response: WechatApiResponse<ApprovedGoodsResponse> = self
            .client
            .wechat_client()
            .get(&format!("/wxaapi/broadcast/goods/getapproved?status={}&offset={}&limit={}", status, offset, limit))
            .await?;
        response.into_result()
    }

    /// 删除商品
    ///
    /// 调用此接口，可删除【小程序直播】商品库中的商品，删除后直播间上架的该商品也将被同步删除，不可恢复。
    /// # 参数说明
    /// * `goods_id` - 商品ID
    pub async fn delete_goods(&self, goods_id: i32) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/wxaapi/broadcast/goods/delete", serde_json::json!({ "goodsId": goods_id }))
            .await?;
        Ok(response)
    }

    // ========================= 成员管理 =========================

    /// 设置成员角色
    ///
    /// 调用此接口设置小程序直播成员的管理员、运营者和主播角色。
    /// # 参数说明
    /// * `username` - 成员微信号
    /// * `role` - 角色，2-主播，3-运营者，4-管理员
    pub async fn add_role(&self, username: &str, role: i32) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/wxaapi/broadcast/role/addrole", serde_json::json!({
                "username": username,
                "role": role
            }))
            .await?;
        Ok(response)
    }

    /// 移除成员角色
    ///
    /// 调用此接口可移除小程序直播成员的管理员、运营者和主播角色。
    /// # 参数说明
    /// * `username` - 成员微信号
    /// * `role` - 角色，2-主播，3-运营者，4-管理员
    pub async fn delete_role(&self, username: &str, role: i32) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/wxaapi/broadcast/role/deleterole", serde_json::json!({
                "username": username,
                "role": role
            }))
            .await?;
        Ok(response)
    }

    /// 查询成员列表
    ///
    /// 该接口用于查询小程序直播成员列表。
    /// # 参数说明
    /// * `role` - 角色，2-主播，3-运营者，4-管理员
    /// * `offset` - 起始偏移量
    /// * `limit` - 获取数量，最大30
    pub async fn get_role_list(&self, role: i32, offset: i32, limit: i32) -> LabradorResult<RoleListResponse> {
        let response: WechatApiResponse<RoleListResponse> = self
            .client
            .wechat_client()
            .get(&format!("/wxaapi/broadcast/role/getrolelist?role={}&offset={}&limit={}", role, offset, limit))
            .await?;
        response.into_result()
    }

    // ========================= 长期订阅 =========================

    /// 发送直播开始事件
    ///
    /// 该接口用于向长期订阅用户群发直播间开始事件。
    /// # 参数说明
    /// * `room_id` - 直播间ID
    pub async fn push_message(&self, room_id: i32) -> LabradorResult<WechatApiResponse> {
        let response: WechatApiResponse = self
            .client
            .wechat_client()
            .post("/wxa/business/push_message", serde_json::json!({ "room_id": room_id }))
            .await?;
        Ok(response)
    }

    /// 获取长期订阅用户列表
    ///
    /// 该接口用于获取长期订阅用户列表。
    /// # 参数说明
    /// * `request` - 包含分页参数和可选的上次拉取时间戳。
    pub async fn get_wxa_followers(&self, request: &GetWxaFollowersRequest) -> LabradorResult<WxaFollowersResponse> {
        let response: WechatApiResponse<WxaFollowersResponse> = self
            .client
            .wechat_client()
            .post("/wxa/business/get_wxa_followers", request)
            .await?;
        response.into_result()
    }
}

// ============================================================================
// 请求与响应结构体 (按模块分组，并包含详细字段注释)
// 注意：使用 #[serde(rename_all = "camelCase")] 处理驼峰字段，符合 Rust 命名规范。
// ============================================================================

// -------------------- 直播间管理 结构体 --------------------

/// 创建直播间请求参数
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRoomRequest {
    /// 直播间名称，最长30个字符
    pub name: String,
    /// 背景图 ID，可通过素材管理接口获得
    pub cover_img: i32,
    /// 主播昵称，最长15个字符
    pub anchor_name: String,
    /// 主播微信号，不是微信昵称
    pub anchor_wechat: String,
    /// 直播间类型：1-推流，0-手机直播
    pub r#type: i32,
    /// 是否关闭点赞：0-开启，1-关闭
    pub close_like: i32,
    /// 是否关闭货架：0-开启，1-关闭
    pub close_goods: i32,
    /// 是否关闭评论：0-开启，1-关闭
    pub close_comment: i32,
    /// 是否关闭回放：0-开启，1-关闭
    pub close_replay: i32,
    /// 是否关闭分享：0-开启，1-关闭
    pub close_share: i32,
    /// 是否关闭客服：0-开启，1-关闭
    pub close_kf: i32,
    /// 计划开始时间戳，单位秒
    pub start_time: i64,
    /// 计划结束时间戳，单位秒
    pub end_time: i64,
    /// 主播副号微信号（可选）
    pub sub_anchor_wechat: Option<String>,
    /// 直播间创建后是否自动开始：0-否，1-是（可选）
    pub is_auto_start: Option<i32>,
}

/// 创建直播间响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRoomResponse {
    /// 直播间ID
    pub room_id: i32,
}

/// 获取直播间列表响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveInfoResponse {
    /// 总房间数
    pub total: i32,
    /// 直播间信息列表
    pub room_info: Vec<RoomInfo>,
}

/// 直播间信息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomInfo {
    /// 直播间名称
    pub name: String,
    /// 直播间ID
    pub room_id: i32,
    /// 直播间封面图URL
    pub cover_img: String,
    /// 主播昵称
    pub anchor_name: String,
    /// 主播微信号
    pub anchor_wechat: String,
    /// 主播副号微信号
    pub sub_anchor_wechat: Option<String>,
    /// 直播间状态
    pub live_status: i32,
    /// 直播间类型
    pub live_type: i32,
    /// 计划开始时间
    pub start_time: i64,
    /// 计划结束时间
    pub end_time: i64,
    /// 是否开启点赞
    pub close_like: i32,
    /// 是否开启评论
    pub close_comment: i32,
    /// 是否开启货架
    pub close_goods: i32,
    /// 是否开启回放
    pub close_replay: i32,
    /// 是否开启客服
    pub close_kf: i32,
    /// 是否开启分享
    pub close_share: i32,
    /// 直播间回放URL列表
    pub live_replay: Option<Vec<LiveReplay>>,
    /// 创建时间
    pub create_time: i64,
    /// 修改时间
    pub modify_time: i64,
    /// 直播间推广二维码图片URL
    pub qrcode_url: Option<String>,
    /// 直播间小程序path
    pub wx_live_path: Option<String>,
}

/// 直播间回放信息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveReplay {
    /// 回放视频过期时间
    pub expire_time: i64,
    /// 回放视频URL
    pub video_url: String,
}

/// 导入商品到直播间请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddGoodsToRoomRequest {
    /// 直播间ID
    pub room_id: i32,
    /// 商品ID列表，单次最多20个
    pub goods_ids: Vec<i32>,
}

/// 编辑直播间请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditRoomRequest {
    /// 直播间ID
    pub room_id: i32,
    /// 直播间名称（可选）
    pub name: Option<String>,
    /// 背景图ID（可选）
    pub cover_img: Option<i32>,
    /// 主播昵称（可选）
    pub anchor_name: Option<String>,
    /// 主播微信号（可选）
    pub anchor_wechat: Option<String>,
    /// 主播副号微信号（可选）
    pub sub_anchor_wechat: Option<String>,
    /// 计划开始时间戳（可选）
    pub start_time: Option<i64>,
    /// 计划结束时间戳（可选）
    pub end_time: Option<i64>,
    /// 是否关闭点赞（可选）
    pub close_like: Option<i32>,
    /// 是否关闭货架（可选）
    pub close_goods: Option<i32>,
    /// 是否关闭评论（可选）
    pub close_comment: Option<i32>,
    /// 是否关闭回放（可选）
    pub close_replay: Option<i32>,
    /// 是否关闭分享（可选）
    pub close_share: Option<i32>,
    /// 是否关闭客服（可选）
    pub close_kf: Option<i32>,
}

/// 获取推流地址响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PushUrlResponse {
    /// 推流地址
    pub push_url: String,
    /// 拉流地址
    pub pull_url: String,
}

/// 获取主播副号响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAnchorResponse {
    /// 主播副号微信号
    pub username: String,
}

/// 商品排序请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SortGoodsRequest {
    /// 直播间ID
    pub room_id: i32,
    /// 排序后的商品ID列表
    pub goods_ids: Vec<i32>,
}

/// 修改小助手请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyAssistantRequest {
    /// 直播间ID
    pub room_id: i32,
    /// 小助手微信号
    pub username: String,
    /// 昵称
    pub nickname: String,
}

/// 小助手列表响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssistantListResponse {
    /// 小助手列表
    pub assistant_list: Vec<AssistantInfo>,
}

/// 小助手信息
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssistantInfo {
    /// 小助手微信号
    pub username: String,
    /// 昵称
    pub nickname: String,
}

/// 商品讲解视频响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodsVideoResponse {
    /// 视频URL
    pub video_url: String,
}

// -------------------- 商品管理 结构体 --------------------

/// 添加商品请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddGoodsRequest {
    /// 商品名称，最长20个字符
    pub name: String,
    /// 商品缩略图ID，通过素材管理接口获得
    pub cover_img: i32,
    /// 商品价格类型：1-一口价，2-区间价
    pub price_type: i32,
    /// 商品价格（分为单位），price_type=1时必填
    pub price: Option<i32>,
    /// 商品最低价格（分为单位），price_type=2时必填
    pub price2: Option<i32>,
    /// 商品详情页小程序path
    pub url: String,
}

/// 添加商品响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddGoodsResponse {
    /// 商品ID
    pub goods_id: i32,
}

/// 商品信息（用于多个响应中）
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodsInfo {
    /// 商品ID
    pub goods_id: i32,
    /// 商品名称
    pub name: String,
    /// 商品缩略图URL
    pub cover_img: String,
    /// 商品详情页path
    pub url: String,
    /// 价格类型
    pub price_type: i32,
    /// 一口价价格
    pub price: Option<i32>,
    /// 区间价最低价
    pub price2: Option<i32>,
    /// 商品审核状态
    pub audit_status: i32,
    /// 审核失败原因
    pub third_party_tag: Option<String>,
    /// 商品是否已删除
    pub is_deleted: bool,
}

/// 商品库信息响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GoodsWarehouseResponse {
    goods: Vec<GoodsInfo>,
}

/// 更新商品请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGoodsRequest {
    /// 商品ID
    pub goods_id: i32,
    /// 商品名称（可选）
    pub name: Option<String>,
    /// 商品缩略图ID（可选）
    pub cover_img: Option<i32>,
    /// 商品价格类型（可选）
    pub price_type: Option<i32>,
    /// 一口价价格（可选）
    pub price: Option<i32>,
    /// 区间价最低价（可选）
    pub price2: Option<i32>,
    /// 商品详情页path（可选）
    pub url: Option<String>,
}

/// 已审核商品列表响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApprovedGoodsResponse {
    /// 总数
    pub total: i32,
    /// 商品列表
    pub goods: Vec<GoodsInfo>,
}

// -------------------- 成员管理 结构体 --------------------

/// 成员列表响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleListResponse {
    /// 总数
    pub total: i32,
    /// 成员微信号列表
    pub list: Vec<String>,
}

// -------------------- 长期订阅 结构体 --------------------

/// 获取长期订阅用户列表请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWxaFollowersRequest {
    /// 上次拉取列表的时间戳，用于分页
    pub last_time: Option<i64>,
    /// 每页数量，最大10000
    pub limit: Option<i32>,
    /// 分页起始位置，从0开始
    pub offset: Option<i32>,
}

/// 长期订阅用户列表响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WxaFollowersResponse {
    /// 用户总数
    pub total: i32,
    /// 用户openid列表
    pub followers: Vec<String>,
}