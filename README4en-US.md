# Forge Store Enhancer Extreme
Enhances the ForgeStore experience, while providing extreme hiding of detection points introduced by bootloader unlocking.

> [!TIP]
> 「[简体中文](README.md)」「[繁體中文](README4zh-Hant.md)」

> [!IMPORTANT]
> This module **specializes** in disguising the bootloader status, **rather than** passed Play Integrity.

## Requirements
- Installed the [ForgeStore](https://github.com/TheGeniusClub/ForgeStore), or [TrickyStore](https://github.com/5ec1cff/TrickyStore), or [TrickyStoreOSS](https://github.com/beakthoven/TrickyStoreOSS) or its branch [TEESimulator](https://github.com/JingMatrix/TEESimulator) module
- The mounted system is not OverlayFS

## Install
1. Flash this module and reboot.
2. Manual configuration (optional).
3. Enjoy!

## Feature
### Main
- Add a remove tag of Force delete to conflict module; Directly uninstall the conflict app when detected; monitor in real time
- Take over the ForgeStore module target.txt, with priority over any similar modules; monitor in real time
- At startup, Automatically correct abnormal VerifiedBootHash Property
- At startup, Masquerade the bootloader status as locked
- At startup, Synchronize Security Patch Level to Property
- Provides Google Hardware Attestation Root Certificate signing keybox<sup>Revoked(About to move from the installation process to inside the WebUI)</sup>

### Other
- Display the running environment and startup results in the module description
- Display zh-Hans or en-US according to the system language: Running Status or Installation Process
- Backup the Keybox during installation, and restore the backup during uninstallation. Path: `/data/adb/tricky_store/keybox_backup/keybox.xml`

### FSEE-CLI
- Invoke
  - Execute in the terminal as root: `PATH="/data/adb/modules/fs_enhancer_extreme/bin:$PATH"`
    - Steal Google Hardware Attestation Root Certificate signing keybox: `fseed --stealkeybox` `[-a|-b|-c]`<sup>「[Tricky-Addon](https://github.com/KOWX712/Tricky-Addon-Update-Target-List)」「[Integrity-Box](https://github.com/MeowDump/Integrity-Box)」「[YuriKey-Manager](https://github.com/YurikeyDev/yurikey)」</sup>
    - Fetch the latest Security Patch Level for Pixel Update Announcements online: `fseed --securitypatchdatefetch`
    - ForgeStore Background Service Stop Process / Start Service / Status Check: `fseed --tsctl` `[-stop|-start|-state]`
    - FSEnhancerExtreme Background Service Stop Process / Start Service / Status Check: `fseed --fseectl` `[-stop|-start|-state]`
- Configuration
  - Config directory path: `/data/adb/fs_enhancer_extreme`
  - Log file path: `/data/adb/fs_enhancer_extreme/log`. If you encounter problems, please create an issue and attach the logs.

### WebUI
- Export Log
- Target List Management
- Customize Security Patch Level
- Import keybox from internal storage
- Invoke fseed - Steal Google Hardware Attestation Root Certificate signing keybox
- Invoke fseed - Fetch the latest Security Patch Level for Pixel Update Announcements online
- Invoke fseed - ForgeStore Background Service Stop Process / Start Service / Status Check
- Invoke fseed - FSEnhancerExtreme Background Service Stop Process / Start Service / Status Check

> [!NOTE]
> ### WebUI supports
>   - **KernelSU or APatch**
>     - Native support
>   - **Magisk** 
>     - Provide action button to navigate to [WebUI X Portable](https://github.com/MMRLApp/WebUI-X-Portable) or [KSUWebUIStandalone](https://github.com/5ec1cff/KsuWebUIStandalone)

## Acknowledgement
- [5ec1cff/cmd-wrapper](https://gist.github.com/5ec1cff/4b3a3ef329094e1427e2397cfa2435ff)
- [vvb2060/KeyAttestation](https://github.com/vvb2060/KeyAttestation)