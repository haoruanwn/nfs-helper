// src-tauri/src/main.rs
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

// 用于检查依赖
#[tauri::command]
fn check_dependencies() -> Result<String, String> {
    // 在Fedora上，我们用 `rpm -q` 来检查包是否安装
    let output = Command::new("rpm")
        .arg("-q")
        .arg("nfs-utils")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                // `rpm -q` 成功会返回包名和版本
                let stdout = String::from_utf8(output.stdout).unwrap_or_default();
                Ok(format!("依赖检查通过: {}", stdout.trim()))
            } else {
                Err("错误: `nfs-utils` 未安装。请运行 `sudo dnf install nfs-utils`。".to_string())
            }
        }
        Err(e) => Err(format!("执行命令失败: {}", e)),
    }
}

// 用于应用NFS共享配置
// 注意：这个函数需要管理员权限来修改 /etc/exports 和重启服务
#[tauri::command]
async fn apply_nfs_share(pc_path: String, board_ip: String, board_path: String) -> Result<String, String> {
    // 这是一个非常重要的安全提示：
    // 直接修改 /etc/exports 并重启服务需要sudo权限。
    // 在Tauri应用中直接提权是一个复杂且有风险的操作。
    
    // **初级阶段建议:**
    // 我们的应用可以生成需要执行的命令，然后让用户自己复制到终端里用sudo执行。
    // 这是最安全、最简单的起步方式。

    let export_line = format!("{} {}(rw,sync,no_subtree_check,no_root_squash)", pc_path, board_ip);
    
    let instructions = format!(
        "请使用sudo权限在终端中执行以下命令来应用配置:\n\n\
        1. 备份当前的exports文件:\n\
        sudo cp /etc/exports /etc/exports.bak\n\n\
        2. 将配置写入exports文件 (这会覆盖原有文件，请谨慎操作!):\n\
        echo '{}' | sudo tee /etc/exports\n\n\
        3. 重新加载NFS配置:\n\
        sudo exportfs -ra\n\n\
        4. 重启NFS服务:\n\
        sudo systemctl restart nfs-server\n\n\
        完成后，请在开发板上尝试挂载:\n\
        mkdir -p {}\n\
        sudo mount -t nfs {}:{} {}",
        export_line, board_path, board_ip, pc_path, board_path
    );

    Ok(instructions)
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