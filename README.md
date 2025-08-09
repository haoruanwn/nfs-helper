## 安装依赖
```bash
pnpm install
```


## 禁用硬件加速
不然我使用的环境会因为n卡驱动问题导致失败
```bash
WEBKIT_DISABLE_COMPOSITING_MODE=1 pnpm tauri dev
```

