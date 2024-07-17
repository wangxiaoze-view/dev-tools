use log::{error, info};
use serde_json::{json, Value};
use std::io;
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::str;

#[tauri::command]
pub fn get_git_config() -> serde_json::Value {
    let mut cmd = Command::new(if cfg!(target_os = "windows") {
        "powershell"
    } else {
        "bash"
    });

    #[cfg(not(target_os = "windows"))]
    {
        cmd.arg("-c").arg("git config --global --list");
    }

    #[cfg(target_os = "windows")]
    {
        cmd.arg("-Command").arg("git config --global --list");
        cmd.creation_flags(0x08000000);
    }
    let output = cmd.output().expect("Failed to execute command");
    let stdout = str::from_utf8(&output.stdout).unwrap_or_default();
    let stderr = str::from_utf8(&output.stderr).unwrap_or_default();

    info!("git配置的结果为：{} ", stdout);
    error!("git配置的错误为：{} ", stderr);

    // 解析stdout中的键值对
    let results: Vec<Value> = stdout
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() == 2 {
                Some(json!({ "key": parts[0].trim(), "value": parts[1].trim() }))
            } else {
                None
            }
        })
        .collect();

    return json!({
        "status": output.status.success(),
        "stdout": results,
        "stderr": stderr,
    });
}

pub fn get_git_key(key: &str, value: &str) -> std::process::Command {
    let mut cmd = Command::new(if cfg!(target_os = "windows") {
        "powershell"
    } else {
        "bash"
    });
    cmd.args(&["git", "config", "--global", key, value]);
    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(0x08000000);
    }
    return cmd;
}

// 设置git
pub fn set_global_git_config(
    name: &str,
    email: &str,
    autocrlf: &str,
) -> Result<(), std::io::Error> {
    let mut user = get_git_key("user.name", name);
    let mut email = get_git_key("user.email", email);
    let mut autocrlf = get_git_key("user.autocrlf", autocrlf);

    user.status()
        .map(|_| ())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    email
        .status()
        .map(|_| ())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    autocrlf
        .status()
        .map(|_| ())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(())
}

#[tauri::command]
pub fn set_git_config(name: &str, email: &str, autocrlf: &str) -> serde_json::Value {
    match set_global_git_config(name, email, autocrlf) {
        Ok(()) => json!({
            "status": true,
            "stdout": "设置成功",
        }),
        Err(e) => json!({
            "status": false,
            "stderr": e.to_string()
        }),
    }
}
