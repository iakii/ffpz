//! Flutter Bridge Server
//!
//! 一个原生 Rust 二进制程序，作为 Zed Flutter 扩展与 Flutter 工具链之间的桥梁。
//!
//! 职责：
//! 1. 通过自定义 LSP 协议与 WASM 扩展通信
//! 2. 管理 `flutter daemon` 子进程的生命周期
//! 3. 处理设备发现、应用运行、热重载等操作
//!
//! 通信协议：
//! - 使用 LSP over stdio 与 WASM 扩展交互
//! - 使用 JSON-RPC over stdin/stdout 与 flutter daemon 交互

mod daemon;
mod lsp_server;
mod protocols;
mod state;

/// Bridge Server 入口
///
/// 初始化日志系统，创建 LSP 服务器，然后运行消息处理循环。
fn main() -> anyhow::Result<()> {
    // 初始化结构化日志（输出到 stderr，因为 stdout 被 LSP 使用）
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .with_writer(std::io::stderr)
        .init();

    tracing::info!(
        "Flutter Bridge Server v{} 启动 ({} {})",
        env!("CARGO_PKG_VERSION"),
        std::env::consts::OS,
        std::env::consts::ARCH,
    );

    // 创建并运行 LSP 服务器（stdio 模式）
    let (mut server, _io_threads) = lsp_server::LspServer::new_stdio();
    server.run()?;

    tracing::info!("Flutter Bridge Server 正常退出");
    Ok(())
}
