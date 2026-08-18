# Forge Store Enhancer Extreme
Enhancer of ForgeStore, Extreme hiding of detection points from unlocking bootloader.

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
- `libc::inotify*` real-time monitoring
  - Add a remove tag / Force delete to conflict module; Directly uninstall the conflict app when detected
  - Take over the ForgeStore module target.txt, with priority over any similar modules
- Provides Google Hardware Attestation Root Certificate signing keybox
- At device startup
  - Automatically correct abnormal VerifiedBootHash prop
  - Set the bootloader prop to locked
  - Sync Security Patch Level to prop

### Other
- Avoid abnormal environments
- Display detailed dashboard in module description, Example:  
`[Root: ✅APatch(11224), MainModule: ✅TrickyStoreOSS(155), Integrity: ✅Verified, Daemon: ✅Running]`  
`[Root: ❌Multiple-APatch(11224)|KernelSU(32525), MainModule: ❌Multiple-ForgeStore(143)|TrickyStore(248), Integrity: ⚠️This build is unsigned, Service: ❌All service will not start]`
- Display zh-Hans or en-US based on the system language: User-visible part
  - Force en-US: Create `/data/adb/fs_enhancer_extreme/config/english` empty file

### WebUI
- DEV VERSION STUB

### CLI
- Execute in the terminal as root `/data/adb/modules/fs_enhancer_extreme/bin/fseec`
- Command List
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
  - Keybox Manager
    - `keybox` `builtin|import<<<path`

### Configuration
  - Config directory path: `/data/adb/fs_enhancer_extreme/config`
  - Log directory path: `/data/adb/fs_enhancer_extreme/log|log.old`. If encounter problems, please create an issue and attach the logs.

> [!NOTE]
> ### WebUI Launch
>   - **KernelSU or APatch**
>     - Native support
>   - **Magisk** 
>     - Provide action button to navigate to [WebUI X Portable](https://github.com/MMRLApp/WebUI-X-Portable) or [KSUWebUIStandalone](https://github.com/5ec1cff/KsuWebUIStandalone)

## Build
### Environment
- JDK 21
- Node.js 24
- Android SDK
- Android NDK 29.0.14206865
- Rust Nightly 2026-01-01
- cargo-ndk

### Packaging
- Execute in the terminal `./gradlew zip`

## Acknowledgement
- [fluentui](https://github.com/microsoft/fluentui)
- [APatch](https://github.com/bmax121/APatch)
  - fseec `cli.rs` References
- [NeoZygisk](https://github.com/JingMatrix/NeoZygisk)
  - fseec `magisk.rs` References
- [KeyAttestation](https://github.com/vvb2060/KeyAttestation)
  - VBMetaProvider `attestation/*` Direct Source
- [Android-Open-Source-Project](https://cs.android.com/android/platform/superproject)
  - fseec `fn pidof` References

## Project address (for users downloading from sources other than GitHub to trace back from this README)
- https://github.com/XtrLumen/FS-Enhancer-Extreme

## Just for fun!