# lib/common/utils.py
import sys
import socket
import subprocess

def print_color(text, color_code):
    print(f"\033[{color_code}m{text}\033[0m")

def print_info(message): print_color(f"[INFO] {message}", "34")
def print_success(message): print_color(f"[SUCCESS] {message}", "32")
def print_warning(message): print_color(f"[WARNING] {message}", "33")
def print_error(message, exit_code=1):
    print_color(f"[ERROR] {message}", "31")
    if exit_code is not None:
        sys.exit(exit_code)

def run_local_command(command, password=None, check=True):
    print_info(f"本地执行: {' '.join(command)}")
    try:
        if command[0] == 'sudo' and password:
            # 如果是sudo命令且提供了密码，使用 echo 和管道
            sudo_command = f"echo '{password}' | sudo -S -p '' {' '.join(command[1:])}"
            # 使用 shell=True 来执行管道命令
            result = subprocess.run(sudo_command, shell=True, check=check, capture_output=True, text=True)
        else:
            result = subprocess.run(command, check=check, capture_output=True, text=True)

        # 检查命令是否成功执行
        if result.returncode != 0:
             raise RuntimeError(f"本地命令执行失败: {result.stderr.strip()}")

    except subprocess.CalledProcessError as e:
        raise RuntimeError(f"本地命令执行失败: {e}\n{e.stderr.strip()}")


def get_host_ip_for_target(target_ip):
    try:
        s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        s.connect((target_ip, 80))
        host_ip = s.getsockname()[0]
        s.close()
        return host_ip
    except socket.error as e:
        raise RuntimeError(f"无法确定本机的IP地址: {e}")