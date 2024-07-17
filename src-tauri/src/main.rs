// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::env::set_var;

fn main() {
    let _ = fix_path_env::fix();
    set_var("RUST_LOG", "debug");
    env_logger::init();
    dev_tools_lib::run()
}
