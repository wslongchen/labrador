use crate::RequestMethod;

#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum TaobaoMethod {
    /// 物料精选
    MaterialSelect,
    /// 物料搜索
    MaterialSearch,
    /// 聚划算商品搜索
    JhsSearch,
    /// 淘宝客商品详情查询(简版)
    ItemDetail,
    /// 阿里妈妈推广券详情查询
    CouponDetail,
    /// 创建淘口令
    CreateTPwd,
    /// 淘宝客-公用-长链转短链
    SpreadGet,
    /// 淘宝客-推广者-淘口令回流数据查询
    GetTPwdReport,
    /// 淘宝客-推广者-官方活动转链
    GetActivityInfo,
}

#[allow(unused)]
impl RequestMethod for TaobaoMethod {
    fn get_method(&self) -> String {
        match *self {
            TaobaoMethod::MaterialSelect => String::from("taobao.tbk.dg.optimus.material"),
            TaobaoMethod::MaterialSearch => String::from("taobao.tbk.dg.material.optional"),
            TaobaoMethod::JhsSearch => String::from("taobao.ju.items.search"),
            TaobaoMethod::ItemDetail => String::from("taobao.tbk.item.info.get"),
            TaobaoMethod::CouponDetail => String::from("taobao.tbk.coupon.get"),
            TaobaoMethod::CreateTPwd => String::from("taobao.tbk.tpwd.create"),
            TaobaoMethod::SpreadGet => String::from("taobao.tbk.spread.get"),
            TaobaoMethod::GetTPwdReport => String::from("taobao.tbk.dg.tpwd.report.get"),
            TaobaoMethod::GetActivityInfo => String::from("taobao.tbk.activity.info.get"),
        }
    }

    fn get_response_key(&self) -> String {
        match *self {
            TaobaoMethod::MaterialSelect => String::from("tbk_dg_optimus_material_response"),
            TaobaoMethod::MaterialSearch => String::from("tbk_dg_material_optional_response"),
            TaobaoMethod::JhsSearch => String::from("ju_items_search_response"),
            TaobaoMethod::ItemDetail => String::from("tbk_item_info_get_response"),
            TaobaoMethod::CouponDetail => String::from("tbk_coupon_get_response"),
            TaobaoMethod::CreateTPwd => String::from("tbk_tpwd_create_response"),
            TaobaoMethod::SpreadGet => String::from("tbk_spread_get_response"),
            TaobaoMethod::GetTPwdReport => String::from("tbk_dg_tpwd_report_get_response"),
            TaobaoMethod::GetActivityInfo => String::from("tbk_activity_info_get_response"),
        }
    }
}
