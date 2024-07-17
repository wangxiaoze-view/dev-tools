// 拼图平台的判断
pub fn is_platform(platform: &str) -> bool {
    match platform {
        "windows" => cfg!(target_os = "windows"),
        "macos" => cfg!(target_os = "macos"),
        "linux" => cfg!(target_os = "linux"),
        _ => false,
    }
}
