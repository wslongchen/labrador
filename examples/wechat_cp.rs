//! 企业微信 API 使用示例
//!
//! 运行: cargo run --example wechat_cp --features "wechat"

use labrador::wechat::cp::builder::WechatCpBuilder;

#[tokio::main]
async fn main() {
    // 1. 初始化客户端
    let client = WechatCpBuilder::new("CORPID", "CORPSECRET")
        .build()
        .expect("创建企业微信客户端失败");

    // 2. 获取部门列表
    match client.department().list(None).await {
        Ok(depts) => {
            println!("✅ 部门数量: {}", depts.len());
            for dept in depts.iter().take(3) {
                println!("  - id={:?}, name={}", dept.id, dept.name);
            }
        }
        Err(e) => println!("❌ 获取部门列表失败: {}", e),
    }

    // 3. 获取成员列表
    match client.user().list_by_department(1, None, None).await {
        Ok(users) => {
            println!("✅ 成员数量: {}", users.len());
            for user in users.iter().take(3) {
                println!("  - openid={}, nickname={}", user.openid, user.nickname);
            }
        }
        Err(e) => println!("❌ 获取成员列表失败: {}", e),
    }

    println!("\n🎉 企业微信示例完成");
}
