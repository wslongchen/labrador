# Labrador &emsp; [![Docs][docs-image]][docs-url] [![Build Status]][actions] [![Latest Version]][crates.io] [![labrador: rustc 1.13+]][Rust 1.13]

<div>
    <p align="center">
      <img src="http://img.snackcloud.cn/snackcloud/shop/snack_logo.png" alt="猫狗试验室" width="248" height="248">
    </p>
    <p align="center"><strong>猫狗试验室出品</strong></p>
    <p align="center">
      这是一个热爱🫶技术，热爱🔥生活的团队. <br>很欢迎大家能够一起学习📑、沟通💬 .
    </p>
  </div>

[Build Status]: https://img.shields.io/docsrs/labrador/0.1.0?style=plastic
[actions]: https://github.com/wslongchen/labrador/actions?query=branch%3Amaster
[Latest Version]: https://img.shields.io/crates/v/labrador?style=plastic
[crates.io]: https://crates.io/crates/labrador
[labrador: rustc 1.13+]: https://img.shields.io/badge/labrador-rustc__1.31%2B-lightgrey
[Rust 1.13]: https://blog.rust-lang.org/2016/11/10/Rust-1.13.html
[Rust 1.31]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html
[docs-image]: https://img.shields.io/badge/文档-英文-blue.svg
[docs-url]: https://github.com/wslongchen/labrador/blob/master/README.md
```Labrador - 一个迷你的便捷的第三方服务客户端SDK ```


Features:

*   ```taobao``` - 淘宝客
*   ```alipay``` - 支付宝
*   ```pdd``` - 拼多多
*   ```jd``` - 京东
*   ```wechat``` - 微信

### Crypto Features

+ openssl-crypto
  如果你开起来了 `openssl-crypto` feature，
  则本项目所有加密都是用openssl进行，后续会支持其他加密库以供挑选，所以需要引入openssl相关，详情见[openssl]相关说明。

[openssl]: https://docs.rs/openssl/0.10.41/openssl/

+ 默认加密

默认加密主要采用 [rust-crypto] & [x509_parser] & [rsa]，感兴趣的可以去相关文档查看。

[rust-crypto]: https://docs.rs/rust-crypto/0.2.36/crypto/
[x509_parser]: https://docs.rs/x509-parser/0.14.0/x509_parser/
[rsa]: https://docs.rs/rsa/0.6.1/rsa/

### Supported Platform

| 平台                                                 | 是否支持 |
|----------------------------------------------------|------|
| Wechat:mp(微信公众号),cp(企业微信),miniapp(微信小程序),pay(微信支付) | √    | 
| Alipay(支付宝)                                        | √    |  
| Taobao(淘宝客)                                        | √    |
| JD(京东联盟)                                           | √    |  
| PDD(拼多多-多多客)                                       | √    |


---

如下:

- [An overview of Labrador](https://crates.io/crates/labrador)
- [Examples](https://github.com/wslongchen/labrador/blob/0.1.0/example/simple.rs)
- [API documentation](https://docs.rs/labrador/0.1.0/labrador/)
- [Release notes](https://github.com/wslongchen/labrador/releases)

## 使用

<details>
<summary>
Click to show Cargo.toml.
<a href="https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018&gist=93bca9fced54f62eb69a2f2a224715c5" target="_blank">Run this code in the playground.</a>
</summary>

```toml
[dependencies]

# The core APIs
labrador = { version = "0.1.0", features = ["wechat", "alipay"] }

```

</details>
<p></p>

## 示例

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <img src="http://img.snackcloud.cn/snackcloud/shop/WechatIMG4566.jpeg" style="width:208px;height:208px;">
        <p><strong>猫狗0791商城小程序</strong></p>
        <p>小程序中微信、支付宝等支付、<br/>退款功能均基于Labrador，欢迎扫码体验</p>
      </td>
      <td align="center" valign="middle">
        <img src="https://mp.weixin.qq.com/mp/qrcode?scene=10000005&size=102&__biz=MzkxNTE0MjczNw==&mid=2247483672&idx=1&sn=2982f8afbb126b401e04e921cb582874&send_time=" style="width:208px;height:208px;">
        <p><strong>猫狗试验室</strong></p>
        <p>欢迎扫码加入我们</p>
      </td>
    </tr>
  </tbody>
</table>
## 文档
### 微信开放平台、包含微信支付

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

### 支付宝

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

### 淘宝客相关

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


### 京东，目前暂时只支持联盟相关

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

### 自定义请求

You can implement this trait and then use the custom request

+ AlipayRequest - 支付宝
+ JDRequest - 京东
+ TaobaoRequest - 淘宝


## 未来

我们将逐步完善相应的API
1. 首先非常欢迎和感谢对本项目发起 `Pull Request` 的热心小伙伴们。
1. **特别提示：请务必在 `develop` 分支提交 `PR`，`release` 分支目前仅是正式版的代码，即发布正式版本后才会从 `develop` 分支进行合并。**
1. 本项目代码风格为使用2个空格代表一个Tab，因此在提交代码时请注意一下，否则很容易在IDE格式化代码后与原代码产生大量diff，这样会给其他人阅读代码带来极大的困扰。
1. **提交代码前，请检查代码是否已经格式化，并且保证新增加或者修改的方法都有完整的参数说明，而pub方法必须拥有相应的单元测试并通过测试。**

## 开发

To setup the development envrionment run `cargo run`.

## 贡献者

	MrPan <1049058427@qq.com>

## Getting help

拉布拉多是个人项目。一开始，我只是喜欢拉布拉多犬，因为我的爱好。
我希望这个项目会变得越来越可爱。许多实用的其他函数将
将在将来添加。我希望你能积极帮助这个项目成长并提出建议。
我相信未来会越来越好。

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
