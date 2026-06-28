use zed_extension_api as zed;

mod dart_lsp;
mod dap;

struct FlutterExtension;

impl zed::Extension for FlutterExtension {
    fn new() -> Self {
        Self
    }

    /// Dart LSP 启动
    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        match language_server_id.as_ref() {
            "dart" => dart_lsp::language_server_command(worktree),
            _ => Err(format!("未知语言服务器: {}", language_server_id)),
        }
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<Option<serde_json::Value>> {
        match language_server_id.as_ref() {
            "dart" => dart_lsp::initialization_options(),
            _ => Ok(None),
        }
    }

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

    /// Flutter DAP 调试适配器
    fn get_dap_binary(
        &mut self,
        _adapter_name: String,
        _config: zed::DebugTaskDefinition,
        _user_installed_path: Option<String>,
        _worktree: &zed::Worktree,
    ) -> zed::Result<zed::DebugAdapterBinary> {
        dap::get_dap_binary()
    }

    fn dap_request_kind(
        &mut self,
        _adapter_name: String,
        _config: serde_json::Value,
    ) -> zed::Result<zed::StartDebuggingRequestArgumentsRequest> {
        dap::dap_request_kind()
    }

    fn dap_config_to_scenario(
        &mut self,
        _config: zed::DebugConfig,
    ) -> zed::Result<zed::DebugScenario> {
        dap::config_to_scenario()
    }
}

zed::register_extension!(FlutterExtension);
