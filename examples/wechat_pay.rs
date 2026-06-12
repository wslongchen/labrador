//! 微信支付 V3 API 使用示例
//!
//! 运行: cargo run --example wechat_pay --features "wechat"

use labrador::platforms::wechat::pay::builder::WechatPayBuilder;
use labrador::platforms::wechat::pay::config::WechatPayApiVersion;
use labrador::platforms::wechat::pay::types::{
    Amount, Detail, GoodsDetail, OrderQueryRequestV3, TradeType, UnifiedOrderRequestV3,
};
use labrador::platforms::wechat::pay::WechatPayClient;

#[tokio::main]
async fn main() {
    // 1. 初始化支付客户端
    let client: WechatPayClient = WechatPayBuilder::new(
        "wxAPPID",
        "商户号",
        "APIv3密钥",
        "https://example.com/notify",
    )
    .p12_path("/path/to/apiclient_cert.p12", Some("商户证书密码"))
    .api_version(WechatPayApiVersion::V3)
    .build()
    .await
    .expect("创建微信支付客户端失败");

    // 2. Native 下单
    let out_trade_no = format!("ORDER{}", chrono::Utc::now().timestamp_millis());
    let request = UnifiedOrderRequestV3::new(
        TradeType::Native,
        &out_trade_no,
        "测试商品",
        Amount::new(1),
        Detail::new(vec![GoodsDetail::new(
            "1001".to_string(),
            "测试商品".to_string(),
            1,
            1,
        )]),
        "https://example.com/notify",
    );
    match client.unified_order_v3(request).await {
        Ok(resp) => println!("✅ Native下单成功: {:?}", resp.code_url),
        Err(e) => println!("❌ 下单失败: {}", e),
    }

    // 3. 订单查询
    let query = OrderQueryRequestV3::by_out_trade_no(&out_trade_no);
    match client.order_query_v3(query).await {
        Ok(resp) => println!("✅ 订单状态: {:?}", resp.trade_state),
        Err(e) => println!("❌ 查询失败: {}", e),
    }

    // 4. 关闭订单
    match client.close_order_v3(&out_trade_no).await {
        Ok(_) => println!("✅ 订单已关闭"),
        Err(e) => println!("⚠️ 关闭订单(预期未支付): {}", e),
    }

    println!("\n🎉 微信支付示例完成");
}
