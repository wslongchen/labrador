use serde::{Deserialize, Serialize};
use serde_json::{Value as JsonValue};

use crate::{request::Response, errors::LabraError, LabradorResult};


//----------------------------------------------------------------------------------------------------------------------------

// 多多客商品推荐 ↓

#[derive(Debug, Deserialize,Serialize)]
pub struct PddGoodsRecommendResponse {
    /// 商品总数
    pub total: Option<u64>,
    /// 列表
    pub list: Option<Vec<PddRecommendItem>>,
    /// 翻页时必填前页返回的list_id值
    pub list_id: Option<String>,
    /// 搜索id，建议生成推广链接时候填写，提高收益。
    pub search_id: Option<String>,
}


#[derive(Debug, Deserialize,Serialize)]
pub struct PddRecommendItem {
    /// 商品类目id
    pub cat_id: Option<String>,
    /// 商品一~四级类目ID列表
    pub cat_ids: Option<Vec<u64>>,
    /// 商品一~四级标签类目ID列表
    pub opt_ids: Option<Vec<u64>>,
    /// 优惠券面额,单位为分
    pub coupon_discount: Option<u64>,
    /// 优惠券失效时间,UNIX时间戳
    pub coupon_end_time: Option<u64>,
    /// 优惠券门槛价格,单位为分
    pub coupon_min_order_amount: Option<u64>,
    /// 优惠券金额
    pub coupon_price: Option<u64>,
    /// 优惠券剩余数量
    pub coupon_remain_quantity: Option<u64>,
    /// 优惠券生效时间,UNIX时间戳
    pub coupon_start_time: Option<u64>,
    /// 优惠券总数量
    pub coupon_total_quantity: Option<u64>,
    /// 创建时间
    pub create_at: Option<u64>,
    /// 商品id
    pub goods_id: Option<u64>,
    /// 商品等级
    pub goods_rate: Option<u64>,
    /// 描述分
    pub desc_txt: Option<String>,
    /// 商品描述
    pub goods_desc: Option<String>,
    /// 商品详情图列表
    pub goods_gallery_urls: Option<String>,
    /// 商品主图
    pub goods_image_url: Option<String>,
    /// 商品名称
    pub goods_name: Option<String>,
    /// 商品goodsSign
    pub goods_sign: Option<String>,
    /// 商品缩略图
    pub goods_thumbnail_url: Option<String>,
    /// 商品类型
    pub goods_type: Option<u64>,
    /// 商家id
    pub mall_id: Option<u64>,
    /// 市场服务费
    pub market_fee: Option<u64>,
    /// 最小成团价格，单位分
    pub min_group_price: Option<u64>,
    /// 最小单买价格，单位分
    pub min_normal_price: Option<u64>,
    /// 比价行为预判定佣金，需要用户备案
    pub predict_promotion_rate: Option<u64>,
    /// 佣金比例,千分比
    pub promotion_rate: Option<u64>,
    /// 商品是否带券,true-带券,false-不带券
    pub has_coupon: Option<bool>,
    /// 物流分
    pub lgst_txt: Option<String>,
    /// 店铺名称
    pub mall_name: Option<String>,
    /// 商家类型
    pub merchant_type: Option<String>,
    /// 商品标签类目ID,使用pdd.goods.opt.get获取
    pub opt_id: Option<String>,
    /// 商品标签名
    pub opt_name: Option<String>,
    /// 二维码主图
    pub qr_code_image_url: Option<String>,
    /// 销售量
    pub sales_tip: Option<String>,
    /// 服务分
    pub serv_txt: Option<String>,
    /// 分享描述
    pub share_desc: Option<String>,
    /// 搜索id，建议生成推广链接时候填写，提高收益。
    pub search_id: Option<String>,
}

impl Response <PddGoodsRecommendResponse>  for JsonValue {
    fn parse_result(&self) -> LabradorResult<PddGoodsRecommendResponse> {
        serde_json::from_value::<PddGoodsRecommendResponse>(self.to_owned()).map_err(LabraError::from)
    }
}

//----------------------------------------------------------------------------------------------------------------------------


// 多多客商品搜索 ↓

#[derive(Debug, Deserialize,Serialize)]
pub struct PddGoodsSearchResponse {
    /// 商品总数
    pub total_count: Option<u64>,
    /// 列表
    pub goods_list: Option<Vec<PddGoodItem>>,
    /// 翻页时必填前页返回的list_id值
    pub list_id: Option<String>,
    /// 搜索id，建议生成推广链接时候填写，提高收益。
    pub search_id: Option<String>,
}


#[derive(Debug, Deserialize,Serialize)]
pub struct PddGoodItem {
    /// 活动商品标记数组，例：[4,7]，4-秒杀，7-百亿补贴，31-品牌黑标，10564-精选爆品-官方直推爆款
    /// 10584-精选爆品-团长推荐，24-品牌高佣，20-行业精选，21-金牌商家，10044-潜力爆品，10475-爆品上新，其他的值请忽略
    pub activity_tags: Option<Vec<u64>>,
    /// 服务标签: 4-送货入户并安装,5-送货入户,6-电子发票,9-坏果包赔,11-闪电退款,12-24小时发货,13-48小时发货,17-顺丰包邮
    /// 18-只换不修,19-全国联保,20-分期付款,24-极速退款,25-品质保障,26-缺重包退,27-当日发货,28-可定制化,29-预约配送
    /// 1000001-正品发票,1000002-送货入户并安装
    pub service_tags: Option<Vec<u64>>,
    /// 商品类目id
    pub cat_ids: Option<Vec<u64>>,
    /// 活动类型，0-无活动;1-秒杀;3-限量折扣;12-限时折扣;13-大促活动;14-名品折扣;15-品牌清仓;16-食品超市;
    /// 17-一元幸运团;18-爱逛街;19-时尚穿搭;20-男人帮;21-9块9;22-竞价活动;23-榜单活动;24-幸运半价购;25-定金预售;
    /// 26-幸运人气购;27-特色主题活动;28-断码清仓;29-一元话费;30-电器城;31-每日好店;32-品牌卡;101-大促搜索池;102-大促品类分会场;
    pub activity_type: Option<u64>,
    /// 店铺收藏券id
    pub clt_cpn_batch_sn: Option<String>,
    /// 店铺收藏券面额,单位为分
    pub clt_cpn_discount: Option<u64>,
    /// 店铺收藏券截止时间
    pub clt_cpn_end_time: Option<u64>,
    /// 店铺收藏券使用门槛价格,单位为分
    pub clt_cpn_min_amt: Option<u64>,
    /// 店铺收藏券总量
    pub clt_cpn_quantity: Option<u64>,
    /// 店铺收藏券剩余量
    pub clt_cpn_remain_quantity: Option<u64>,
    /// 店铺收藏券起始时间
    pub clt_cpn_start_time: Option<u64>,
    /// 优惠券面额，单位为分
    pub coupon_discount: Option<u64>,
    /// 优惠券失效时间，UNIX时间戳
    pub coupon_end_time: Option<u64>,
    /// 优惠券门槛价格，单位为分
    pub coupon_min_order_amount: Option<u64>,
    /// 优惠券剩余数量
    pub coupon_remain_quantity: Option<u64>,
    /// 优惠券生效时间，UNIX时间戳
    pub coupon_start_time: Option<u64>,
    /// 优惠券总数量
    pub coupon_total_quantity: Option<u64>,
    /// 创建时间（unix时间戳）
    pub create_at: Option<u64>,
    /// 描述分
    pub desc_txt: Option<String>,
    /// 商品描述
    pub goods_desc: Option<String>,
    /// 商品主图
    pub goods_image_url: Option<String>,
    /// 商品名称
    pub goods_name: Option<String>,
    /// 商品缩略图
    pub goods_thumbnail_url: Option<String>,
    /// 商品是否带券,true-带券,false-不带券
    pub has_coupon: Option<bool>,
    /// 是否有店铺券
    pub has_mall_coupon: Option<bool>,
    /// 物流分
    pub lgst_txt: Option<String>,
    /// 店铺券折扣
    pub mall_coupon_discount_pct: Option<u64>,
    /// 店铺券结束使用时间
    pub mall_coupon_end_time: Option<u64>,
    /// 店铺券id
    pub mall_coupon_id: Option<u64>,
    /// 最大使用金额
    pub mall_coupon_max_discount_amount: Option<u64>,
    /// 最小使用金额
    pub mall_coupon_min_order_amount: Option<u64>,
    /// 店铺券余量
    pub mall_coupon_remain_quantity: Option<u64>,
    /// 店铺券开始使用时间
    pub mall_coupon_start_time: Option<u64>,
    /// 店铺券总量
    pub mall_coupon_total_quantity: Option<u64>,
    /// 店铺id
    pub mall_id: Option<u64>,
    /// 该商品所在店铺是否参与全店推广，0：否，1：是
    pub mall_cps: Option<u8>,
    /// 店铺名称
    pub mall_name: Option<String>,
    /// 店铺类型，1-个人，2-企业，3-旗舰店，4-专卖店，5-专营店，6-普通店
    pub merchant_type: Option<u8>,
    /// 最小成团价格，单位分
    pub min_group_price: Option<u64>,
    /// 最小单买价格，单位分
    pub min_normal_price: Option<u64>,
    /// 快手专享
    pub only_scene_auth: Option<bool>,
    /// 商品标签名
    pub opt_name: Option<String>,
    /// 已售卖件数
    pub sales_tip: Option<String>,
    /// 搜索id，建议生成推广链接时候填写，提高收益
    pub search_id: Option<String>,
    /// 推广计划类型 3:定向 4:招商
    pub plan_type: Option<u8>,
    /// 佣金比例,千分比
    pub promotion_rate: Option<u64>,
    /// 招商团长id
    pub zs_duo_id: Option<u64>,
    /// 比价行为预判定佣金，需要用户备案
    pub predict_promotion_rate: Option<u64>,
    /// 商品标签ID，使用pdd.goods.opts.get接口获取
    pub opt_id: Option<u64>,
    /// 商品标签id
    pub opt_ids: Option<Vec<u64>>,
    /// 服务分
    pub serv_txt: Option<String>,
    /// 商品goodsSign
    pub goods_sign: Option<String>,
    /// 商品视频url
    pub video_urls: Option<String>,
    /// 优惠标签列表
    pub unified_tags: Option<Vec<String>>,
}

impl Response <PddGoodsSearchResponse>  for JsonValue {
    fn parse_result(&self) -> LabradorResult<PddGoodsSearchResponse> {
        serde_json::from_value::<PddGoodsSearchResponse>(self.to_owned()).map_err(LabraError::from)
    }
}
//----------------------------------------------------------------------------------------------------------------------------


// 拼多多备案查询 ↓

#[derive(Debug, Deserialize,Serialize)]
pub struct PddAuthorityQueryResponse {
    /// 1-已绑定；0-未绑定
    pub bind: Option<u8>,
}

impl Response <PddAuthorityQueryResponse>  for JsonValue {
    fn parse_result(&self) -> LabradorResult<PddAuthorityQueryResponse> {
        serde_json::from_value::<PddAuthorityQueryResponse>(self.to_owned()).map_err(LabraError::from)
    }
}

//----------------------------------------------------------------------------------------------------------------------------


// 拼多多备案查询 ↓

#[derive(Debug, Deserialize,Serialize)]
pub struct PddPromotionUrlGenerateResponse {
    /// 多多进宝推广链接对象列表
    pub goods_promotion_url_list: Option<Vec<GoodsPromotionUrl>>,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct GoodsPromotionUrl {
    /// 对应出参mobile_url的短链接
    pub mobile_short_url: Option<String>,
    /// 使用此推广链接，用户安装拼多多APP的情况下会唤起APP，否则唤起H5页面
    pub mobile_url: Option<String>,
    /// 使用此推广链接，用户安装拼多多APP的情况下会唤起APP（需客户端支持schema跳转协议）
    pub schema_url: Option<String>,
    /// 对应出参url的短链接
    pub short_url: Option<String>,
    /// 普通推广长链接，唤起H5页面
    pub url: Option<String>,
    pub we_app_info: Option<WeAppInfo>,
    pub qq_app_info: Option<QQAppInfo>,

}

#[derive(Debug, Deserialize,Serialize)]
pub struct QQAppInfo {
    /// 拼多多小程序id
    pub app_id: Option<String>,
    /// Banner图
    pub banner_url: Option<String>,
    /// 描述
    pub desc: Option<String>,
    /// 小程序path值
    pub page_path: Option<String>,
    /// 小程序icon
    pub qq_app_icon_url: Option<String>,
    /// 来源名
    pub source_display_name: Option<String>,
    /// 小程序标题
    pub title: Option<String>,
    /// 用户名
    pub user_name: Option<String>,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct WeAppInfo {
    /// 拼多多小程序id
    pub app_id: Option<String>,
    /// Banner图
    pub banner_url: Option<String>,
    /// 描述
    pub desc: Option<String>,
    /// 小程序path值
    pub page_path: Option<String>,
    /// 小程序图片
    pub we_app_icon_url: Option<String>,
    /// 来源名
    pub source_display_name: Option<String>,
    /// 小程序标题
    pub title: Option<String>,
    /// 用户名
    pub user_name: Option<String>,
}

impl Response <PddPromotionUrlGenerateResponse>  for JsonValue {
    fn parse_result(&self) -> LabradorResult<PddPromotionUrlGenerateResponse> {
        serde_json::from_value::<PddPromotionUrlGenerateResponse>(self.to_owned()).map_err(LabraError::from)
    }
}
//----------------------------------------------------------------------------------------------------------------------------


// 拼多多产品详情 ↓

#[derive(Debug, Deserialize,Serialize)]
pub struct PddGoodsDetailResponse {
    /// 多多进宝商品对象列表
    pub goods_details: Option<Vec<PddGoodsDetail>>,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct PddGoodsDetail {
    /// 活动商品标记数组，例：[4,7]，4-秒杀，7-百亿补贴，31-品牌黑标，10564-精选爆品-官方直推爆款
    /// 10584-精选爆品-团长推荐，24-品牌高佣，20-行业精选，21-金牌商家，10044-潜力爆品，10475-爆品上新，其他的值请忽略
    pub activity_tags: Option<Vec<u64>>,
    /// 服务标签: 4-送货入户并安装,5-送货入户,6-电子发票,9-坏果包赔,11-闪电退款,12-24小时发货,13-48小时发货,17-顺丰包邮
    /// 18-只换不修,19-全国联保,20-分期付款,24-极速退款,25-品质保障,26-缺重包退,27-当日发货,28-可定制化,29-预约配送
    /// 1000001-正品发票,1000002-送货入户并安装
    pub service_tags: Option<Vec<u64>>,
    /// 商品类目id
    pub cat_ids: Option<Vec<u64>>,
    /// 活动类型，0-无活动;1-秒杀;3-限量折扣;12-限时折扣;13-大促活动;14-名品折扣;15-品牌清仓;16-食品超市;
    /// 17-一元幸运团;18-爱逛街;19-时尚穿搭;20-男人帮;21-9块9;22-竞价活动;23-榜单活动;24-幸运半价购;25-定金预售;
    /// 26-幸运人气购;27-特色主题活动;28-断码清仓;29-一元话费;30-电器城;31-每日好店;32-品牌卡;101-大促搜索池;102-大促品类分会场;
    pub activity_type: Option<u64>,
    /// 店铺收藏券id
    pub clt_cpn_batch_sn: Option<String>,
    /// 店铺收藏券面额,单位为分
    pub clt_cpn_discount: Option<u64>,
    /// 店铺收藏券截止时间
    pub clt_cpn_end_time: Option<u64>,
    /// 店铺收藏券使用门槛价格,单位为分
    pub clt_cpn_min_amt: Option<u64>,
    /// 店铺收藏券总量
    pub clt_cpn_quantity: Option<u64>,
    /// 店铺收藏券剩余量
    pub clt_cpn_remain_quantity: Option<u64>,
    /// 店铺收藏券起始时间
    pub clt_cpn_start_time: Option<u64>,
    /// 优惠券面额，单位为分
    pub coupon_discount: Option<u64>,
    /// 优惠券失效时间，UNIX时间戳
    pub coupon_end_time: Option<u64>,
    /// 优惠券门槛价格，单位为分
    pub coupon_min_order_amount: Option<u64>,
    /// 优惠券剩余数量
    pub coupon_remain_quantity: Option<u64>,
    /// 优惠券生效时间，UNIX时间戳
    pub coupon_start_time: Option<u64>,
    /// 优惠券总数量
    pub coupon_total_quantity: Option<u64>,
    /// 创建时间（unix时间戳）
    pub create_at: Option<u64>,
    /// 描述分
    pub desc_txt: Option<String>,
    /// 商品描述
    pub goods_desc: Option<String>,
    /// 商品主图
    pub goods_image_url: Option<String>,
    /// 商品名称
    pub goods_name: Option<String>,
    /// 商品缩略图
    pub goods_thumbnail_url: Option<String>,
    /// 商品是否带券,true-带券,false-不带券
    pub has_coupon: Option<bool>,
    /// 是否有店铺券
    pub has_mall_coupon: Option<bool>,
    /// 物流分
    pub lgst_txt: Option<String>,
    /// 店铺券折扣
    pub mall_coupon_discount_pct: Option<u64>,
    /// 店铺券结束使用时间
    pub mall_coupon_end_time: Option<u64>,
    /// 店铺券id
    pub mall_coupon_id: Option<u64>,
    /// 最大使用金额
    pub mall_coupon_max_discount_amount: Option<u64>,
    /// 最小使用金额
    pub mall_coupon_min_order_amount: Option<u64>,
    /// 店铺券余量
    pub mall_coupon_remain_quantity: Option<u64>,
    /// 店铺券开始使用时间
    pub mall_coupon_start_time: Option<u64>,
    /// 店铺券总量
    pub mall_coupon_total_quantity: Option<u64>,
    /// 店铺id
    pub mall_id: Option<u64>,
    /// 该商品所在店铺是否参与全店推广，0：否，1：是
    pub mall_cps: Option<u8>,
    /// 店铺名称
    pub mall_name: Option<String>,
    /// 店铺类型，1-个人，2-企业，3-旗舰店，4-专卖店，5-专营店，6-普通店
    pub merchant_type: Option<u8>,
    /// 最小成团价格，单位分
    pub min_group_price: Option<u64>,
    /// 最小单买价格，单位分
    pub min_normal_price: Option<u64>,
    /// 快手专享
    pub only_scene_auth: Option<bool>,
    /// 商品标签名
    pub opt_name: Option<String>,
    /// 已售卖件数
    pub sales_tip: Option<String>,
    /// 搜索id，建议生成推广链接时候填写，提高收益
    pub search_id: Option<String>,
    /// 推广计划类型 3:定向 4:招商
    pub plan_type: Option<u8>,
    /// 佣金比例,千分比
    pub promotion_rate: Option<u64>,
    /// 招商团长id
    pub zs_duo_id: Option<u64>,
    /// 比价行为预判定佣金，需要用户备案
    pub predict_promotion_rate: Option<u64>,
    /// 商品标签ID，使用pdd.goods.opts.get接口获取
    pub opt_id: Option<u64>,
    /// 商品类目ID，使用pdd.goods.cats.get接口获取
    pub cat_id: Option<u64>,
    /// 参与多多进宝的商品ID
    pub goods_id: Option<u64>,
    /// 商品标签id
    pub opt_ids: Option<Vec<u64>>,
    /// 服务分
    pub serv_txt: Option<String>,
    /// 商品goodsSign
    pub goods_sign: Option<String>,
    /// 商品视频url
    pub video_urls: Option<Vec<String>>,
    /// 优惠标签列表
    pub unified_tags: Option<Vec<String>>,
    /// 商品轮播图
    pub goods_gallery_urls: Option<Vec<String>>,
}

impl Response <PddGoodsDetailResponse>  for JsonValue {
    fn parse_result(&self) -> LabradorResult<PddGoodsDetailResponse> {
        serde_json::from_value::<PddGoodsDetailResponse>(self.to_owned()).map_err(LabraError::from)
    }
}

//----------------------------------------------------------------------------------------------------------------------------


// 多多进宝转链接口 ↓

#[derive(Debug, Deserialize,Serialize)]
pub struct PddZsUrlGenerateResponse {
    /// 推广短链接（可唤起拼多多app）
    pub mobile_short_url: Option<String>,
    /// 推广长链接（唤起拼多多app）
    pub mobile_url: Option<String>,
    /// 推广短链接（唤起拼多多app）
    pub multi_group_mobile_short_url: Option<String>,
    /// 推广长链接（可唤起拼多多app）
    pub multi_group_mobile_url: Option<String>,
    /// 双人团推广短链接
    pub multi_group_short_url: Option<String>,
    /// 双人团推广长链接
    pub multi_group_url: Option<String>,
    /// 单人团推广短链接
    pub short_url: Option<String>,
    /// 单人团推广长链接
    pub url: Option<String>,
}


impl Response <PddZsUrlGenerateResponse>  for JsonValue {
    fn parse_result(&self) -> LabradorResult<PddZsUrlGenerateResponse> {
        serde_json::from_value::<PddZsUrlGenerateResponse>(self.to_owned()).map_err(LabraError::from)
    }
}


//----------------------------------------------------------------------------------------------------------------------------


// 多多客获取爆款排行商品 ↓

#[derive(Debug, Deserialize,Serialize)]
pub struct PddGoodsTopResponse {
    /// 商品总数
    pub total: Option<u64>,
    /// 列表
    pub list: Option<Vec<PddTopItem>>,
    /// 翻页时必填前页返回的list_id值
    pub list_id: Option<String>,
    /// 搜索id，建议生成推广链接时候填写，提高收益。
    pub search_id: Option<String>,
}


#[derive(Debug, Deserialize,Serialize)]
pub struct PddTopItem {
    /// 商品类目id
    pub cat_id: Option<String>,
    /// 商品一~四级类目ID列表
    pub cat_ids: Option<Vec<u64>>,
    /// 商品一~四级标签类目ID列表
    pub opt_ids: Option<Vec<u64>>,
    /// 优惠券面额,单位为分
    pub coupon_discount: Option<u64>,
    /// 优惠券失效时间,UNIX时间戳
    pub coupon_end_time: Option<u64>,
    /// 优惠券门槛价格,单位为分
    pub coupon_min_order_amount: Option<u64>,
    /// 优惠券金额
    pub coupon_price: Option<u64>,
    /// 优惠券剩余数量
    pub coupon_remain_quantity: Option<u64>,
    /// 优惠券生效时间,UNIX时间戳
    pub coupon_start_time: Option<u64>,
    /// 优惠券总数量
    pub coupon_total_quantity: Option<u64>,
    /// 创建时间
    pub create_at: Option<u64>,
    /// 商品id
    pub goods_id: Option<u64>,
    /// 商品等级
    pub goods_rate: Option<u64>,
    /// 描述分
    pub desc_txt: Option<String>,
    /// 商品描述
    pub goods_desc: Option<String>,
    /// 商品详情图列表
    pub goods_gallery_urls: Option<Vec<String>>,
    /// 商品主图
    pub goods_image_url: Option<String>,
    /// 商品名称
    pub goods_name: Option<String>,
    /// 商品goodsSign
    pub goods_sign: Option<String>,
    /// 商品缩略图
    pub goods_thumbnail_url: Option<String>,
    /// 商品类型
    pub goods_type: Option<u64>,
    /// 商家id
    pub mall_id: Option<u64>,
    /// 市场服务费
    pub market_fee: Option<u64>,
    /// 最小成团价格，单位分
    pub min_group_price: Option<u64>,
    /// 最小单买价格，单位分
    pub min_normal_price: Option<u64>,
    /// 比价行为预判定佣金，需要用户备案
    pub predict_promotion_rate: Option<u64>,
    /// 佣金比例,千分比
    pub promotion_rate: Option<u64>,
    /// 商品是否带券,true-带券,false-不带券
    pub has_coupon: Option<bool>,
    /// 物流分
    pub lgst_txt: Option<String>,
    /// 店铺名称
    pub mall_name: Option<String>,
    /// 商家类型
    pub merchant_type: Option<String>,
    /// 商品标签类目ID,使用pdd.goods.opt.get获取
    pub opt_id: Option<u64>,
    /// 商品标签名
    pub opt_name: Option<String>,
    /// 二维码主图
    pub qr_code_image_url: Option<String>,
    /// 销售量
    pub sales_tip: Option<String>,
    /// 服务分
    pub serv_txt: Option<String>,
    /// 分享描述
    pub share_desc: Option<String>,
    /// 搜索id，建议生成推广链接时候填写，提高收益。
    pub search_id: Option<String>,
}

impl Response <PddGoodsTopResponse>  for JsonValue {
    fn parse_result(&self) -> LabradorResult<PddGoodsTopResponse> {
        serde_json::from_value::<PddGoodsTopResponse>(self.to_owned()).map_err(LabraError::from)
    }
}


//----------------------------------------------------------------------------------------------------------------------------


// 生成营销工具推广链接 ↓

#[derive(Debug, Deserialize,Serialize)]
pub struct PddRpUrlGenerateResponse {
    /// 列表
    pub resource_list: Option<ResourceList>,
    pub url_list: Option<Vec<UrlList>>,

}


#[derive(Debug, Deserialize,Serialize)]
pub struct ResourceList {
    /// 活动描述
    pub desc: Option<String>,
    /// 活动地址
    pub url: Option<String>,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct UrlList {
    /// 对应出参mobile_url的短链接
    pub mobile_short_url: Option<String>,
    /// 使用此推广链接，用户安装拼多多APP的情况下会唤起APP，否则唤起H5页面
    pub mobile_url: Option<String>,
    /// 推广多人团移动短链接
    pub multi_group_mobile_short_url: Option<String>,
    /// 推广多人团移动链接
    pub multi_group_mobile_url: Option<String>,
    /// 推广多人团短链接
    pub multi_group_short_url: Option<String>,
    /// 推广多人团链接
    pub multi_group_url: Option<String>,
    pub qq_app_info: Option<QQAppInfo>,
    pub we_app_info: Option<WeAppInfo>,
    /// 使用此推广链接，用户安装拼多多APP的情况下会唤起APP（需客户端支持schema跳转协议）
    pub schema_url: Option<String>,
    /// 对应出参url的短链接
    pub short_url: Option<String>,
    /// 普通推广长链接，唤起H5页面
    pub url: Option<String>,
}

impl Response <PddRpUrlGenerateResponse>  for JsonValue {
    fn parse_result(&self) -> LabradorResult<PddRpUrlGenerateResponse> {
        serde_json::from_value::<PddRpUrlGenerateResponse>(self.to_owned()).map_err(LabraError::from)
    }
}

//----------------------------------------------------------------------------------------------------------------------------


// 生成商城-频道推广链接 ↓

#[derive(Debug, Deserialize,Serialize)]
pub struct PddCmsUrlGenerateResponse {
    /// 列表
    pub total: Option<u64>,
    pub url_list: Option<Vec<UrlList>>,
    pub multi_url_list: Option<UrlList>,

}

#[derive(Debug, Deserialize,Serialize)]
pub struct CmsUrlList {
    /// 唤醒拼多多app短链
    pub mobile_short_url: Option<String>,
    /// 唤醒拼多多app链接
    pub mobile_url: Option<String>,
    /// 多人团唤醒拼多多app链接
    pub multi_group_mobile_short_url: Option<String>,
    /// 多人团唤醒拼多多app长链接
    pub multi_group_mobile_url: Option<String>,
    /// 多人团短链
    pub multi_group_short_url: Option<String>,
    pub single_url_list: Option<UrlList>,
    /// 多人团长链
    pub multi_group_url: Option<UrlList>,
    /// 双人团链接列表
    pub multi_url_list: Option<String>,
    /// CPSsign
    pub sign: Option<String>,
    /// 对应出参url的短链接
    pub short_url: Option<String>,
    /// 普通推广长链接，唤起H5页面
    pub url: Option<String>,	
}

impl Response <PddCmsUrlGenerateResponse>  for JsonValue {
    fn parse_result(&self) -> LabradorResult<PddCmsUrlGenerateResponse> {
        serde_json::from_value::<PddCmsUrlGenerateResponse>(self.to_owned()).map_err(LabraError::from)
    }
}


//----------------------------------------------------------------------------------------------------------------------------


// 用时间段查询推广订单 | 最后更新时间段增量同步推广订单信息 | 订单详情 ↓

#[derive(Debug, Deserialize,Serialize)]
pub struct PddOrderRangeQueryResponse {
    /// last_order_id
    pub last_order_id: Option<String>,
    /// 多多进宝推广位对象列表
    pub order_list: Option<Vec<PddOrderDetail>>,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct PddOrderIncrementQueryResponse {
    /// 请求到的结果数
    pub total_count: Option<u64>,
    /// 多多进宝推广位对象列表
    pub order_list: Option<Vec<PddOrderDetail>>,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct PddOrderDetail {
    /// 多多客工具id
    pub auth_duo_id: Option<u64>,
    /// 商品ID
    pub goods_id: Option<u64>,
    /// 购买商品的数量
    pub goods_quantity: Option<u64>,
    /// 成团编号
    pub group_id: Option<u64>,
    /// 订单中sku的单件价格，单位为分
    pub goods_price: Option<u64>,
    /// 实际支付金额，单位为分
    pub order_amount: Option<u64>,
    /// 是否是 cpa 新用户，1表示是，0表示否
    pub cpa_new: Option<u8>,
    /// 是否直推 ，1表示是，0表示否
    pub is_direct: Option<u8>,
    /// 结算批次号
    pub batch_no: Option<String>,
    /// 自定义参数
    pub custom_parameters: Option<String>,
    /// 商品缩略图
    pub goods_thumbnail_url: Option<String>,
    /// 商品goodsSign
    pub goods_sign: Option<String>,
    /// 订单审核失败原因
    pub fail_reason: Option<String>,
    /// 商品标题
    pub goods_name: Option<String>,
    /// 商品一~四级类目ID列表
    pub cat_ids: Option<Vec<u64>>,
    /// 订单生成时间，UNIX时间戳
    pub order_create_time: Option<u64>,
    /// 成团时间
    pub order_group_success_time: Option<u64>,
    /// 最后更新时间
    pub order_modify_at: Option<u64>,
    /// 支付时间
    pub order_pay_time: Option<u64>,
    /// 确认收货时间
    pub order_receive_time: Option<u64>,
    /// 结算时间
    pub order_settle_time: Option<u64>,
    /// 推广订单编号
    pub order_sn: Option<String>,
    /// 订单状态描述
    pub order_status_desc: Option<String>,
    /// 推广位ID
    pub p_id: Option<String>,
    /// 订单状态： -1 未支付; 0-已支付；1-已成团；2-确认收货；3-审核成功；4-审核失败（不可提现）；5-已经结算；8-非多多进宝商品（无佣金订单）
    pub order_status: Option<u64>,
    /// 比价状态：0：正常，1：比价
    pub price_compare_status: Option<u8>,
    /// 审核时间
    pub order_verify_time: Option<u64>,
    /// 佣金金额，单位为分
    pub promotion_amount: Option<u64>,
    /// 佣金比例，千分比
    pub promotion_rate: Option<u64>,
    /// 直播间订单推广duoId
    pub sep_duo_id: Option<u64>,
    /// 直播间推广佣金
    pub sep_market_fee: Option<u64>,
    /// 招商多多客id
    pub zs_duo_id: Option<u64>,
    /// 直播间推广佣金比例
    pub sep_rate: Option<u64>,
    /// 直播间推广自定义参数
    pub sep_parameters: Option<String>,
    /// 直播间订单推广位
    pub sep_pid: Option<String>,
    /// 订单类型：0：领券页面， 1： 红包页， 2：领券页， 3： 题页
    #[serde(rename = "type")]
    pub bill_type: Option<String>,
    /// 订单ID
    pub order_id: Option<String>,
}

impl Response <PddOrderRangeQueryResponse>  for JsonValue {
    fn parse_result(&self) -> LabradorResult<PddOrderRangeQueryResponse> {
        serde_json::from_value::<PddOrderRangeQueryResponse>(self.to_owned()).map_err(LabraError::from)
    }
}

impl Response <PddOrderIncrementQueryResponse>  for JsonValue {
    fn parse_result(&self) -> LabradorResult<PddOrderIncrementQueryResponse> {
        serde_json::from_value::<PddOrderIncrementQueryResponse>(self.to_owned()).map_err(LabraError::from)
    }
}

impl Response <PddOrderDetail>  for JsonValue {
    fn parse_result(&self) -> LabradorResult<PddOrderDetail> {
        serde_json::from_value::<PddOrderDetail>(self.to_owned()).map_err(LabraError::from)
    }
}


//----------------------------------------------------------------------------------------------------------------------------


// 拼多多-创建多多进宝推广位 ↓

#[derive(Debug, Deserialize,Serialize)]
pub struct PddPidGenerateResponse {
    /// PID剩余数量
    pub remain_pid_count: Option<u64>,
    /// 多多进宝推广位对象列表
    pub p_id_list: Option<Vec<PidList>>,

}

#[derive(Debug, Deserialize,Serialize)]
pub struct PidList {
    /// 推广位创建时间
    pub create_time: Option<u64>,
    /// 媒体id
    pub media_id: Option<u64>,
    /// 推广位名称
    pub pid_name: Option<String>,
    /// 调用方推广位ID
    pub p_id: Option<String>,
    /// 推广位状态：0-正常，1-封禁
    pub status: Option<u8>,
}


impl Response <PddPidGenerateResponse>  for JsonValue {
    fn parse_result(&self) -> LabradorResult<PddPidGenerateResponse> {
        serde_json::from_value::<PddPidGenerateResponse>(self.to_owned()).map_err(LabraError::from)
    }
}


//----------------------------------------------------------------------------------------------------------------------------


// 查询已经生成的推广位信息 ↓
#[derive(Debug, Deserialize,Serialize)]
pub struct PddPidQueryResponse {
    /// 返回推广位总数
    pub total_count: Option<u64>,
    /// 多多进宝推广位对象列表
    pub p_id_list: Option<Vec<PidList>>,
}

impl Response <PddPidQueryResponse>  for JsonValue {
    fn parse_result(&self) -> LabradorResult<PddPidQueryResponse> {
        serde_json::from_value::<PddPidQueryResponse>(self.to_owned()).map_err(LabraError::from)
    }
}

//----------------------------------------------------------------------------------------------------------------------------


// 批量绑定推广位的媒体id ↓
#[derive(Debug, Deserialize,Serialize)]
pub struct PddPidBindMediaResponse {
    /// 绑定结果文本提示
    pub msg: Option<String>,
    /// 绑定结果
    pub result: Option<bool>,
}

impl Response <PddPidBindMediaResponse>  for JsonValue {
    fn parse_result(&self) -> LabradorResult<PddPidBindMediaResponse> {
        if let Some(res) = self.get("result") {
            let msg = if let Some(msg) = res.get("msg") {
                msg.as_str().unwrap_or_default().to_owned()
            } else {
                String::default()
            };
            let result = if let Some(result) = res.get("msg") {
                result.as_bool().unwrap_or_default().to_owned()
            } else {
                false
            };
            Ok(PddPidBindMediaResponse{ msg: msg.into(), result: result.into()})
        } else {
            Err(LabraError::MissingField("返回结果有误，缺少字段。".to_owned()))
        }
    }
}
