use crate::RequestMethod;

#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum PDDMethod {
    /// 多多进宝商品推荐
    GoodsRecommend,
    /// 多多进宝商品搜索
    GoodsSearch,
    /// 多多进宝推广链接生成
    PromotionUrlGenerate,
    /// 查询是否绑定备案
    AuthorityQuery,
    /// 多多客获取爆款排行商品
    GoodsTop,
    /// 多多进宝商品详情查询
    GoodsDetail,
    /// 生成营销工具推广链接
    RpPromoteUrlGenerate,
    /// 多多进宝转链接口
    ZsUrlGenerate,
    /// 生成商城-频道推广链接
    CmsUrlGenerate,
    /// 用时间段查询推广订单接口
    OrderRangeQuery,
    /// 最后更新时间段增量同步推广订单信息
    OrderIncrementQuery,
    /// 订单详情
    OrderDetail,
    /// 创建多多进宝推广位
    PidGenerate,
    /// 查询已经生成的推广位信息
    PidQuery,
    /// 批量绑定推广位的媒体id
    PidBindMedia,
}

#[allow(unused)]
impl RequestMethod for PDDMethod {
    fn get_method(&self) -> String {
        match *self {
            PDDMethod::GoodsRecommend => String::from("pdd.ddk.goods.recommend.get"),
            PDDMethod::GoodsSearch => String::from("pdd.ddk.goods.search"),
            PDDMethod::PromotionUrlGenerate => String::from("pdd.ddk.goods.promotion.url.generate"),
            PDDMethod::AuthorityQuery => String::from("pdd.ddk.member.authority.query"),
            PDDMethod::GoodsTop => String::from("pdd.ddk.top.goods.list.query"),
            PDDMethod::RpPromoteUrlGenerate => String::from("pdd.ddk.rp.prom.url.generate"),
            PDDMethod::GoodsDetail => String::from("pdd.ddk.goods.detail"),
            PDDMethod::ZsUrlGenerate => String::from("pdd.ddk.goods.zs.unit.url.gen"),
            PDDMethod::CmsUrlGenerate => String::from("pdd.ddk.cms.prom.url.generate"),
            PDDMethod::OrderRangeQuery => String::from("pdd.ddk.order.list.range.get"),
            PDDMethod::OrderDetail => String::from("pdd.ddk.order.detail.get"),
            PDDMethod::OrderIncrementQuery => String::from("pdd.ddk.order.list.increment.get"),
            PDDMethod::PidGenerate => String::from("pdd.ddk.goods.pid.generate"),
            PDDMethod::PidQuery => String::from("pdd.ddk.goods.pid.query"),
            PDDMethod::PidBindMedia => String::from("pdd.ddk.pid.mediaid.bind"),
        }
    }

    fn get_response_key(&self) -> String {
        match *self {
            PDDMethod::GoodsRecommend => String::from("goods_basic_detail_response"),
            PDDMethod::GoodsSearch => String::from("goods_search_response"),
            PDDMethod::PromotionUrlGenerate => String::from("goods_promotion_url_generate_response"),
            PDDMethod::AuthorityQuery => String::from("authority_query_response"),
            PDDMethod::RpPromoteUrlGenerate => String::from("rp_promotion_url_generate_response"),
            PDDMethod::GoodsTop => String::from("top_goods_list_get_response"),
            PDDMethod::GoodsDetail => String::from("goods_detail_response"),
            PDDMethod::ZsUrlGenerate => String::from("goods_zs_unit_generate_response"),
            PDDMethod::CmsUrlGenerate => String::from("cms_promotion_url_generate_response"),
            PDDMethod::OrderRangeQuery | PDDMethod::OrderIncrementQuery => String::from("order_list_get_response"),
            PDDMethod::OrderDetail => String::from("order_detail_response"),
            PDDMethod::PidGenerate => String::from("p_id_generate_response"),
            PDDMethod::PidQuery => String::from("p_id_query_response"),
            PDDMethod::PidBindMedia => String::from("p_id_bind_response"),
        }
    }
}