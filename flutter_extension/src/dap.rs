use zed_extension_api as zed;

/// 获取 Flutter 调试适配器启动命令
///
/// 使用 Flutter SDK 内置的 DAP 实现。
pub fn get_dap_binary() -> zed::Result<zed::DebugAdapterBinary> {
    Ok(zed::DebugAdapterBinary {
        command: Some("flutter".to_string()),
        arguments: vec!["debug-adapter".to_string()],
        envs: vec![],
        cwd: None,
        connection: None,
        request_args: zed::StartDebuggingRequestArguments {
            configuration: serde_json::json!({
                "request": "launch",
                "type": "dart",
                "program": "lib/main.dart",
                "flutterMode": "debug",
            })
            .to_string(),
            request: zed::StartDebuggingRequestArgumentsRequest::Launch,
        },
    })
}

/// 确定调试请求类型（launch/attach）
pub fn dap_request_kind() -> zed::Result<zed::StartDebuggingRequestArgumentsRequest> {
    Ok(zed::StartDebuggingRequestArgumentsRequest::Launch)
}

/// 将调试配置转换为调试场景
///
/// 处理用户提供的调试配置，转化为 DAP 可识别的格式。
pub fn config_to_scenario() -> zed::Result<zed::DebugScenario> {
    Ok(zed::DebugScenario {
        label: "Flutter Debug".to_string(),
        adapter: "flutter".to_string(),
        build: None,
        config: serde_json::json!({
            "request": "launch",
            "type": "dart",
            "program": "lib/main.dart",
            "flutterMode": "debug",
        })
        .to_string(),
        tcp_connection: None,
    })
}
