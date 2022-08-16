use std::vec;

use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

use crate::{session::SessionStore, request::{RequestType}, errors::LabraError, WechatCommonResponse, WeChatMpClient, LabradorResult};
use crate::util::md5::md5;
use crate::wechat::mp::method::{CustomServiceMethod, WechatMpMethod};


#[derive(Debug, Clone)]
pub struct WeChatCustomService<'a, T: SessionStore> {
    client: &'a WeChatMpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WeChatCustomService<'a, T> {

    #[inline]
    pub fn new(client: &WeChatMpClient<T>) -> WeChatCustomService<T> {
        WeChatCustomService {
            client,
        }
    }

    /// 添加账号
    pub async fn add_account(&self, account: &str, nickname: &str, password: &str) -> LabradorResult<WechatCommonResponse<String>> {
        // let encrypted_password = hash::hash(MessageDigest::md5(), password.as_bytes())?;
        let encrypted_password = md5(password);
        let data = json!({
            "kf_account": account.to_owned(),
            "nickname": nickname.to_owned(),
            "password": encrypted_password
        });
        let v = self.client.post(WechatMpMethod::CustomService(CustomServiceMethod::AccountAdd), data, RequestType::Json).await?.json::<serde_json::Value>().await?;
        serde_json::from_value::<WechatCommonResponse<_>>(v).map_err(LabraError::from)
    }

    /// 修改账号
    pub async fn update_account(&self, account: &str, nickname: &str, password: &str) -> LabradorResult<WechatCommonResponse<String>> {
        // let encrypted_password = hash::hash(MessageDigest::md5(), password.as_bytes())?;
        let encrypted_password = md5(password);
        let data = json!({
            "kf_account": account.to_owned(),
            "nickname": nickname.to_owned(),
            "password": encrypted_password
        });
        let v = self.client.post(WechatMpMethod::CustomService(CustomServiceMethod::AccountUpdate), data, RequestType::Json).await?.json::<serde_json::Value>().await?;
        serde_json::from_value::<WechatCommonResponse<_>>(v).map_err(LabraError::from)
    }

    /// 删除账号
    pub async fn delete_account(&self, account: &str) -> LabradorResult<WechatCommonResponse<String>> {
        let v= self.client.get(WechatMpMethod::CustomService(CustomServiceMethod::AccountDelete), vec![], RequestType::Json).await?.json::<Value>().await?;
        serde_json::from_value::<WechatCommonResponse<_>>(v).map_err(LabraError::from)
    }

    /// 获取账号列表
    pub async fn get_accounts(&self) -> LabradorResult<WechatCommonResponse<Vec<KFAccount>>> {
        let res = self.client.get(WechatMpMethod::CustomService(CustomServiceMethod::AccountList), vec![], RequestType::Json).await?.json::<Value>().await?;
        let mut result = serde_json::from_value::<WechatCommonResponse<_>>(res.to_owned())?;
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
            result.result = accounts.into();
        }
        Ok(result)
    }

    /// 获取在线账号
    pub async fn get_online_accounts(&self) -> LabradorResult<WechatCommonResponse<Vec<OnlineKFAccount>>> {
        let res = self.client.get(WechatMpMethod::CustomService(CustomServiceMethod::AccountOnlineList), vec![], RequestType::Json).await?.json::<serde_json::Value>().await?;
        let mut result = serde_json::from_value::<WechatCommonResponse<_>>(res.to_owned())?;
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
            result.result = accounts.into();
        }
        Ok(result)
    }

    /* pub async fn upload_avatar<R: Read>(&self, account: &str, avatar: &mut R) -> LabradorResult<()> {
        let mut files = HashMap::new();
        files.insert("media".to_owned(), avatar);
        self.client.upload_file(
                "https://api.weixin.qq.com/customservice/kfaccount/uploadheadimg",
                vec![("kf_account", account)],
                &mut files
        ).await?;
        Ok(())
    } */
}

//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq,Serialize, Deserialize)]
pub struct KFAccount {
    pub id: String,
    pub nick: String,
    pub account: String,
    pub avatar: String,
}

#[derive(Debug, Clone, PartialEq, Eq,Serialize, Deserialize)]
pub struct OnlineKFAccount {
    pub id: String,
    pub account: String,
    pub status: u64,
    pub auto_accept: u64,
    pub accepted_case: u64,
}
