use zed_extension_api as zed;

mod dart_lsp;
mod flutter_bridge;
mod dap;
mod slash_commands;

/// Zed Flutter 扩展主结构体
struct FlutterExtension;

impl zed::Extension for FlutterExtension {
    /// 创建扩展实例（必需）
    fn new() -> Self {
        Self
    }

    /// 返回语言服务器启动命令
    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        match language_server_id.as_ref() {
            "dart" => dart_lsp::language_server_command(worktree),
            "flutter-bridge" => flutter_bridge::language_server_command(worktree),
            _ => Err(format!("未知的语言服务器 ID: {}", language_server_id)),
        }
    }

    /// 返回语言服务器初始化选项
    fn language_server_initialization_options(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<Option<serde_json::Value>> {
        match language_server_id.as_ref() {
            "dart" => dart_lsp::initialization_options(),
            "flutter-bridge" => flutter_bridge::initialization_options(),
            _ => Ok(None),
        }
    }

    /// 返回语言服务器的工作区配置
    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<Option<serde_json::Value>> {
        match language_server_id.as_ref() {
            "dart" => dart_lsp::workspace_configuration(worktree),
            _ => Ok(None),
        }
    }

    /// 为补全项生成显示标签
    fn label_for_completion(
        &self,
        _language_server_id: &zed::LanguageServerId,
        completion: zed::lsp::Completion,
    ) -> Option<zed::CodeLabel> {
        dart_lsp::label_for_completion(completion)
    }

    /// DAP 调试适配器 - 返回调试器启动命令
    fn get_dap_binary(
        &mut self,
        _adapter_name: String,
        _config: zed::DebugTaskDefinition,
        _user_installed_path: Option<String>,
        _worktree: &zed::Worktree,
    ) -> zed::Result<zed::DebugAdapterBinary> {
        dap::get_dap_binary()
    }

    /// DAP 调试适配器 - 确定启动/附加模式
    fn dap_request_kind(
        &mut self,
        _adapter_name: String,
        _config: serde_json::Value,
    ) -> zed::Result<zed::StartDebuggingRequestArgumentsRequest> {
        dap::dap_request_kind()
    }

    /// 将调试配置转换为调试场景
    fn dap_config_to_scenario(
        &mut self,
        _config: zed::DebugConfig,
    ) -> zed::Result<zed::DebugScenario> {
        dap::config_to_scenario()
    }

    /// Slash 命令处理
    fn run_slash_command(
        &self,
        command: zed::SlashCommand,
        args: Vec<String>,
        _worktree: Option<&zed::Worktree>,
    ) -> zed::Result<zed::SlashCommandOutput> {
        slash_commands::run(command, args)
    }
}

zed::register_extension!(FlutterExtension);
