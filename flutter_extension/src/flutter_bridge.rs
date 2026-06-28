use zed_extension_api as zed;

/// Flutter Bridge Server 管理
///
/// Bridge Server 是一个原生二进制程序，负责管理 flutter daemon 进程。
/// 通过 GitHub Releases 自动下载，或使用本地已安装的版本。
const BRIDGE_SERVER_REPO: &str = "your-org/zed-flutter-plugin";

/// 获取 Bridge Server 启动命令
///
/// 返回一个 Command，Zed 将用它来启动 Bridge Server 进程。
pub fn language_server_command(worktree: &zed::Worktree) -> zed::Result<zed::Command> {
    let server_path = get_or_download_bridge_server(worktree)?;

    Ok(zed::Command {
        command: server_path,
        args: vec![],
        env: vec![],
    })
}

/// 返回 Bridge Server 的初始化选项
pub fn initialization_options() -> zed::Result<Option<serde_json::Value>> {
    Ok(Some(serde_json::json!({
        "name": "flutter-bridge",
    })))
}

/// 获取或下载 Bridge Server 二进制
///
/// 查找顺序：
/// 1. PATH 中的 flutter-bridge-server（开发模式）
/// 2. 缓存目录中已有的二进制
/// 3. 从 GitHub Releases 下载
fn get_or_download_bridge_server(worktree: &zed::Worktree) -> zed::Result<String> {
    let (os, arch) = zed::current_platform();
    let binary_name = bridge_binary_name(os);

    // 1. 先在 PATH 中查找（开发模式）
    if let Some(path) = worktree.which(&binary_name) {
        return Ok(path);
    }

    // 2. 检查本地缓存目录
    let cache_dir = bridge_cache_dir();
    let cached_path = format!("{}/{}", cache_dir, binary_name);

    if std::path::Path::new(&cached_path).exists() {
        return Ok(cached_path);
    }

    // 3. 从 GitHub Releases 下载
    let platform_tag = bridge_platform_tag(os, arch);
    let release = zed::latest_github_release(
        BRIDGE_SERVER_REPO,
        zed::GithubReleaseOptions {
            require_assets: true,
            pre_release: false,
        },
    )?;

    // 找到匹配当前平台的资产
    let asset = release
        .assets
        .iter()
        .find(|a| a.name.contains(&platform_tag))
        .ok_or_else(|| format!("未找到 {} 平台的 Bridge Server 二进制", platform_tag))?;

    // 下载二进制
    zed::download_file(&asset.download_url, &cached_path, zed::DownloadedFileType::Uncompressed)?;

    // 在非 Windows 平台上设置可执行权限
    if os != zed::Os::Windows {
        zed::make_file_executable(&cached_path)?;
    }

    Ok(cached_path)
}

/// 返回 Bridge Server 的缓存目录路径
fn bridge_cache_dir() -> String {
    // 使用默认的工作目录下的子目录
    "flutter_bridge_cache".to_string()
}

/// 返回平台特定的二进制名称
fn bridge_binary_name(os: zed::Os) -> &'static str {
    match os {
        zed::Os::Windows => "flutter-bridge-server.exe",
        _ => "flutter-bridge-server",
    }
}

/// 返回平台标签（用于 GitHub Release asset 命名）
fn bridge_platform_tag(os: zed::Os, arch: zed::Architecture) -> String {
    let os_str = match os {
        zed::Os::Mac => "macos",
        zed::Os::Linux => "linux",
        zed::Os::Windows => "windows",
    };
    let arch_str = match arch {
        zed::Architecture::Aarch64 => "aarch64",
        zed::Architecture::X8664 => "x86_64",
        zed::Architecture::X86 => "x86",
    };
    format!("{}-{}", os_str, arch_str)
}
