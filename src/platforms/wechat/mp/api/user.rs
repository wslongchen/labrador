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
use serde_json::{json, Value};
use serde::{Serialize, Deserialize};
use crate::{AesEncryptor, AesMode};
use crate::errors::{LabradorResult, LabraError};
use crate::utils::encryption::base64_encode;
use crate::wechat::client::WechatApiResponse;
use crate::wechat::mp::WechatMpClient;

/// 用户管理模块（包含标签管理、黑名单、用户信息、openid转换）
#[derive(Debug, Clone)]
pub struct WechatMpUser<'a> {
    client: &'a WechatMpClient,
}

#[allow(unused)]
impl<'a> WechatMpUser<'a> {
    #[inline]
    pub fn new(client: &'a WechatMpClient) -> WechatMpUser<'a> {
        WechatMpUser { client }
    }

    // ==================== 标签管理 ====================

    /// 创建标签
    ///
    /// 本接口用于创建公众号标签。
    /// 一个公众号最多可以创建100个标签。
    pub async fn create_tag(&self, name: &str) -> LabradorResult<Tag> {
        let request = json!({
            "tag": { "name": name }
        });
        let response: WechatApiResponse<CreateTagResponse> = self.client.wechat_client()
            .post("/cgi-bin/tags/create", request)
            .await?;
        Ok(response.into_result()?.tag)
    }

    /// 获取标签列表
    ///
    /// 本接口用于获取公众号已创建的标签列表。
    pub async fn get_tags(&self) -> LabradorResult<Vec<Tag>> {
        let response: WechatApiResponse<TagListResponse> = self.client.wechat_client()
            .get("/cgi-bin/tags/get")
            .await?;
        Ok(response.into_result()?.tags)
    }

    /// 编辑标签
    ///
    /// 本接口用于修改已存在的标签信息。
    pub async fn update_tag(&self, tag_id: i32, name: &str) -> LabradorResult<WechatApiResponse> {
        let request = json!({
            "tag": { "id": tag_id, "name": name }
        });
        let response: WechatApiResponse = self.client.wechat_client()
            .post("/cgi-bin/tags/update", request)
            .await?;
        Ok(response)
    }

    /// 删除标签
    ///
    /// 本接口用于删除已存在的标签。
    /// 请注意，当某个标签下的粉丝超过10w时，后台不可直接删除标签。
    pub async fn delete_tag(&self, tag_id: i32) -> LabradorResult<WechatApiResponse> {
        let request = json!({
            "tag": { "id": tag_id }
        });
        let response: WechatApiResponse = self.client.wechat_client()
            .post("/cgi-bin/tags/delete", request)
            .await?;
        Ok(response)
    }

    /// 获取标签下粉丝列表
    ///
    /// 本接口用于获取标签下粉丝列表。
    /// 每次拉取最多10000个，可通过next_openid分批拉取。
    pub async fn get_tag_followers(&self, tag_id: i32, next_openid: Option<&str>) -> LabradorResult<TagFollowersResponse> {
        let mut request = json!({ "tagid": tag_id });
        if let Some(openid) = next_openid {
            request["next_openid"] = json!(openid);
        }
        let response: WechatApiResponse<TagFollowersResponse> = self.client.wechat_client()
            .post("/cgi-bin/user/tag/get", request)
            .await?;
        response.into_result()
    }

    /// 批量为用户打标签
    ///
    /// 本接口用于为指定的用户批量添加标签。
    /// 每次最多支持50个用户，标签ID必须合法。
    pub async fn batch_tagging(&self, tag_id: i32, openid_list: Vec<String>) -> LabradorResult<WechatApiResponse> {
        let request = json!({
            "tagid": tag_id,
            "openid_list": openid_list
        });
        let response: WechatApiResponse = self.client.wechat_client()
            .post("/cgi-bin/tags/members/batchtagging", request)
            .await?;
        Ok(response)
    }

    /// 批量为用户取消标签
    ///
    /// 本接口用于为指定的用户批量取消标签。
    /// 每次最多支持50个用户。
    pub async fn batch_untagging(&self, tag_id: i32, openid_list: Vec<String>) -> LabradorResult<WechatApiResponse> {
        let request = json!({
            "tagid": tag_id,
            "openid_list": openid_list
        });
        let response: WechatApiResponse = self.client.wechat_client()
            .post("/cgi-bin/tags/members/batchuntagging", request)
            .await?;
        Ok(response)
    }

    /// 获取用户身上的标签列表
    ///
    /// 本接口用于获取指定用户被添加的标签。
    pub async fn get_user_tag_ids(&self, openid: &str) -> LabradorResult<Vec<i32>> {
        let request = json!({ "openid": openid });
        let response: WechatApiResponse<UserTagResponse> = self.client.wechat_client()
            .post("/cgi-bin/tags/getidlist", request)
            .await?;
        Ok(response.into_result()?.tagid_list)
    }

    // ==================== 用户信息管理 ====================

    /// 获取用户基本信息（语言为默认的zh_CN 简体）
    pub async fn get(&self, openid: &str) -> LabradorResult<WechatUser> {
        self.get_with_lang(openid, "zh_CN").await
    }

    /// 获取用户基本信息（可指定语言）
    pub async fn get_with_lang(&self, openid: &str, lang: &str) -> LabradorResult<WechatUser> {
        let response: WechatApiResponse<WechatUser> = self.client.wechat_client()
            .get(&format!("/cgi-bin/user/info?openid={}&lang={}", openid, lang))
            .await?;
        response.into_result()
    }

    /// 批量获取用户信息
    ///
    /// 每次最多支持100个用户。
    pub async fn batch_get_user_info(&self, openids: Vec<String>, lang: Option<&str>) -> LabradorResult<Vec<UserInfo>> {
        let lang = lang.unwrap_or("zh_CN");
        let user_list: Vec<BatchUser> = openids.into_iter()
            .map(|openid| BatchUser {
                openid,
                lang: lang.to_string(),
            })
            .collect();

        let request = BatchRequest { user_list };
        let response: WechatApiResponse<BatchUserInfoResponse> = self.client.wechat_client()
            .post("/cgi-bin/user/info/batchget", serde_json::to_value(request).unwrap())
            .await?;
        Ok(response.into_result()?.user_info_list)
    }

    /// 获取用户列表
    ///
    /// 一次拉取调用最多拉取10000个关注者的OpenID。
    pub async fn get_user_list(&self, next_openid: Option<&str>) -> LabradorResult<UserListResponse> {
        let url = if let Some(openid) = next_openid {
            format!("/cgi-bin/user/get?next_openid={}", openid)
        } else {
            "/cgi-bin/user/get".to_string()
        };
        let response: WechatApiResponse<UserListResponse> = self.client.wechat_client()
            .get(&url)
            .await?;
        response.into_result()
    }

    /// 获取关注者列表（兼容原有方法）
    pub async fn get_followers(&self, next_openid: Option<&str>) -> LabradorResult<Followers> {
        let response: WechatApiResponse<GetFollowersResponse> = self.client.wechat_client()
            .get(&format!("/cgi-bin/user/get?next_openid={}", next_openid.unwrap_or("")))
            .await?;
        let data = response.into_result()?;
        Ok(Followers {
            total: data.total,
            count: data.count,
            openids: data.data.openid,
            next_openid: data.next_openid,
        })
    }

    /// 设置用户备注名
    pub async fn update_remark(&self, openid: &str, remark: &str) -> LabradorResult<WechatApiResponse> {
        let data = json!({
            "openid": openid,
            "remark": remark
        });
        let response: WechatApiResponse = self.client.wechat_client()
            .post("/cgi-bin/user/info/updateremark", data)
            .await?;
        Ok(response)
    }

    /// 获取分组编号（兼容旧接口）
    pub async fn get_group_id(&self, openid: &str) -> LabradorResult<u64> {
        let data = json!({ "openid": openid });
        let response: WechatApiResponse<GetGroupIdResponse> = self.client.wechat_client()
            .post("/cgi-bin/groups/getid", data)
            .await?;
        Ok(response.into_result()?.groupid)
    }

    // ==================== 黑名单管理 ====================

    /// 获取黑名单列表
    ///
    /// 本接口用于获取公众号的黑名单列表，每次拉取最多10000个。
    pub async fn get_blacklist(&self, begin_openid: Option<&str>) -> LabradorResult<BlacklistResponse> {
        let request = json!({
            "begin_openid": begin_openid.unwrap_or("")
        });
        let response: WechatApiResponse<BlacklistResponse> = self.client.wechat_client()
            .post("/cgi-bin/tags/members/getblacklist", request)
            .await?;
        response.into_result()
    }

    /// 批量拉黑用户
    ///
    /// 本接口用于将指定用户批量拉入黑名单，每次最多支持20个用户。
    pub async fn batch_blacklist(&self, openid_list: Vec<String>) -> LabradorResult<WechatApiResponse> {
        let request = json!({
            "openid_list": openid_list
        });
        let response: WechatApiResponse = self.client.wechat_client()
            .post("/cgi-bin/tags/members/batchblacklist", request)
            .await?;
        Ok(response)
    }

    /// 批量取消拉黑用户
    ///
    /// 本接口用于将指定用户批量移出黑名单，每次最多支持20个用户。
    pub async fn batch_unblacklist(&self, openid_list: Vec<String>) -> LabradorResult<WechatApiResponse> {
        let request = json!({
            "openid_list": openid_list
        });
        let response: WechatApiResponse = self.client.wechat_client()
            .post("/cgi-bin/tags/members/batchunblacklist", request)
            .await?;
        Ok(response)
    }

    // ==================== openid转换 ====================

    /// 将其他平台的openid转换为公众号的openid
    ///
    /// 本接口用于将其他平台（如小程序、APP、移动应用）的openid转换为当前公众号的openid。
    /// 适用于同一微信开放平台帐号下的移动应用、网站应用、小程序或公众号。
    pub async fn change_openid(&self, from_appid: &str, openid_list: Vec<String>) -> LabradorResult<Vec<OpenidMapping>> {
        let request = json!({
            "from_appid": from_appid,
            "openid_list": openid_list
        });
        let response: WechatApiResponse<ChangeOpenidResponse> = self.client.wechat_client()
            .post("/cgi-bin/changeopenid", request)
            .await?;
        Ok(response.into_result()?.result_list)
    }

    // ==================== 用户信息解密 ====================

    /// 解密用户信息（适用于小程序获取的用户信息）
    pub fn decrypt_user_info(&self, session_key: &str, encrypted_data: &str, iv: &str) -> LabradorResult<WechatUser> {
        let session_key = base64_encode(session_key.as_bytes());
        let iv = base64_encode(iv.as_bytes());
        let encrypted_data = base64_encode(encrypted_data.as_bytes());
        let encryptor = AesEncryptor::new(AesMode::Cbc, session_key.as_bytes(), iv.as_bytes())?;
        let decrypted = encryptor.decrypt(encrypted_data.as_bytes())?;
        match serde_json::from_slice::<Value>(&decrypted) {
            Ok(data) => {
                let openid = data["openId"].as_str().unwrap_or_default().to_owned();
                let nick_name = data["nickName"].as_str().unwrap_or_default().to_owned();
                let gender = data["gender"].as_u64().unwrap_or_default();
                let language = data["language"].as_str().unwrap_or_default().to_owned();
                let city = data["city"].as_str().unwrap_or_default().to_owned();
                let province = data["province"].as_str().unwrap_or_default().to_owned();
                let country = data["country"].as_str().unwrap_or_default().to_owned();
                let avatar = data["avatarUrl"].as_str().unwrap_or_default().to_owned();
                let unionid = data.get("unionId").and_then(|uid| uid.as_str()).map(|s| s.to_owned());

                Ok(WechatUser {
                    subscribe: false,
                    openid,
                    nickname: nick_name,
                    sex: gender as u8,
                    language,
                    city,
                    province,
                    country,
                    avatar,
                    subscribe_time: 0,
                    unionid,
                    remark: "".to_string(),
                    group_id: 0,
                })
            }
            Err(err) => Err(LabraError::Sign(err.to_string())),
        }
    }
}

// ============================================================================
// 请求与响应结构体
// ============================================================================

// -------------------- 标签管理 --------------------

/// 标签信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    /// 标签id，由微信分配
    pub id: i32,
    /// 标签名，UTF8编码
    pub name: String,
    /// 标签下粉丝数
    pub count: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
struct CreateTagResponse {
    tag: Tag,
}

#[derive(Debug, Clone, Deserialize)]
struct TagListResponse {
    tags: Vec<Tag>,
}

/// 标签下粉丝列表响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagFollowersResponse {
    /// 本次获取的粉丝数量
    pub count: i32,
    /// 粉丝数据
    pub data: TagFollowersData,
    /// 拉取列表最后一个用户的openid
    pub next_openid: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TagFollowersData {
    /// 粉丝 openid 列表
    pub openid: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct UserTagResponse {
    tagid_list: Vec<i32>,
}

// -------------------- 用户信息 --------------------

/// 用户信息（完整版，符合最新文档）
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    /// 用户的标识，对当前公众号唯一
    pub openid: String,
    /// 用户的昵称
    pub nickname: String,
    /// 用户的性别，值为1时是男性，值为2时是女性，值为0时是未知
    pub sex: i32,
    /// 用户所在城市
    pub city: String,
    /// 用户所在省份
    pub province: String,
    /// 用户所在国家
    pub country: String,
    /// 用户头像，最后一个数值代表正方形头像大小
    pub headimgurl: Option<String>,
    /// 用户特权信息，json 数组
    pub privilege: Vec<String>,
    /// 用户统一标识
    pub unionid: Option<String>,
    /// 用户是否订阅该公众号标识
    pub subscribe: Option<i32>,
    /// 用户关注时间，为时间戳
    pub subscribe_time: Option<i64>,
    /// 公众号运营者对粉丝的备注
    pub remark: Option<String>,
    /// 用户所在的分组ID（兼容旧的用户分组接口）
    pub groupid: Option<i32>,
    /// 用户被打上的标签ID列表
    pub tagid_list: Vec<i32>,
    /// 返回用户关注的渠道来源
    pub subscribe_scene: Option<String>,
    /// 二维码扫码场景
    pub qr_scene: Option<i64>,
    /// 二维码扫码场景描述
    pub qr_scene_str: Option<String>,
}

/// 原有 WechatUser 保留兼容性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatUser {
    pub subscribe: bool,
    pub openid: String,
    pub nickname: String,
    pub sex: u8,
    pub language: String,
    pub city: String,
    pub province: String,
    pub country: String,
    #[serde(rename = "headimgurl")]
    pub avatar: String,
    pub subscribe_time: u64,
    pub unionid: Option<String>,
    pub remark: String,
    #[serde(rename = "groupid")]
    pub group_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Followers {
    pub total: u64,
    pub count: u64,
    pub openids: Vec<String>,
    pub next_openid: String,
}

/// 批量获取用户信息响应
#[derive(Debug, Clone, Deserialize)]
pub struct BatchUserInfoResponse {
    pub user_info_list: Vec<UserInfo>,
}

/// 用户列表响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserListResponse {
    /// 关注该公众账号的总用户数
    pub total: i32,
    /// 拉取的OPENID个数，最大值为10000
    pub count: i32,
    /// 列表数据，OPENID的列表
    pub data: UserListData,
    /// 拉取列表的最后一个用户的OPENID
    pub next_openid: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserListData {
    /// OPENID列表
    pub openid: Vec<String>,
}

// -------------------- 黑名单管理 --------------------

/// 黑名单列表响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlacklistResponse {
    /// 总记录数
    pub total: i32,
    /// 本次获取数量
    pub count: i32,
    /// 黑名单数据
    pub data: BlacklistData,
    /// 拉取列表最后一个用户的openid
    pub next_openid: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BlacklistData {
    /// 黑名单 openid 列表
    pub openid: Vec<String>,
}

// -------------------- openid转换 --------------------

/// openid转换响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeOpenidResponse {
    pub result_list: Vec<OpenidMapping>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenidMapping {
    /// 转换前的openid
    pub from_openid: String,
    /// 转换后的openid
    pub to_openid: String,
    /// 错误码
    pub errcode: Option<i32>,
    /// 错误信息
    pub errmsg: Option<String>,
}

// -------------------- 辅助请求结构体 --------------------

#[derive(Debug, Serialize)]
struct BatchRequest {
    user_list: Vec<BatchUser>,
}

#[derive(Debug, Serialize)]
struct BatchUser {
    openid: String,
    lang: String,
}

#[derive(Debug, Clone, Deserialize)]
struct GetFollowersResponse {
    total: u64,
    count: u64,
    data: FollowersData,
    next_openid: String,
}

#[derive(Debug, Clone, Deserialize)]
struct FollowersData {
    openid: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct GetGroupIdResponse {
    groupid: u64,
}