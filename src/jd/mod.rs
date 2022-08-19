use chrono::Local;
use crate::{client::{APIClient}, request::{RequestType, Method, LabraRequest, RequestMethod}, session::{SessionStore, SimpleStorage}, LabradorResult, RequestParametersHolder, md5};
use crate::jd::constants::{RESPONSE_GETRESULT, RESPONSE_QUERYRESULT, SIGN_TYPE_MD5, VERSION_1};

mod method;
mod request;
mod response;
#[allow(unused)]
mod constants;

use std::collections::BTreeMap;
use serde::Serialize;
pub use request::*;
pub use response::*;
use crate::jd::method::JDMethod;

#[derive(Debug, Clone)]
pub struct JDClient <T: SessionStore> {
    api_client: APIClient<T>,
}


pub trait JDRequest {

    ///
    /// 获取TOP的API名称。
    ///
    /// @return API名称
    fn get_api_method_name(&self) -> JDMethod;

    ///
    /// 获取所有的Key-Value形式的文本请求参数集合。其中：
    /// <ul>
    /// <li>Key: 请求参数名</li>
    /// <li>Value: 请求参数值</li>
    /// </ul>
    ///
    /// @return 文本请求参数集合
    fn get_text_params(&self) -> BTreeMap<String, String> {
        BTreeMap::default()
    }

    ///
    /// 得到当前接口的版本
    ///
    /// @return API版本
    fn get_api_version(&self) -> String {
        VERSION_1.to_string()
    }

    fn get_biz_content(&self) -> String where Self: Serialize + Sized {
        serde_json::to_string(self).unwrap_or_default()
    }

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
            api_client: APIClient::<SimpleStorage>::new::<Q, String, S>(app_key, secret, "https://api.jd.com/routerjson".to_owned()),
        }
    }


    pub fn from_session<Q: Into<String>, S: Into<String>>(app_key: Q, secret: S, session: T) -> JDClient<T> {
        JDClient {
            api_client: APIClient::from_session(app_key, secret, String::from("https://api.jd.com/routerjson"), session),
        }
    }

    /// 签名
    fn sign(&self, sign_content: &str) -> String {
        let content = format!("{}{}{}", self.api_client.secret.to_string(), sign_content, self.api_client.secret.to_string());
        let sign = md5::md5(content).to_uppercase();
        sign
    }

    fn get_request_url(&self, holder: &RequestParametersHolder) -> LabradorResult<String> {
        let mut url_sb = self.api_client.api_path.to_owned();
        // let sys_must = &holder.protocal_must_params;
        // let sys_must_query = serde_urlencoded::to_string(sys_must)?;
        // let opt_param = &holder.application_params;
        // let sys_opt_query = serde_urlencoded::to_string(opt_param)?;
        // url_sb += "?";
        // url_sb += &sys_must_query;
        // if !sys_opt_query.is_empty() {
        //     url_sb += "&";
        //     url_sb += &sys_opt_query;
        // }
        Ok(url_sb)
    }

    ///
    /// 组装接口参数，处理加密、签名逻辑
    ///
    /// @param request
    fn get_request_holder_with_sign<D>(&self, request: &D) -> LabradorResult<RequestParametersHolder> where D: JDRequest {
        let mut holder = RequestParametersHolder::new();
        let mut app_params = request.get_text_params();
        holder.set_application_params(app_params);

        let mut protocal_must_params = BTreeMap::new();
        protocal_must_params.insert(constants::METHOD.to_string(), request.get_api_method_name().get_method());
        protocal_must_params.insert(constants::VERSION.to_string(), request.get_api_version());
        protocal_must_params.insert(constants::APP_KEY.to_string(), self.api_client.app_key.to_owned());
        protocal_must_params.insert(constants::SIGN_METHOD.to_string(), SIGN_TYPE_MD5.to_string());
        protocal_must_params.insert(constants::TIMESTAMP.to_string(), Local::now().naive_local().format(constants::FORMAT_TIME).to_string());
        holder.set_protocal_must_params(protocal_must_params.to_owned());
        let pairs = holder.get_sorted_map();
        let sign_content = pairs.iter().filter(|(k, v)| !k.is_empty() && !v.is_empty()).map(|(k, v)| format!("{}{}", k, v)).collect::<Vec<String>>().join("");
        protocal_must_params.insert(constants::SIGN.to_string(), self.sign(&sign_content));
        holder.set_protocal_must_params(protocal_must_params);
        Ok(holder)
    }

    /// 发送请求数据
    async fn excute<D>(&self, request: D) -> LabradorResult<JDResponse> where D: JDRequest + Serialize {
        let method = request.get_api_method_name();
        let holder = self.get_request_holder_with_sign(&request)?;
        let url = self.get_request_url(&holder)?;
        let data = holder.get_sorted_map();
        let req = LabraRequest::new().url(url).method(Method::Post).data(data).req_type(RequestType::Form);
        let result = self.api_client.request(req).await?.text()?;
        JDResponse::parse(&result, method)
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
    ///     use labrador::{JdJFGoodsParam};
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
    pub async fn get_jf_select(&self, request: JdJFGoodsParam) -> LabradorResult<JdCommonResponse<Vec<JdJFGoodsSelect>>> {
        self.excute(JdJFGoodsRequest { goods_req: request }).await?.get_biz_model::<JdCommonResponse<Vec<JdJFGoodsSelect>>>(RESPONSE_QUERYRESULT.into())
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
    ///     use labrador::{JdGoodsInfoQueryRequest};
    /// 
    ///     async fn main() {
    ///         let param = JdGoodsInfoQueryRequest {
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
    pub async fn get_goods_detail(&self, request: JdGoodsInfoQueryRequest) -> LabradorResult<JdCommonResponse<Vec<JdGoodsInfoQuery>>> {
        self.excute(request).await?.get_biz_model::<JdCommonResponse<Vec<JdGoodsInfoQuery>>>(RESPONSE_QUERYRESULT.into())
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
    ///         match client.generate_promotion_url(param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    ///
    /// ```
    ///
    pub async fn generate_promotion_url(&self, request: JdPromotionUrlGenerateParam) -> LabradorResult<JdCommonResponse<JdPromotionUrlGenerateResponse>> {
        self.excute(JdPromotionUrlGenerateRequest{ promotion_code_req: request }).await?.get_biz_model::<JdCommonResponse<JdPromotionUrlGenerateResponse>>(RESPONSE_GETRESULT.into())
    }

    /// 订单查询
    ///
    /// 查询推广订单及佣金信息，可查询最近90天内下单的订单，会随着订单状态变化同步更新数据。支持按下单时间、完成时间或更新时间查询。建议按更新时间每分钟调用一次，查询最近一分钟的订单更新数据。
    /// 支持查询subunionid、推广位、PID参数，支持普通推客及工具商推客订单查询。
    /// 该接口即将下线，请使用订单行查询接口https://union.jd.com/openplatform/api/12707
    ///
    /// # 示例
    /// ```no_run
    ///
    ///     use labrador::JDClient;
    ///     use labrador::{JdOrderRecentQueryParam};
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
    #[deprecated]
    pub async fn query_recent_order(&self, request: JdOrderRecentQueryParam) -> LabradorResult<JdCommonResponse<Vec<JdOrderQueryResponse>>> {
        self.excute(JdOrderRequest { order_req: request}).await?.get_biz_model::<JdCommonResponse<Vec<JdOrderQueryResponse>>>(None)
    }

    /// 订单行查询
    ///
    /// 查询推广订单及佣金信息，可查询最近90天内下单的订单，会随着订单状态变化同步更新数据。
    /// 支持按下单时间、完成时间或更新时间查询。建议按更新时间每分钟调用一次，查询最近一分钟的订单更新数据。
    /// 支持查询subunionid、推广位、PID参数，支持普通推客及工具商推客订单查询。
    ///
    /// 如需要通过SDK调用此接口，请接入JOS [SDK](https://union.jd.com/helpcenter/13246-13312-108188)
    ///
    /// # 示例
    /// ```no_run
    ///
    ///     use labrador::JDClient;
    ///     use labrador::{JdOrderRawQueryParam};
    ///
    ///     async fn main() {
    ///         let param = JdOrderRawQueryParam {
    ///             page_index: 1.into(),
    ///             page_size: 1.into(),
    ///             bill_type: 1,
    ///             start_time: "".to_string(),
    ///             end_time: "".to_string(),
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
    pub async fn query_raw_order(&self, request: JdOrderRawQueryParam) -> LabradorResult<JdCommonResponse<Vec<JdOrderQueryResponse>>> {
        self.excute(JdOrderRawRequest { order_req: request}).await?.get_biz_model::<JdCommonResponse<Vec<JdOrderQueryResponse>>>(RESPONSE_QUERYRESULT.into())
    }

    /// 转链获取接口
    ///
    /// 转链获取，支持工具商
    /// [文档](https://jos.jd.com/apilist?apiGroupId=531&apiId=17775&apiName=jd.union.open.selling.promotion.get&apiGroupName=%E4%BA%AC%E4%B8%9C%E8%81%94%E7%9B%9Fapi)
    ///
    /// # 示例
    /// ```no_run
    ///
    ///     use labrador::JDClient;
    ///     use labrador::{JdPromotionCodeGetParam};
    ///
    ///     async fn main() {
    ///         let param = JdPromotionCodeGetParam {
    ///             material_id: "".to_string(),
    ///             site_id: "".to_string(),
    ///             chain_type: None,
    ///             coupon_url: None,
    ///             position_id: None,
    ///             sub_union_id: None,
    ///             ext1: None,pid: None,
    ///             union_id: None
    ///         };
    ///         let client = JDClient::new("appKey", "secret");
    ///         match client.get_promotion_code(param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    ///
    /// ```
    ///
    pub async fn get_promotion_code(&self, request: JdPromotionCodeGetParam) -> LabradorResult<JdCommonResponse<JdPromotionCodeGetResponse>> {
        self.excute(JdPromotionCodeGetRequest { req: request}).await?.get_biz_model::<JdCommonResponse<JdPromotionCodeGetResponse>>(RESPONSE_GETRESULT.into())
    }

    /// 社交媒体获取推广链接接口
    ///
    /// 通过商品链接、领券链接、活动链接获取普通推广链接或优惠券二合一推广链接，支持传入subunionid参数，可用于区分媒体自身的用户ID，该参数可在订单查询接口返回。
    /// 接口和subunionid参数需向cps-qxsq@jd.com申请权限。功能同宙斯接口的优惠券,商品二合一转接API-通过subUnionId获取推广链接、联盟微信手q通过subUnionId获取推广链接。
    /// [文档](https://jos.jd.com/apilist?apiGroupId=531&apiId=15157&apiName=jd.union.open.promotion.bysubunionid.get&apiGroupName=%E4%BA%AC%E4%B8%9C%E8%81%94%E7%9B%9Fapi)
    ///
    /// # 示例
    /// ```no_run
    ///
    ///     use labrador::JDClient;
    ///     use labrador::{JdPromotionBySubUnionIdGetParam};
    ///
    ///     async fn main() {
    ///         let param = JdPromotionBySubUnionIdGetParam {
    ///             material_id: "".to_string(),
    ///             chain_type: None,
    ///             coupon_url: None,
    ///             position_id: None,
    ///             sub_union_id: None,
    ///             pid: None,
    ///             channel_id: None,
    ///             command: None,
    ///             gift_coupon_key: None
    ///         };
    ///         let client = JDClient::new("appKey", "secret");
    ///         match client.get_promotion_code_by_sub_unionid(param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    ///
    /// ```
    ///
    pub async fn get_promotion_code_by_sub_unionid(&self, request: JdPromotionBySubUnionIdGetParam) -> LabradorResult<JdCommonResponse<JdPromotionCodeGetResponse>> {
        self.excute(JdPromotionBySubUnionIdGetRequest { promotion_code_req: request}).await?.get_biz_model::<JdCommonResponse<JdPromotionCodeGetResponse>>(RESPONSE_GETRESULT.into())
    }

    /// 商羚商品查询接口
    ///
    /// 通过SKUID查询商羚商品的名称、主图、类目、价格、30天销量等详细信息，支持批量查询
    /// [文档](https://jos.jd.com/apilist?apiGroupId=531&apiId=17769&apiName=jd.union.open.selling.goods.query&apiGroupName=%E4%BA%AC%E4%B8%9C%E8%81%94%E7%9B%9Fapi)
    ///
    /// # 示例
    /// ```no_run
    ///
    ///     use labrador::JDClient;
    ///     use labrador::{SellingGoodsQueryParam};
    ///
    ///     async fn main() {
    ///         let param = SellingGoodsQueryParam {
    ///             sku_ids: vec![]
    ///         };
    ///         let client = JDClient::new("appKey", "secret");
    ///         match client.get_selling_goods_query(param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    ///
    /// ```
    ///
    pub async fn get_selling_goods_query(&self, request: SellingGoodsQueryParam) -> LabradorResult<JdCommonResponse<Vec<SellingGoodsQueryResponse>>> {
        self.excute(SellingGoodsQueryRequest { req: request}).await?.get_biz_model::<JdCommonResponse<Vec<SellingGoodsQueryResponse>>>(RESPONSE_QUERYRESULT.into())
    }
}


#[cfg(test)]
#[allow(unused, non_snake_case)]
mod tests {
    use std::collections::BTreeMap;
    use std::fs::File;
    use std::io::Read;
    use std::ops::Add;
    use chrono::{DateTime, Local, NaiveDateTime, SecondsFormat};
    use reqwest::Url;
    use serde::{Deserializer, Deserialize, Serialize};
    use serde_json::{json, Value};
    use crate::ResponseType::Text;
    use crate::{SimpleStorage, JDClient, JdPromotionUrlGenerateRequest, JdPromotionUrlGenerateParam, JdOrderRecentQueryParam, JdOrderRawQueryParam};
    use crate::jd::request::{JdGoodsInfoQueryRequest, JdJFGoodsParam};

    #[test]
    fn test_get_jf_select() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let r = rt.spawn(async {
            let client =  JDClient::<SimpleStorage>::new("appkey", "secert");
            let param = JdOrderRawQueryParam {
                page_index: 1.into(),
                page_size: 10.into(),
                bill_type: 1,
                start_time: "2022-08-02 21:23:00".to_string(),
                end_time: "2022-08-02 21:43:00".to_string(),
                child_union_id: None,
                key: None,
                fields: None
            };
            let result = client.query_raw_order(param);
            match result.await {
                Ok(res) => {
                    println!("请求成功:{:?}",res);
                }
                Err(err) => {
                    println!("err:{:?}", err);
                }
            }
        });
        rt.block_on(r);
    }

    #[test]
    fn test_query_recent_order() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let r = rt.spawn(async {
            let client =  JDClient::<SimpleStorage>::new("appkey", "secert");
            let param = JdOrderRecentQueryParam {
                page_no: None,
                page_size: None,
                bill_type: 0,
                time: "".to_string(),
                child_union_id: None,
                key: None
            };
            let result = client.query_recent_order(param);
            match result.await {
                Ok(res) => {
                    println!("请求成功:{:?}",res);
                }
                Err(err) => {
                    println!("err:{:?}", err);
                }
            }
        });
        rt.block_on(r);
    }


    #[test]
    fn test_generate_promotion_url() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let r = rt.spawn(async {
            let client =  JDClient::<SimpleStorage>::new("appkey", "secert");
            let param = JdPromotionUrlGenerateParam {
                material_id: "https://item.jd.com/100023064623.html".to_string(),
                site_id: "1".to_string(),
                position_id: None,
                sub_union_id: None,
                ext1: None,
                pid: None,
                coupon_url: None,
                gift_coupon_key: None
            };
            let result = client.generate_promotion_url(param);
            match result.await {
                Ok(res) => {
                    println!("请求成功:{:?}",res);
                }
                Err(err) => {
                    println!("err:{:?}", err);
                }
            }
        });
        rt.block_on(r);
    }


    #[test]
    fn test_get_goods_detail() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let r = rt.spawn(async {
            let client =  JDClient::<SimpleStorage>::new("appkey", "secert");
            let param = JdGoodsInfoQueryRequest {
                sku_ids: "100023064623".to_string().into(),
            };
            let result = client.get_goods_detail(param);
            match result.await {
                Ok(res) => {
                    println!("请求成功:{:?}",res);
                }
                Err(err) => {
                    println!("err:{:?}", err);
                }
            }
        });
        rt.block_on(r);
    }


    #[test]
    fn test_get_jf_select1() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let r = rt.spawn(async {
            let client =  JDClient::<SimpleStorage>::new("appkey", "secert");
            let param = JdJFGoodsParam {
                elite_id: 22,
                page_index: 1.into(),
                page_size: 1.into(),
                sort_name: None,
                sort: None,
                pid: "1".to_owned().into(),
                fields: None,
                forbid_types: None,
            };
            let result = client.get_jf_select(param);
            match result.await {
                Ok(res) => {
                    println!("请求成功:{:?}",res);
                }
                Err(err) => {
                    println!("err:{:?}", err);
                }
            }
        });
        rt.block_on(r);
    }

}
