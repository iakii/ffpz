//! 全局状态管理
//!
//! 管理 Bridge Server 的核心状态：已连接的设备列表、运行中的应用列表等。
//!
//! 注意：部分方法为后续阶段预留（Phase 2 和 Phase 3），
//! 当前阶段未被使用但保留以保持 API 的完整性。

#![allow(dead_code)]

use serde_json::Value;
use std::collections::HashMap;
use std::sync::Mutex;

/// 设备条目
#[derive(Debug, Clone)]
pub struct DeviceEntry {
    pub id: String,
    pub name: String,
    pub platform: String,
    pub platform_type: String,
    pub category: Option<String>,
    pub ephemeral: bool,
    pub emulator: bool,
    pub emulator_id: Option<String>,
    pub sdk: Option<String>,
}

impl DeviceEntry {
    /// 从 JSON-RPC 响应中创建设备条目
    ///
    /// 解析 flutter daemon 返回的设备对象，提取设备信息。
    pub fn from_daemon(params: Value) -> Self {
        Self {
            id: params.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            name: params.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown").to_string(),
            platform: params.get("platform").and_then(|v| v.as_str()).unwrap_or("unknown").to_string(),
            platform_type: params.get("platformType").and_then(|v| v.as_str()).unwrap_or("unknown").to_string(),
            category: params.get("category").and_then(|v| v.as_str()).map(|s| s.to_string()),
            ephemeral: params.get("ephemeral").and_then(|v| v.as_bool()).unwrap_or(false),
            emulator: params.get("emulator").and_then(|v| v.as_bool()).unwrap_or(false),
            emulator_id: params.get("emulatorId").and_then(|v| v.as_str()).map(|s| s.to_string()),
            sdk: params.get("sdk").and_then(|v| v.as_str()).map(|s| s.to_string()),
        }
    }

    /// 转换为 JSON 值
    pub fn to_json(&self) -> Value {
        serde_json::json!({
            "id": self.id,
            "name": self.name,
            "platform": self.platform,
            "platformType": self.platform_type,
            "category": self.category,
            "ephemeral": self.ephemeral,
            "emulator": self.emulator,
            "emulatorId": self.emulator_id,
            "sdk": self.sdk,
        })
    }
}

/// 应用条目
#[derive(Debug, Clone)]
pub struct AppEntry {
    pub app_id: String,
    pub device_id: String,
    pub project_directory: String,
    pub mode: String,
    pub ws_uri: Option<String>,
    pub dtd_uri: Option<String>,
    pub started_at: std::time::Instant,
}

/// 应用状态管理器
///
/// 维护当前运行的设备和应用列表，提供线程安全的访问。
#[derive(Debug)]
pub struct AppState {
    devices: Mutex<HashMap<String, DeviceEntry>>,
    apps: Mutex<HashMap<String, AppEntry>>,
}

impl AppState {
    /// 创建新的应用状态管理器
    pub fn new() -> Self {
        Self {
            devices: Mutex::new(HashMap::new()),
            apps: Mutex::new(HashMap::new()),
        }
    }

    // ── 设备管理 ──

    /// 添加设备
    pub fn add_device(&self, device: DeviceEntry) {
        let mut devices = self.devices.lock().unwrap();
        devices.insert(device.id.clone(), device);
    }

    /// 移除设备
    pub fn remove_device(&self, device_id: &str) {
        let mut devices = self.devices.lock().unwrap();
        devices.remove(device_id);
    }

    /// 获取所有设备
    pub fn get_devices(&self) -> Vec<DeviceEntry> {
        let devices = self.devices.lock().unwrap();
        devices.values().cloned().collect()
    }

    /// 获取所有设备（JSON 格式）
    pub fn get_devices_json(&self) -> Vec<Value> {
        let devices = self.devices.lock().unwrap();
        devices.values().map(|d| d.to_json()).collect()
    }

    /// 获取设备数量
    pub fn device_count(&self) -> usize {
        let devices = self.devices.lock().unwrap();
        devices.len()
    }

    // ── 应用管理 ──

    /// 添加应用
    pub fn add_app(&self, app: AppEntry) {
        let mut apps = self.apps.lock().unwrap();
        apps.insert(app.app_id.clone(), app);
    }

    /// 移除应用
    pub fn remove_app(&self, app_id: &str) {
        let mut apps = self.apps.lock().unwrap();
        apps.remove(app_id);
    }

    /// 获取运行中的应用列表
    pub fn get_apps(&self) -> Vec<AppEntry> {
        let apps = self.apps.lock().unwrap();
        apps.values().cloned().collect()
    }

    /// 检查是否有应用在运行
    pub fn has_running_apps(&self) -> bool {
        let apps = self.apps.lock().unwrap();
        !apps.is_empty()
    }
}
