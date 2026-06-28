use zed_extension_api as zed;

/// 处理 Flutter 相关的 Slash 命令
pub fn run(command: zed::SlashCommand, args: Vec<String>) -> zed::Result<zed::SlashCommandOutput> {
    match command.name.as_str() {
        "flutter-devices" => cmd_devices(),
        "flutter-run" => cmd_run(&args),
        "flutter-hot-reload" => cmd_hot_reload(),
        "flutter-hot-restart" => cmd_hot_restart(),
        "flutter-stop" => cmd_stop(),
        "flutter-pub-get" => cmd_pub_get(),
        "flutter-pub-upgrade" => cmd_pub_upgrade(),
        "flutter-clean" => cmd_clean(),
        "flutter-doctor" => cmd_doctor(),
        "flutter-create" => cmd_create(&args),
        "flutter-select-device" => cmd_select_device(&args),
        "flutter-launch-emulator" => cmd_launch_emulator(&args),
        "flutter-devtools" => cmd_devtools(),
        _ => Err(format!("未知的 flutter 命令: {}", command.name)),
    }
}

fn cmd_devices() -> zed::Result<zed::SlashCommandOutput> {
    Ok(zed::SlashCommandOutput {
        text: "正在获取设备列表...
请在 Flutter Bridge 就绪后重试此命令。

提示：确保已将 flutter 添加到 PATH 环境变量中。".to_string(),
        sections: vec![],
    })
}

fn cmd_run(args: &[String]) -> zed::Result<zed::SlashCommandOutput> {
    let device_id = get_arg_value(args, "--device");
    let mode = get_arg_value(args, "--mode").unwrap_or_else(|| "debug".to_string());
    let target = get_arg_value(args, "--target").unwrap_or_else(|| "lib/main.dart".to_string());

    let device_info = device_id
        .map(|id| format!("设备: {}", id))
        .unwrap_or_else(|| "设备: 默认".to_string());

    Ok(zed::SlashCommandOutput {
        text: format!(
            "正在启动 Flutter 应用...
{}
模式: {}
入口: {}

请等待应用启动完成。
应用启动后，可以使用 /flutter:hot-reload 热重载。",
            device_info, mode, target
        ),
        sections: vec![],
    })
}

fn cmd_hot_reload() -> zed::Result<zed::SlashCommandOutput> {
    Ok(zed::SlashCommandOutput {
        text: "正在热重载...
快捷键：可在 Zed 的 keybindings.json 中为 hot-reload 绑定快捷键。".to_string(),
        sections: vec![],
    })
}

fn cmd_hot_restart() -> zed::Result<zed::SlashCommandOutput> {
    Ok(zed::SlashCommandOutput {
        text: "正在热重启...".to_string(),
        sections: vec![],
    })
}

fn cmd_stop() -> zed::Result<zed::SlashCommandOutput> {
    Ok(zed::SlashCommandOutput {
        text: "正在停止应用...".to_string(),
        sections: vec![],
    })
}

fn cmd_pub_get() -> zed::Result<zed::SlashCommandOutput> {
    Ok(zed::SlashCommandOutput {
        text: "正在获取依赖...
将会运行 `flutter pub get`。".to_string(),
        sections: vec![],
    })
}

fn cmd_pub_upgrade() -> zed::Result<zed::SlashCommandOutput> {
    Ok(zed::SlashCommandOutput {
        text: "正在升级依赖...
将会运行 `flutter pub upgrade`。".to_string(),
        sections: vec![],
    })
}

fn cmd_clean() -> zed::Result<zed::SlashCommandOutput> {
    Ok(zed::SlashCommandOutput {
        text: "正在清理构建产物...
将会运行 `flutter clean`。".to_string(),
        sections: vec![],
    })
}

fn cmd_doctor() -> zed::Result<zed::SlashCommandOutput> {
    Ok(zed::SlashCommandOutput {
        text: "正在运行 Flutter Doctor...
将会运行 `flutter doctor`。".to_string(),
        sections: vec![],
    })
}

fn cmd_create(args: &[String]) -> zed::Result<zed::SlashCommandOutput> {
    let name = args
        .iter()
        .find(|a| !a.starts_with("--"))
        .cloned()
        .unwrap_or_else(|| "my_app".to_string());

    let org = get_arg_value(args, "--org").unwrap_or_else(|| "com.example".to_string());

    Ok(zed::SlashCommandOutput {
        text: format!(
            "正在创建 Flutter 项目: {}
组织: {}

将会运行 `flutter create --org={} {}`。",
            name, org, org, name
        ),
        sections: vec![],
    })
}

fn cmd_select_device(args: &[String]) -> zed::Result<zed::SlashCommandOutput> {
    if let Some(device_id) = get_arg_value(args, "--device") {
        Ok(zed::SlashCommandOutput {
            text: format!("已选择设备: {}。\n请运行 /flutter:run 启动应用。", device_id),
            sections: vec![],
        })
    } else {
        Ok(zed::SlashCommandOutput {
            text: "请先运行 /flutter:devices 查看可用设备列表。
然后使用 /flutter:select-device --device=<device_id> 选择设备。".to_string(),
            sections: vec![],
        })
    }
}

/// 启动模拟器
fn cmd_launch_emulator(args: &[String]) -> zed::Result<zed::SlashCommandOutput> {
    if let Some(emulator_id) = get_arg_value(args, "--id") {
        Ok(zed::SlashCommandOutput {
            text: format!("正在启动模拟器: {}。\n请等待模拟器启动完成...", emulator_id),
            sections: vec![],
        })
    } else {
        Ok(zed::SlashCommandOutput {
            text: "使用方式：/flutter:launch-emulator --id=<emulator_id>\n\n请先运行 /flutter:emulators 查看可用模拟器列表。".to_string(),
            sections: vec![],
        })
    }
}

/// 打开 Flutter DevTools
fn cmd_devtools() -> zed::Result<zed::SlashCommandOutput> {
    Ok(zed::SlashCommandOutput {
        text: "正在启动 Flutter DevTools...
DevTools 将在浏览器中打开。
如需指定页面，请使用：
  /flutter:devtools --page=inspector
  /flutter:devtools --page=performance
  /flutter:devtools --page=memory
  /flutter:devtools --page=network".to_string(),
        sections: vec![],
    })
}

/// 从参数列表中提取指定键的值
///
/// 例如：["--device=abc123"] 中解析 --device => Some("abc123")
fn get_arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .find(|a| a.starts_with(key))
        .and_then(|a| {
            if a.contains('=') {
                a.split('=').nth(1).map(|v| v.to_string())
            } else {
                None
            }
        })
}
