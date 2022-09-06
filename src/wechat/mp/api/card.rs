use std::vec;

use serde::{Serialize, Deserialize, Serializer};
use serde_json::{json, Value};

use crate::{session::SessionStore, request::{RequestType}, WechatCommonResponse, WechatMpClient, LabradorResult, LabraError, get_timestamp, TicketType, get_nonce_str, WechatCrypto, BaseInfo, AdvancedInfo};
use crate::wechat::mp::constants::{QR_CODE};
use crate::wechat::mp::method::{MpCardMethod, WechatMpMethod};

/// 卡券相关.
#[derive(Debug, Clone)]
pub struct WechatMpCard<'a, T: SessionStore> {
    client: &'a WechatMpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WechatMpCard<'a, T> {

    #[inline]
    pub fn new(client: &WechatMpClient<T>) -> WechatMpCard<T> {
        WechatMpCard {
            client,
        }
    }

    /// <pre>
    /// 获得卡券api_ticket，不强制刷新卡券api_ticket.
    /// </pre>
    pub async fn get_card_api_ticket(&self) -> LabradorResult<String> {
        self.get_card_api_ticket_force(false).await
    }

    /// <pre>
    /// 获得卡券api_ticket.
    /// 获得时会检查卡券apiToken是否过期，如果过期了，那么就刷新一下，否则就什么都不干
    /// </pre>
    pub async fn get_card_api_ticket_force(&self, force_refresh: bool) -> LabradorResult<String> {
       self.client.get_ticket_force(TicketType::WxCard, force_refresh).await
    }

    /// <pre>
    /// 创建调用卡券api时所需要的签名
    /// </pre>
    pub async fn create_card_api_signature(&self, mut params: Vec<String>) -> LabradorResult<WechatMpCardApiSignature> {
        let timestamp = get_timestamp() / 1000;
        let noncestr = get_nonce_str();
        let api_ticket = self.get_card_api_ticket_force(false).await?;
        params.push(timestamp.to_string());
        params.push(noncestr.to_string());
        params.push(api_ticket);
        params.sort();
        let signature = WechatCrypto::get_sha1_sign(&params.join(""));
        Ok(WechatMpCardApiSignature{
            app_id: self.client.appid.to_string(),
            card_id: "".to_string(),
            card_type: "".to_string(),
            location_id: "".to_string(),
            code: "".to_string(),
            openid: "".to_string(),
            nonce_str: noncestr,
            signature,
            timestamp,
        })
    }

    /// <pre>
    /// 卡券Code解码
    /// </pre>
    pub async fn decrypt_card_code(&self, encrypt_code: &str) -> LabradorResult<String> {
        let req = json!({
            "encrypt_code": encrypt_code
        });
        let v = self.client.post(WechatMpMethod::Card(MpCardMethod::CodeDecrypt), vec![], req, RequestType::Json).await?.json::<Value>()?;
        let v=  WechatCommonResponse::parse::<Value>(v)?;
        let code = v["code"].as_str().unwrap_or_default();
        Ok(code.to_string())
    }

    /// <pre>
    /// 卡券Code查询.
    /// 文档地址： <a href="https://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1451025272&anchor=1">文档</a>
    /// </pre>
    pub async fn query_card_code(&self, card_id: &str, code: &str, check_consume: bool) -> LabradorResult<WechatMpCardResponse> {
        let req = json!({
            "card_id": card_id,
            "code": code,
            "check_consume": check_consume,
        });
        let v = self.client.post(WechatMpMethod::Card(MpCardMethod::CodeGet), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatMpCardResponse>(v)
    }

    /// <pre>
    /// 卡券Code核销。核销失败会抛出异常
    /// </pre>
    pub async fn consume_card_code(&self, code: &str) -> LabradorResult<WechatMpCardCodeConsumeResponse> {
        self.consume_card_code_with_cardid(None, code).await
    }

    /// <pre>
    /// 卡券Code核销。核销失败会抛出异常
    /// </pre>
    pub async fn consume_card_code_with_cardid(&self, card_id: Option<&str>, code: &str) -> LabradorResult<WechatMpCardCodeConsumeResponse> {
        let req = json!({
            "card_id": card_id,
            "code": code,
        });
        let v = self.client.post(WechatMpMethod::Card(MpCardMethod::CodeConsume), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatMpCardCodeConsumeResponse>(v)
    }

    /// <pre>
    /// 卡券Mark接口.
    /// 开发者在帮助消费者核销卡券之前，必须帮助先将此code（卡券串码）与一个openid绑定（即mark住），
    /// 才能进一步调用核销接口，否则报错。
    /// </pre>
    pub async fn mark_card_code(&self, code: &str, card_id: &str, openid: &str, is_mark: bool) -> LabradorResult<WechatCommonResponse> {
        let req = json!({
            "card_id": card_id,
            "code": code,
            "openid": openid,
            "is_mark": is_mark,
        });
       self.client.post(WechatMpMethod::Card(MpCardMethod::CodeMark), vec![], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 查看卡券详情接口.
    /// 详见 <a href="https://mp.weixin.qq.com/wiki/14/8dd77aeaee85f922db5f8aa6386d385e.html#.E6.9F.A5.E7.9C.8B.E5.8D.A1.E5.88.B8.E8.AF.A6.E6.83.85">文档</a>
    /// </pre>
    pub async fn get_card_detail(&self, card_id: &str) -> LabradorResult<Value> {
        let req = json!({
            "card_id": card_id,
        });
        let v= self.client.post(WechatMpMethod::Card(MpCardMethod::Get), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<Value>(v)
    }

    /// <pre>
    /// 添加测试白名单
    /// </pre>
    pub async fn add_test_white_list(&self, openid: &str) -> LabradorResult<WechatCommonResponse> {
        let req = json!({
            "openid": vec![openid],
        });
        self.client.post(WechatMpMethod::Card(MpCardMethod::SetWhiteList), vec![], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 创建卡券
    /// </pre>
    pub async fn create_card(&self, req: WechatMpCardCreateRequest) -> LabradorResult<WechatMpCardCreateResponse> {
        let v = self.client.post(WechatMpMethod::Card(MpCardMethod::Create), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatMpCardCreateResponse>(v)
    }

    /// <pre>
    /// 创建卡券二维码
    /// </pre>
    pub async fn create_qrcode_card(&self, card_id: &str, outer_str: &str) -> LabradorResult<WechatMpCardQrcodeCreateResponse> {
        self.create_qrcode_card_expire(card_id, outer_str, 0).await
    }

    /// <pre>
    /// 创建卡券二维码
    /// </pre>
    pub async fn create_qrcode_card_expire(&self, card_id: &str, outer_str: &str, expires_in: i64) -> LabradorResult<WechatMpCardQrcodeCreateResponse> {
        self.create_qrcode_card_complex(card_id, outer_str, expires_in, None, None, false).await
    }

    /// <pre>
    /// 创建卡券二维码
    /// </pre>
    pub async fn create_qrcode_card_complex(&self, card_id: &str, outer_str: &str, expires_in: i64, openid: Option<&str>, code: Option<&str>, is_unique_code: bool) -> LabradorResult<WechatMpCardQrcodeCreateResponse> {
        let mut req = json!({
           "action_name" : QR_CODE,
        });
        if expires_in > 0 {
            req["expire_seconds"] = expires_in.into();
        }
        let action_info = json!({
            "card": {
                "openid": openid,
                "code": code,
                "is_unique_code": is_unique_code,
                "card_id": card_id,
                "outer_str": outer_str,
            }
        });
        req["action_info"] = action_info;
        let v = self.client.post(WechatMpMethod::Card(MpCardMethod::CreateQrcode), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatMpCardQrcodeCreateResponse>(v)
    }

    /// <pre>
    /// 创建卡券货架
    /// </pre>
    pub async fn create_landing_page(&self, req: WechatMpCardLandingPageCreateRequest) -> LabradorResult<WechatMpCardLandingPageCreateResponse> {
        let v = self.client.post(WechatMpMethod::Card(MpCardMethod::CreateLandingpage), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatMpCardLandingPageCreateResponse>(v)
    }

    /// <pre>
    /// 将用户的卡券设置为失效状态.
    /// 详见:<a href="https://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1451025272&anchor=9">文档</a>
    /// </pre>
    pub async fn unavailable_card_code(&self, card_id: &str, code: &str, reason: &str) -> LabradorResult<WechatCommonResponse> {
        let req = json!({
           "card_id": card_id,
            "code": code,
            "reason": reason
        });
        self.client.post(WechatMpMethod::Card(MpCardMethod::UnavailabeCode), vec![], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 删除卡券接口
    /// </pre>
    pub async fn delete_card(&self, card_id: &str) -> LabradorResult<WechatCommonResponse> {
        let req = json!({
           "card_id": card_id,
        });
        self.client.post(WechatMpMethod::Card(MpCardMethod::Delete), vec![], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 导入自定义code(仅对自定义code商户)
    /// </pre>
    pub async fn card_code_deposit(&self, card_id: &str, code_list: Vec<&str>) -> LabradorResult<WechatMpCardCodeDepositResponse> {
        if code_list.len() == 0 || code_list.len() > 100 {
            return Err(LabraError::RequestError("code数量为0或者code数量超过100个".to_string()))
        }
        let req = json!({
           "card_id": card_id,
           "code": code_list,
        });
        let v = self.client.post(WechatMpMethod::Card(MpCardMethod::CodeDeposit), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatMpCardCodeDepositResponse>(v)
    }

    /// <pre>
    /// 查询导入code数目接口
    /// </pre>
    pub async fn card_code_deposit_count(&self, card_id: &str) -> LabradorResult<WechatMpCardCodeDepositCountResponse> {
        let req = json!({
           "card_id": card_id,
        });
        let v = self.client.post(WechatMpMethod::Card(MpCardMethod::GetDepositCount), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatMpCardCodeDepositCountResponse>(v)
    }

    /// <pre>
    /// 核查code接口
    /// </pre>
    pub async fn card_code_check_code(&self, card_id: &str, code_list: Vec<&str>) -> LabradorResult<WechatMpCardCheckCodeResponse> {
        if code_list.len() == 0 || code_list.len() > 100 {
            return Err(LabraError::RequestError("code数量为0或者code数量超过100个".to_string()))
        }
        let req = json!({
           "card_id": card_id,
           "code": code_list,
        });
        let v = self.client.post(WechatMpMethod::Card(MpCardMethod::CheckCode), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatMpCardCheckCodeResponse>(v)
    }

    /// <pre>
    /// 图文消息群发卡券获取内嵌html
    /// </pre>
    pub async fn card_mpnews_get_html(&self, card_id: &str) -> LabradorResult<WechatMpCardMpnewsGethtmlResponse> {
        let req = json!({
           "card_id": card_id,
        });
        let v = self.client.post(WechatMpMethod::Card(MpCardMethod::GetHtml), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatMpCardMpnewsGethtmlResponse>(v)
    }

    /// <pre>
    /// 修改库存接口
    /// <a href="https://developers.weixin.qq.com/doc/offiaccount/Cards_and_Offer/Managing_Coupons_Vouchers_and_Cards.html#5">文档</a>
    /// </pre>
    pub async fn card_modify_stock(&self, card_id: &str, change_value: i64) -> LabradorResult<WechatCommonResponse> {
        let mut req = json!({
           "card_id": card_id,
        });
        if change_value > 0 {
            req["increase_stock_value"] = change_value.into();
        } else {
            req["reduce_stock_value"] = change_value.abs().into();
        }
        self.client.post(WechatMpMethod::Card(MpCardMethod::ModifyStock), vec![], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 更改Code接口
    /// <a href="https://developers.weixin.qq.com/doc/offiaccount/Cards_and_Offer/Managing_Coupons_Vouchers_and_Cards.html#6">文档</a>
    /// </pre>
    pub async fn card_code_update(&self, card_id: &str, old_code: &str, new_code: &str) -> LabradorResult<WechatCommonResponse> {
        let mut req = json!({
           "card_id": card_id,
           "code": old_code,
           "new_code": new_code,
        });
        self.client.post(WechatMpMethod::Card(MpCardMethod::UpdateCode), vec![], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 设置买单接口
    /// <a href="https://developers.weixin.qq.com/doc/offiaccount/Cards_and_Offer/Create_a_Coupon_Voucher_or_Card.html#12">文档</a>
    /// </pre>
    pub async fn card_paycell_set(&self, card_id: &str, is_open: bool) -> LabradorResult<WechatCommonResponse> {
        let mut req = json!({
           "card_id": card_id,
           "is_open": is_open,
        });
        self.client.post(WechatMpMethod::Card(MpCardMethod::SetPayCell), vec![], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 设置自助核销
    /// <a href="https://developers.weixin.qq.com/doc/offiaccount/Cards_and_Offer/Create_a_Coupon_Voucher_or_Card.html#14">文档</a>
    /// </pre>
    pub async fn card_self_consume_cell_set(&self, card_id: &str, is_open: bool, need_verify_cod: bool, need_remark_amount: bool) -> LabradorResult<WechatCommonResponse> {
        let mut req = json!({
           "card_id": card_id,
           "is_open": is_open,
           "need_verify_cod": need_verify_cod,
           "need_remark_amount": need_remark_amount,
        });
        self.client.post(WechatMpMethod::Card(MpCardMethod::SetSelfConsumerCell), vec![], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 获取用户已领取卡券接口
    /// <a href="https://developers.weixin.qq.com/doc/offiaccount/Cards_and_Offer/Managing_Coupons_Vouchers_and_Cards.html#1">文档</a>
    /// </pre>
    pub async fn get_user_card_list(&self, card_id: &str, openid: &str) -> LabradorResult<WechatUserCardListResponse> {
        let mut req = json!({
           "card_id": card_id,
           "openid": openid,
        });
        let v = self.client.post(WechatMpMethod::Card(MpCardMethod::GetUserCardList), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatUserCardListResponse>(v)
    }

    /// 为了降低商户接入卡券的难度，微信公众平台向所有已具备卡券功能的公众号开放“第三方代制”功能。申请并开通此功能后，具备开发能力的开发者，可通过 API 接口协助无公众号的商户快速接入并使用卡券。协助制券的开发者称为“母商户”，被协助制券的商户称为“子商户”。
    ///
    /// 母商户需将旗下子商户资料提前上传报备，通过审核方可生效。在制券过程中允许母商户从报备的子商户列表中，选择一个子商户协助制券。
    ///
    /// 开通步骤
    ///
    /// 第一步，申请路径：微信公众平台 - 卡券功能 - 右上角 - 商户信息 - 第三方代制模式。
    ///
    /// 第二步，商户通过微信公众平台或 API 接口，提交子商户资料、资质，审核通过后可使用该子商户信息制券。
    ///
    /// 第三步，调用 API 接口创建卡券时，需传入该模式的特有字段，具体字段参考创建子商户接口的返回字段说明。该模式下，仅创建卡券接口有变动，其余接口和卡券整体接口的使用保持不变。详情参考首页
    /// <pre>
    /// 创建子商户接口
    /// <a href="https://developers.weixin.qq.com/doc/offiaccount/Cards_and_Offer/Third-party_developer_mode.html#_1-1-%E5%88%9B%E5%BB%BA%E5%AD%90%E5%95%86%E6%88%B7%E6%8E%A5%E5%8F%A3">文档</a>
    /// 支持母商户调用该接口传入子商户的相关资料，并获取子商户ID，用于子商户的卡券功能管理。 子商户的资质包括：商户名称、商户logo（图片）、卡券类目、授权函（扫描件或彩照）、授权函有效期截止时间。
    /// 备注：授权函请在 (《第三方代制模式指引文档》)[https://mp.weixin.qq.com/cgi-bin/announce?action=getannouncement&key=1459357007&version=1&lang=zh_CN&platform=2]内下载，手填并加盖鲜章后，上传彩色扫描件或彩照。
    ///
    /// 1、授权函必须加盖企业公章，或个体户店铺章、发票专用章、财务章、合同章等具备法律效力的盖章，不可使用个人私章；
    ///
    /// 2、若子商户是个体工商户，且无上述公章，授权函可用个体工商户经营者签字代替公章，且须同时额外上传《个体工商户营业执照》及该执照内登记的经营者的身份证彩照。（本方案仅适用于子商户是个体工商户，且无公章的场景。其他场景必须在授权函加盖公章）
    ///
    /// 3、子商户若有公众号，且不愿意自己运营，通过授权方式让第三方代制，支持配置子商户公众号。配置后，1）该子商户的制券配额不再限制，2）该卡券详情页关联的公众号为子商户配置这个公众号。
    /// </pre>
    pub async fn create_submerchant(&self, req: WechatMpCardCreateSubmerchantRequest) -> LabradorResult<WechatMpCardSubmerchantResponse> {
        let v = self.client.post(WechatMpMethod::Card(MpCardMethod::SubmitSubmerchant), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatMpCardSubmerchantResponse>(v)
    }

    /// <pre>
    /// 卡券开放类目查询接口
    /// <a href="https://developers.weixin.qq.com/doc/offiaccount/Cards_and_Offer/Third-party_developer_mode.html#_1-1-%E5%88%9B%E5%BB%BA%E5%AD%90%E5%95%86%E6%88%B7%E6%8E%A5%E5%8F%A3">文档</a>
    /// 通过调用该接口查询卡券开放的类目ID，类目会随业务发展变更，请每次用接口去查询获取实时卡券类目。
    ///
    /// 注意：
    ///
    /// 1.本接口查询的返回值还有卡券资质 ID ,此处的卡券资质为：已微信认证的公众号通过微信公众平台申请卡券功能时，所需的资质。
    ///
    /// 2.对于第三方开发者代制（无公众号）模式，子商户无论选择什么类目，均暂不需按照此返回提供资质，返回值仅参考类目ID 即可。
    /// </pre>
    pub async fn get_cateogry(&self) -> LabradorResult<WechatMpCardCategoryResponse> {
        let v = self.client.post(WechatMpMethod::Card(MpCardMethod::GetApplyProtocol), vec![], Value::Null, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatMpCardCategoryResponse>(v)
    }



    /// <pre>
    /// 更新子商户接口
    /// <a href="https://developers.weixin.qq.com/doc/offiaccount/Cards_and_Offer/Third-party_developer_mode.html#_1-1-%E5%88%9B%E5%BB%BA%E5%AD%90%E5%95%86%E6%88%B7%E6%8E%A5%E5%8F%A3">文档</a>
    /// 支持调用该接口更新子商户信息。
    /// </pre>
    pub async fn update_submerchant(&self, req: WechatMpCardCreateSubmerchantRequest) -> LabradorResult<WechatMpCardSubmerchantResponse> {
        let v = self.client.post(WechatMpMethod::Card(MpCardMethod::UpdateSubmerchant), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatMpCardSubmerchantResponse>(v)
    }



    /// <pre>
    /// 拉取单个子商户信息接口
    /// <a href="https://developers.weixin.qq.com/doc/offiaccount/Cards_and_Offer/Third-party_developer_mode.html#_1-1-%E5%88%9B%E5%BB%BA%E5%AD%90%E5%95%86%E6%88%B7%E6%8E%A5%E5%8F%A3">文档</a>
    /// 通过指定的子商户appid，拉取该子商户的基础信息。 注意，用母商户去调用接口，但接口内传入的是子商户的appid。
    /// </pre>
    pub async fn get_submerchant(&self, merchant_id: i32) -> LabradorResult<WechatMpCardSubmerchantResponse> {
        let req = json!({
            "merchant_id": merchant_id
        });
        let v = self.client.post(WechatMpMethod::Card(MpCardMethod::GetSubmerchant), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatMpCardSubmerchantResponse>(v)
    }

    /// <pre>
    /// 批量拉取子商户信息接口
    /// <a href="https://developers.weixin.qq.com/doc/offiaccount/Cards_and_Offer/Third-party_developer_mode.html#_1-1-%E5%88%9B%E5%BB%BA%E5%AD%90%E5%95%86%E6%88%B7%E6%8E%A5%E5%8F%A3">文档</a>
    /// 母商户可以通过该接口批量拉取子商户的相关信息，一次调用最多拉取100个子商户的信息，可以通过多次拉去满足不同的查询需求
    /// </pre>
    pub async fn get_submerchant_batch(&self, begin_id: i64, limit: i64, status: &str) -> LabradorResult<WechatMpCardSubmerchantBatchResponse> {
        let req = json!({
          "begin_id": begin_id,
          "limit": limit,
          "status": status
        });
        let v = self.client.post(WechatMpMethod::Card(MpCardMethod::BatchGetSubmerchant), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatMpCardSubmerchantBatchResponse>(v)
    }
}

//----------------------------------------------------------------------------------------------------------------------------
/// 卡券Api签名
#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct WechatMpCardApiSignature {
    pub app_id: String,
    pub card_id: String,
    pub card_type: String,
    pub location_id: String,
    pub code: String,
    pub openid: String,
    #[serde(rename="nonceStr")]
    pub nonce_str: String,
    pub signature: String,
    pub timestamp: i64,
}

/// 卡券查询Code，核销Code接口返回结果.
#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct WechatMpCardResponse {
    pub card: WechatMpCardInfo,
    pub user_card_status: Option<String>,
    pub can_consume: bool,
    pub out_str: Option<String>,
    pub background_pic_url: Option<String>,
    pub openid: String,
    pub unionid: Option<String>,
}

/// 微信卡券
#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct WechatMpCardInfo {
    pub card_id: String,
    pub begin_time: i64,
    pub end_time: i64,
    pub user_card_status: Option<String>,
    pub membership_number: Option<String>,
    pub code: Option<String>,
    pub bonus: Option<i64>,
}

/// 核销返回
#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct WechatMpCardCodeConsumeResponse {
    pub card: CodeConsumeCard,
    pub openid: String,
}
#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct CodeConsumeCard {
    pub card_id: String,
}


#[allow(unused)]
pub struct WechatMpCardCreateRequest {
    pub card: AbstractCardCreateRequest,
}

#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct GrouponCardCreateRequest {
    pub card_type: String,
    pub groupon: GrouponCard,
}

#[derive(Serialize, Deserialize)]
pub struct GrouponCard {
    /// 团购券专用，团购详情
    pub deal_detail: String,
    /// 基本信息
    pub base_info: BaseInfo,
    /// 基本信息
    pub advanced_info: AdvancedInfo,
}
#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct GiftCardCreateRequest {
    pub card_type: String,
    pub gift: GiftCard,
}

#[derive(Serialize, Deserialize)]
pub struct GiftCard {
    /// 兑换券专用，填写兑换内容的名称。
    pub gift: String,
    /// 基本信息
    pub base_info: BaseInfo,
    /// 基本信息
    pub advanced_info: AdvancedInfo,
}

#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct GeneralCouponCreateRequest {
    pub card_type: String,
    pub general_coupon: GeneralCoupon,
}

#[derive(Serialize, Deserialize)]
pub struct GeneralCoupon {
    /// 兑换券专用，填写兑换内容的名称。
    pub default_detail: String,
    /// 基本信息
    pub base_info: BaseInfo,
    /// 基本信息
    pub advanced_info: AdvancedInfo,
}

#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct DiscountCardCreateRequest {
    pub card_type: String,
    pub discount: DiscountCard,
}

#[derive(Serialize, Deserialize)]
pub struct DiscountCard {
    /// 折扣券专用，表示打折额度（百分比）。填30就是七折。
    pub discount: i32,
    /// 基本信息
    pub base_info: BaseInfo,
    /// 基本信息
    pub advanced_info: AdvancedInfo,
}

#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct CashCardCreateRequest {
    pub card_type: String,
    pub cash: CashCard,
}

#[derive(Serialize, Deserialize)]
pub struct CashCard {
    /// 代金券专用，表示起用金额（单位为分）,如果无起用门槛则填0
    pub least_cost: i32,
    /// 代金券专用，表示减免金额。（单位为分）
    pub reduce_cost: i32,
    /// 基本信息
    pub base_info: BaseInfo,
    /// 基本信息
    pub advanced_info: AdvancedInfo,
}

pub enum AbstractCardCreateRequest {
    Cash(CashCardCreateRequest),
    Discount(DiscountCardCreateRequest),
    GeneralCoupon(GeneralCouponCreateRequest),
    Gift(GiftCardCreateRequest),
    Groupon(GrouponCardCreateRequest),
}

impl Serialize for WechatMpCardCreateRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match &self.card {
            AbstractCardCreateRequest::Cash(v) => v.serialize(serializer),
            AbstractCardCreateRequest::Discount(v) => v.serialize(serializer),
            AbstractCardCreateRequest::GeneralCoupon(v) => v.serialize(serializer),
            AbstractCardCreateRequest::Gift(v) => v.serialize(serializer),
            AbstractCardCreateRequest::Groupon(v) => v.serialize(serializer),
        }
    }
}

#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct WechatMpCardCreateResponse {
    pub card_id: String,
}

#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct WechatMpCardQrcodeCreateResponse {
    pub ticket: String,
    pub url: Option<String>,
    pub show_qrcode_url: Option<String>,
    pub expire_seconds: i64,
}
#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct WechatMpCardLandingPageCreateRequest {
    /// 页面的banner图片链接，须调用，建议尺寸为640*300。
    pub banner: String,
    /// 页面的title
    pub page_title: String,
    /// 投放页面的场景值；
    /// SCENE_NEAR_BY 附近
    /// SCENE_MENU 自定义菜单
    /// SCENE_QRCODE 二维码
    /// SCENE_ARTICLE 公众号文章
    /// SCENE_H5 h5页面
    /// SCENE_IVR 自动回复
    /// SCENE_CARD_CUSTOM_CELL 卡券自定义cell
    pub scene: String,
    pub can_share: bool,
    pub card_list: Vec<CardLandingPage>,
}
#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct CardLandingPage {
    pub card_id: String,
    pub thumb_url: String,
}


#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct WechatMpCardLandingPageCreateResponse {
    /// 货架链接
    pub url: String,
    /// 货架ID。货架的唯一标识
    pub page_id: i32,
}


#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct WechatMpCardCodeDepositResponse {
    /// 成功的code
    pub succ_code: Vec<String>,
    /// 重复导入的code
    pub duplicate_code: Vec<String>,
    /// 失败的code
    pub fail_code: Vec<String>,
}

#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct WechatMpCardCheckCodeResponse {
    /// 已经成功存入的code数目
    pub exist_code: Vec<String>,
    pub not_exist_code: Vec<String>,
}


#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct WechatMpCardCodeDepositCountResponse {
    /// 已经成功存入的code数目
    pub count: i32,
}
#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct WechatMpCardMpnewsGethtmlResponse {
    /// 返回一段html代码，可以直接嵌入到图文消息的正文里。即可以把这段代码嵌入到 上传图文消息素材接口 中的content字段里
    pub content: String,
}

/// 用户已领卡券返回
#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct WechatUserCardListResponse {
    /// 卡券列表
    pub card_list: Vec<UserCard>,
    /// 是否有可用的朋友的券
    pub has_share_card: bool,
}
#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct UserCard {
    /// 用户卡券code码
    pub code: String,
    /// 卡券ID
    pub card_id: String,
}

#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct WechatMpCardCreateSubmerchantRequest {
    pub info: CreateSubmerchantRequest,
}

#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct CreateSubmerchantRequest {
    /// 子商户id，一个母商户公众号下唯一
    pub merchant_id: Option<i32>,
    /// 子商户的公众号app_id，配置后子商户卡券券面上的app_id为该app_id。注意：该app_id须经过认证
    pub app_id: Option<String>,
    /// 子商户名称（12个汉字内），该名称将在制券时填入并显示在卡券页面上
    pub brand_name: String,
    /// 子商户logo，可通过 上传图片接口 获取。该 logo 将在制券时填入并显示在卡券页面上
    pub logo_url: String,
    /// 授权函ID，即通过 上传临时素材接口 上传授权函后获得的media_id
    pub protocol: String,
    /// 授权函有效期截止时间（东八区时间，单位为秒），需要与提交的扫描件一致
    pub end_time: i64,
    /// 一级类目 id ,可以通过本文档中接口查询
    pub primary_category_id: u8,
    /// 二级类目id，可以通过本文档中接口查询
    pub secondary_category_id: u8,
    /// 营业执照或个体工商户营业执照彩照或扫描件
    pub agreement_media_id: Option<String>,
    /// 营业执照内登记的经营者身份证彩照或扫描件
    pub operator_media_id: Option<String>,
}

#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct WechatMpCardSubmerchantResponse {
    pub info: SubmerchantResponse,
}

#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct WechatMpCardSubmerchantBatchResponse {
    pub info_list: Vec<SubmerchantResponse>,
}

#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct SubmerchantResponse {
    /// 子商户id，对于一个母商户公众号下唯一
    pub merchant_id: Option<i32>,
    ///子商户若有公众号，且不愿意自己运营，通过授权方式让第三方代制，支持配置子商户公众号。配置后，
    /// 1）该子商户的制券配额不再限制，
    /// 2）该卡券详情页关联的公众号为子商户配置这个公众号。
    pub app_id: Option<String>,
    /// 子商户信息创建时间
    pub create_time: Option<i64>,
    /// 子商户信息更新时间
    pub update_time: Option<i64>,
    /// 子商户名称（12个汉字内），该名称将在制券时填入并显示在卡券页面上
    pub brand_name: String,
    /// 子商户logo，可通过 上传图片接口 获取。该 logo 将在制券时填入并显示在卡券页面上
    pub logo_url: String,
    /// 子商户状态，"CHECKING" 审核中, "APPROVED" , 已通过；"REJECTED"被驳回, "EXPIRED"协议已过期
    pub status: String,
    /// 创建时间（非协议开始时间）
    pub bengin_time: Option<i64>,
    /// 授权函有效期截止时间（东八区时间，单位为秒）
    pub end_time: Option<i64>,
    /// 一级类目 id ,可以通过本文档中接口查询
    pub primary_category_id: Option<u8>,
    /// 二级类目id，可以通过本文档中接口查询
    pub secondary_category_id: Option<u8>,
}

#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct WechatMpCardCategoryResponse {
    /// 子商户id，对于一个母商户公众号下唯一
    pub category: Vec<CardCategory>,
}
#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct CardCategory {
    /// 一级类目 id ,可以通过本文档中接口查询
    pub primary_category_id: Option<u8>,
    /// 分类名称
    pub category_name: Option<String>,
    pub secondary_category: Vec<SecondaryCategory>,
}

#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct SecondaryCategory {
    /// 分类名称
    pub category_name: Option<String>,
    /// 二级类目id，可以通过本文档中接口查询
    pub secondary_category_id: Option<u8>,
    pub can_choose_prepaid_card: Option<u8>,
    pub can_choose_payment_card: Option<u8>,
    pub need_qualification_stuffs: Option<Vec<String>>,
}

