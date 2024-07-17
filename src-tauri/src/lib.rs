// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app;
mod git;
mod node;

use app::{is_installed, open_link};
use git::{get_git_config, set_git_config};

use node::{
    change_node, delete_node, delete_npm_package, get_list_node, get_list_npm_package,
    get_node_version, get_npm_package, get_npm_registry, install_fnm, install_node,
    install_npm_package, set_npm_registry, update_npm_package,
};

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            is_installed,
            open_link,
            install_fnm,
            get_node_version,
            install_node,
            get_list_node,
            change_node,
            delete_node,
            get_npm_registry,
            set_npm_registry,
            get_npm_package,
            get_list_npm_package,
            install_npm_package,
            delete_npm_package,
            update_npm_package,
            get_git_config,
            set_git_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
