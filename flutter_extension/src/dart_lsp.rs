use zed_extension_api as zed;

/// 启动 Dart Analysis Server（LSP）
///
/// 查找 dart 命令（优先使用 Flutter SDK 捆绑的 Dart），然后启动语言服务器。
/// 查找顺序：
/// 1. 工作目录 PATH 中的 dart
/// 2. 工作目录 PATH 中的 flutter（使用捆绑 Dart）
pub fn language_server_command(worktree: &zed::Worktree) -> zed::Result<zed::Command> {
    // 尝试在 PATH 中找到 dart 命令
    if let Some(dart_path) = worktree.which("dart") {
        return Ok(zed::Command {
            command: dart_path,
            args: vec!["language-server".into(), "--protocol=lsp".into()],
            env: vec![],
        });
    }

    // 如果 dart 未找到，尝试找到 flutter 并使用其捆绑的 dart
    if let Some(flutter_path) = worktree.which("flutter") {
        return Ok(zed::Command {
            command: flutter_path,
            args: vec![
                "dart".into(),
                "language-server".into(),
                "--protocol=lsp".into(),
            ],
            env: vec![],
        });
    }

    Err("未找到 dart 或 flutter 命令。请确保已安装 Dart SDK 或 Flutter SDK。".into())
}

/// 返回 Dart LSP 初始化选项
pub fn initialization_options() -> zed::Result<Option<serde_json::Value>> {
    Ok(Some(serde_json::json!({
        "suggestFromUnimportedLibraries": true,
        "closingLabels": true,
        "outline": true,
        "flutterOutline": true,
        "onlyAnalyzeProjectsWithOpenFiles": false,
    })))
}

/// 返回 Dart LSP 工作区配置
///
/// 使用 Zed 的设置系统读取 dart.* 配置项。
pub fn workspace_configuration(worktree: &zed::Worktree) -> zed::Result<Option<serde_json::Value>> {
    // 通过 zed settings API 读取配置
    // 注意：在 WASM 环境中通过 get_settings 函数从 Zed 的设置系统中读取
    let _ = worktree; // 后续可用 settings API 读取 dart.lineLength 等设置

    Ok(Some(serde_json::json!({
        "dart": {
            "lineLength": 80_u64,
            "completeFunctionCalls": false,
            "showTodos": true,
        }
    })))
}

/// 为补全项生成显示标签
///
/// 使用补全项的 detail 字段（通常包含返回类型）作为显示标签。
pub fn label_for_completion(completion: zed::lsp::Completion) -> Option<zed::CodeLabel> {
    let _detail = completion.detail?;
    let label_text = completion.label.clone();
    let label_len = label_text.len() as u32;

    Some(zed::CodeLabel {
        code: label_text.clone(),
        spans: vec![zed::CodeLabelSpan::literal(label_text, None)],
        filter_range: zed::Range { start: 0, end: label_len },
    })
}
