# 开发指南

## 环境要求

- Rust 1.75+
- `wasm32-wasip1` 目标：`rustup target add wasm32-wasip1`

## 项目结构

```
zed_flutter_plugin/
├── flutter_extension/       # Zed 扩展包（编译为 WASM）
│   ├── extension.toml       # 扩展清单
│   ├── src/
│   │   ├── lib.rs           # 入口：register_extension!
│   │   ├── dart_lsp.rs      # Dart LSP 配置
│   │   ├── flutter_bridge.rs# Bridge Server 下载/启动
│   │   ├── dap.rs           # DAP 调试适配器
│   │   └── slash_commands.rs# Slash 命令实现
│   ├── languages/dart/      # Tree-sitter 语法查询
│   ├── snippets/            # 代码片段
│   └── debug_adapter_schemas/
├── flutter_bridge_server/   # 原生配套程序
│   └── src/
│       ├── main.rs          # 入口
│       ├── lsp_server.rs    # LSP 服务器
│       ├── daemon.rs        # Flutter daemon 客户端
│       ├── protocols.rs     # 协议类型
│       └── state.rs         # 状态管理
└── .github/workflows/
    ├── ci.yml               # PR 检查
    └── release.yml          # 发布构建
```

## 构建

```bash
# 所有 crate
cargo build --workspace

# 仅 Bridge Server（原生二进制）
cargo build --package flutter-bridge-server

# 仅 WASM 扩展
cargo build --package flutter-extension --target wasm32-wasip1

# 代码检查
cargo clippy --workspace

# 测试
cargo test --workspace
```

## 本地测试

### 测试 Bridge Server

Bridge Server 是一个原生二进制程序，可以在终端中手动启动测试：

```bash
# 构建
cargo build --package flutter-bridge-server

# 直接启动（会等待 LSP 握手）
RUST_LOG=debug ./target/debug/flutter-bridge-server
```

### 测试 WASM 扩展

1. 编译 WASM 扩展：
   ```bash
   cargo build --package flutter-extension --target wasm32-wasip1 --release
   ```

2. 在 Zed 中加载本地扩展：
   - 打开 Zed 设置 → Extensions → Install Local Extension
   - 选择 `flutter_extension/` 目录

## 自定义 LSP 协议

Bridge Server 与 WASM 扩展之间通过自定义 LSP 方法通信。

### 请求方向

| 方法 | 说明 |
|------|------|
| `flutter/daemon/start` | 启动 flutter daemon |
| `flutter/daemon/shutdown` | 关闭 flutter daemon |
| `flutter/device/getDevices` | 获取设备列表 |
| `flutter/device/enable` | 启用设备轮询 |
| `flutter/app/start` | 启动应用 |
| `flutter/app/restart` | 热重载/热重启 |
| `flutter/app/stop` | 停止应用 |

## 发布流程

1. 更新版本号：`git tag v0.1.0`
2. 推送 tag：`git push origin v0.1.0`
3. GitHub Actions 自动构建并创建 Draft Release
4. 在 Release 页面确认后发布
5. 向 Zed 扩展仓库提交 PR 添加我们的扩展
