use std::collections::BTreeMap;
use serde::Serialize;
use serde_json::Value;

use crate::taobao::method::TaobaoMethod;
use crate::TaobaoRequest;

//----------------------------------------------------------------------------------------------------------------------------

// 淘宝 ↓`

#[derive(Debug, Serialize, Default)]
pub struct TbMaterialSelectRequest {
    /// 一页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u64>,
    /// mm_xxx_xxx_xxx的第三位
    pub adzone_id: u64,
    /// 页码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<u64>,
    /// 智能匹配-设备号加密后的值（MD5加密需32位小写），类型为OAID时传原始OAID值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_value: Option<String>,
    /// 智能匹配-设备号加密类型：MD5，类型为OAID时不传
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_encrypt: Option<String>,
    /// 智能匹配-设备号类型：IMEI，或者IDFA，或者UTDID（UTDID不支持MD5加密），或者OAID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    /// 内容专用-内容详情ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_id: Option<u64>,
    /// 内容专用-内容渠道信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_source: Option<String>,
    /// 选品库投放id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorites_id: Option<String>,
    /// 商品ID，用于相似商品推荐
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<u64>,
    pub material_id: u64,
}

impl TaobaoRequest for TbMaterialSelectRequest {
    fn get_api_method_name(&self) -> TaobaoMethod {
        TaobaoMethod::MaterialSelect
    }

    fn get_text_params(&self) -> BTreeMap<String, String> {
        let mut params = BTreeMap::new();
        let v = serde_json::to_value(self).unwrap_or(Value::Null);
        if let Some(data) = v.as_object() {
            for (k, v) in data.into_iter() {
                if v.is_null() {
                    continue;
                }
                params.insert(k.to_string(), v.to_string());
            }
        }
        params
    }
}

//----------------------------------------------------------------------------------------------------------------------------


#[derive(Debug, Serialize, Default)]
pub struct TbJhsSearchRequest {
    /// 页码
    pub current_page: Option<u64>,
    /// 一页大小
    pub page_size: Option<u64>,
    /// 媒体pid
    pub pid: String,
    /// 是否包邮
    pub postage: Option<bool>,
    /// 状态，预热：1，正在进行中：2
    pub status: Option<u8>,
    /// 淘宝类目id
    pub taobao_category_id: Option<u64>,
    /// 搜索关键词
    pub word: Option<String>,

}

impl TaobaoRequest for TbJhsSearchRequest {
    fn get_api_method_name(&self) -> TaobaoMethod {
        TaobaoMethod::JhsSearch
    }

    fn get_text_params(&self) -> BTreeMap<String, String> {
        BTreeMap::from([("param_top_item_query".to_string(), serde_json::to_string(&self).unwrap_or_default())])
    }
}

//----------------------------------------------------------------------------------------------------------------------------




#[derive(Debug, Serialize, Default)]
pub struct TbItemDetailRequest {
    /// 商品ID串，用,分割，最大40个
    pub num_iids: Option<String>,
    /// 链接形式：1：PC，2：无线，默认：１
    pub platform: Option<u8>,
    /// ip地址，影响邮费获取，如果不传或者传入不准确，邮费无法精准提供
    pub ip: Option<String>,
}

impl TaobaoRequest for TbItemDetailRequest {
    fn get_api_method_name(&self) -> TaobaoMethod {
        TaobaoMethod::ItemDetail
    }

    fn get_text_params(&self) -> BTreeMap<String, String> {
        let mut params = BTreeMap::new();
        params.insert("num_iids".to_string(), self.num_iids.to_owned().unwrap_or_default().to_string());
        if let Some(platform) = &self.platform {
            params.insert("platform".to_string(), platform.to_string());
        }
        if let Some(ip) = &self.ip {
            params.insert("ip".to_string(), ip.to_string());
        }
        params
    }
}

//----------------------------------------------------------------------------------------------------------------------------




#[derive(Debug, Serialize, Default)]
pub struct TbCouponDetailRequest {
    /// 带券ID与商品ID的加密串
    pub me: Option<String>,
    /// 商品ID
    pub item_id: Option<u64>,
    /// 券ID
    pub activity_id: Option<String>,
}

impl TaobaoRequest for TbCouponDetailRequest {
    fn get_api_method_name(&self) -> TaobaoMethod {
        TaobaoMethod::CouponDetail
    }

    fn get_text_params(&self) -> BTreeMap<String, String> {
        let mut params = BTreeMap::new();
        if let Some(item_id) = &self.item_id {
            params.insert("item_id".to_string(), item_id.to_string());
        }
        if let Some(me) = &self.me {
            params.insert("me".to_string(), me.to_string());
        }
        if let Some(activity_id) = &self.activity_id {
            params.insert("activity_id".to_string(), activity_id.to_string());
        }
        params
    }
}

//----------------------------------------------------------------------------------------------------------------------------



#[derive(Debug, Serialize, Default)]
pub struct TbCreateTPwdRequest {
    /// 生成口令的淘宝用户ID
    pub user_id: Option<String>,
    /// 口令弹框内容
    pub text: String,
    /// 口令跳转目标页
    pub url: String,
    /// 口令弹框logoURL
    pub logo: Option<String>,
}

impl TaobaoRequest for TbCreateTPwdRequest {
    fn get_api_method_name(&self) -> TaobaoMethod {
        TaobaoMethod::CreateTPwd
    }

    fn get_text_params(&self) -> BTreeMap<String, String> {
        let mut params = BTreeMap::new();
        if let Some(user_id) = &self.user_id {
            params.insert("user_id".to_string(), user_id.to_string());
        }
        if let Some(logo) = &self.logo {
            params.insert("logo".to_string(), logo.to_string());
        }
        params.insert("url".to_string(), self.url.to_string());
        params.insert("text".to_string(), self.text.to_string());
        params
    }
}

//----------------------------------------------------------------------------------------------------------------------------


#[derive(Debug, Serialize, Default)]
pub struct TbSpreadGetRequest {
    /// 请求列表，内部包含多个url
    pub requests: Vec<SpreadGetUrl>,
}
#[derive(Debug, Serialize, Default)]
pub struct SpreadGetUrl {
    /// 原始url, 只支持uland.taobao.com，s.click.taobao.com， ai.taobao.com，temai.taobao.com的域名转换，否则判错
    pub url: Option<String>,
}



impl TaobaoRequest for TbSpreadGetRequest {
    fn get_api_method_name(&self) -> TaobaoMethod {
        TaobaoMethod::SpreadGet
    }

    fn get_text_params(&self) -> BTreeMap<String, String> {
        let mut params = BTreeMap::new();
        params.insert("requests".to_string(), serde_json::to_string(&self.requests).unwrap_or_default());
        params
    }
}

//----------------------------------------------------------------------------------------------------------------------------




#[derive(Debug, Serialize, Default)]
pub struct TbTPwdReportGetRequest {
    /// 待查询的口令
    pub tao_password: String,
    /// mm_xxx_xxx_xxx的第3段数字
    pub adzone_id: u64,
}

impl TaobaoRequest for TbTPwdReportGetRequest {
    fn get_api_method_name(&self) -> TaobaoMethod {
        TaobaoMethod::GetTPwdReport
    }

    fn get_text_params(&self) -> BTreeMap<String, String> {
        let mut params = BTreeMap::new();
        params.insert("tao_password".to_string(), self.tao_password.to_string());
        params.insert("adzone_id".to_string(), self.adzone_id.to_string());
        params
    }
}

//----------------------------------------------------------------------------------------------------------------------------



#[derive(Debug, Serialize, Default)]
pub struct TbGetActivityInfoRequest {
    /// 官方活动会场ID，从淘宝客后台“我要推广-活动推广”中获取
    pub activity_material_id: String,
    /// mm_xxx_xxx_xxx的第三位
    pub adzone_id: u64,
    /// mm_xxx_xxx_xxx 仅三方分成场景使用
    pub sub_pid: Option<String>,
    /// 渠道关系id
    pub relation_id: Option<String>,
    /// 自定义输入串，英文和数字组成，长度不能大于12个字符，区分不同的推广渠道
    pub union_id: Option<String>,
}

impl TaobaoRequest for TbGetActivityInfoRequest {
    fn get_api_method_name(&self) -> TaobaoMethod {
        TaobaoMethod::GetTPwdReport
    }

    fn get_text_params(&self) -> BTreeMap<String, String> {
        let mut params = BTreeMap::new();
        params.insert("activity_material_id".to_string(), self.activity_material_id.to_string());
        params.insert("adzone_id".to_string(), self.adzone_id.to_string());
        if let Some(sub_pid) = &self.sub_pid {
            params.insert("sub_pid".to_string(), sub_pid.to_string());
        }
        if let Some(relation_id) = &self.relation_id {
            params.insert("relation_id".to_string(), relation_id.to_string());
        }
        if let Some(union_id) = &self.union_id {
            params.insert("union_id".to_string(), union_id.to_string());
        }
        params
    }
}

//----------------------------------------------------------------------------------------------------------------------------




#[derive(Debug, Serialize, Default)]
pub struct TbMaterialSearchRequest {
    /// 一页大小
    pub page_size: Option<u64>,
    /// mm_xxx_xxx_xxx的第三位
    pub adzone_id: u64,
    /// 页码
    pub page_no: Option<u64>,
    /// 链接形式：1：PC，2：无线，默认：１
    pub platform: Option<u8>,
    /// 商品筛选-淘客佣金比率上限。如：1234表示12.34%
    pub end_tk_rate: Option<u64>,
    /// 商品筛选-淘客佣金比率下限。如：1234表示12.34%
    pub start_tk_rate: Option<u64>,
    /// 商品筛选-折扣价范围上限。单位：元
    pub end_price: Option<u64>,
    /// 商品筛选-折扣价范围下限。单位：元
    pub start_price: Option<u64>,
    /// 商品筛选-KA媒体淘客佣金比率上限。如：1234表示12.34%
    pub end_ka_tk_rate: Option<u64>,
    /// 商品筛选-KA媒体淘客佣金比率下限。如：1234表示12.34%
    pub start_ka_tk_rate: Option<u64>,
    /// 商品筛选-是否天猫商品。true表示属于天猫商品，false或不设置表示不限
    pub is_tmall: Option<bool>,
    /// 优惠券筛选-是否有优惠券。true表示该商品有优惠券，false或不设置表示不限
    pub has_coupon: Option<bool>,
    /// 商品筛选-是否包邮。true表示包邮，false或不设置表示不限
    pub need_free_shipment: Option<bool>,
    /// 商品筛选-是否加入消费者保障。true表示加入，false或不设置表示不限
    pub need_prepay: Option<bool>,
    /// 商品筛选-好评率是否高于行业均值。True表示大于等于，false或不设置表示不限
    pub include_good_rate: Option<bool>,
    /// 商品筛选(特定媒体支持)-成交转化是否高于行业均值。True表示大于等于，false或不设置表示不限
    pub include_pay_rate_30: Option<bool>,
    /// 商品筛选(特定媒体支持)-退款率是否低于行业均值。True表示大于等于，false或不设置表示不限
    pub include_rfd_rate: Option<bool>,
    /// 商品筛选-牛皮癣程度。取值：1不限，2无，3轻微
    pub npx_level: Option<u8>,
    /// 排序_des（降序），排序_asc（升序），销量（total_sales），淘客佣金比率（tk_rate）， 累计推广量（tk_total_sales），总支出佣金（tk_total_commi），价格（price）
    pub sort: Option<String>,
    /// 商品筛选-所在地
    pub itemloc: Option<String>,
    /// ip参数影响邮费获取，如果不传或者传入不准确，邮费无法精准提供
    pub ip: Option<String>,
    /// 商品筛选-查询词
    pub q: Option<String>,
    /// 商品筛选-后台类目ID。用,分割，最大10个，该ID可以通过taobao.itemcats.get接口获取到
    pub cat: Option<String>,
    /// 不传时默认物料id=2836；如果直接对消费者投放，可使用官方个性化算法优化的搜索物料id=17004
    pub material_id: Option<u64>,
    /// 商品ID，用于相似商品推荐
    pub item_id: Option<u64>,
    /// 智能匹配-设备号加密后的值（MD5加密需32位小写），类型为OAID时传原始OAID值
    pub device_value: Option<String>,
    /// 智能匹配-设备号加密类型：MD5，类型为OAID时不传
    pub device_encrypt: Option<String>,
    /// 本地化业务入参-LBS信息-经度
    pub longitude: Option<String>,
    /// 本地化业务入参-LBS信息-纬度
    pub latitude: Option<String>,
    /// 本地化业务入参-LBS信息-国标城市码，仅支持单个请求，请求饿了么卡券物料时，该字段必填。 （详细城市ID见：https://mo.m.taobao.com/page_2020010315120200508）
    pub city_code: Option<String>,
    /// 商家id，仅支持饿了么卡券商家ID，支持批量请求1-100以内，多个商家ID使用英文逗号分隔
    pub seller_ids: Option<String>,
    /// 会员运营ID
    pub special_id: Option<String>,
    /// 渠道关系ID，仅适用于渠道推广场景
    pub relation_id: Option<String>,
    /// 智能匹配-设备号类型：IMEI，或者IDFA，或者UTDID（UTDID不支持MD5加密），或者OAID
    pub device_type: Option<String>,
    /// 锁佣结束时间
    pub lock_rate_end_time: Option<u64>,
    /// 锁佣开始时间
    pub lock_rate_start_time: Option<u64>,
}

impl TaobaoRequest for TbMaterialSearchRequest {
    fn get_api_method_name(&self) -> TaobaoMethod {
        TaobaoMethod::MaterialSearch
    }

    fn get_text_params(&self) -> BTreeMap<String, String> {
        let mut params = BTreeMap::new();
        params.insert("adzone_id".to_string(), self.adzone_id.to_string());
        if let Some(page_size) = &self.page_size {
            params.insert("page_size".to_string(), page_size.to_string());
        }
        if let Some(material_id) = &self.material_id {
            params.insert("material_id".to_string(), material_id.to_string());
        }
        if let Some(page_no) = &self.page_no {
            params.insert("page_no".to_string(), page_no.to_string());
        }
        if let Some(item_id) = &self.item_id {
            params.insert("item_id".to_string(), item_id.to_string());
        }
        if let Some(q) = &self.q {
            params.insert("q".to_string(), q.to_string());
        }
        if let Some(cat) = self.cat.to_owned() {
            params.insert("cat".to_owned(), cat.to_string());
        }
        params
    }
}

//----------------------------------------------------------------------------------------------------------------------------

