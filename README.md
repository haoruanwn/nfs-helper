# NFS-HELPER

基于 Tauri 2 框架开发的开发者小工具，自动化执行将本地路径挂载到 Linux 开发板。

### 界面展示

![Snipaste_2025-08-09_22-19-08](https://markdownforyuanhao.oss-cn-hangzhou.aliyuncs.com/img1/20250809222122877.png)

### 操作指南

#### 使用 Docker 进行构建

在项目根目录执行以下命令，以使用 Docker 容器环境进行构建。

1. **创建并启动容器**

   ```bash
   docker compose up -d
   ```
   
2. **进入容器环境**

   ```bash
   docker exec -it tauri_builder_custom bash
   ```
   
3. **环境初始化**

   在容器内执行初始化脚本。

   ```bash
   ./init.sh
   ```
   
4. **打包 AppImage**

   执行打包脚本以生成 `AppImage` 文件。

   ```bash
   ./appimage.sh
   ```

### 开发说明

如果需要进行开发和调试，可以直接在原生环境中运行测试脚本。由于在 Docker 容器中运行脚本无法显示图形化界面，因此推荐在原生环境进行开发。

**前置条件**:

- 确保你的原生环境已安装 `Node.js` 和 `Cargo` 等必要的开发工具。

**运行测试**:

```bash
./test.sh
```

### 发行版兼容性说明

#### 构建环境

本应用使用基于 Debian 12 的 Docker 镜像进行构建，以确保良好的兼容性。

| 镜像                                                         | 基础镜像          | 描述                                         |
| ------------------------------------------------------------ | ----------------- | -------------------------------------------- |
| [`ivangabriele/tauri-builder:debian-bookworm-20`](https://hub.docker.com/r/ivangabriele/tauri) | `rust:1-bookworm` | Debian v12 ("bookworm") + Rust v1 + Node v20 |

#### 已测试的发行版

目前已在以下发行版上测试并可以成功运行：

| 发行版       | 桌面环境 | 显示服务器 |
| ------------ | -------- | ---------- |
| Fedora 42    | KDE      | Wayland    |
| Debian 12    | KDE      | Wayland    |
| Ubuntu 24.04 | Gnome    | X11        |


### Bug说明

#### 1\. 禁用硬件加速

在 Fedora 42 KDE Plasma Wayland 环境中，由于 NVIDIA 驱动问题，运行时可能会出现以下报错：

```bash
Failed to create GBM buffer of size 800x600: 无效的参数
```

可以暂时禁用硬件加速来解决，参考 [测试脚本](./test.sh)。

运行 AppImage 时也需要添加相应环境变量：

```bash
WEBKIT_DISABLE_COMPOSITING_MODE=1 ./nfs-helper_0.1.0_amd64.AppImage
```

#### 2\. Ubuntu22.04运行问题
应用使用基于debian12的docker镜像构建，打包的appimage运行在ubuntu22.04上时，会有一个库有版本兼容性问题。目前猜测可以把这个动态库直接打包进appimage，等待后续支持

![C2768852E67BBD66071D379E8D7585BE](https://markdownforyuanhao.oss-cn-hangzhou.aliyuncs.com/img1/20250810003817934.png)

### 致谢与第三方许可证

本项目采用 Apache 2.0 开源协议。项目使用了下列开源组件，这些组件遵循其各自的许可协议。

| Library                                                | License               |
| ------------------------------------------------------ | --------------------- |
| [**Tauri 2**](https://github.com/tauri-apps/tauri.git) | `Apache-2.0` OR `MIT` |
| [**Paramiko**](https://github.com/paramiko/paramiko)   | `LGPL-2.1`            |

