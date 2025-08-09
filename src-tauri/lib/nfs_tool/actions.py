# lib/nfs/actions.py
import os
import subprocess
from ..common.utils import (print_info, print_success, print_warning, 
                            print_error, run_local_command, get_host_ip_for_target)
from ..common.ssh_manager import SSHManager

def check_host_nfs_server():
    print_info("检查主机的NFS服务端环境...")
    if not os.path.exists('/usr/sbin/exportfs'):
        raise RuntimeError("NFS服务端工具(exportfs)未找到。")
    try:
        status = subprocess.run(['systemctl', 'is-active', '--quiet', 'nfs-kernel-server']).returncode
        if status != 0:
            print_warning("主机的NFS服务(nfs-kernel-server)未运行，正在尝试启动...")
            run_local_command(['systemctl', 'restart', 'nfs-kernel-server'])
    except Exception as e:
        raise RuntimeError(f"检查或启动主机NFS服务时出错: {e}")
    print_success("主机的NFS服务端环境已就绪。")

def check_board_dependencies(ssh):
    print_info("正在检查开发板上的NFS客户端依赖...")
    mount_helper_path = "/usr/sbin/mount.nfs"
    check_command = f"test -x {mount_helper_path}"
    try:
        ssh.execute(check_command)
        print_success(f"开发板NFS客户端依赖满足 ({mount_helper_path} 可执行)。")
    except RuntimeError:
        raise RuntimeError(f"开发板上未找到可执行的 '{mount_helper_path}' 工具。请先在开发板上运行 'sudo apt-get install nfs-common'。")

def manage_host_exports(action, config):
    exports_file = '/etc/exports'
    share_path = os.path.expanduser(config.get('NFS_HOST', 'share_path'))
    board_ip = config.get('BOARD_INFO', 'ip')
    tag_comment = f"# Managed by toolkit for board {board_ip}"
    export_line = f'"{share_path}" {board_ip}(rw,sync,no_subtree_check,no_root_squash)'
    full_line_to_add = f"{export_line} {tag_comment}\n"
    print_info(f"正在管理主机的 {exports_file} 文件...")
    try:
        with open(exports_file, 'r') as f: lines = f.readlines()
    except FileNotFoundError: lines = []

    if action == 'add':
        lines = [line for line in lines if tag_comment not in line]
        lines.append(full_line_to_add)
    elif action == 'remove':
        lines = [line for line in lines if tag_comment not in line]
    
    with open(exports_file, 'w') as f: f.writelines(lines)
    print_info(f"{exports_file} 文件已更新。")

def handle_share(config):
    ssh = None
    try:
        # 检查主机的NFS服务端环境，交给Tauri的按钮处理，这里注释掉
        # check_host_nfs_server()
        host_share_path = os.path.expanduser(config.get('NFS_HOST', 'share_path'))
        if not os.path.exists(host_share_path):
            os.makedirs(host_share_path, exist_ok=True)
        manage_host_exports('add', config)

        pc_password = config.get('PC_INFO', 'password') 
        run_local_command(['sudo', 'exportfs', '-ra'], password=pc_password)

        ssh = SSHManager(config.get('BOARD_INFO', 'ip'), config.get('BOARD_INFO', 'user'), config.get('BOARD_INFO', 'password'))
        ssh.connect()
        check_board_dependencies(ssh)
        
        board_mount_point = config.get('MOUNT_POINT', 'path')
        mkdir_cmd = "mkdir" if ssh.is_root else "/bin/mkdir"
        mount_cmd_base = "mount" if ssh.is_root else "/bin/mount"
        
        ssh.execute(f"{mkdir_cmd} -p {board_mount_point}")

        host_ip = get_host_ip_for_target(config.get('BOARD_INFO', 'ip'))
        nfs_source = f"{host_ip}:{host_share_path}"
        mount_cmd = f"{mount_cmd_base} -t nfs -o vers=3,nolock '{nfs_source}' '{board_mount_point}'"
        ssh.execute(mount_cmd)

        print_success(f"操作成功！已将主机目录 '{host_share_path}' 共享到开发板 '{board_mount_point}'")

    except Exception as e:
        print_error(f"共享挂载过程中发生致命错误: {e}", exit_code=None)
        print_warning("错误发生，正在触发自动清理回滚...")
        try:
            handle_unshare(config, is_rollback=True)
        except Exception as cleanup_e:
            print_error(f"清理回滚操作本身也失败了: {cleanup_e}", exit_code=None)
        raise RuntimeError("挂载操作失败并已尝试回滚。")
    finally:
        if ssh and ssh.client: ssh.disconnect()

def handle_unshare(config, is_rollback=False):
    if not is_rollback: print_info("--- 开始卸载清理流程 ---")
    else: print_info("--- 开始回滚清理流程 ---")

    ssh = SSHManager(config.get('BOARD_INFO', 'ip'), config.get('BOARD_INFO', 'user'), config.get('BOARD_INFO', 'password'))
    try:
        ssh.connect()
        umount_cmd_base = "umount" if ssh.is_root else "/bin/umount"
        board_mount_point = config.get('MOUNT_POINT', 'path')
        ssh.execute(f"{umount_cmd_base} -f -l '{board_mount_point}' || true")
    except Exception as e:
        print_warning(f"连接或在开发板上卸载时发生错误 (可能本就未挂载): {e}")
    finally:
        if ssh and ssh.client: ssh.disconnect()

    manage_host_exports('remove', config)
    try:
        pc_password = config.get('PC_INFO', 'password')
        run_local_command(['sudo', 'exportfs', '-ra'], password=pc_password)
    except Exception as e:
        print_warning(f"重新加载主机NFS导出配置时出错: {e}")
    
    if not is_rollback: print_success("所有清理操作已完成。")
    else: print_success("回滚清理已完成。")