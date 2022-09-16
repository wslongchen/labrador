use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

use crate::{session::SessionStore, request::{RequestType}, WechatCommonResponse, LabradorResult, WechatCpTpClient};
use crate::wechat::cp::constants::{PROVIDER_ACCESS_TOKEN};
use crate::wechat::cp::method::{CpLicenseMethod, WechatCpMethod};

/// 服务商接口调用许可相关
#[derive(Debug, Clone)]
pub struct WechatCpTpLicense<'a, T: SessionStore> {
    client: &'a WechatCpTpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WechatCpTpLicense<'a, T> {

    #[inline]
    pub fn new(client: &WechatCpTpClient<T>) -> WechatCpTpLicense<T> {
        WechatCpTpLicense {
            client,
        }
    }

    /// <pre>
    /// 下单购买帐号
    /// 服务商下单为企业购买新的帐号，可以同时购买基础帐号与互通帐号。
    /// 下单之后，需要到服务商管理端发起支付，支付完成之后，订单才能生效。
    /// 文档地址：<a href="https://developer.work.weixin.qq.com/document/path/95644">文档</a>
    /// </pre>
    pub async fn create_order(&self, req: WechatCpTpLicenseNewOrderRequest) -> LabradorResult<String> {
        let v = self.client.post(WechatCpMethod::License(CpLicenseMethod::CreateOrder), vec![], req, RequestType::Json).await?.json::<Value>()?;
        let v = WechatCommonResponse::parse::<Value>(v)?;
        let order_id = v["order_id"].as_str().unwrap_or_default();
        Ok(order_id.to_string())
    }


    /// 创建下单续期帐号任务
    /// <pre>
    ///  可以下单为一批已激活帐号的成员续期，续期下单分为两个步骤：
    /// 传入userid列表创建一个任务，创建之后，可以往同一个任务继续追加待续期的userid列表；
    /// 根据步骤1得到的jobid提交订单。
    /// </pre>
    pub async fn create_renew_order_job(&self, req: WechatCpTpLicenseRenewOrderJobRequest) -> LabradorResult<WechatCpTpLicenseRenewOrderJobResponse> {
        let v = self.client.post(WechatCpMethod::License(CpLicenseMethod::CreateRenewOrderJob), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpTpLicenseRenewOrderJobResponse>(v)
    }


    /// <pre>
    ///  提交续期订单
    /// 创建续期任务之后，需要调用该接口，以提交订单任务。
    /// 注意，提交之后，需要到服务商管理端发起支付，支付完成之后，订单才能生效。
    /// 文档地址：<a href="https://developer.work.weixin.qq.com/document/path/95646">文档</a>
    /// </pre>
    pub async fn submit_renew_order(&self, req: WechatCpTpLicenseRenewOrderRequest) -> LabradorResult<String> {
        let v = self.client.post(WechatCpMethod::License(CpLicenseMethod::SubmitOrderJob), vec![], req, RequestType::Json).await?.json::<Value>()?;
        let v = WechatCommonResponse::parse::<Value>(v)?;
        let order_id = v["order_id"].as_str().unwrap_or_default();
        Ok(order_id.to_string())
    }


    /// <pre>
    ///  获取订单列表
    /// 服务商查询自己某段时间内的平台能力服务订单列表
    /// 文档地址：<a href="https://developer.work.weixin.qq.com/document/path/95647">文档</a>
    /// </pre>
    pub async fn get_order_list(&self, corp_id: &str, start: Option<i64>, end: Option<i64>, cursor: &str, limit: i32) -> LabradorResult<WechatCpTpLicenseOrderListResp> {
        let mut req = json!({
            "corpid": corp_id,
            "cursor": cursor,
            "limit": limit,
        });
        if let Some(start) = start {
            req["start_time"] = start.into();
        }
        if let Some(end) = end {
            req["end_time"] = end.into();
        }
        let access_token = self.client.get_wechat_provider_token().await?;
        let v = self.client.post(WechatCpMethod::License(CpLicenseMethod::ListOrder), vec![(PROVIDER_ACCESS_TOKEN.to_string(), access_token)], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpTpLicenseOrderListResp>(v)
    }


    /// <pre>
    ///  获取订单列表
    /// 服务商查询自己某段时间内的平台能力服务订单列表
    /// 文档地址：<a href="https://developer.work.weixin.qq.com/document/path/95647">文档</a>
    /// </pre>
    pub async fn get_order(&self, order_id: &str) -> LabradorResult<WechatCpTpLicenseOrderInfoResponse> {
        let mut req = json!({
            "order_id": order_id,
        });
        let access_token = self.client.get_wechat_provider_token().await?;
        let v = self.client.post(WechatCpMethod::License(CpLicenseMethod::GetOrder), vec![(PROVIDER_ACCESS_TOKEN.to_string(), access_token)], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpTpLicenseOrderInfoResponse>(v)
    }


    /// <pre>
    ///  查询指定订单下的平台能力服务帐号列表。
    /// 若为购买帐号的订单或者存量企业的版本付费迁移订单，则返回帐号激活码列表；
    /// 若为续期帐号的订单，则返回续期帐号的成员列表。注意，若是购买帐号的订单，
    /// 则仅订单支付完成时，系统才会生成帐号，故支付完成之前，该接口不会返回帐号激活码。
    /// 文档地址：<a href="https://developer.work.weixin.qq.com/document/path/95649">文档</a>
    /// </pre>
    pub async fn get_order_account_list(&self, order_id: &str, limit: i32, cursor: &str) -> LabradorResult<WechatCpTpLicenseOrderAccountListResponse> {
        let mut req = json!({
            "order_id": order_id,
            "cursor": cursor,
            "limit": limit,
        });
        let access_token = self.client.get_wechat_provider_token().await?;
        let v = self.client.post(WechatCpMethod::License(CpLicenseMethod::ListOrderAccount), vec![(PROVIDER_ACCESS_TOKEN.to_string(), access_token)], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpTpLicenseOrderAccountListResponse>(v)
    }


    /// <pre>
    ///  取消订单
    /// 取消接口许可购买和续费订单，只可取消未支付且未失效的订单。
    /// 文档地址：<a href="https://developer.work.weixin.qq.com/document/path/96106">文档</a>
    /// </pre>
    pub async fn cancel_order(&self, corp_id: &str, order_id: &str) -> LabradorResult<WechatCommonResponse> {
        let mut req = json!({
            "corpid": corp_id,
            "order_id": order_id,
        });
        let access_token = self.client.get_wechat_provider_token().await?;
        self.client.post(WechatCpMethod::License(CpLicenseMethod::CancelOrder), vec![(PROVIDER_ACCESS_TOKEN.to_string(), access_token)], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    ///  激活帐号
    /// 下单购买帐号并支付完成之后，先调用获取订单中的帐号列表接口获取到帐号激活码，
    /// 然后可以调用该接口将激活码绑定到某个企业员工，以对其激活相应的平台服务能力。
    /// 文档地址：<a href="https://developer.work.weixin.qq.com/document/path/95553">文档</a>
    /// </pre>
    pub async fn active_code(&self, code: &str, corp_id: &str, user_id: &str) -> LabradorResult<WechatCommonResponse> {
        let mut req = json!({
            "active_code": code,
            "corpid": corp_id,
            "userid": user_id,
        });
        let access_token = self.client.get_wechat_provider_token().await?;
        self.client.post(WechatCpMethod::License(CpLicenseMethod::ActiveAccount), vec![(PROVIDER_ACCESS_TOKEN.to_string(), access_token)], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 批量激活帐号
    /// 可在一次请求里为一个企业的多个成员激活许可帐号，便于服务商批量化处理。
    /// 一个userid允许激活一个基础帐号以及一个互通帐号。
    /// 单次激活的员工数量不超过1000
    /// </pre>
    pub async fn batch_active_code(&self, corp_id: &str, active_accounts: Vec<WechatCpTpLicenseActiveAccount>) -> LabradorResult<WechatCpTpLicenseOrderAccountListResponse> {
        let mut req = json!({
            "corp_id": corp_id,
            "active_list": active_accounts,
        });
        let access_token = self.client.get_wechat_provider_token().await?;
        let v = self.client.post(WechatCpMethod::License(CpLicenseMethod::BatchActiveAccount), vec![(PROVIDER_ACCESS_TOKEN.to_string(), access_token)], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpTpLicenseOrderAccountListResponse>(v)
    }

    /// <pre>
    /// 获取激活码详情
    /// 查询某个帐号激活码的状态以及激活绑定情况。
    /// 文档地址：<a href="https://developer.work.weixin.qq.com/document/path/95552">文档</a>
    /// </pre>
    pub async fn get_active_info_by_code(&self, code: &str, corp_id: &str) -> LabradorResult<WechatCpTpLicenseCodeInfoResponse> {
        let mut req = json!({
            "active_code": code,
            "corpid": corp_id,
        });
        let access_token = self.client.get_wechat_provider_token().await?;
        let v = self.client.post(WechatCpMethod::License(CpLicenseMethod::GetActiveInfoByCode), vec![(PROVIDER_ACCESS_TOKEN.to_string(), access_token)], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpTpLicenseCodeInfoResponse>(v)
    }


    /// <pre>
    /// 获取激活码详情
    /// 查询某个帐号激活码的状态以及激活绑定情况。
    /// 文档地址：<a href="https://developer.work.weixin.qq.com/document/path/95552">文档</a>
    /// </pre>
    pub async fn batch_get_active_info_by_code(&self, codes: Vec<&str>, corp_id: &str) -> LabradorResult<WechatCpTpLicenseBatchCodeInfoResponse> {
        let mut req = json!({
            "active_code_list": codes,
            "corpid": corp_id,
        });
        let access_token = self.client.get_wechat_provider_token().await?;
        let v = self.client.post(WechatCpMethod::License(CpLicenseMethod::BatchGetActiveInfoByCode), vec![(PROVIDER_ACCESS_TOKEN.to_string(), access_token)], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpTpLicenseBatchCodeInfoResponse>(v)
    }


    /// <pre>
    /// 获取企业的帐号列表
    /// 查询指定企业下的平台能力服务帐号列表。
    /// 文档地址：<a href="https://developer.work.weixin.qq.com/document/path/95544">文档</a>
    /// </pre>
    pub async fn get_corp_account_list(&self, corp_id: &str, limit: i32, cursor: &str) -> LabradorResult<WechatCpTpLicenseCorpAccountListResponse> {
        let mut req = json!({
            "cursor": cursor,
            "corpid": corp_id,
            "limit": limit,
        });
        let access_token = self.client.get_wechat_provider_token().await?;
        let v = self.client.post(WechatCpMethod::License(CpLicenseMethod::ListActivedAccount), vec![(PROVIDER_ACCESS_TOKEN.to_string(), access_token)], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpTpLicenseCorpAccountListResponse>(v)
    }


    /// <pre>
    /// 获取成员的激活详情
    /// 查询某个企业成员的激活情况。
    /// 文档地址：<a href="https://developer.work.weixin.qq.com/document/path/95555">文档</a>
    /// </pre>
    pub async fn get_active_info_by_user(&self, corp_id: &str, user_id: &str) -> LabradorResult<WechatCpTpLicenseActiveInfoByUserResponse> {
        let mut req = json!({
            "corpid": corp_id,
            "user_id": user_id,
        });
        let access_token = self.client.get_wechat_provider_token().await?;
        let v = self.client.post(WechatCpMethod::License(CpLicenseMethod::GetActiveInfoByUser), vec![(PROVIDER_ACCESS_TOKEN.to_string(), access_token)], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpTpLicenseActiveInfoByUserResponse>(v)
    }


    /// <pre>
    /// 帐号继承
    /// 在企业员工离职或者工作范围的有变更时，允许将其许可帐号继承给其他员工。
    /// 文档地址：<a href="https://developer.work.weixin.qq.com/document/path/95555">文档</a>
    /// </pre>
    pub async fn batch_transfer_license(&self, corp_id: &str, transfers: Vec<WechatCpTpLicenseTransfer>) -> LabradorResult<WechatCpTpLicenseBatchTransferResponse> {
        let mut req = json!({
            "corpid": corp_id,
            "transfer_list": transfers,
        });
        let access_token = self.client.get_wechat_provider_token().await?;
        let v = self.client.post(WechatCpMethod::License(CpLicenseMethod::BatchTransferLicense), vec![(PROVIDER_ACCESS_TOKEN.to_string(), access_token)], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpTpLicenseBatchTransferResponse>(v)
    }

}

//----------------------------------------------------------------------------------------------------------------------------
/// 下单购买帐号
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpTpLicenseNewOrderRequest {
    /// 企业ID
    pub corpid: String,
    /// 购买者ID
    pub buyer_userid: String,
    /// 账号个数
    pub account_count: WechatCpTpLicenseAccountCount,
    /// 购买时长
    pub account_duration: WechatCpTpLicenseAccountDuration,
}


/// 创建下单续期帐号任务
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpTpLicenseRenewOrderJobRequest {
    /// 企业ID
    pub corpid: String,
    /// 续费的用户UserId
    pub account_list: Vec<WechatCpTpLicenseBaseAccount>,
    /// 任务id，若不传则默认创建一个新任务。若指定第一次调用后拿到jobid，可以通过该接口将jobid关联多个userid
    pub jobid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpTpLicenseAccountCount {
    pub base_count: i32,
    pub external_contact_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpTpLicenseAccountDuration {
    pub months: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpTpLicenseBaseAccount {
    pub userid: String,
    #[serde(rename="type")]
    pub r#type: i32,
}

/// 创建下单购买帐号任务返回结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpTpLicenseRenewOrderJobResponse {
    /// 任务ID
    pub jobid: String,
    /// 有效的续费账号列表
    pub invalid_account_list: Option<Vec<WechatCpTpLicenseBaseAccount>>,
}

/// 续期帐号订单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpTpLicenseRenewOrderRequest {
    pub buyer_userid: String,
    pub jobid: String,
    pub account_duration: WechatCpTpLicenseAccountDuration,
}

/// 获取订单列表详情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpTpLicenseOrderListResp {
    pub next_cursor: Option<String>,
    pub has_more: Option<i32>,
    pub order_list: Option<Vec<WechatCpTpLicenseSimpleOrder>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpTpLicenseSimpleOrder {
    pub order_id: Option<String>,
    pub order_type: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpTpLicenseOrderInfoResponse {
    pub order: Option<WechatCpTpLicenseOrder>,
}

/// 详细的订单信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpTpLicenseOrder {
    pub order_id: Option<String>,
    pub corpid: Option<String>,
    pub order_type: Option<i32>,
    pub order_status: Option<i32>,
    pub price: Option<i32>,
    pub create_time: Option<i64>,
    pub pay_time: Option<i64>,
    pub account_count: Option<WechatCpTpLicenseAccountCount>,
    pub account_duration: Option<WechatCpTpLicenseAccountDuration>,
}

/// 获取订单中的帐号列表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpTpLicenseOrderAccountListResponse {
    pub next_cursor: Option<String>,
    pub has_more: Option<i32>,
    pub account_list: Option<Vec<WechatCpTpLicenseAccount>>,
}

/// 订单账号信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpTpLicenseAccount {
    /// 激活码
    pub active_code: Option<String>,
    /// 用户ID
    pub userid: Option<String>,
    #[serde(rename="type")]
    pub r#type: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpTpLicenseActiveAccount {
    /// 用户ID
    pub userid: String,
    /// 激活码
    pub active_code: String,
}

/// 查询的激活码详情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpTpLicenseCodeInfoResponse {
    pub active_info: Option<WechatCpTpLicenseActiveCodeInfo>,
}

/// 批量查询的激活码详情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpTpLicenseBatchCodeInfoResponse {
    pub active_info_list: Option<Vec<WechatCpTpLicenseActiveCodeInfo>>,
    pub invalid_active_code_list: Option<Vec<String>>,
}
/// 激活码信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpTpLicenseActiveCodeInfo {
    pub active_code: Option<String>,
    pub status: Option<i32>,
    pub create_time: Option<i64>,
    pub active_time: Option<i64>,
    pub expire_time: Option<i64>,
}
/// 企业的帐号列表（已激活）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpTpLicenseCorpAccountListResponse {
    pub next_cursor: Option<String>,
    pub has_more: Option<i32>,
    pub account_list: Option<Vec<WechatCpTpLicenseCorpAccount>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpTpLicenseCorpAccount {
    pub active_time: Option<i64>,
    pub expire_time: Option<i64>,
}

/// 某个企业成员的激活情况
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpTpLicenseActiveInfoByUserResponse {
    pub active_status: Option<i32>,
    pub active_info_list: Option<Vec<WechatCpTpLicenseActiveCodeInfo>>,
}

/// 基础结果返回信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpTpLicenseBatchTransferResponse {
    pub transfer_result: Option<Vec<WechatCpTpLicenseTransfer>>,
}

/// 基础的信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpTpLicenseTransfer {
    /// 转移成员加密的userid
    pub handover_userid: Option<String>,
    /// 接收成员加密的userid
    pub takeover_userid: Option<String>,
}