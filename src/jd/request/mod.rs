use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use crate::jd::method::JDMethod;
use crate::JDRequest;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JdJFGoodsRequest {
    pub goods_req: JdJFGoodsParam,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JdJFGoodsParam {
    /// 频道ID:1-好券商品,2-精选卖场,10-9.9包邮,15-京东配送,22-实时热销榜,23-为你推荐,24-数码家电,25-超市,26-母婴玩具
    /// 27-家具日用,28-美妆穿搭,30-图书文具,31-今日必推,32-京东好物,33-京东秒杀,34-拼购商品,40-高收益榜,41-自营热卖榜
    /// 108-秒杀进行中,109-新品首发,110-自营,112-京东爆品,125-首购商品,129-高佣榜单,130-视频商品,153-历史最低价商品榜，210-极速版商品
    pub elite_id: u64,
    /// 页码，默认1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_index: Option<u64>,
    /// 每页数量，默认20，上限50，建议20
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u64>,
    /// 排序字段(price：单价, commissionShare：佣金比例, commission：佣金， inOrderCount30DaysSku：sku维度30天引单量，comments：评论数，goodComments：好评数)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_name: Option<String>,
    /// asc,desc升降序,默认降序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// 联盟id_应用id_推广位id，三段式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<String>,
    /// 支持出参数据筛选，逗号','分隔，目前可用：videoInfo,documentInfo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    /// 10微信京东购物小程序禁售，11微信京喜小程序禁售
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forbid_types: Option<String>,
}

impl JDRequest for JdJFGoodsRequest {
    fn get_api_method_name(&self) -> JDMethod {
        JDMethod::FanGoodsSelect
    }

    fn get_text_params(&self) -> BTreeMap<String, String> {
        BTreeMap::from([("360buy_param_json".to_owned(), serde_json::to_string(self).unwrap_or_default())])
    }
}
//----------------------------------------------------------------------------------------------------------------------------


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JdGoodsInfoQueryRequest {
    /// 京东skuID串，逗号分割，最多100个，开发示例如param_json={'skuIds':'5225346,7275691'}（
    /// 非常重要 请大家关注：如果输入的sk串中某个skuID的商品不在推广中[就是没有佣金]，返回结果中不会包含这个商品的信息）
    pub sku_ids: String,
}

impl JDRequest for JdGoodsInfoQueryRequest {
    fn get_api_method_name(&self) -> JDMethod {
        JDMethod::GoodsInfoQuery
    }

    fn get_text_params(&self) -> BTreeMap<String, String> {
        BTreeMap::from([("360buy_param_json".to_owned(), serde_json::to_string(self).unwrap_or_default())])
    }
}

//----------------------------------------------------------------------------------------------------------------------------



#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JdPromotionUrlGenerateRequest {
    /// 请求入参
    pub promotion_code_req: JdPromotionUrlGenerateParam,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JdPromotionUrlGenerateParam {
    /// 推广物料url，例如活动链接、商品链接等；不支持仅传入skuid
    pub material_id: String,
    /// 网站ID/APP ID，入口：京东联盟-推广管理-网站管理/APP管理-查看网站ID/APP ID
    /// （1、接口禁止使用导购媒体id入参；2、投放链接的网址或应用必须与传入的网站ID/AppID备案一致，否则订单会判“无效-来源与备案网址不符”）
    pub site_id: String,
    /// 推广位id
    pub position_id: Option<u64>,
    /// 子渠道标识，您可自定义传入字母、数字或下划线，最多支持80个字符，该参数会在订单行查询接口中展示
    /// （需申请权限，申请方法请见https://union.jd.com/helpcenter/13246-13247-46301）
    pub sub_union_id: Option<String>,
    /// 系统扩展参数（需申请权限，申请方法请见https://union.jd.com/helpcenter/13246-13247-46301）
    /// 最多支持40字符，参数会在订单行查询接口中展示
    pub ext1: Option<String>,
    /// 联盟子推客身份标识（不能传入接口调用者自己的pid）
    pub pid: Option<String>,
    /// 优惠券领取链接，在使用优惠券、商品二合一功能时入参，且materialId须为商品详情页链接
    pub coupon_url: Option<String>,
    /// 礼金批次号
    pub gift_coupon_key: Option<String>,
}

impl JDRequest for JdPromotionUrlGenerateRequest {
    fn get_api_method_name(&self) -> JDMethod {
        JDMethod::PromotionUrlGenerate
    }

    fn get_text_params(&self) -> BTreeMap<String, String> {
        BTreeMap::from([("360buy_param_json".to_owned(), serde_json::to_string(self).unwrap_or_default())])
    }
}

//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JdOrderRequest {
    pub order_req: JdOrderRecentQueryParam,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JdOrderRecentQueryParam {
    /// 页码，返回第几页结果
    pub page_no: Option<u64>,
    /// 每页包含条数，上限为500
    pub page_size: Option<u64>,
    /// 订单时间查询类型(1：下单时间，2：完成时间（购买用户确认收货时间），3：更新时间
    #[serde(rename = "type")]
    pub bill_type: u8,
    /// 查询时间，建议使用分钟级查询
    /// 格式：yyyyMMddHH、yyyyMMddHHmm或yyyyMMddHHmmss，如201811031212 的查询范围从12:12:00--12:12:59
    pub time: String,
    /// 子推客unionID，传入该值可查询子推客的订单
    /// 注意不可和key同时传入。（需联系运营开通PID权限才能拿到数据）
    pub child_union_id: Option<u64>,
    /// 工具商传入推客的授权key，可帮助该推客查询订单，注意不可和childUnionid同时传入。
    /// （需联系运营开通工具商权限才能拿到数据）
    pub key: Option<String>,
}

#[allow(deprecated)]
impl JDRequest for JdOrderRequest {
    fn get_api_method_name(&self) -> JDMethod {
        JDMethod::OrderRecentQuery
    }

    fn get_text_params(&self) -> BTreeMap<String, String> {
        BTreeMap::from([("360buy_param_json".to_owned(), serde_json::to_string(self).unwrap_or_default())])
    }
}

//----------------------------------------------------------------------------------------------------------------------------


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JdOrderRawQueryParam {
    /// 页码
    pub page_index: Option<u64>,
    /// 每页包含条数，上限为500
    pub page_size: Option<u64>,
    /// 订单时间查询类型(1：下单时间，2：完成时间（购买用户确认收货时间），3：更新时间
    #[serde(rename = "type")]
    pub bill_type: u8,
    /// 开始时间 格式yyyy-MM-dd HH:mm:ss，与endTime间隔不超过1小时
    pub start_time: String,
    /// 结束时间 格式yyyy-MM-dd HH:mm:ss，与startTime间隔不超过1小时
    pub end_time: String,
    /// 子推客unionID，传入该值可查询子推客的订单
    /// 注意不可和key同时传入。（需联系运营开通PID权限才能拿到数据）
    pub child_union_id: Option<u64>,
    /// 工具商传入推客的授权key，可帮助该推客查询订单，注意不可和childUnionid同时传入。
    /// （需联系运营开通工具商权限才能拿到数据）
    pub key: Option<String>,
    /// 支持出参数据筛选，逗号','分隔，目前可用：goodsInfo（商品信息）,categoryInfo(类目信息）
    pub fields: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JdOrderRawRequest {
    pub order_req: JdOrderRawQueryParam,
}


impl JDRequest for JdOrderRawRequest {
    fn get_api_method_name(&self) -> JDMethod {
        JDMethod::OrderRawQuery
    }

    fn get_text_params(&self) -> BTreeMap<String, String> {
        BTreeMap::from([("360buy_param_json".to_owned(), serde_json::to_string(self).unwrap_or_default())])
    }
}

//----------------------------------------------------------------------------------------------------------------------------

