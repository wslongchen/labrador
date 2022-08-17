use chrono::Local;
use crate::{client::{APIClient}, request::{RequestType, Method, LabraRequest, RequestMethod}, errors::LabraError, session::{SimpleStorage, SessionStore}, LabradorResult, RequestParametersHolder, md5};

use std::collections::BTreeMap;
use serde::Serialize;
use crate::taobao::constants::{FORMAT_JSON, SIGN_TYPE_MD5, VERSION_2};
use crate::taobao::request::{TbGetActivityInfoRequest, TbSpreadGetRequest, TbTPwdReportGetRequest};
use crate::taobao::response::{TaobaoResponse, TbGetActivityInfoResponse, TbSpreadGetResponse, TbTPwdReportGetResponse};

use self::{method::TaobaoMethod, request::{TbMaterialSelectRequest, TbJhsSearchRequest, TbItemDetailRequest, TbCouponDetailRequest, TbCreateTPwdRequest, TbMaterialSearchRequest}, response::{TbMaterialSelectResponse, TbJhsSearchResponse, TbItemDetailResponse, TbCreateTPwdResponse, TbMaterialSearchResponse, TbCouponDetailResponse}};

mod request;
mod response;
mod method;
mod constants;

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct TaobaoClient<T: SessionStore> {
    api_client: APIClient<T>,
    /// 格式类型
    format: String,
    /// 签名方法
    sign_method: String,
}



pub trait TaobaoRequest {

    ///
    /// 获取TOP的API名称。
    ///
    /// @return API名称
    fn get_api_method_name(&self) -> TaobaoMethod;

    ///
    /// 获取所有的Key-Value形式的文本请求参数集合。其中：
    /// <ul>
    /// <li>Key: 请求参数名</li>
    /// <li>Value: 请求参数值</li>
    /// </ul>
    ///
    /// @return 文本请求参数集合
    fn get_text_params(&self) -> BTreeMap<String, String> {
        BTreeMap::default()
    }

    ///
    /// 得到当前接口的版本
    ///
    /// @return API版本
    fn get_api_version(&self) -> String {
        VERSION_2.to_string()
    }

    fn get_biz_content(&self) -> String where Self: Serialize + Sized {
        serde_json::to_string(self).unwrap_or_default()
    }

}

/// TaobaoClient
/// 
/// 
/// # Example
/// 
/// ```no_run
/// use labrador::TaobaoClient;
/// async fn main() {
///     let client = TaobaoClient::new("appKey", "secret");
///     // Do Some Thing You Want
///     // ...
/// }
/// ```
/// 
#[allow(unused)]
impl <T: SessionStore> TaobaoClient<T> {

    pub fn new<Q: Into<String>, S: Into<String>>(app_key: Q, secret: S) -> TaobaoClient<SimpleStorage> {
        TaobaoClient {
            api_client: APIClient::<SimpleStorage>::new::<Q, String, S>(app_key, secret, "http://gw.api.taobao.com/router/rest".to_owned()),
            format: FORMAT_JSON.to_string(),
            sign_method: SIGN_TYPE_MD5.to_string()
        }
    }

    pub fn from_session<Q: Into<String>, S: Into<String>>(app_key: Q, secret: S, session: T) -> TaobaoClient<T> {
        TaobaoClient {
            api_client: APIClient::from_session(app_key, secret, String::from("http://gw.api.taobao.com/router/rest"), session),
            format: FORMAT_JSON.to_string(),
            sign_method: SIGN_TYPE_MD5.to_string()
        }
    }

    /// 签名
    fn sign(&self, sign_content: &str) -> LabradorResult<String> {
        match self.sign_method.as_str() {
            constants::SIGN_TYPE_MD5 => {
                let content = format!("{}{}{}", self.api_client.secret.to_string(), sign_content, self.api_client.secret.to_string());
                let sign = md5::md5(content).to_uppercase();
                Ok(sign)
            }
            // constants::SIGN_TYPE_RSA => {
            //
            // }
            _ => return Err(LabraError::InvalidSignature("不支持的加密方式".to_string()))
        }
    }

    fn get_request_url(&self, holder: &RequestParametersHolder) -> LabradorResult<String> {
        let mut url_sb = self.api_client.api_path.to_owned();
        let sys_must = &holder.protocal_must_params;
        let sys_must_query = serde_urlencoded::to_string(sys_must)?;
        let opt_param = &holder.protocal_opt_params;
        let sys_opt_query = serde_urlencoded::to_string(opt_param)?;
        url_sb += "?";
        url_sb += &sys_must_query;
        if !sys_opt_query.is_empty() {
            url_sb += "&";
            url_sb += &sys_opt_query;
        }
        Ok(url_sb)
    }

    ///
    /// 组装接口参数，处理加密、签名逻辑
    ///
    /// @param request
    fn get_request_holder_with_sign<D>(&self, request: &D) -> LabradorResult<RequestParametersHolder> where D: TaobaoRequest {
        let mut holder = RequestParametersHolder::new();
        let mut app_params = request.get_text_params();
        holder.set_application_params(app_params);

        let mut protocal_must_params = BTreeMap::new();
        protocal_must_params.insert(constants::METHOD.to_string(), request.get_api_method_name().get_method());
        protocal_must_params.insert(constants::VERSION.to_string(), request.get_api_version());
        protocal_must_params.insert(constants::APP_KEY.to_string(), self.api_client.app_key.to_owned());
        protocal_must_params.insert(constants::SIGN_METHOD.to_string(), self.sign_method.to_string());
        protocal_must_params.insert(constants::TIMESTAMP.to_string(), Local::now().naive_local().format(constants::FORMAT_TIME).to_string());
        protocal_must_params.insert(constants::FORMAT.to_string(), FORMAT_JSON.to_string());
        holder.set_protocal_must_params(protocal_must_params.to_owned());
        if !self.sign_method.is_empty() {
            let pairs = holder.get_sorted_map();
            let sign_content = pairs.iter().filter(|(k, v)| !k.is_empty() && !v.is_empty()).map(|(k, v)| format!("{}{}", k, v)).collect::<Vec<String>>().join("");
            protocal_must_params.insert(constants::SIGN.to_string(), self.sign(&sign_content)?);
        } else {
            protocal_must_params.insert(constants::SIGN.to_string(), "".to_string());
        }
        holder.set_protocal_must_params(protocal_must_params);
        Ok(holder)
    }

    /// 发送请求数据
    async fn excute<D>(&self, request: D) -> LabradorResult<TaobaoResponse> where D: TaobaoRequest + Serialize {
        let method = request.get_api_method_name();
        let holder = self.get_request_holder_with_sign(&request)?;
        let url = self.get_request_url(&holder)?;
        let req = LabraRequest::new().url(url).method(Method::Post).data(request).req_type(RequestType::Form);
        let result = self.api_client.request(req).await?.text().await?;
        TaobaoResponse::parse(&result, method)
    }

    /// 获取淘宝客物料精选
    /// 
    /// # 示例
    /// ```no_run
    /// 
    ///     use labrador::TaobaoClient;
    ///     use labrador::{TbMaterialSelectRequest};
    /// 
    ///     async fn main() {
    ///         let material = TbMaterialSelectRequest::default();
    ///         let client = TaobaoClient::new("appKey", "secret");
    ///         match client.get_material_selected(param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    /// 
    /// ```
    /// 
    pub async fn get_material_selected(&self, request: TbMaterialSelectRequest) -> LabradorResult<TbMaterialSelectResponse> {
        let resp = self.excute(request).await?;
        resp.get_biz_model::<TbMaterialSelectResponse>()
    }

    /// 获取淘宝客聚划算搜索商品
    /// 
    /// # 示例
    /// 
    /// ```no_run
    /// 
    ///     use labrador::TaobaoClient;
    ///     use labrador::{TbJhsSearchRequest};
    /// 
    ///     async fn main() {
    ///         let param = TbJhsSearchRequest::default();
    ///         let client = TaobaoClient::new("appKey", "secret");
    ///         match client.search_jhs_items(param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    /// 
    /// ```
    /// 
    #[deprecated]
    pub async fn search_jhs_items(&self, request: TbJhsSearchRequest) -> LabradorResult<TbJhsSearchResponse> {
        let resp = self.excute(request).await?;
        resp.get_biz_model::<TbJhsSearchResponse>()
    }

    /// 淘宝客商品详情查询(简版)
    /// 
    /// # 示例
    /// 
    /// ```no_run
    /// 
    ///     use labrador::TaobaoClient;
    ///     use labrador::{TbItemDetailRequest};
    /// 
    ///     async fn main() {
    ///         let param = TbItemDetailRequest::default();
    ///         let client = TaobaoClient::new("appKey", "secret");
    ///         match client.get_item_detail(param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    /// 
    /// ```
    ///
    pub async fn get_item_detail(&self, request: TbItemDetailRequest) -> LabradorResult<TbItemDetailResponse> {
        let resp = self.excute(request).await?;
        TbItemDetailResponse::from_resp(&resp)
    }

    /// 淘宝客阿里妈妈推广券详情查询
    /// 
    /// # 示例
    /// 
    /// ```no_run
    /// 
    ///     use labrador::TaobaoClient;
    ///     use labrador::{TbCouponDetailRequest};
    /// 
    ///     async fn main() {
    ///         let param = TbCouponDetailRequest::default();
    ///         let client = TaobaoClient::new("appKey", "secret");
    ///         match client.get_coupon_detail(param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    /// 
    /// ```
    ///
    pub async fn get_coupon_detail(&self, request: TbCouponDetailRequest) -> LabradorResult<TbCouponDetailResponse> {
        let resp = self.excute(request).await?;
        TbCouponDetailResponse::from_resp(&resp)
    }

    /// 淘宝客淘口令生成
    /// 
    /// # 示例
    /// 
    /// ```no_run
    /// 
    ///     use labrador::TaobaoClient;
    ///     use labrador::{TbCreateTPwdRequest};
    /// 
    ///     async fn main() {
    ///         let param = TbCreateTPwdRequest::default();
    ///         let client = TaobaoClient::new("appKey", "secret");
    ///         match client.create_tpwd(param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    /// 
    /// ```
    ///
    pub async fn create_tpwd(&self, request: TbCreateTPwdRequest) -> LabradorResult<TbCreateTPwdResponse> {
        let resp = self.excute(request).await?;
        TbCreateTPwdResponse::from_resp(&resp)
    }

    /// 淘宝客-公用-长链转短链
    /// 输入一个原始的链接，转换得到指定的传播方式，如二维码，淘口令，短连接； 现阶段只支持短连接。
    /// # 示例
    ///
    /// ```no_run
    ///
    ///     use labrador::TaobaoClient;
    ///     use labrador::{TbSpreadGetRequest};
    ///
    ///     async fn main() {
    ///         let param = TbSpreadGetRequest::default();
    ///         let client = TaobaoClient::new("appKey", "secret");
    ///         match client.get_spread(param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    ///
    /// ```
    ///
    pub async fn get_spread(&self, request: TbSpreadGetRequest) -> LabradorResult<TbSpreadGetResponse> {
        let resp = self.excute(request).await?;
        resp.get_biz_model::<TbSpreadGetResponse>()
    }

    /// 淘宝客-推广者-淘口令回流数据查询
    /// 淘宝客获取单个淘口令的回流PV、UV数据。
    /// # 示例
    ///
    /// ```no_run
    ///
    ///     use labrador::TaobaoClient;
    ///     use labrador::{TbTPwdReportGetRequest};
    ///
    ///     async fn main() {
    ///         let param = TbTPwdReportGetRequest::default();
    ///         let client = TaobaoClient::new("appKey", "secret");
    ///         match client.get_tpwd_report(param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    ///
    /// ```
    ///
    pub async fn get_tpwd_report(&self, request: TbTPwdReportGetRequest) -> LabradorResult<TbTPwdReportGetResponse> {
        let resp = self.excute(request).await?;
        TbTPwdReportGetResponse::from_resp(&resp)
    }

    /// 淘宝客物料搜索
    /// 
    /// # 示例
    /// 
    /// ```no_run
    /// 
    ///     use labrador::TaobaoClient;
    ///     use labrador::{TbCreateTPwdRequest};
    /// 
    ///     async fn main() {
    ///         let param = TbMaterialSearchRequest::default();
    ///         let client = TaobaoClient::new("appKey", "secret");
    ///         match client.search_material(param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    /// 
    /// ```
    ///
    pub async fn search_material(&self, request: TbMaterialSearchRequest) -> LabradorResult<TbMaterialSearchResponse> {
        let resp = self.excute(request).await?;
        resp.get_biz_model::<TbMaterialSearchResponse>()
    }

    /// 淘宝客-推广者-官方活动转链
    /// 支持入参推广位和官方活动会场ID，获取活动信息和推广链接，包含推广长链接、短链接、淘口令、微信推广二维码地址等。改该API支持二方、三方类型的转链。官方活动会场ID，从淘宝客后台“我要推广-活动推广”中获取。
    /// # 示例
    ///
    /// ```no_run
    ///
    ///     use labrador::TaobaoClient;
    ///     use labrador::{TbCreateTPwdRequest};
    ///
    ///     async fn main() {
    ///         let param = TbMaterialSearchRequest::default();
    ///         let client = TaobaoClient::new("appKey", "secret");
    ///         match client.search_material(param).await {
    ///             Ok(res) => {}
    ///             Err(err) => {}
    ///         }
    ///     }
    ///
    /// ```
    ///
    pub async fn get_activity_info(&self, request: TbGetActivityInfoRequest) -> LabradorResult<TbGetActivityInfoResponse> {
        let resp = self.excute(request).await?;
        TbGetActivityInfoResponse::from_resp(&resp)
    }

    // 36d8db3427d17dd0e583285a1239741c
    // 25304006


}



#[cfg(test)]
#[allow(unused, non_snake_case)]
mod tests {
    use std::collections::BTreeMap;
    use std::fs::File;
    use std::io::Read;
    use std::ops::Add;
    use chrono::{DateTime, Local, NaiveDateTime, SecondsFormat};
    use reqwest::Url;
    use serde::{Deserializer, Deserialize, Serialize};
    use serde_json::{json, Value};
    use crate::ResponseType::Text;
    use crate::{SimpleStorage, TaobaoClient};
    use crate::taobao::request::{TbItemDetailRequest, TbJhsSearchRequest, TbMaterialSearchRequest, TbMaterialSelectRequest};

    #[test]
    fn test_get_material_selected() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let r = rt.spawn(async {
            let client =  TaobaoClient::<SimpleStorage>::new("appkey", "secret");
            let req = TbMaterialSelectRequest {
                material_id: 13366,
                adzone_id: 0,
                page_size: 2.into(),
                page_no: 1.into(),
                device_value: None,
                device_encrypt: None,
                device_type: None,
                content_id: None,
                content_source: None,
                favorites_id: None,
                item_id: None,
            };
            let result = client.get_material_selected(req);
            match result.await {
                Ok(res) => {
                    println!("请求成功:{:?}",res);
                }
                Err(err) => {
                    println!("err:{:?}", err);
                }
            }
        });
        rt.block_on(r);
    }
    #[test]
    fn test_search_jhs_items() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let r = rt.spawn(async {
            let client =  TaobaoClient::<SimpleStorage>::new("appkey", "secret");
            let req = TbJhsSearchRequest {
                page_size: 2.into(),
                current_page: None,
                pid: "230050008".to_string(),
                postage: None,
                status: None,
                taobao_category_id: None,
                word: None
            };
            let result = client.search_jhs_items(req);
            match result.await {
                Ok(res) => {
                    println!("请求成功:{:?}",res);
                }
                Err(err) => {
                    println!("err:{:?}", err);
                }
            }
        });
        rt.block_on(r);
    }

    #[test]
    fn test_search_material() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let r = rt.spawn(async {
            let client =  TaobaoClient::<SimpleStorage>::new("appkey", "secret");
            let req = TbMaterialSearchRequest {
                page_size: None,
                adzone_id: 75275600257,
                platform: None,
                end_tk_rate: None,
                start_tk_rate: None,
                end_price: None,
                start_price: None,
                end_ka_tk_rate: None,
                start_ka_tk_rate: None,
                is_tmall: None,
                has_coupon: None,
                need_free_shipment: None,
                need_prepay: None,
                include_good_rate: None,
                include_pay_rate_30: None,
                include_rfd_rate: None,
                npx_level: None,
                sort: None,
                itemloc: None,
                ip: None,
                q: Some("面包".to_string()),
                cat: None,
                material_id: None,
                item_id: None,
                device_value: None,
                device_encrypt: None,
                longitude: None,
                latitude: None,
                city_code: None,
                seller_ids: None,
                special_id: None,
                relation_id: None,
                device_type: None,
                lock_rate_end_time: None,
                page_no: None,
                lock_rate_start_time: None
            };
            let result = client.search_material(req);
            match result.await {
                Ok(res) => {
                    println!("请求成功:{:?}",res);
                }
                Err(err) => {
                    println!("err:{:?}", err);
                }
            }
        });
        rt.block_on(r);
    }

    #[test]
    fn test_get_item_detail() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let r = rt.spawn(async {
            let client =  TaobaoClient::<SimpleStorage>::new("appkey", "secret");
            let req = TbItemDetailRequest {
                num_iids: Some("597649283190".to_string()),
                platform: None,
                ip: None
            };
            let result = client.get_item_detail(req);
            match result.await {
                Ok(res) => {
                    println!("请求成功:{:?}",res);
                }
                Err(err) => {
                    println!("err:{:?}", err);
                }
            }
        });
        rt.block_on(r);
    }

}
