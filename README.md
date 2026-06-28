# Zed Flutter 扩展

为 [Zed 编辑器](https://zed.dev/) 提供 Dart 和 Flutter 开发支持。

## 功能

- ✅ **Dart LSP** — 代码补全、诊断、跳转定义、悬停提示、重构等
- ✅ **Flutter 调试** — 通过 DAP 支持断点、变量查看、调用栈
- ✅ **设备管理** — 查看已连接的设备/模拟器列表
- ✅ **应用运行** — 在设备上启动 Flutter 应用
- ✅ **热重载/热重启** — 保存代码后快速更新应用
- ✅ **常用命令** — pub get、pub upgrade、flutter clean、flutter doctor、flutter create
- ✅ **Flutter 代码片段** — stless、stful、build、Container 等常用代码模板
- 🚧 **DevTools 集成** — 打开 DevTools (Inspector/Performance/Memory/Network)
- 🚧 **Widget Inspector** — Widget 树可视化与选择模式
- 🚧 **调试服务扩展** — Debug Paint、Performance Overlay 等开关

## 安装

从 Zed 扩展商店搜索 "Dart & Flutter" 安装。

或者手动安装：
1. 从 [Releases](https://github.com/your-org/zed-flutter-plugin/releases) 下载最新版本
2. 解压到 Zed 的扩展目录

## 使用方法

### 打开 Flutter 项目

打开包含 `pubspec.yaml` 的目录，扩展自动启动 Dart LSP 和 Flutter Bridge Server。

### 命令

| 命令 | 说明 |
|------|------|
| `/flutter:devices` | 显示已连接设备列表 |
| `/flutter:run` | 启动应用（支持 `--device=id --mode=profile/release`） |
| `/flutter:hot-reload` | 热重载 |
| `/flutter:hot-restart` | 热重启 |
| `/flutter:stop` | 停止应用 |
| `/flutter:pub-get` | 获取依赖 |
| `/flutter:pub-upgrade` | 升级依赖 |
| `/flutter:clean` | 清理构建产物 |
| `/flutter:doctor` | 运行环境检查 |
| `/flutter:create` | 创建新项目 |
| `/flutter:select-device` | 切换选中设备 |

### 调试

Zed 的调试面板中提供了 "Flutter: Debug"、"Flutter: Profile" 等调试配置。
支持：断点、单步执行、变量查看、调用栈。

### 代码片段

| 前缀 | 说明 |
|------|------|
| `stless` | StatelessWidget |
| `stful` | StatefulWidget |
| `initState` | initState 方法 |
| `dispose` | dispose 方法 |
| `build` | build 方法 |
| `column` | Column widget |
| `row` | Row widget |
| `container` | Container widget |
| `scaffold` | Scaffold widget |
| `materialapp` | MaterialApp |
| 以及更多... | |

## 架构

```
Zed 编辑器
  └── WASM 扩展 (flutter_extension)
       ├── Dart LSP 配置 → dart language-server
       ├── Flutter Bridge → flutter_bridge_server (原生二进制)
       │    └── 管理 → flutter daemon
       ├── DAP 调试器  → flutter debug-adapter
       ├── Slash Commands
       └── Snippets + Tree-sitter Grammar
```

## 开发

参见 [DEVELOPMENT.md](DEVELOPMENT.md)

## 许可证

Apache-2.0
