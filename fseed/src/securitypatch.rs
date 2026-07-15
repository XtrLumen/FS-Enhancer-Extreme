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
    define::{
        ENV_NORMAL,
        FINAL_MAIN_MODULE_CONFIG
    },
    bridge::{
        log_i,
        log_e,
        log_w
    },
    util_functions::{
        pidof,
        kill,
        resetprop,
        read_to_string
    }
};

use std::{
    process,
    path::Path,
};

use regex::Regex;

pub fn sync() -> anyhow::Result<()> {
    if *ENV_NORMAL {
        let security_patch_full_path = format!("{}/security_patch.txt", *FINAL_MAIN_MODULE_CONFIG);
        let security_patch_file = Path::new(&security_patch_full_path);
        if security_patch_file.exists() {
            if let Ok(success) = read_to_string(security_patch_file) {
                if !success.is_empty() {
                    let date = if Regex::new(r"^\d{4}-\d{2}-\d{2}$")?.is_match(&success) {
                        Some(success)
                    } else if Regex::new(r"^\d{8}$")?.is_match(&success) {
                        Some(format!(
                            "{}-{}-{}",
                            &success[0..4],
                            &success[4..6],
                            &success[6..8]
                        ))
                    } else {
                        None
                    };
                    match date {
                        Some(date) => {
                            resetprop(&["ro.vendor.build.security_patch", &date])?;
                            resetprop(&["ro.build.version.security_patch", &date])?;
    
                            if let Ok(Some(pid)) = pidof("com.google.android.gms.unstable") {
                                kill(pid)?;
                            }
    
                            log_i("同步完毕");
                        }
                        None => {
                            log_e("格式错误");
                            process::exit(1)
                        }
                    }
                } else {
                    log_w("解析失败");
                    process::exit(1)
                }
            }
        } else {
            log_w("文件不存在");
            process::exit(1)
        }
    } else {
        println!("Environment abnormal")
    }

    Ok(())
}