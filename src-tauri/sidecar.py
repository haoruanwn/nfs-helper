# scripts/sidecar.py

import sys
import argparse
from lib.nfs_tool import NfsTool
import logging

logging.basicConfig(level=logging.INFO, format='%(message)s', stream=sys.stdout)

def main():
    parser = argparse.ArgumentParser(description="NFS and SSH configuration helper.")
    subparsers = parser.add_subparsers(dest='command', help='Available commands', required=True)

    # 定义 "share" 子命令及其参数
    # 【重要】这里的参数顺序必须和 Rust `args` vector 中的顺序严格对应
    parser_share = subparsers.add_parser('share', help='Configure NFS share and mount on board.')
    parser_share.add_argument('pc_path', type=str, help='The local directory to share.')
    parser_share.add_argument('pc_password', type=str, help='Sudo password for the local PC.')
    parser_share.add_argument('pc_ip', type=str, help='IP address of the local PC (the NFS server).')
    parser_share.add_argument('board_ip', type=str, help='IP address of the remote board.')
    parser_share.add_argument('board_user', type=str, help='Username for the remote board.')
    parser_share.add_argument('board_password', type=str, help='Password for the remote board.')
    parser_share.add_argument('board_path', type=str, help='The mount point on the remote board.')

    args = parser.parse_args()

    if args.command == 'share':
        try:
            tool = NfsTool(
                pc_ip=args.pc_ip,           
                pc_password=args.pc_password,
                share_path=args.pc_path,
                board_ip=args.board_ip,
                board_user=args.board_user,
                board_password=args.board_password,
                board_mount_path=args.board_path
            )
            tool.run()
            sys.exit(0)
        except Exception as e:
            logging.error(f"An error occurred: {e}")
            sys.exit(1)

if __name__ == '__main__':
    main()