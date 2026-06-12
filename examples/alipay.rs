//! 支付宝 API 使用示例
//!
//! 运行: cargo run --example alipay --features "alipay"

use labrador::platforms::alipay::builder::AlipayClientBuilder;
use labrador::platforms::alipay::client::AlipayClient;
use labrador::platforms::alipay::pay::{AlipayTradePreCreateModel, AlipayTradeQueryModel};
use labrador::platforms::alipay::AlipayBizRequest;

#[tokio::main]
async fn main() {
    // 1. 初始化客户端
    let client: AlipayClient = AlipayClientBuilder::new()
        .app_id("2021xxxxxxxxxx")
        .app_private_key("YOUR_PRIVATE_KEY")
        .alipay_public_key("ALIPAY_PUBLIC_KEY")
        .sign_type("RSA2")
        .format("JSON")
        .sandbox(false)
        .build()
        .expect("创建支付宝客户端失败");

    let srv = client.alipay_service();
    let out_trade_no = format!("ORDER{}", chrono::Utc::now().timestamp_millis());

    // 2. 预下单生成二维码
    let model = AlipayTradePreCreateModel {
        out_trade_no: out_trade_no.clone(),
        total_amount: 0.01,
        subject: "测试商品".to_string(),
        product_code: "FACE_TO_FACE_PAYMENT".to_string(),
        seller_id: None,
        body: Some("Labrador SDK 示例".to_string()),
        ..Default::default()
    };
    let mut request = AlipayBizRequest::new();
    request.set_biz_model(model);
    match srv.precreate(request).await {
        Ok(resp) => println!("✅ 预下单成功, qr_code: {:?}", resp.qr_code),
        Err(e) => println!("❌ 预下单失败: {}", e),
    }

    // 3. 订单查询
    let query_model = AlipayTradeQueryModel {
        out_trade_no: Some(out_trade_no.clone()),
        trade_no: None,
        org_pid: None,
        query_options: None,
    };
    match srv.query(query_model.into()).await {
        Ok(resp) => println!("✅ 订单查询: {:?}", resp.trade_status),
        Err(e) => println!("❌ 查询失败: {}", e),
    }

    println!("\n🎉 支付宝示例完成");
}
