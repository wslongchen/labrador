# 微信支付模块完善 + Examples + 单元测试 + CI 实施计划

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 完善微信支付子模块文档、补充 examples 示例、增加单元测试覆盖、搭建 GitHub Actions CI

**Architecture:** 分四个阶段推进：(1) 补充 pay/partner/payscore/profitsharing/transfer 的文档注释 (2) 为各平台编写独立 examples (3) 补充核心模块的单元测试 (4) 搭建 CI 自动构建/测试/文档

**Tech Stack:** Rust, reqwest, serde, tokio, GitHub Actions

---

## 阶段一：微信支付模块文档完善

### Task 1.1: 补充 partner.rs 方法文档

**Files:**
- Modify: `src/platforms/wechat/pay/partner.rs`

**Step 1: 读取 partner.rs 当前内容**
读取文件，了解所有 `pub async fn` 方法的具体参数。

**Step 2: 为每个方法补充完整的文档注释**

需要补充文档的方法（8个 async 方法）：
- `get_certificates` - 自动获取平台证书
- `partner_unified_order_v3` - 服务商统一下单 V3
- `partner_order_query_by_out_trade_no` - 服务商查询订单 V3
- `partner_close_order_v3` - 服务商关闭订单 V3
- `partner_refund_v3` - 服务商申请退款 V3
- `partner_refund_query_v3` - 服务商退款查询 V3
- `partner_parse_payment_notify_v3` - 服务商支付通知解析 V3
- `partner_parse_refund_notify_v3` - 服务商退款通知解析 V3

每个方法补充：`# 参数`、`# 返回`、`# 官方文档` 三个区块。

**Step 3: 编译验证**
```bash
cargo build --features "wechat" 2>&1 | tail -5
```

### Task 1.2: 补充 payscore.rs 方法文档

**Files:**
- Modify: `src/platforms/wechat/pay/payscore.rs`

**Step 1: 为 7 个方法补充文档**
- `create_order` - 创建支付分订单
- `query_order` - 查询支付分订单
- `cancel_order` - 取消支付分订单
- `modify_order` - 修改支付分订单金额
- `complete_order` - 完结支付分订单
- `query_user_authorization` - 查询用户授权状态
- `terminate_user_authorization` - 解除用户授权

**Step 2: 编译验证**

### Task 1.3: 补充 profitsharing.rs 方法文档

**Files:**
- Modify: `src/platforms/wechat/pay/profitsharing.rs`

**Step 1: 为 7 个方法补充文档**
- `add_receiver` - 添加分账接收方
- `delete_receiver` - 删除分账接收方
- `create_order` - 请求分账
- `query_order` - 查询分账结果
- `unfreeze_order` - 解冻剩余资金
- `query_unsplit_amount` - 查询剩余待分金额
- `create_return_order` - 分账回退

**Step 2: 编译验证**

### Task 1.4: 补充 transfer.rs 方法文档

**Files:**
- Modify: `src/platforms/wechat/pay/transfer.rs`

**Step 1: 为 3 个方法补充文档**
- `create_batch` - 发起批量转账
- `query_batch` - 查询批量转账
- `query_detail` - 查询转账明细

**Step 2: 编译验证**

---

## 阶段二：补充 Examples 目录

### Task 2.1: 创建微信公众号示例

**Files:**
- Create: `examples/wechat_mp.rs`

**Step 1: 编写示例代码**
包含：初始化客户端、获取 access_token、创建菜单、获取用户列表、生成 JS-SDK 配置

**Step 2: 编译验证**
```bash
cargo build --example wechat_mp --features "wechat"
```

### Task 2.2: 创建微信小程序示例

**Files:**
- Create: `examples/wechat_miniapp.rs`

**Step 1: 编写示例代码**
包含：初始化客户端、code2session、获取小程序码、获取订阅消息模板

**Step 2: 编译验证**
```bash
cargo build --example wechat_miniapp --features "wechat"
```

### Task 2.3: 创建企业微信示例

**Files:**
- Create: `examples/wechat_cp.rs`

**Step 1: 编写示例代码**
包含：初始化客户端、获取部门列表、发送消息

**Step 2: 编译验证**
```bash
cargo build --example wechat_cp --features "wechat"
```

### Task 2.4: 创建微信支付示例

**Files:**
- Create: `examples/wechat_pay.rs`

**Step 1: 编写示例代码**
包含：初始化客户端、Native 下单、订单查询、关闭订单

**Step 2: 编译验证**
```bash
cargo build --example wechat_pay --features "wechat"
```

### Task 2.5: 创建支付宝示例

**Files:**
- Create: `examples/alipay.rs`

**Step 1: 编写示例代码**
包含：初始化客户端、预下单、查询订单、手机网站支付

**Step 2: 编译验证**
```bash
cargo build --example alipay --features "alipay"
```

### Task 2.6: 更新 Cargo.toml 注册 examples

**Files:**
- Modify: `Cargo.toml`

**Step 1: 添加 example 条目**
```toml
[[example]]
name = "wechat_mp"
required-features = ["wechat"]

[[example]]
name = "wechat_miniapp"
required-features = ["wechat"]

[[example]]
name = "wechat_cp"
required-features = ["wechat"]

[[example]]
name = "wechat_pay"
required-features = ["wechat"]

[[example]]
name = "alipay"
required-features = ["alipay"]
```

**Step 2: 编译验证所有 examples**
```bash
cargo build --examples --features "wechat,alipay"
```

---

## 阶段三：补充单元测试

### Task 3.1: 补充 crypto 模块单元测试

**Files:**
- Modify: `src/crypto/rust_impl.rs` - 在已有测试模块中增加测试
- Modify: `src/crypto/openssl_iml.rs` - 新增测试模块

**Step 1: rust_impl.rs 补充测试**
- `test_aes_encrypt_decrypt` - AES 加解密往返测试
- `test_hmac_sign_verify` - HMAC 签名验证测试
- `test_rsa_sign_verify` - RSA 签名验证测试（已有 test_rsa，扩展）

**Step 2: openssl_iml.rs 补充测试**
- `test_openssl_rsa_sign_verify` - OpenSSL RSA 签名验证
- `test_openssl_aes_encrypt_decrypt` - OpenSSL AES 加解密

**Step 3: 运行测试**
```bash
cargo test --lib -- crypto
```

### Task 3.2: 补充 errors 模块单元测试

**Files:**
- Modify: `src/errors.rs` - 新增 `#[cfg(test)]` 模块

**Step 1: 编写测试**
- `test_labra_error_display` - 各种错误类型的 Display
- `test_labra_error_from_reqwest` - From<reqwest::Error> 转换
- `test_labra_error_from_serde_json` - From<serde_json::Error> 转换
- `test_should_retry` - 重试判断逻辑
- `test_is_network` - 网络错误判断
- `test_is_timeout` - 超时判断

**Step 2: 运行测试**
```bash
cargo test --lib -- errors
```

### Task 3.3: 补充微信支付签名/序列化单元测试

**Files:**
- Modify: `src/platforms/wechat/pay/mod.rs` - 已有 8 个测试，可补充

**Step 1: 补充测试**
- `test_signature_generation` - 签名生成（如果有可测函数）
- `test_notify_decrypt` - 回调解密测试
- `test_v3_header_generation` - V3 请求头生成

**Step 2: 运行测试**
```bash
cargo test --lib --features "wechat" -- wechat::pay
```

### Task 3.4: 补充支付宝签名单元测试

**Files:**
- Modify: `src/platforms/alipay/pay.rs` - 已有 5 个测试，可补充

**Step 1: 补充测试**
- `test_sign_type_detection` - 签名类型检测
- `test_biz_content_serialization` - 业务内容序列化
- `test_wap_pay_html_generation` - 手机支付 HTML 生成

**Step 2: 运行测试**
```bash
cargo test --lib --features "alipay" -- alipay::pay
```

### Task 3.5: 运行全部测试确认
```bash
cargo test --features "wechat,alipay" -- --nocapture 2>&1 | grep -E "^(running|test result)"
```

---

## 阶段四：GitHub Actions CI

### Task 4.1: 创建 CI 配置文件

**Files:**
- Create: `.github/workflows/ci.yml`

**Step 1: 编写 CI 配置**

```yaml
name: CI

on:
  push:
    branches: [master, develop]
  pull_request:
    branches: [master, develop]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    name: Test (${{ matrix.os }})
    runs-on: ${{ matrix.os }}
    strategy:
      fail-fast: false
      matrix:
        os: [ubuntu-latest, macos-latest]
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
      
      - name: Cache cargo registry
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: |
            ${{ runner.os }}-cargo-
      
      - name: Build (all features)
        run: cargo build --features "wechat,alipay,taobao,pdd,jd,qiniu" --verbose
      
      - name: Run tests
        run: cargo test --features "wechat,alipay" --verbose
      
      - name: Build examples
        run: cargo build --examples --features "wechat,alipay"

  lint:
    name: Lint & Format
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      
      - name: Check formatting
        run: cargo fmt --all -- --check
      
      - name: Run clippy
        run: cargo clippy --features "wechat,alipay" -- -D warnings

  docs:
    name: Build Docs
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
      
      - name: Build documentation
        run: cargo doc --no-deps --features "wechat,alipay"
        env:
          RUSTDOCFLAGS: "-D warnings"
```

**Step 2: 验证 CI 配置语法**
检查 YAML 语法是否正确。

### Task 4.2: 更新 README 添加 CI badge

**Files:**
- Modify: `README.md`
- Modify: `README_CN.md`

**Step 1: 在顶部 badge 区域添加 CI badge**
```markdown
[![CI](https://github.com/woofcloud/labrador/actions/workflows/ci.yml/badge.svg)](https://github.com/woofcloud/labrador/actions/workflows/ci.yml)
```

---

## 执行顺序

1. **阶段一** (文档) → 可以立即开始，不影响其他工作
2. **阶段二** (examples) → 可以立即开始，需要阶段一完成后验证编译
3. **阶段三** (单元测试) → 可以并行进行
4. **阶段四** (CI) → 最后进行，依赖前面所有阶段通过

推荐使用并行团队模式：同时启动 2 个 agent 分别处理阶段一+二 和 阶段三。
