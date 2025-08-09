# lib/common/preset_manager.py
import os
import shutil
from .utils import print_success, print_error, print_info, print_warning

def _get_paths(tool_name):
    """辅助函数，获取预设和配置文件的相关路径"""
    project_root = os.path.dirname(os.path.dirname(os.path.dirname(os.path.realpath(__file__))))
    # 存放配置文件的目录
    config_dir = os.path.join(project_root, 'config')
    presets_dir = os.path.join(config_dir, 'presets', tool_name)
    default_config_path = os.path.join(config_dir, f'{tool_name}_default.ini')
    os.makedirs(presets_dir, exist_ok=True)
    return presets_dir, default_config_path

def list_presets(tool_name):
    """列出指定工具的所有可用预设"""
    presets_dir, _ = _get_paths(tool_name)
    print_info(f"'{tool_name}' 工具的可用预设方案:")
    try:
        presets = [f for f in os.listdir(presets_dir) if f.endswith('.ini')]
        if not presets:
            print_warning("  -> 未找到任何预设。")
            return None
        for i, preset_name in enumerate(presets, 1):
            print(f"  {i}. {preset_name.replace('.ini', '')}")
        return presets
    except FileNotFoundError:
        print_warning("  -> 预设目录不存在。")
        return None

def save_preset(tool_name):
    """将当前工具的默认配置保存为一个新的预设"""
    presets_dir, default_config_path = _get_paths(tool_name)
    
    try:
        preset_name = input("请输入新预设的名称 (例如: project_alpha): ").strip()
        if not preset_name or not preset_name.isidentifier():
            print_error("名称无效，只能包含字母、数字和下划线，且不能以数字开头。", exit_code=None)
            return

        destination_path = os.path.join(presets_dir, f"{preset_name}.ini")
        if os.path.exists(destination_path):
            overwrite = input(f"预设 '{preset_name}' 已存在。是否覆盖? (y/N): ").lower()
            if overwrite != 'y':
                print_info("操作已取消。")
                return

        shutil.copy(default_config_path, destination_path)
        print_success(f"当前配置已成功保存为预设: '{preset_name}'")

    except KeyboardInterrupt:
        print_warning("\n操作已取消。")
    except Exception as e:
        print_error(f"保存预设时发生错误: {e}", exit_code=None)

def load_preset(tool_name):
    """加载一个预设方案，使其成为新的默认配置"""
    presets_dir, default_config_path = _get_paths(tool_name)
    presets = list_presets(tool_name)

    if not presets:
        return

    try:
        choice = input("请选择要加载的预设编号: ").strip()
        if not choice.isdigit() or not (1 <= int(choice) <= len(presets)):
            print_error("无效的选择。", exit_code=None)
            return
        
        selected_preset_file = presets[int(choice) - 1]
        source_path = os.path.join(presets_dir, selected_preset_file)
        
        shutil.copy(source_path, default_config_path)
        print_success(f"预设 '{selected_preset_file.replace('.ini', '')}' 已加载，并成为当前默认配置。")

    except KeyboardInterrupt:
        print_warning("\n操作已取消。")
    except Exception as e:
        print_error(f"加载预设时发生错误: {e}", exit_code=None)