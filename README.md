# Forge Store Enhancer Extreme
提升ForgeStore体验，同时极致隐藏由解锁引导加载程序产生的相关检测点。

> [!TIP]
> 「[English](README4en-US.md)」

> [!IMPORTANT]  
> 本模块**专精**伪装引导加载程序状态，**而非**通过PlayIntegrity。

## 条件
- 已安装 [ForgeStore](https://github.com/TheGeniusClub/ForgeStore)，或 [TrickyStore](https://github.com/5ec1cff/TrickyStore)，或 [TrickyStoreOSS](https://github.com/beakthoven/TrickyStoreOSS) 或它的分支 [TEESimulator](https://github.com/JingMatrix/TEESimulator) 或它的分支 [TEESimulator-RS](https://github.com/Enginex0/TEESimulator-RS) 模块
- 挂载系统不是 OverlayFS

## 安装
1. 刷入模块并重新启动设备。
2. 手动配置(可选)。
3. 完成！

## 功能
### 主要
- 对冲突模块添加移除标签/强制删除；检测到冲突软件时直接卸载；`libc::inotify_add_watch`实时监控
- 接管ForgeStore模块target.txt，优先级高于任何类似模块；`libc::inotify_add_watch`实时监控
- 提供谷歌硬件认证根证书签名的keybox
- 设备启动时
  - 全自动修正异常VerifiedBootHash属性
  - 将安全补丁级别同步到属性
  - 将引导程序属性设置为锁定

### 其他
- 规避异常环境
- 在模块描述显示详细仪表盘
- 根据系统语言分别显示zh-Hans或en-US: 用户可见部分

### WebUI
- DEV VERSION STUB

### 命令行工具
- 调用
  - 于终端以Root身份执行`/data/adb/modules/fs_enhancer_extreme/bin/fseed`
  - 命令
    - Operation Forge Store service
      - `fsctl` `restart|start|stop|state`
    - Operation FS Enhancer Extreme service
      - `fseectl` `restart|start|stop|state`
    - Check running environment if normal from envcollect cache
      - `envcheck`
    - Check and directly uninstall conflict apps
      - `appcheck`
    - Check and add remove tag or force delete conflict modules
      - `modcheck/-d|--daemon`
    - Through Bootloader unlock related prop detection
      - `passprop`
    - Automatically correct abnormal VerifiedBootHash prop
      - `passvbhash`
    - Launch standalone WebUI app to id fs_enhancer_extreme
      - `startwebui`
    - Sync Security Patch Level from security_patch.txt to prop
      - `spsyncprop`
    - Detect and cache all necessity runtime environments
      - `envcollect`
    - Refresh module decription line from envcollect cache
      - `descrefresh/-d|--debug`
    - Refresh Forge Store target.txt from user config
      - `listrefresh`

### 配置
  - 配置目录路径: `/data/adb/fs_enhancer_extreme/config`
  - 日志目录路径: `/data/adb/fs_enhancer_extreme/log|log.old`，如遇到问题，请创建 issue 并附上日志。

> [!NOTE]
> ### WebUI支持
>   - **KernelSU 或 APatch**
>     - 原生支持
>   - **Magisk**
>     - 提供跳转到 [WebUI X Portable](https://github.com/MMRLApp/WebUI-X-Portable) 或 [KSUWebUIStandalone](https://github.com/5ec1cff/KsuWebUIStandalone) 的 Action 按钮

## 致谢
- [bmax121/APatch](https://github.com/bmax121/APatch) fseed 命令行解析部分 参考来源
- [JingMatrix/NeoZygisk](https://github.com/JingMatrix/NeoZygisk) fseed-root实现分析 执行部分 参考来源
- [vvb2060/KeyAttestation](https://github.com/vvb2060/KeyAttestation) vbmeta提供服务 执行部分 来源
- [Google-Inc/Android-Open-Source-Project](https://cs.android.com/android/platform/superproject/) fseed-pidof 参考来源

## 本项目地址(用于非Github下载的用户从本自述文件溯源)
- https://github.com/XtrLumen/FS-Enhancer-Extreme

## Just for fun