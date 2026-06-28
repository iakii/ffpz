# Zed Flutter 插件 — 实施方案

## Context

用户希望在 Zed 编辑器中开发功能对标 VS Code Flutter 插件的 Flutter/Dart 开发扩展。项目位于 `C:\Users\Kai\iProj\rust_apps\zed_flutter_plugin\`，为 greenfield 项目（目录为空）。目标是提供完整的 Flutter 开发体验：设备管理、应用运行与热重载、调试、包管理、Widget Inspector 等。

## 整体架构：三层设计

```
┌─────────────────────────────────────────────────────────┐
│  Zed 编辑器                                              │
│  ┌───────────────────────────────────────────────────┐  │
│  │  WASM 扩展 (flutter_extension)                     │  │
│  │                                                    │  │
│  │  Dart LSP 配置 · Flutter Bridge 客户端 · DAP 适配器  │  │
│  │  Slash Commands · Snippets · Tree-sitter Grammar   │  │
│  └────────┬──────────────┬─────────────────┬──────────┘  │
└───────────│──────────────│─────────────────│─────────────┘
            │ LSP (标准)    │ LSP (自定义协议)  │ DAP
            ▼              ▼                  ▼
┌─────────────────────────────────────────────────────────┐
│  原生配套二进制各层                                       │
│                                                         │
│  dart language-server  │  flutter_bridge_server (我们构建) │
│  (Dart SDK 提供)       │  管理 flutter daemon            │
│                        │  JSON-RPC 通信 + 事件转发        │
│                        │                                │
│                        │  flutter debug-adapter         │
│                        │  (Flutter SDK 提供)             │
└─────────────────────────────────────────────────────────┘
            │              │                  │
            ▼              ▼                  ▼
┌─────────────────────────────────────────────────────────┐
│  Flutter / Dart 工具链                                   │
│  flutter daemon · dart · Android SDK · iOS SDK · etc.   │
└─────────────────────────────────────────────────────────┘
```

### 核心设计决策

- **"全面替代"策略**：本扩展完全替代 zed-extensions/dart，提供 Dart 基础 + Flutter 增强功能
- **"LSP Bridge" 模式**：WASM 扩展通过 `language_server_command` 启动原生 Bridge Server，Bridge Server 管理 `flutter daemon`，之间通过自定义 LSP 方法通信
- **下载机制**：Bridge Server 二进制通过 GitHub Releases 自动下载缓存

---

## 第一阶段：MVP（核心工作流）

### 功能清单

| 编号 | 功能 | 优先级 | 实现方式 |
|------|------|--------|---------|
| F1 | Dart 语法高亮与基础编辑 | P0 | Tree-sitter grammar + .scm queries |
| F2 | Dart LSP（补全/诊断/跳转/悬停） | P0 | 配置 `dart language-server` |
| F3 | 调试支持（断点/变量/调用栈） | P0 | 配置 `flutter debug-adapter` DAP |
| F4 | 设备发现与管理 | P0 | Native Bridge + `/flutter:devices` |
| F5 | 应用运行 | P0 | Native Bridge + `/flutter:run` |
| F6 | 热重载 | P0 | Native Bridge + `/flutter:hot-reload` |
| F7 | 热重启 | P1 | Native Bridge + `/flutter:hot-restart` |
| F8 | 停止应用 | P1 | Native Bridge + `/flutter:stop` |
| F9 | 常用命令 | P0 | Slash Commands |
| F10 | Flutter 代码片段 | P0 | Snippets JSON |

### 项目文件结构

```
zed_flutter_plugin/
├── Cargo.toml                          # workspace: flutter_extension + flutter_bridge_server
├── .github/workflows/
│   ├── ci.yml
│   └── release.yml
├── flutter_extension/                  # Zed 扩展包
│   ├── extension.toml
│   ├── Cargo.toml                      # crate-type = ["cdylib"]
│   ├── src/
│   │   ├── lib.rs                      # register_extension!(FlutterExtension)
│   │   ├── dart_lsp.rs                 # Dart LSP 启动配置
│   │   ├── flutter_bridge.rs           # Bridge Server 下载/启动/通信
│   │   ├── dap.rs                      # DAP 适配器配置
│   │   └── slash_commands.rs           # /flutter:* 命令注册
│   ├── languages/dart/
│   │   ├── config.toml
│   │   ├── highlights.scm / brackets.scm / indents.scm / outline.scm
│   │   ├── injections.scm / overrides.scm / runnables.scm / textobjects.scm
│   └── debug_adapter_schemas/
│       └── flutter.json
│   └── snippets/
│       └── flutter.json
├── flutter_bridge_server/              # 原生二进制（独立 Rust 程序）
│   ├── Cargo.toml                      # [[bin]] name = "flutter-bridge-server"
│   └── src/
│       ├── main.rs                     # 入口：tracing 初始化 + LSP server 启动
│       ├── lsp_server.rs               # LSP 传输层 + 消息路由
│       ├── daemon.rs                   # Flutter daemon JSON-RPC 客户端
│       ├── state.rs                    # 全局状态：设备/应用/daemon
│       └── protocols.rs               # 自定义协议类型定义
```

### 自定义 LSP 协议（扩展 <-> Bridge）

**请求方向**（Extension -> Bridge）：
- `flutter/daemon/start`, `flutter/daemon/shutdown`
- `flutter/device/getDevices`, `flutter/device/enable`, `flutter/device/disable`
- `flutter/emulator/getEmulators`, `flutter/emulator/launch`
- `flutter/app/start`, `flutter/app/restart`, `flutter/app/stop`
- `flutter/app/callServiceExtension`
- `flutter/devtools/serve`

**通知方向**（Bridge -> Extension）：
- `flutter/daemon/connected`, `flutter/daemon/log`
- `flutter/device/added`, `flutter/device/removed`
- `flutter/app/start`, `flutter/app/debugPort`, `flutter/app/started`
- `flutter/app/log`, `flutter/app/stop`, `flutter/app/devTools`

### MVP Slash Commands

`/flutter:devices`, `/flutter:run`, `/flutter:hot-reload`, `/flutter:hot-restart`, `/flutter:stop`, `/flutter:pub-get`, `/flutter:pub-upgrade`, `/flutter:clean`, `/flutter:doctor`, `/flutter:create`

### 技术选型

**WASM 扩展**：`zed_extension_api = "0.7.0"`, `serde`, `serde_json`

**Bridge Server**：`tokio`（异步运行时+进程管理）, `lsp-server`（LSP stdio 传输）, `lsp-types`（LSP 类型）, `serde`/`serde_json`, `tracing`（结构化日志）, `anyhow`/`thiserror`（错误处理）, `directories`（跨平台缓存目录）

---

## 第二阶段：增强功能

| 编号 | 功能 | 实现方式 |
|------|------|---------|
| F11 | DevTools 集成 | Bridge 管理 DevTools，slash command 打开浏览器 |
| F12 | 调试服务扩展开关 | 通过 `app.callServiceExtension` 切换 debugPaint/Performance Overlay 等 |
| F13 | 模拟器管理 | `/flutter:emulators` + `/flutter:launch-emulator` |
| F14 | pubspec.yaml 自动 pub get | LSP workspace 配置 |
| F15 | 增强 DAP 调试配置 | 自定义 schema 支持 deviceId/flutterMode/platform |
| F16 | 任务模板 | `tasks.json` 提供 flutter run/build/test 任务 |
| F17 | 输出通道 | Bridge 转发应用/daemon 日志到 Zed |
| F18 | Flutter Outline 支持 | Dart LSP `dart.textDocument.publishFlutterOutline` |

---

## 第三阶段：高级功能

| 编号 | 功能 | 实现方式 |
|------|------|---------|
| F19 | Widget Inspector | Bridge 调用 VM Service Inspector API，slash command 文本展示 Widget 树 |
| F20 | Widget 预览 | Dart LSP `dart/textDocument/getFlutterWidgetPreviews` |
| F21 | 属性编辑器 | VM Service ext.flutter.inspector.* 系列 API |
| F22 | 多设备同时调试 | 管理多个 DAP 会话和 daemon app 实例 |
| F23 | 依赖管理增强 | 解析 `.dart_tool/package_config.json`，slash command 展示 |
| F24 | DevTools 深度集成 | 直接打开 Inspector/Performance/Memory/Network 特定页面 |

---

## 关键数据流

### 设备发现
```
用户打开 Dart 文件 → WASM 扩展启动 Bridge Server
→ Bridge 启动 flutter daemon
→ Bridge 发送 device.enable
→ daemon 检测设备，发送 device.added 事件
→ Bridge 缓存并转发 flutter/device/added 到扩展
→ 用户 /flutter:devices 查看设备列表
```

### 应用运行与热重载
```
用户 /flutter:run → 扩展 → Bridge (flutter/app/start)
→ daemon (app.start) → 应用在设备启动
→ daemon 事件流: app.start → debugPort → started
→ Bridge 转发 flutter/app/started 到扩展

用户 /flutter:hot-reload → 扩展 → Bridge (flutter/app/restart, fullRestart:false)
→ daemon → 设备上增量更新 → 返回 reloaded
```

### 调试
```
Zed 调试 UI 启动 "Flutter Debug" → WASM 扩展 get_dap_binary()
→ 返回 flutter debug-adapter 启动命令
→ Zed 启动 flutter debug-adapter 进程
→ flutter debug-adapter 内部管理 flutter run --machine
→ 标准 DAP 功能：断点/单步/变量/调用栈
→ 自定义事件：flutter.appStarted, flutter.serviceExtensionStateChanged
```

---

## 风险与应对

| 风险 | 应对 |
|------|------|
| 无自定义 UI 面板 | 用 slash commands + 输出文本替代设备选择器/调试工具栏 |
| 无编辑器事件钩子 | MVP 手动热重载；后续通过 LSP didSave 间接实现 |
| 无 WebView | DevTools 用外部浏览器打开（与 VS Code 一致） |
| WASM 不能 spawn 进程 | 使用 Native Bridge Server 模式 |
| Bridge Server 崩溃 | WASM 扩展检测 LSP 断开，自动重启 |
| flutter daemon 崩溃 | Bridge 监听进程退出，自动重启 + 重新启用设备轮询 |
| 多实例冲突 | Bridge 使用锁文件防止多实例 |
| 平台差异（flutter.bat vs flutter） | Bridge Server 内部处理 |
| 与 zed-extensions/dart 竞争 | 全面替代策略，提供完整 Dart+Flutter 体验 |

---

## 验证方案

### 构建验证
1. `cargo check --workspace` — 检查所有代码编译
2. `cargo build --workspace` — 完整构建
3. `cargo test --workspace` — 运行单元测试
4. `wasm-pack build flutter_extension` — 构建 WASM 扩展

### 功能验证
1. 在 Zed 中安装本地扩展，打开 Flutter 项目验证 Dart LSP 工作正常
2. 运行 `/flutter:devices` 验证设备发现
3. 运行 `/flutter:run` 验证应用启动
4. 运行 `/flutter:hot-reload` 验证热重载
5. 启动 Zed 调试器验证断点和变量查看
6. 运行 `/flutter:pub-get` 验证包管理
7. 验证代码片段是否正常展开

### 多平台验证
- macOS：flutter 命令路径 `which flutter`
- Windows：flutter.bat 路径查找
- Linux：snap/homebrew/path 中的 flutter
