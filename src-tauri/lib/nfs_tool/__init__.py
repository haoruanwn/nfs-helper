# src-tauri/scripts/lib/nfs_tool.py (Final, Robust Version)

import subprocess
import paramiko
import logging
import time
import os

class NfsTool:
    def __init__(self, pc_ip, pc_password, share_path, board_ip, board_user, board_password, board_mount_path):
        self.pc_ip = str(pc_ip)
        self.pc_password = str(pc_password)
        self.share_path = os.path.expanduser(str(share_path))
        
        self.board_ip = str(board_ip)
        self.board_user = str(board_user)
        self.board_password = str(board_password)
        self.board_mount_path = str(board_mount_path)
        
        self.ssh_client = paramiko.SSHClient()
        self.ssh_client.set_missing_host_key_policy(paramiko.AutoAddPolicy())
        logging.info("--- NFS Tool Initialized (Final Version) ---")

    def _run_local_sudo_command(self, command):
        logging.info(f"Executing on PC: {command}")
        proc = subprocess.Popen(
            ['sudo', '-S'] + command.split(),
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True
        )
        stdout, stderr = proc.communicate(self.pc_password + '\n')
        if proc.returncode != 0:
            raise Exception(f"Local command failed: '{command}'. Error: {stderr.strip()}")
        return stdout.strip()

    def _is_nfs_share_configured(self):
        logging.info(f"Checking if share for '{self.share_path}' is already configured...")
        try:
            exports_content = self._run_local_sudo_command(f"cat /etc/exports")
            for line in exports_content.splitlines():
                if line.strip().startswith('#') or not line.strip(): continue
                if self.share_path in line.split()[0]:
                    logging.info(f"Found existing configuration for '{self.share_path}'. Skipping modification.")
                    return True
            logging.info("No existing configuration found.")
            return False
        except Exception as e:
            logging.warning(f"Could not check /etc/exports, proceeding anyway. Error: {e}")
            return False

    def _configure_local_nfs(self):
        logging.info("Step 1: Configuring local NFS share...")
        if self._is_nfs_share_configured(): return

        exports_line = f'{self.share_path} {self.board_ip}(rw,sync,no_subtree_check,no_root_squash)'
        command = f'echo "{exports_line}" | tee -a /etc/exports'
        proc = subprocess.Popen(
            ['sudo', '-S', 'sh', '-c', command],
            stdin=subprocess.PIPE, text=True
        )
        proc.communicate(self.pc_password + '\n')
        if proc.returncode != 0: raise Exception("Failed to update /etc/exports")
        logging.info("/etc/exports configured successfully.")
        self._run_local_sudo_command('exportfs -ra')
        logging.info("NFS exports applied.")

    def _run_remote_command(self, command, check=True):
        """
        【已升级】在远程设备上执行命令。
        智能判断是否需要sudo。
        """
        logging.info(f"Executing on board: {command}")
        
        # 【新逻辑】如果用户是root，就不需要sudo
        if self.board_user == 'root':
            final_command = command
            stdin, stdout, stderr = self.ssh_client.exec_command(final_command)
        else:
            final_command = f"sudo -S {command}"
            stdin, stdout, stderr = self.ssh_client.exec_command(final_command)
            stdin.write(self.board_password + '\n')
            stdin.flush()
        
        exit_status = stdout.channel.recv_exit_status()
        
        if check and exit_status != 0:
            error_output = stderr.read().decode().strip()
            raise Exception(f"Remote command failed: '{command}'. Exit code: {exit_status}. Error: {error_output}")
        
        return exit_status, stdout.read().decode().strip(), stderr.read().decode().strip()

    def _mount_on_board(self):
        """【已升级】在开发板上挂载，使用更简单的命令序列。"""
        logging.info(f"Step 2: Preparing mount point on board {self.board_user}@{self.board_ip}...")
        try:
            self.ssh_client.connect(
                hostname=self.board_ip, username=self.board_user, password=self.board_password, timeout=15
            )
            logging.info("SSH connection successful.")

            # 1. 检查是否已挂载 (mountpoint 命令可能不存在，使用更通用的 'grep' 检查)
            check_mount_cmd = f"grep -qs '{self.board_mount_path}' /proc/mounts"
            exit_code, _, _ = self._run_remote_command(check_mount_cmd, check=False)
            if exit_code == 0:
                logging.warning(f"'{self.board_mount_path}' is already a mount point. Unmounting first.")
                self._run_remote_command(f"umount -f -l {self.board_mount_path}")

            # 2. 【新逻辑】检查目录是否存在
            check_dir_cmd = f"test -d {self.board_mount_path}"
            exit_code, _, _ = self._run_remote_command(check_dir_cmd, check=False)
            if exit_code == 0:
                logging.warning(f"Directory '{self.board_mount_path}' already exists. Removing it.")
                self._run_remote_command(f"rm -rf {self.board_mount_path}")

            # 3. 创建新目录并挂载
            logging.info(f"Creating new mount point '{self.board_mount_path}'...")
            self._run_remote_command(f"mkdir -p {self.board_mount_path}")

            mount_command = f"mount -t nfs -o nfsvers=3,nolock {self.pc_ip}:{self.share_path} {self.board_mount_path}"
            self._run_remote_command(mount_command)

            logging.info("Mount operation completed successfully on board.")

        finally:
            if self.ssh_client.get_transport() and self.ssh_client.get_transport().is_active():
                self.ssh_client.close()
                logging.info("SSH connection closed.")

    def run(self):
        try:
            self._configure_local_nfs()
            self._mount_on_board()
            logging.info("✅ All operations completed successfully!")
        except Exception as e:
            logging.error(f"A critical error occurred: {e}")
            raise