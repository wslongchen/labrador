use crate::{session::SessionStore, errors::LabraError, request::{RequestType}, WechatCommonResponse, WeChatMpClient, LabradorResult};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use crate::wechat::mp::constants::{QR_LIMIT_SCENE, QR_SCENE};
use crate::wechat::mp::method::{MpQrCodeMethod, WechatMpMethod};

#[derive(Debug, Clone)]
pub struct WeChatMpQRCode<'a, T: SessionStore> {
    client: &'a WeChatMpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WeChatMpQRCode<'a, T> {

    #[inline]
    pub fn new(client: &WeChatMpClient<T>) -> WeChatMpQRCode<T> {
        WeChatMpQRCode {
            client,
        }
    }

    /// <pre>
    /// 换取临时二维码ticket
    /// 详情请见: <a href="https://mp.weixin.qq.com/wiki?action=doc&id=mp1443433542&t=0.9274944716856435">生成带参数的二维码</a>
    /// </pre>
    pub async fn create_temp_ticket_sceneid<D: Serialize>(&self, scene_id: i32, expire_seconds: u64) -> LabradorResult<QRCodeTicket> {
        if scene_id == 0 {
            return Err(LabraError::RequestError("临时二维码场景值不能为0！".to_string()));
        }
        self.create_qrcode(QR_SCENE, None, scene_id.into(), expire_seconds.into()).await
    }

    /// <pre>
    /// 换取临时二维码ticket
    /// 详情请见: <a href="https://mp.weixin.qq.com/wiki?action=doc&id=mp1443433542&t=0.9274944716856435">生成带参数的二维码</a>
    /// </pre>
    pub async fn create_temp_ticket_scenestr<D: Serialize>(&self, scene_str: &str, expire_seconds: u64) -> LabradorResult<QRCodeTicket> {
        if scene_str.is_empty() {
            return Err(LabraError::RequestError("临时二维码场景值不能为空！".to_string()));
        }
        self.create_qrcode(QR_SCENE, scene_str.into(), None, expire_seconds.into()).await
    }

    async fn create_qrcode(&self, action_name: &str, scene_str: Option<&str>, scene_id: Option<i32>, mut expire_seconds: Option<u64>) -> LabradorResult<QRCodeTicket> {
        //expireSeconds 该二维码有效时间，以秒为单位。 最大不超过2592000（即30天），此字段如果不填，则默认有效期为30秒。
        if expire_seconds.is_some() && expire_seconds.unwrap_or_default() > 2592000 {
            return Err(LabraError::RequestError("临时二维码有效时间最大不能超过2592000（即30天）！".to_string()));
        }
        if expire_seconds.is_none() {
            expire_seconds = Some(30);
        }

        self.get_qrcode_ticket(action_name, scene_str, scene_id, expire_seconds).await
    }

    async fn get_qrcode_ticket(&self, action_name: &str, scene_str: Option<&str>, scene_id: Option<i32>, mut expire_seconds: Option<u64>) -> LabradorResult<QRCodeTicket> {

        let mut scene = if let Some(scene_str) = scene_str {
            json!({"scene_str":scene_str})
        } else {
            if let Some(scene_id) = scene_id {
                json!({"scene_id": scene_id})
            } else {
                Value::Null
            }
        };
        let mut req = json!({
            "action_name": action_name,
            "action_info": {
                "scene": scene
            }
        });
        if let Some(expire_seconds) = expire_seconds {
            req["expire_seconds"] = expire_seconds.into();
        }
        let v = self.client.post(WechatMpMethod::QrCode(MpQrCodeMethod::Create), vec![], req, RequestType::Json).await?.json::<serde_json::Value>()?;
        WechatCommonResponse::parse::<QRCodeTicket>(v)
    }

    /// <pre>
    /// 换取永久二维码ticket
    /// 详情请见: <a href="https://mp.weixin.qq.com/wiki?action=doc&id=mp1443433542&t=0.9274944716856435">生成带参数的二维码</a>
    /// </pre>
    pub async fn get_unlimited_scenestr(&self, scene_str: &str) -> LabradorResult<QRCodeTicket> {
        self.get_qrcode_ticket(QR_LIMIT_SCENE, scene_str.into(), None, None).await
    }

    /// <pre>
    /// 换取永久二维码ticket
    /// 详情请见: <a href="https://mp.weixin.qq.com/wiki?action=doc&id=mp1443433542&t=0.9274944716856435">生成带参数的二维码</a>
    /// </pre>
    pub async fn get_unlimited_sceneid(&self, scene_id: i32) -> LabradorResult<QRCodeTicket> {
        self.get_qrcode_ticket(QR_LIMIT_SCENE, None, scene_id.into(), None).await
    }

    /// <pre>
    /// 换取二维码图片url地址
    /// 详情请见: <a href="https://mp.weixin.qq.com/wiki?action=doc&id=mp1443433542&t=0.9274944716856435">生成带参数的二维码</a>
    /// </pre>
    pub fn get_url_with_ticket(&self, ticket: &str) -> String {
        format!("{}?ticket={}", MpQrCodeMethod::ShowQrCode.get_method(), ticket)
    }

    /// <pre>
    /// 换取二维码图片url地址（可以选择是否生成压缩的网址）
    /// 详情请见: <a href="https://mp.weixin.qq.com/wiki?action=doc&id=mp1443433542&t=0.9274944716856435">生成带参数的二维码</a>
    /// </pre>
    pub fn get_url(&self, qrcode_ticket: &QRCodeTicket) -> String {
        let ticket = &qrcode_ticket.ticket.to_owned().unwrap_or_default();
        self.get_url_with_ticket(ticket)
    }
}

//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct QRCodeTicket {
    pub ticket: Option<String>,
    pub expire_seconds: Option<i32>,
    pub url: Option<String>,
}

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct TempQRCodeRequest {
    scene_id: Option<u64>,
    scene_str: Option<String>,
    expire_seconds: u32,
}

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct PermQRCodeRequest {
    scene_str: String,
}

#[derive(Debug, Clone,  Serialize, Deserialize)]
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