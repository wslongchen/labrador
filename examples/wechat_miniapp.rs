//! 微信小程序 API 使用示例
//!
//! 运行: cargo run --example wechat_miniapp --features "wechat"

use labrador::wechat::miniapp::api::WxacodeUnlimitedRequest;
use labrador::wechat::miniapp::builder::WechatMiniAppBuilder;

#[tokio::main]
async fn main() {
    // 1. 初始化客户端
    let client = WechatMiniAppBuilder::new("wxAPPID", "APPSECRET")
        .build()
        .expect("创建小程序客户端失败");

    // 2. 登录凭证校验
    match client.code2session("login_code").await {
        Ok(session) => println!(
            "✅ openid: {:?}, unionid: {:?}",
            session.openid, session.unionid
        ),
        Err(e) => println!("⚠️ code2session(预期无效code): {}", e),
    }

    // 3. 获取小程序码
    let request = WxacodeUnlimitedRequest {
        scene: "demo_scene".to_string(),
        page: None,
        check_path: Some(false),
        env_version: None,
        width: Some(430),
        auto_color: Some(false),
        line_color: None,
        is_hyaline: Some(false),
    };
    match client.qrcode().get_wxacode_unlimited(&request).await {
        Ok(bytes) => println!("✅ 小程序码图片: {} bytes", bytes.len()),
        Err(e) => println!("❌ 获取小程序码失败: {}", e),
    }

    // 4. 获取订阅消息类目
    match client.message().get_category().await {
        Ok(categories) => {
            println!("✅ 订阅消息类目数: {}", categories.len());
            for cat in categories.iter().take(3) {
                println!("  - id={}, name={}", cat.id, cat.name);
            }
        }
        Err(e) => println!("❌ 获取类目失败: {}", e),
    }

    println!("\n🎉 微信小程序示例完成");
}
