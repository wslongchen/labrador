use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json::{Value as JsonValue};

use crate::{errors::LabraError, LabradorResult, request::Response, RequestMethod};
use crate::jd::constants::ERROR_RESPONSE_KEY;


// 京东 ↓


#[derive(Debug, Serialize, Deserialize)]
pub struct JDResponse {
    pub code: Option<String>,
    pub message: Option<String>,
    pub body: Option<String>,
}


impl JDResponse {
    pub fn new() -> Self {
        Self {
            code: Some("0".to_string()),
            message: None,
            body: None,
        }
    }

    pub fn parse(str: &str,method: impl RequestMethod) -> LabradorResult<Self> {
        let v= serde_json::from_str(str).unwrap_or(JsonValue::Null);
        // 判断是否异常
        let err= &v[ERROR_RESPONSE_KEY];
        if !err.is_null() {
            let resp = serde_json::from_str::<Self>(&err.to_string()).unwrap_or(JDResponse::new());
            Err(LabraError::ClientError {errcode: resp.code.to_owned().unwrap_or_default().to_string(), errmsg: resp.message.to_owned().unwrap_or_default()})
        } else {
            let response = &v[&method.get_response_key()];
            if !response.is_null() {
                let mut resp = serde_json::from_str::<Self>(&response.to_string()).unwrap_or(JDResponse::new());
                if resp.code.is_none() {
                    resp.code = "0".to_string().into();
                }
                resp.body = response.to_string().into();
                Ok(resp)
            } else {
                Err(LabraError::MissingField(format!("无法获取解析返回结果：【{}】", str)))
            }
        }

    }

    pub fn is_success(&self) -> bool {
        self.code.to_owned().unwrap_or_default().eq("0")
    }

    pub fn get_biz_model<T: DeserializeOwned>(&self, key: Option<&str>) -> LabradorResult<T> {
        if self.is_success() {
            if let Some(key) = key {
                let v = serde_json::from_str::<JsonValue>(&self.body.to_owned().unwrap_or_default())?;
                let result = &v[key];
                if result.is_string() {
                    serde_json::from_str::<T>(result.as_str().unwrap_or_default()).map_err(LabraError::from)
                } else {
                    serde_json::from_value::<T>(v[key].to_owned()).map_err(LabraError::from)
                }
            } else {
                serde_json::from_str::<T>(&self.body.to_owned().unwrap_or_default()).map_err(LabraError::from)
            }
        } else {
            Err(LabraError::ClientError { errcode: self.code.to_owned().unwrap_or_default().to_string(), errmsg: self.message.to_owned().unwrap_or_default() })
        }
    }

}

//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JdCommonResponse <T> {
    /// 返回信息
    pub message: Option<String>,
    /// 请求编号
    pub request_id: Option<String>,
    /// 是否还有更多,true：还有数据；false:已查询完毕，没有数据
    pub has_more: Option<bool>,
    /// 返回编码
    /// 200:success; 500:服务端异常; 400:参数错误; 401:验证失败; 403:无访问权限; 404数据不存在; 409:数据已存在;
    /// 410:联盟用户不存在，请检查unionId是否正确; 411:unionId不正确，请检查unionId是否正确; 2003101:参数异常，skuIds为空;
    /// 2003102:参数异常，sku个数为1-100个; 2003103:接口异常，没有相关权限; 2003104:请求商品信息{X}条，成功返回{Y}条, 失败{Z}条;
    pub code: Option<i32>,
    /// 总数
    pub total_count: Option<u64>,
    pub data: Option<T>,
}

// 京粉精选商品查询接口 ↓

#[derive(Debug, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JdJFGoodsSelect {
    /// 类目信息
    pub category_info: Option<JdCategoryInfo>,
    /// 佣金信息
    pub commission_info: Option<JdCommissionInfo>,
    /// 优惠券信息，返回内容为空说明该SKU无可用优惠券
    pub coupon_info: Option<JdCouponInfo>,
    /// 商品好评率
    pub good_comments_share: Option<f64>,
    /// 评论数
    pub comments: Option<u64>,
    pub image_info: Option<ImageInfo>,
    /// 30天引单数量
    pub in_order_count30_days: Option<u64>,
    /// 商品落地页
    pub material_url: Option<String>,
    /// 价格信息
    pub price_info: Option<PriceInfo>,
    /// 店铺信息
    pub shop_info: Option<ShopInfo>,
    /// 商品ID
    pub sku_id: Option<u64>,
    /// 商品名称
    pub sku_name: Option<String>,
    /// spuid，其值为同款商品的主skuid
    pub spuid: Option<u64>,
    /// 品牌code
    pub brand_code: Option<String>,
    /// 品牌名
    pub brand_name: Option<String>,
    /// g=自营，p=pop
    pub owner: Option<String>,
    /// 拼购信息
    pub pin_gou_info: Option<PinGouInfo>,
    /// 资源信息
    pub resource_info: Option<ResourceInfo>,
    /// 30天引单数量(sku维度)
    pub in_order_count30_days_sku: Option<u64>,
    /// 秒杀信息
    pub seckill_info: Option<SeckillInfo>,
    /// 京喜商品类型，1京喜、2京喜工厂直供、3京喜优选（包含3时可在京东APP购买）
    pub jx_flags: Option<Vec<u8>>,
    /// 段子信息
    pub document_info: Option<DocumentInfo>,
    /// 图书信息
    pub book_info: Option<DocumentInfo>,
    /// 0普通商品，10微信京东购物小程序禁售，11微信京喜小程序禁售
    pub forbid_types: Option<Vec<u32>>,
    /// 京东配送 1：是，0：不是
    pub delivery_type: Option<u8>,
    /// 是否热点数据
    pub is_hot: Option<u8>,
}

#[derive(Debug, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JdCategoryInfo {
    /// 一级类目ID
    pub cid1: Option<u64>,
    /// 二级类目ID
    pub cid2: Option<u64>,
    /// 三级类目ID
    pub cid3: Option<u64>,
    /// 一级类目名称
    pub cid1_name: Option<String>,
    /// 二级类目名称
    pub cid2_name: Option<String>,
    /// 三级类目名称
    pub cid3_name: Option<String>,
}

#[derive(Debug, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentInfo {
    /// 描述文案
    pub document: Option<String>,
    /// 优惠力度文案
    pub discount: Option<String>,
}

#[derive(Debug, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookInfo {
    /// 图书编号
    pub isbn: Option<String>,
}

#[derive(Debug, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SeckillInfo {
    /// 秒杀价原价
    pub seckill_ori_price: Option<String>,
    /// 秒杀价
    pub seckill_price: Option<String>,
    /// 秒杀开始时间(时间戳，毫秒)
    pub seckill_start_time: Option<String>,
    /// 秒杀结束时间(时间戳，毫秒)
    pub seckill_end_time: Option<String>,
}

#[derive(Debug, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceInfo {
    /// 频道id
    pub elite_id: Option<u64>,
    /// 频道名称
    pub elite_name: Option<String>,
}

#[derive(Debug, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PinGouInfo {
    /// 拼购价格
    pub pingou_price: Option<f64>,
    /// 拼购成团所需人数
    pub pingou_tm_count: Option<u64>,
    /// 拼购落地页url
    pub pingou_url: Option<String>,
    /// 拼购开始时间(时间戳，毫秒)
    pub pingou_start_time: Option<u64>,
    /// 拼购结束时间(时间戳，毫秒)
    pub pingou_end_time: Option<u64>,
}

#[derive(Debug, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShopInfo {
    /// 店铺名称（或供应商名称）
    pub shop_name: Option<String>,
    /// 店铺Id
    pub shop_id: Option<u64>,
    pub afs_factor_score_rank_grade: Option<String>,
    pub after_service_score: Option<String>,
    pub comment_factor_score_rank_grade: Option<String>,
    pub logistics_factor_score_rank_grade: Option<String>,
    pub logistics_lvyue_score: Option<String>,
    pub user_evaluate_score: Option<String>,
    pub score_rank_rate: Option<String>,
    pub shop_label: Option<String>,
    /// 店铺评分
    pub shop_level: Option<f64>,
}

#[derive(Debug, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceInfo {
    /// 商品价格
    pub price: Option<f64>,
    /// 促销价
    pub lowest_price: Option<f64>,
    /// 促销价类型，1：无线价格；2：拼购价格； 3：秒杀价格
    pub lowest_price_type: Option<u8>,
    /// 券后价（有无券都返回此字段）
    pub lowest_coupon_price: Option<f64>,
    /// 历史最低价天数（例：当前券后价最近180天最低）
    pub history_price_day: Option<f64>,
}

#[derive(Debug, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JdCouponInfo {
    /// 优惠券合集
    pub coupon_list: Option<Vec<JdCouponDetail>>,
}

#[derive(Debug, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageInfo {
    /// 图片合集
    pub image_list: Option<Vec<Url>>,
    /// 白底图
    pub white_image: Option<String>,
}

#[derive(Debug, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Url {
    /// 图片链接地址，第一个图片链接为主图链接,修改图片尺寸拼接方法：/s***x***_jfs/
    /// 例如：[示例链接](http://img14.360buyimg.com/ads/s300x300_jfs/t22495/56/628456568/380476/9befc935/5b39fb01N7d1af390.jpg)
    pub url: Option<String>,
    
}

#[derive(Debug, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JdCouponDetail {
    /// 券领取结束时间(时间戳，毫秒)
    pub get_end_time: Option<u64>,
    /// 领取开始时间(时间戳，毫秒)
    pub get_start_time: Option<u64>,
    /// 券消费限额
    pub quota: Option<f64>,
    /// 券使用平台 (平台类型：0 - 全平台券，1 - 限平台券)
    pub platform_type: Option<u8>,
    /// 券有效使用结束时间(时间戳，毫秒)
    pub use_end_time: Option<u64>,
    /// 券有效使用开始时间(时间戳，毫秒)
    pub use_start_time: Option<u64>,
    /// 券种类 (优惠券种类：0 - 全品类，1 - 限品类（自营商品），2 - 限店铺，3 - 店铺限商品券)
    pub bind_type: Option<u8>,
    /// 最优优惠券，1：是；0：否，购买一件商品可使用的面额最大优惠券
    pub is_best: Option<u8>,
    /// 券链接
    pub link: Option<String>,
    /// 券热度，值越大热度越高，区间:0,10
    pub hot_value: Option<u8>,
    /// 券面额
    pub discount: Option<f64>,
}

#[derive(Debug, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JdCommissionInfo {
    /// 佣金
    pub commission: Option<f64>,
    /// 佣金比例
    pub commission_share: Option<f64>,
    /// 券后佣金，（促销价-优惠券面额）*佣金比例
    pub coupon_commission: Option<f64>,
    /// plus佣金比例，plus用户购买推广者能获取到的佣金比例
    pub plus_commission_share: Option<f64>,
    /// 是否锁定佣金比例：1是，0否
    pub is_lock: Option<u8>,
    /// 计划开始时间（时间戳，毫秒）
    pub start_time: Option<u64>,
    /// 计划结束时间（时间戳，毫秒）
    pub end_time: Option<u64>,
}


impl Response <JdCommonResponse<Vec<JdJFGoodsSelect>>>  for JsonValue {
    fn parse_result(&self) -> LabradorResult<JdCommonResponse<Vec<JdJFGoodsSelect>>> {
        serde_json::from_value::<JdCommonResponse<Vec<JdJFGoodsSelect>>>(self.to_owned()).map_err(LabraError::from)
    }
}


#[derive(Debug, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JdGoodsInfoQuery {
    /// 商品ID
    pub sku_id: Option<u64>,
    /// 商品单价即京东价
    pub unit_price: Option<f64>,
    /// 无线佣金比例
    pub commision_ratio_wl: Option<f64>,
    /// PC佣金比例
    pub commision_ratio_pc: Option<f64>,
    /// 商品无线京东价（单价为-1表示未查询到该商品单价）
    pub wl_unit_price: Option<f64>,
    /// 商品落地页
    pub material_url: Option<String>,
    /// 二级类目名称
    pub cid2_name: Option<String>,
    /// 一级类目名称
    pub cid_name: Option<String>,
    /// 三级类目名称
    pub cid3_name: Option<String>,
    /// 图片地址
    pub img_url: Option<String>,
    /// 商品名称
    pub goods_name: Option<String>,
    /// plus佣金比例，plus用户购买推广者能获取到的佣金比例
    pub plus_commission_share: Option<f64>,
    /// 是否支持运费险(1:是,0:否)
    pub is_free_freight_risk: Option<u8>,
    /// 是否包邮(1:是,0:否,2:自营商品遵从主站包邮规则)
    pub is_free_shipping: Option<u8>,
    /// 是否秒杀(1:是,0:否)
    pub is_seckill: Option<u8>,
    /// 是否自营(1:是,0:否)
    pub is_jd_sale: Option<u8>,
    /// 推广结束日期(时间戳，毫秒)
    pub end_date: Option<u64>,
    /// 推广开始日期（时间戳，毫秒）
    pub start_date: Option<u64>,
    /// 30天引单数量
    pub in_order_count: Option<u64>,
    /// 商家ID
    pub vid: Option<u64>,
    /// 二级类目ID
    pub cid2: Option<u64>,
    /// 三级类目ID
    pub cid3: Option<u64>,
    /// 一级类目ID
    pub cid: Option<u64>,
    /// 店铺ID
    pub shop_id: Option<u64>,
}


impl Response <JdCommonResponse<Vec<JdGoodsInfoQuery>>>  for JsonValue {
    fn parse_result(&self) -> LabradorResult<JdCommonResponse<Vec<JdGoodsInfoQuery>>> {
        serde_json::from_value::<JdCommonResponse<Vec<JdGoodsInfoQuery>>>(self.to_owned()).map_err(LabraError::from)
    }
}


#[derive(Debug, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JdPromotionUrlGenerateResponse{
    /// 生成的目标推广链接，长期有效
    pub click_u_r_l: Option<String>,
}


impl Response <JdCommonResponse<JdPromotionUrlGenerateResponse>>  for JsonValue {
    fn parse_result(&self) -> LabradorResult<JdCommonResponse<JdPromotionUrlGenerateResponse>> {
        serde_json::from_value::<JdCommonResponse<JdPromotionUrlGenerateResponse>>(self.to_owned()).map_err(LabraError::from)
    }
}

//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JdOrderQueryResponse {
    /// 订单完成时间(（购买用户确认收货时间）时间戳，毫秒)
    pub order_row_resp: Option<Vec<JdOrderQueryResp>>,
}

#[derive(Debug, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JdOrderQueryResp{
    /// 订单完成时间(（购买用户确认收货时间）时间戳，毫秒)
    pub finish_time: Option<u64>,
    /// 下单设备(1:PC,2:无线)
    pub order_emt: Option<u8>,
    /// 下单用户是否为PLUS会员 0：否，1：是
    pub plus: Option<u8>,
    /// 订单ID
    pub order_id: Option<u64>,
    /// 下单时间(时间戳，毫秒)
    pub order_time: Option<u64>,
    /// 父单的订单号：如某个订单因为仓储物流或其它原因拆成多笔订单时，拆分前的原订单号会作为父单号存储在该字段中
    /// 拆分出的新订单号作为子单号存储在orderid中，若未发生拆单，该字段为0
    pub parent_id: Option<u64>,
    /// 推客的联盟ID
    pub union_id: Option<u64>,
    /// 订单维度商家ID，不建议使用，可以用订单行sku维度popId参考
    pub pop_id: Option<u64>,
    /// 订单维度预估结算时间,不建议使用，可以用订单行sku维度paymonth字段参考
    /// （格式：yyyyMMdd），0：未结算，订单'预估结算时间'仅供参考。账号未通过资质审核或订单发生售后，会影响订单实际结算时间。
    pub pay_month: Option<String>,
    /// 订单维度的推客生成推广链接时传入的扩展字段，不建议使用
    /// 可以用订单行sku维度ext1参考,（需要联系运营开放白名单才能拿到数据）
    pub ext1: Option<String>,
    /// sku维度的有效码
    /// （-1：未知,2.无效-拆单,3.无效-取消,4.无效-京东帮帮主订单,5.无效-账号异常,6.无效-赠品类目不返佣,7.无效-校园订单
    /// 8.无效-企业订单,9.无效-团购订单,11.无效-乡村推广员下单,13.无效-违规订单
    /// 14.无效-来源与备案网址不符,15.待付款,16.已付款,17.已完成（购买用户确认收货）,20.无效-此复购订单对应的首购订单无效,21.无效-云店订单
    pub valid_code: Option<u8>,
    /// 订单包含的商品信息列表
    pub sku_list: Option<SkuInfo>,
}

#[derive(Debug, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SkuInfo{
    /// 实际计算佣金的金额。订单完成后，会将误扣除的运费券金额更正。
    /// 如订单完成后发生退款，此金额会更新。
    pub actual_cos_price: Option<f64>,
    /// 推客获得的实际佣金（实际计佣金额*佣金比例*最终比例）。
    /// 如订单完成后发生退款，此金额会更新。
    pub actual_fee: Option<f64>,
    /// 佣金比例
    pub commission_rate: Option<f64>,
    /// 预估计佣金额：由订单的实付金额拆分至每个商品的预估计佣金额，不包括运费，以及京券、东券、E卡、余额等虚拟资产支付的金额。该字段仅为预估值，实际佣金以actualCosPrice为准进行计算
    pub estimate_cos_price: Option<f64>,
    /// 推客的预估佣金（预估计佣金额*佣金比例*最终比例），如订单完成前发生退款，此金额也会更新。
    pub estimate_fee: Option<f64>,
    /// 最终比例（分成比例+补贴比例）
    pub final_rate: Option<f64>,
    /// 商品单价
    pub price: Option<f64>,
    /// 一级类目ID
    pub cid1: Option<u64>,
    /// 商品售后中数量
    pub frozen_sku_num: Option<u64>,
    /// 推广位ID,0代表无推广位
    pub position_id: Option<u64>,
    /// 二级类目ID
    pub cid2: Option<u64>,
    /// 网站ID，0：无网站
    pub site_id: Option<u64>,
    /// 商品ID
    pub sku_id: Option<u64>,
    /// 商品数量
    pub sku_num: Option<u64>,
    /// 商品已退货数量
    pub sku_return_num: Option<u64>,
    /// 分成比例
    pub sub_side_rate: Option<u64>,
    /// 补贴比例
    pub subsidy_rate: Option<u64>,
    /// 三级类目ID
    pub cid3: Option<u64>,
    /// 联盟子站长身份标识，格式：子站长ID_子站长网站ID_子站长推广位ID
    pub pid: Option<String>,
    /// 商品名称
    pub sku_name: Option<String>,
    /// PID所属母账号平台名称（原第三方服务商来源）
    pub union_alias: Option<String>,
    /// 联盟标签数据（32位整型二进制字符串：00000000000000000000000000000001。
    /// 数据从右向左进行，每一位为1表示符合特征，第1位：红包，第2位：组合推广，第3位：拼购
    /// 第5位：有效首次购（0000000000011XXX表示有效首购，最终奖励活动结算金额会结合订单状态判断，以联盟后台对应活动效果[数据报表](https://union.jd.com/active)为准）
    /// 第8位：复购订单，第9位：礼金，第10位：联盟礼金，第11位：推客礼金，第12位：京喜APP首购，第13位：京喜首购，第14位：京喜复购，第15位：京喜订单
    /// 第16位：京东极速版APP首购，第17位白条首购，第18位校园订单，第19位是0或1时，均代表普通订单
    /// 例如：00000000000000000000000000000001:红包订单，00000000000000000000000000000010:组合推广订单，00000000000000000000000000000100:拼购订单，00000000000000000000000000011000:有效首购，00000000000000000000000000000111：红包+组合推广+拼购等） 
    /// 注：一个订单同时使用礼金和红包，仅礼金位数为1，红包位数为0
    pub union_tag: Option<String>,
    /// 渠道组 1：1号店，其他：京东
    pub union_traffic_group: Option<u8>,
    /// sku维度的有效码
    /// （-1：未知,2.无效-拆单,3.无效-取消,4.无效-京东帮帮主订单,5.无效-账号异常,6.无效-赠品类目不返佣,7.无效-校园订单,8.无效-企业订单,9.无效-团购订单,10.无效-开增值税专用发票订单
    /// 11.无效-乡村推广员下单,13.无效-违规订单,14.无效-来源与备案网址不符,15.待付款,16.已付款,17.已完成,18.已结算（5.9号不再支持结算状态回写展示））
    pub valid_code: Option<u64>,
    /// 子渠道标识，在转链时可自定义传入，格式要求：字母、数字或下划线，最多支持80个字符(需要联系运营开放白名单才能拿到数据)
    pub sub_union_id: Option<String>,
    /// 2：同店；3：跨店
    pub trace_type: Option<u8>,
    /// 订单行维度预估结算时间（格式：yyyyMMdd） ，0：未结算。订单'预估结算时间'仅供参考。
    /// 账号未通过资质审核或订单发生售后，会影响订单实际结算时间。
    pub pay_month: Option<u64>,
    /// 商家ID。'订单行维度'
    pub pop_id: Option<u64>,
    /// 招商团活动id：当商品参加了招商团会有该值，为0时表示无活动
    pub cp_act_id: Option<u64>,
    /// 1	站长角色：1 推客 2 团长 3内容服务商
    pub union_role: Option<u8>,
    /// 礼金分摊金额：使用礼金的订单会有该值
    pub gift_coupon_ocs_amount: Option<u64>,
    /// 价保赔付金额：订单申请价保或赔付的金额，实际计佣金额已经减去此金额，您无需处理
    pub pro_price_amount: Option<f64>,
    /// 推客生成推广链接时传入的扩展字段（需要联系运营开放白名单才能拿到数据）。'订单行维度'
    pub ext1: Option<String>,
    /// 礼金批次ID：使用礼金的订单会有该值
    pub gift_coupon_key: Option<String>,
}

//----------------------------------------------------------------------------------------------------------------------------

/// 转链获取
#[derive(Debug, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JdPromotionCodeGetResponse{
    /// 生成的推广目标链接，以短链接形式，有效期60天
    pub short_u_r_l: Option<String>,
    /// 生成推广目标的长链，长期有效
    pub click_u_r_l: Option<String>,
    /// 需要权限申请，京口令（匹配到红包活动有效配置才会返回京口令）
    pub j_command: Option<String>,
    /// 需要权限申请，短口令
    pub j_short_command: Option<String>,
}

//----------------------------------------------------------------------------------------------------------------------------

/// 转链获取
#[derive(Debug, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JdPromotionBySubUnionIdGetResponse {
    /// 生成的推广目标链接，以短链接形式，有效期60天
    pub short_u_r_l: Option<String>,
    /// 生成推广目标的长链，长期有效
    pub click_u_r_l: Option<String>,
}

//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PromotionCode {
    /// 优惠码名称
    pub promo_name: Option<String>,
    /// 优惠码code
    pub promo_code: Option<String>,
    /// 开始时间
    pub begin_time: Option<u64>,
    /// 结束时间
    pub end_time: Option<u64>,
    /// 可用总次数
    pub total_cnt: Option<u64>,
    /// 促销类型，1直降，2满减，3满折
    pub promo_type: Option<u8>,
    /// 优惠码状态,1有效，0失效
    pub state: Option<u8>,
    /// 优惠方式，直降金额
    pub direct_reduce_amount: Option<u8>,
}

#[derive(Debug, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SellingGoodsQueryShopInfo {
    /// 店铺名称
    pub shop_name: Option<String>,
    /// 店铺类型：0国内，1海外
    pub shop_type: Option<u8>,
}

/// 商羚商品查询
#[derive(Debug, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SellingGoodsQueryResponse {
    /// 商品ID
    pub sku_id: Option<u64>,
    /// 商品名称
    pub sku_name: Option<String>,
    /// 商品落地页
    pub material_url: Option<String>,
    /// g=自营，p=pop
    pub owner: Option<String>,
    /// 主图链接
    pub image_url: Option<String>,
    /// 图片链接集合
    pub img_list: Option<Vec<String>>,
    /// 一级类目ID
    pub cid1: Option<u64>,
    /// 二级类目ID
    pub cid2: Option<u64>,
    /// 三级类目ID
    pub cid3: Option<u64>,
    /// 一级类目名称
    pub cid1_name: Option<String>,
    /// 二级类目名称
    pub cid2_name: Option<String>,
    /// 三级类目名称
    pub cid3_name: Option<String>,
    /// 30天引单数量
    pub in_order_count30_days: Option<u64>,
    /// 好评数
    pub goods_comments: Option<u64>,
    /// 无线价格
    pub wl_price: Option<f64>,
    /// 促销价格
    pub lowest_price: Option<f64>,
    /// 无线佣金比例
    pub wl_commission_share: Option<f64>,
    /// 无线佣金
    pub wl_commission: Option<f64>,
    /// 优惠券合集
    pub coupon_list: Option<Vec<JdCouponDetail>>,
    /// 店铺信息
    pub shop_info: Option<ShopInfo>,
    /// 优惠码列表，目前仅商羚海外站有该字段
    pub promotion_code_list: Option<Vec<PromotionCode>>,
    /// 币种：USD代表美元，CNY代表人民币
    pub currency: Option<String>,
}

//----------------------------------------------------------------------------------------------------------------------------
