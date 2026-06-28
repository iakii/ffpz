//! 自定义 LSP 协议类型定义
//!
//! 定义了 Flutter Bridge Server 与 WASM 扩展之间通信的自定义协议类型。
//! 协议基于 LSP 的 JSON-RPC 机制，使用自定义方法名进行通信。
//!
//! 注意：部分类型为后续阶段预留（Phase 2 和 Phase 3），
//! 当前阶段未被使用但保留以保持协议的完整性。

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

// ── 设备相关类型 ─────────────────────────────────────

/// 设备信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// 设备唯一标识符
    pub id: String,
    /// 人类可读的设备名称
    pub name: String,
    /// 平台：ios, android, linux, macos, windows, web
    pub platform: String,
    /// 平台类型（更详细的分类）
    pub platform_type: String,
    /// 设备分类：mobile, web, desktop
    pub category: Option<String>,
    /// 是否为临时连接（如 USB 真机）
    pub ephemeral: bool,
    /// 是否为模拟器
    pub emulator: bool,
    /// 模拟器 ID
    pub emulator_id: Option<String>,
    /// SDK 版本信息
    pub sdk: Option<String>,
    /// 设备能力
    pub capabilities: Option<DeviceCapabilities>,
}

/// 设备能力
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCapabilities {
    pub hot_reload: bool,
    pub hot_restart: bool,
    pub screenshot: bool,
    pub flutter_exit: bool,
    pub hardware_rendering: bool,
    pub start_paused: bool,
}

// ── 模拟器相关类型 ───────────────────────────────────

/// 模拟器信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmulatorInfo {
    pub id: String,
    pub name: String,
    pub platform: String,
    pub category: Option<String>,
}

// ── 应用相关类型 ─────────────────────────────────────

/// 应用信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo {
    /// 应用实例 ID（daemon 分配）
    pub app_id: String,
    /// 运行应用的设备 ID
    pub device_id: String,
    /// 应用项目目录
    pub project_directory: String,
    /// 运行模式
    pub mode: String,
    /// 应用状态
    pub status: AppStatus,
    /// VM Service WebSocket URI（调试用）
    pub ws_uri: Option<String>,
    /// DTD URI
    pub dtd_uri: Option<String>,
}

/// 应用状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AppStatus {
    Starting,
    Running,
    Debugging,
    Stopped,
    Error(String),
}

// ── Daemon 状态 ──────────────────────────────────────

/// Flutter Daemon 连接状态
#[derive(Debug, Clone)]
pub enum DaemonState {
    Disconnected,
    Connecting,
    Connected {
        version: String,
        pid: u32,
    },
    Error(String),
}

// ── 自定义 LSP 方法名 ──────────────────────────────

/// 自定义 LSP 方法名常量
pub mod methods {
    // ── Daemon 管理 ──
    pub const DAEMON_START: &str = "flutter/daemon/start";
    pub const DAEMON_SHUTDOWN: &str = "flutter/daemon/shutdown";
    pub const DAEMON_CONNECTED: &str = "flutter/daemon/connected";
    pub const DAEMON_LOG: &str = "flutter/daemon/log";

    // ── 设备管理 ──
    pub const DEVICE_GET_DEVICES: &str = "flutter/device/getDevices";
    pub const DEVICE_ENABLE: &str = "flutter/device/enable";
    pub const DEVICE_DISABLE: &str = "flutter/device/disable";
    pub const DEVICE_ADDED: &str = "flutter/device/added";
    pub const DEVICE_REMOVED: &str = "flutter/device/removed";

    // ── 模拟器管理 ──
    pub const EMULATOR_GET_EMULATORS: &str = "flutter/emulator/getEmulators";
    pub const EMULATOR_LAUNCH: &str = "flutter/emulator/launch";

    // ── 应用管理 ──
    pub const APP_START: &str = "flutter/app/start";
    pub const APP_RESTART: &str = "flutter/app/restart";
    pub const APP_STOP: &str = "flutter/app/stop";
    pub const APP_DEBUG_PORT: &str = "flutter/app/debugPort";
    pub const APP_STARTED: &str = "flutter/app/started";
    pub const APP_LOG: &str = "flutter/app/log";
    pub const APP_STOP_EVENT: &str = "flutter/app/stop";

    // ── 调试服务扩展 ──
    pub const APP_CALL_SERVICE_EXTENSION: &str = "flutter/app/callServiceExtension";

    // ── DevTools ──
    pub const DEVTOOLS_SERVE: &str = "flutter/devtools/serve";
}
