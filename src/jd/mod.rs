use chrono::Local;
use crate::{client::{APIClient}, request::{RequestType, Method, Response, LabraRequest, RequestMethod}, util::get_sign, errors::LabraError, session::{SessionStore, SimpleStorage}, Params, LabradorResult};
use serde_json::{Value as JsonValue};

mod method;
mod request;
mod response;

use std::collections::BTreeMap;
use serde::Serialize;

use self::{request::{JdJFGoodsParam, JdJFGoodsRequest, JdGoodsInfoQueryParam, JdOrderRecentQueryParam, JdOrderRequest, JdOrderRawQueryParam, JdOrderRawRequest, JdPromotionUrlGenerateParam}, response::{JdCommonResponse, JdJFGoodsSelect, JdGoodsInfoQuery, JdOrderQueryResponse, JdPromotionUrlGenerateResponse}, method::JDMethod};


#[derive(Debug, Clone)]
pub struct JDClient <T: SessionStore> {
    api_client: APIClient<T>
}

/// JDClient
/// 
/// 
/// # Example
/// 
/// ```no_run
/// # use labrador::JDClient;
/// async fn main() {
///     use labrador::JDClient;
/// let client = JDClient::new("appKey", "secret");
///     // Do Some Thing You Want
///     // ...
/// }
/// ```
/// 
#[allow(unused)]
impl <T: SessionStore> JDClient<T> {

    pub fn new<Q: Into<String>, S: Into<String>>(app_key: Q, secret: S) -> JDClient<SimpleStorage> {
        JDClient {
            api_client: APIClient::<SimpleStorage>::new::<Q, String, S>(app_key, secret, "https://router.jd.com/api".to_owned())
        }
    }


    pub fn from_session<Q: Into<String>, S: Into<String>>(app_key: Q, secret: S, session: T) -> JDClient<T> {
        JDClient {
            api_client: APIClient::from_session(app_key, secret, String::from("https://router.jd.com/api"), session)
        }
    }

    #[inline]
    fn build_common_params(&self) -> Vec<(String, String)> {
        // build common params
        let mut params: Vec<(String, String)> = Vec::new();
        params.push(("app_key".to_owned(), self.api_client.app_key.to_owned()));
        let now = Local::now().naive_local().format("%Y-%m-%d %H:%M:%S").to_string();
        params.push(("timestamp".to_owned(), now));
        params.push(("format".to_owned(), "json".to_owned()));
        params.push(("v".to_owned(), "1.0".to_owned()));
        params.push(("sign_method".to_owned(), "md5".to_owned()));
        params
    }

    /// 发送请求数据
    async fn send<D: Serialize + Params>(&self, method: JDMethod, data: D) -> LabradorResult<JsonValue> {
        let mut params = self.build_common_params();
        let method_name_str = method.get_method();
        params.push(("method".to_owned(), method_name_str));
        let request_type = RequestType::Json;
        // build sign
        params.extend_from_slice(data.get_params().as_slice());
        let mut pairs = BTreeMap::new();
        for (key, value) in params.iter() {
            pairs.insert(key.to_string(), value.to_string());
        }
        let sign = get_sign(&pairs, self.api_client.secret.to_owned().as_str());
        params.push(("sign".to_owned(), sign));
        let result = self.api_client.request(LabraRequest::new().method(Method::Post).data(data).req_type(request_type).params(params)).await?.json::<serde_json::Value>().await?;
        self.json_decode(result, &method.get_response_key())
    }


    #[inline]
    fn json_decode(&self, obj: JsonValue, response_key: &String) -> LabradorResult<JsonValue> {
        match obj.get("error_response") {
            Some(error_response) => {
                let errcode = if let Some(code) = error_response.get("code") {
                    code.as_i64().unwrap_or_default() as i32
                } else {
                    0
                };
                if errcode != 0 {
                    let errmsg = match error_response.get("zh_desc") {
                        Some(msg) => msg.as_str().unwrap_or_default().to_owned(),
                        None => "".to_string()
                    };
                    return Err(LabraError::ClientError { errcode: errcode.to_string(), errmsg: errmsg.to_owned() });
                }
            },
            None => {},
        }
        match obj.get(response_key) {
            Some(response) => {
                match response.get("result") {
                    Some(query_result) => {
                        match serde_json::from_str::<JsonValue>(query_result.as_str().unwrap_or_default()) {
                            Ok(res) => {
                                let code = if let Some(code) = res.get("code") {
                                    code.as_i64().unwrap_or_default() as i32
                                } else {
                                    0
                                };
                                if code != 200 {
                                    let errmsg = match res.get("message") {
                                        Some(msg) => msg.as_str().unwrap_or_default().to_owned(),
                                        None => "".to_string()
                                    };
                                    return Err(LabraError::ClientError { errcode: code.to_string(), errmsg: errmsg.to_owned() });
                                } 
                            }
                            Err(err) => {
                                return Err(LabraError::ClientError { errcode: "-3".to_string(), errmsg: format!("Response decode error: No Query Result") });
                            }
                        }
                        Ok(query_result.to_owned())
                    },
                    None => {
                        Err(LabraError::ClientError { errcode: "-3".to_string(), errmsg: format!("Response decode error: No Query Result") })
                    },
                }
            },
            None => {
                Err(LabraError::ClientError { errcode: "-3".to_string(), errmsg: format!("Response decode error") })
            }
        }
    }

    /// 京粉精选商品查询
    /// 
    /// 京东联盟精选优质商品，每日更新，可通过频道ID查询各个频道下的精选商品。
    /// 用获取的优惠券链接调用转链接口时，需传入搜索接口link字段返回的原始优惠券链接
    /// 切勿对链接进行任何encode、decode操作，否则将导致转链二合一推广链接时校验失败。
    /// 
    /// # 示例
    /// ```no_run
    /// 
    ///     use labrador::JDClient;
    ///     use labrador::{TbMaterialSelectParam};
    /// 
    ///     async fn main() {
    ///         let param = JdJFGoodsParam {
    ///         elite_id: 22,
    ///         page_index: 1.into(),
    ///         page_size: 1.into(),
    ///         sort_name: None,
    ///         sort: None,
    ///         pid: "1001969763_4100247890_3003271490".to_owned().into(),
    ///         fields: None,
    ///         forbid_types: None,
    ///     };
    ///         let client = JDClient::new("appKey", "secret");
    ///         match client.get_jf_select(param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    /// 
    /// ```
    /// 
    pub async fn get_jf_select(&self, param: JdJFGoodsParam) -> LabradorResult<JdCommonResponse<Vec<JdJFGoodsSelect>>> {
        self.send(JDMethod::FanGoodsSelect, JdJFGoodsRequest{ goods_req: param}).await?.parse_result()
    }


    /// 根据skuid查询商品信息
    /// 
    /// 通过SKUID查询推广商品的名称、主图、类目、价格、物流、是否自营、30天引单数量等详细信息，支持批量获取。
    /// 通常用于在媒体侧展示商品详情。
    /// 
    /// # 示例
    /// ```no_run
    /// 
    ///     use labrador::JDClient;
    ///     use labrador::{JdGoodsInfoQueryParam};
    /// 
    ///     async fn main() {
    ///         let param = JdGoodsInfoQueryParam {
    ///             sku_ids: "60566006897".to_string().into(),
    ///         };
    ///         let client = JDClient::new("appKey", "secret");
    ///         match client.get_goods_detail(param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    /// 
    /// ```
    /// 
    pub async fn get_goods_detail(&self, param: JdGoodsInfoQueryParam) -> LabradorResult<JdCommonResponse<Vec<JdGoodsInfoQuery>>> {
        self.send(JDMethod::GoodsInfoQuery, param).await?.parse_result()
    }

    /// 网站/APP获取推广链接接口
    ///
    /// 网站/APP来获取的推广链接，功能同宙斯接口的自定义链接转换、 APP领取代码接口通过商品链接、活动链接获取普通推广链接
    /// 支持传入subunionid参数，可用于区分媒体自身的用户ID，该参数可在订单查询接口返回，需向cps-qxsq@jd.com申请权限。
    ///
    /// # 示例
    /// ```no_run
    ///
    ///     use labrador::JDClient;
    ///     use labrador::{JdPromotionUrlGenerateParam};
    ///
    ///     async fn main() {
    ///         let param = JdPromotionUrlGenerateParam {
    ///             material_id: "".to_owned(),
    ///             site_id: "".to_owned(),
    ///             position_id: None,
    ///             sub_union_id: None,
    ///             ext1: None,
    ///             pid: None,
    ///             coupon_url: None,
    ///             gift_coupon_key: None,
    ///         };
    ///         let client = JDClient::new("appKey", "secret");
    ///         match client.get_goods_detail(param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    ///
    /// ```
    ///
    pub async fn generate_promotion_url(&self, param: JdPromotionUrlGenerateParam) -> LabradorResult<JdCommonResponse<JdPromotionUrlGenerateResponse>> {
        self.send(JDMethod::PromotionUrlGenerate, param).await?.parse_result()
    }

    /// 订单查询
    ///
    /// 查询推广订单及佣金信息，可查询最近90天内下单的订单，会随着订单状态变化同步更新数据。
    /// 支持按下单时间、完成时间或更新时间查询。建议按更新时间每分钟调用一次，查询最近一分钟的订单更新数据。
    /// 支持查询subunionid、推广位、PID参数，支持普通推客及工具商推客订单查询。
    ///
    /// # 示例
    /// ```no_run
    ///
    ///     use labrador::JDClient;
    ///     use labrador::{JdPromotionUrlGenerateParam};
    ///
    ///     async fn main() {
    ///         let param = JdOrderRecentQueryParam {
    ///             page_no: 1.into(),
    ///             page_size: 1.into(),
    ///             bill_type: 1,
    ///             time: "".to_owned(),
    ///             child_union_id: None,
    ///             key: None,
    ///         };
    ///         let client = JDClient::new("appKey", "secret");
    ///         match client.query_recent_order(param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    ///
    /// ```
    ///
    pub async fn query_recent_order(&self, param: JdOrderRecentQueryParam) -> LabradorResult<JdCommonResponse<Vec<JdOrderQueryResponse>>> {
        self.send(JDMethod::OrderRecentQuery, JdOrderRequest { order_req: param}).await?.parse_result()
    }

    /// 订单行查询
    ///
    /// 查询推广订单及佣金信息，可查询最近90天内下单的订单，会随着订单状态变化同步更新数据。
    /// 支持按下单时间、完成时间或更新时间查询。建议按更新时间每分钟调用一次，查询最近一分钟的订单更新数据。
    /// 支持查询subunionid、推广位、PID参数，支持普通推客及工具商推客订单查询。
    ///
    /// 如需要通过SDK调用此接口，请接入JOS SDK：https://union.jd.com/helpcenter/13246-13312-108188
    ///
    /// # 示例
    /// ```no_run
    ///
    ///     use labrador::JDClient;
    ///     use labrador::{JdPromotionUrlGenerateParam};
    ///
    ///     async fn main() {
    ///         let param = JdOrderRawQueryParam {
    ///             page_index: 1.into(),
    ///             page_size: 1.into(),
    ///             bill_type: 1,
    ///             startTime: "".to_owned(),
    ///             endTime: "".to_owned(),
    ///             child_union_id: None,
    ///             key: None,
    ///             fields: None,
    ///         };
    ///         let client = JDClient::new("appKey", "secret");
    ///         match client.query_raw_order(param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    ///
    /// ```
    ///
    pub async fn query_raw_order(&self, param: JdOrderRawQueryParam) -> LabradorResult<JdCommonResponse<Vec<JdOrderQueryResponse>>> {
        self.send(JDMethod::OrderRawQuery, JdOrderRawRequest { order_req: param}).await?.parse_result()
    }
}
