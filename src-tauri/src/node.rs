use encoding_rs::GBK;
use log::{error, info};
use serde_json::{json, Value};
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::str;

fn get_cmd() -> std::process::Command {
    let cmd = Command::new(if cfg!(target_os = "windows") {
        "powershell"
    } else {
        "bash"
    });
    return cmd;
}

// 安装fnm
#[tauri::command]
pub fn install_fnm() -> serde_json::Value {
    let mut cmd = get_cmd();

    #[cfg(not(target_os = "windows"))]
    {
        cmd.arg("-c")
            .arg("curl -fsSL https://fnm.vercel.app/install | bash");
    }

    #[cfg(target_os = "windows")]
    {
        cmd.arg("-c").arg("winget install Schniz.fnm");
        cmd.creation_flags(0x08000000);
    }

    let output = cmd.output().expect("Failed to execute command");

    let stdout = GBK.decode(&output.stdout);
    let stderr = GBK.decode(&output.stderr);

    info!("fnm安装的结果为：{} ", stdout.0);
    error!("fnm安装的错误为：{} ", stderr.0);

    return json!({
        "status": output.status.success(),
        "stdout": stdout.0,
        "stderr": stderr.0,
    });
}

// 获取当前node版本
#[tauri::command]
pub fn get_node_version() -> String {
    let mut cmd = get_cmd();
    cmd.arg("-c").arg("node -v");
    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(0x08000000);
    }
    let output = cmd.output().expect("Failed to execute command");
    let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
    info!("当前的Node版本为：{} ", version);
    return version;
}

// 安装node
#[tauri::command]
pub fn install_node(version: &str) -> serde_json::Value {
    let mut cmd = get_cmd();
    cmd.arg("-c").arg(format!("fnm install {}", version));
    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(0x08000000);
    }
    let output = cmd.output().expect("Failed to execute command");
    let stdout = GBK.decode(&output.stdout);
    let stderr = GBK.decode(&output.stderr);
    info!("node安装的结果为：{} ", stdout.0);
    error!("node安装的错误为：{} ", stderr.0);
    return json!({
        "status": output.status.success(),
        "stdout": stdout.0,
        "stderr": stderr.0,
    });
}

// 查询已经安装的node
#[tauri::command]
pub fn get_list_node() -> serde_json::Value {
    let mut cmd = get_cmd();
    cmd.arg("-c").arg("fnm ls");
    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(0x08000000);
    }
    let output = cmd.output().expect("Failed to execute command");
    let stdout = GBK.decode(&output.stdout);
    if output.status.success() {
        let versions: Vec<&str> = stdout.0.split('\n').collect();
        return json!({
            "status": output.status.success(),
            "versions": versions,
        });
    } else {
        return json!({
            "status": output.status.success(),
            "versions": [],
        });
    }
}

// 切换node
#[tauri::command]
pub fn change_node(version: &str) -> serde_json::Value {
    let mut cmd = get_cmd();
    cmd.arg("-c").arg(format!("fnm default {}", version));
    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(0x08000000);
    }
    let output = cmd.output().expect("Failed to execute command");
    let stdout = GBK.decode(&output.stdout);
    let stderr = GBK.decode(&output.stderr);
    info!("更换node的结果为：{} ", stdout.0);
    error!("更换node的错误为：{} ", stderr.0);
    return json!({
        "status": output.status.success(),
        "stdout": stdout.0,
        "stderr": stderr.0,
    });
}

// 删除node
#[tauri::command]
pub fn delete_node(version: &str) -> serde_json::Value {
    let mut cmd = get_cmd();
    cmd.arg("-c").arg(format!("fnm uninstall {}", version));
    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(0x08000000);
    }

    let output = cmd.output().expect("Failed to execute command");
    let stdout = GBK.decode(&output.stdout);
    let stderr = GBK.decode(&output.stderr);
    info!("删除node的结果为：{} ", stdout.0);
    error!("删除node的错误为：{} ", stderr.0);
    return json!({
        "status": output.status.success(),
        "stdout": stdout.0,
        "stderr": stderr.0,
    });
}

// 获取当前的npm源
#[tauri::command]
pub fn get_npm_registry() -> String {
    let mut cmd = get_cmd();
    cmd.arg("-c").arg("npm config get registry --g");
    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(0x08000000);
    }
    let output = cmd.output().expect("Failed to execute command");
    let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
    info!("当前的npm源为：{} ", result);
    return result;
}

// 设置npm源
#[tauri::command]
pub fn set_npm_registry(registry: &str) -> serde_json::Value {
    let mut cmd = get_cmd();
    cmd.arg("-c")
        .arg(format!("npm config set registry {} -g", registry));
    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(0x08000000);
    }
    let output = cmd.output().expect("Failed to execute command");
    let stdout = GBK.decode(&output.stdout);
    let stderr = GBK.decode(&output.stderr);
    info!("设置npm源的结果为：{} ", stdout.0);
    error!("设置npm源的错误为：{} ", stderr.0);

    return json!({
        "status": output.status.success(),
        "stdout": stdout.0,
        "stderr": stderr.0,
    });
}

// 获取npm包
#[tauri::command]
pub fn get_npm_package(package: &str) -> serde_json::Value {
    let mut cmd = get_cmd();
    cmd.arg("-c")
        .arg(format!("npm view {} version --json", package));
    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(0x08000000);
    }
    let output = cmd.output().expect("Failed to execute command");
    let stdout = GBK.decode(&output.stdout);
    let stderr = GBK.decode(&output.stderr);
    info!("获取npm包的结果为：{} ", stdout.0);
    error!("获取npm包的错误为：{} ", stderr.0);
    return json!({
        "status": output.status.success(),
        "stdout": stdout.0,
        "stderr": stderr.0,
    });
}

// 查询已经安装的npm包
#[tauri::command]
pub fn get_list_npm_package() -> serde_json::Value {
    let mut cmd = get_cmd();
    cmd.arg("-c").arg("npm ls -g --depth=0 --json");
    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(0x08000000);
    }
    let output = cmd.output().expect("Failed to execute command");
    let stdout = GBK.decode(&output.stdout);
    let stderr = GBK.decode(&output.stderr);
    info!("查询已经安装的npm包的结果为：{} ", stdout.0);
    error!("查询已经安装的npm包的错误为：{} ", stderr.0);
    // stdout.0 转为json
    let json: Value = serde_json::from_str(&stdout.0).unwrap();

    return json!({
        "status": output.status.success(),
        "stdout": json,
        "stderr": stderr.0,
    });
}

// 安装全局npm包
#[tauri::command]
pub fn install_npm_package(package: &str) -> serde_json::Value {
    let mut cmd = get_cmd();
    cmd.arg("-c").arg(format!("npm install {} -g", package));
    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(0x08000000);
    }
    let output = cmd.output().expect("Failed to execute command");
    let stdout = GBK.decode(&output.stdout);
    let stderr = GBK.decode(&output.stderr);
    info!("安装全局npm包的结果为：{} ", stdout.0);
    error!("安装全局npm包的错误为：{} ", stderr.0);
    return json!({
        "status": output.status.success(),
        "stdout": stdout.0,
        "stderr": stderr.0,
    });
}

// 删除全局包
#[tauri::command]
pub fn delete_npm_package(package: &str) -> serde_json::Value {
    let mut cmd = get_cmd();
    cmd.arg("-c").arg(format!("npm uninstall {} -g", package));
    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(0x08000000);
    }
    let output = cmd.output().expect("Failed to execute command");
    let stdout = GBK.decode(&output.stdout);
    let stderr = GBK.decode(&output.stderr);
    info!("删除全局npm包的结果为：{} ", stdout.0);
    error!("删除全局npm包的错误为：{} ", stderr.0);
    return json!({
        "status": output.status.success(),
        "stdout": stdout.0,
        "stderr": stderr.0,
    });
}

// 更新全局包
#[tauri::command]
pub fn update_npm_package(package: &str) -> serde_json::Value {
    let mut cmd = get_cmd();
    cmd.arg("-c").arg(format!("npm update {} -g", package));
    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(0x08000000);
    }
    let output = cmd.output().expect("Failed to execute command");
    let stdout = GBK.decode(&output.stdout);
    let stderr = GBK.decode(&output.stderr);
    info!("更新全局npm包的结果为：{} ", stdout.0);
    error!("更新全局npm包的错误为：{} ", stderr.0);
    return json!({
        "status": output.status.success(),
        "stdout": stdout.0,
        "stderr": stderr.0,
    });
}
