// src-tauri/src/main.rs
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command; 


// 用于检查依赖，通过检查NFS相关命令的版本
#[tauri::command]
fn check_dependencies() -> Result<String, String> {
    // --- 修改点1：增加了 sshpass 的检查 ---
    let commands_to_try = vec![
        ("rpc.nfsd", "--version"),
        ("mount.nfs", "-V"),
        ("showmount", "--version"),
        ("sshpass", "-V"), // 新增对 sshpass 的检查
    ];

    let mut successful_checks = Vec::new();

    for (cmd, arg) in &commands_to_try {
        match Command::new(cmd).arg(arg).output() {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8(output.stdout).unwrap_or_default();
                    let version_info = stdout.lines().next().unwrap_or("").trim();
                    successful_checks.push(format!("✅ {} ( {} )", cmd, version_info));
                }
            }
            Err(_) => {
                // 命令不存在，可以记录一个失败信息
                successful_checks.push(format!("❌ {} (未找到)", cmd));
                continue;
            }
        }
    }
    
    // 返回所有检查的结果
    if successful_checks.len() == commands_to_try.len() && !successful_checks.iter().any(|s| s.contains('❌')) {
       Ok(format!("所有依赖检查通过:\n{}", successful_checks.join("\n")))
    } else {
       Err(format!("依赖检查不完整:\n{}\n\n请确保NFS工具和sshpass均已正确安装。", successful_checks.join("\n")))
    }
}




// 用于应用NFS共享配置
// Command现在需要是 async 的，并且需要一个窗口句柄来发送事件
// apply_nfs_share 函数，更新了签名以接收所有参数
#[tauri::command]
async fn apply_nfs_share(
    pc_path: String,
    pc_password: String,
    pc_ip: String,
    board_ip: String,
    board_user: String, 
    board_password: String, 
    board_path: String
) -> Result<String, String> {
   let mut logs: Vec<String> = Vec::new();

    // 打印收到的信息以供调试
    logs.push("[DEBUG] 收到配置信息:".to_string());
    logs.push(format!("  - 本机IP: {}", pc_ip));
    logs.push(format!("  - 本机共享路径: {}", pc_path));
    logs.push(format!("  - 开发板IP: {}", board_ip));
    logs.push(format!("  - 开发板用户名: {}", board_user));
    logs.push(format!("  - 开发板挂载路径: {}", board_path));
    logs.push("-".repeat(40));

    logs.push("[INFO] 开始原生NFS共享流程...".to_string());

    // --- 1. 本地NFS配置 ---
    logs.push("[INFO] 正在配置本地 /etc/exports...".to_string());
    let export_line = format!("\"{}\" {}(rw,sync,no_subtree_check,no_root_squash)", pc_path, board_ip);
    let command_to_run = format!("echo '{}' | sudo -S -p '' tee /etc/exports > /dev/null && sudo -S -p '' exportfs -ra", export_line);

    let mut child = Command::new("sh")
        .arg("-c")
        .arg(&command_to_run)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("[ERROR] 启动本地命令失败: {}", e))?;

    use std::io::Write;
    let password_input = format!("{}\n{}\n", pc_password, pc_password);
    if let Some(mut stdin) = child.stdin.as_mut() {
        stdin.write_all(password_input.as_bytes()).map_err(|e| format!("[ERROR] 写入密码失败: {}", e))?;
    }

    let output = child.wait_with_output().map_err(|e| format!("[ERROR] 等待本地命令完成失败: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("[ERROR] 配置本地NFS失败:\n{}", stderr));
    }
    logs.push("[SUCCESS] 本地NFS导出配置成功。".to_string());

    // --- 2. 远程开发板配置 ---
    logs.push("[INFO] 正在通过SSH连接并挂载开发板...".to_string());

    // --- 修改点2：应用传入的 pc_ip ---
    let nfs_source = format!("{}:{}", pc_ip, pc_path);
    logs.push(format!("[INFO] 构建的NFS源: {}", nfs_source));
    
    let remote_mkdir_cmd = format!("mkdir -p {}", board_path);
    let remote_mount_cmd = format!("mount -t nfs -o vers=3,nolock '{}' '{}'", nfs_source, board_path);

    for (desc, cmd) in [("创建挂载点", remote_mkdir_cmd), ("执行挂载", remote_mount_cmd)] {
        logs.push(format!("[INFO] 远程操作: {}...", desc));
        let ssh_output = Command::new("sshpass")
            .arg("-p")
            .arg(&board_password)
            .arg("ssh")
            .arg("-o")
            .arg("StrictHostKeyChecking=no")
            .arg("-o")
            .arg("UserKnownHostsFile=/dev/null") // 避免主机密钥问题
            .arg(format!("{}@{}", board_user, board_ip))
            .arg(&cmd)
            .output()
            .map_err(|e| format!("[ERROR] 执行sshpass失败，请确认已安装: {}", e))?;

        if !ssh_output.status.success() {
            let stderr = String::from_utf8_lossy(&ssh_output.stderr);
            return Err(format!("[ERROR] 远程命令 '{}' 失败:\n{}", cmd, stderr));
        }
        logs.push(format!("[SUCCESS] {} 操作成功。", desc));
    }

    logs.push("--- 所有操作成功完成！ ---".to_string());
    Ok(logs.join("\n"))
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