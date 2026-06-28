# Zed Flutter 插件

## 项目概述

在 Zed 编辑器中开发功能对标 VS Code Flutter 插件的 Flutter/Dart 开发扩展。

## 项目结构

```
flutter_extension/         # Zed 扩展包（编译为 WASM，cdylib）
flutter_bridge_server/     # 原生配套二进制（管理 flutter daemon）
```

## 构建命令

```bash
# 完整构建
cargo build --workspace

# 代码检查
cargo check --workspace

# 仅 Bridge Server
cargo build --package flutter-bridge-server

# 仅 WASM 扩展（需要 wasm32-wasip1 目标）
cargo build --package flutter-extension --target wasm32-wasip1

# 运行测试
cargo test --workspace
```

## 核心架构

**三层设计**：
1. **WASM 扩展** — Linux LSP 配置、DAP 调试、Slash Commands、Snippets、Tree-sitter 语法
2. **Flutter Bridge Server**（Native Rust 二进制）— 管理 `flutter daemon`，通过自定义 LSP 协议与 WASM 扩展通信
3. **Flutter/Dart 工具链** — `dart language-server`、`flutter daemon`、`flutter debug-adapter`

### 通信协议

Bridge Server ↔ WASM 扩展：自定义 LSP 方法（标准 LSP over stdio）
Bridge Server ↔ `flutter daemon`：JSON-RPC over stdin/stdout（Flutter Daemon Protocol v0.6.1）

### 自定义 LSP 方法

请求方向：`flutter/daemon/start`, `flutter/device/getDevices`, `flutter/app/start`, `flutter/app/restart` 等
通知方向（预留）：`flutter/device/added`, `flutter/app/started` 等

## 关键 API

- `zed_extension_api = "0.7.0"` — Zed 扩展 API
- `lsp-server = "0.7"` — LSP 传输层（Bridge Server 使用 `Connection::stdio()` + `initialize()` 模式）
- Flutter Daemon 协议：`flutter daemon` → `device.enable` → 监听 `device.added`/`device.removed` 事件

## 实施计划（分三阶段）

### Phase 1（已完成 MVP）
- Dart LSP、DAP 调试、设备管理、应用运行、热重载、Slash Commands、Snippets、Tree-sitter

### Phase 2（待实施）
- DevTools 集成、调试服务扩展开关（Debug Paint 等）、模拟器管理、pubspec.yaml 自动 pub get、增强 DAP 配置

### Phase 3（待实施）
- Widget Inspector、Widget 预览、多设备调试、依赖管理增强

## 环境要求

- Rust 1.75+
- `wasm32-wasip1` target: `rustup target add wasm32-wasip1`

## 设计约束

- Zed WASM 沙箱限制了扩展的能力：不能 spawn 进程、不能直接网络、不能自定义 UI 面板
- 解决方案：Native Bridge Server 模式 —— 核心功能通过原生二进制实现，WASM 扩展作为配置和通信层
