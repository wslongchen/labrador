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
    #[deprecated]
    OrderRecentQuery,
    /// 订单行查询
    OrderRawQuery,
    /// 转链获取接口
    PromotionCodeGet,
    /// 社交媒体获取推广链接接口
    PromotionBySubUnionIdGet,
    /// 商羚商品查询接口
    SellingGoodsQuery,
}

#[allow(unused,deprecated)]
impl RequestMethod for JDMethod {
    fn get_method(&self) -> String {
        match *self {
            JDMethod::FanGoodsSelect => String::from("jd.union.open.goods.jingfen.query"),
            JDMethod::GoodsInfoQuery => String::from("jd.union.open.goods.promotiongoodsinfo.query"),
            JDMethod::PromotionUrlGenerate => String::from("jd.union.open.promotion.common.get"),
            JDMethod::OrderRecentQuery => String::from("jd.union.open.order.query"),
            JDMethod::OrderRawQuery => String::from("jd.union.open.order.row.query"),
            JDMethod::PromotionCodeGet => String::from("jd.union.open.selling.promotion.get"),
            JDMethod::PromotionBySubUnionIdGet => String::from("jd.union.open.promotion.bysubunionid.get"),
            JDMethod::SellingGoodsQuery => String::from("jd.union.open.selling.goods.query"),

        }
    }

    fn get_response_key(&self) -> String {
        match *self {
            // TODO: 京东居然返回的response写成了responce，服了
            JDMethod::FanGoodsSelect => String::from("jd_union_open_goods_jingfen_query_responce"),
            JDMethod::GoodsInfoQuery => String::from("jd_union_open_goods_promotiongoodsinfo_query_responce"),
            JDMethod::PromotionUrlGenerate => String::from("jd_union_open_promotion_common_get_responce"),
            JDMethod::OrderRecentQuery => String::from("jd_union_open_order_query_responce"),
            JDMethod::OrderRawQuery => String::from("jd_union_open_order_row_query_responce"),
            JDMethod::PromotionCodeGet => String::from("jd_union_open_selling_promotion_get_responce"),
            JDMethod::PromotionBySubUnionIdGet => String::from("jd_union_open_promotion_bysubunionid_get_response"),
            JDMethod::SellingGoodsQuery => String::from("jd_union_open_selling_goods_query_response"),
        }
    }
}