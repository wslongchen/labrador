# Labrador &emsp; [![Docs][docs-image]][docs-url] [![CI][ci-badge]][ci-url] [![Latest Version]][crates.io] [![labrador: rustc 1.31+]][Rust 1.31]

<p align="center">
  <img src="labrador.png" alt="Labrador" width="160">
</p>

> **Labrador** — 轻量级、易用的 Rust 第三方平台服务客户端 SDK。

[ci-badge]: https://img.shields.io/github/actions/workflow/status/woofcloud/labrador/ci.yml?branch=master&style=plastic
[ci-url]: https://github.com/woofcloud/labrador/actions/workflows/ci.yml
[Latest Version]: https://img.shields.io/crates/v/labrador?style=plastic
[crates.io]: https://crates.io/crates/labrador
[labrador: rustc 1.31+]: https://img.shields.io/badge/labrador-rustc__1.31%2B-lightgrey
[Rust 1.31]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html
[docs-image]: https://img.shields.io/badge/docs-英文-blue.svg
[docs-url]: https://github.com/woofcloud/labrador/blob/master/README.md

## 什么是 Labrador？

Labrador 为 Rust 提供了一套统一、易用的主流第三方平台服务 SDK，覆盖微信、支付宝、京东、拼多多、淘宝、七牛等平台，让你专注于业务逻辑而不是 API 集成。

### 支持的服务

| 服务              | 说明                                       |
|------------------|--------------------------------------------|
| `wechat`         | 微信公众号 / 小程序 / 企业微信 / 开放平台 / 支付 |
| `alipay`         | 支付宝支付 / 小程序 / 开放平台              |
| `taobao`         | 淘宝客                                     |
| `jd`             | 京东联盟                                   |
| `pdd`            | 拼多多-多多客                              |
| `qiniu`          | 七牛云对象存储                              |

### 加密后端

| Feature            | 后端                                         |
|--------------------|----------------------------------------------|
| `openssl-crypto`   | [OpenSSL](https://docs.rs/openssl/)          |
| (默认)              | [rust-crypto](https://docs.rs/rust-crypto/) + [x509-parser](https://docs.rs/x509-parser/) + [rsa](https://docs.rs/rsa/) |

启用方式：

```toml
[dependencies]
labrador = { version = "0.3.0", features = ["wechat", "alipay", "openssl-crypto"] }
```

### 平台覆盖

| 平台                   | 模块                                        | 状态 |
|-----------------------|---------------------------------------------|------|
| 微信 (WeChat)          | 公众号 / 小程序 / 企业微信 / 开放平台 / 支付    | ✅   |
| 支付宝 (Alipay)         | 支付 / 小程序 / 开放平台                      | ✅   |
| 淘宝客 (Taobao)         | 商品 / 订单 / 推广                           | ✅   |
| 京东联盟 (JD)            | 订单查询                                     | ✅   |
| 拼多多-多多客 (PDD)       | 商品 / 订单 / 推广                           | ✅   |
| 七牛云存储 (Qiniu)        | 对象存储                                     | ✅   |


---

如下:

- [An overview of Labrador](https://crates.io/crates/labrador)
- [Examples](https://github.com/woofcloud/labrador/blob/master/examples/simple.rs)
- [API documentation](https://docs.rs/labrador/0.3.0/labrador/)
- [Release notes](https://github.com/woofcloud/labrador/releases)

## 快速开始

在 `Cargo.toml` 中添加：

```toml
[dependencies]
labrador = { version = "0.3.0", features = ["wechat", "alipay"] }
```

## 生产案例

**[BorderCollie](https://review.woofcloud.com)** 是一款代码审查与修复协作平台。其付费订阅中的微信支付、支付宝支付均基于 Labrador 实现，是 Labrador 在生产环境中的真实应用案例。

> 访问 [review.woofcloud.com](https://review.woofcloud.com) 即可体验。

## 文档

### 微信公众号 (mp)

```rust
use labrador::wechat::mp::{WechatMpClientBuilder, Menu, Button};

#[tokio::main]
async fn main() {
    let client = WechatMpClientBuilder::new()
        .app_id("wxAPPID")
        .app_secret("APPSECRET")
        .build()
        .unwrap();

    // 创建自定义菜单
    let menu = Menu::new(vec![
        Button::click("点击按钮", "CLICK_KEY"),
        Button::view("跳转网页", "https://www.example.com"),
    ]);
    client.menu().create_menu(&menu).await.unwrap();

    // 获取用户列表
    let users = client.user().get_user_list(None).await.unwrap();
    println!("关注用户数: {}", users.total);
}
```

### 微信小程序 (miniapp)

```rust
use labrador::wechat::miniapp::WechatMiniAppClientBuilder;
use labrador::wechat::miniapp::api::WxacodeUnlimitedRequest;

#[tokio::main]
async fn main() {
    let client = WechatMiniAppClientBuilder::new()
        .app_id("wxAPPID")
        .app_secret("APPSECRET")
        .build()
        .unwrap();

    // 登录凭证校验
    let session = client.auth().code2session("login_code").await.unwrap();
    println!("openid: {}", session.openid);

    // 获取小程序码
    let qrcode = client.qrcode().get_wxacode_unlimited(&WxacodeUnlimitedRequest {
        scene: "test".to_string(),
        page: None, check_path: Some(false), env_version: None,
        width: Some(430), auto_color: Some(false),
        line_color: None, is_hyaline: Some(false),
    }).await.unwrap();
}
```

### 企业微信 (cp)

```rust
use labrador::wechat::cp::WechatCpClientBuilder;

#[tokio::main]
async fn main() {
    let client = WechatCpClientBuilder::new()
        .corp_id("CORPID")
        .corp_secret("CORPSECRET")
        .build()
        .unwrap();

    // 获取部门列表
    let depts = client.department().list(None).await.unwrap();

    // 发送文本消息
    client.message().send_text("userid1|userid2", None, None, 0, "Hello").await.unwrap();
}
```

### 微信开放平台 (open)

```rust
use labrador::wechat::open::WechatOpenClientBuilder;

#[tokio::main]
async fn main() {
    let client = WechatOpenClientBuilder::new()
        .app_id("component_appid")
        .app_secret("component_appsecret")
        .token("TOKEN")
        .encoding_aes_key("AESKEY")
        .build()
        .unwrap();

    // 获取第三方平台 component_access_token
    let token = client.component().get_component_access_token("ticket").await.unwrap();
}
```

### 支付宝 - 支付

```rust
use labrador::alipay::{AlipayClientBuilder, AlipayBizRequest};
use labrador::platforms::alipay::pay::AlipayTradePreCreateModel;

#[tokio::main]
async fn main() {
    let client = AlipayClientBuilder::new()
        .app_id("2021xxxxxxxxxx")
        .app_private_key("YOUR_PRIVATE_KEY")
        .alipay_public_key("ALIPAY_PUBLIC_KEY")
        .sign_type("RSA2")
        .build()
        .unwrap();

    // 预下单生成二维码
    let mut req = AlipayBizRequest::new();
    req.set_biz_model(AlipayTradePreCreateModel {
        out_trade_no: "ORDER001".to_string(),
        total_amount: 0.01,
        subject: "测试商品".to_string(),
        product_code: "FACE_TO_FACE_PAYMENT".to_string(),
        ..Default::default()
    });
    let resp = client.alipay_service().precreate(req).await.unwrap();
    println!("二维码: {}", resp.qr_code.unwrap());
}
```

### 支付宝 - 小程序 & 开放平台

```rust
use labrador::alipay::AlipayClientBuilder;
use std::collections::BTreeMap;
use labrador::platforms::alipay::miniapp::TemplateDataItem;

#[tokio::main]
async fn main() {
    let client = AlipayClientBuilder::new()
        .app_id("2021xxxxxxxxxx")
        .app_private_key("YOUR_PRIVATE_KEY")
        .alipay_public_key("ALIPAY_PUBLIC_KEY")
        .sign_type("RSA2")
        .build()
        .unwrap();

    // 发送模板消息
    let mut data = BTreeMap::new();
    data.insert("keyword1".to_string(), TemplateDataItem {
        value: "测试内容".to_string(), color: None,
    });
    client.miniapp().send_template_message(
        "2088xxxxx".to_string(), "form_id", "template_id",
        None, data, None,
    ).await.unwrap();
}
```

> 更多完整 API 文档请查看 [docs.rs/labrador](https://docs.rs/labrador/)。每个方法都包含详细的参数说明、返回值说明和官方文档链接。


## 参与贡献

我们欢迎所有形式的贡献！请向 `develop` 分支提交 PR。

- 代码风格：2 空格缩进
- 提交前确保 `cargo fmt` 和 `cargo clippy` 通过
- 为新增的 pub 方法编写单元测试

## 贡献者

- **MrPan** <1049058427@qq.com>

## 社区

- [GitHub Issues](https://github.com/woofcloud/labrador/issues)
- [GitHub Discussions](https://github.com/woofcloud/labrador/discussions)

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Labrador by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>

<br>

<p align="center">
  <img src="woofcloud_logo.png" alt="WoofCloud" width="160">
  <br>
  <strong>WoofCloud 出品</strong>
  <br>
  <sub>热爱技术，热爱生活 · 欢迎一起学习、交流与贡献</sub>
</p>
