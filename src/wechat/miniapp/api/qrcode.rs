use crate::{session::SessionStore, errors::LabraError, request::{RequestType}, LabradorResult, WechatCommonResponse};
use bytes::Bytes;
use serde::{Serialize, Deserialize};
use crate::wechat::miniapp::method::{MaQrCodeMethod, WechatMaMethod};
use crate::wechat::miniapp::WeChatMaClient;

///<pre>
/// 二维码相关操作接口.
///
/// 接口A（createWxaCode）加上接口C（createQrcode），总共生成的码数量限制为100,000，请谨慎调用。
/// </pre>
/// [文档地址](https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/qrcode-link/qr-code/getQRCode.html)
///
#[derive(Debug, Clone)]
pub struct WechatMaQrcode<'a, T: SessionStore> {
    client: &'a WeChatMaClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WechatMaQrcode<'a, T> {

    #[inline]
    pub fn new(client: &WeChatMaClient<T>) -> WechatMaQrcode<T> {
        WechatMaQrcode {
            client,
        }
    }

    ///
    /// 接口C: 获取小程序页面二维码.
    /// <pre>
    /// 适用于需要的码数量较少的业务场景
    /// 通过该接口，仅能生成已发布的小程序的二维码。
    /// 可以在开发者工具预览时生成开发版的带参二维码。
    /// 带参二维码只有 100000 个，请谨慎调用。
    /// </pre>
    /// [`path`] 扫码进入的小程序页面路径，最大长度 128 字节，不能为空；对于小游戏，可以只传入 query 部分，来实现传参效果，如：传入 "?foo=bar"，即可在 wx.getLaunchOptionsSync 接口中的 query 参数获取到 {foo:"bar"}。
    /// [`width`] 二维码的宽度，单位 px。最小 280px，最大 1280px;默认是430
    pub async fn create_qrcode<D: Serialize>(&self, path: &str, width: Option<i32>) -> LabradorResult<Bytes> {
        let width = width.unwrap_or(430);
        let mini_qr_code = QRCodeRequest {
            width,
            path: path.to_string()
        };
        let result = self.client.post(WechatMaMethod::QrCode(MaQrCodeMethod::CreateWxaQrCode), vec![], &mini_qr_code, RequestType::Json).await?.bytes()?;
        let res_str = String::from_utf8(result.to_vec()).unwrap_or_default();
        match WechatCommonResponse::from_str(&res_str) {
            Ok(r) => {
                return Err(LabraError::ClientError { errcode: r.errcode.to_owned().unwrap_or_default().to_string(), errmsg: r.errmsg.to_owned().unwrap_or_default()})
            }
            Err(err) => {  }
        };
        Ok(result)
    }


    /// 获取不限制的小程序码
    /// 该接口用于获取小程序码，适用于需要的码数量极多的业务场景。通过该接口生成的小程序码，永久有效，数量暂无限制。 更多用法详见 获取小程序码。
    /// <pre>
    /// 注意事项
    /// 如果调用成功，会直接返回图片二进制内容，如果请求失败，会返回 JSON 格式的数据。
    /// POST 参数需要转成 JSON 字符串，不支持 form 表单提交。
    /// 接口只能生成已发布的小程序码
    /// 调用分钟频率受限（5000次/分钟），如需大量小程序码，建议预生成
    /// 获取 scene 值
    /// scene 字段的值会作为 query 参数传递给小程序/小游戏。用户扫描该码进入小程序/小游戏后，开发者可以获取到二维码中的 scene 值，再做处理逻辑。
    /// 调试阶段可以使用开发工具的条件编译自定义参数 scene=xxxx 进行模拟，开发工具模拟时的 scene 的参数值需要进行 encodeURIComponent
    /// </pre>
    pub async fn get_unlimited_qrcode(&mut self, scene: &str, page: &str) -> LabradorResult<Bytes> {
        let mini_qr_code = MiniQRCodeRequest {
            scene: scene.to_owned(),
            page: page.to_owned(),
        };
        let result = self.client.post(WechatMaMethod::QrCode(MaQrCodeMethod::GetWxaCodeUnlimit), vec![], &mini_qr_code, RequestType::Json).await?.bytes()?;
        let res_str = String::from_utf8(result.to_vec()).unwrap_or_default();
        match WechatCommonResponse::from_str(&res_str) {
            Ok(r) => {
                return Err(LabraError::ClientError { errcode: r.errcode.to_owned().unwrap_or_default().to_string(), errmsg: r.errmsg.to_owned().unwrap_or_default()})
            }
            Err(err) => {  }
        };
        Ok(result)
    }


    /// 获取不限制的小程序码
    /// 该接口用于获取小程序码，适用于需要的码数量较少的业务场景。通过该接口生成的小程序码，永久有效，有数量限制，详见获取小程序码。
    /// <pre>
    /// 注意事项
    /// 如果调用成功，会直接返回图片二进制内容，如果请求失败，会返回 JSON 格式的数据。
    /// POST 参数需要转成 JSON 字符串，不支持 form 表单提交。
    /// 接口只能生成已发布的小程序码
    /// 与 createQRCode 总共生成的码数量限制为 100,000，请谨慎调用。
    /// </pre>
    pub async fn get_qrcode(&mut self, path: &str, width: Option<i32>) -> LabradorResult<Bytes> {
        let width = width.unwrap_or(430);
        let mini_qr_code = QRCodeRequest {
            width,
            path: path.to_string()
        };
        let result = self.client.post(WechatMaMethod::QrCode(MaQrCodeMethod::GetWxaCodeUnlimit), vec![], &mini_qr_code, RequestType::Json).await?.bytes()?;
        let res_str = String::from_utf8(result.to_vec()).unwrap_or_default();
        match WechatCommonResponse::from_str(&res_str) {
            Ok(r) => {
                return Err(LabraError::ClientError { errcode: r.errcode.to_owned().unwrap_or_default().to_string(), errmsg: r.errmsg.to_owned().unwrap_or_default()})
            }
            Err(err) => {  }
        };
        Ok(result)
    }


}

//----------------------------------------------------------------------------------------------------------------------------


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MiniQRCodeRequest {
    scene: String,
    page: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QRCodeRequest {
    width: i32,
    path: String,
}

