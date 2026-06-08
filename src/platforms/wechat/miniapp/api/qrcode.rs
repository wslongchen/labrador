/*
 *
 *  *
 *  *      Copyright (c) 2018-2025, SnackCloud All rights reserved.
 *  *
 *  *   Redistribution and use in source and binary forms, with or without
 *  *   modification, are permitted provided that the following conditions are met:
 *  *
 *  *   Redistributions of source code must retain the above copyright notice,
 *  *   this list of conditions and the following disclaimer.
 *  *   Redistributions in binary form must reproduce the above copyright
 *  *   notice, this list of conditions and the following disclaimer in the
 *  *   documentation and/or other materials provided with the distribution.
 *  *   Neither the name of the www.snackcloud.cn developer nor the names of its
 *  *   contributors may be used to endorse or promote products derived from
 *  *   this software without specific prior written permission.
 *  *   Author: SnackCloud
 *  *
 *
 */
use crate::errors::{LabraError, LabradorResult};
use crate::request::{HttpMethod, RequestBody};
use crate::wechat::client::WechatApiResponse;
use crate::wechat::miniapp::WechatMiniAppClient;
use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct WechatMxaQrcode<'a> {
    client: &'a WechatMiniAppClient,
}

#[allow(unused)]
impl<'a> WechatMxaQrcode<'a> {
    #[inline]
    pub fn new(client: &'a WechatMiniAppClient) -> WechatMxaQrcode<'a> {
        WechatMxaQrcode { client }
    }

    /// 获取小程序码（数量有限，请勿滥用）
    pub async fn get_wxacode(&self, request: &WxacodeRequest) -> LabradorResult<Bytes> {
        let response = self
            .client
            .wechat_client()
            .request_internal(
                HttpMethod::Post,
                "/wxa/getwxacode",
                Some(RequestBody::from(request)),
            )
            .await?;

        // 检查是否是错误响应
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok());

        if let Some(content_type) = content_type {
            if content_type.contains("application/json") {
                // 如果是JSON响应，说明出错了
                let error: WechatApiResponse<WechatApiResponse> =
                    serde_json::from_slice(&response.bytes()).map_err(LabraError::Json)?;
                return error.into_result().map(|_| Bytes::new());
            }
        }

        Ok(response.bytes())
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
    pub async fn get_wxacode_unlimited(
        &self,
        request: &WxacodeUnlimitedRequest,
    ) -> LabradorResult<Bytes> {
        let response = self
            .client
            .wechat_client()
            .request_internal(
                HttpMethod::Post,
                "/wxa/getwxacodeunlimit",
                Some(RequestBody::from(request)),
            )
            .await?;

        // 检查是否是错误响应
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok());

        if let Some(content_type) = content_type {
            if content_type.contains("application/json") {
                // 如果是JSON响应，说明出错了
                let error: WechatApiResponse<WechatApiResponse> =
                    serde_json::from_slice(&response.bytes()).map_err(LabraError::Json)?;
                return error.into_result().map(|_| Bytes::new());
            }
        }

        Ok(response.bytes())
    }

    /// 获取小程序二维码（数量有限，请勿滥用）
    /// <pre>
    /// 适用于需要的码数量较少的业务场景
    /// 通过该接口，仅能生成已发布的小程序的二维码。
    /// 可以在开发者工具预览时生成开发版的带参二维码。
    /// 带参二维码只有 100000 个，请谨慎调用。
    /// </pre>
    /// [`path`] 扫码进入的小程序页面路径，最大长度 128 字节，不能为空；对于小游戏，可以只传入 query 部分，来实现传参效果，如：传入 "?foo=bar"，即可在 wx.getLaunchOptionsSync 接口中的 query 参数获取到 {foo:"bar"}。
    /// [`width`] 二维码的宽度，单位 px。最小 280px，最大 1280px;默认是430
    pub async fn create_wxaqrcode(&self, request: &WxaqrcodeRequest) -> LabradorResult<Bytes> {
        let response = self
            .client
            .wechat_client()
            .request_internal(
                HttpMethod::Post,
                "/cgi-bin/wxaapp/createwxaqrcode",
                Some(RequestBody::from(request)),
            )
            .await?;

        // 检查是否是错误响应
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok());

        if let Some(content_type) = content_type {
            if content_type.contains("application/json") {
                // 如果是JSON响应，说明出错了
                let error: WechatApiResponse<WechatApiResponse> =
                    serde_json::from_slice(&response.bytes()).map_err(LabraError::Json)?;
                return error.into_result().map(|_| Bytes::new());
            }
        }

        Ok(response.bytes())
    }

    /// 获取小程序URL Scheme
    pub async fn generate_url_scheme(
        &self,
        request: &UrlSchemeRequest,
    ) -> LabradorResult<UrlSchemeResponse> {
        let response: WechatApiResponse<UrlSchemeResponse> = self
            .client
            .wechat_client()
            .post("/wxa/generatescheme", RequestBody::from(request))
            .await?;

        response.into_result()
    }

    /// 获取小程序URL Link
    pub async fn generate_url_link(
        &self,
        request: &UrlLinkRequest,
    ) -> LabradorResult<UrlLinkResponse> {
        let response: WechatApiResponse<UrlLinkResponse> = self
            .client
            .wechat_client()
            .post("/wxa/generate_urllink", RequestBody::from(request))
            .await?;

        response.into_result()
    }

    /// 获取小程序Short Link
    pub async fn generate_short_link(
        &self,
        request: &ShortLinkRequest,
    ) -> LabradorResult<ShortLinkResponse> {
        let response: WechatApiResponse<ShortLinkResponse> = self
            .client
            .wechat_client()
            .post("/wxa/genwxashortlink", RequestBody::from(request))
            .await?;

        response.into_result()
    }
}

/// 小程序码请求
#[derive(Debug, Clone, Serialize)]
pub struct WxacodeRequest {
    /// 扫码进入的小程序页面路径，最大长度 128 字节，不能为空
    pub path: String,
    /// 二维码的宽度，单位 px。最小 280px，最大 1280px
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    /// 自动配置线条颜色，如果颜色依然是黑色，则说明不建议配置主色调
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_color: Option<bool>,
    /// auto_color 为 false 时生效，使用 rgb 设置颜色 例如 {"r":"xxx","g":"xxx","b":"xxx"} 十进制表示
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_color: Option<LineColor>,
    /// 是否需要透明底色，为 true 时，生成透明底色的小程序码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_hyaline: Option<bool>,
}

impl WxacodeRequest {
    /// 创建新的小程序码请求
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            width: None,
            auto_color: None,
            line_color: None,
            is_hyaline: None,
        }
    }

    /// 设置宽度
    pub fn with_width(mut self, width: i32) -> Self {
        self.width = Some(width);
        self
    }

    /// 设置自动颜色
    pub fn with_auto_color(mut self, auto_color: bool) -> Self {
        self.auto_color = Some(auto_color);
        self
    }

    /// 设置线条颜色
    pub fn with_line_color(mut self, r: &str, g: &str, b: &str) -> Self {
        self.line_color = Some(LineColor {
            r: r.to_string(),
            g: g.to_string(),
            b: b.to_string(),
        });
        self
    }

    /// 设置透明底色
    pub fn with_is_hyaline(mut self, is_hyaline: bool) -> Self {
        self.is_hyaline = Some(is_hyaline);
        self
    }
}

/// 线条颜色
#[derive(Debug, Clone, Serialize)]
pub struct LineColor {
    /// 红色值
    pub r: String,
    /// 绿色值
    pub g: String,
    /// 蓝色值
    pub b: String,
}

/// 小程序码（无限制）请求
#[derive(Debug, Clone, Serialize)]
pub struct WxacodeUnlimitedRequest {
    /// 最大32个可见字符，只支持数字，大小写英文以及部分特殊字符：!#$&'()*+,/:;=?@-._~，其它字符请自行编码为合法字符
    pub scene: String,
    /// 必须是已经发布的小程序存在的页面（否则报错），例如 pages/index/index, 根路径前不要填加 /,不能携带参数（参数请放在scene字段里），如果不填写这个字段，默认跳主页面
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    /// 二维码的宽度，单位 px。最小 280px，最大 1280px
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    /// 自动配置线条颜色，如果颜色依然是黑色，则说明不建议配置主色调
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_color: Option<bool>,
    /// auto_color 为 false 时生效，使用 rgb 设置颜色 例如 {"r":"xxx","g":"xxx","b":"xxx"} 十进制表示
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_color: Option<LineColor>,
    /// 是否需要透明底色，为 true 时，生成透明底色的小程序码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_hyaline: Option<bool>,
    /// 检查page 是否存在，为 true 时 page 必须是已经发布的小程序存在的页面（否则报错）；为 false 时允许小程序未发布或者 page 不存在， 但page 有数量上限（60000个）请勿滥用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_path: Option<bool>,
    /// 要打开的小程序版本。正式版为 "release"，体验版为 "trial"，开发版为 "develop"。默认是正式版。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_version: Option<String>,
}

impl WxacodeUnlimitedRequest {
    /// 创建新的无限制小程序码请求
    pub fn new(scene: &str) -> Self {
        Self {
            scene: scene.to_string(),
            page: None,
            width: None,
            auto_color: None,
            line_color: None,
            is_hyaline: None,
            check_path: None,
            env_version: None,
        }
    }

    /// 设置页面
    pub fn with_page(mut self, page: &str) -> Self {
        self.page = Some(page.to_string());
        self
    }

    /// 设置宽度
    pub fn with_width(mut self, width: i32) -> Self {
        self.width = Some(width);
        self
    }

    /// 设置自动颜色
    pub fn with_auto_color(mut self, auto_color: bool) -> Self {
        self.auto_color = Some(auto_color);
        self
    }

    /// 设置线条颜色
    pub fn with_line_color(mut self, r: &str, g: &str, b: &str) -> Self {
        self.line_color = Some(LineColor {
            r: r.to_string(),
            g: g.to_string(),
            b: b.to_string(),
        });
        self
    }

    /// 设置透明底色
    pub fn with_is_hyaline(mut self, is_hyaline: bool) -> Self {
        self.is_hyaline = Some(is_hyaline);
        self
    }

    /// 设置检查路径
    pub fn with_check_path(mut self, check_path: bool) -> Self {
        self.check_path = Some(check_path);
        self
    }

    /// 设置环境版本
    pub fn with_env_version(mut self, env_version: &str) -> Self {
        self.env_version = Some(env_version.to_string());
        self
    }
}

/// 小程序二维码请求
#[derive(Debug, Clone, Serialize)]
pub struct WxaqrcodeRequest {
    /// 扫码进入的小程序页面路径，最大长度 128 字节，不能为空
    pub path: String,
    /// 二维码的宽度，单位 px。最小 280px，最大 1280px
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}

impl WxaqrcodeRequest {
    /// 创建新的小程序二维码请求
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            width: None,
        }
    }

    /// 设置宽度
    pub fn with_width(mut self, width: i32) -> Self {
        self.width = Some(width);
        self
    }
}

/// URL Scheme请求
#[derive(Debug, Clone, Serialize)]
pub struct UrlSchemeRequest {
    /// 跳转到的目标小程序信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jump_wxa: Option<JumpWxa>,
    /// 生成的scheme码类型，到期失效：true，永久有效：false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_expire: Option<bool>,
    /// 到期失效的scheme码的失效时间，为Unix时间戳。生成的到期失效scheme码在该时间前有效。最长有效期为1年。生成到期失效的scheme时必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<i64>,
}

impl UrlSchemeRequest {
    /// 创建新的URL Scheme请求
    pub fn new() -> Self {
        Self {
            jump_wxa: None,
            is_expire: None,
            expire_time: None,
        }
    }

    /// 设置跳转小程序
    pub fn with_jump_wxa(mut self, path: &str, query: Option<&str>) -> Self {
        self.jump_wxa = Some(JumpWxa {
            path: path.to_string(),
            query: query.map(|s| s.to_string()),
        });
        self
    }

    /// 设置是否到期失效
    pub fn with_is_expire(mut self, is_expire: bool) -> Self {
        self.is_expire = Some(is_expire);
        self
    }

    /// 设置到期时间
    pub fn with_expire_time(mut self, expire_time: i64) -> Self {
        self.expire_time = Some(expire_time);
        self
    }
}

impl Default for UrlSchemeRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// 跳转小程序
#[derive(Debug, Clone, Serialize)]
pub struct JumpWxa {
    /// 通过scheme码进入的小程序页面路径，必须是已经发布的小程序存在的页面，不可携带query。path为空时会跳转小程序主页
    pub path: String,
    /// 通过scheme码进入小程序时的query，最大1024个字符，只支持数字，大小写英文以及部分特殊字符：!#$&'()*+,/:;=?@-._~
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

/// URL Scheme响应
#[derive(Debug, Clone, Deserialize)]
pub struct UrlSchemeResponse {
    /// 错误码
    pub errcode: i32,
    /// 错误信息
    pub errmsg: String,
    /// 生成的小程序scheme码
    pub openlink: Option<String>,
}

/// URL Link请求
#[derive(Debug, Clone, Serialize)]
pub struct UrlLinkRequest {
    /// 跳转到的目标小程序信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// 通过URL Link进入小程序时的query，最大1024个字符，只支持数字，大小写英文以及部分特殊字符：!#$&'()*+,/:;=?@-._~
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// 生成的URL Link类型，到期失效：true，永久有效：false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_expire: Option<bool>,
    /// 到期失效的URL Link的失效时间，为Unix时间戳。生成的到期失效URL Link在该时间前有效。最长有效期为1年。生成到期失效的URL Link时必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<i64>,
    /// 云开发静态网站自定义H5配置参数，可配置中转的云开发H5页面。不填默认用官方H5页面
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_base: Option<CloudBase>,
}

impl UrlLinkRequest {
    /// 创建新的URL Link请求
    pub fn new() -> Self {
        Self {
            path: None,
            query: None,
            is_expire: None,
            expire_time: None,
            cloud_base: None,
        }
    }

    /// 设置路径
    pub fn with_path(mut self, path: &str) -> Self {
        self.path = Some(path.to_string());
        self
    }

    /// 设置查询参数
    pub fn with_query(mut self, query: &str) -> Self {
        self.query = Some(query.to_string());
        self
    }

    /// 设置是否到期失效
    pub fn with_is_expire(mut self, is_expire: bool) -> Self {
        self.is_expire = Some(is_expire);
        self
    }

    /// 设置到期时间
    pub fn with_expire_time(mut self, expire_time: i64) -> Self {
        self.expire_time = Some(expire_time);
        self
    }

    /// 设置云开发配置
    pub fn with_cloud_base(mut self, env: &str, domain: &str, path: &str, query: &str) -> Self {
        self.cloud_base = Some(CloudBase {
            env: env.to_string(),
            domain: Some(domain.to_string()),
            path: path.to_string(),
            query: Some(query.to_string()),
        });
        self
    }
}

impl Default for UrlLinkRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// 云开发配置
#[derive(Debug, Clone, Serialize)]
pub struct CloudBase {
    /// 云开发环境
    pub env: String,
    /// 静态网站自定义域名，不填则使用默认域名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 云开发静态网站H5页面路径，不可携带query
    pub path: String,
    /// 云开发静态网站H5页面query参数，最大1024个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

/// URL Link响应
#[derive(Debug, Clone, Deserialize)]
pub struct UrlLinkResponse {
    /// 错误码
    pub errcode: i32,
    /// 错误信息
    pub errmsg: String,
    /// 生成的小程序URL Link
    pub url_link: Option<String>,
}

/// Short Link请求
#[derive(Debug, Clone, Serialize)]
pub struct ShortLinkRequest {
    /// 通过Short Link进入的小程序页面路径，必须是已经发布的小程序存在的页面，可携带query，最大1024个字符
    pub page_url: String,
    /// 页面标题，不能包含违法信息，超过20字符会用...截断
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_title: Option<String>,
    /// 是否永久有效，默认false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_permanent: Option<bool>,
}

impl ShortLinkRequest {
    /// 创建新的Short Link请求
    pub fn new(page_url: &str) -> Self {
        Self {
            page_url: page_url.to_string(),
            page_title: None,
            is_permanent: None,
        }
    }

    /// 设置页面标题
    pub fn with_page_title(mut self, page_title: &str) -> Self {
        self.page_title = Some(page_title.to_string());
        self
    }

    /// 设置是否永久有效
    pub fn with_is_permanent(mut self, is_permanent: bool) -> Self {
        self.is_permanent = Some(is_permanent);
        self
    }
}

/// Short Link响应
#[derive(Debug, Clone, Deserialize)]
pub struct ShortLinkResponse {
    /// 错误码
    pub errcode: i32,
    /// 错误信息
    pub errmsg: String,
    /// 生成的小程序Short Link
    pub link: Option<String>,
}
