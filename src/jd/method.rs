use crate::RequestMethod;

#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum JDMethod {
    /// 京粉精选商品查询
    FanGoodsSelect,
    /// 根据skuid查询商品信息
    GoodsInfoQuery,
    /// 网站/APP获取推广链接接口
    PromotionUrlGenerate,
    /// 订单查询接口
    OrderRecentQuery,
    /// 订单行查询
    OrderRawQuery,
}

#[allow(unused)]
impl RequestMethod for JDMethod {
    fn get_method(&self) -> String {
        match *self {
            JDMethod::FanGoodsSelect => String::from("jd.union.open.goods.jingfen.query"),
            JDMethod::GoodsInfoQuery => String::from("jd.union.open.goods.promotiongoodsinfo.query"),
            JDMethod::PromotionUrlGenerate => String::from("jd.union.open.promotion.common.get"),
            JDMethod::OrderRecentQuery => String::from("jd.union.open.order.query"),
            JDMethod::OrderRawQuery => String::from("jd.union.open.order.row.query"),
           
        }
    }

    fn get_response_key(&self) -> String {
        match *self {
            JDMethod::FanGoodsSelect => String::from("jd_union_open_goods_jingfen_query_response"),
            JDMethod::GoodsInfoQuery => String::from("jd_union_open_goods_promotiongoodsinfo_query_response"),
            JDMethod::PromotionUrlGenerate => String::from("jd_union_open_promotion_common_get_response"),
            JDMethod::OrderRecentQuery => String::from("jd_union_open_order_query_response"),
            JDMethod::OrderRawQuery => String::from("jd_union_open_order_row_query_response"),
        }
    }
}