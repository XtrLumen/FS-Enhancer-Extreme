# Forge Store Enhancer Extreme
Enhance ForgeStore experience, while providing extreme hiding of detection points introduced by bootloader unlocking.

> [!TIP]
> 「[简体中文](README.md)」

> [!IMPORTANT]
> This module **specializes** in disguising the bootloader status, **rather than** passed Play Integrity.

## Requirements
- Installed the [ForgeStore](https://github.com/TheGeniusClub/ForgeStore), or [TrickyStore](https://github.com/5ec1cff/TrickyStore), or [TrickyStoreOSS](https://github.com/beakthoven/TrickyStoreOSS) or its branch [TEESimulator](https://github.com/JingMatrix/TEESimulator) or its branch [TEESimulator-RS](https://github.com/Enginex0/TEESimulator-RS) module
- The mounted system is not OverlayFS

## Install
1. Flash this module and reboot.
2. Manual configuration (optional).
3. Enjoy!

## Feature
### Main
- Add a remove tag / Force delete to conflict module; Directly uninstall the conflict app when detected; `libc::inotify_add_watch` real-time monitoring
- Take over the ForgeStore module target.txt, with priority over any similar modules; `libc::inotify_add_watch` real-time monitoring
- Provides Google Hardware Attestation Root Certificate signing keybox
- At device startup
  - Automatically correct abnormal VerifiedBootHash prop
  - Set the bootloader prop to locked
  - Sync Security Patch Level to prop

### Other
- Avoid abnormal environments
- Display detailed dashboard in module description
- Display zh-Hans or en-US based on the system language: User-visible part

### CLI
- Invoke
  - Execute in the terminal as root`/data/adb/modules/fs_enhancer_extreme/bin/fseed`
  - Command
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
      - `descrefresh/-f|--force`
    - Refresh Forge Store target.txt from user config
      - `listrefresh`

- Configuration
  - Config directory path: `/data/adb/fs_enhancer_extreme/config`
  - Log file path: `/data/adb/fs_enhancer_extreme/log|log.old`. If you encounter problems, please create an issue and attach the logs.

### WebUI
- DEV VERSION STUB

> [!NOTE]
> ### WebUI supports
>   - **KernelSU or APatch**
>     - Native support
>   - **Magisk** 
>     - Provide action button to navigate to [WebUI X Portable](https://github.com/MMRLApp/WebUI-X-Portable) or [KSUWebUIStandalone](https://github.com/5ec1cff/KsuWebUIStandalone)

## Acknowledgement
- [bmax121/APatch](https://github.com/bmax121/APatch)
- [JingMatrix/NeoZygisk](https://github.com/JingMatrix/NeoZygisk)
- [vvb2060/KeyAttestation](https://github.com/vvb2060/KeyAttestation)
- [Google-Inc/Android-Open-Source-Project](https://cs.android.com/android/platform/superproject/)

## Project address (for users downloading from sources other than GitHub to trace back from this README)
- https://github.com/XtrLumen/FS-Enhancer-Extreme

## Just for fun!