//! 微信小程序集成测试
//!
//! 运行方式：
//! ```bash
//! cargo test --features "wechat" --test wechat_miniapp_integration -- --nocapture
//! ```

use labrador::wechat::miniapp::builder::WechatMiniAppBuilder;
use labrador::wechat::miniapp::WechatMiniAppClient;

/// 创建小程序客户端（使用提供的凭据）
fn create_miniapp_client() -> WechatMiniAppClient {
    WechatMiniAppBuilder::new("wxc04c214b68e0d93f", "15061e4fb8ae895f12f25e17d553630a")
        .build()
        .expect("创建小程序客户端失败")
}

// ==================== 基础 API ====================

#[tokio::test]
async fn test_miniapp_get_access_token() {
    let client = create_miniapp_client();
    let result = client.get_access_token().await;
    println!("access_token result: {:?}", result);
    assert!(result.is_ok(), "获取access_token失败: {:?}", result.err());
    let token = result.unwrap();
    assert!(!token.access_token.is_empty(), "access_token不应为空");
    println!(
        "✅ access_token: {}...",
        &token.access_token[..20.min(token.access_token.len())]
    );
    println!("✅ expires_in: {}", token.expires_in);
}

#[tokio::test]
async fn test_miniapp_get_wechat_ips() {
    let client = create_miniapp_client();
    let result = client.wechat_client().get_wechat_ips().await;
    println!("wechat ips result: {:?}", result);
    assert!(result.is_ok(), "获取微信服务器IP失败: {:?}", result.err());
    let ips = result.unwrap();
    assert!(!ips.is_empty(), "IP列表不应为空");
    println!("✅ 微信服务器IP数量: {}", ips.len());
}

// ==================== 订阅消息 ====================

#[tokio::test]
async fn test_miniapp_get_category() {
    let client = create_miniapp_client();
    let result = client.message().get_category().await;
    println!("category result: {:?}", result);
    match &result {
        Ok(categories) => {
            println!("✅ 类目数量: {}", categories.len());
            for cat in categories.iter().take(5) {
                println!("  - id={}, name={}", cat.id, cat.name);
            }
        }
        Err(e) => println!("⚠️ 获取类目错误: {}", e),
    }
}

#[tokio::test]
async fn test_miniapp_get_template_list() {
    let client = create_miniapp_client();
    let result = client.message().get_template_list().await;
    println!("template list result: {:?}", result);
    match &result {
        Ok(list) => {
            println!("✅ 私有模板数量: {}", list.len());
            for tmpl in list.iter().take(5) {
                println!("  - pri_tmpl_id={}, title={}", tmpl.pri_tmpl_id, tmpl.title);
            }
        }
        Err(e) => println!("⚠️ 获取模板列表错误(可能未添加模板): {}", e),
    }
}

#[tokio::test]
async fn test_miniapp_get_pub_template_titles() {
    let client = create_miniapp_client();
    // 先获取类目ID
    let categories = client.message().get_category().await;
    if let Ok(cats) = categories {
        if let Some(first_cat) = cats.first() {
            let result = client
                .message()
                .get_pub_template_titles(
                    &labrador::wechat::miniapp::api::PubTemplateTitlesRequest::new(
                        &first_cat.id.to_string(),
                        0,
                        10,
                    ),
                )
                .await;
            println!("pub template titles result: {:?}", result);
            match &result {
                Ok(titles) => {
                    println!("✅ 公共模板标题数量: {}", titles.data.len());
                    for t in titles.data.iter().take(5) {
                        println!("  - tid={}, title={}", t.tid, t.title);
                    }
                }
                Err(e) => println!("⚠️ 获取公共模板标题错误: {}", e),
            }
        } else {
            println!("⚠️ 没有可用类目");
        }
    } else {
        println!("⚠️ 无法获取类目");
    }
}

// ==================== 二维码 ====================

#[tokio::test]
async fn test_miniapp_qrcode_get() {
    let client = create_miniapp_client();
    // 获取小程序码（无限数量）
    use labrador::wechat::miniapp::api::WxacodeUnlimitedRequest;
    let request = WxacodeUnlimitedRequest {
        scene: "test_scene".to_string(),
        page: None,
        check_path: Some(false),
        env_version: None,
        width: Some(430),
        auto_color: Some(false),
        line_color: None,
        is_hyaline: Some(false),
    };
    let result = client.qrcode().get_wxacode_unlimited(&request).await;
    println!("get unlimited qrcode result: {:?}", result.is_ok());
    match &result {
        Ok(bytes) => {
            assert!(!bytes.is_empty(), "二维码图片不应为空");
            println!("✅ 小程序码图片大小: {} bytes", bytes.len());
        }
        Err(e) => println!("⚠️ 获取小程序码错误: {}", e),
    }
}

// ==================== 用户管理 ====================

#[tokio::test]
async fn test_miniapp_get_plugin_open_pid() {
    let client = create_miniapp_client();
    // 这个需要真实的小程序 code，通常不可用，仅测试接口可达性
    let result = client
        .user()
        .get_plugin_open_pid(&labrador::wechat::miniapp::api::PluginOpenPidRequest {
            code: "test_code".to_string(),
        })
        .await;
    println!("get plugin open pid result: {:?}", result);
    // 预期返回错误（code无效），但至少证明接口调用链正确
    match &result {
        Ok(pid) => println!("✅ plugin open pid: {}", pid.openpid),
        Err(e) => println!("⚠️ 预期错误(code无效): {}", e),
    }
}

// ==================== 物流服务 ====================

#[tokio::test]
async fn test_miniapp_express_get_all_delivery() {
    let client = create_miniapp_client();
    let result = client.express().get_all_delivery().await;
    println!("get all delivery result: {:?}", result);
    match &result {
        Ok(deliveries) => {
            println!("✅ 支持的快递公司数量: {}", deliveries.len());
            for d in deliveries.iter().take(5) {
                println!("  - id={}, name={}", d.delivery_id, d.delivery_name);
            }
        }
        Err(e) => println!("⚠️ 获取快递公司列表错误(可能需要物流助手权限): {}", e),
    }
}

// ==================== OCR识别 ====================

#[tokio::test]
async fn test_miniapp_ocr_bankcard() {
    let client = create_miniapp_client();
    // OCR 需要图片URL，测试接口可达性
    let result = client
        .ocr()
        .ocr_bankcard("https://example.com/test_bankcard.jpg")
        .await;
    println!("ocr bankcard result: {:?}", result);
    match &result {
        Ok(info) => println!("✅ 银行卡OCR结果: {:?}", info),
        Err(e) => println!("⚠️ OCR错误(图片可能无效): {}", e),
    }
}

// ==================== 数据预拉取 ====================

#[tokio::test]
async fn test_miniapp_cloud_invoke() {
    let client = create_miniapp_client();
    // 云开发调用需要配置环境，测试接口可达性
    let result = client
        .cloud()
        .invoke_cloud_function(
            "test_env",
            "test_func",
            Some(serde_json::json!({"key": "value"})),
        )
        .await;
    println!("cloud invoke result: {:?}", result);
    match &result {
        Ok(resp) => println!("✅ 云函数调用结果: {:?}", resp),
        Err(e) => println!("⚠️ 云函数调用错误(环境可能未配置): {}", e),
    }
}

// ==================== code2session（需要真实code） ====================

#[tokio::test]
async fn test_miniapp_code2session_invalid_code() {
    let client = create_miniapp_client();
    // 使用无效code测试接口可达性和错误处理
    let result = client.code2session("invalid_test_code").await;
    println!("code2session with invalid code: {:?}", result);
    // 预期返回错误（code无效），验证错误处理链路
    match &result {
        Ok(session) => println!("✅ code2session成功: openid={:?}", session.openid),
        Err(e) => {
            let err_str = e.to_string();
            if err_str.contains("40029") || err_str.contains("invalid code") {
                println!("✅ 正确返回code无效错误: {}", e);
            } else {
                println!("⚠️ 未知错误: {}", e);
            }
        }
    }
}
