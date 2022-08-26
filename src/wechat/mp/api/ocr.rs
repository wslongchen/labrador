use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::vec;

use serde::{Serialize, Deserialize};
use serde_json::{Value};

use crate::{session::SessionStore, request::{RequestType}, WechatCommonResponse, WeChatMpClient, LabradorResult, WechatRequest, RequestBody};
use crate::wechat::mp::constants::IMG_URL;
use crate::wechat::mp::method::{MpOcrMethod, WechatMpMethod};

/// 微信连接WI-FI接口.
#[derive(Debug, Clone)]
pub struct WeChatMpOcr<'a, T: SessionStore> {
    client: &'a WeChatMpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WeChatMpOcr<'a, T> {

    #[inline]
    pub fn new(client: &WeChatMpClient<T>) -> WeChatMpOcr<T> {
        WeChatMpOcr {
            client,
        }
    }

    /// 身份证OCR识别接口
    pub async fn id_card(&self, img_url: &str) -> LabradorResult<WechatOcrIdCardResponse> {
        let img_url = urlencoding::encode(img_url).to_string();
        let v = self.client.post(WechatMpMethod::Ocr(MpOcrMethod::IdCard), vec![(IMG_URL.to_string(), img_url)], Value::Null, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatOcrIdCardResponse>(v)
    }

    /// 身份证OCR识别接口
    pub async fn id_card_file(&self, file_path: &str) -> LabradorResult<WechatOcrIdCardResponse> {
        let path = Path::new(file_path);
        let file_name = path.file_name().map(|v| v.to_str().unwrap_or_default()).unwrap_or_default();
        let mut f = File::open(path)?;
        let mut content: Vec<u8> = Vec::new();
        let _ = f.read_to_end(&mut content)?;
        let req = WechatMpOcrRequest {
            ocr_type: 1,
            filename: "".to_string(),
            data: content
        };
        let v = self.client.execute::<WechatMpOcrRequest, String>(req).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatOcrIdCardResponse>(v)
    }

    /// 银行卡OCR识别接口
    /// 文件大小限制：小于2M
    pub async fn back_card(&self, img_url: &str) -> LabradorResult<WechatOcrBankCardResponse> {
        let img_url = urlencoding::encode(img_url).to_string();
        let v = self.client.post(WechatMpMethod::Ocr(MpOcrMethod::BankCard), vec![(IMG_URL.to_string(), img_url)], Value::Null, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatOcrBankCardResponse>(v)
    }

    /// 银行卡OCR识别接口
    /// 文件大小限制：小于2M
    pub async fn back_card_file(&self, file_path: &str) -> LabradorResult<WechatOcrBankCardResponse> {
        let path = Path::new(file_path);
        let file_name = path.file_name().map(|v| v.to_str().unwrap_or_default()).unwrap_or_default();
        let mut f = File::open(path)?;
        let mut content: Vec<u8> = Vec::new();
        let _ = f.read_to_end(&mut content)?;
        let req = WechatMpOcrRequest {
            ocr_type: 2,
            filename: "".to_string(),
            data: content
        };
        let v = self.client.execute::<WechatMpOcrRequest, String>(req).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatOcrBankCardResponse>(v)
    }

    /// 行驶证OCR识别接口
    /// 文件大小限制：小于2M
    pub async fn driving(&self, img_url: &str) -> LabradorResult<WechatOcrDrivingResponse> {
        let img_url = urlencoding::encode(img_url).to_string();
        let v = self.client.post(WechatMpMethod::Ocr(MpOcrMethod::Driving), vec![(IMG_URL.to_string(), img_url)], Value::Null, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatOcrDrivingResponse>(v)
    }

    /// 行驶证OCR识别接口
    /// 文件大小限制：小于2M
    pub async fn driving_file(&self, file_path: &str) -> LabradorResult<WechatOcrDrivingResponse> {
        let path = Path::new(file_path);
        let file_name = path.file_name().map(|v| v.to_str().unwrap_or_default()).unwrap_or_default();
        let mut f = File::open(path)?;
        let mut content: Vec<u8> = Vec::new();
        let _ = f.read_to_end(&mut content)?;
        let req = WechatMpOcrRequest {
            ocr_type: 3,
            filename: "".to_string(),
            data: content
        };
        let v = self.client.execute::<WechatMpOcrRequest, String>(req).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatOcrDrivingResponse>(v)
    }

    /// 驾驶证OCR识别接口
    /// 文件大小限制：小于2M
    pub async fn driving_license(&self, img_url: &str) -> LabradorResult<WechatOcrDrivingLicenseResponse> {
        let img_url = urlencoding::encode(img_url).to_string();
        let v = self.client.post(WechatMpMethod::Ocr(MpOcrMethod::DrivingLicense), vec![(IMG_URL.to_string(), img_url)], Value::Null, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatOcrDrivingLicenseResponse>(v)
    }

    /// 驾驶证OCR识别接口
    /// 文件大小限制：小于2M
    pub async fn driving_license_file(&self, file_path: &str) -> LabradorResult<WechatOcrDrivingLicenseResponse> {
        let path = Path::new(file_path);
        let file_name = path.file_name().map(|v| v.to_str().unwrap_or_default()).unwrap_or_default();
        let mut f = File::open(path)?;
        let mut content: Vec<u8> = Vec::new();
        let _ = f.read_to_end(&mut content)?;
        let req = WechatMpOcrRequest {
            ocr_type: 4,
            filename: "".to_string(),
            data: content
        };
        let v = self.client.execute::<WechatMpOcrRequest, String>(req).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatOcrDrivingLicenseResponse>(v)
    }

    /// 营业执照OCR识别接口
    /// 文件大小限制：小于2M
    pub async fn biz_license(&self, img_url: &str) -> LabradorResult<WechatOcrBizLicenseResponse> {
        let img_url = urlencoding::encode(img_url).to_string();
        let v = self.client.post(WechatMpMethod::Ocr(MpOcrMethod::BizLicense), vec![(IMG_URL.to_string(), img_url)], Value::Null, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatOcrBizLicenseResponse>(v)
    }

    /// 营业执照OCR识别接口
    /// 文件大小限制：小于2M
    pub async fn biz_license_file(&self, file_path: &str) -> LabradorResult<WechatOcrBizLicenseResponse> {
        let path = Path::new(file_path);
        let file_name = path.file_name().map(|v| v.to_str().unwrap_or_default()).unwrap_or_default();
        let mut f = File::open(path)?;
        let mut content: Vec<u8> = Vec::new();
        let _ = f.read_to_end(&mut content)?;
        let req = WechatMpOcrRequest {
            ocr_type: 5,
            filename: "".to_string(),
            data: content
        };
        let v = self.client.execute::<WechatMpOcrRequest, String>(req).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatOcrBizLicenseResponse>(v)
    }

    /// 通用印刷体OCR识别接口
    /// 文件大小限制：小于2M
    /// 适用于屏幕截图、印刷体照片等场景
    pub async fn comm(&self, img_url: &str) -> LabradorResult<WechatOcrCommResponse> {
        let img_url = urlencoding::encode(img_url).to_string();
        let v = self.client.post(WechatMpMethod::Ocr(MpOcrMethod::Comm), vec![(IMG_URL.to_string(), img_url)], Value::Null, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatOcrCommResponse>(v)
    }

    /// 通用印刷体OCR识别接口
    /// 文件大小限制：小于2M
    /// 适用于屏幕截图、印刷体照片等场景
    pub async fn comm_file(&self, file_path: &str) -> LabradorResult<WechatOcrCommResponse> {
        let path = Path::new(file_path);
        let file_name = path.file_name().map(|v| v.to_str().unwrap_or_default()).unwrap_or_default();
        let mut f = File::open(path)?;
        let mut content: Vec<u8> = Vec::new();
        let _ = f.read_to_end(&mut content)?;
        let req = WechatMpOcrRequest {
            ocr_type: 6,
            filename: "".to_string(),
            data: content
        };
        let v = self.client.execute::<WechatMpOcrRequest, String>(req).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatOcrCommResponse>(v)
    }

}

//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatOcrIdCardResponse {
    #[serde(rename="type")]
    pub r#type: Option<String>,
    pub name: Option<String>,
    pub id: Option<String>,
    pub valid_date: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatOcrCommResponse {
    pub img_size: Option<WechatOcrImgSize>,
    pub items: Option<Vec<WechatOcrItems>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatOcrItems {
    pub text: Option<String>,
    pub pos: Option<WechatOcrPos>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatOcrBankCardResponse {
    pub number: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatOcrDrivingLicenseResponse {
    /// 证号
    pub id_num: Option<String>,
    /// 姓名
    pub name: Option<String>,
    /// 性别
    pub sex: Option<String>,
    /// 国籍
    pub nationality: Option<String>,
    /// 住址
    pub address: Option<String>,
    /// 出生日期
    pub birth_date: Option<String>,
    /// 初次领证日期
    pub issue_date: Option<String>,
    /// 准驾车型
    pub car_class: Option<String>,
    /// 有效期限起始日
    pub valid_from: Option<String>,
    /// 有效期限终止日
    pub valid_to: Option<String>,
    /// 印章文字
    pub official_seal: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatOcrBizLicenseResponse {
    /// 注册号
    pub reg_num: Option<String>,
    /// 编号
    pub serial: Option<String>,
    /// 法定代表人姓名
    pub legal_representative: Option<String>,
    /// 企业名称
    pub enterprise_name: Option<String>,
    /// 组成形式
    pub type_of_organization: Option<String>,
    /// 经营场所/企业住所
    pub address: Option<String>,
    /// 公司类型
    pub type_of_enterprise: Option<String>,
    /// 经营范围
    pub business_scope: Option<String>,
    /// 注册资本
    pub registered_capital: Option<String>,
    /// 实收资本
    pub paid_in_capital: Option<String>,
    /// 营业期限
    pub valid_period: Option<String>,
    /// 注册日期/成立日期
    pub registered_date: Option<String>,
    /// 营业执照位置
    pub cert_position: Option<CertPosition>,
    /// 图片大小
    pub img_size: Option<WechatOcrImgSize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatOcrDrivingResponse {
    /// 车牌号码
    pub plate_num: Option<String>,
    /// 车辆类型
    pub vehicle_type: Option<String>,
    /// 所有人
    pub owner: Option<String>,
    /// 住址
    pub addr: Option<String>,
    /// 使用性质
    pub use_character: Option<String>,
    /// 品牌型号
    pub model: Option<String>,
    /// 车辆识别代码
    pub vin: Option<String>,
    /// 发动机号码
    pub engine_num: Option<String>,
    /// 注册日期
    pub register_date: Option<String>,
    /// 发证日期
    pub issue_date: Option<String>,
    /// 车牌号码
    pub plate_num_b: Option<String>,
    /// 号牌
    pub record: Option<String>,
    /// 核定载人数
    pub passengers_num: Option<String>,
    /// 总质量
    pub total_quality: Option<String>,
    /// 整备质量
    pub prepare_quality: Option<String>,
    /// 外廓尺寸
    pub overall_size: Option<String>,
    /// 卡片正面位置（检测到卡片正面才会返回）
    pub card_position_front: Option<CardPosition>,
    /// 卡片反面位置（检测到卡片反面才会返回）
    pub card_position_back: Option<CardPosition>,
    /// 图片大小
    pub img_size: Option<WechatOcrImgSize>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardPosition {
    pub pos: Option<WechatOcrPos>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertPosition {
    pub pos: Option<WechatOcrPos>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatOcrPos {
    pub left_top: Option<Coordinate>,
    pub right_top: Option<Coordinate>,
    pub right_bottom: Option<Coordinate>,
    pub left_bottom: Option<Coordinate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coordinate {
    pub x: Option<i64>,
    pub y: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatOcrImgSize {
    pub w: Option<i64>,
    pub h: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpOcrRequest {
    /// 1 身份证 2 银行卡 3 行驶证 4 驾驶证 5 营业执照 6 通用印刷体
    pub ocr_type: u8,
    pub filename: String,
    pub data: Vec<u8>
}

impl WechatRequest for WechatMpOcrRequest {
    fn get_api_method_name(&self) -> String {
        match self.ocr_type {
            1 => MpOcrMethod::IdCard.get_method(),
            2 => MpOcrMethod::BankCard.get_method(),
            3 => MpOcrMethod::Driving.get_method(),
            4 => MpOcrMethod::DrivingLicense.get_method(),
            5 => MpOcrMethod::BizLicense.get_method(),
            6 => MpOcrMethod::Comm.get_method(),
            _=> "".to_string()
        }
    }

    fn get_request_body<T: Serialize>(&self) -> RequestBody<T> {
        let form = reqwest::multipart::Form::new().part("media", reqwest::multipart::Part::stream(self.data.to_vec()).file_name(self.filename.to_string()));
        form.into()
    }
}