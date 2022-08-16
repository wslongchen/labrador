use crate::{session::SessionStore, errors::LabraError, request::{RequestType}, WechatCommonResponse, WeChatMpClient, LabradorResult};
use bytes::Bytes;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use crate::wechat::mp::method::{QrCodeMethod, WechatMpMethod};

#[derive(Debug, Clone)]
pub struct WeChatQRCode<'a, T: SessionStore> {
    client: &'a WeChatMpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WeChatQRCode<'a, T> {

    #[inline]
    pub fn new(client: &WeChatMpClient<T>) -> WeChatQRCode<T> {
        WeChatQRCode {
            client,
        }
    }

    /// 创建二维码凭证
    pub async fn create<D: Serialize>(&mut self, data: D) -> LabradorResult<WechatCommonResponse<QRCodeTicket>> {
        let res = self.client.post(WechatMpMethod::QrCode(QrCodeMethod::Create), data, RequestType::Json).await?.json::<serde_json::Value>().await?;
        let mut result = serde_json::from_value::<WechatCommonResponse<_>>(res.to_owned())?;
        if result.is_success() {
            let ticket = &res["ticket"];
            let ticket = ticket.as_str().unwrap_or_default().to_owned();
            let expire_seconds = match res.get("expire_seconds") {
                Some(seconds) => seconds.as_u64().unwrap(),
                None => 0u64,
            };
            let url = &res["url"];
            let url = url.as_str().unwrap_or_default().to_owned();
            result.result = QRCodeTicket {
                ticket: ticket.to_owned(),
                expire_seconds: expire_seconds as u32,
                url: url.to_owned(),
            }.into();
        }
        Ok(result)
    }


    /// 获取二维码
    pub async fn get_unlimited(&mut self, scene: &str, page: &str) -> LabradorResult<Bytes> {
        let mini_qr_code = MiniQRCodeRequest {
            scene: scene.to_owned(),
            page: page.to_owned(),
        };
        let res = self.client.post(WechatMpMethod::QrCode(QrCodeMethod::GetWxaCodeUnlimit), &mini_qr_code, RequestType::Json).await?;
        let bytes = res.bytes().await?;
        let res_str = String::from_utf8(bytes.to_owned().to_vec()).unwrap_or_default();
        match serde_json::from_str::<Value>(res_str.as_str()) {
            Ok(r) => {
                let errcode = &r["errcode"].as_i64().unwrap_or_default().to_owned();
                let errmsg = &r["errmsg"].as_str().unwrap_or_default().to_owned();
                return Err(LabraError::ClientError { errcode: errcode.to_string(), errmsg: errmsg.to_owned()})
            }
            Err(err) => {  }
        };
        Ok(bytes)
    }

    pub fn get_url_with_ticket(&self, ticket: &str) -> String {
        format!("{}?ticket={}", QrCodeMethod::ShowQrCode.get_method(), ticket)
    }

    pub fn get_url(&self, qrcode_ticket: &QRCodeTicket) -> String {
        let ticket = &qrcode_ticket.ticket;
        self.get_url_with_ticket(ticket)
    }
}

//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QRCodeTicket {
    pub ticket: String,
    pub expire_seconds: u32,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TempQRCodeRequest {
    scene_id: Option<u64>,
    scene_str: Option<String>,
    expire_seconds: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermQRCodeRequest {
    scene_str: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MiniQRCodeRequest {
    scene: String,
    page: String,
}


#[allow(unused)]
impl TempQRCodeRequest {
    pub fn new(scene_id: Option<u64>, expire_seconds: u32, scene_str: Option<String>) -> TempQRCodeRequest {
        TempQRCodeRequest {
            scene_id: scene_id,
            expire_seconds: expire_seconds,
            scene_str: scene_str,
        }
    }
}


#[allow(unused)]
impl TempQRCodeRequest {
    pub fn to_json_default(&self) -> Value {
        // {"expire_seconds": 604800, "action_name": "QR_STR_SCENE", "action_info": {"scene": {"scene_str": "test"}}}
        json!({
            "action_name": "QR_SCENE".to_owned(),
            "expire_seconds": self.expire_seconds,
            "action_info": {
                "scene": {
                    "scene_id": self.scene_id.unwrap_or_default()
                },
            }
        })
    }

    pub fn to_json_str(&self) -> Value {
        // {"expire_seconds": 604800, "action_name": "QR_STR_SCENE", "action_info": {"scene": {"scene_str": "test"}}}
        json!({
            "action_name": "QR_STR_SCENE".to_owned(),
            "expire_seconds": self.expire_seconds,
            "action_info": {
                "scene": {
                    "scene_str": self.scene_str.to_owned().unwrap_or_default()
                },
            }
        })
    }
}

#[allow(unused)]
impl MiniQRCodeRequest {
    fn to_json(&self) -> Value {
        json!({
            "scene": self.scene,
        })
    }
}


#[allow(unused)]
impl PermQRCodeRequest {
    pub fn new<S: Into<String>>(scene_str: S) -> PermQRCodeRequest {
        PermQRCodeRequest  {
            scene_str: scene_str.into(),
        }
    }
}


#[allow(unused)]
impl PermQRCodeRequest {
    fn  to_json(&self) -> Value {
        json!({
            "action_name": "QR_LIMIT_STR_SCENE".to_owned(),
            "action_info": {
                "scene": {
                    "scene_str": self.scene_str.to_owned()
                }
            }
        })
    }
}