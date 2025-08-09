import sys
import argparse
import logging
from lib.nfs_tool import NfsTool

# 配置日志，使其能被Rust实时捕获
logging.basicConfig(level=logging.INFO, format='%(message)s', stream=sys.stdout)

def main():
    parser = argparse.ArgumentParser(description="NFS and SSH configuration helper.")
    subparsers = parser.add_subparsers(dest='command', help='Available commands', required=True)

    parser_share = subparsers.add_parser('share', help='Configure NFS share and mount on board.')
    parser_share.add_argument('pc_path', type=str)
    parser_share.add_argument('pc_password', type=str)
    parser_share.add_argument('pc_ip', type=str)
    parser_share.add_argument('board_ip', type=str)
    parser_share.add_argument('board_user', type=str)
    parser_share.add_argument('board_password', type=str)
    parser_share.add_argument('board_path', type=str)
    
    args = parser.parse_args()

    if args.command == 'share':
        try:
            # 将解析出的字符串参数直接传递
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
            sys.exit(0) # 明确成功退出
        except Exception as e:
            # NfsTool中发生的任何错误都会在这里被捕获
            logging.error(f"NFS share operation failed: {e}")
            sys.exit(1) # 明确失败退出

if __name__ == '__main__':
    main()