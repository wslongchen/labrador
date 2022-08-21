use std::collections::HashMap;
use serde_json::{json, Value};

use serde::{Serialize, Deserialize};

use crate::{session::SessionStore, errors::LabraError, wechat::{cryptos::WeChatCrypto}, request::RequestType, WechatCommonResponse, WeChatMpClient, LabradorResult};
use crate::wechat::mp::method::{UserMethod, WechatMpMethod};


#[derive(Debug, Clone)]
pub struct WeChatUser<'a, T: SessionStore> {
    client: &'a WeChatMpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WeChatUser<'a, T> {

    #[inline]
    pub fn new(client: &WeChatMpClient<T>) -> WeChatUser<T> {
        WeChatUser {
            client,
        }
    }

    /// 获取用户信息
    pub async fn get(&mut self, openid: &str) -> LabradorResult<WechatUser> {
        self.get_with_lang(openid, "zh_CN").await
    }

    /// 获取用户信息
    pub async fn get_with_lang(&mut self, openid: &str, lang: &str) -> LabradorResult<WechatUser> {
        let res = self.client.get(WechatMpMethod::User(UserMethod::Info), vec![("openid", openid), ("lang", lang)], RequestType::Json).await?.json::<serde_json::Value>()?;
        let result = WechatCommonResponse::from_value(res.clone())?;
        if result.is_success() {
            Ok(self.json_to_user(&res))
        } else {
            Err(LabraError::ClientError {errcode: result.errcode.to_owned().unwrap_or_default().to_string(), errmsg: result.errmsg.to_owned().unwrap_or_default()})
        }
    }

    /// 解密用户信息
    pub fn decrypt_user_info(&self, session_key: &str, encrypted_data: &str, iv: &str) -> LabradorResult<WechatUser> {
        let res = WeChatCrypto::decrypt_data(session_key, encrypted_data, iv)?;
        match serde_json::from_str::<Value>(res.as_str()) {
            Ok(data) => {
                let openid = &data["openId"];
                let openid = openid.as_str().unwrap_or_default().to_owned();
                let nick_name = &data["nickName"];
                let nick_name = nick_name.as_str().unwrap_or_default().to_owned();
                let gender = &data["gender"];
                let gender = gender.as_u64().unwrap_or_default();
                let language = &data["language"];
                let language = language.as_str().unwrap_or_default().to_owned();
                let city = &data["city"];
                let city = city.as_str().unwrap_or_default().to_owned();
                
                let province = &data["province"];
                let province = province.as_str().unwrap_or_default().to_owned();
                
                let country = &data["country"];
                let country = country.as_str().unwrap_or_default().to_owned();
                
                let avatar = &data["avatarUrl"];
                let avatar = avatar.as_str().unwrap_or_default().to_owned();
                let unionid = match data.get("unionId") {
                    Some(ref uid) => {
                        let _uid = uid.as_str().unwrap_or_default().to_owned();
                        Some(_uid.to_owned())
                    },
                    None => None,
                };
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
                
            },
            Err(err) => Err(LabraError::InvalidSignature(err.to_string())),
        }
        
    }

    /// 修改备注
    pub async fn update_remark(&mut self, openid: &str, remark: &str) -> LabradorResult<WechatCommonResponse> {
        let data = json!({
            "openid": openid.to_owned(),
            "remark": remark.to_owned()
        });
        self.client.post(WechatMpMethod::User(UserMethod::UpdateRemark), vec![], data, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// 获取关注的人
    pub async fn get_followers(&mut self, next_openid: Option<&str>) -> LabradorResult<Followers> {
        let params = match next_openid {
            Some(openid) => vec![("next_openid", openid)],
            None => vec![],
        };
        let res = self.client.get(WechatMpMethod::User(UserMethod::Get), params, RequestType::Json, ).await?.json::<serde_json::Value>()?;
        let mut result = WechatCommonResponse::from_value(res.clone())?;
        if result.is_success() {
            let total = &res["total"];
            let total = total.as_u64().unwrap_or_default();
            let count = &res["count"];
            let count = count.as_u64().unwrap_or_default();
            let next_id = &res["next_openid"];
            let next_id = next_id.as_str().unwrap_or_default().to_owned();
            let s = res.as_object().unwrap();
            // res.find_path(&["data", "openid"])
            let openids = match res["data"].as_object() {
                Some(data) => {
                    if let Some(ids) = data.get("openid") {
                        let openids_array = ids.as_array().unwrap();
                        openids_array.iter()
                            .map(|x| x.as_str().unwrap_or_default().to_owned())
                            .collect::<Vec<String>>()
                    }else {
                        vec![]
                    }

                },
                None => vec![],
            };
            Ok(Followers {
                total,
                count,
                openids,
                next_openid: next_id.to_owned(),
            })
        } else {
            Err(LabraError::ClientError {errcode: result.errcode.to_owned().unwrap_or_default().to_string(), errmsg: result.errmsg.to_owned().unwrap_or_default()})
        }
    }

    /// 获取分组编号
    pub async fn get_group_id(&mut self, openid: &str) -> LabradorResult<u64> {
        let data = json!({
            "openid": openid.to_owned()
        });
        let res = self.client.post(WechatMpMethod::User(UserMethod::GetGroupId), vec![], data, RequestType::Json).await?.json::<serde_json::Value>()?;
        let group_id = &res["groupid"];
        let group_id = group_id.as_u64().unwrap_or_default();
        let mut result = WechatCommonResponse::from_value(res.clone())?;
        if result.is_success() {
            Ok(group_id)
        } else {
            Err(LabraError::ClientError {errcode: result.errcode.to_owned().unwrap_or_default().to_string(), errmsg: result.errmsg.to_owned().unwrap_or_default()})
        }
    }

    fn json_to_user(&self, res: &Value) -> WechatUser {
        let _subscribe = &res["subscribe"];
        let subscribe = match _subscribe.as_u64().unwrap_or_default() {
            1 => true,
            0 => false,
            _ => unreachable!(),
        };
        let openid = &res["openid"];
        let openid = openid.as_str().unwrap_or_default().to_owned();
        let nickname = &res["nickname"];
        let nickname = nickname.as_str().unwrap_or_default().to_owned();
        let sex = &res["sex"];
        let sex = sex.as_u64().unwrap_or_default();
        let language = &res["language"];
        let language = language.as_str().unwrap_or_default().to_owned();
        let city = &res["city"];
        let city = city.as_str().unwrap_or_default().to_owned();
        let province = &res["province"];
        let province = province.as_str().unwrap_or_default().to_owned();
        let country = &res["country"];
        let country = country.as_str().unwrap_or_default().to_owned();
        let avatar = &res["headimgurl"];
        let avatar = avatar.as_str().unwrap_or_default().to_owned();
        let subscribe_time = &res["subscribe_time"];
        let subscribe_time = subscribe_time.as_u64().unwrap_or_default();
        let unionid = match res.get("unionid") {
            Some(ref uid) => {
                let _uid = uid.as_str().unwrap_or_default().to_owned();
                Some(_uid.to_owned())
            },
            None => None,
        };
        let remark = &res["remark"];
        let remark = remark.as_str().unwrap_or_default().to_owned();
        let group_id = &res["groupid"];
        let group_id = group_id.as_u64().unwrap_or_default();
        WechatUser {
            subscribe,
            openid: openid.to_owned(),
            nickname: nickname.to_owned(),
            sex: sex as u8,
            language: language.to_owned(),
            city: city.to_owned(),
            province: province.to_owned(),
            country: country.to_owned(),
            avatar: avatar.to_owned(),
            subscribe_time,
            unionid,
            remark: remark.to_owned(),
            group_id,
        }
    }

    /// 批量获取用户
    pub async fn get_batch(&mut self, user_list: &[HashMap<String, String>]) -> LabradorResult<Vec<WechatUser>> {
        let data = json!({
            "user_list": user_list.to_vec()
        });
        let res = self.client.post(WechatMpMethod::User(UserMethod::GetBatch), vec![], data, RequestType::Json).await?.json::<serde_json::Value>()?;
        let mut result = WechatCommonResponse::from_value(res.clone())?;
        if result.is_success() {
            let info_list = &res["user_info_list"];
            let info_list = info_list.as_array().unwrap();
            let mut users = vec![];
            for info in info_list {
                users.push(self.json_to_user(&info));
            }
            Ok(users)
        } else {
            Err(LabraError::ClientError {errcode: result.errcode.to_owned().unwrap_or_default().to_string(), errmsg: result.errmsg.to_owned().unwrap_or_default()})
        }
    }

    /// 批量获取用户
    pub async fn get_batch_with_lang(&mut self, user_list: &[String], lang: &str) -> LabradorResult<Vec<WechatUser>> {
        let mut users = vec![];
        for openid in user_list {
            let mut user = HashMap::new();
            user.insert("openid".to_owned(), openid.to_owned());
            user.insert("lang".to_owned(), lang.to_owned());
            users.push(user);
        }
        self.get_batch(&users).await
    }
}

//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WechatUser {
    pub subscribe: bool,
    pub openid: String,
    pub nickname: String,
    pub sex: u8,
    pub language: String,
    pub city: String,
    pub province: String,
    pub country: String,
    pub avatar: String,
    pub subscribe_time: u64,
    pub unionid: Option<String>,
    pub remark: String,
    pub group_id: u64,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Followers {
    pub total: u64,
    pub count: u64,
    pub openids: Vec<String>,
    pub next_openid: String,
}
