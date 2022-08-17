use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json::{Value as JsonValue, Value};

use crate::{errors::LabraError, LabradorResult, RequestMethod};
use crate::taobao::constants::ERROR_RESPONSE_KEY;

// 淘宝 ↓
#[derive(Debug, Serialize, Deserialize)]
pub struct TaobaoResponse {
    pub code: Option<i32>,
    pub msg: Option<String>,
    pub request_id: Option<String>,
    pub sub_code: Option<String>,
    pub sub_msg: Option<String>,
    pub body: Option<String>,
}


impl TaobaoResponse {
    pub fn new() -> Self {
        Self {
            code: None,
            msg: None,
            request_id: None,
            sub_code: None,
            sub_msg: None,
            body: None,
        }
    }

    pub fn parse(str: &str,method: impl RequestMethod) -> LabradorResult<Self> {
        let v= serde_json::from_str(str).unwrap_or(JsonValue::Null);
        // 判断是否异常
        let err= &v[ERROR_RESPONSE_KEY];
        if !err.is_null() {
            let resp = serde_json::from_str::<Self>(&err.to_string()).unwrap_or(TaobaoResponse::new());
            Err(LabraError::ClientError {errcode: resp.code.to_owned().unwrap_or_default().to_string(), errmsg: resp.msg.to_owned().unwrap_or_default()})
        } else {
            let response = &v[&method.get_response_key()];
            if !response.is_null() {
                let mut resp = serde_json::from_str::<Self>(&response.to_string()).unwrap_or(TaobaoResponse::new());
                if resp.code.is_none() {
                    resp.code = 0.into();
                }
                resp.body = response.to_string().into();
                Ok(resp)
            } else {
                Err(LabraError::MissingField(format!("无法获取解析返回结果：【{}】", str)))
            }
        }

    }

    pub fn is_success(&self) -> bool {
        self.code.to_owned().unwrap_or_default() == 0
    }

    pub fn get_biz_model<T: DeserializeOwned>(&self) -> LabradorResult<T> {
        if self.is_success() {
            serde_json::from_str::<T>(&self.body.to_owned().unwrap_or_default()).map_err(LabraError::from)
        } else {
            Err(LabraError::ClientError { errcode: self.code.to_owned().unwrap_or_default().to_string(), errmsg: self.sub_msg.to_owned().unwrap_or_default() })
        }
    }

}


//----------------------------------------------------------------------------------------------------------------------------
#[derive(Debug, Deserialize,Serialize)]
pub struct DescList {
    string: Option<Vec<String>>
}


// 物料精选 ↓

#[derive(Debug, Deserialize,Serialize)]
pub struct ResultlList<T> {
    /// 商品数据
    pub map_data: Option<Vec<T>>,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct TbMaterialSelectResponse {
    
    /// 商品总数-目前只有全品库商品查询有该字段
    pub total_count: Option<u64>,
    /// 推荐信息-是否抄底
    pub is_default: Option<String>,
    /// 商品数据
    pub result_list: Option<ResultlList<MaterialSelectItem>>,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct MaterialSelectItem {
    /// 优惠券（元） 若属于预售商品，该优惠券付尾款可用，付定金不可用
    pub coupon_amount: Option<u64>,
    /// 商品信息-商品小图列表
    pub small_images: Option<DescList>,
    /// 店铺信息-店铺名称
    pub shop_title: Option<String>,
    /// 商品信息-宝贝描述（推荐理由,不一定有）
    pub item_description: Option<String>,
    /// 商品信息-商品标题
    pub title: Option<String>,
    /// 优惠券信息-优惠券起用门槛，满X元可用。如：满299元减20元
    pub coupon_start_fee: Option<String>,
    /// 折扣价（元） 若属于预售商品，付定金时间内，折扣价=预售价
    pub zk_final_price: Option<String>,
    /// 商品信息-佣金比率(%)
    pub commission_rate: Option<String>,
    /// 优惠券信息-优惠券开始时间
    pub coupon_start_time: Option<String>,
    /// 优惠券信息-优惠券结束时间
    pub coupon_end_time: Option<String>,
    /// 链接-宝贝+券二合一页面链接(该字段废弃，请勿再用)
    pub coupon_click_url: Option<String>,
    /// 商品信息-商品主图
    pub pict_url: Option<String>,
    /// 链接-宝贝推广链接
    pub click_url: Option<String>,
    /// 商品信息-叶子类目id
    pub category_id: Option<u64>,
    /// 商品信息-宝贝id
    pub item_id: Option<u64>,
    /// 商品信息-30天销量
    pub volume: Option<u64>,
    /// 店铺信息-卖家id
    pub seller_id: Option<u64>,
    /// 优惠券信息-优惠券总量
    pub coupon_total_count: Option<u64>,
    /// 优惠券信息-优惠券剩余量
    pub coupon_remain_count: Option<u64>,
    /// 店铺信息-卖家类型，0表示集市，1表示商城
    pub user_type: Option<u8>,
    /// 拼团专用-拼团剩余库存
    pub stock: Option<u64>,
    /// 拼团专用-拼团已售数量
    pub sell_num: Option<u64>,
    /// 拼团专用-拼团库存数量
    pub total_stock: Option<u64>,
    /// 拼团专用-拼团结束时间
    pub oetime: Option<String>,
    /// 拼团专用-拼团开始时间
    pub ostime: Option<String>,
    /// 拼团专用-拼团几人团
    pub jdd_num: Option<u64>,
    /// 拼团专用-拼团拼成价，单位元
    pub jdd_price: Option<String>,
    /// 拼团专用-拼团一人价（原价)，单位元
    pub orig_price: Option<String>,
    /// 商品信息-一级类目名称
    pub level_one_category_name: Option<String>,
    /// 商品信息-商品白底图
    pub white_image: Option<String>,
    /// 商品信息-商品短标题
    pub short_title: Option<String>,
    /// 营销-天猫营销玩法
    pub tmall_play_activity_info: Option<String>,
    /// 商品信息-商品关联词
    pub word_list: Option<WordData>,
    /// 商品信息-叶子类目名称
    pub category_name: Option<String>,
    /// 商品信息-新人价
    pub new_user_price: Option<String>,
    /// 优惠券信息-优惠券满减信息
    pub coupon_info: Option<String>,
    /// 链接-宝贝+券二合一页面链接
    pub coupon_share_url: Option<String>,
    /// 店铺信息-卖家昵称
    pub nick: Option<String>,
    /// 商品信息-一口价
    pub reserve_price: Option<String>,
    /// 聚划算信息-聚淘结束时间
    pub ju_online_end_time: Option<String>,
    /// 聚划算信息-聚淘开始时间
    pub ju_online_start_time: Option<String>,
    /// 猫超玩法信息-活动结束时间，精确到毫秒
    pub maochao_play_end_time: Option<String>,
    /// 猫超玩法信息-活动开始时间，精确到毫秒
    pub maochao_play_start_time: Option<String>,
    /// 猫超玩法信息-折扣条件，价格百分数存储，件数按数量存储。可以有多个折扣条件，与折扣字段对应，','分割
    pub maochao_play_conditions: Option<String>,
    /// 猫超玩法信息-折扣，折扣按照百分数存储，其余按照单位分存储。可以有多个折扣，','分割
    pub maochao_play_discounts: Option<String>,
    /// 猫超玩法信息-玩法类型，2:折扣(满n件折扣),5:减钱(满n元减m元)
    pub maochao_play_discount_type: Option<String>,
    /// 猫超玩法信息-当前是否包邮，1:是，0:否
    pub maochao_play_free_post_fee: Option<String>,
    /// 多件券优惠比例
    pub multi_coupon_zk_rate: Option<String>,
    /// 多件券件单价
    pub price_after_multi_coupon: Option<String>,
    /// 多件券单品件数
    pub multi_coupon_item_count: Option<String>,
    /// 锁住的佣金率
    pub lock_rate: Option<String>,
    /// 锁佣结束时间
    pub lock_rate_end_time: Option<u64>,
    /// 锁佣开始时间
    pub lock_rate_start_time: Option<u64>,
    /// 满减满折的类型（1. 满减 2. 满折）
    pub promotion_type: Option<String>,
    /// 满减满折信息
    pub promotion_info: Option<String>,
    /// 满减满折门槛（满2件打5折中值为2；满300减20中值为300）
    pub promotion_discount: Option<String>,
    /// 满减满折优惠（满2件打5折中值为5；满300减20中值为20）
    pub promotion_condition: Option<String>,
    /// 预售商品-优惠信息
    pub presale_discount_fee_text: Option<String>,
    /// 预售商品-付尾款结束时间（毫秒）
    pub presale_tail_end_time: Option<u64>,
    /// 预售商品-付尾款开始时间（毫秒）
    pub presale_tail_start_time: Option<u64>,
    /// 预售商品-付定金结束时间（毫秒）
    pub presale_end_time: Option<u64>,
    /// 预售商品-付定金开始时间（毫秒）
    pub presale_start_time: Option<u64>,
    /// 预售商品-定金（元）
    pub presale_deposit: Option<String>,
    /// 预售有礼-淘礼金使用开始时间
    pub ysyl_tlj_use_start_time: Option<String>,
    /// 预售有礼-佣金比例（ 预售有礼活动享受的推广佣金比例，注：推广该活动有特殊分成规则，请详见：https://tbk.bbs.taobao.com/detail.html?appId=45301&postId=9334376 ）
    pub ysyl_commission_rate: Option<String>,
    /// 预售有礼-淘礼金发放时间
    pub ysyl_tlj_send_time: Option<String>,
    /// 预售有礼-预估淘礼金（元）
    pub ysyl_tlj_face: Option<String>,
    /// 预售有礼-推广链接
    pub ysyl_click_url: Option<String>,
    /// 预售有礼-淘礼金使用结束时间
    pub ysyl_tlj_use_end_time: Option<String>,
    /// 聚划算信息-商品预热开始时间（毫秒）
    pub ju_pre_show_end_time: Option<String>,
    /// 聚划算信息-商品预热结束时间（毫秒）
    pub ju_pre_show_start_time: Option<String>,
    /// 活动价
    pub sale_price: Option<String>,
    /// 跨店满减信息
    pub kuadian_promotion_info: Option<String>,
    /// 商品子标题
    pub sub_title: Option<String>,
    /// 聚划算商品价格卖点描述
    pub jhs_price_usp_list: Option<String>,
    /// 淘抢购商品专用-结束时间
    pub tqg_online_end_time: Option<String>,
    /// 淘抢购商品专用-开团时间
    pub tqg_online_start_time: Option<String>,
    /// 淘抢购商品专用-已抢购数量
    pub tqg_sold_count: Option<u64>,
    /// 淘抢购商品专用-总库存
    pub tqg_total_count: Option<u64>,
    /// 是否品牌精选，0不是，1是
    pub superior_brand: Option<String>,
    /// 是否品牌快抢，0不是，1是
    pub is_brand_flash_sale: Option<String>,
    /// 聚划算满减 -结束时间（毫秒）
    pub ju_play_end_time: Option<u64>,
    /// 聚划算满减 -开始时间（毫秒）
    pub ju_play_start_time: Option<u64>,
    /// 聚划算满减：满N件减X元，满N件X折，满N件X元） 2天猫限时抢：前N分钟每件X元，前N分钟满N件每件X元，前N件每件X元）
    pub play_info: Option<String>,
    /// 天猫限时抢可售 -结束时间（毫秒）
    pub tmall_play_activity_end_time: Option<u64>,
    /// 天猫限时抢可售 -开始时间（毫秒）
    pub tmall_play_activity_start_time: Option<u64>,
    /// 商品信息-一级类目ID
    pub level_one_category_id: Option<u64>,
    /// 商品信息-预售数量
    pub uv_sum_pre_sale: Option<u64>,
    /// 选品库信息
    pub favorites_info: Option<Vec<Favorite>>,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct WordData{
    /// 链接-商品相关关联词落地页地址
    pub url: Option<String>,
    /// 商品相关的关联词
    pub word: Option<String>,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct Favorite{
    /// 选品库总数量
    pub total_count: Option<u64>,
    /// 选品库详细信息
    pub favorites_list: Option<Vec<FavoritesDetail>>,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct FavoritesDetail{
    /// 选品库总数量
    pub favorites_id: Option<u64>,
    pub favorites_title: Option<String>,
}


//----------------------------------------------------------------------------------------------------------------------------


// 聚划算搜索 ↓

#[derive(Debug, Deserialize,Serialize)]
pub struct TbJhsSearchResponse {
    /// 一页大小
    pub page_size: Option<u64>,
    /// 商品总数
    pub total_item: Option<u64>,
    /// 总页数
    pub total_page: Option<u64>,
    /// 页码
    pub current_page: Option<u64>,
    /// 商品数据
    pub model_list: Option<ModelList>,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct ModelList {
    /// 商品数据
    pub items: Option<Vec<JhsItem>>,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct JhsItem {
    /// 卖点描述
    pub usp_desc_list: Option<DescList>,
    /// 商品卖点
    pub item_usp_list: Option<DescList>,
    /// 价格卖点
    pub price_usp_list: Option<DescList>,
    /// 无线链接
    pub wap_url: Option<String>,
    /// 商品标题
    pub title: Option<String>,
    /// 淘宝类目id
    pub tb_first_cat_id: Option<u64>,
    /// 原价
    pub orig_price: Option<String>,
    /// pc主图
    pub pic_url_for_p_c: Option<String>,
    /// 聚划算价格，单位分
    pub act_price: Option<String>,
    /// 无线主图
    pub pic_url_for_w_l: Option<String>,
    /// 类目名称
    pub category_name: Option<String>,
    /// itemId
    pub item_id: Option<u64>,
    /// 展示结束时间
    pub show_end_time: Option<u64>,
    /// pc链接
    pub pc_url: Option<String>,
    /// 频道id
    pub platform_id: Option<u64>,
    /// 开始展示时间
    pub show_start_time: Option<u64>,
    /// 开团结束时间
    pub online_end_time: Option<u64>,
    /// 聚划算id
    pub ju_id: Option<u64>,
    /// 开团时间
    pub online_start_time: Option<u64>,
    /// 是否包邮
    pub pay_postage: Option<bool>,
}


//----------------------------------------------------------------------------------------------------------------------------


// 淘宝客商品详情查询(简版) ↓
#[derive(Debug, Deserialize,Serialize)]
pub struct TbItemDetailResponse {
    /// 淘宝客商品列表
    pub n_tbk_item: Option<Vec<ItemSimpleDetail>>,
}
#[derive(Debug, Deserialize,Serialize)]
pub struct ItemSimpleDetail {
    /// 一级类目名称
    pub cat_name: Option<String>,
    /// 商品ID
    pub num_iid: Option<u64>,
    /// 商品标题
    pub title: Option<String>,
    /// 商品主图
    pub pict_url: Option<String>,
    /// 商品小图列表
    pub small_images: Option<DescList>,
    /// 商品一口价格
    pub reserve_price: Option<String>,
    /// 折扣价（元） 若属于预售商品，付定金时间内，折扣价=预售价
    pub zk_final_price: Option<String>,
    /// 卖家类型，0表示集市，1表示商城
    pub user_type: Option<u8>,
    /// 商品所在地
    pub provcity: Option<String>,
    /// 商品链接
    pub item_url: Option<String>,
    /// 卖家id
    pub seller_id: Option<u64>,
    /// 30天销量
    pub volume: Option<u64>,
    /// 店铺名称
    pub nick: Option<String>,
    /// 叶子类目名称
    pub cat_leaf_name: Option<String>,
    /// 是否加入消费者保障
    pub is_prepay: Option<bool>,
    /// 店铺dsr 评分
    pub shop_dsr: Option<u64>,
    /// 卖家等级
    pub ratesum: Option<u64>,
    /// 退款率是否低于行业均值
    pub i_rfd_rate: Option<bool>,
    /// 好评率是否高于行业均值
    pub h_good_rate: Option<bool>,
    /// 成交转化是否高于行业均值
    pub h_pay_rate30: Option<bool>,
    /// 是否包邮
    pub free_shipment: Option<bool>,
    /// 商品库类型，支持多库类型输出，以英文逗号分隔“,”分隔，1:营销商品主推库，如果值为空则不属于1这种商品类型
    pub material_lib_type: Option<String>,
    /// 预售商品-商品优惠信息
    pub presale_discount_fee_text: Option<String>,
    /// 预售商品-付定金结束时间（毫秒）
    pub presale_tail_end_time: Option<u64>,
    /// 预售商品-付尾款开始时间（毫秒）
    pub presale_tail_start_time: Option<u64>,
    /// 预售商品-付定金结束时间（毫秒）
    pub presale_end_time: Option<u64>,
    /// 预售商品-付定金开始时间（毫秒）
    pub presale_start_time: Option<u64>,
    /// 预售商品-定金（元）
    pub presale_deposit: Option<String>,
    /// 聚划算满减 -结束时间（毫秒）
    pub ju_play_end_time: Option<u64>,
    /// 聚划算满减 -开始时间（毫秒）
    pub ju_play_start_time: Option<u64>,
    /// 天猫限时抢可售 -结束时间（毫秒）
    pub tmall_play_activity_end_time: Option<u64>,
    /// 天猫限时抢可售 -开始时间（毫秒）
    pub tmall_play_activity_start_time: Option<u64>,
    /// 聚划算信息-聚淘开始时间（毫秒）
    pub ju_online_start_time: Option<String>,
    /// 1聚划算满减：满N件减X元，满N件X折，满N件X元） 2天猫限时抢：前N分钟每件X元，前N分钟满N件每件X元，前N件每件X元）
    pub play_info: Option<String>,
    /// 聚划算信息-聚淘结束时间（毫秒）
    pub ju_online_end_time: Option<String>,
    /// 聚划算信息-商品预热开始时间（毫秒）
    pub ju_pre_show_start_time: Option<String>,
    /// 聚划算信息-商品预热结束时间（毫秒）
    pub ju_pre_show_end_time: Option<String>,
    /// 活动价
    pub sale_price: Option<String>,
    /// 跨店满减信息
    pub kuadian_promotion_info: Option<String>,
    /// 是否品牌精选，0不是，1是
    pub superior_brand: Option<String>,
}

impl TbItemDetailResponse {
    pub fn from_resp(resp: &TaobaoResponse) -> LabradorResult<Self> {
        let body = resp.body.to_owned().unwrap_or_default();
        let v = serde_json::from_str(&body).unwrap_or(Value::Null);
        if let Some(result) = v.get("results") {
            serde_json::from_value::<Self>(result.to_owned()).map_err( LabraError::from)
        } else {
            Err(LabraError::MissingField("返回结果有误，缺少字段。".to_owned()))
        }
        
    }
}


//----------------------------------------------------------------------------------------------------------------------------


// 阿里妈妈推广券详情查询 ↓
#[derive(Debug, Deserialize,Serialize)]
pub struct TbCouponDetailResponse {
    /// 优惠券门槛金额
    pub coupon_start_fee: Option<String>,
    pub cat_name: Option<String>,
    /// 优惠券结束时间
    pub coupon_end_time: Option<String>,
    /// 优惠券开始时间
    pub coupon_start_time: Option<String>,
    /// 优惠券金额
    pub coupon_amount: Option<String>,
    /// 优惠券剩余量
    pub coupon_remain_count: Option<u64>,
    /// 优惠券总量
    pub coupon_total_count: Option<u64>,
    /// 券类型，1 表示全网公开券，4 表示妈妈渠道券
    pub coupon_src_scene: Option<u8>,
    /// 券属性，0表示店铺券，1表示单品券
    pub coupon_type: Option<u8>,
    /// 券ID
    pub coupon_activity_id: Option<String>,
}

impl TbCouponDetailResponse{
     pub fn from_resp(resp: &TaobaoResponse) -> LabradorResult<Self> {
        let body = resp.body.to_owned().unwrap_or_default();
        let v = serde_json::from_str(&body).unwrap_or(Value::Null);
        if let Some(result) = v.get("results") {
            serde_json::from_value::<Self>(result.to_owned()).map_err( LabraError::from)
        } else {
            Err(LabraError::MissingField("返回结果有误，缺少字段。".to_owned()))
        }
    }
}

//----------------------------------------------------------------------------------------------------------------------------


// 淘口令生成 ↓
#[derive(Debug, Deserialize,Serialize)]
pub struct TbCreateTPwdResponse {
    /// 针对苹果ios14及以上版本的苹果设备，手淘将按照示例值信息格式读取淘口令(需包含：数字+羊角符+url，识别规则可能根据ios情况变更)。如需更改淘口令内文案、url等内容，请务必先验证更改后的淘口令在手淘可被识别打开！
    pub model: Option<String>,
    /// 非苹果ios14以上版本的设备（即其他ios版本、Android系统等），可以用此淘口令正常在复制到手淘打开
    pub password_simple: Option<String>,
}

impl TbCreateTPwdResponse{
    pub fn from_resp(resp: &TaobaoResponse) -> LabradorResult<Self> {
        let body = resp.body.to_owned().unwrap_or_default();
        let v = serde_json::from_str(&body).unwrap_or(Value::Null);
        if let Some(result) = v.get("data") {
            serde_json::from_value::<Self>(result.to_owned()).map_err( LabraError::from)
        } else {
            Err(LabraError::MissingField("返回结果有误，缺少字段。".to_owned()))
        }
    }
}


//----------------------------------------------------------------------------------------------------------------------------

// 淘口令生成 ↓
#[derive(Debug, Deserialize,Serialize)]
pub struct TbTPwdReportGetResponse {
    /// 截止查询时刻近1小时回流pv
    pub hour_pv: Option<u64>,
    /// 截止查询时刻近1小时回流uv
    pub hour_uv: Option<u64>,
    /// 今日截止查询时刻累计uv
    pub uv: Option<u64>,
    /// 今日截止查询时刻累计pv
    pub pv: Option<u64>,
}

impl TbTPwdReportGetResponse{
    pub fn from_resp(resp: &TaobaoResponse) -> LabradorResult<Self> {
        let body = resp.body.to_owned().unwrap_or_default();
        let v = serde_json::from_str(&body).unwrap_or(Value::Null);
        if let Some(result) = v.get("data") {
            serde_json::from_value::<Self>(result.to_owned()).map_err( LabraError::from)
        } else {
            Err(LabraError::MissingField("返回结果有误，缺少字段。".to_owned()))
        }
    }
}


//----------------------------------------------------------------------------------------------------------------------------


// 淘宝客-公用-长链转短链 ↓
#[derive(Debug, Deserialize,Serialize)]
pub struct TbSpreadGetResponse {
    /// 传播形式对象列表
    pub results: Option<Vec<TbkSpread>>,
    /// total_results
    pub total_results: Option<u64>,
}

// 淘宝客-公用-长链转短链 ↓
#[derive(Debug, Deserialize,Serialize)]
pub struct TbkSpread {
    /// 传播形式, 目前只支持短链接
    pub content: Option<String>,
    /// 调用错误信息；由于是批量接口，请重点关注每条请求返回的结果，如果非OK，则说明该结果对应的content不正常，请酌情处理;
    pub err_msg: Option<String>,
}

//----------------------------------------------------------------------------------------------------------------------------

// 淘宝客-活动 ↓
#[derive(Debug, Deserialize,Serialize)]
pub struct TbGetActivityInfoResponse {
    /// 【本地化】微信推广二维码地址
    pub wx_qrcode_url: Option<String>,
    /// 淘客推广长链
    pub click_url: Option<String>,
    /// 淘客推广短链
    pub short_click_url: Option<String>,
    /// 投放平台, 1-PC 2-无线
    pub terminal_type: Option<String>,
    /// 物料素材下载地址
    pub material_oss_url: Option<String>,
    /// 会场名称
    pub page_name: Option<String>,
    /// 活动开始时间
    pub page_start_time: Option<String>,
    /// 活动结束时间
    pub page_end_time: Option<String>,
    /// 活动结束时间
    pub wx_miniprogram_path: Option<String>,
}


impl TbGetActivityInfoResponse{
    pub fn from_resp(resp: &TaobaoResponse) -> LabradorResult<Self> {
        let body = resp.body.to_owned().unwrap_or_default();
        let v = serde_json::from_str(&body).unwrap_or(Value::Null);
        if let Some(result) = v.get("data") {
            serde_json::from_value::<Self>(result.to_owned()).map_err( LabraError::from)
        } else {
            Err(LabraError::MissingField("返回结果有误，缺少字段。".to_owned()))
        }
    }
}

//----------------------------------------------------------------------------------------------------------------------------


// 淘宝客物料搜索 ↓
#[derive(Debug, Deserialize,Serialize)]
pub struct TbMaterialSearchResponse {
    /// 商品总数-目前只有全品库商品查询有该字段
    pub total_results: Option<u64>,
    /// 商品数据
    pub result_list: Option<ResultlList<MaterialSearchItem>>,
}


#[derive(Debug, Deserialize,Serialize)]
pub struct MaterialSearchItem {
    /// 折扣价（元） 若属于预售商品，付定金时间内，折扣价=预售价
    pub zk_final_price: Option<String>,
    /// 商品信息-一口价
    pub reserve_price: Option<String>,
    /// 商品信息-商品小图列表
    pub small_images: Option<DescList>,
    /// 商品信息-商品主图
    pub pict_url: Option<String>,
    /// 商品信息-商品标题
    pub title: Option<String>,
    /// 优惠券信息-优惠券开始时间
    pub coupon_start_time: Option<String>,
    /// 优惠券信息-优惠券结束时间
    pub coupon_end_time: Option<String>,
    /// 商品信息-定向计划信息{"19013551":"2850","74510538":"2550"}
    pub info_dxjh: Option<String>,
    /// 商品信息-淘客30天推广量
    pub tk_total_sales: Option<String>,
    /// 商品信息-月支出佣金(该字段废弃，请勿再用)
    pub tk_total_commi: Option<String>,
    /// 优惠券信息-优惠券id
    pub coupon_id: Option<String>,
    /// 商品信息-宝贝id(该字段废弃，请勿再用)
    pub num_iid: Option<u64>,
    /// 店铺信息-卖家类型。0表示集市，1表示天猫
    pub user_type: Option<u8>,
    /// 商品信息-宝贝所在地
    pub provcity: Option<String>,
    /// 链接-宝贝地址
    pub item_url: Option<String>,
    /// 商品信息-是否包含营销计划
    pub include_mkt: Option<String>,
    /// 商品信息-是否包含定向计划
    pub include_dxjh: Option<String>,
    /// 商品信息-佣金比率。1550表示15.5%
    pub commission_rate: Option<String>,
    /// 商品信息-30天销量（饿了么卡券信息-总销量）
    pub volume: Option<u64>,
    /// 店铺信息-卖家id
    pub seller_id: Option<u64>,
    /// 优惠券信息-优惠券总量
    pub coupon_total_count: Option<u64>,
    /// 惠券信息-优惠券剩余量
    pub coupon_remain_count: Option<u64>,
    /// 优惠券信息-优惠券满减信息
    pub coupon_info: Option<String>,
    /// MKT表示营销计划，SP表示定向计划，COMMON表示通用计划	商品信息-佣金类型。MKT表示营销计划，SP表示定向计划，COMMON表示通用计划
    pub commission_type: Option<String>,
    /// 店铺信息-店铺名称
    pub shop_title: Option<String>,
    /// 店铺信息-店铺dsr评分
    pub shop_dsr: Option<u64>,
    /// 商品信息-叶子类目id
    pub category_id: Option<u64>,
    /// 拼团专用-拼团几人团
    pub jdd_num: Option<u64>,
    /// 预售专用-预售数量
    pub uv_sum_pre_sale: Option<u64>,
    /// 链接-宝贝+券二合一页面链接
    pub coupon_share_url: Option<String>,
    /// 链接-宝贝推广链接
    pub url: Option<String>,
    /// 商品信息-一级类目名称
    pub level_one_category_name: Option<String>,
    /// 商品信息-叶子类目名称
    pub category_name: Option<String>,
    /// 商品信息-商品短标题
    pub short_title: Option<String>,
    /// 商品信息-商品白底图
    pub white_image: Option<String>,
    /// 拼团专用-拼团结束时间
    pub oetime: Option<String>,
    /// 拼团专用-拼团开始时间
    pub ostime: Option<String>,
    /// 拼团专用-拼团拼成价，单位元
    pub jdd_price: Option<String>,
    /// 链接-物料块id(测试中请勿使用)
    pub x_id: Option<String>,
    /// 优惠券信息-优惠券起用门槛，满X元可用。如：满299元减20元
    pub coupon_start_fee: Option<String>,
    /// 优惠券（元） 若属于预售商品，该优惠券付尾款可用，付定金不可用
    pub coupon_amount: Option<String>,
    /// 全棉亲肤	商品信息-宝贝描述(推荐理由)
    pub item_description: Option<String>,
    /// 店铺信息-卖家昵称
    pub nick: Option<String>,
    /// 拼团专用-拼团一人价（原价)，单位元
    pub orig_price: Option<String>,
    /// 营销-天猫营销玩法
    pub tmall_play_activity_info: Option<String>,
    /// 拼团专用-拼团库存数量
    pub total_stock: Option<u64>,
    /// 拼团专用-拼团已售数量
    pub sell_num: Option<u64>,
    /// 拼团专用-拼团剩余库存
    pub stock: Option<u64>,
    /// 商品信息-宝贝id
    pub item_id: Option<u64>,
    /// 锁佣结束时间
    pub lock_rate_end_time: Option<u64>,
    /// 锁佣开始时间
    pub lock_rate_start_time: Option<u64>,
    /// 商品邮费
    pub real_post_fee: Option<String>,
    /// 锁住的佣金率
    pub lock_rate: Option<String>,
    /// 预售商品-优惠
    pub presale_discount_fee_text: Option<String>,
    /// 预售商品-付定金结束时间（毫秒）
    pub presale_tail_end_time: Option<u64>,
    /// 预售商品-付尾款开始时间（毫秒）
    pub presale_tail_start_time: Option<u64>,
    /// 预售商品-付定金结束时间（毫秒）
    pub presale_end_time: Option<u64>,
    /// 预售商品-付定金开始时间（毫秒）
    pub presale_start_time: Option<u64>,
    /// 比价场景专用，当系统检测到入参消费者ID购买当前商品会获得《天天开彩蛋》玩法的彩蛋时，该字段显示1，否则为0
    pub reward_info: Option<u64>,
    /// 预售商品-定金（元）
    pub presale_deposit: Option<String>,
    /// 预售有礼-淘礼金使用开始时间
    pub ysyl_tlj_use_start_time: Option<String>,
    /// 预售有礼-佣金比例（ 预售有礼活动享受的推广佣金比例，注：推广该活动有特殊分成规则，请详见：https://tbk.bbs.taobao.com/detail.html?appId=45301&postId=9334376 ）
    pub ysyl_commission_rate: Option<String>,
    /// 预售有礼-淘礼金发放时间
    pub ysyl_tlj_send_time: Option<String>,
    /// 预售有礼-预估淘礼金（元）
    pub ysyl_tlj_face: Option<String>,
    /// 预售有礼-推广链接
    pub ysyl_click_url: Option<String>,
    /// 预售有礼-淘礼金使用结束时间
    pub ysyl_tlj_use_end_time: Option<String>,
    /// 聚划算信息-商品预热开始时间（毫秒）
    pub ju_pre_show_end_time: Option<String>,
    /// 聚划算信息-商品预热结束时间（毫秒）
    pub ju_pre_show_start_time: Option<String>,
    /// 活动价
    pub sale_price: Option<String>,
    /// 跨店满减信息
    pub kuadian_promotion_info: Option<String>,
    /// 本地化-销售开始时间
    pub sale_begin_time: Option<String>,
    /// 本地化-销售结束时间
    pub sale_end_time: Option<String>,
    /// 本地化-到门店距离（米）
    pub distance: Option<String>,
    /// 本地化-可用店铺id
    pub usable_shop_id: Option<String>,
    /// 本地化-可用店铺名称
    pub usable_shop_name: Option<String>,
    /// 是否品牌精选，0不是，1是
    pub superior_brand: Option<String>,
    /// 是否品牌快抢，0不是，1是
    pub is_brand_flash_sale: Option<String>,
}
