# 介绍

基于 Tauri 2 框架开发的开发者小工具，自动化执行将本地路径挂载到 Linux 开发板。

## 效果展示

![Snipaste_2025-08-09_22-19-08](https://markdownforyuanhao.oss-cn-hangzhou.aliyuncs.com/img1/20250809222122877.png)

## 操作指南

如何编译请参考：[搭建环境](https://v2.tauri.app/zh-cn/start/)

项目提供了初始化脚本 `init.sh` 和便捷测试脚本 `test.sh`。

## Bug说明

### 1\. 禁用硬件加速

在 Fedora 42 KDE Plasma Wayland 环境中，由于 NVIDIA 驱动问题，运行时可能会出现以下报错：

```bash
Failed to create GBM buffer of size 800x600: 无效的参数
```

可以暂时禁用硬件加速来解决，参考 [测试脚本](https://www.google.com/search?q=./test.sh)。

运行 AppImage 时也需要添加相应环境变量：

```bash
WEBKIT_DISABLE_COMPOSITING_MODE=1 ./nfs-helper_0.1.0_amd64.AppImage
```

### 2\. 打包 AppImage 时无法运行 linuxdeploy

在打包 AppImage 过程中，可能会遇到 `linuxdeploy` 运行失败的问题，错误日志如下：

```bash
nfs-helper on  main [!] via  v22.17.1 
❯ APPIMAGE_STRIP_SKIP=1 pnpm tauri build

> nfs-helper@0.1.0 tauri /home/hao/projects/tauri/nfs-helper
> tauri build

      Running beforeBuildCommand `pnpm build`

> nfs-helper@0.1.0 build /home/hao/projects/tauri/nfs-helper
> tsc && vite build

vite v6.3.5 building for production...
✓ 7 modules transformed.
dist/index.html               1.91 kB │ gzip: 0.72 kB
dist/assets/index-D4T-YFLJ.css  2.62 kB │ gzip: 0.97 kB
dist/assets/index-C8nuDSjR.js   3.72 kB │ gzip: 1.70 kB
✓ built in 106ms
    Compiling nfs-helper v0.1.0 (/home/hao/projects/tauri/nfs-helper/src-tauri)
     Finished `release` profile [optimized] target(s) in 58.93s
       Built application at: /home/hao/projects/tauri/nfs-helper/src-tauri/target/release/nfs-helper
        Info Patching binary "/home/hao/projects/tauri/nfs-helper/src-tauri/target/release/nfs-helper" for type appimage
     Bundling nfs-helper_0.1.0_amd64.AppImage (/home/hao/projects/tauri/nfs-helper/src-tauri/target/release/bundle/appimage/nfs-helper_0.1.0_amd64.AppImage)
failed to bundle project: `failed to run linuxdeploy`
        Error failed to bundle project: `failed to run linuxdeploy`
```

根据相关讨论 [[bug] Calling strip causes Tauri to fail building AppImage](https://github.com/tauri-apps/tauri/issues/11149)，可以在打包时执行以下命令来解决：

```bash
NO_STRIP=true pnpm tauri build
```
