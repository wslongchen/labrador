use serde::Serialize;

use crate::request::Params;

//----------------------------------------------------------------------------------------------------------------------------

// 拼多多 ↓

#[derive(Debug, Serialize)]
pub struct PddGoodsRecommendParam {
    /// 请求数量；默认值 ： 400
    pub limit: Option<u64>,
    /// 猜你喜欢场景的商品类目
    /// 20100-百货，20200-母婴，20300-食品，20400-女装，20500-电器，20600-鞋包，20700-内衣
    /// 20800-美妆，20900-男装，21000-水果，21100-家纺，21200-文具,21300-运动,21400-虚拟
    /// 21500-汽车,21600-家装,21700-家具,21800-医药;
    pub cat_id: Option<u64>,
    /// 0-1.9包邮, 1-今日爆款, 2-品牌清仓,3-相似商品推荐,4-猜你喜欢,5-实时热销,6-实时收益,7-今日畅销,8-高佣榜单，默认1
    pub channel_type: Option<u64>,
    /// 从多少位置开始请求；默认值 ： 0，offset需是limit的整数倍，仅支持整页翻页
    pub offset: Option<u64>,
    /// 相似商品推荐场景时必传。商品Id，请求相似商品时，仅取数组的第一位
    pub goods_ids: Option<Vec<u64>>,
    /// 相似商品推荐场景时必传。goodsSign，请求相似商品时，仅取数组的第一位
    pub goods_sign_list: Option<Vec<String>>,
    /// 推广位id
    pub pid: Option<String>,
    /// 自定义参数，为链接打上自定义标签
    /// 自定义参数最长限制64个字节；格式为： {"uid":"11111","sid":"22222"} ，其中 uid 为用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填；
    ///  sid 为上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key。
    pub custom_parameters: Option<String>,
    /// 翻页时建议填写前页返回的list_id值
    pub list_id: Option<String>,
}

impl Params for PddGoodsRecommendParam {
    fn get_params(&self) -> Vec<(String, String)> { 
        let mut params = Vec::new();
        if let Some(limit) = self.limit {
            params.push(("limit".to_owned(), limit.to_string()));
        }
        if let Some(cat_id) = self.cat_id {
            params.push(("cat_id".to_owned(), cat_id.to_string()));
        }
        if let Some(channel_type) = self.channel_type {
            params.push(("channel_type".to_owned(), channel_type.to_string()));
        }
        if let Some(offset) = self.offset {
            params.push(("offset".to_owned(), offset.to_string()));
        }
        if let Some(pid) = self.pid.to_owned() {
            params.push(("pid".to_owned(), pid.to_string()));
        }
        if let Some(list_id) = self.list_id.to_owned() {
            params.push(("list_id".to_owned(), list_id.to_string()));
        }
        if let Some(custom_parameters) = self.custom_parameters.to_owned() {
            params.push(("custom_parameters".to_owned(), custom_parameters.to_string()));
        }
        if let Some(goods_sign_list) = self.goods_sign_list.to_owned() {
            params.push(("goods_sign_list".to_owned(), format!("[{:?}]",goods_sign_list.join(",").to_string())));
        }
        if let Some(goods_ids) = self.goods_ids.to_owned() {
            params.push(("goods_ids".to_owned(), format!("[{:?}]",goods_ids.iter().map(|item| item.to_string()).collect::<Vec<_>>().join(",").to_string())));
        }
        params
    }
}

#[derive(Debug, Serialize)]
pub struct PddGoodsSearchParam {
    /// 活动商品标记数组
    /// 例：[4,7]，4-秒杀，7-百亿补贴，31-品牌黑标，10564-精选爆品-官方直推爆款，10584-精选爆品-团长推荐
    /// 24-品牌高佣，20-行业精选，21-金牌商家，10044-潜力爆品，10475-爆品上新，其他的值请忽略
    pub activity_tags: Option<Vec<u64>>,
    /// 自定义屏蔽一级/二级/三级类目ID，自定义数量不超过20个;使用pdd.goods.cats.get接口获取cat_id
    pub block_cats: Option<Vec<u64>>,
    /// 商品类目ID，使用pdd.goods.cats.get接口获取
    pub cat_id: Option<u64>,
    /// 自定义参数，为链接打上自定义标签；自定义参数最长限制64个字节；
    /// 格式为： {"uid":"11111","sid":"22222"} ，其中 uid 用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填；
    /// sid 上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key
    pub custom_parameters: Option<String>,
    /// goodsSign列表，支持通过goodsSign查询商品，使用说明：https://jinbao.pinduoduo.com/qa-system?questionId=252
    pub goods_sign_list: Option<Vec<String>>,
    /// 是否为品牌商品
    pub is_brand_goods: Option<bool>,
    /// 是否只返回优惠券的商品，false返回所有商品，true只返回有优惠券的商品
    pub with_coupon: Option<bool>,
    /// 商品关键词，与opt_id字段选填一个或全部填写。
    /// 可支持短链（短链即为pdd.ddk.goods.promotion.url.generate接口生成的短链）
    pub keyword: Option<String>,
    /// 翻页时建议填写前页返回的list_id值
    pub list_id: Option<String>,
    /// 店铺类型，1-个人，2-企业，3-旗舰店，4-专卖店，5-专营店，6-普通店（未传为全部）
    pub merchant_type: Option<u8>,
    /// 商品标签类目ID，使用pdd.goods.opt.get获取
    pub opt_id: Option<u64>,
    /// 默认值1，商品分页数
    pub page: Option<u64>,
    /// 默认100，每页商品数量
    pub page_size: Option<u64>,
    /// 推广位id
    pub pid: Option<String>,
    /// 排序方式:0-综合排序;2-按佣金比例降序;3-按价格升序;4-按价格降序;6-按销量降序;9-券后价升序排序;10-券后价降序排序;16-店铺描述评分降序
    pub sort_type: Option<u8>,
    /// 店铺类型数组
    pub merchant_type_list: Option<Vec<u8>>,
    /// 屏蔽商品类目包：1-拼多多小程序屏蔽的类目&关键词;2-虚拟类目;3-医疗器械;4-处方药;5-非处方药
    pub block_cat_packages: Option<Vec<u8>>,
}

impl Params for PddGoodsSearchParam {
    fn get_params(&self) -> Vec<(String, String)> { 
        let mut params = Vec::new();
        if let Some(page) = self.page {
            params.push(("page".to_owned(), page.to_string()));
        }
        if let Some(cat_id) = self.cat_id {
            params.push(("cat_id".to_owned(), cat_id.to_string()));
        }
        if let Some(page_size) = self.page_size {
            params.push(("page_size".to_owned(), page_size.to_string()));
        }
        if let Some(keyword) = self.keyword.to_owned() {
            params.push(("keyword".to_owned(), keyword));
        }
        if let Some(pid) = self.pid.to_owned() {
            params.push(("pid".to_owned(), pid.to_string()));
        }
        if let Some(list_id) = self.list_id.to_owned() {
            params.push(("list_id".to_owned(), list_id));
        }
        if let Some(custom_parameters) = self.custom_parameters.to_owned() {
            params.push(("custom_parameters".to_owned(), custom_parameters));
        }
        if let Some(is_brand_goods) = self.is_brand_goods.to_owned() {
            params.push(("is_brand_goods".to_owned(), is_brand_goods.to_string()));
        }
        if let Some(with_coupon) = self.with_coupon.to_owned() {
            params.push(("with_coupon".to_owned(), with_coupon.to_string()));
        }
        if let Some(merchant_type) = self.merchant_type.to_owned() {
            params.push(("merchant_type".to_owned(), merchant_type.to_string()));
        }
        if let Some(opt_id) = self.opt_id.to_owned() {
            params.push(("opt_id".to_owned(), opt_id.to_string()));
        }
        if let Some(sort_type) = self.sort_type.to_owned() {
            params.push(("sort_type".to_owned(), sort_type.to_string()));
        }
        if let Some(goods_sign_list) = self.goods_sign_list.to_owned() {
            params.push(("goods_sign_list".to_owned(), format!("[{:?}]",goods_sign_list.join(",").to_string())));
        }
        if let Some(activity_tags) = self.activity_tags.to_owned() {
            params.push(("activity_tags".to_owned(), format!("[{:?}]",activity_tags.iter().map(|item| item.to_string()).collect::<Vec<_>>().join(",").to_string())));
        }
        params
    }
}


#[derive(Debug, Serialize)]
pub struct PddAuthorityQueryParam {
    /// 推广位id
    pub pid: Option<String>,
    /// 自定义参数，为链接打上自定义标签；自定义参数最长限制64个字节；
    /// 格式为： {"uid":"11111","sid":"22222"} ，其中 uid 用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填；
    /// sid 上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key
    pub custom_parameters: Option<String>,
}

impl Params for PddAuthorityQueryParam {
    fn get_params(&self) -> Vec<(String, String)> { 
        let mut params = Vec::new();
        if let Some(pid) = self.pid.to_owned() {
            params.push(("pid".to_owned(), pid.to_string()));
        }
        if let Some(custom_parameters) = self.custom_parameters.to_owned() {
            params.push(("custom_parameters".to_owned(), custom_parameters));
        }
        params
    }
}

#[derive(Debug, Serialize)]
pub struct PddPromoteUrlGenerateParam {
    /// 推广位id
    pub p_id: String,
    /// 多多礼金ID
    pub crash_gift_id: Option<u64>,
    /// 自定义参数，为链接打上自定义标签；自定义参数最长限制64个字节；
    /// 格式为： {"uid":"11111","sid":"22222"} ，其中 uid 用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填；
    /// sid 上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key
    pub custom_parameters: Option<String>,
    /// 是否生成带授权的单品链接。如果未授权，则会走授权流程
    pub generate_authority_url: Option<bool>,
    /// 是否生成店铺收藏券推广链接
    pub generate_mall_collect_coupon: Option<bool>,
    /// 是否生成qq小程序
    pub generate_qq_app: Option<bool>,
    /// 是否返回 schema URL
    pub generate_schema_url: Option<bool>,
    /// 是否生成短链接，true-是，false-否
    pub generate_short_url: Option<bool>,
    /// 是否生成小程序推广
    pub generate_we_app: Option<bool>,
    /// true--生成多人团推广链接 false--生成单人团推广链接（默认false）
    /// 1、单人团推广链接：用户访问单人团推广链接，可直接购买商品无需拼团。
    /// 2、多人团推广链接：用户访问双人团推广链接开团，若用户分享给他人参团，则开团者和参团者的佣金均结算给推手
    pub multi_group: Option<bool>,
    /// 商品ID，仅支持单个查询
    pub goods_id_list: Option<Vec<u64>>,
    /// 招商多多客ID
    pub zs_duo_id: Option<u64>,
    /// 商品goodsSign，用于查询指定商品，仅支持单个查询。
    pub goods_sign: Option<String>,
    /// 直播间id列表，如果生成直播间推广链接该参数必填，goods_id_list填[1]
    pub room_id_list: Option<Vec<String>>,
    /// 搜索id，建议填写，提高收益。来自pdd.ddk.goods.recommend.get、pdd.ddk.goods.search、pdd.ddk.top.goods.list.query等接口
    pub search_id: Option<String>,
    /// 直播预约id列表，如果生成直播间预约推广链接该参数必填，goods_id_list填[1]，room_id_list不填
    pub target_id_list: Option<Vec<String>>,
}

impl Params for PddPromoteUrlGenerateParam {
    fn get_params(&self) -> Vec<(String, String)> { 
        let mut params = Vec::new();
        params.push(("p_id".to_owned(), self.p_id.to_owned()));
        if let Some(custom_parameters) = self.custom_parameters.to_owned() {
            params.push(("custom_parameters".to_owned(), custom_parameters));
        }
        if let Some(generate_authority_url) = self.generate_authority_url.to_owned() {
            params.push(("generate_authority_url".to_owned(), generate_authority_url.to_string()));
        }
        if let Some(goods_sign) = self.goods_sign.to_owned() {
            params.push(("goods_sign".to_owned(), goods_sign));
        }
        params
    }
}


#[derive(Debug, Serialize)]
pub struct PddGoodsTopParam {
     /// 请求数量；默认值 ： 400
     pub limit: Option<u64>,
     /// 从多少位置开始请求；默认值 ： 0，offset需是limit的整数倍，仅支持整页翻页
     pub offset: Option<u64>,
     /// 1-实时热销榜；2-实时收益榜
     pub sort_type: Option<u8>,
     /// 推广位id
     pub p_id: Option<String>,
     /// 自定义参数，为链接打上自定义标签
    /// 自定义参数最长限制64个字节；格式为： {"uid":"11111","sid":"22222"} ，其中 uid 为用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填；
    ///  sid 为上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key。
    pub custom_parameters: Option<String>,
    /// 翻页时建议填写前页返回的list_id值
    pub list_id: Option<String>,
}

impl Params for PddGoodsTopParam {
    fn get_params(&self) -> Vec<(String, String)> { 
        let mut params = Vec::new();
        if let Some(limit) = self.limit {
            params.push(("limit".to_owned(), limit.to_string()));
        }
        if let Some(p_id) = self.p_id.to_owned() {
            params.push(("p_id".to_owned(), p_id.to_string()));
        }
        if let Some(sort_type) = self.sort_type {
            params.push(("sort_type".to_owned(), sort_type.to_string()));
        }
        if let Some(offset) = self.offset {
            params.push(("offset".to_owned(), offset.to_string()));
        }
        if let Some(list_id) = self.list_id.to_owned() {
            params.push(("list_id".to_owned(), list_id.to_string()));
        }
        if let Some(custom_parameters) = self.custom_parameters.to_owned() {
            params.push(("custom_parameters".to_owned(), custom_parameters.to_string()));
        }
        params
    }
}

#[derive(Debug, Serialize)]
pub struct PddGoodsDetailParam {
     /// 商品ID，仅支持单个查询。例如：[123456]
     pub goods_id_list: Option<Vec<u64>>,
     /// 商品goodsSign，支持通过goods_sign查询商品。优先使用此字段进行查询
     pub goods_sign: Option<String>,
     /// 搜索id，建议填写，提高收益。来自pdd.ddk.goods.recommend.get、pdd.ddk.goods.search、pdd.ddk.top.goods.list.query等接口
     pub search_id: Option<String>,
     /// 推广位id
     pub pid: Option<String>,
     /// 自定义参数，为链接打上自定义标签
    /// 自定义参数最长限制64个字节；格式为： {"uid":"11111","sid":"22222"} ，其中 uid 为用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填；
    ///  sid 为上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key。
    pub custom_parameters: Option<String>,
    /// 招商多多客ID
    pub zs_duo_id: Option<u64>,
}

impl Params for PddGoodsDetailParam {
    fn get_params(&self) -> Vec<(String, String)> { 
        let mut params = Vec::new();
        if let Some(goods_sign) = self.goods_sign.to_owned() {
            params.push(("goods_sign".to_owned(), goods_sign));
        }
        if let Some(pid) = self.pid.to_owned() {
            params.push(("pid".to_owned(), pid));
        }
        if let Some(search_id) = self.search_id.to_owned() {
            params.push(("search_id".to_owned(), search_id));
        }
        if let Some(zs_duo_id) = self.zs_duo_id {
            params.push(("zs_duo_id".to_owned(), zs_duo_id.to_string()));
        }
        if let Some(goods_id_list) = self.goods_id_list.to_owned() {
            params.push(("goods_id_list".to_owned(), format!("[{:?}]",goods_id_list.iter().map(|item| item.to_string()).collect::<Vec<_>>().join(",").to_string())));
        }
        if let Some(custom_parameters) = self.custom_parameters.to_owned() {
            params.push(("custom_parameters".to_owned(), custom_parameters));
        }
        params
    }
}


#[derive(Debug, Serialize)]
pub struct PddZsUrlGenerateParam {
     /// 需转链的链接【重要：2020年8月24号后不再支持长链】
     pub source_url: Option<String>,
     /// 推广位id
     pub pid: Option<String>,
     /// 自定义参数，为链接打上自定义标签
    ///  自定义参数最长限制64个字节；格式为： {"uid":"11111","sid":"22222"} ，其中 uid 为用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填；
    ///  sid 为上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key。
    pub custom_parameters: Option<String>,
}

impl Params for PddZsUrlGenerateParam {
    fn get_params(&self) -> Vec<(String, String)> { 
        let mut params = Vec::new();
        if let Some(source_url) = self.source_url.to_owned() {
            params.push(("source_url".to_owned(), source_url));
        }
        if let Some(pid) = self.pid.to_owned() {
            params.push(("pid".to_owned(), pid));
        }
        if let Some(custom_parameters) = self.custom_parameters.to_owned() {
            params.push(("custom_parameters".to_owned(), custom_parameters));
        }
        params
    }
}


#[derive(Debug, Serialize)]
pub struct PddRpUrlGenerateParam {
     /// -1-活动列表，0-默认红包，2–新人红包，3-刮刮卡，5-员工内购，6-购物车，7-大促会场，8-直播间列表集合页，10-生成绑定备案链接，12-砸金蛋
     pub channel_type: Option<u64>,
     /// 自定义参数，为链接打上自定义标签
    ///  自定义参数最长限制64个字节；格式为： {"uid":"11111","sid":"22222"} ，其中 uid 为用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填；
    ///  sid 为上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key。
    pub custom_parameters: Option<String>,
    /// 红包自定义参数
    pub diy_red_packet_param: Option<PddDiyRedPacket>,
    /// 是否生成qq小程序
    pub generate_qq_app: Option<bool>,
    /// 是否返回 schema URL
    pub generate_schema_url: Option<bool>,
    /// 是否生成短链接。true-是，false-否，默认false
    pub generate_short_url: Option<bool>,
    /// 是否生成小程序推广
    pub generate_we_app: Option<bool>,
    /// 推广位列表，例如：["60005_612"]
    pub p_id_list: Option<Vec<String>>,
    /// 初始金额（单位分），有效金额枚举值：300、500、700、1100和1600，默认300
    pub amount: Option<u64>,
    /// 刮刮卡指定金额（单位分），可指定2-100元间数值，即有效区间为：[200,10000]
    pub scratch_card_amount: Option<u64>,

}

#[derive(Debug, Serialize)]
pub struct PddDiyRedPacket {
    /// 红包金额列表，200、300、500、1000、2000，单位分。红包金额和红包抵后价设置只能二选一，默认设置了红包金额会忽略红包抵后价设置
    pub amount_probability: Option<Vec<u64>>,
    /// 优先展示类目
    pub opt_id: Option<Vec<u64>>,
    /// 设置玩法，false-现金红包, true-现金券
    pub dis_text: Option<bool>,
    /// 推广页设置，false-红包开启页, true-红包领取页
    pub not_show_background: Option<bool>,
    /// 
    pub range_items: Option<PddRangeItem>,
}

#[derive(Debug, Serialize)]
pub struct PddRangeItem {
    /// 区间的开始值
    pub range_from: Option<u64>,
    /// range_id为1表示红包抵后价（单位分）， range_id为2表示佣金比例（单位千分之几)
    pub range_id: Option<u64>,
    /// 区间的结束值
    pub range_to: Option<u64>,
}



impl Params for PddRpUrlGenerateParam {
    fn get_params(&self) -> Vec<(String, String)> { 
        let mut params = Vec::new();
        if let Some(channel_type) = self.channel_type.to_owned() {
            params.push(("channel_type".to_owned(), channel_type.to_string()));
        }
        if let Some(generate_qq_app) = self.generate_qq_app.to_owned() {
            params.push(("generate_qq_app".to_owned(), generate_qq_app.to_string()));
        }
        if let Some(generate_schema_url) = self.generate_schema_url.to_owned() {
            params.push(("generate_schema_url".to_owned(), generate_schema_url.to_string()));
        }
        if let Some(generate_short_url) = self.generate_short_url.to_owned() {
            params.push(("generate_short_url".to_owned(), generate_short_url.to_string()));
        }
        if let Some(generate_we_app) = self.generate_we_app.to_owned() {
            params.push(("generate_we_app".to_owned(), generate_we_app.to_string()));
        }
        if let Some(amount) = self.amount.to_owned() {
            params.push(("amount".to_owned(), amount.to_string()));
        }
        if let Some(scratch_card_amount) = self.scratch_card_amount.to_owned() {
            params.push(("scratch_card_amount".to_owned(), scratch_card_amount.to_string()));
        }
        if let Some(p_id_list) = self.p_id_list.to_owned() {
            params.push(("p_id_list".to_owned(), format!("[\"{}\"]",p_id_list.join("\",\"").to_string())));
        }
        if let Some(diy_red_packet_param) = &self.diy_red_packet_param {
            params.push(("diy_red_packet_param".to_owned(), serde_json::to_string(diy_red_packet_param).unwrap_or_default()));
        }
        if let Some(custom_parameters) = self.custom_parameters.to_owned() {
            params.push(("custom_parameters".to_owned(), custom_parameters));
        }
        params
    }
}

#[derive(Debug, Serialize)]
pub struct PddCmsUrlGenerateParam {
     /// -1-活动列表，0-默认红包，2–新人红包，3-刮刮卡，5-员工内购，6-购物车，7-大促会场，8-直播间列表集合页，10-生成绑定备案链接，12-砸金蛋
     pub channel_type: Option<u64>,
     /// 自定义参数，为链接打上自定义标签
    ///  自定义参数最长限制64个字节；格式为： {"uid":"11111","sid":"22222"} ，其中 uid 为用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填；
    ///  sid 为上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key。
    pub custom_parameters: Option<String>,
    /// 是否返回 schema URL
    pub generate_schema_url: Option<bool>,
    /// 是否生成手机跳转链接。true-是，false-否，默认false
    pub generate_mobile: Option<bool>,
    /// 是否生成短链接。true-是，false-否，默认false
    pub generate_short_url: Option<bool>,
    /// 单人团多人团标志。true-多人团，false-单人团 默认false
    pub multi_group: Option<bool>,
    /// 推广位列表，例如：["60005_612"]
    pub p_id_list: Option<Vec<String>>,

}

impl Params for PddCmsUrlGenerateParam {
    fn get_params(&self) -> Vec<(String, String)> { 
        let mut params = Vec::new();
        if let Some(channel_type) = self.channel_type.to_owned() {
            params.push(("channel_type".to_owned(), channel_type.to_string()));
        }
       
        if let Some(generate_schema_url) = self.generate_schema_url.to_owned() {
            params.push(("generate_schema_url".to_owned(), generate_schema_url.to_string()));
        }
        if let Some(generate_mobile) = self.generate_mobile.to_owned() {
            params.push(("generate_mobile".to_owned(), generate_mobile.to_string()));
        }
        if let Some(multi_group) = self.multi_group.to_owned() {
            params.push(("multi_group".to_owned(), multi_group.to_string()));
        }
        if let Some(generate_short_url) = self.generate_short_url.to_owned() {
            params.push(("generate_short_url".to_owned(), generate_short_url.to_string()));
        }
        
        if let Some(p_id_list) = self.p_id_list.to_owned() {
            params.push(("p_id_list".to_owned(), format!("[\"{}\"]",p_id_list.join("\",\"").to_string())));
        }

        if let Some(custom_parameters) = self.custom_parameters.to_owned() {
            params.push(("custom_parameters".to_owned(), custom_parameters));
        }
        params
    }
}

#[derive(Debug, Serialize)]
pub struct PddOrderRangeQueryParam {
    /// 上一次的迭代器id(第一次不填)
    pub last_order_id: Option<String>,
    /// 支付起始时间，如2019-05-07 00:00:00
    pub start_time: Option<String>,
    /// 支付结束时间，如2019-05-07 00:00:00
    pub end_time: Option<String>,
    /// 每次请求多少条，建议300
    pub page_size: Option<u64>,
    /// 订单类型：1-推广订单；2-直播间订单
    pub query_order_type: Option<u8>,
}

impl Params for PddOrderRangeQueryParam {
    fn get_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(last_order_id) = self.last_order_id.to_owned() {
            params.push(("last_order_id".to_owned(), last_order_id.to_string()));
        }
       
        if let Some(start_time) = self.start_time.to_owned() {
            params.push(("start_time".to_owned(), start_time.to_string()));
        }
        if let Some(end_time) = self.end_time.to_owned() {
            params.push(("end_time".to_owned(), end_time.to_string()));
        }
        if let Some(page_size) = self.page_size.to_owned() {
            params.push(("page_size".to_owned(), page_size.to_string()));
        }
        if let Some(query_order_type) = self.query_order_type.to_owned() {
            params.push(("query_order_type".to_owned(), query_order_type.to_string()));
        }
        params
    }
}

#[derive(Debug, Serialize)]
pub struct PddOrderDetailParam {
    /// 订单号
    pub order_sn: String,
    /// 订单类型：1-推广订单；2-直播间订单
    pub query_order_type: Option<u8>,
}

impl Params for PddOrderDetailParam {
    fn get_params(&self) -> Vec<(String, String)> { 
        let mut params = Vec::new();
        params.push(("order_sn".to_owned(), self.order_sn.to_string()));
       
        if let Some(query_order_type) = self.query_order_type.to_owned() {
            params.push(("query_order_type".to_owned(), query_order_type.to_string()));
        }
        params
    }
}


#[derive(Debug, Serialize)]
pub struct PddOrderIncrementQueryParam {
    /// 查询结束时间，和开始时间相差不能超过24小时。
    /// note：此时间为时间戳，指格林威治时间 1970 年01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总秒数
    pub end_update_time: u64,
    /// 最近90天内多多进宝商品订单更新时间--查询时间开始。
    /// note：此时间为时间戳，指格林威治时间 1970 年01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总秒数
    pub start_update_time: u64,
    /// 是否返回总数，默认为true，如果指定false, 则返回的结果中不包含总记录数
    /// 通过此种方式获取增量数据，效率在原有的基础上有80%的提升。
    pub return_count: Option<bool>,
    /// 返回的每页结果订单数，默认为100，范围为10到100
    /// 建议使用40~50，可以提高成功率，减少超时数量。
    pub page_size: Option<u64>,
    /// 第几页，从1到10000，默认1
    /// 注：使用最后更新时间范围增量同步时，必须采用倒序的分页方式（从最后一页往回取）才能避免漏单问题。
    pub page: Option<u64>,
    /// 订单类型：1-推广订单；2-直播间订单
    pub query_order_type: Option<u8>,
}

impl Params for PddOrderIncrementQueryParam {
    fn get_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(return_count) = self.return_count.to_owned() {
            params.push(("return_count".to_owned(), return_count.to_string()));
        }
        params.push(("end_update_time".to_owned(), self.end_update_time.to_string()));
        params.push(("start_update_time".to_owned(), self.start_update_time.to_string()));
        if let Some(page_size) = self.page_size.to_owned() {
            params.push(("page_size".to_owned(), page_size.to_string()));
        }
        if let Some(page) = self.page.to_owned() {
            params.push(("page".to_owned(), page.to_string()));
        }
        if let Some(query_order_type) = self.query_order_type.to_owned() {
            params.push(("query_order_type".to_owned(), query_order_type.to_string()));
        }
        params
    }
}


#[derive(Debug, Serialize)]
pub struct PddPidGenerateParam {
    /// 要生成的推广位数量，默认为10，范围为：1~100
    pub number: u32,
    /// 推广位名称，例如["1","2"]
    pub p_id_name_list: Option<Vec<String>>,
    /// 媒体id
    pub media_id: Option<u64>,
}

impl Params for PddPidGenerateParam {
    fn get_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(media_id) = self.media_id.to_owned() {
            params.push(("media_id".to_owned(), media_id.to_string()));
        }
        params.push(("number".to_owned(), self.number.to_string()));
        if let Some(p_id_name_list) = self.p_id_name_list.to_owned() {
            params.push(("p_id_name_list".to_owned(), format!("[\"{}\"]",p_id_name_list.join("\",\"").to_string())));
        }
        params
    }
}

#[derive(Debug, Serialize)]
pub struct PddPidQueryParam {
    /// 返回的每页推广位数量
    pub page_size: Option<u64>,
    /// 返回的页数
    pub page: Option<u64>,
    /// 推广位状态：0-正常，1-封禁
    pub status: Option<u8>,
    /// 推广位id列表
    pub pid_list: Option<Vec<String>>,
}

impl Params for PddPidQueryParam {
    fn get_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(status) = self.status.to_owned() {
            params.push(("status".to_owned(), status.to_string()));
        }
        if let Some(page_size) = self.page_size.to_owned() {
            params.push(("page_size".to_owned(), page_size.to_string()));
        }
        if let Some(page) = self.page.to_owned() {
            params.push(("page".to_owned(), page.to_string()));
        }
        if let Some(pid_list) = self.pid_list.to_owned() {
            params.push(("pid_list".to_owned(), format!("[\"{}\"]",pid_list.join("\",\"").to_string())));
        }
        params
    }
}

#[derive(Debug, Serialize)]
pub struct PddPidBindMediaParam {
    /// 媒体id
    pub media_id: u64,
    /// pid列表，最多支持同时传入1000个
    pub pid_list: Vec<String>,
}

impl Params for PddPidBindMediaParam {
    fn get_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("media_id".to_owned(), self.media_id.to_string()));
        params.push(("pid_list".to_owned(), format!("[\"{}\"]",self.pid_list.join("\",\"").to_string())));
        params
    }
}