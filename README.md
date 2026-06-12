# Labrador &emsp; [![Docs][docs-image]][docs-url] [![CI][ci-badge]][ci-url] [![Latest Version]][crates.io] [![labrador: rustc 1.31+]][Rust 1.31]

<p align="center">
  <img src="labrador.png" alt="Labrador" width="160">
</p>

> **Labrador** — A lightweight, ergonomic Rust client SDK for third-party platform services.

[ci-badge]: https://img.shields.io/github/actions/workflow/status/woofcloud/labrador/ci.yml?branch=master&style=plastic
[ci-url]: https://github.com/woofcloud/labrador/actions/workflows/ci.yml
[Latest Version]: https://img.shields.io/crates/v/labrador?style=plastic
[crates.io]: https://crates.io/crates/labrador
[labrador: rustc 1.31+]: https://img.shields.io/badge/labrador-rustc__1.31%2B-lightgrey
[Rust 1.31]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html
[docs-image]: https://img.shields.io/badge/docs-中文-blue.svg
[docs-url]: https://github.com/woofcloud/labrador/blob/master/README_CN.md

## What is Labrador?

Labrador provides a unified, ergonomic Rust SDK for mainstream third-party platform services — including WeChat, Alipay, JD, Pinduoduo, Taobao, and Qiniu — so you can focus on business logic instead of API integration.

### Supported Services

| Service          | Description                    |
|------------------|--------------------------------|
| `wechat`         | WeChat Official Accounts, Mini Programs, WeCom, Open Platform, Payment |
| `alipay`         | Alipay Payment, Mini Programs, Open Platform |
| `taobao`         | Taobao Affiliate (Taoke)       |
| `jd`             | JD Affiliate (Jingdong Union)  |
| `pdd`            | Pinduoduo Affiliate (Duoduo Ke) |
| `qiniu`          | Qiniu Cloud Object Storage     |

### Crypto Backend

| Feature            | Backend                                      |
|--------------------|----------------------------------------------|
| `openssl-crypto`   | [OpenSSL](https://docs.rs/openssl/)          |
| (default)          | [rust-crypto](https://docs.rs/rust-crypto/) + [x509-parser](https://docs.rs/x509-parser/) + [rsa](https://docs.rs/rsa/) |

Enable with:

```toml
[dependencies]
labrador = { version = "0.3.0", features = ["wechat", "alipay", "openssl-crypto"] }
```

### Platform Coverage

| Platform              | Modules                                                | Status |
|-----------------------|--------------------------------------------------------|--------|
| WeChat (微信)          | Official Accounts / Mini Programs / WeCom / Open / Pay | ✅     |
| Alipay (支付宝)         | Payment / Mini Programs / Open Platform                | ✅     |
| Taobao (淘宝客)         | Products / Orders / Promotion                          | ✅     |
| JD (京东联盟)            | Order Query                                            | ✅     |
| PDD (拼多多-多多客)       | Products / Orders / Promotion                          | ✅     |
| Qiniu (七牛云存储)        | Object Storage                                         | ✅     |


---

You may be looking for:

- [An overview of Labrador](https://crates.io/crates/labrador)
- [Examples](https://github.com/woofcloud/labrador/blob/master/examples/simple.rs)
- [API documentation](https://docs.rs/labrador/0.3.0/labrador/)
- [Release notes](https://github.com/woofcloud/labrador/releases)

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
labrador = { version = "0.3.0", features = ["wechat", "alipay"] }
```

## Labrador in Production

**[BorderCollie](https://review.woofcloud.com)** is a code review & fix collaboration platform. Its paid subscription features — including WeChat Pay and Alipay — are built on top of Labrador, making it a real-world production use case.

> Try it at [review.woofcloud.com](https://review.woofcloud.com).

## API Documentation

### WeChat Official Accounts (mp)

```rust
use labrador::wechat::mp::{WechatMpClientBuilder, Menu, Button};

#[tokio::main]
async fn main() {
    let client = WechatMpClientBuilder::new()
        .app_id("wxAPPID")
        .app_secret("APPSECRET")
        .build()
        .unwrap();

    // Create custom menu
    let menu = Menu::new(vec![
        Button::click("Click Me", "CLICK_KEY"),
        Button::view("Open URL", "https://www.example.com"),
    ]);
    client.menu().create_menu(&menu).await.unwrap();

    // Get user list
    let users = client.user().get_user_list(None).await.unwrap();
    println!("Total followers: {}", users.total);
}
```

### WeChat Mini Programs (miniapp)

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

    // Code-to-session
    let session = client.auth().code2session("login_code").await.unwrap();
    println!("openid: {}", session.openid);

    // Get unlimited wxacode
    let qrcode = client.qrcode().get_wxacode_unlimited(&WxacodeUnlimitedRequest {
        scene: "test".to_string(),
        page: None, check_path: Some(false), env_version: None,
        width: Some(430), auto_color: Some(false),
        line_color: None, is_hyaline: Some(false),
    }).await.unwrap();
}
```

### WeCom (cp)

```rust
use labrador::wechat::cp::WechatCpClientBuilder;

#[tokio::main]
async fn main() {
    let client = WechatCpClientBuilder::new()
        .corp_id("CORPID")
        .corp_secret("CORPSECRET")
        .build()
        .unwrap();

    // List departments
    let depts = client.department().list(None).await.unwrap();

    // Send text message
    client.message().send_text("userid1|userid2", None, None, 0, "Hello").await.unwrap();
}
```

### WeChat Open Platform (open)

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

    // Get component_access_token
    let token = client.component().get_component_access_token("ticket").await.unwrap();
}
```

### Alipay — Payment

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

    // Pre-create order (generate QR code)
    let mut req = AlipayBizRequest::new();
    req.set_biz_model(AlipayTradePreCreateModel {
        out_trade_no: "ORDER001".to_string(),
        total_amount: 0.01,
        subject: "Test Product".to_string(),
        product_code: "FACE_TO_FACE_PAYMENT".to_string(),
        ..Default::default()
    });
    let resp = client.alipay_service().precreate(req).await.unwrap();
    println!("QR Code: {}", resp.qr_code.unwrap());
}
```

### Alipay — Mini Programs & Open Platform

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

    // Mini program — send template message
    let mut data = BTreeMap::new();
    data.insert("keyword1".to_string(), TemplateDataItem {
        value: "Test content".to_string(), color: None,
    });
    client.miniapp().send_template_message(
        "2088xxxxx".to_string(), "form_id", "template_id",
        None, data, None,
    ).await.unwrap();
}
```

> For the full API reference, see [docs.rs/labrador](https://docs.rs/labrador/).


## Contributing

We welcome contributions! Please submit PRs to the `develop` branch.

- Code style: 2-space indentation
- Ensure `cargo fmt` and `cargo clippy` pass before submitting
- Add unit tests for new public methods

## Contributors

- **MrPan** <1049058427@qq.com>

## Community

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
