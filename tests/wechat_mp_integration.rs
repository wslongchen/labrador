//! 微信公众号集成测试
//!
//! 运行方式：
//! ```bash
//! cargo test --features "wechat" --test wechat_mp_integration -- --nocapture
//! ```

use labrador::wechat::mp::builder::WechatMpBuilder;
use labrador::wechat::mp::WechatMpClient;

/// 创建公众号客户端（使用提供的凭据）
fn create_mp_client() -> WechatMpClient {
    WechatMpBuilder::new("wx7959501b424a9e93", "cbc9ae18a1d87ecac3bdb10976230546")
        .token("test_token")
        .encoding_aes_key("abcdefghijklmnopqrstuvwxyz0123456789ABCDEFG")
        .build()
        .expect("创建公众号客户端失败")
}

// ==================== 基础 API ====================

#[tokio::test]
async fn test_mp_get_access_token() {
    let client = create_mp_client();
    let result = client.wechat_client().get_access_token().await;
    println!("access_token result: {:?}", result);
    assert!(result.is_ok(), "获取access_token失败: {:?}", result.err());
    let token = result.unwrap();
    assert!(!token.is_empty(), "access_token不应为空");
    println!("✅ access_token: {}...", &token[..20.min(token.len())]);
}

#[tokio::test]
async fn test_mp_get_wechat_ips() {
    let client = create_mp_client();
    let result = client.wechat_client().get_wechat_ips().await;
    println!("wechat ips result: {:?}", result);
    assert!(result.is_ok(), "获取微信服务器IP失败: {:?}", result.err());
    let ips = result.unwrap();
    assert!(!ips.is_empty(), "IP列表不应为空");
    println!("✅ 微信服务器IP数量: {}", ips.len());
}

// ==================== 菜单管理 ====================

#[tokio::test]
async fn test_mp_get_menu() {
    let client = create_mp_client();
    let result = client.menu().get_menu().await;
    println!("menu result: {:?}", result);
    // 可能返回菜单不存在(46003)或正常结果
    match &result {
        Ok(menu) => println!("✅ 获取菜单成功: {:?}", menu),
        Err(e) => println!("⚠️ 获取菜单返回错误(可能是未设置菜单): {}", e),
    }
}

#[tokio::test]
async fn test_mp_get_current_selfmenu_info() {
    let client = create_mp_client();
    let result = client.menu().get_current_selfmenu_info().await;
    println!("selfmenu info result: {:?}", result);
    match &result {
        Ok(info) => println!(
            "✅ 获取当前菜单配置成功: is_menu_open={:?}",
            info.is_menu_open
        ),
        Err(e) => println!("⚠️ 获取当前菜单配置错误: {}", e),
    }
}

// ==================== 自动回复 ====================

#[tokio::test]
async fn test_mp_get_current_autoreply_info() {
    let client = create_mp_client();
    let result = client.autoreply().get_current_autoreply_info().await;
    println!("autoreply info result: {:?}", result);
    match &result {
        Ok(info) => println!(
            "✅ 自动回复信息: add_friend_open={}, autoreply_open={}",
            info.is_add_friend_reply_open, info.is_autoreply_open
        ),
        Err(e) => println!("⚠️ 获取自动回复信息错误: {}", e),
    }
}

// ==================== 模板消息 ====================

#[tokio::test]
async fn test_mp_get_industry() {
    let client = create_mp_client();
    let result = client.template_msg().get_industry().await;
    println!("industry result: {:?}", result);
    match &result {
        Ok(industry) => println!(
            "✅ 行业信息: primary={}, secondary={}",
            industry.primary_industry.first_class, industry.secondary_industry.first_class
        ),
        Err(e) => println!("⚠️ 获取行业信息错误: {}", e),
    }
}

#[tokio::test]
async fn test_mp_get_template_list() {
    let client = create_mp_client();
    let result = client.template_msg().get_template_list().await;
    println!("template list result: {:?}", result);
    match &result {
        Ok(list) => println!("✅ 模板列表数量: {}", list.len()),
        Err(e) => println!("⚠️ 获取模板列表错误: {}", e),
    }
}

// ==================== 二维码 ====================

#[tokio::test]
async fn test_mp_qrcode_create_temp() {
    let client = create_mp_client();
    let result = client.qrcode().create_temp_by_id(1, 3600).await;
    println!("create temp qrcode result: {:?}", result);
    match &result {
        Ok(ticket) => {
            println!(
                "✅ 临时二维码: ticket={:?}, url={:?}",
                ticket.ticket, ticket.url
            );
            // 测试生成二维码图片URL
            if let Some(ref t) = ticket.ticket {
                let url = client.qrcode().get_image_url(t);
                println!("✅ 二维码图片URL: {}", url);
            }
        }
        Err(e) => println!("⚠️ 创建临时二维码错误: {}", e),
    }
}

#[tokio::test]
async fn test_mp_qrcode_create_perm() {
    let client = create_mp_client();
    let result = client
        .qrcode()
        .create_perm_by_str("test_scene_labrador")
        .await;
    println!("create perm qrcode result: {:?}", result);
    match &result {
        Ok(ticket) => {
            println!(
                "✅ 永久二维码: ticket={:?}, url={:?}",
                ticket.ticket, ticket.url
            );
        }
        Err(e) => println!("⚠️ 创建永久二维码错误: {}", e),
    }
}

// ==================== 短链接 ====================

#[tokio::test]
async fn test_mp_short_url() {
    let client = create_mp_client();
    let result = client
        .qrcode()
        .gen_short_key("https://www.baidu.com", Some(3600))
        .await;
    println!("gen short key result: {:?}", result);
    match &result {
        Ok(short) => {
            println!(
                "✅ 短链接: short_key={}, short_url={}",
                short.short_key, short.short_url
            );
            // 测试通过short_key获取原始链接
            let fetch_result = client.qrcode().fetch_short_url(&short.short_key).await;
            println!("fetch short url result: {:?}", fetch_result);
        }
        Err(e) => println!("⚠️ 生成短链接错误: {}", e),
    }
}

// ==================== 用户管理 ====================

#[tokio::test]
async fn test_mp_get_tags() {
    let client = create_mp_client();
    let result = client.user().get_tags().await;
    println!("get tags result: {:?}", result);
    match &result {
        Ok(tags) => {
            println!("✅ 标签数量: {}", tags.len());
            for tag in tags {
                println!(
                    "  - id={}, name={}, count={:?}",
                    tag.id, tag.name, tag.count
                );
            }
        }
        Err(e) => println!("⚠️ 获取标签列表错误: {}", e),
    }
}

#[tokio::test]
async fn test_mp_get_user_list() {
    let client = create_mp_client();
    let result = client.user().get_user_list(None).await;
    println!("get user list result: {:?}", result);
    match &result {
        Ok(list) => {
            println!(
                "✅ 关注用户: total={}, count={}, next_openid={}",
                list.total, list.count, list.next_openid
            );
        }
        Err(e) => println!("⚠️ 获取用户列表错误: {}", e),
    }
}

// ==================== JS-SDK ====================

#[tokio::test]
async fn test_mp_get_jsapi_ticket() {
    let client = create_mp_client();
    let result = client.get_jsapi_ticket().await;
    println!("jsapi_ticket result: {:?}", result);
    match &result {
        Ok(ticket) => {
            assert!(!ticket.is_empty(), "ticket不应为空");
            println!("✅ jsapi_ticket: {}...", &ticket[..20.min(ticket.len())]);
        }
        Err(e) => println!("⚠️ 获取jsapi_ticket错误: {}", e),
    }
}

#[tokio::test]
async fn test_mp_generate_js_sdk_config() {
    let client = create_mp_client();
    let result = client
        .generate_js_sdk_config(
            "https://example.com/test",
            vec![
                "onMenuShareTimeline".to_string(),
                "onMenuShareAppMessage".to_string(),
            ],
            false,
        )
        .await;
    println!("js_sdk_config result: {:?}", result);
    match &result {
        Ok(config) => {
            assert!(!config.signature.is_empty(), "签名不应为空");
            assert!(!config.nonce_str.is_empty(), "随机字符串不应为空");
            println!(
                "✅ JS-SDK配置: app_id={}, timestamp={}",
                config.app_id, config.timestamp
            );
        }
        Err(e) => println!("⚠️ 生成JS-SDK配置错误: {}", e),
    }
}

// ==================== 标签管理（需要粉丝，先获取openid再测试） ====================

#[tokio::test]
async fn test_mp_tag_crud_flow() {
    let client = create_mp_client();

    // 1. 创建标签
    let create_result = client.user().create_tag("labrador_test_tag").await;
    println!("create tag result: {:?}", create_result);
    match &create_result {
        Ok(tag) => {
            println!("✅ 创建标签成功: id={}, name={}", tag.id, tag.name);

            // 2. 更新标签
            let update_result = client
                .user()
                .update_tag(tag.id, "labrador_test_tag_updated")
                .await;
            println!("update tag result: {:?}", update_result);
            if let Ok(_) = &update_result {
                println!("✅ 更新标签成功");
            }

            // 3. 获取标签列表确认
            let tags = client.user().get_tags().await;
            println!("tags after crud: {:?}", tags);

            // 4. 删除标签
            let delete_result = client.user().delete_tag(tag.id).await;
            println!("delete tag result: {:?}", delete_result);
            if let Ok(_) = &delete_result {
                println!("✅ 删除标签成功");
            }
        }
        Err(e) => {
            let err_str = e.to_string();
            if err_str.contains("45157") {
                // 标签名非法或已存在，跳过
                println!("⚠️ 标签名已存在或非法，跳过CRUD测试: {}", e);
            } else {
                println!("⚠️ 创建标签失败: {}", e);
            }
        }
    }
}

// ==================== API管理 ====================

#[tokio::test]
async fn test_mp_api_manage_get_quota() {
    let client = create_mp_client();
    let result = client.api_manage().get_api_quota("/cgi-bin/token").await;
    println!("api quota result: {:?}", result);
    match &result {
        Ok(quota) => {
            println!(
                "✅ API配额: daily_limit={}, remaining={}, used={}",
                quota.quota.daily_limit, quota.quota.remaining, quota.quota.used
            );
            assert!(quota.quota.daily_limit > 0, "每日配额应大于0");
        }
        Err(e) => println!("⚠️ 获取API配额错误: {}", e),
    }
}
