use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

use crate::{session::SessionStore, request::{RequestType}, WechatCommonResponse, LabradorResult, WechatCpTpClient, DealerCorpInfo};
use crate::wechat::cp::constants::{PROVIDER_ACCESS_TOKEN};
use crate::wechat::cp::method::{CpLicenseMethod, WechatCpMethod};

/// 服务商接口调用许可相关
#[derive(Debug, Clone)]
pub struct WechatCpTpOrder<'a, T: SessionStore> {
    client: &'a WechatCpTpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WechatCpTpOrder<'a, T> {

    #[inline]
    pub fn new(client: &WechatCpTpClient<T>) -> WechatCpTpOrder<T> {
        WechatCpTpOrder {
            client,
        }
    }

    /// <pre>
    ///  获取订单详情
    /// <p>
    /// <a href='https://developer.work.weixin.qq.com/document/15219#%E8%8E%B7%E5%8F%96%E8%AE%A2%E5%8D%95%E8%AF%A6%E6%83%85'>文档地址</a>
    /// <p/>
    pub async fn get_order_info(&self, order_id: &str) -> LabradorResult<WechatCpTpOrderDetailsResponse> {
        let mut req = json!({
            "orderid": order_id,
        });
        let v = self.client.post(WechatCpMethod::GetOrder, vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpTpOrderDetailsResponse>(v)
    }

    /// <pre>
    ///  获取订单列表
    /// <p>
    /// <a href='https://developer.work.weixin.qq.com/document/15219#%E8%8E%B7%E5%8F%96%E8%AE%A2%E5%8D%95%E5%88%97%E8%A1%A8'>文档地址</a>
    /// <p/>
    /// </pre>
    pub async fn get_order_list(&self, start_time: Option<i64>, end_time: Option<i64>, test_mode: u8) -> LabradorResult<WechatCpTpOrderListGetResponse> {
        let mut req = json!({
            "test_mode": test_mode,
        });
        if let Some(start) = start_time {
            req["start_time"] = (start / 1000).into();
        }
        if let Some(end) = end_time {
            req["end_time"] = (end / 1000).into();
        }
        let access_token = self.client.get_wechat_provider_token().await?;
        let v = self.client.post(WechatCpMethod::GetOrderList, vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpTpOrderListGetResponse>(v)
    }


}

//----------------------------------------------------------------------------------------------------------------------------
/// 应用版本付费订单详情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpTpOrderDetailsResponse {
    /// 订单号
    pub orderid: Option<String>,
    /// 订单状态。0-未支付，1-已支付，2-已关闭， 3-未支付且已过期， 4-申请退款中， 5-申请退款成功， 6-退款被拒绝
    pub order_status: Option<u8>,
    /// 订单类型。0-普通订单，1-扩容订单，2-续期，3-版本变更
    pub order_type: Option<u8>,
    /// 客户企业的corpid
    pub paid_corpid: Option<String>,
    /// 下单操作人员userid。如果是服务商代下单，没有该字段。
    pub operator_id: Option<String>,
    /// 应用id
    pub suiteid: Option<String>,
    /// 应用id。（仅旧套件有该字段）
    pub appid: Option<String>,
    /// 购买版本ID
    pub edition_id: Option<String>,
    /// 购买版本名字
    pub edition_name: Option<String>,
    /// 实付款金额，单位分
    pub price: Option<i64>,
    /// 购买的人数
    pub user_count: Option<i64>,
    /// 购买的时间，单位天
    pub order_period: Option<i64>,
    /// 下单时间，秒级时间戳
    pub order_time: Option<i64>,
    /// 付款时间，秒级时间戳
    pub paid_time: Option<i64>,
    /// 购买生效期的开始时间，秒级时间戳
    pub begin_time: Option<i64>,
    /// 购买生效期的结束时间，秒级时间戳
    pub end_time: Option<i64>,
    /// 下单来源。0-客户下单；1-服务商代下单；2-代理商代下单
    pub order_from: Option<u8>,
    /// 下单方corpid
    pub operator_corpid: Option<String>,
    /// 服务商分成金额，单位分
    pub service_share_amount: Option<i64>,
    /// 平台分成金额，单位分
    pub platform_share_amount: Option<i64>,
    /// 代理商分成金额，单位分
    pub dealer_share_amount: Option<i64>,
    /// 渠道商信息（仅当有渠道商报备后才会有此字段）
    pub dealer_corp_info: Option<DealerCorpInfo>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpTpOrderListGetResponse {
    /// 订单列表
    pub order_list: Option<Vec<WechatCpTpOrderDetailsResponse>>,
}

