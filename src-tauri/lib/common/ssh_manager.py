# lib/common/ssh_manager.py
import paramiko
from .utils import print_info, print_success, print_error, print_color

class SSHManager:
    def __init__(self, ip, user, password):
        self.ip, self.user, self.password = ip, user, password
        self.client = None
        self.is_root = (self.user == 'root')

    def connect(self):
        print_info(f"正在通过 SSH 连接到 {self.user}@{self.ip}...")
        try:
            self.client = paramiko.SSHClient()
            self.client.set_missing_host_key_policy(paramiko.AutoAddPolicy())
            self.client.connect(self.ip, username=self.user, password=self.password, timeout=10)
            print_success("SSH 连接成功！")
        except Exception as e:
            raise RuntimeError(f"SSH 连接失败: {e}")

    def disconnect(self):
        if self.client:
            self.client.close()
            self.client = None

    def execute(self, command):
        """在远程设备上执行命令，并实时流式传输输出。"""
        if not self.client:
            raise RuntimeError("SSH 未连接，无法执行命令。")
        
        if self.is_root:
            full_command = command
            print_info(f"远程执行 (as root): {command}")
        else:
            full_command = f"echo '{self.password}' | sudo -S -p '' {command}"
            print_info(f"远程执行 (via sudo): {command}")
            
        # 使用 get_pty=True 来模拟一个终端，这对于很多命令的输出格式更友好
        stdin, stdout, stderr = self.client.exec_command(full_command, get_pty=True)
        
        # --- 实时输出 ---
        print_color("--- [远程输出 START] ---", "1;35") # Bold Magenta
        # 使用 iter(stdout.readline, "") 技巧来逐行读取，直到流关闭
        for line in iter(stdout.readline, ""):
            # 加上一个标记，清晰地表明这是远程输出
            print(f"\033[36m[REMOTE]\033[0m {line.strip()}")
        print_color("--- [远程输出 END] ---", "1;35")
        
        exit_status = stdout.channel.recv_exit_status()
        
        if exit_status != 0:
            # 如果命令失败，读取并抛出标准错误流中的信息
            error_output = stderr.read().decode('utf-8').strip()
            raise RuntimeError(f"远程命令 '{command}' 执行失败，退出码: {exit_status}\n错误详情: {error_output}")
            
        return exit_status