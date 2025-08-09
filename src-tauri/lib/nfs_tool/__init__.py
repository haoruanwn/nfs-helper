import os
from .actions import handle_share, handle_unshare
from ..common.utils import print_error, print_info

class NfsTool:
    """
    一个用于管理 NFS 共享和挂载操作的工具类。

    该类封装了与主机和远程开发板交互所需的所有配置信息，
    并提供了简单的方法来执行共享挂载和卸载清理操作。
    """
    def __init__(self, pc_ip, pc_password, share_path, board_ip, board_user, board_password, board_mount_path):
        """
        初始化 NfsTool 实例。

        参数:
            pc_ip (str): 本地主机（NFS 服务端）的 IP 地址。
            pc_password (str): 本地主机的 sudo 密码。
            share_path (str): 本地主机上要共享的目录路径。
            board_ip (str): 远程开发板的 IP 地址。
            board_user (str): 登录远程开发板的用户名。
            board_password (str): 登录远程开发板的密码。
            board_mount_path (str): 在远程开发板上的挂载点路径。
        """
        self.pc_ip = pc_ip
        self.pc_password = pc_password
        self.share_path = os.path.expanduser(share_path) # 展开波浪号 ~
        self.board_ip = board_ip
        self.board_user = board_user
        self.board_password = board_password
        self.board_mount_path = board_mount_path

    def _get_config_for_actions(self):
        """
        辅助方法，生成一个字典来模拟原 actions.py 所需的 config 对象。
        这样做可以避免大规模重构 actions.py 中的函数签名。
        """
        return {
            'NFS_HOST': {'share_path': self.share_path},
            'PC_INFO': {'password': self.pc_password},
            'BOARD_INFO': {
                'ip': self.board_ip,
                'user': self.board_user,
                'password': self.board_password
            },
            'MOUNT_POINT': {'path': self.board_mount_path}
        }

    def run(self):
        """
        执行核心的“共享与挂载”操作。

        它会调用 actions.py 中的 handle_share 函数，并传入所需的全部配置。
        如果过程中发生任何错误，handle_share 内部会尝试自动回滚。
        """
        print_info("--- 开始执行 NFS 共享和挂载流程 ---")
        config = self._get_config_for_actions()
        try:
            # 调用核心处理函数
            handle_share(config)
        except Exception as e:
            # actions.py 中的函数已经打印了详细错误并尝试了回滚
            # 这里我们只需将异常再次抛出，以便 sidecar.py 能捕获到并以失败状态退出
            raise RuntimeError(f"NFS 共享操作最终失败: {e}")

    def unshare(self):
        """
        执行“卸载与清理”操作。
        """
        print_info("--- 开始执行 NFS 卸载和清理流程 ---")
        config = self._get_config_for_actions()
        try:
            handle_unshare(config)
        except Exception as e:
            print_error(f"卸载清理过程中发生错误: {e}")
            raise