//! 微信公众号 API 使用示例
//!
//! 运行: cargo run --example wechat_mp --features "wechat"

use labrador::wechat::mp::api::{Button, Menu};
use labrador::wechat::mp::builder::WechatMpBuilder;

#[tokio::main]
async fn main() {
    // 1. 初始化客户端
    let client = WechatMpBuilder::new("wxAPPID", "APPSECRET")
        .token("TOKEN")
        .encoding_aes_key("AESKEY")
        .build()
        .expect("创建公众号客户端失败");

    // 2. 获取 access_token
    match client.get_jsapi_ticket().await {
        Ok(ticket) => println!("✅ jsapi_ticket: {}...", &ticket[..20.min(ticket.len())]),
        Err(e) => println!("❌ 获取jsapi_ticket失败: {}", e),
    }

    // 3. 创建自定义菜单
    let menu = Menu::new(vec![
        Button::click("点击按钮", "CLICK_KEY"),
        Button::view("跳转网页", "https://www.example.com"),
    ]);
    match client.menu().create_menu(&menu).await {
        Ok(_) => println!("✅ 菜单创建成功"),
        Err(e) => println!("❌ 菜单创建失败: {}", e),
    }

    // 4. 获取用户列表
    match client.user().get_user_list(None).await {
        Ok(list) => println!("✅ 关注用户数: {}", list.total),
        Err(e) => println!("❌ 获取用户列表失败: {}", e),
    }

    // 5. 生成 JS-SDK 配置
    match client
        .generate_js_sdk_config(
            "https://example.com/page",
            vec!["onMenuShareTimeline".to_string()],
            false,
        )
        .await
    {
        Ok(config) => println!(
            "✅ JS-SDK签名: {}...",
            &config.signature[..20.min(config.signature.len())]
        ),
        Err(e) => println!("❌ 生成JS-SDK配置失败: {}", e),
    }

    println!("\n🎉 微信公众号示例完成");
}
