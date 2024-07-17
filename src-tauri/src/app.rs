use encoding_rs::GBK;
use log::{error, info};
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::str;

/**
 *
 * @description 检测软件是否安装
 */
#[tauri::command]
pub fn is_installed(program: &str) -> bool {
    let mut cmd = Command::new(if cfg!(target_os = "windows") {
        "cmd"
    } else {
        "bash"
    });
    let arg = if cfg!(target_os = "windows") {
        format!("where {}", program)
    } else {
        format!("which {}", program)
    };
    cmd.arg("-c").arg(arg);

    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(0x08000000);
    }
    let output = cmd.output().expect("Failed to execute command");
    let stdout = GBK.decode(&output.stdout);

    let result: bool = output.status.success() && !output.stdout.is_empty();

    info!(
        "{} 是否已经安装 {}, 安装结果为: {}",
        program, result, stdout.0
    );
    result
}

// 打开某个链接
#[tauri::command]
pub fn open_link(url: &str) {
    info!("当前的URL为：{}", url);
    if let Err(e) = open::that(url) {
        error!("链接: {} 打开失败, 失败原因：{}", url, e)
    }
}
