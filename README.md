## 安装依赖
```bash
pnpm install
```


## 禁用硬件加速
不然我使用的环境会因为n卡驱动问题导致失败
```bash
WEBKIT_DISABLE_COMPOSITING_MODE=1 pnpm tauri dev
```

## nfs自动执行逻辑
使用tauri的sidecar，调用python进程，来执行本地配置和ssh到目标开发板，来自动化执行代码

## 打包sidecar
创建虚拟环境
```bash
# 在src-tauri/下执行
python3 -m venv sidecar-venv

# 激活环境
source sidecar-venv/bin/activate

# 在这个激活的环境中，安装 paramiko 和 pyinstaller
pip install paramiko pyinstaller
```
执行打包操作
```bash
pyinstaller --onefile --name nfs-automator sidecar.py
```

## 操作指南
提供了项目初始化脚本和便捷测试脚本(init.sh 和 test.sh)
## 效果展示

![Snipaste_2025-08-09_20-02-37](https://markdownforyuanhao.oss-cn-hangzhou.aliyuncs.com/img1/20250809201028072.png)
