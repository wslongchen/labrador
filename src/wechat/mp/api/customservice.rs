use std::vec;

use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

use crate::{session::SessionStore, request::{RequestType}, errors::LabraError, WechatCommonResponse, WechatMpClient, LabradorResult};
use crate::util::md5::md5;
use crate::wechat::mp::method::{MpCustomServiceMethod, WechatMpMethod};

/// 客服接口.
#[derive(Debug, Clone)]
pub struct WechatMpCustomService<'a, T: SessionStore> {
    client: &'a WechatMpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WechatMpCustomService<'a, T> {

    #[inline]
    pub fn new(client: &WechatMpClient<T>) -> WechatMpCustomService<T> {
        WechatMpCustomService {
            client,
        }
    }

    /// <pre>
    /// 发送客服消息
    /// 详情请见: <a href="https://developers.weixin.qq.com/doc/offiaccount/Message_Management/Service_Center_messages.html">发送客服消息</a>
    /// 接口url格式：https://api.weixin.qq.com/cgi-bin/message/custom/send?access_token=ACCESS_TOKEN
    /// </pre>
    pub async fn send_kefu_message<D: Serialize>(&self, data: D) -> LabradorResult<WechatCommonResponse> {
        self.client.post(WechatMpMethod::CustomService(MpCustomServiceMethod::CustomSend), vec![], data, RequestType::Json).await?.json::<WechatCommonResponse>()
    }


    /// 客服接口 - 发送文字消息
    pub async fn send_text(&self, openid: &str, content: &str) -> LabradorResult<WechatCommonResponse> {
        let req = SendTextRequest::new(openid, content);
        self.send_kefu_message(req.to_json()).await
    }

    /// 客服接口 - 发送图片消息
    pub async fn send_image(&self, openid: &str, media_id: &str) -> LabradorResult<WechatCommonResponse> {
        let req = SendImageRequest::new(openid, media_id);
        self.send_kefu_message(req.to_json()).await
    }

    /// 客服接口 - 发送声音消息
    pub async fn send_voice(&self, openid: &str, media_id: &str) -> LabradorResult<WechatCommonResponse> {
        let req = SendVoiceRequest::new(openid, media_id);
        self.send_kefu_message(req.to_json()).await
    }


    //*******************客服管理接口***********************//

    /// <pre>
    /// 添加客服账号
    /// 详情请见：<a href="http://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1458044813&token=&lang=zh_CN">客服管理</a>
    /// 接口url格式：https://api.weixin.qq.com/customservice/kfaccount/add?access_token=ACCESS_TOKEN
    /// </pre>
    pub async fn add_account(&self, account: &str, nickname: &str, password: &str) -> LabradorResult<WechatCommonResponse> {
        let encrypted_password = md5(password);
        let data = json!({
            "kf_account": account.to_owned(),
            "nickname": nickname.to_owned(),
            "password": encrypted_password
        });
       self.client.post(WechatMpMethod::CustomService(MpCustomServiceMethod::AccountAdd), vec![], data, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 设置客服信息（即更新客服信息）
    /// 详情请见：<a href="http://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1458044813&token=&lang=zh_CN">客服管理</a>
    /// 接口url格式：https://api.weixin.qq.com/customservice/kfaccount/update?access_token=ACCESS_TOKEN
    /// </pre>
    pub async fn update_account(&self, account: &str, nickname: &str, password: &str) -> LabradorResult<WechatCommonResponse> {
        // let encrypted_password = hash::hash(MessageDigest::md5(), password.as_bytes())?;
        let encrypted_password = md5(password);
        let data = json!({
            "kf_account": account.to_owned(),
            "nickname": nickname.to_owned(),
            "password": encrypted_password
        });
        self.client.post(WechatMpMethod::CustomService(MpCustomServiceMethod::AccountUpdate), vec![], data, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 删除客服账号
    /// 详情请见：<a href="http://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1458044813&token=&lang=zh_CN">客服管理</a>
    /// 接口url格式：https://api.weixin.qq.com/customservice/kfaccount/del?access_token=ACCESS_TOKEN&kf_account=KFACCOUNT
    /// </pre>
    pub async fn delete_account(&self, account: &str) -> LabradorResult<WechatCommonResponse> {
        self.client.get(WechatMpMethod::CustomService(MpCustomServiceMethod::AccountDelete), vec![], RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// 获取账号列表
    /// <pre>
    /// 获取客服基本信息
    /// 详情请见：<a href="http://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1458044813&token=&lang=zh_CN">客服管理</a>
    /// 接口url格式：https://api.weixin.qq.com/cgi-bin/customservice/getkflist?access_token=ACCESS_TOKEN
    /// </pre>
    pub async fn get_accounts(&self) -> LabradorResult<Vec<KFAccount>> {
        let res = self.client.get(WechatMpMethod::CustomService(MpCustomServiceMethod::AccountList), vec![], RequestType::Json).await?.json::<Value>()?;
        let mut result = WechatCommonResponse::from_value(res.clone())?;
        if result.is_success() {
            let kf_list = &res["kf_list"];
            let kf_list = kf_list.as_array().unwrap();
            let mut accounts = vec![];
            for kf in kf_list {
                let kf_id = &kf["kf_id"];
                let kf_id = kf_id.as_str().unwrap_or_default().to_owned();
                let kf_nick = &kf["kf_nick"];
                let kf_nick = kf_nick.as_str().unwrap_or_default().to_owned();
                let kf_account = &kf["kf_account"];
                let kf_account = kf_account.as_str().unwrap_or_default().to_owned();
                let avatar = &kf["kf_headimgurl"];
                let avatar = avatar.as_str().unwrap_or_default().to_owned();
                let account = KFAccount {
                    id: kf_id.to_owned(),
                    nick: kf_nick.to_owned(),
                    account: kf_account.to_owned(),
                    avatar: avatar.to_owned(),
                };
                accounts.push(account);
            }
            Ok(accounts)
        } else {
            Err(LabraError::ClientError {errcode: result.errcode.to_owned().unwrap_or_default().to_string(), errmsg: result.errmsg.to_owned().unwrap_or_default()})
        }
    }

    /// <pre>
    /// 获取在线客服接待信息
    /// 详情请见：<a href="http://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1458044813&token=&lang=zh_CN">客服管理</a>
    /// 接口url格式：https://api.weixin.qq.com/cgi-bin/customservice/getonlinekflist?access_token=ACCESS_TOKEN
    /// </pre>
    pub async fn get_online_accounts(&self) -> LabradorResult<Vec<OnlineKFAccount>> {
        let res = self.client.get(WechatMpMethod::CustomService(MpCustomServiceMethod::AccountOnlineList), vec![], RequestType::Json).await?.json::<serde_json::Value>()?;
        let mut result = WechatCommonResponse::from_value(res.clone())?;
        if result.is_success() {
            let kf_list = &res["kf_online_list"];
            let kf_list = kf_list.as_array().unwrap();
            let mut accounts = vec![];
            for kf in kf_list {
                let kf_id = &kf["kf_id"];
                let kf_id = kf_id.as_str().unwrap_or_default().to_owned();
                let kf_account = &kf["kf_account"];
                let kf_account = kf_account.as_str().unwrap_or_default().to_owned();
                let status = &kf["status"];
                let status = status.as_u64().unwrap();
                let auto_accept = &kf["auto_accept"];
                let auto_accept = auto_accept.as_u64().unwrap();
                let accepted_case = &kf["accepted_case"];
                let accepted_case = accepted_case.as_u64().unwrap();
                let account = OnlineKFAccount {
                    id: kf_id.to_owned(),
                    account: kf_account.to_owned(),
                    status: status,
                    auto_accept: auto_accept,
                    accepted_case: accepted_case,
                };
                accounts.push(account);
            }
            Ok(accounts)
        } else {
            Err(LabraError::ClientError {errcode: result.errcode.to_owned().unwrap_or_default().to_string(), errmsg: result.errmsg.to_owned().unwrap_or_default()})
        }
    }
}

//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KFAccount {
    pub id: String,
    pub nick: String,
    pub account: String,
    pub avatar: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnlineKFAccount {
    pub id: String,
    pub account: String,
    pub status: u64,
    pub auto_accept: u64,
    pub accepted_case: u64,
}





#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendVoiceRequest {
    openid: String,
    account: Option<String>,
    media_id: String,
}

#[allow(unused)]
impl SendVoiceRequest {
    pub fn new<S: Into<String>>(openid: S, media_id: S) -> SendVoiceRequest {
        SendVoiceRequest {
            openid: openid.into(),
            account: None,
            media_id: media_id.into(),
        }
    }

    pub fn with_account<S: Into<String>>(openid: S, media_id: S, account: S) -> SendVoiceRequest {
        SendVoiceRequest {
            openid: openid.into(),
            account: Some(account.into()),
            media_id: media_id.into(),
        }
    }

    fn to_json(&self) -> Value {
        let mut data = json!({
            "msgtype": "voice".to_owned(),
            "touser": self.openid.to_owned(),
            "voice": {
                "media_id": self.media_id.to_owned()
            }
        });
        if let Some(ref account) = self.account {
            data.as_object_mut().unwrap().insert("customservice".to_string(), json!({
                "kf_account": account.to_owned()
            }));
        }
        data
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendImageRequest {
    openid: String,
    account: Option<String>,
    media_id: String,
}

#[allow(unused)]
impl SendImageRequest {
    pub fn new<S: Into<String>>(openid: S, media_id: S) -> SendImageRequest {
        SendImageRequest {
            openid: openid.into(),
            account: None,
            media_id: media_id.into(),
        }
    }

    pub fn with_account<S: Into<String>>(openid: S, media_id: S, account: S) -> SendImageRequest {
        SendImageRequest {
            openid: openid.into(),
            account: Some(account.into()),
            media_id: media_id.into(),
        }
    }

    fn to_json(&self) -> Value {

        let mut data = json!({
            "msgtype": "image".to_owned(),
            "touser": self.openid.to_owned(),
            "image": {
                "media_id": self.media_id.to_owned()
            }
        });
        if let Some(ref account) = self.account {
            data.as_object_mut().unwrap().insert("customservice".to_string(), json!({
                "kf_account": account.to_owned()
            }));
        }

        data
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTextRequest {
    openid: String,
    account: Option<String>,
    content: String,
}

#[allow(unused)]
impl SendTextRequest {
    pub fn new<S: Into<String>>(openid: S, content: S) -> SendTextRequest {
        SendTextRequest {
            openid: openid.into(),
            content: content.into(),
            account: None,
        }
    }

    pub fn with_account<S: Into<String>>(openid: S, content: S, account: S) -> SendTextRequest {
        SendTextRequest {
            openid: openid.into(),
            content: content.into(),
            account: Some(account.into()),
        }
    }

    fn to_json(&self) -> Value {
        let mut data = json!({
            "msgtype": "text".to_owned(),
            "touser": self.openid.to_owned(),
            "text": {
                "content": self.content.to_owned()
            }
        });
        if let Some(ref account) = self.account {
            data.as_object_mut().unwrap().insert("customservice".to_string(), json!({
                "kf_account": account.to_owned()
            }));
        }

        data
    }
}
