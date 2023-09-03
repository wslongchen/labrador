# Labrador &emsp; [![Docs][docs-image]][docs-url] [![Build Status]][actions] [![Latest Version]][crates.io] [![labrador: rustc 1.13+]][Rust 1.13]

[Build Status]: https://img.shields.io/docsrs/labrador/0.2.0?style=plastic
[actions]: https://github.com/wslongchen/labrador/actions?query=branch%3Amaster
[Latest Version]: https://img.shields.io/crates/v/labrador?style=plastic
[crates.io]: https://crates.io/crates/labrador
[labrador: rustc 1.13+]: https://img.shields.io/badge/labrador-rustc__1.31%2B-lightgrey
[Rust 1.13]: https://blog.rust-lang.org/2016/11/10/Rust-1.13.html
[Rust 1.31]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html
[docs-image]: https://img.shields.io/badge/文档-中文-blue.svg
[docs-url]: https://github.com/wslongchen/labrador/blob/master/README_CN.md
```Labrador - Mini client for rust ```


# This create offers:

*   A convenient mainstream third-party service client
*   Convenient and quick use of corresponding services in rust

Features:

*   ```taobao``` - Taobao customer related services
*   ```alipay``` - Alipay related services
*   ```pdd``` - Pinduoduo related services
*   ```jd``` - Jingdong related services
*   ```wechat``` - Wechat related services
*   ```qiniu``` - Qiniu OSS services

### Crypto Features

+ openssl-crypto
If you enable `openssl-crypto` feature.
All encryption in this project is done with openssl. Later, other encryption databases will be supported for selection. Therefore, openssl needs to be introduced. See [openssl] for details.

[openssl]: https://docs.rs/openssl/0.10.41/openssl/

+ default crpto

the default encryption in this project is [rust-crypto] & [x509_parser] & [rsa]

[rust-crypto]: https://docs.rs/rust-crypto/0.2.36/crypto/
[x509_parser]: https://docs.rs/x509-parser/0.14.0/x509_parser/
[rsa]: https://docs.rs/rsa/0.6.1/rsa/

### Supported Platform

| Platform                                           | is supported |
|----------------------------------------------------|--------------|
| Wechat:mp(微信公众号),cp(企业微信),miniapp(微信小程序),pay(微信支付) | √            | 
| Alipay(支付宝)                                        | √            |  
| Taobao(淘宝客)                                        | √            |
| JD(京东联盟)                                           | √            |  
| PDD(拼多多-多多客)                                       | √            |
| Qiniu(七牛云存储)                                       | √            |


---

You may be looking for:

- [An overview of Labrador](https://crates.io/crates/labrador)
- [Examples](https://github.com/wslongchen/labrador/blob/0.1.0/example/simple.rs)
- [API documentation](https://docs.rs/labrador/0.1.0/labrador/)
- [Release notes](https://github.com/wslongchen/labrador/releases)

## Labrador in action

<details>
<summary>
Click to show Cargo.toml.
<a href="https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018&gist=93bca9fced54f62eb69a2f2a224715c5" target="_blank">Run this code in the playground.</a>
</summary>

```toml
[dependencies]

# The core APIs
labrador = { version = "0.2.0", features = ["wechat", "alipay"] }

```

</details>
<p></p>

## Example

### 猫狗0791商城小程序
 
猫狗0791商城小程序中微信、支付宝等支付、退款功能均基于Labrador，欢迎扫码体验

![小程序](http://img.snackcloud.cn/snackcloud/shop/WechatIMG4566.jpeg)


## API Documentation

### With Wechat（微信开放平台、包含微信支付）

 ```rust
use labrador::{WechatPayClient, SimpleStorage, TradeType, WechatPayRequestV3, Amount, Payer};
use chrono::{Local, SecondsFormat};

 #[tokio::main]
 async fn main() {
     let c =  WechatPayClient::new("appid", "secret", SimpleStorage::new());
     let mut client =c.wxpay();
     let date = Local::now().to_rfc3339_opts(SecondsFormat::Secs, false);
     let result = client.unified_order_v3(TradeType::Jsapi, WechatPayRequestV3 {
         appid: "appid".to_string().into(),
         mch_id: "mchid".to_string(),
         description: "测试商品支付".to_string(),
         out_trade_no: "1602920235sdfsdfas32234234".to_string(),
         time_expire: date,
         attach: None,
         notify_url: "https:xxx.cn/trade/notify".to_string(),
         amount: Amount {
             total: 1,
             currency: String::from("CNY").into(),
             payer_total: None,
             payer_currency: None
         },
         payer: Payer {
             openid: "oUVZc6S_uGx3bsNPUA-davo4Dt7Us".to_string()
         }.into(),
         detail: None,
         scene_info: None,
         settle_info: None
     });
     match result.await {
         Ok(res) => {}
         Err(err) => {}
     }
 }
 ```

### With Alipay（支付宝）

 ```rust
use labrador::{AlipayTradeWapPayRequest, AlipayClient};

 #[tokio::main]
 async fn main() {
     let param = AlipayTradeWapPayRequest::default();
     let client = AlipayClient::new("appKey", false);
     match client.wap_pay("POST".into(), param).await {
         Ok(res) => {}
         Err(err) => {}
     }
     match result.await {
         Ok(res) => {}
         Err(err) => {}
     }
 }
 ```

### With Taobao（淘宝客相关）

 ```rust
use labrador::{TbItemDetailRequest, TaobaoClient};

 #[tokio::main]
 async fn main() {
     let client =  TaobaoClient::<SimpleStorage>::new("appkey", "secret");
     let req = TbItemDetailRequest {
         num_iids: Some("597649283190".to_string()),
         platform: None,
         ip: None
     };
     let result = client.get_item_detail(req);
     match result.await {
         Ok(res) => {
         }
         Err(err) => {
         }
     }
 }
 ```


### With JD（京东，目前暂时只支持联盟相关）

 ```rust
use labrador::{JDClient, JdOrderRawQueryParam};
use chrono::{Local, SecondsFormat};

 #[tokio::main]
 async fn main() {
     let client =  JDClient::<SimpleStorage>::new("appkey", "secert");
     let param = JdOrderRawQueryParam {
         page_index: 1.into(),
         page_size: 10.into(),
         bill_type: 1,
         start_time: "2022-08-02 21:23:00".to_string(),
         end_time: "2022-08-02 21:43:00".to_string(),
         child_union_id: None,
         key: None,
         fields: None
     };
     let result = client.query_raw_order(param);
     match result.await {
         Ok(res) => {
         }
         Err(err) => {
         }
     }
 }
 ```

### With Custom Request

You can implement this trait and then use the custom request

+ AlipayRequest - For Alipay(支付宝)
+ JDRequest - For jingdong(京东)
+ TaobaoRequest - For taobao(淘宝)


## Feature

We will gradually improve the corresponding API


## Developing

To setup the development envrionment run `cargo run`.

## Contributers

MrPan <1049058427@qq.com>

## Getting help

Labrador is a personal project. At the beginning, I just like Labrador dog because of my hobbies.
I hope this project will grow more and more lovely. Many practical database functions will
be added in the future. I hope you can actively help this project grow and put forward suggestions.
I believe the future will be better and better.

[#general]: https://discord.com/channels/273534239310479360/274215136414400513
[#beginners]: https://discord.com/channels/273534239310479360/273541522815713281
[#rust-usage]: https://discord.com/channels/442252698964721669/443150878111694848
[zulip]: https://rust-lang.zulipchat.com/#narrow/stream/122651-general
[stackoverflow]: https://stackoverflow.com/questions/tagged/rust
[/r/rust]: https://www.reddit.com/r/rust
[discourse]: https://users.rust-lang.org

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