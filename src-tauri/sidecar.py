# src-tauri/sidecar.py
import sys
import os
import configparser
import traceback

# 确保脚本可以找到lib目录
project_root = os.path.dirname(os.path.realpath(__file__))
sys.path.insert(0, os.path.join(project_root))

# 捕获可能的导入错误
try:
    from lib.nfs_tool.actions import handle_share, handle_unshare
    from lib.common.utils import print_error
except ImportError as e:
    print(f"PYTHON_ERROR: 导入模块失败: {e}", file=sys.stderr)
    sys.exit(1)


def main():
    # 1. 解析命令行参数 
    if len(sys.argv) < 7:
        print_error(f"参数不足。用法: {sys.argv[0]} <share|unshare> <pc_path> <pc_password> <board_ip> <board_user> <board_password> <board_path>")
        sys.exit(1)

    action = sys.argv[1]
    pc_path = sys.argv[2]
    pc_password = sys.argv[3] # 本机密码，用于执行sudo命令
    board_ip = sys.argv[4]
    board_user = sys.argv[5]
    board_password = sys.argv[6]
    board_path = sys.argv[7]

    # 2. 动态创建配置对象
    # 注意：actions.py 内部使用了 sudo，它可能需要本机密码
    # 我们需要稍微调整 `run_local_command` 来处理密码
    # (为了演示，我们先假设它可以工作，后面再优化)

    config.add_section('PC_INFO')
    config.set('PC_INFO', 'password', pc_password)
    
    config = configparser.ConfigParser()
    config.add_section('NFS_HOST')
    config.set('NFS_HOST', 'share_path', pc_path)

    config.add_section('BOARD_INFO')
    config.set('BOARD_INFO', 'ip', board_ip)
    config.set('BOARD_INFO', 'user', board_user)
    config.set('BOARD_INFO', 'password', board_password)

    config.add_section('MOUNT_POINT')
    config.set('MOUNT_POINT', 'path', board_path)

    # 3. 执行核心逻辑
    try:
        # 这里的逻辑需要能够处理 sudo 密码，我们需要修改 run_local_command
        print(f"INFO: 正在执行操作 '{action}'...")
        if action == 'share':
            handle_share(config)
        elif action == 'unshare':
            handle_unshare(config, is_rollback=False)
        else:
            print_error(f"未知操作: {action}")

        print("INFO: Python脚本执行完毕。")

    except Exception as e:
        # 打印详细的错误信息和追溯，方便调试
        error_message = f"PYTHON_ERROR: {e}\n{traceback.format_exc()}"
        print(error_message, file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()