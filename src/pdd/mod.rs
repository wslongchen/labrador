use serde_json::{Value as JsonValue};
use std::collections::BTreeMap;
use serde::Serialize;

use crate::{client::APIClient, util::{get_timestamp, get_sign}, request::{Params, RequestType, Method, Response, LabraRequest, RequestMethod}, errors::LabraError, session::{SessionStore, SimpleStorage}, LabradorResult};

use self::{method::PDDMethod, request::{PddPidQueryParam, PddPidBindMediaParam, PddPidGenerateParam, PddOrderDetailParam, PddOrderIncrementQueryParam, PddOrderRangeQueryParam, PddCmsUrlGenerateParam, PddZsUrlGenerateParam, PddGoodsDetailParam, PddRpUrlGenerateParam, PddPromoteUrlGenerateParam, PddAuthorityQueryParam, PddGoodsSearchParam, PddGoodsTopParam, PddGoodsRecommendParam}, response::{PddPidQueryResponse, PddPidBindMediaResponse, PddPidGenerateResponse, PddOrderDetail, PddOrderIncrementQueryResponse, PddOrderRangeQueryResponse, PddCmsUrlGenerateResponse, PddZsUrlGenerateResponse, PddGoodsDetailResponse, PddRpUrlGenerateResponse, PddPromotionUrlGenerateResponse, PddAuthorityQueryResponse, PddGoodsSearchResponse, PddGoodsTopResponse, PddGoodsRecommendResponse}};

mod request;
mod response;
mod method;

#[derive(Debug, Clone)]
pub struct PDDClient<T: SessionStore> {
    api_client: APIClient<T>
}

/// PDDClient
/// 
/// 
/// # Example
/// 
/// ```no_run
/// use labrador::PDDClient;
/// async fn main() {
///     let client = PDDClient::new("appKey", "secret");
///     // Do Some Thing You Want
///     // ...
/// }
/// ```
/// 
#[allow(unused)]
impl <T: SessionStore> PDDClient<T> {

    pub fn new<Q: Into<String>, S: Into<String>>(app_key: Q, secret: S) -> PDDClient<SimpleStorage> {
        PDDClient {
            api_client: APIClient::<SimpleStorage>::new::<Q, String, S>(app_key, secret, String::from("https://gw-api.pinduoduo.com/api/router"))
        }
    }

    pub fn from_session<Q: Into<String>, S: Into<String>>(app_key: Q, secret: S, session: T) -> PDDClient<T> {
        PDDClient {
            api_client: APIClient::from_session(app_key, secret, String::from("https://gw-api.pinduoduo.com/api/router"), session)
        }
    }

    #[inline]
    fn build_common_params(&self) -> Vec<(String, String)> {
        // build common params
        let mut params: Vec<(String, String)> = Vec::new();
        params.push(("client_id".to_owned(), self.api_client.app_key.to_owned()));
        params.push(("timestamp".to_owned(), get_timestamp().to_string()));
        params.push(("data_type".to_owned(), "JSON".to_owned()));
        // params.push(("version".to_owned(), "V1".to_owned()));
        params
    }

    /// 根据方法名发送请求
    async fn send<D: Serialize + Params>(&self, method: PDDMethod, data: D) -> LabradorResult<JsonValue> {
        let mut params = self.build_common_params();
        let method_name_str = method.get_method();
        params.push(("type".to_owned(), method_name_str));
        let request_type = RequestType::Json;
        params.extend_from_slice(data.get_params().as_slice());
        // build sign
        let mut pairs = BTreeMap::new();
        for (key, value) in params.iter() {
            pairs.insert(key.to_string(), value.to_string());
        }
        let sign = get_sign(&pairs, self.api_client.secret.to_owned().as_str());
        params.push(("sign".to_owned(), sign));
        let result = self.api_client.request(LabraRequest::new().method(Method::Post).data(data).req_type(request_type).params(params)).await?.json::<serde_json::Value>()?;
        self.json_decode(result, &method.get_response_key())
    }


    #[inline]
    fn json_decode(&self, obj: JsonValue, response_key: &String) -> LabradorResult<JsonValue> {
        match obj.get("error_response") {
            Some(error_response) => {
                let errcode = if let Some(code) = error_response.get("error_code") {
                    code.as_i64().unwrap_or_default() as i32
                } else {
                    0
                };
                if errcode != 0 {
                    let errmsg = match error_response.get("sub_msg") {
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
                Ok(response.to_owned())
            },
            None => {
                Err(LabraError::ClientError { errcode: "-3".to_string(), errmsg: format!("Response decode error") })
            }
        }
    }

    /// 获取多多客推荐的商品
    /// 
    /// # 示例
    /// 
    /// ```no_run
    /// 
    ///     use labrador::PDDClient;
    ///     use labrador::{PddGoodsRecommendParam};
    /// 
    ///     async fn main() {
    ///         let param = PddGoodsRecommendParam {
    ///             limit: 1.into(),
    ///             cat_id: None,
    ///             channel_type: None,
    ///             offset: None,
    ///             goods_ids: None,
    ///             goods_sign_list: None,
    ///             pid: "14171776_184065943".to_owned().into(),
    ///             custom_parameters: None,
    ///             list_id: None,
    ///         };
    ///         let client = PDDClient::new("appKey", "secret");
    ///         match block_on(client.get_goods_recommend(param)) {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    /// 
    /// ```
    /// 
    pub async fn get_goods_recommend(&self, param: PddGoodsRecommendParam) -> LabradorResult<PddGoodsRecommendResponse> {
        self.send(PDDMethod::GoodsRecommend, param).await?.parse_result()
    }

    /// 多多客获取爆款排行商品接口
    /// 
    /// 用于查询进宝网站热销榜单商品列表(入参sort_type：1-实时热销榜；2-实时收益榜)
    /// 
    /// # 示例
    /// 
    /// ```no_run
    /// 
    ///     use labrador::PDDClient;
    ///     use labrador::{PddGoodsRecommendParam};
    /// 
    ///     async fn main() {
    ///         let param = PddGoodsTopParam {
    ///             p_id: "14171776_184065943".to_owned().into(),
    ///             custom_parameters: "{\"uid\":\"454\"}".to_owned().into(),
    ///             limit: 10.into(),
    ///             offset: 0.into(),
    ///             sort_type: 1.into(),
    ///             list_id: None,
    ///         };
    ///         let client = PDDClient::new("appKey", "secret");
    ///         match block_on(client.get_goods_top(param)) {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    /// 
    /// ```
    /// 
    pub async fn get_goods_top(&self, param: PddGoodsTopParam) -> LabradorResult<PddGoodsTopResponse> {
        self.send(PDDMethod::GoodsTop, param).await?.parse_result()
    }
    /// 多多客商品搜索
    /// 
    /// 一、支持入参筛选条件有：标题；拼团价/券后价；佣金比例/佣金金额区间；优惠券金额区间；是否有优惠券；商品标签；商品类目。
    /// 某一个标签id的商品、某一个分类id的商品。二者的区别在于：标签是一个商品有多个同级标签，例：某商品既属于一级标签服饰，也属于一级标签男装。 分类是一个商品仅有一个同级分类。
    /// 获取标签id/分类id的方法，请查看以下接口
    /// 商品API——》pdd.goods.opt.get（查询商品标签列表）
    /// 进宝网站单品推广首页类目映射对应如下：
    ///
    /// opt_id	15	4	1	14	18	1281	1282	16	743	13	818	2478	1451	590	2048	1917	2974	3279
    /// 网站类目映射	百货	母婴	食品	女装	电器	鞋包	内衣	美妆	男装	水果	家纺	文具	运动	虚拟	汽车	家装	家具	医药
    /// 二、支持入参排序条件有：由入参range_list字段控制，range_id映射值如下：0-最小成团价； 1-券后价； 2-佣金比例； 3-优惠券价格； 4-广告创建时间； 5-销量； 6-佣金金额；
    /// 7-店铺描述分； 8-店铺物流分； 9-店铺服务分； 10-店铺描述分击败同行业百分比； 11-店铺物流分击败同行业百分比； 12-店铺服务分击败同行业百分比； 13-商品分； 17-优惠券/最小团购价； 18-过去两小时pv ；19-过去两小时销量
    /// 
    /// # 示例
    /// 
    /// ```no_run
    /// 
    ///     use labrador::PDDClient;
    ///     use labrador::{PddGoodsSearchParam};
    /// 
    ///     async fn main() {
    ///         let param = PddGoodsSearchParam {
    ///             limit: 1.into(),
    ///             cat_id: None,
    ///             channel_type: None,
    ///             offset: None,
    ///             goods_ids: None,
    ///             goods_sign_list: None,
    ///             pid: "14171776_184065943".to_owned().into(),
    ///             custom_parameters: None,
    ///             list_id: None,
    ///         };
    ///         let client = PDDClient::new("appKey", "secret");
    ///         match block_on(client.search_goods(param)) {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    /// 
    /// ```
    /// 
    pub async fn search_goods(&self, param: PddGoodsSearchParam) -> LabradorResult<PddGoodsSearchResponse> {
        self.send(PDDMethod::GoodsSearch, param).await?.parse_result()
    }

    /// 拼多多备案查询
    /// 
    /// 用于通过pid和自定义参数来查询是否已经绑定备案
    /// 
    /// # 示例
    /// 
    /// ```no_run
    /// 
    ///     use labrador::PDDClient;
    ///     use labrador::{PddAuthorityQueryParam};
    /// 
    ///     async fn main() {
    ///         let param = PddAuthorityQueryParam {
    ///             pid: "14171776_184065943".to_owned().into(),
    ///             custom_parameters: None,
    ///         };
    ///         let client = PDDClient::new("appKey", "secret");
    ///         match block_on(client.query_authority(param)) {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    /// 
    /// ```
    /// 
    pub async fn query_authority(&self, param: PddAuthorityQueryParam) -> LabradorResult<PddAuthorityQueryResponse> {
        self.send(PDDMethod::AuthorityQuery, param).await?.parse_result()
    }

    /// 多多进宝推广链接生成
    /// 
    /// 1）目前支持生成单人团商品推广链接和双人团推广链接。二者的区别是:
    ///    单人团是用户可以无需拼团，只接用拼团价购买商品
    ///    双人团是用户开团后分享给好友参团，好友参团后推手可获得双份佣金
    ///2）推广链接类型有2种：普通链接、唤起拼多多app链接。其中，
    ///    普通链接用于微信内环境使用
    ///    唤起拼多多app链接用于非微信内环境。目前支持两种方式唤醒拼多多APP：唤起APPH5和schemaURL，您可根据推广方式自由选择。
    /// 
    /// # 示例
    /// 
    /// ```no_run
    /// 
    ///     use labrador::PDDClient;
    ///     use labrador::{PddPromoteUrlGenerateParam};
    /// 
    ///     async fn main() {
    ///         let param = PddPromoteUrlGenerateParam {
    ///             p_id: "14171776_184065943".to_owned().into(),
    ///             custom_parameters: "{\"uid\":\"454\"}".to_owned().into(),
    ///             crash_gift_id:  None,
    ///             generate_authority_url:  true.into(),
    ///             generate_mall_collect_coupon:  None,
    ///             generate_qq_app:  None,
    ///             generate_schema_url:  None,
    ///             generate_short_url:  None,
    ///             generate_we_app:  None,
    ///             multi_group:  None,
    ///             goods_id_list:  vec![201252887350].into(),
    ///             zs_duo_id:  None,
    ///             goods_sign:  "c9T2jLHVpaJKD4bhwfbY-x1fgxld_Jimi5z0Th".to_owned().into(),
    ///             room_id_list:  None,
    ///             search_id:  None,
    ///             target_id_list:  None,
    ///         };
    ///         let client = PDDClient::new("appKey", "secret");
    ///         match block_on(client.query_authority(param)) {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    /// 
    /// ```
    /// 
    pub async fn generate_promote_url(&self, param: PddPromoteUrlGenerateParam) -> LabradorResult<PddPromotionUrlGenerateResponse> {
        self.send(PDDMethod::PromotionUrlGenerate, param).await?.parse_result()
    }


    /// 生成营销工具推广链接
    /// 
    /// 用于您生成多多进宝营销工具的推广链接。
    /// （入参channel_type：-1-活动列表，0-默认红包，2–新人红包，3-刮刮卡，5-员工内购，6-购物车，7-大促会场，8-直播间列表集合页，10-生成绑定备案链接，12-砸金蛋）
    /// 
    /// # 示例
    /// 
    /// ```no_run
    /// 
    ///     use labrador::PDDClient;
    ///     use labrador::{PddRpUrlGenerateParam};
    /// 
    ///     async fn main() {
    ///         let param = PddRpUrlGenerateParam {
    ///             channel_type: 1.into(),
    ///             custom_parameters: "{\"uid\":\"454\"}".to_owned().into(),
    ///             diy_red_packet_param: None,
    ///             generate_qq_app:  None,
    ///             generate_schema_url:  None,
    ///             generate_short_url:  true.into(),
    ///             generate_we_app:  None,
    ///             p_id_list: vec!["60005_612".to_owned()].into(),
    ///             amount: None,
    ///             scratch_card_amount: None,
    ///         };
    ///         let client = PDDClient::new("appKey", "secret");
    ///         match block_on(client.query_authority(param)) {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    /// 
    /// ```
    /// 
    pub async fn generate_rp_promote_url(&self, param: PddRpUrlGenerateParam) -> LabradorResult<PddRpUrlGenerateResponse> {
        self.send(PDDMethod::RpPromoteUrlGenerate, param).await?.parse_result()
    }

    /// 多多进宝商品详情查询
    /// 
    /// 用于本接口用于查询某个商品的详情信息：商品标题 - 商品描述 商品分类/标签
    /// 最小拼团价、最小单买价、商品历史累计售卖件数（模糊值，string类型，非数值）
    /// 优惠券金额、优惠券数量、优惠券剩余数量、优惠券门槛、优惠券有效期
    /// 商品评价分、商品评价数量、佣金比例、商品轮播图、商品主图。用于首页
    /// 店铺dsr、店铺名称
    /// 
    /// # 示例
    /// 
    /// ```no_run
    /// 
    ///     use labrador::PDDClient;
    ///     use labrador::{PddGoodsDetailParam};
    /// 
    ///     async fn main() {
    ///         let param = PddGoodsDetailParam {
    ///             pid: "14171776_184065943".to_owned().into(),
    ///             custom_parameters: "{\"uid\":\"454\"}".to_owned().into(),
    ///             goods_id_list: vec![201252887350].into(),
    ///             goods_sign: None,
    ///             search_id: None,
    ///             zs_duo_id: None,
    ///         };
    ///         let client = PDDClient::new("appKey", "secret");
    ///         match block_on(client.get_goods_detail(param)) {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    /// 
    /// ```
    /// 
    pub async fn get_goods_detail(&self, param: PddGoodsDetailParam) -> LabradorResult<PddGoodsDetailResponse> {
        self.send(PDDMethod::GoodsDetail, param).await?.parse_result()
    }

    /// 多多进宝转链
    /// 
    /// 用于将其他推广者的推广链接直接转换为自己的，如果您的推广场景为采集群，可直接使用此接口
    /// 
    /// # 示例
    /// 
    /// ```no_run
    /// 
    ///     use labrador::PDDClient;
    ///     use labrador::{PddZsUrlGenerateParam};
    /// 
    ///     async fn main() {
    ///         let param = PddZsUrlGenerateParam {
    ///             pid: "14171776_184065943".to_owned().into(),
    ///             custom_parameters: "{\"uid\":\"454\"}".to_owned().into(),
    ///             source_url: "https://p.pinduoduo.com/a5Iy6wZZ".to_owned().into(),
    ///         };
    ///         let client = PDDClient::new("appKey", "secret");
    ///         match block_on(client.get_zs_url_generate(param)) {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    /// 
    /// ```
    /// 
    pub async fn get_zs_url_generate(&self, param: PddZsUrlGenerateParam) -> LabradorResult<PddZsUrlGenerateResponse> {
        self.send(PDDMethod::ZsUrlGenerate, param).await?.parse_result()
    }

    /// 生成商城-频道推广链接
    /// 
    /// # 示例
    /// 
    /// ```no_run
    /// 
    ///     use labrador::PDDClient;
    ///     use labrador::{PddCmsUrlGenerateParam};
    /// 
    ///     async fn main() {
    ///         let param = PddCmsUrlGenerateParam {
    ///             pid: "14171776_184065943".to_owned().into(),
    ///             custom_parameters: "{\"uid\":\"454\"}".to_owned().into(),
    ///             source_url: "https://p.pinduoduo.com/a5Iy6wZZ".to_owned().into(),
    ///         };
    ///         let client = PDDClient::new("appKey", "secret");
    ///         match block_on(client.get_cms_url_generate(param)) {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    /// 
    /// ```
    /// 
    pub async fn get_cms_url_generate(&self, param: PddCmsUrlGenerateParam) -> LabradorResult<PddCmsUrlGenerateResponse> {
        self.send(PDDMethod::CmsUrlGenerate, param).await?.parse_result()
    }


    /// 用时间段查询推广订单
    /// 
    /// 此方法可以订单支付时间为维度供您同步订单，一般情况下，您用上述增量订单更新接口同步即可
    /// 在每月月结等有大量订单发生更新的情况，如您用上述接口同步压力较大，可更换为此接口同步。
    /// 
    /// # 示例
    /// 
    /// ```no_run
    /// 
    ///     use labrador::PDDClient;
    ///     use labrador::{PddOrderRangeQueryParam};
    /// 
    ///     async fn main() {
    ///         let param = PddOrderRangeQueryParam {
    ///         last_order_id: None,
    ///         start_time: "".to_owned().into(),
    ///         end_time: "".to_owned().into(),
    ///         page_size: 20.into(),
    ///         query_order_type: 1.into(),
    ///     };
    ///         let client = PDDClient::new("appKey", "secret");
    ///         match block_on(client.get_order_list(param)) {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    /// 
    /// ```
    /// 
    pub async fn get_order_list(&self, param: PddOrderRangeQueryParam) -> LabradorResult<PddOrderRangeQueryResponse> {
        self.send(PDDMethod::OrderRangeQuery, param).await?.parse_result()
    }

    /// 最后更新时间段增量同步推广订单信息
    /// 
    /// 1）订单信息主要包括：
    /// 订单状态以及各状态变更时间
    /// 订单支付金额、佣金比例以及金额
    /// 订单所属商品编号、标题、缩略图
    /// 
    /// 2）订单状态包括：
    /// 已支付
    /// 已成团：单人团支付成功后会状态会马上置为成团；双人团支付后需有其他人参团状态才会置为成团
    /// 确认收货
    /// 审核成功：确认收货15天后若未发生售后，订单状态会置为审核成功
    /// 审核失败（不可提现）：若订单发生售后成功，订单状态会置为审核失败。
    /// 已经结算：每月20号会结算当月15号及以前审核通过的订单，状态并置为已结算
    /// 非多多进宝商品（无佣金订单）：用户访问推广链接时，该商品不在多多进宝推广计划中，因此购买商品产生的订单为非多多进宝订单。
    /// 已处罚：当判定某个pid在拼多多主站站内导流，该pid所有的订单均不会结算佣金，并会置为已处罚状态。
    /// 
    /// # 示例
    /// 
    /// ```no_run
    /// 
    ///     use labrador::PDDClient;
    ///     use labrador::{PddOrderIncrementQueryParam};
    /// 
    ///     async fn main() {
    ///         let param = PddOrderIncrementQueryParam {
    ///         page_size: 20.into(),
    ///         query_order_type: 1.into(),
    ///         end_update_time: 0,
    ///         start_update_time: 0,
    ///         return_count: true.into(),
    ///         page: 1.into(),
    ///     };
    ///         let client = PDDClient::new("appKey", "secret");
    ///         match block_on(client.get_increment_order_list(param)) {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    /// 
    /// ```
    /// 
    pub async fn get_increment_order_list(&self, param: PddOrderIncrementQueryParam) -> LabradorResult<PddOrderIncrementQueryResponse> {
        self.send(PDDMethod::OrderRangeQuery, param).await?.parse_result()
    }

    /// 查询订单详情
    /// 
    /// 用于查询单笔订单详情
    /// 接口场景：当您出现疑似丢单情况，即用户产生的订单在您的订单库或者接口里没有捞取到，此时，您可用这个接口进行验证，传入该笔订单号
    /// 若返回的所有字段皆不为空，则该笔订单归属为你，您可再次通过订单接口捞取确认；若返回部分字段为空，则该笔订单不归属于您
    /// 
    /// # 示例
    /// 
    /// ```no_run
    /// 
    ///     use labrador::PDDClient;
    ///     use labrador::{PddOrderDetailParam};
    /// 
    ///     async fn main() {
    ///         let param = PddOrderDetailParam {
    ///         query_order_type: 1.into(),
    ///         order_sn: "".into(),
    ///     };
    ///         let client = PDDClient::new("appKey", "secret");
    ///         match block_on(client.get_order_detail(param)) {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    /// 
    /// ```
    /// 
    pub async fn get_order_detail(&self, param: PddOrderDetailParam) -> LabradorResult<PddOrderDetail> {
        self.send(PDDMethod::OrderDetail, param).await?.parse_result()
    }
    
    /// 创建多多进宝推广位
    /// 
    /// 用于您创建推广位
    /// 推广位的用途：
    /// 1、与自己的用户体系进行关联，做代理/分销模式。 举例：多多客为推手用户A生成一个pid =1_1 , 为推手用户B生成pid=1_2。若A是B的上级，当用户通过pid=1_2的推广链接买了商品。由于用户和pid进行了映射关系，所以可以实现结算部分佣金给到A。
    /// 2、用于识别各投放资源位效果。 举例：将投放至群A的推广链接由pid=1_1生成，投放至群B的推广链接由pid=1_2生成。由于订单查询接口可以区分订单由某个pid推广产生，因此多多客可以统计查看群A和群B的推广效果数据。
    /// 
    /// # 示例
    /// 
    /// ```no_run
    /// 
    ///     use labrador::PDDClient;
    ///     use labrador::{PddPidGenerateParam};
    /// 
    ///     async fn main() {
    ///         let param = PddPidGenerateParam {
    ///         number: 1,
    ///         p_id_name_list: vec!["测试".to_owned()].into(),
    ///         media_id: 1.into(),
    ///     };
    ///         let client = PDDClient::new("appKey", "secret");
    ///         match block_on(client.pid_generate(param)) {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    /// 
    /// ```
    /// 
    pub async fn pid_generate(&self, param: PddPidGenerateParam) -> LabradorResult<PddPidGenerateResponse> {
        self.send(PDDMethod::PidGenerate, param).await?.parse_result()
    }

    /// 查询已经生成的推广位信息
    /// 
    /// 用于您查询已经生成的推广位信息（推广位列表、推广位名称、剩余可用推广位数量，请注意，您的推广位数量有限，初始只有30万个，请谨慎使用）
    /// 
    /// # 示例
    /// 
    /// ```no_run
    /// 
    ///     use labrador::PDDClient;
    ///     use labrador::{PddPidQueryParam};
    /// 
    ///     async fn main() {
    ///         let param = PddPidQueryParam {
    ///         page_size: 1.into(),
    ///         page: 1.into(),
    ///         status: None,
    ///         pid_list: vec!["14171776_184065943".to_owned()].into(),
    ///     };
    ///         let client = PDDClient::new("appKey", "secret");
    ///         match block_on(client.pid_query(param)) {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    /// 
    /// ```
    /// 
    pub async fn pid_query(&self, param: PddPidQueryParam) -> LabradorResult<PddPidQueryResponse> {
        self.send(PDDMethod::PidQuery, param).await?.parse_result()
    }

    /// 批量绑定推广位的媒体id
    /// 
    /// # 示例
    /// 
    /// ```no_run
    /// 
    ///     use labrador::PDDClient;
    ///     use labrador::{PddPidBindMediaParam};
    /// 
    ///     async fn main() {
    ///         let param = PddPidBindMediaParam {
    ///         pid_list: vec!["14171776_184065943".to_owned()],
    ///         media_id: 1,
    ///     };
    ///         let client = PDDClient::new("appKey", "secret");
    ///         match block_on(client.pid_bind_media(param)) {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    /// 
    /// ```
    /// 
    pub async fn pid_bind_media(&self, param: PddPidBindMediaParam) -> LabradorResult<PddPidBindMediaResponse> {
        self.send(PDDMethod::PidBindMedia, param).await?.parse_result()
    }
}