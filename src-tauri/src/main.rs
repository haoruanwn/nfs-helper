// src-tauri/src/main.rs
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command; 


// 用于检查依赖，通过检查NFS相关命令的版本
#[tauri::command]
fn check_dependencies() -> Result<String, String> {
    // 定义一个包含元组的列表，元组内容为：(命令, 版本参数)
    // 我们会按顺序尝试这些命令，直到有一个成功为止
    let commands_to_try = vec![
        ("rpc.nfsd", "--version"),
        ("mount.nfs", "-V"),
        ("showmount", "--version"),
    ];

    for (cmd, arg) in commands_to_try {
        match Command::new(cmd).arg(arg).output() {
            Ok(output) => {
                if output.status.success() {
                    // 命令成功执行，我们提取标准输出的第一行作为版本信息
                    let stdout = String::from_utf8(output.stdout).unwrap_or_default();
                    let version_info = stdout.lines().next().unwrap_or("").trim();
                    return Ok(format!("依赖检查通过: {} ({})", version_info, cmd));
                }
                // 如果命令执行了但状态码不为0（例如，参数不被支持），我们就继续尝试下一个
            }
            Err(_) => {
                // 如果命令本身就找不到 (e.g., "command not found")，我们就继续尝试下一个
                continue;
            }
        }
    }

    // 如果循环结束了，说明所有尝试都失败了
    Err("错误: 未找到可用的NFS工具 (如 rpc.nfsd, mount.nfs)。\n请根据你的Linux发行版安装相应软件包，例如 'nfs-utils' (Fedora/Arch) 或 'nfs-common' (Debian/Ubuntu)。".to_string())
}




// 用于应用NFS共享配置
// Command现在需要是 async 的，并且需要一个窗口句柄来发送事件
// apply_nfs_share 函数，更新了签名以接收所有参数
#[tauri::command]
async fn apply_nfs_share(
    window: tauri::Window,
    pc_path: String,
    pc_password: String,
    board_ip: String,
    board_user: String, 
    board_password: String, 
    board_path: String
) -> Result<String, String> {

    // 新的功能待实现
    Ok("功能正在重构中...".to_string())
}


fn main() {
    tauri::Builder::default()
        // 注册Tauri插件
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        // 注册定义的Commands
        .invoke_handler(tauri::generate_handler![
            check_dependencies,
            apply_nfs_share
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}