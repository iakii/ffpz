//! LSP Server 实现
//!
//! 使用 `lsp-server` crate 的 Connection API 与 WASM 扩展通信。
//! 处理自定义 flutter/* 方法的请求和响应。

use anyhow::{Context, Result};
use lsp_server::{Connection, Message, Response};
use lsp_types::{
    ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
};
use serde_json::Value;
use tracing::{debug, error, info, warn};

use crate::daemon::DaemonClient;
use crate::protocols::methods;

/// LSP 服务器
pub struct LspServer {
    /// LSP 连接
    connection: Connection,
    /// Flutter daemon 客户端
    daemon: Option<DaemonClient>,
}

impl LspServer {
    /// 创建新的 LSP 服务器（通过 stdio）
    pub fn new_stdio() -> (Self, lsp_server::IoThreads) {
        let (connection, io_threads) = Connection::stdio();
        (Self {
            connection,
            daemon: None,
        }, io_threads)
    }

    /// 主循环：处理 LSP 消息
    pub fn run(&mut self) -> Result<()> {
        info!("Flutter Bridge Server 启动中...");

        // 1. 执行 LSP 初始化握手
        let server_caps = serde_json::json!(ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
            ..Default::default()
        });

        let _init_params = self.connection
            .initialize(server_caps)
            .context("LSP 初始化握手失败")?;

        info!("LSP 连接已初始化");

        // 2. 处理消息循环
        // 注意：不能使用 for msg in &self.connection.receiver 因为那会创建不可变借用，
        // 与 handle_request 的可变借用冲突。
        loop {
            let msg = match self.connection.receiver.recv() {
                Ok(msg) => msg,
                Err(_) => {
                    info!("LSP 连接已关闭");
                    break;
                }
            };

            match msg {
                Message::Request(req) => {
                    // 检查是否是 shutdown 请求
                    if self.connection.handle_shutdown(&req)? {
                        info!("收到 shutdown 请求");
                        break;
                    }

                    // 处理自定义 flutter/* 请求
                    if let Err(e) = self.handle_request(req) {
                        error!("处理请求失败: {}", e);
                    }
                }
                Message::Notification(notif) => {
                    match notif.method.as_str() {
                        "exit" => {
                            info!("收到 exit 通知");
                            break;
                        }
                        _ => {
                            debug!("忽略通知: {}", notif.method);
                        }
                    }
                }
                Message::Response(resp) => {
                    debug!("收到意外的响应: {:?}", resp);
                }
            }
        }

        // 3. 清理
        self.shutdown()?;
        Ok(())
    }

    /// 处理自定义请求
    fn handle_request(&mut self, req: lsp_server::Request) -> Result<()> {
        let method = req.method.as_str();
        debug!("收到请求: {} id={:?}", method, req.id);

        let result = match method {
            methods::DAEMON_START => self.handle_daemon_start(req.params),
            methods::DAEMON_SHUTDOWN => self.handle_daemon_shutdown(),
            methods::DEVICE_GET_DEVICES => self.handle_get_devices(),
            methods::DEVICE_ENABLE => self.handle_device_enable(),
            methods::DEVICE_DISABLE => self.handle_device_disable(),
            methods::EMULATOR_GET_EMULATORS => self.handle_get_emulators(),
            methods::EMULATOR_LAUNCH => self.handle_launch_emulator(req.params),
            methods::APP_START => self.handle_app_start(req.params),
            methods::APP_RESTART => self.handle_app_restart(req.params),
            methods::APP_STOP => self.handle_app_stop(req.params),
            methods::APP_CALL_SERVICE_EXTENSION => self.handle_call_service_extension(req.params),
            methods::DEVTOOLS_SERVE => self.handle_devtools_serve(),
            _ => {
                warn!("未知方法: {}", method);
                Err(anyhow::anyhow!("未知方法: {}", method))
            }
        };

        // 发送响应
        let response = match result {
            Ok(result) => Response::new_ok(req.id, result),
            Err(err) => Response::new_err(
                req.id,
                lsp_server::ErrorCode::InternalError as i32,
                format!("{}", err),
            ),
        };

        self.connection
            .sender
            .send(response.into())
            .context("发送 LSP 响应失败")?;

        Ok(())
    }

    // ── Daemon 管理 ──

    fn ensure_daemon(&mut self) -> Result<&mut DaemonClient> {
        if self.daemon.is_none() {
            let mut daemon = DaemonClient::new();
            daemon.start()?;
            self.daemon = Some(daemon);
        }
        Ok(self.daemon.as_mut().unwrap())
    }

    fn handle_daemon_start(&mut self, _params: Value) -> Result<Value> {
        if let Some(mut daemon) = self.daemon.take() {
            daemon.shutdown()?;
        }

        let mut daemon = DaemonClient::new();
        daemon.start()?;
        let devices = daemon.get_devices()?;
        self.daemon = Some(daemon);

        Ok(serde_json::json!({
            "success": true,
            "devices": devices,
        }))
    }

    fn handle_daemon_shutdown(&mut self) -> Result<Value> {
        if let Some(mut daemon) = self.daemon.take() {
            daemon.shutdown()?;
        }
        Ok(serde_json::json!({ "success": true }))
    }

    // ── 设备管理 ──

    fn handle_get_devices(&mut self) -> Result<Value> {
        let devices = self.ensure_daemon()?.get_devices()?;
        Ok(serde_json::json!({ "devices": devices }))
    }

    fn handle_device_enable(&mut self) -> Result<Value> {
        Ok(serde_json::json!({ "success": true }))
    }

    fn handle_device_disable(&mut self) -> Result<Value> {
        let daemon = self.ensure_daemon()?;
        daemon.send_command("device.disable", Value::Null)
    }

    // ── 模拟器管理 ──

    fn handle_get_emulators(&mut self) -> Result<Value> {
        let emulators = self.ensure_daemon()?.get_emulators()?;
        Ok(serde_json::json!({ "emulators": emulators }))
    }

    fn handle_launch_emulator(&mut self, params: Value) -> Result<Value> {
        let emulator_id = params
            .get("emulatorId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("缺少 emulatorId 参数"))?;
        self.ensure_daemon()?.launch_emulator(emulator_id)
    }

    // ── 应用管理 ──

    fn handle_app_start(&mut self, params: Value) -> Result<Value> {
        let device_id = params
            .get("deviceId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("缺少 deviceId 参数"))?;
        let project_root = params
            .get("projectRoot")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("缺少 projectRoot 参数"))?;
        let mode = params
            .get("mode")
            .and_then(|v| v.as_str())
            .unwrap_or("debug");
        let target = params
            .get("target")
            .and_then(|v| v.as_str())
            .unwrap_or("lib/main.dart");

        self.ensure_daemon()?
            .start_app(device_id, project_root, mode, target)
    }

    fn handle_app_restart(&mut self, params: Value) -> Result<Value> {
        let app_id = params
            .get("appId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("缺少 appId 参数"))?;
        let full_restart = params
            .get("fullRestart")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        self.ensure_daemon()?.restart_app(app_id, full_restart)
    }

    fn handle_app_stop(&mut self, params: Value) -> Result<Value> {
        let app_id = params
            .get("appId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("缺少 appId 参数"))?;
        self.ensure_daemon()?.stop_app(app_id)
    }

    fn handle_call_service_extension(&mut self, params: Value) -> Result<Value> {
        let app_id = params
            .get("appId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("缺少 appId 参数"))?;
        let method = params
            .get("methodName")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("缺少 methodName 参数"))?;
        let ext_params = params.get("params").cloned().unwrap_or(Value::Null);

        self.ensure_daemon()?
            .call_service_extension(app_id, method, ext_params)
    }

    fn handle_devtools_serve(&mut self) -> Result<Value> {
        self.ensure_daemon()?.serve_devtools()
    }

    /// 关闭服务器
    fn shutdown(&mut self) -> Result<()> {
        info!("正在关闭 Bridge Server...");
        if let Some(mut daemon) = self.daemon.take() {
            daemon.shutdown()?;
        }
        Ok(())
    }
}
