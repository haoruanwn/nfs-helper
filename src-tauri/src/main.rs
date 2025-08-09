// src-tauri/src/main.rs
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::{Command, Stdio}; 
use std::io::{BufRead, BufReader};
use tauri::{Manager, Emitter};


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
    app_handle: tauri::AppHandle,
    pc_path: String,
    pc_password: String,
    pc_ip: String,
    board_ip: String,
    board_user: String, 
    board_password: String, 
    board_path: String
) -> Result<(), String> {
    // 1. 定位Python脚本路径
    // tauri::api::path::resolve_path 会处理开发和发布环境的路径差异
    let cli_path = app_handle.path().resolve("resources/nfs_helper_cli", tauri::path::BaseDirectory::Resource)
    .map_err(|e| format!("无法解析可执行文件路径: {}", e))?;

    // 2. 准备参数
    let args = vec![
        "share".to_string(),
        pc_path,
        pc_password,
        pc_ip, 
        board_ip,
        board_user,
        board_password,
        board_path,
    ];

    // 3. 创建子进程
    let mut child = Command::new(cli_path) 
        .args(args)
        .stdout(Stdio::piped()) // 捕获标准输出
        .stderr(Stdio::piped()) // 捕获标准错误
        .spawn()
        .map_err(|e| format!("无法启动Python子进程: {}", e))?;

    // 4. 异步读取输出并发送到前端
    let stdout = child.stdout.take().expect("无法获取子进程的stdout");
    let stderr = child.stderr.take().expect("无法获取子进程的stderr");

    let window_stdout = window.clone();
    tauri::async_runtime::spawn(async move {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line_str) = line {
                window_stdout.emit("sidecar-output", Some(format!("[INFO] {}", line_str))).unwrap();
            }
        }
    });

    let window_stderr = window.clone();
    tauri::async_runtime::spawn(async move {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(line_str) = line {
                window_stderr.emit("sidecar-output", Some(format!("[ERROR] {}", line_str))).unwrap();
            }
        }
    });

    // 5. 监听进程结束
    tauri::async_runtime::spawn(async move {
        let status = child.wait().expect("子进程执行出错");
        let code = status.code();
        if status.success() {
            window.emit("sidecar-output", Some("--- 操作成功完成 ---".to_string())).unwrap();
        } else {
            window.emit("sidecar-output", Some(format!("--- 操作失败，退出码: {:?} ---", code))).unwrap();
        }
    });


    Ok(())
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