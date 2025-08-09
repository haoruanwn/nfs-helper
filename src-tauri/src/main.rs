// src-tauri/src/main.rs
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command; // 引入Command用于执行系统命令


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
#[tauri::command]
async fn apply_nfs_share(pc_path: String, pc_password: String, board_ip: String, board_path: String, board_user: String, board_password: String) -> Result<String, String> {
    // 这是一个非常重要的安全提示：
    // 直接修改 /etc/exports 并重启服务需要sudo权限。
    // 在Tauri应用中直接提权是一个复杂且有风险的操作。
    
    // **初级阶段建议:**
    // 我们的应用可以生成需要执行的命令，然后让用户自己复制到终端里用sudo执行。
    // 这是最安全、最简单的起步方式。

    let export_line = format!("{} {}(rw,sync,no_subtree_check,no_root_squash)", pc_path, board_ip);
    
    let instructions = format!(
        "目前已有信息: \n\
        PC路径: {}\n\
        PC密码: {}\n\
        开发板IP: {}\n\
        开发板路径: {}\n\
        开发板用户: {}\n\
        开发板密码: {}\n\
        请使用sudo权限在终端中执行以下命令来应用配置:\n\n\
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
        pc_path, pc_password, board_ip, board_path, board_user, board_password,
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