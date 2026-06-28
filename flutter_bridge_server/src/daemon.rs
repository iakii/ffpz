//! Flutter Daemon 客户端
//!
//! 管理 `flutter daemon` 子进程的生命周期，通过 JSON-RPC over stdin/stdout 进行通信。
//! 参考：https://github.com/flutter/flutter/blob/master/packages/flutter_tools/doc/daemon.md

use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

use anyhow::{Context, Result};
use serde_json::Value;
use tracing::{debug, error, info, warn};

use crate::state::AppState;

/// Flutter Daemon 客户端
pub struct DaemonClient {
    child: Option<Child>,
    stdin: Option<ChildStdin>,
    reader: Option<BufReader<ChildStdout>>,
    request_id: AtomicU64,
    version: Option<String>,
    app_state: AppState,
}

impl DaemonClient {
    /// 创建新的 Daemon 客户端实例
    pub fn new() -> Self {
        Self {
            child: None,
            stdin: None,
            reader: None,
            request_id: AtomicU64::new(1),
            version: None,
            app_state: AppState::new(),
        }
    }

    /// 启动 flutter daemon 进程
    pub fn start(&mut self) -> Result<()> {
        info!("正在启动 flutter daemon...");

        let flutter_path = which::which("flutter")
            .or_else(|_| which::which("flutter.bat"))
            .context("未找到 flutter 命令。请确保 Flutter SDK 已安装并添加到 PATH。")?;

        info!("flutter 路径: {:?}", flutter_path);

        let mut child = Command::new(&flutter_path)
            .args(["daemon"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .context("无法启动 flutter daemon 进程")?;

        let stdin = child.stdin.take().context("无法获取 daemon 进程的 stdin")?;
        let stdout = child.stdout.take().context("无法获取 daemon 进程的 stdout")?;

        self.child = Some(child);
        self.stdin = Some(stdin);
        self.reader = Some(BufReader::new(stdout));

        // 发送 daemon.version 握手
        let version = self.send_command("daemon.version", Value::Null)?;
        self.version = version.as_str().map(|s| s.to_string());
        info!("Flutter Daemon 已连接，协议版本: {:?}", self.version);

        // 启用设备发现
        self.send_command("device.enable", Value::Null)?;
        info!("设备发现已启用");

        Ok(())
    }

    /// 发送 JSON-RPC 命令到 daemon 并等待响应
    ///
    /// 注意：此方法会在等待响应过程中消费 daemon 的事件通知，
    /// 但只返回与请求 ID 匹配的响应消息。
    pub fn send_command(&mut self, method: &str, params: Value) -> Result<Value> {
        let stdin = self.stdin.as_mut().context("daemon 未启动")?;
        let id = self.request_id.fetch_add(1, Ordering::SeqCst);

        // 构建并发送 JSON-RPC 请求
        let request = serde_json::json!([{
            "id": id,
            "method": method,
            "params": params,
        }]);

        debug!("发送请求: {}", request);

        writeln!(stdin, "{}", request).context("无法写入 daemon stdin")?;
        stdin.flush().context("无法刷新 daemon stdin")?;

        // 读取响应（在等待当前请求的响应时处理其他事件）
        self.wait_for_response(id)
    }

    /// 等待指定 ID 的响应，期间处理接收到的事件通知
    fn wait_for_response(&mut self, target_id: u64) -> Result<Value> {
        // 注意：我们需要小心处理借用问题。
        // 读取一行后释放 reader 的借用，再处理事件。
        loop {
            let line = self.read_line()?;
            if let Some(msg) = DaemonClient::parse_daemon_message(&line)? {
                // 检查是否是目标响应
                if let Some(id) = msg.get("id").and_then(|v| v.as_u64()) {
                    if id == target_id {
                        return Ok(msg.get("result").cloned().unwrap_or(Value::Null));
                    }
                }

                // 是事件通知，处理它
                if msg.get("event").is_some() {
                    self.handle_event(msg)?;
                }
            }
        }
    }

    /// 从 daemon stdout 读取一行
    fn read_line(&mut self) -> Result<String> {
        let reader = self.reader.as_mut().context("daemon reader 未初始化")?;
        let mut line = String::new();
        reader.read_line(&mut line).context("无法读取 daemon stdout")?;
        Ok(line.trim().to_string())
    }

    /// 解析 daemon 消息行
    fn parse_daemon_message(line: &str) -> Result<Option<Value>> {
        if line.is_empty() {
            return Ok(None);
        }
        let parsed: Vec<Value> = serde_json::from_str(line)
            .context("无法解析 daemon 响应")?;
        Ok(parsed.into_iter().next())
    }

    /// 处理 daemon 事件通知
    fn handle_event(&mut self, msg: Value) -> Result<()> {
        let event = msg.get("event").and_then(|v| v.as_str()).unwrap_or("unknown");
        let params = msg.get("params");

        debug!("收到 daemon 事件: {}", event);

        match event {
            "daemon.connected" => {
                info!("Daemon 连接确认");
            }
            "device.added" => {
                if let Some(p) = params {
                    let device = crate::state::DeviceEntry::from_daemon(p.clone());
                    self.app_state.add_device(device);
                    info!("设备已接入");
                }
            }
            "device.removed" => {
                if let Some(p) = params {
                    if let Some(id) = p.get("id").and_then(|v| v.as_str()) {
                        self.app_state.remove_device(id);
                        info!("设备已断开: {}", id);
                    }
                }
            }
            "app.start" => {
                info!("应用正在启动");
            }
            "app.debugPort" => {
                info!("调试端口可用");
            }
            "app.started" => {
                info!("应用已启动");
            }
            "app.log" => {
                if let Some(p) = params {
                    if let Some(log) = p.get("log").and_then(|v| v.as_str()) {
                        debug!("应用日志: {}", log);
                    }
                }
            }
            "app.stop" => {
                info!("应用已停止");
            }
            _ => {
                debug!("未处理事件: {}", event);
            }
        }

        Ok(())
    }

    // ── 公共 API ──

    /// 获取已连接的设备列表
    pub fn get_devices(&mut self) -> Result<Vec<Value>> {
        Ok(self.app_state.get_devices_json())
    }

    /// 获取可用的模拟器列表
    pub fn get_emulators(&mut self) -> Result<Vec<Value>> {
        let result = self.send_command("emulator.getEmulators", Value::Null)?;
        Ok(result.as_array().cloned().unwrap_or_default())
    }

    /// 启动模拟器
    pub fn launch_emulator(&mut self, emulator_id: &str) -> Result<Value> {
        let params = serde_json::json!({ "emulatorId": emulator_id });
        self.send_command("emulator.launch", params)
    }

    /// 启动 Flutter 应用
    pub fn start_app(&mut self, device_id: &str, project_root: &str, mode: &str, target: &str) -> Result<Value> {
        let params = serde_json::json!({
            "deviceId": device_id,
            "projectDirectory": project_root,
            "mode": mode,
            "target": target,
        });
        self.send_command("app.start", params)
    }

    /// 触发热重载或热重启
    pub fn restart_app(&mut self, app_id: &str, full_restart: bool) -> Result<Value> {
        let params = serde_json::json!({
            "appId": app_id,
            "fullRestart": full_restart,
        });
        self.send_command("app.restart", params)
    }

    /// 停止应用
    pub fn stop_app(&mut self, app_id: &str) -> Result<Value> {
        let params = serde_json::json!({ "appId": app_id });
        self.send_command("app.stop", params)
    }

    /// 调用 VM Service 扩展方法
    pub fn call_service_extension(&mut self, app_id: &str, method: &str, params: Value) -> Result<Value> {
        let request = serde_json::json!({
            "appId": app_id,
            "methodName": method,
            "params": params,
        });
        self.send_command("app.callServiceExtension", request)
    }

    /// 启动 DevTools 服务器
    pub fn serve_devtools(&mut self) -> Result<Value> {
        self.send_command("devtools.serve", Value::Null)
    }

    /// 关闭 daemon
    pub fn shutdown(&mut self) -> Result<()> {
        info!("正在关闭 Flutter Daemon...");

        if let Err(e) = self.send_command("daemon.shutdown", Value::Null) {
            warn!("发送 shutdown 请求失败: {}", e);
        }

        if let Some(mut child) = self.child.take() {
            std::thread::sleep(std::time::Duration::from_millis(500));
            match child.try_wait() {
                Ok(Some(status)) => info!("Daemon 退出，状态: {}", status),
                Ok(None) => {
                    warn!("Daemon 未响应 shutdown，强制终止");
                    let _ = child.kill();
                    let _ = child.wait();
                }
                Err(e) => {
                    error!("等待 daemon 退出失败: {}", e);
                    let _ = child.kill();
                }
            }
        }

        Ok(())
    }
}

impl Drop for DaemonClient {
    fn drop(&mut self) {
        if self.child.is_some() {
            let _ = self.shutdown();
        }
    }
}
