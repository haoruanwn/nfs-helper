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
    pc_path: String,
    pc_password: String,
    pc_ip: String,
    board_ip: String,
    board_user: String, 
    board_password: String, 
    board_path: String
) -> Result<String, String> {
    let mut logs: Vec<String> = Vec::new();

    // 打印以上的信息，验证是否正常获取信息
    logs.push(format!("[DEBUG] 本机IP: {}", pc_ip));
    logs.push(format!("[DEBUG] 本机NFS路径: {}", pc_path));
    logs.push(format!("[DEBUG] 本机NFS密码: {}", pc_password));
    logs.push(format!("[DEBUG] 开发板IP: {}", board_ip));
    logs.push(format!("[DEBUG] 开发板用户名: {}", board_user));
    logs.push(format!("[DEBUG] 开发板密码: {}", board_password));
    logs.push(format!("[DEBUG] 开发板挂载路径: {}", board_path));


    logs.push("[INFO] 开始原生NFS共享流程...".to_string());

    // // --- 1. 本地NFS配置 ---
    // logs.push("[INFO] 正在配置本地 /etc/exports...".to_string());
    // let export_line = format!("\"{}\" {}(rw,sync,no_subtree_check,no_root_squash)", pc_path, board_ip);
    // let command_to_run = format!("echo '{}' | sudo -S -p '' tee /etc/exports > /dev/null && sudo -S -p '' exportfs -ra", export_line);
    
    // let mut child = Command::new("sh")
    //     .arg("-c")
    //     .arg(&command_to_run)
    //     .stdin(std::process::Stdio::piped())
    //     .stdout(std::process::Stdio::piped())
    //     .stderr(std::process::Stdio::piped())
    //     .spawn()
    //     .map_err(|e| format!("[ERROR] 启动本地命令失败: {}", e))?;

    // // 将密码写入子进程的stdin
    // use std::io::Write;
    // // 需要两次密码，一次给tee，一次给exportfs
    // let password_input = format!("{}\n{}\n", pc_password, pc_password);
    // child.stdin.as_mut().unwrap().write_all(password_input.as_bytes()).map_err(|e| format!("[ERROR] 写入密码失败: {}", e))?;

    // let output = child.wait_with_output().map_err(|e| format!("[ERROR] 等待本地命令完成失败: {}", e))?;
    
    // if !output.status.success() {
    //     let stderr = String::from_utf8_lossy(&output.stderr);
    //     return Err(format!("[ERROR] 配置本地NFS失败:\n{}", stderr));
    // }
    // logs.push("[SUCCESS] 本地NFS导出配置成功。".to_string());


    // // --- 2. 远程开发板配置 ---
    // // 为了非交互式执行ssh，通常需要sshpass或者配置ssh密钥。
    // // 这里我们使用 sshpass (需要在主机上安装: sudo apt-get install sshpass)
    // logs.push("[INFO] 正在通过SSH连接并挂载开发板...".to_string());
    
    // // 远程执行的命令
    // let remote_mkdir_cmd = format!("mkdir -p {}", board_path);
    // let nfs_source = format!("{}:{}", "获取本机IP的逻辑需要补充", pc_path); // 这里的IP获取需要一个辅助函数
    // let remote_mount_cmd = format!("mount -t nfs -o vers=3,nolock '{}' '{}'", nfs_source, board_path);
    
    // // 依次执行远程命令
    // for (desc, cmd) in [("创建挂载点", remote_mkdir_cmd), ("执行挂载", remote_mount_cmd)] {
    //     logs.push(format!("[INFO] 远程操作: {}...", desc));
    //     let ssh_output = Command::new("sshpass")
    //         .arg("-p")
    //         .arg(&board_password)
    //         .arg("ssh")
    //         .arg("-o")
    //         .arg("StrictHostKeyChecking=no") // 避免首次连接的确认
    //         .arg(format!("{}@{}", board_user, board_ip))
    //         .arg(&cmd)
    //         .output()
    //         .map_err(|e| format!("[ERROR] 执行sshpass失败，请确认已安装: {}", e))?;
        
    //     if !ssh_output.status.success() {
    //         let stderr = String::from_utf8_lossy(&ssh_output.stderr);
    //         return Err(format!("[ERROR] 远程命令 '{}' 失败:\n{}", cmd, stderr));
    //     }
    //     logs.push(format!("[SUCCESS] {} 操作成功。", desc));
    // }
    
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