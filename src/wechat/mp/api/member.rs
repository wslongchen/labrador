use std::vec;

use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

use crate::{session::SessionStore, request::{RequestType}, WechatCommonResponse, WechatMpClient, LabradorResult, LabraError, get_timestamp};
use crate::wechat::mp::constants::MEMBER_CARD;
use crate::wechat::mp::method::{MpMemeberCardMethod, WechatMpMethod};

/// 会员卡相关.
#[derive(Debug, Clone)]
pub struct WechatMpMember<'a, T: SessionStore> {
    client: &'a WechatMpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WechatMpMember<'a, T> {

    #[inline]
    pub fn new(client: &WechatMpClient<T>) -> WechatMpMember<T> {
        WechatMpMember {
            client,
        }
    }

    /// <pre>
    /// 会员卡创建接口
    /// </pre>
    pub async fn create_member_card_custom<D: Serialize>(&self, req: D) -> LabradorResult<WechatMpCardCreateResponse> {
        let v = serde_json::to_value(req)?;
        let req = serde_json::from_value::<WechatMpMemberCardCreateRequest>(v)?;
        self.create_member_card(req).await
    }

    /// <pre>
    /// 会员卡创建接口
    /// </pre>
    pub async fn create_member_card(&self, req: WechatMpMemberCardCreateRequest) -> LabradorResult<WechatMpCardCreateResponse> {
        req.valid_check()?;
        let v = self.client.post(WechatMpMethod::MemberCard(MpMemeberCardMethod::Create), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatMpCardCreateResponse>(v)
    }

    /// <pre>
    /// 会员卡激活接口
    /// </pre>
    pub async fn activate_member_card(&self, req: WechatMpMemberCardActivateRequest) -> LabradorResult<WechatCommonResponse> {
        self.client.post(WechatMpMethod::MemberCard(MpMemeberCardMethod::Activate), vec![], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 拉取会员信息接口
    /// </pre>
    pub async fn get_user_info(&self, card_id: &str, code: &str) -> LabradorResult<WechatMpMemberCardUserInfoResponse> {
        let req = json!({
            "card_id": card_id,
            "code": code
        });
        let v = self.client.post(WechatMpMethod::MemberCard(MpMemeberCardMethod::GetUserInfo), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatMpMemberCardUserInfoResponse>(v)
    }

    /// <pre>
    /// 当会员持卡消费后，支持开发者调用该接口更新会员信息.
    /// 会员卡交易后的每次信息变更需通过该接口通知微信，便于后续消息通知及其他扩展功能。
    /// 1.开发者可以同时传入add_bonus和bonus解决由于同步失败带来的幂等性问题。
    /// 同时传入add_bonus和bonus时 add_bonus作为积分变动消息中的变量值，而bonus作为卡面上的总积分额度显示。余额变动同理。
    /// 2.开发者可以传入is_notify_bonus控制特殊的积分对账变动不发送消息，余额变动同理。
    /// </pre>
    pub async fn update_user_member_card(&self, req: WechatMpMemberCardUpdateRequest) -> LabradorResult<WechatMpMemberCardUpdateResponse> {
        let v = self.client.post(WechatMpMethod::MemberCard(MpMemeberCardMethod::UpdateUser), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatMpMemberCardUpdateResponse>(v)
    }

    /// <pre>
    /// 设置会员卡激活的字段（会员卡设置：wx_activate=true 时需要）.
    /// </pre>
    pub async fn set_activate_user_form(&self, req: WechatMpMemberCardActivateUserFormRequest) -> LabradorResult<WechatCommonResponse> {
        self.client.post(WechatMpMethod::MemberCard(MpMemeberCardMethod::ActivateSetUser), vec![], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 获取会员卡开卡插件参数(跳转型开卡组件需要参数).
    /// </pre>
    pub async fn get_activate_plugin_param(&self, card_id: &str, out_str: &str) -> LabradorResult<ActivatePluginParam> {
        let url = self.get_activate_plugin_url(card_id, out_str).await?;
        let decode_url = urlencoding::encode(&url);
        let mut params = serde_urlencoded::from_str::<ActivatePluginParam>(decode_url.as_ref())?;
        params.biz = params.biz + "==";
        Ok(params)
    }

    /// <pre>
    /// 获取开卡组件链接接口
    /// </pre>
    pub async fn get_activate_plugin_url(&self, card_id: &str, out_str: &str) -> LabradorResult<String> {
        let req = json!({
           "card_id": card_id,
            "outer_str": out_str
        });
        let v = self.client.post(WechatMpMethod::MemberCard(MpMemeberCardMethod::ActivateGetUrl), vec![], req, RequestType::Json).await?.json::<Value>()?;
        let v = WechatCommonResponse::parse::<Value>(v)?;
        let url = v["url"].as_str().unwrap_or_default();
        Ok(url.to_string())
    }

    /// <pre>
    /// 更新会员卡信息
    /// </pre>
    pub async fn update_card_info(&self, req: MemberCardUpdateRequest) -> LabradorResult<bool> {
        let v = self.client.post(WechatMpMethod::MemberCard(MpMemeberCardMethod::Update), vec![], req, RequestType::Json).await?.json::<Value>()?;
        let v = WechatCommonResponse::parse::<Value>(v)?;
        let send_check = v["send_check"].as_bool().unwrap_or_default();
        /// 此次更新是否需要提审，true为需要，false为不需要。
        Ok(send_check)
    }

    /// <pre>
    /// 解析跳转型开卡字段用户提交的资料.
    /// 开发者在URL上截取ticket后须先进行urldecode
    /// </pre>
    pub async fn get_activate_tempinfo(&self, activate_ticket: &str) -> LabradorResult<WechatMpMemberCardActivateTempInfoResponse> {
        let v = self.client.post(WechatMpMethod::MemberCard(MpMemeberCardMethod::Update), vec![], json!({ "activate_ticket": activate_ticket }), RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatMpMemberCardActivateTempInfoResponse>(v)
    }
}

//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpMemberCardCreateRequest {
    pub card: MemberCardCreateRequest,
}

impl WechatMpMemberCardCreateRequest {
    pub fn valid_check(&self) -> LabradorResult<()> {
        let req = &self.card;
        if req.card_type.ne(MEMBER_CARD) {
            return Err(LabraError::RequestError("卡券类型必须等于MEMBER_CARD".to_string()));
        }
        let member_card = &req.member_card;
        if member_card.prerogative.is_empty() {
            return Err(LabraError::RequestError("会员卡特权说明不能为空:prerogative".to_string()));
        }
        //卡片激活规则
        if !member_card.auto_activate && member_card.wx_activate && member_card.activate_url.is_none() {
            return Err(LabraError::RequestError("会员卡激活方式为接口激活，activate_url不能为空".to_string()));
        }
        let base_info = &member_card.base_info;
        if base_info.logo_url.is_empty() {
            return Err(LabraError::RequestError("会员卡基本信息的商户logo:logo_url不能为空".to_string()));
        }
        if base_info.code_type.is_empty() {
            return Err(LabraError::RequestError("会员卡基本信息的条码类型:code_type不能为空".to_string()));
        }
        if base_info.brand_name.is_empty() {
            return Err(LabraError::RequestError("会员卡基本信息的商户名字:brand_name不能为空".to_string()));
        }
        if base_info.brand_name.len() > 12 {
            return Err(LabraError::RequestError("会员卡基本信息的商户名字:brand_name长度不能大于12个汉字".to_string()));
        }
        if base_info.title.is_empty() {
            return Err(LabraError::RequestError("会员卡基本信息的卡券名称:title不能为空".to_string()));
        }
        if base_info.title.len() > 9 {
            return Err(LabraError::RequestError("会员卡基本信息的卡券名称:title长度不能大于9个汉字".to_string()));
        }
        if base_info.color.is_empty() {
            return Err(LabraError::RequestError("会员卡基本信息的卡颜色:color不能为空".to_string()));
        }
        if CardColor::from_str(&base_info.color) == CardColor::Unknow {
            return Err(LabraError::RequestError(format!("会员卡基本信息的卡颜色:{} 不支持", base_info.color)));
        }
        if base_info.notice.is_empty() {
            return Err(LabraError::RequestError("会员卡基本信息的使用提醒:notice不能为空".to_string()));
        }
        if base_info.description.is_empty() {
            return Err(LabraError::RequestError("会员卡基本信息的使用说明:description不能为空".to_string()));
        }

        let date_info = &base_info.date_info;
        let date_info_type = DateInfoType::from_str(&date_info.r#type);
        if date_info_type == DateInfoType::Unknow {
            return Err(LabraError::RequestError(format!("会员卡基本信息的使用日期类型:{} 不合法", date_info.r#type)));
        }
        //固定时长
        if date_info_type == DateInfoType::DATE_TYPE_FIX_TERM && (date_info.fixed_term.is_none() || date_info.fixed_begin_term.is_none()) {
            return Err(LabraError::RequestError(format!("会员卡基本信息的使用日期为:固定日期 fixedTerm和fixedBeginTerm不能为空")));
        }
        //固定期限
        if date_info_type == DateInfoType::DATE_TYPE_FIX_TIME_RANGE && (date_info.begin_timestamp.is_none() || date_info.end_timestamp.is_none()) {
            return Err(LabraError::RequestError(format!("会员卡基本信息的使用日期为:固定期限 fixedTerm和fixedBeginTerm不能为空")));
        }
        let current_tmp = get_timestamp();
        if date_info_type == DateInfoType::DATE_TYPE_FIX_TIME_RANGE && (date_info.begin_timestamp.unwrap_or_default() * 1000 < current_tmp || date_info.end_timestamp.unwrap_or_default() * 1000 < current_tmp || date_info.begin_timestamp.unwrap_or_default() > date_info.end_timestamp.unwrap_or_default()) {
            return Err(LabraError::RequestError(format!("会员卡基本信息的使用日期为:固定期限，beginTimestamp和endTimestamp的值不合法，请检查")));
        }

        if !base_info.use_all_locations.unwrap_or_default() && base_info.location_id_list.is_none() {
            return Err(LabraError::RequestError("会员卡基本信息的门店使用范围选择指定门店,门店列表:locationIdList不能为空".to_string()));
        }
        // 校验高级信息
        if let Some(advanced_info) = &member_card.advanced_info {
            if let Some(busi_serv_list) = &advanced_info.business_service {
                for bs in busi_serv_list {
                    if BusinessServiceType::from_str(bs) == BusinessServiceType::Unknow {
                        return Err(LabraError::RequestError(format!("会员卡高级信息的商户服务:{} 不合法", bs)));
                    }
                }
            }
        }
        Ok(())
    }
}

/// 创建会员卡请求对象
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberCardCreateRequest {
    pub card_type: String,
    pub member_card: MemberCard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberCard {
    /// 会员卡背景图
    pub background_pic_url: String,
    /// 基本信息
    pub base_info: BaseInfo,
    /// 特权说明
    pub prerogative: String,
    /// 自动激活
    pub auto_activate: bool,
    /// 显示积分
    pub supply_bonus: bool,
    /// 查看积分外链,设置跳转外链查看积分详情。仅适用于积分无法通过激活接口同步的情况下使用该字段.
    pub bonus_url: Option<String>,
    /// 支持储值
    pub supply_balance: bool,
    /// 余额外链,仅适用于余额无法通过激活接口同步的情况下使用该字段.
    pub balance_url: Option<String>,
    /// 自定义会员类目1,会员卡激活后显示.
    pub custom_field1: Option<CustomField>,
    /// 自定义会员类目2
    pub custom_field2: Option<CustomField>,
    /// 自定义会员类目3
    pub custom_field3: Option<CustomField>,
    /// 积分清零规则
    pub bonus_cleared: Option<String>,
    /// 积分规则
    pub bonus_rules: Option<String>,
    /// 储值规则.
    pub balance_rules: Option<String>,
    /// 激活会员卡的url.
    pub activate_url: Option<String>,
    /// 激活会原卡url对应的小程序user_name，仅可跳转该公众号绑定的小程序.
    pub activate_app_brand_user_name: Option<String>,
    /// 激活会原卡url对应的小程序path
    pub activate_app_brand_pass: Option<String>,
    /// 自定义会员信息类目，会员卡激活后显示.
    pub custom_cell1: Option<CustomCell1>,
    /// 自定义会员信息类目，会员卡激活后显示.
    pub custom_cell2: Option<CustomCell1>,
    /// 自定义会员信息类目，会员卡激活后显示.
    pub custom_cell3: Option<CustomCell1>,
    /// 积分规则,JSON结构积分规则.
    pub bonus_rule: Option<BonusRule>,
    /// 折扣,该会员卡享受的折扣优惠,填10就是九折.
    pub discount: Option<i64>,
    /// 创建优惠券特有的高级字段
    pub advanced_info: Option<AdvancedInfo>,
    /// 是否支持一键激活 ，填true或false.
    pub wx_activate: bool,
    /// 是否支持跳转型一键激活，填true或false.
    pub wx_activate_after_submit: bool,
    /// 跳转型一键激活跳转的地址链接，请填写http:// 或者https://开头的链接.
    pub wx_activate_after_submit_url: Option<String>,
    /// 参照https://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1499332673_Unm7V卡券内跳转小程序
    /// 积分信息类目对应的小程序 user_name，格式为原始id+@app
    pub bonus_app_brand_user_name: Option<String>,
    /// 积分入口小程序的页面路径
    pub bonus_app_brand_pass: Option<String>,
    /// 余额信息类目对应的小程序 user_name，格式为原始id+@app
    pub balance_app_brand_user_name: Option<String>,
    /// 余额入口小程序的页面路径
    pub balance_app_brand_pass: Option<String>,
}

/// 微信会员卡基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseInfo {
    /// 卡券的商户logo,建议像素为300*300.
    pub logo_url: String,
    /// Code展示类型.
    /// "CODE_TYPE_TEXT" 文本 "CODE_TYPE_BARCODE" 一维码 "CODE_TYPE_QRCODE" 二维码 "CODE_TYPE_ONLY_QRCODE" 仅显示二维码 "CODE_TYPE_ONLY_BARCODE" 仅显示一维码 "CODE_TYPE_NONE" 不显示任何码型
    pub code_type: String,
    pub pay_info: Option<PayInfo>,
    /// 是否设置该会员卡中部的按钮同时支持微信支付刷卡和会员卡二维码
    pub is_pay_and_qrcode: Option<bool>,
    /// 商户名字,字数上限为12个汉字.
    pub brand_name: String,
    /// 卡券名,字数上限为9个汉字 (建议涵盖卡券属性、服务及金额).
    pub title: String,
    /// 券颜色,按色彩规范标注填写Color010-Color100.
    pub color: String,
    /// 卡券使用提醒,字数上限为16个汉字.
    pub notice: String,
    /// 卡券使用说明,字数上限为1024个汉字.
    pub description: String,
    /// 商品信息
    pub sku: MemberCardSkuInfo,
    /// 使用日期,有效期的信息.
    pub date_info: DateInfo,
    /// 是否自定义Code码,填写true或false.
    /// 默认为false 通常自有优惠码系统的开发者选择自定义Code码，详情见 是否自定义code
    pub use_custom_code: Option<bool>,
    /// 是否指定用户领取,填写true或false。默认为false.
    pub bind_openid: Option<bool>,
    /// 客服电话
    pub service_phone: Option<String>,
    /// 门店位置ID,调用 POI门店管理接口 获取门店位置ID.
    pub location_id_list: Option<Vec<String>>,
    /// 会员卡是否支持全部门店,填写后商户门店更新时会自动同步至卡券.
    pub use_all_locations: Option<bool>,
    /// 卡券中部居中的按钮,仅在卡券激活后且可用状态 时显示.
    pub center_title: Option<String>,
    /// 显示在入口下方的提示语,仅在卡券激活后且可用状态时显示.
    pub center_sub_title: Option<String>,
    /// 顶部居中的url,仅在卡券激活后且可用状态时显示.
    pub center_url: Option<String>,
    /// 自定义跳转外链的入口名字
    pub custom_url_name: Option<String>,
    /// 自定义跳转的URL
    pub custom_url: Option<String>,
    /// 显示在入口右侧的提示语
    pub custom_url_sub_title: Option<String>,
    /// 营销场景的自定义入口名称
    pub promotion_url_name: Option<String>,
    /// 入口跳转外链的地址链接
    pub promotion_url: Option<String>,
    /// 显示在营销入口右侧的提示语
    pub promotion_url_sub_title: Option<String>,
    /// 每人可领券的数量限制,建议会员卡每人限领一张.
    pub get_limit: Option<i32>,
    /// 每人可核销的数量限制,不填写默认为50.
    pub use_limit: Option<i32>,
    /// 卡券领取页面是否可分享,默认为true.
    pub can_share: Option<bool>,
    /// 卡券是否可转赠,默认为true.
    pub can_give_friend: Option<bool>,
    /// 用户点击进入会员卡时推送事件.
    /// 填写true为用户点击进入会员卡时推送事件，默认为false。详情见 进入会员卡事件推送
    pub need_push_on_view: Option<bool>,
    /// 微信小程序开放功能 小程序&卡券打通部分新增8个字段 https://mp.weixin.qq.com/cgi-bin/announce?action=getannouncement&key=1490190158&version=1&lang=zh_CN&platform=2
    /// 自定义使用入口跳转小程序的user_name，格式为原始id+@app
    pub custom_app_brand_user_name: Option<String>,
    /// 自定义使用入口小程序页面地址
    pub custom_app_brand_pass: Option<String>,
    /// 小程序的user_name
    pub center_app_brand_user_name: Option<String>,
    /// 自定义居中使用入口小程序页面地址
    pub center_app_brand_pass: Option<String>,
    /// 小程序的user_name
    pub promotion_app_brand_user_name: Option<String>,
    /// 自定义营销入口小程序页面地址
    pub promotion_app_brand_pass: Option<String>,
    /// 小程序的user_name
    pub activate_app_brand_user_name: Option<String>,
    /// 激活小程序页面地址
    pub activate_app_brand_pass: Option<String>,
    /// https://developers.weixin.qq.com/doc/offiaccount/Cards_and_Offer/Managing_Coupons_Vouchers_and_Cards.html#2
    /// “CARD_STATUS_NOT_VERIFY”,待审核 ；
    /// “CARD_STATUS_VERIFY_FAIL”,审核失败；
    /// “CARD_STATUS_VERIFY_OK”，通过审核；
    /// “CARD_STATUS_DELETE”，卡券被商户删除；
    /// “CARD_STATUS_DISPATCH”，在公众平台投放过的卡券
    pub status: Option<String>,
}
/// 微信会员卡高级字段信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedInfo {
    /// 使用门槛（条件）.
    /// 若不填写使用条件则在券面拼写 ：无最低消费限制，全场通用，不限品类；并在使用说明显示： 可与其他优惠共享
    pub use_condition: Option<UseCondition>,
    /// 封面摘要
    #[serde(rename="abstract")]
    pub abstracts: Option<Abstracts>,
    /// 图文列表.
    /// 显示在详情内页 ，优惠券券开发者须至少传入 一组图文列表
    pub text_image_list: Option<Vec<TextImageList>>,
    /// 商家服务类型.
    /// 数组类型:BIZ_SERVICE_DELIVER 外卖服务； BIZ_SERVICE_FREE_PARK 停车位； BIZ_SERVICE_WITH_PET 可带宠物； BIZ_SERVICE_FREE_WIFI 免费wifi， 可多选
    pub business_service: Option<Vec<String>>,
    /// 使用时段限制
    pub time_limit: Option<Vec<TimeLimit>>,
    /// 是否可以分享朋友
    pub share_friends: Option<bool>,
}
/// 使用时段限制
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeLimit {
    /// 限制类型枚举值,支持填入 MONDAY 周一 TUESDAY 周二 WEDNESDAY 周三 THURSDAY 周四 FRIDAY 周五 SATURDAY 周六 SUNDAY 周日 此处只控制显示， 不控制实际使用逻辑，不填默认不显示
    #[serde(rename="type")]
    pub r#type: Option<Abstracts>,
    /// 起始时间（小时）,当前type类型下的起始时间（小时） ，如当前结构体内填写了MONDAY， 此处填写了10，则此处表示周一 10:00可用
    pub begin_hour: Option<i64>,
    /// 起始时间（分钟）,如当前结构体内填写了MONDAY， begin_hour填写10，此处填写了59， 则此处表示周一 10:59可用
    pub begin_minute: Option<i64>,
    /// 结束时间（小时）,如当前结构体内填写了MONDAY， 此处填写了20， 则此处表示周一 10:00-20:00可用
    pub end_hour: Option<i64>,
    /// 结束时间（分钟）,如当前结构体内填写了MONDAY， begin_hour填写10，此处填写了59， 则此处表示周一 10:59-00:59可用
    pub end_minute: Option<i64>,
}
/// 图文列表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextImageList {
    /// 图片链接,必须调用 上传图片接口 上传图片获得链接，并在此填入， 否则报错
    pub image_url: Option<String>,
    /// 图文描述
    pub text: Option<String>,
}
/// 封面摘要
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Abstracts {
    /// 指定可用的商品类目,仅用于代金券类型 ，填入后将在券面拼写适用于xxx
    #[serde(rename="abstract")]
    pub abstracts: Option<String>,
    /// 封面图片列表.
    /// 仅支持填入一 个封面图片链接， 上传图片接口 上传获取图片获得链接，填写 非CDN链接会报错，并在此填入。 建议图片尺寸像素850*350
    pub icon_url_list: Option<String>,
}
/// 使用门槛
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseCondition {
    /// 指定可用的商品类目,仅用于代金券类型 ，填入后将在券面拼写适用于xxx
    pub accept_category: Option<String>,
    /// 指定不可用的商品类目,仅用于代金券类型 ，填入后将在券面拼写不适用于xxxx
    pub reject_category: Option<String>,
    /// 满减门槛字段,可用于兑换券和代金券 ，填入后将在全面拼写消费满xx元可用
    pub least_cost: Option<i64>,
    /// 购买xx可用类型门槛,仅用于兑换 ，填入后自动拼写购买xxx可用
    pub object_use_for: Option<String>,
    /// 不可以与其他类型共享门槛,填写false时系统将在使用须知里 拼写“不可与其他优惠共享”， 填写true时系统将在使用须知里 拼写“可与其他优惠共享”， 默认为true
    pub can_use_with_other_discount: Option<bool>,
}
/// 积分规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BonusRule {
    /// 消费金额,以分为单位.
    pub cost_money_unit: i64,
    /// 对应增加的积分
    pub increase_bonus: i64,
    /// 用户单次可获取的积分上限
    pub max_increase_bonus: i64,
    /// 初始设置积分
    pub init_increase_bonus: i64,
    /// 每使用积分
    pub cost_bonus_unit: i64,
    /// 抵扣xx元,这里以分为单位）.
    pub reduce_money: i64,
    /// 抵扣条件,满xx元（这里以分为单位）可用.
    pub least_money_to_use_bonus: i64,
    /// 抵扣条件,单笔最多使用xx积分.
    pub max_reduce_bonus: i64,
}
/// 自定义会员信息类目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomCell1 {
    /// 入口名称
    pub name: String,
    /// 入口右侧提示语,6个汉字内.
    pub tips: String,
    /// 点击类目跳转外链url
    pub url: String,
    /// 参考https://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1499332673_Unm7V卡券内跳转小程序参数说明：会员卡顶部的信息类目字段，包含以下两个字段
    /// 自定义信息类目小程序user_name，格式为原始id+@app
    pub app_brand_user_name: Option<String>,
    /// 自定义信息类目小程序的页面路径
    pub app_brand_pass: Option<String>,
}
/// 自定义会员信息类目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomField {
    /// 半自定义名称,当开发者变更这类类目信息的value值时 可以选择触发系统模板消息通知用户。 FIELD_NAME_TYPE_LEVEL 等级 FIELD_NAME_TYPE_COUPON 优惠券 FIELD_NAME_TYPE_STAMP 印花 FIELD_NAME_TYPE_DISCOUNT 折扣 FIELD_NAME_TYPE_ACHIEVEMEN 成就 FIELD_NAME_TYPE_MILEAGE 里程 FIELD_NAME_TYPE_SET_POINTS 集点 FIELD_NAME_TYPE_TIMS 次数
    pub name_type: String,
    /// 自定义名称,当开发者变更这类类目信息的value值时 不会触发系统模板消息通知用户
    pub name: String,
    /// 点击类目跳转外链url
    pub url: String,
    /// 参考https://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1499332673_Unm7V卡券内跳转小程序参数说明：会员卡顶部的信息类目字段，包含以下两个字段
    /// 自定义信息类目小程序user_name，格式为原始id+@app
    pub app_brand_user_name: Option<String>,
    /// 自定义信息类目小程序的页面路径
    pub app_brand_pass: Option<String>,
}
/// 支付功能
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayInfo {
    /// 刷卡功能
    pub swipe_card: SwipeCard,
}
/// 刷卡功能
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwipeCard {
    /// 是否设置该会员卡支持拉出微信支付刷卡界面
    pub is_swipe_card: bool,
}
/// 商品信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberCardSkuInfo {
    /// 卡券库存的数量，不支持填写0，上限为100000000
    pub quantity: i32,
    /// 卡券全部库存的数量，上限为100000000。
    /// https://developers.weixin.qq.com/doc/offiaccount/Cards_and_Offer/Managing_Coupons_Vouchers_and_Cards.html#4
    pub total_quantity: i32,
}
/// 会员卡颜色
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq,Serialize, Deserialize)]
pub enum CardColor {
    Color010,
    Color020,
    Color030,
    Color040,
    Color050,
    Color060,
    Color070,
    Color080,
    Color081,
    Color082,
    Color090,
    Color100,
    Color101,
    Color102,
    Unknow,
}

impl CardColor {
    fn from_str(color: &str) -> Self {
        match color {
            "#63b359" => Self::Color010,
            "#2c9f67" => Self::Color020,
            "#509fc9" => Self::Color030,
            "#5885cf" => Self::Color040,
            "#9062c0" => Self::Color050,
            "#d09a45" => Self::Color060,
            "#e4b138" => Self::Color070,
            "#ee903c" => Self::Color080,
            "#f08500" => Self::Color081,
            "#a9d92d" => Self::Color082,
            "#dd6549" => Self::Color090,
            "#cc463d" => Self::Color100,
            "#cf3e36" => Self::Color101,
            "#5E6671" => Self::Color102,
            _ => {
                Self::Unknow
            }
        }
    }
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq,Serialize, Deserialize)]
pub enum DateInfoType {
    /// 永久有效类型
    DATE_TYPE_PERMANENT,
    /// 固定日期
    DATE_TYPE_FIX_TIME_RANGE,
    /// 固定时长
    DATE_TYPE_FIX_TERM,
    Unknow,
}

impl DateInfoType {
    fn from_str(v: &str) -> Self {
        match v {
            "DATE_TYPE_PERMANENT" => Self::DATE_TYPE_PERMANENT,
            "DATE_TYPE_FIX_TIME_RANGE" => Self::DATE_TYPE_FIX_TIME_RANGE,
            "DATE_TYPE_FIX_TERM" => Self::DATE_TYPE_FIX_TERM,
            _ => {
                Self::Unknow
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq,Serialize, Deserialize)]
pub enum BusinessServiceType {
    /// 外卖服务
    BIZ_SERVICE_DELIVER,
    /// 停车位
    BIZ_SERVICE_FREE_PARK,
    /// 可带宠物
    BIZ_SERVICE_WITH_PET,
    /// WIFI
    BIZ_SERVICE_FREE_WIFI,
    Unknow,
}

impl BusinessServiceType {
    fn from_str(v: &str) -> Self {
        match v {
            "BIZ_SERVICE_DELIVER" => Self::BIZ_SERVICE_DELIVER,
            "BIZ_SERVICE_FREE_PARK" => Self::BIZ_SERVICE_FREE_PARK,
            "BIZ_SERVICE_WITH_PET" => Self::BIZ_SERVICE_WITH_PET,
            "BIZ_SERVICE_FREE_WIFI" => Self::BIZ_SERVICE_FREE_WIFI,
            _ => {
                Self::Unknow
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq,Serialize, Deserialize)]
pub enum CardRichFieldType {
    /// 自定义单选
    FORM_FIELD_RADIO,
    /// 自定义选择项
    FORM_FIELD_SELECT,
    /// 自定义多选
    FORM_FIELD_CHECK_BOX,
    Unknow,
}

#[allow(unused)]
impl CardRichFieldType {
    fn from_str(v: &str) -> Self {
        match v {
            "FORM_FIELD_RADIO" => Self::FORM_FIELD_RADIO,
            "FORM_FIELD_SELECT" => Self::FORM_FIELD_SELECT,
            "FORM_FIELD_CHECK_BOX" => Self::FORM_FIELD_CHECK_BOX,
            _ => {
                Self::Unknow
            }
        }
    }
}

/// 使用日期，有效期的信息.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateInfo {
    /// 使用时间的类型.
    /// 支持固定时长有效类型 固定日期有效类型 永久有效类型：DATE_TYPE_FIX_TERM_RANGE、DATE_TYPE_FIX_TERM 、DATE_TYPE_PERMANENT
    #[serde(rename="type")]
    pub r#type: String,
    /// 起用时间.
    /// type为DATE_TYPE_FIX_TIME_RANGE时专用， 表示起用时间。从1970年1月1日00:00:00至起用时间的秒数 （ 东八区时间,UTC+8，单位为秒 ）
    pub begin_timestamp: Option<i64>,
    /// 结束时间.
    /// type为DATE_TYPE_FIX_TERM_RANGE时专用，表示结束时间 （ 东八区时间,UTC+8，单位为秒 ）
    pub end_timestamp: Option<i64>,
    /// 自领取后多少天开始生效.
    /// type为DATE_TYPE_FIX_TERM时专用，表示自领取后多少天开始生效。（单位为天）
    pub fixed_term: Option<i64>,
    /// 自领取后多少天开始生效.
    /// type为DATE_TYPE_FIX_TERM时专用，表示自领取后多少天开始生效。（单位为天）
    pub fixed_begin_term: Option<i64>,
}

/// 使用日期，有效期的信息.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpCardCreateResponse {
    pub card_id: String,
}

/// 会员卡激活接口的参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpMemberCardActivateRequest {
    /// 会员卡编号，由开发者填入，作为序列号显示在用户的卡包里。可与Code码保持等值。
    pub membership_number: String,
    /// 领取会员卡用户获得的code
    pub code: String,
    /// 卡券ID,自定义code卡券必填
    pub card_id: Option<String>,
    /// 商家自定义会员卡背景图，须先调用上传图片接口将背景图上传至CDN，否则报错。卡面设计请遵循微信会员卡自定义背景设计规范
    pub background_pic_url: Option<String>,
    /// 激活后的有效起始时间。若不填写默认以创建时的 data_info 为准。Unix时间戳格式。
    pub activate_begin_time: Option<i64>,
    /// 激活后的有效截至时间。若不填写默认以创建时的 data_info 为准。Unix时间戳格式。
    pub activate_end_time: Option<i64>,
    /// 初始积分，不填为0。
    pub init_bonus: Option<i64>,
    /// 积分同步说明
    pub init_bonus_record: Option<String>,
    /// 初始余额，不填为0。
    pub init_balance: Option<f64>,
    /// 创建时字段custom_field1定义类型的初始值，限制为4个汉字，12字节。
    pub init_custom_field_value1: Option<String>,
    /// 创建时字段custom_field2定义类型的初始值，限制为4个汉字，12字节。
    pub init_custom_field_value2: Option<String>,
    /// 创建时字段custom_field3定义类型的初始值，限制为4个汉字，12字节。
    pub init_custom_field_value3: Option<String>,
}

/// 拉取会员信息返回的结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpMemberCardUserInfoResponse {
    /// 用户在本公众号内唯一识别码
    pub openid: String,
    /// 用户昵称
    pub nickname: Option<String>,
    /// 积分信息
    pub bonus: Option<i64>,
    /// 余额信息
    pub balance: Option<f64>,
    /// 用户性别
    pub sex: Option<String>,
    /// 会员信息
    pub user_info: Option<MemberCardUserInfo>,
    /// 当前用户会员卡状态，NORMAL 正常 EXPIRE 已过期 GIFTING 转赠中 GIFT_SUCC 转赠成功 GIFT_TIMEOUT 转赠超时 DELETE 已删除，UNAVAILABLE 已失效
    pub user_card_status: Option<String>,
    pub has_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberCardUserInfo {
    pub custom_field_list: Vec<NameValues>,
    pub common_field_list: Vec<NameValues>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NameValues {
    pub name: Option<String>,
    pub value: Option<String>,
    pub value_list: Option<Vec<String>>,
}



/// <pre>
/// 更新会员信息所需字段消息。
///
/// 1.开发者可以同时传入add_bonus和bonus解决由于同步失败带来的幂等性问题。同时传入add_bonus和bonus时
/// add_bonus作为积分变动消息中的变量值，而bonus作为卡面上的总积分额度显示。余额变动同理。
/// 2.开发者可以传入is_notify_bonus控制特殊的积分对账变动不发送消息，余额变动同理。
/// </pre>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpMemberCardUpdateRequest {
    /// 领取会员卡用户获得的code
    pub code: String,
    /// 卡券ID,自定义code卡券必填
    pub card_id: Option<String>,
    /// 支持商家激活时针对单个会员卡分配自定义的会员卡背景
    pub background_pic_url: Option<String>,
    /// 需要设置的积分全量值，传入的数值会直接显示
    pub bonus: Option<i64>,
    /// 本次积分变动值，传负数代表减少
    pub add_bonus: Option<i64>,
    /// 商家自定义积分消耗记录，不超过14个汉字
    pub record_bonus: Option<String>,
    /// 需要设置的余额全量值，传入的数值会直接显示在卡面
    pub balance: Option<f64>,
    /// 本次余额变动值，传负数代表减少
    pub add_balance: Option<f64>,
    /// 商家自定义金额消耗记录，不超过14个汉字。
    pub record_balance: Option<String>,
    pub notify_optional: Option<NotifyOptional>,
    /// 创建时字段custom_field1定义类型的最新数值，限制为4个汉字，12字节。
    pub custom_field_value1: Option<String>,
    /// 创建时字段custom_field2定义类型的最新数值，限制为4个汉字，12字节。
    pub custom_field_value2: Option<String>,
    /// 创建时字段custom_field3定义类型的最新数值，限制为4个汉字，12字节。
    pub custom_field_value3: Option<String>,
}

/// 控制原生消息结构体，包含各字段的消息控制字段。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyOptional {
    /// 积分变动时是否触发系统模板消息，默认为true
    pub is_notify_bonus: Option<bool>,
    /// 余额变动时是否触发系统模板消息，默认为true
    pub is_notify_balance: Option<bool>,
    /// 自定义group1变动时是否触发系统模板消息，默认为false。（2、3同理）
    pub is_notify_custom_field1: Option<bool>,
    pub is_notify_custom_field2: Option<bool>,
    pub is_notify_custom_field3: Option<bool>,
}


/// 更新会员信息的接口调用后的返回结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpMemberCardUpdateResponse {
    /// 用户在本公众号内唯一识别码
    pub openid: String,
    pub result_bonus: Option<i64>,
    /// 余额信息
    pub result_balance: Option<f64>,
}

/// 会员卡激活，用户字段提交请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpMemberCardActivateUserFormRequest {
    /// 用户在本公众号内唯一识别码
    pub card_id: String,
    pub service_statement: Value,
    pub bind_old_card: Value,
    /// 必填项
    pub required_form: MemberCardUserForm,
    /// 可选项
    pub optional_form: MemberCardUserForm,
}

impl WechatMpMemberCardActivateUserFormRequest {
    /// 绑定老会员卡信息
    pub fn set_bind_old_card(&mut self, name: &str, url: &str) {
        if name.is_empty() || url.is_empty() {
            return;
        }
        self.bind_old_card["name"] = name.into();
        self.bind_old_card["url"] = url.into();
    }
    /// 设置服务声明，用于放置商户会员卡守则
    pub fn set_service_statement(&mut self, name: &str, url: &str) {
        if name.is_empty() || url.is_empty() {
            return;
        }
        self.service_statement["name"] = name.into();
        self.service_statement["url"] = url.into();
    }
}

/// 用户表单对象
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberCardUserForm {

    /// 富文本类型字段列表
    pub rich_field_list: Vec<MemberCardUserFormRichField>,
    /// 文本选项类型列表
    pub custom_field_list: Vec<String>,
    /// 微信格式化的选项类型
    pub common_field_id_list: Vec<String>,
}

/// 富文本字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberCardUserFormRichField {
    /// 富文本类型
    #[serde(rename="type")]
    pub r#type: String,
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivatePluginParam {
    pub encrypt_card_id: String,
    pub outer_str: String,
    pub biz: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberCardUpdateRequest {
    pub card_id: String,
    pub member_card: MemberCardUpdate,
}

/// 会员卡更新对象
/// 以下字段顺序根据微信官方文档顺序相同，不能传入非文档之外的字段
/// https://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1466494654_K9rNz
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberCardUpdate {
    /// 基本信息
    pub base_info: BaseInfoUpdate,
    /// 会员卡背景图
    pub background_pic_url: String,
    /// 是否支持积分，仅支持从false变为true，默认为false
    pub supply_bonus: bool,
    /// 积分清零规则
    pub bonus_cleared: Option<String>,
    /// 积分规则
    pub bonus_rules: Option<String>,
    /// 查看积分外链,设置跳转外链查看积分详情。仅适用于积分无法通过激活接口同步的情况下使用该字段.
    pub bonus_url: Option<String>,
    /// 余额外链,仅适用于余额无法通过激活接口同步的情况下使用该字段.
    pub balance_url: Option<String>,
    /// 支持储值
    pub supply_balance: bool,
    /// 储值规则.
    pub balance_rules: Option<String>,
    /// 特权说明
    pub prerogative: String,
    /// 自动激活
    pub auto_activate: bool,
    /// 是否支持一键激活 ，填true或false.
    pub wx_activate: bool,
    /// 激活会员卡的url.
    pub activate_url: Option<String>,
    /// 自定义会员类目1,会员卡激活后显示.
    pub custom_field1: Option<CustomField>,
    /// 自定义会员类目2
    pub custom_field2: Option<CustomField>,
    /// 自定义会员类目3
    pub custom_field3: Option<CustomField>,
    /// 自定义会员信息类目，会员卡激活后显示.
    pub custom_cell1: Option<CustomCell1>,
    /// 自定义会员信息类目，会员卡激活后显示.
    pub custom_cell2: Option<CustomCell1>,
    /// 自定义会员信息类目，会员卡激活后显示.
    pub custom_cell3: Option<CustomCell1>,
    /// 积分规则,JSON结构积分规则.
    pub bonus_rule: Option<BonusRule>,
    /// 折扣,该会员卡享受的折扣优惠,填10就是九折.
    pub discount: Option<i64>,
}

/// 微信会员卡基本信息更新
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseInfoUpdate {
    /// 卡券名,字数上限为9个汉字 (建议涵盖卡券属性、服务及金额).
    pub title: String,
    /// 卡券的商户logo,建议像素为300*300.
    pub logo_url: String,
    /// 卡券使用提醒,字数上限为16个汉字.
    pub notice: String,
    /// 卡券使用说明,字数上限为1024个汉字.
    pub description: String,
    /// 客服电话
    pub service_phone: Option<String>,
    /// 券颜色,按色彩规范标注填写Color010-Color100.
    pub color: String,
    /// 门店位置ID,调用 POI门店管理接口 获取门店位置ID.
    pub location_id_list: Option<Vec<String>>,
    /// 会员卡是否支持全部门店,填写后商户门店更新时会自动同步至卡券.
    pub use_all_locations: Option<bool>,
    /// 卡券中部居中的按钮,仅在卡券激活后且可用状态 时显示.
    pub center_title: Option<String>,
    /// 显示在入口下方的提示语,仅在卡券激活后且可用状态时显示.
    pub center_sub_title: Option<String>,
    /// 顶部居中的url,仅在卡券激活后且可用状态时显示.
    pub center_url: Option<String>,
    /// 自定义跳转外链的入口名字
    pub custom_url_name: Option<String>,
    /// 自定义跳转的URL
    pub custom_url: Option<String>,
    /// 显示在入口右侧的提示语
    pub custom_url_sub_title: Option<String>,
    /// 营销场景的自定义入口名称
    pub promotion_url_name: Option<String>,
    /// 入口跳转外链的地址链接
    pub promotion_url: Option<String>,
    /// 显示在营销入口右侧的提示语
    pub promotion_url_sub_title: Option<String>,
    /// Code展示类型.
    /// "CODE_TYPE_TEXT" 文本 "CODE_TYPE_BARCODE" 一维码 "CODE_TYPE_QRCODE" 二维码 "CODE_TYPE_ONLY_QRCODE" 仅显示二维码 "CODE_TYPE_ONLY_BARCODE" 仅显示一维码 "CODE_TYPE_NONE" 不显示任何码型
    pub code_type: String,
    pub pay_info: Option<PayInfo>,
    /// 是否设置该会员卡中部的按钮同时支持微信支付刷卡和会员卡二维码
    pub is_pay_and_qrcode: Option<bool>,
    /// 每人可领券的数量限制,建议会员卡每人限领一张.
    pub get_limit: Option<i32>,
    /// 卡券领取页面是否可分享,默认为true.
    pub can_share: Option<bool>,
    /// 卡券是否可转赠,默认为true.
    pub can_give_friend: Option<bool>,
    /// 使用日期,有效期的信息.
    pub date_info: DateInfo,
    /// 微信小程序开放功能 小程序&卡券打通部分新增8个字段 https://mp.weixin.qq.com/cgi-bin/announce?action=getannouncement&key=1490190158&version=1&lang=zh_CN&platform=2
    /// 自定义使用入口跳转小程序的user_name，格式为原始id+@app
    pub custom_app_brand_user_name: Option<String>,
    /// 自定义使用入口小程序页面地址
    pub custom_app_brand_pass: Option<String>,
    /// 小程序的user_name
    pub center_app_brand_user_name: Option<String>,
    /// 自定义居中使用入口小程序页面地址
    pub center_app_brand_pass: Option<String>,
    /// 小程序的user_name
    pub promotion_app_brand_user_name: Option<String>,
    /// 自定义营销入口小程序页面地址
    pub promotion_app_brand_pass: Option<String>,
    /// 小程序的user_name
    pub activate_app_brand_user_name: Option<String>,
    /// 激活小程序页面地址
    pub activate_app_brand_pass: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpMemberCardActivateTempInfoResponse {
    pub user_info: MemberCardUserInfo,
}