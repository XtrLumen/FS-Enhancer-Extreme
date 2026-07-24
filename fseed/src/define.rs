/*
 * This file is part of FS-Enhancer-Extreme.
 *
 * This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
 * without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with this program;
 * If not, see <https://www.gnu.org/licenses/>.
 *
 * Copyright (C) 2026 XtrLumen
 */

use crate::{
    util_functions::{
        getprop,
        read_multiple_bool,
        read_identity_string
    },
    envcollect::internal_entry
};

use std::{
    path::Path,
    sync::LazyLock
};

pub static IS_ZHCN: LazyLock<bool> = LazyLock::new(||
    getprop("persist.sys.locale").unwrap().contains("zh") || getprop("ro.product.locale").unwrap().contains("zh")
);

pub static CONFLICT_DESC_LINE: LazyLock<&str> = LazyLock::new(||
    if *IS_ZHCN {
        "description=此模块与FS-Enhancer-Extreme证实冲突,已被添加移除标签,将在设备下一次启动时被移除."
    } else {
        "description=This module has been confirmed to conflict with the FS-Enhancer-Extreme. Has been tagged for remove, Will be removed upon the devide next boot."
    }
);

pub const CONFLICT_APP: &[&str] = &[
    "com.lingqian.appbl",
    "com.topmiaohan.hidebllist"
];

pub const TAG_CONFLICT_MOD: &[&str] = &[
    "Yurikey",
    "xiaocaiye",
    "vbmeta-fixer",
    "safetynet-fix",
    "playintegrity",
    "integrity_box",
    "SukiSU_module",
    "ShamikoManager",
    "Reset_BootHash",
    "Tricky_store-bm",
    "Hide_Bootloader",
    "extreme_hide_root",
    "ts_enhancer_extreme",
    "Tricky_Store-xiaoyi",
    "tricky_store_assistant",
    "extreme_hide_bootloader",
    "wjw_hiderootauxiliarymod"
];

pub const DEL_CONFLICT_MOD: &[&str] = &[
    "TA_utl",
    ".TA_utl",
    "Yamabukiko"
];

pub const LOG_TAG: &str = "cli";
pub const VERSION_NAME: &str = env!("VERSION_NAME");

pub const BLKSSZGET: libc::c_int = 0x1268u32 as libc::c_int;

pub const OFF: &str = "OFF";
pub const MULTIPLE: &str = "MULTIPLE";
pub const UNKNOWN: &str = "Unknown";

pub const FS_STR: &str = "ForgeStore";
pub const FSMODDIR: &str = "/data/adb/modules/forge_store";
pub const TSMODDIR: &str = "/data/adb/modules/tricky_store";
pub const FSEECONFIG: &str = "/data/adb/fs_enhancer_extreme/config";
pub const FSEEMODDIR: &str = "/data/adb/modules/fs_enhancer_extreme";
pub const MODULESDIR: &str = "/data/adb/modules";
pub const MODULESUPDATEDIR: &str = "/data/adb/modules_update";
pub const ROOT_IMPL_ENV_FILE: &str = "/data/adb/fs_enhancer_extreme/root_impl";
pub const MAIN_MODULE_ENV_FILE: &str = "/data/adb/fs_enhancer_extreme/main_module";

pub static MAIN_MODULE_IDENTITY: LazyLock<String> = LazyLock::new(||{
    if !Path::new(MAIN_MODULE_ENV_FILE).exists() {
        internal_entry()
    }
    let main_module_identity = read_identity_string(MAIN_MODULE_ENV_FILE);
    let fs_disable = Path::new(&format!("{}/disable", FSMODDIR)).exists();
    let ts_disable = Path::new(&format!("{}/disable", TSMODDIR)).exists();
    if read_multiple_bool(MAIN_MODULE_ENV_FILE) {
        if fs_disable && ts_disable {
            String::from(OFF)
        } else if !fs_disable && ts_disable {
            String::from(FS_STR)
        } else if fs_disable && !ts_disable {
            main_module_identity
                .rsplit('|').next().unwrap()
                .split('(').next().unwrap()
                .to_string()
        } else {
            String::from(MULTIPLE)
        }
    } else {
        if fs_disable || ts_disable {
            String::from(OFF)
        } else {
            main_module_identity
        }
    }
});

pub static ENV_NORMAL: LazyLock<bool> = LazyLock::new(||
    !(read_multiple_bool(ROOT_IMPL_ENV_FILE) || matches!(MAIN_MODULE_IDENTITY.as_str(), MULTIPLE | OFF))
);

pub static FINAL_NICE_NAME: LazyLock<&str> = LazyLock::new(||
    match MAIN_MODULE_IDENTITY.as_str() {
        "TEESimulator" | "TEESimulatorRS" => "TEESimulator",
        other => other
    }
);
pub static FINAL_MAIN_MODULE_DIR: LazyLock<&str> = LazyLock::new(||
    if *MAIN_MODULE_IDENTITY != FS_STR {
        TSMODDIR
    } else {
        FSMODDIR
    }
);
pub static FINAL_MAIN_MODULE_CONFIG: LazyLock<&str> = LazyLock::new(||
    if *MAIN_MODULE_IDENTITY != FS_STR {
        "/data/adb/tricky_store"
    } else {
        "/data/adb/forge_store"
    }
);

pub static DESC_BASE: LazyLock<String> = LazyLock::new(||{
    let final_identity: &str = if *ENV_NORMAL {
        MAIN_MODULE_IDENTITY.as_str()
    } else {
        FS_STR
    };
    if *IS_ZHCN {
        format!("提升{}体验,同时极致隐藏由解锁引导加载程序产生的相关检测点.", final_identity)
    } else {
        format!("Enhance {} experience, while providing extreme hiding of detection points introduced by bootloader unlocking.", final_identity)
    }
});

pub static DESC_SERVICE_SUCCESS: LazyLock<&str> = LazyLock::new(||
    if *IS_ZHCN {
        "✅服务运行中"
    } else {
        "✅Service is running"
    }
);
pub static DESC_SERVICE_FAILURE: LazyLock<&str> = LazyLock::new(||
    if *IS_ZHCN {
        "❌服务无法启动"
    } else {
        "❌Daemon cannot start"
    }
);
pub static DESC_SERVICE_NOT_START: LazyLock<&str> = LazyLock::new(||
    if *IS_ZHCN {
        "❌所有服务将不会启动!"
    } else {
        "❌All service will not start!"
    }
);
pub static DESC_MULTIPLE: LazyLock<&str> = LazyLock::new(||
    if *IS_ZHCN {
        "❌多重共存-"
    } else {
        "❌Multiple-"
    }
);
pub static DESC_MAIN_MODULE_NOT_INSTALL: LazyLock<&str> = LazyLock::new(||
    if *IS_ZHCN {
        "未安装!"
    } else {
        "Not installed!"
    }
);
pub static DESC_DISABLE: LazyLock<&str> = LazyLock::new(||
    if *IS_ZHCN {
        "被禁用"
    } else {
        "Disabled"
    }
);
pub static DESC_ROOT_IMPL: LazyLock<&str> = LazyLock::new(||
    if *IS_ZHCN {
        "根实现: "
    } else {
        "Root: "
    }
);
pub static DESC_MAIN_MODULE: LazyLock<&str> = LazyLock::new(||
    if *IS_ZHCN {
        "主模块: "
    } else {
        "MainModule: "
    }
);