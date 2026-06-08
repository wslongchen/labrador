//! Labrador 支付功能测试工具
//!
//! 按顺序逐项测试微信支付和支付宝的每个API接口。
//! 所有配置通过环境变量(.env)或命令行参数传入，不会硬编码凭证。
//!
//! ## 使用方式
//!
//! ```bash
//! # 1. 复制环境变量模板并填写真实配置
//! cp .env.example .env
//! vim .env
//!
//! # 2. 运行测试
//! cargo run --example payment_tester -- wechat all    # 测试微信所有API
//! cargo run --example payment_tester -- alipay all    # 测试支付宝所有API
//! cargo run --example payment_tester -- wechat step1  # 只测试微信下单
//! cargo run --example payment_tester -- alipay step1  # 只测试支付宝下单
//! ```
//!
//! ## 测试流程
//!
//! ### 微信支付 V3（推荐顺序）
//! ```
//! step1  → JSAPI/Native/App/H5 下单
//! step2  → 订单查询（微信订单号 / 商户订单号）
//! step3  → 关闭订单
//! step4  → 付款码支付（codepay）
//! step5  → 退款申请 + 退款查询
//! step6  → 撤销订单（reverse）
//! step7  → 申请交易账单 / 资金账单
//! step8  → 合单支付（combine order）
//! ```
//!
//! ### 支付宝（推荐顺序）
//! ```
//! step1  → 预下单 precreate + 查询 query
//! step2  → 电脑网站支付 page_pay + APP支付 app_pay
//! step3  → 当面付 jsapi_pay / face_to_face_pay
//! step4  → 退款 refund + 退款查询 refund_query
//! step5  → 关闭 close + 撤销 cancel
//! step6  → 交易结算 settle
//! step7  → 账单下载 bill_download
//! ```

use std::env;
use std::io::{self, Write};

use labrador::errors::LabradorResult;
use labrador::platforms::wechat::pay::builder::WechatPayBuilder;
use labrador::platforms::wechat::pay::config::WechatPayApiVersion;
use labrador::platforms::wechat::pay::types::{
    Amount, BillType, CodepayOrderRequestV3, CombineCloseOrderRequestV3,
    CombineOrderQueryRequestV3, CombineOrderRequestV3, Detail, GoodsDetail, OrderQueryRequestV3,
    OrderReverseRequestV3, Payer, RefundRequestV3, TradeBillRequestV3, TradeType,
    UnifiedOrderRequestV3,
};
use labrador::platforms::wechat::pay::WechatPayClient;

use labrador::platforms::alipay::builder::AlipayClientBuilder;
use labrador::platforms::alipay::client::AlipayClient;
use labrador::platforms::alipay::pay::{
    AlipayBizRequest, AlipayFaceOrderPayModel, AlipayTradeAppPayModel, AlipayTradeCancelModel,
    AlipayTradeCloseModel, AlipayTradeOrderSettleModel, AlipayTradePagePayModel,
    AlipayTradePreCreateModel, AlipayTradeQueryModel, AlipayTradeRefundModel,
    GoodsDetail as AliGoodsDetail,
};

// ==================== 辅助函数 ====================

fn load_env() {
    // 尝试加载 .env 文件（简单实现，不依赖 dotenv crate）
    let env_path = concat!(env!("CARGO_MANIFEST_DIR"), "/.env");
    if let Ok(content) = std::fs::read_to_string(env_path) {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim().trim_matches('"').trim_matches('\'');
                if env::var(key).is_err() {
                    env::set_var(key, value);
                }
            }
        }
    }
}

fn get_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| {
        eprintln!("\n❌ 缺少环境变量: {}", key);
        eprintln!("   请在 .env 文件中配置，参考 .env.example\n");
        std::process::exit(1);
    })
}

fn get_env_or(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

fn gen_out_trade_no() -> String {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    format!("TEST{}", ts)
}

fn print_result<T: std::fmt::Debug>(name: &str, result: &LabradorResult<T>) {
    match result {
        Ok(data) => println!("  ✅ {} 成功: {:?}", name, data),
        Err(e) => println!("  ❌ {} 失败: {:?}", name, e),
    }
}

fn wait_confirm(step: &str, desc: &str) -> bool {
    println!();
    println!("══════════════════════════════════════════════════");
    println!("  📋 {}: {}", step, desc);
    println!("══════════════════════════════════════════════════");
    print!("  是否执行? [y/n] (默认 y): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_lowercase();
    input.is_empty() || input == "y" || input == "yes"
}

// ==================== 微信支付测试 ====================

async fn create_wechat_client() -> WechatPayClient {
    let app_id = get_env("WECHAT_APP_ID");
    let mch_id = get_env("WECHAT_MCH_ID");
    let api_v3_key = get_env("WECHAT_API_V3_KEY");
    let notify_url = get_env("WECHAT_NOTIFY_URL");
    let p12_path = get_env("WECHAT_P12_PATH");
    let p12_password = env::var("WECHAT_P12_PASSWORD").ok();

    WechatPayBuilder::new(&app_id, &mch_id, &api_v3_key, &notify_url)
        .p12_path(&p12_path, p12_password.as_deref())
        .api_key_v3(&api_v3_key)
        .api_version(WechatPayApiVersion::V3)
        .build()
        .await
        .expect("创建微信支付客户端失败")
}

async fn test_wechat_unified_order(client: &WechatPayClient) {
    println!("\n--- 微信 V3 统一下单 ---");

    let out_trade_no = gen_out_trade_no();
    let notify_url = get_env("WECHAT_NOTIFY_URL");

    let trade_types = [
        ("Native下单", TradeType::Native, Option::<Payer>::None),
        (
            "JSAPI下单",
            TradeType::Jsapi,
            Some(Payer::new(&get_env("WECHAT_OPENID"))),
        ),
        ("H5下单", TradeType::H5, Option::<Payer>::None),
        ("App下单", TradeType::App, Option::<Payer>::None),
        (
            "小程序下单",
            TradeType::Miniapp,
            Some(Payer::new(&get_env("WECHAT_OPENID"))),
        ),
    ];

    for (name, trade_type, payer) in &trade_types {
        print!("  测试 {} ... ", name);
        let mut request = UnifiedOrderRequestV3::new(
            *trade_type,
            &out_trade_no,
            &format!("测试商品-{}", name),
            Amount::new(1),
            Detail::new(vec![GoodsDetail::new(
                "1001".to_string(),
                format!("测试商品-{}", name),
                1,
                1,
            )]),
            &notify_url,
        );
        if let Some(p) = payer {
            request = request.payer(p.clone());
        }
        match client.unified_order_v3(request).await {
            Ok(resp) => println!("✅ {:?}", resp),
            Err(e) => println!("❌ {}", e),
        }
    }
}

async fn test_wechat_order_query(client: &WechatPayClient) {
    println!("\n--- 微信 V3 订单查询 ---");

    print!("  输入商户订单号查询: ");
    io::stdout().flush().unwrap();
    let mut out_trade_no = String::new();
    io::stdin().read_line(&mut out_trade_no).unwrap();
    let out_trade_no = out_trade_no.trim();

    if out_trade_no.is_empty() {
        println!("  ⏭️ 跳过（未输入订单号）");
        return;
    }

    let request = OrderQueryRequestV3::by_out_trade_no(out_trade_no);
    print_result("商户订单号查单", &client.order_query_v3(request).await);
}

async fn test_wechat_close_order(client: &WechatPayClient) {
    println!("\n--- 微信 V3 关闭订单 ---");

    print!("  输入要关闭的商户订单号: ");
    io::stdout().flush().unwrap();
    let mut out_trade_no = String::new();
    io::stdin().read_line(&mut out_trade_no).unwrap();
    let out_trade_no = out_trade_no.trim();

    if out_trade_no.is_empty() {
        println!("  ⏭️ 跳过");
        return;
    }

    print_result("关闭订单", &client.close_order_v3(out_trade_no).await);
}

async fn test_wechat_codepay(client: &WechatPayClient) {
    println!("\n--- 微信 V3 付款码支付 ---");

    print!("  输入付款码（auth_code）: ");
    io::stdout().flush().unwrap();
    let mut auth_code = String::new();
    io::stdin().read_line(&mut auth_code).unwrap();
    let auth_code = auth_code.trim();

    if auth_code.is_empty() {
        println!("  ⏭️ 跳过（付款码支付需要真实扫码）");
        return;
    }

    let request = CodepayOrderRequestV3::new(
        &gen_out_trade_no(),
        "付款码测试商品",
        &auth_code,
        Amount::new(1),
        labrador::platforms::wechat::pay::types::SceneInfo {
            payer_client_ip: "127.0.0.1".to_string(),
            device_id: None,
            store_info: None,
        },
    );
    print_result("付款码支付", &client.codepay_v3(request).await);
}

async fn test_wechat_refund(client: &WechatPayClient) {
    println!("\n--- 微信 V3 退款 + 退款查询 ---");

    print!("  输入要退款的商户订单号: ");
    io::stdout().flush().unwrap();
    let mut out_trade_no = String::new();
    io::stdin().read_line(&mut out_trade_no).unwrap();
    let out_trade_no = out_trade_no.trim();

    if out_trade_no.is_empty() {
        println!("  ⏭️ 跳过");
        return;
    }

    print!("  输入退款金额(分): ");
    io::stdout().flush().unwrap();
    let mut amount = String::new();
    io::stdin().read_line(&mut amount).unwrap();
    let amount: i32 = amount.trim().parse().unwrap_or(1);

    print!("  确认退款 {} 分? [y/n]: ", amount);
    io::stdout().flush().unwrap();
    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm).unwrap();
    if confirm.trim().to_lowercase() != "y" {
        println!("  ⏭️ 取消退款");
        return;
    }

    let refund_amount = labrador::platforms::wechat::pay::types::RefundAmount {
        refund: amount as i64,
        total: amount as i64,
        payer_total: None,
        payer_refund: None,
        currency: None,
    };
    let refund_request = RefundRequestV3::new(
        &format!("REFUND{}", chrono::Utc::now().timestamp_millis()),
        refund_amount,
    )
    .out_trade_no(out_trade_no)
    .notify_url(&get_env("WECHAT_NOTIFY_URL"));

    let refund_result = client.refund_v3(refund_request).await;
    print_result("退款申请", &refund_result);

    // 退款查询
    if let Ok(refund) = &refund_result {
        let query_result = client.refund_query_v3(&refund.out_refund_no).await;
        print_result("退款查询", &query_result);
    }
}

async fn test_wechat_reverse(client: &WechatPayClient) {
    println!("\n--- 微信 V3 撤销订单 ---");

    print!("  输入要撤销的商户订单号: ");
    io::stdout().flush().unwrap();
    let mut out_trade_no = String::new();
    io::stdin().read_line(&mut out_trade_no).unwrap();
    let out_trade_no = out_trade_no.trim();

    if out_trade_no.is_empty() {
        println!("  ⏭️ 跳过");
        return;
    }

    let request = OrderReverseRequestV3::new(&get_env("WECHAT_MCH_ID"), out_trade_no);
    print_result("撤销订单", &client.reverse_order_v3(request).await);
}

async fn test_wechat_bill(client: &WechatPayClient) {
    println!("\n--- 微信 V3 账单下载 ---");

    // 交易账单
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    println!("  申请交易账单 (日期: {})...", today);
    let trade_bill = TradeBillRequestV3::new(&today).bill_type(BillType::All);
    print_result("交易账单", &client.trade_bill_v3(&trade_bill).await);

    // 资金账单
    println!("  申请资金账单 (日期: {})...", today);
    print_result(
        "资金账单",
        &client.download_bill_v3(&today, None, None).await,
    );
}

async fn test_wechat_combine_order(client: &WechatPayClient) {
    println!("\n--- 微信 V3 合单支付 ---");

    let out_trade_no = gen_out_trade_no();
    let notify_url = get_env("WECHAT_NOTIFY_URL");

    // 合单 Native 下单
    println!("  合单Native下单...");
    let sub_orders = vec![labrador::platforms::wechat::pay::types::SubOrder::new(
        &get_env("WECHAT_MCH_ID"),
        "1",
        "商品A",
        Amount::new(1),
    )];
    let combine_request =
        CombineOrderRequestV3::new(TradeType::Native, &out_trade_no, sub_orders, &notify_url);
    print_result("合单下单", &client.combine_order_v3(combine_request).await);

    // 合单查询
    println!("  合单查询...");
    let query_request = CombineOrderQueryRequestV3::new(&out_trade_no);
    print_result(
        "合单查询",
        &client.combine_order_query_v3(query_request).await,
    );

    // 合单关单
    println!("  合单关单...");
    let close_request =
        CombineCloseOrderRequestV3::new(&out_trade_no, &get_env("WECHAT_MCH_ID"), vec![]);
    print_result(
        "合单关单",
        &client.combine_close_order_v3(close_request).await,
    );
}

// ==================== 支付宝测试 ====================

fn create_alipay_client() -> AlipayClient {
    AlipayClientBuilder::new()
        .app_id(get_env("ALIPAY_APP_ID"))
        .app_private_key(get_env("ALIPAY_APP_PRIVATE_KEY"))
        .alipay_public_key(get_env("ALIPAY_PUBLIC_KEY"))
        .alipay_root_cert_path(get_env_or("ALIPAY_ROOT_CERT_PATH", ""))
        .sign_type(&get_env_or("ALIPAY_SIGN_TYPE", "RSA2"))
        .format("JSON")
        .sandbox(false)
        .build()
        .expect("创建支付宝客户端失败")
}

async fn test_alipay_precreate(client: &AlipayClient) {
    println!("\n--- 支付宝 预下单 + 查询 ---");

    let out_trade_no = gen_out_trade_no();
    let srv = client.alipay_service();

    let model = AlipayTradePreCreateModel {
        out_trade_no: out_trade_no.clone(),
        total_amount: 0.01,
        subject: "测试商品-预下单".to_string(),
        product_code: "".to_string(),
        seller_id: None,
        body: Some("Labrador SDK测试".to_string()),
        goods_detail: Some(vec![AliGoodsDetail {
            goods_id: "1".to_string(),
            alipay_goods_id: None,
            goods_name: "测试商品".to_string(),
            quantity: 1,
            price: 0.01,
            goods_category: None,
            body: None,
            show_url: None,
        }]),
        store_id: None,
        terminal_id: None,
        operator_id: None,
        extend_params: None,
        business_params: None,
        discountable_amount: None,
        undiscountable_amount: None,
        merchant_order_no: None,
    };

    let mut request = AlipayBizRequest::new();
    request.set_biz_model(model);
    let precreate_result = srv.precreate(request).await;
    print_result("预下单", &precreate_result);

    // 查询
    let query_model = AlipayTradeQueryModel {
        out_trade_no: Some(out_trade_no),
        trade_no: None,
        org_pid: None,
        query_options: None,
    };
    let query_result = srv.query(query_model.into()).await;
    print_result("订单查询", &query_result);
}

async fn test_alipay_page_app_pay(client: &AlipayClient) {
    println!("\n--- 支付宝 电脑网站支付 + APP支付 ---");

    let srv = client.alipay_service();

    // 电脑网站支付（返回HTML表单）
    let mut page_request = AlipayBizRequest::new();
    let page_model = AlipayTradePagePayModel {
        out_trade_no: gen_out_trade_no(),
        total_amount: 0.01,
        subject: "测试-电脑网站支付".to_string(),
        product_code: "FAST_INSTANT_TRADE_PAY".to_string(),
        body: None,
        qr_pay_mode: None,
        qrcode_width: None,
        quit_url: None,
        goods_detail: None,
        extend_params: None,
        time_expire: None,
        business_params: None,
        promo_params: None,
        integration_type: None,
        request_from_url: None,
        store_id: None,
        sub_merchant: None,
        invoice_info: None,
        merchant_order_no: None,
    };
    page_request.set_biz_model(page_model);
    let page_result = srv.page_pay(page_request).await;
    match &page_result {
        Ok(body) => println!("  ✅ 电脑网站支付: 返回HTML表单 {} 字节", body.len()),
        Err(e) => println!("  ❌ 电脑网站支付失败: {:?}", e),
    }

    // APP支付（返回参数串）
    let mut app_request = AlipayBizRequest::new();
    let app_model = AlipayTradeAppPayModel {
        out_trade_no: gen_out_trade_no(),
        total_amount: 0.01,
        subject: "测试-APP支付".to_string(),
        product_code: "QUICK_MSECURITY_PAY".to_string(),
        body: None,
        goods_detail: None,
        extend_params: None,
        time_expire: None,
        timeout_express: None,
        passback_params: None,
        merchant_order_no: None,
        ext_user_info: None,
        query_options: None,
    };
    app_request.set_biz_model(app_model);
    let app_result = srv.app_pay(app_request).await;
    match &app_result {
        Ok(body) => println!("  ✅ APP支付: 返回参数串 {} 字节", body.len()),
        Err(e) => println!("  ❌ APP支付失败: {:?}", e),
    }
}

async fn test_alipay_face_to_face(client: &AlipayClient) {
    println!("\n--- 支付宝 当面付(face_to_face_pay) ---");

    let out_trade_no = gen_out_trade_no();
    let srv = client.alipay_service();

    print!("  输入付款码（auth_code）: ");
    io::stdout().flush().unwrap();
    let mut auth_code = String::new();
    io::stdin().read_line(&mut auth_code).unwrap();
    let auth_code = auth_code.trim();

    if auth_code.is_empty() {
        println!("  ⏭️ 跳过（当面付需要真实付款码）");
        return;
    }

    let model = AlipayFaceOrderPayModel {
        out_trade_no,
        total_amount: 0.01,
        subject: "当面付测试".to_string(),
        scene: "bar_code".to_string(),
        auth_code: auth_code.to_string(),
        product_code: Some("FACE_TO_FACE_PAYMENT".to_string()),
        seller_id: None,
        body: None,
        goods_detail: None,
        operator_id: None,
        store_id: None,
        terminal_id: None,
        extend_params: None,
        timeout_express: Some("5m".to_string()),
        business_params: None,
        discountable_amount: None,
        undiscountable_amount: None,
        merchant_order_no: None,
    };

    let mut request = AlipayBizRequest::new();
    request.set_biz_model(model);
    let result = srv.face_to_face_pay(request).await;
    print_result("当面付", &result);
}

async fn test_alipay_refund(client: &AlipayClient) {
    println!("\n--- 支付宝 退款 + 退款查询 ---");

    print!("  输入要退款的商户订单号: ");
    io::stdout().flush().unwrap();
    let mut out_trade_no = String::new();
    io::stdin().read_line(&mut out_trade_no).unwrap();
    let out_trade_no = out_trade_no.trim();

    if out_trade_no.is_empty() {
        println!("  ⏭️ 跳过");
        return;
    }

    print!("  确认退款 0.01 元? [y/n]: ");
    io::stdout().flush().unwrap();
    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm).unwrap();
    if confirm.trim().to_lowercase() != "y" {
        println!("  ⏭️ 取消退款");
        return;
    }

    let srv = client.alipay_service();
    let model = AlipayTradeRefundModel {
        out_trade_no: Some(out_trade_no.to_string()),
        trade_no: None,
        refund_amount: Some(0.01),
        refund_reason: Some("SDK测试退款".to_string()),
        out_request_no: Some(format!("REF{}", chrono::Utc::now().timestamp_millis())),
        refund_royalty_parameters: None,
        query_options: None,
    };
    let refund_result = srv.refund(model.into()).await;
    print_result("退款申请", &refund_result);

    // 退款查询
    let query_result = srv
        .query_refund(
            Some(out_trade_no.to_string()),
            None,
            format!("REF{}", chrono::Utc::now().timestamp_millis()),
        )
        .await;
    print_result("退款查询", &query_result);
}

async fn test_alipay_close_cancel(client: &AlipayClient) {
    println!("\n--- 支付宝 关闭订单 + 撤销订单 ---");

    print!("  输入要关闭/撤销的商户订单号: ");
    io::stdout().flush().unwrap();
    let mut out_trade_no = String::new();
    io::stdin().read_line(&mut out_trade_no).unwrap();
    let out_trade_no = out_trade_no.trim();

    if out_trade_no.is_empty() {
        println!("  ⏭️ 跳过");
        return;
    }

    let srv = client.alipay_service();

    // 关闭
    let close_model = AlipayTradeCloseModel {
        out_trade_no: Some(out_trade_no.to_string()),
        trade_no: None,
        operator_id: None,
    };
    let close_result = srv.close(close_model.into()).await;
    print_result("关闭订单", &close_result);

    // 撤销
    let cancel_model = AlipayTradeCancelModel {
        out_trade_no: Some(out_trade_no.to_string()),
        trade_no: None,
    };
    let cancel_result = srv.cancel(cancel_model.into()).await;
    print_result("撤销订单", &cancel_result);
}

async fn test_alipay_settle(client: &AlipayClient) {
    println!("\n--- 支付宝 交易结算 ---");

    print!("  输入要结算的商户订单号: ");
    io::stdout().flush().unwrap();
    let mut out_trade_no = String::new();
    io::stdin().read_line(&mut out_trade_no).unwrap();
    let out_trade_no = out_trade_no.trim();

    if out_trade_no.is_empty() {
        println!("  ⏭️ 跳过");
        return;
    }

    let srv = client.alipay_service();
    let model = AlipayTradeOrderSettleModel {
        out_request_no: format!("SETTLE{}", chrono::Utc::now().timestamp_millis()),
        trade_no: None,
        royalty_parameters: vec![],
        operator_id: None,
        extend_params: None,
    };
    let mut request = AlipayBizRequest::new();
    request.set_biz_model(model);
    let result = srv.settle(request).await;
    print_result("交易结算", &result);
}

async fn test_alipay_bill(client: &AlipayClient) {
    println!("\n--- 支付宝 账单下载 ---");

    print!("  输入账单日期 (yyyy-MM-dd, 默认今天): ");
    io::stdout().flush().unwrap();
    let mut bill_date = String::new();
    io::stdin().read_line(&mut bill_date).unwrap();
    let bill_date = bill_date.trim().to_string();
    let bill_date = if bill_date.is_empty() {
        chrono::Local::now().format("%Y-%m-%d").to_string()
    } else {
        bill_date
    };

    let srv = client.alipay_service();
    print_result("账单下载", &srv.bill_download("trade", &bill_date).await);
}

// ==================== 主函数 ====================

#[tokio::main]
async fn main() {
    load_env();

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        print_usage(&args[0]);
        return;
    }

    let platform = &args[1];
    let command = &args[2];

    match platform.as_str() {
        "wechat" | "wx" => run_wechat_tests(command).await,
        "alipay" | "ali" => run_alipay_tests(command).await,
        _ => {
            eprintln!("❌ 未知平台: {}", platform);
            print_usage(&args[0]);
        }
    }
}

fn print_usage(prog: &str) {
    println!("Labrador 支付功能测试工具");
    println!();
    println!("用法: {} <平台> <命令>", prog);
    println!();
    println!("平台:");
    println!("  wechat (或 wx)    - 微信支付 V3");
    println!("  alipay (或 ali)   - 支付宝");
    println!();
    println!("命令:");
    println!("  all       - 交互式运行所有测试步骤");
    println!("  step1     - 下单测试");
    println!("  step2     - 订单查询");
    println!("  step3     - 关闭订单");
    println!("  step4     - 付款码支付 / 当面付");
    println!("  step5     - 退款 + 退款查询");
    println!("  step6     - 撤销订单 / 交易结算");
    println!("  step7     - 账单下载");
    println!("  step8     - 合单支付 (仅微信)");
    println!();
    println!("示例:");
    println!("  {} wechat all", prog);
    println!("  {} wechat step1", prog);
    println!("  {} alipay step5", prog);
}

// ==================== 微信测试路由 ====================

async fn run_wechat_tests(command: &str) {
    println!("🔧 初始化微信支付客户端...");
    let client = create_wechat_client().await;
    println!("✅ 微信支付客户端就绪\n");

    match command {
        "all" => {
            let steps = [
                ("step1", "下单测试 (Native/JSAPI/H5/App/小程序)"),
                ("step2", "订单查询"),
                ("step3", "关闭订单"),
                ("step4", "付款码支付 (codepay)"),
                ("step5", "退款 + 退款查询"),
                ("step6", "撤销订单 (reverse)"),
                ("step7", "账单下载 (交易账单 + 资金账单)"),
                ("step8", "合单支付 (combine order)"),
            ];
            for (step, desc) in &steps {
                if !wait_confirm(step, desc) {
                    println!("  ⏭️ 跳过 {}\n", step);
                    continue;
                }
                run_wechat_step(&client, step).await;
            }
        }
        step => {
            run_wechat_step(&client, step).await;
        }
    }
    println!("\n🎉 微信支付测试完成！");
}

async fn run_wechat_step(client: &WechatPayClient, step: &str) {
    match step {
        "step1" => test_wechat_unified_order(client).await,
        "step2" => test_wechat_order_query(client).await,
        "step3" => test_wechat_close_order(client).await,
        "step4" => test_wechat_codepay(client).await,
        "step5" => test_wechat_refund(client).await,
        "step6" => test_wechat_reverse(client).await,
        "step7" => test_wechat_bill(client).await,
        "step8" => test_wechat_combine_order(client).await,
        _ => println!("❌ 未知步骤: {} (微信支持 step1-step8)", step),
    }
}

// ==================== 支付宝测试路由 ====================

async fn run_alipay_tests(command: &str) {
    println!("🔧 初始化支付宝客户端...");
    let client = create_alipay_client();
    println!("✅ 支付宝客户端就绪\n");

    match command {
        "all" => {
            let steps = [
                ("step1", "预下单 precreate + 订单查询"),
                ("step2", "电脑网站支付 page_pay + APP支付 app_pay"),
                ("step3", "当面付 face_to_face_pay"),
                ("step4", "退款 + 退款查询"),
                ("step5", "关闭订单 + 撤销订单"),
                ("step6", "交易结算 settle"),
                ("step7", "账单下载 bill_download"),
            ];
            for (step, desc) in &steps {
                if !wait_confirm(step, desc) {
                    println!("  ⏭️ 跳过 {}\n", step);
                    continue;
                }
                run_alipay_step(&client, step).await;
            }
        }
        step => {
            run_alipay_step(&client, step).await;
        }
    }
    println!("\n🎉 支付宝测试完成！");
}

async fn run_alipay_step(client: &AlipayClient, step: &str) {
    match step {
        "step1" => test_alipay_precreate(client).await,
        "step2" => test_alipay_page_app_pay(client).await,
        "step3" => test_alipay_face_to_face(client).await,
        "step4" => test_alipay_refund(client).await,
        "step5" => test_alipay_close_cancel(client).await,
        "step6" => test_alipay_settle(client).await,
        "step7" => test_alipay_bill(client).await,
        _ => println!("❌ 未知步骤: {} (支付宝支持 step1-step7)", step),
    }
}
