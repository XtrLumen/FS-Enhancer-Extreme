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
        MAIN_MODULE_IDENTITY,
        FSEEMODDIR,
        FSEECONFIG
    },
    bridge::{
        log_i,
        log_e,
        log_w,
        log_raw
    },
    util_functions::{
        resetprop,
        getprop,
        pm_install,
        pm_uninstall,
        read_to_string,
        read_version_integer
    }
};

use std::{
    fs,
    process,
    path::Path
};

fn get_vbhash() -> Option<String> {
    if let Ok(success) = getprop("ro.boot.vbmeta.digest") {
        Some(success)
    } else {
        None
    }
}

pub fn entry() -> anyhow::Result<()> {
    let persist_hash_full_path = format!("{}/verifiedboothash", FSEECONFIG);

    let set_vbhash = |value: &str| -> anyhow::Result<()> {
        resetprop(&["-n", "ro.boot.vbmeta.digest", value])
    };

    let write_persist_hash = |data: &str| {
        if let Err(error) = fs::write(&persist_hash_full_path, &data) {
            log_e(&format!("写入失败: {}", error))
        } else {
            log_i("写入成功")
        }
    };

    let now_vbhash: Option<String> = get_vbhash();

    let contentapp = |cache: bool| -> bool {
        log_i("安装服务");
        if pm_install(&format!("{}/provider.apk", FSEEMODDIR)) {
            log_i("安装完毕");
            log_i("尝试启动");
            let content_result = process::Command::new("content").args(&["call", "--uri", "content://VBMetaProvider", "--method", "GET", "--extra", "field:s:verifiedBootHash"])
                .output();
            let new_vbhash: Option<String> = match content_result {
                Ok(output) => {
                    let content_stdout = String::from_utf8(output.stdout).unwrap().trim().to_string();
                    log_i(&content_stdout);
                    let marker: &str = "=verifiedBootHash";
                    if content_stdout.contains(marker) {
                        let value = content_stdout.split(marker).next().unwrap();
                        Some(value[value.len() - 64..].to_string())
                    } else {
                        None
                    }
                }
                Err(error) => {
                    log_e(&format!("content执行失败: {}", error));
                    None
                }
            };
            let is_success: bool = if let Some(success) = new_vbhash {
                log_i("解析成功");
                let is_success: bool = if Some(&success) == now_vbhash.as_ref() {
                    log_i("无需修正");
                    true
                } else {
                    if let Some(success) = now_vbhash.as_ref() {
                        if set_vbhash(&success).is_ok() {
                            log_i("修正完毕");
                            true
                        } else {
                            log_e("修正失败");
                            false
                        }
                    } else {
                        false
                    }
                };
                if cache {
                    log_i("缓存数据");
                    write_persist_hash(&success);
                }

                is_success
            } else {
                log_e("解析失败");
                log_e("抓取日志");
                match process::Command::new("logcat").args(&["-d", "-s", "[FSEE]"]).output() {
                    Ok(result) => {
                        let stdout = String::from_utf8(result.stdout).unwrap();
                        let filtered: String = stdout.lines().filter(|line|
                            !line.contains("beginning of")
                        ).intersperse("\n").collect();
                        if !filtered.is_empty() {
                            log_raw(&filtered);
                        } else {
                            log_e("抓取失败");
                        }
                    }
                    Err(error) => log_e(&format!("执行失败: {}", error))
                }

                false
            };
            log_i("卸载服务");
            pm_uninstall("io.github.xtrlumen.vbmeta");

            is_success
        } else {
            false
        }
    };

    let err_apply_random_vbhash = || {
        if !contentapp(true) {
            log_w("获取失败,生成随机哈希值作为VerifiedBootHash并缓存数据");
            let mut buffer = [0u8; 32];
            if let Err(error) = getrandom::fill(&mut buffer) {
                log_e(&format!("getrandom调用失败: {}", error));
                return;
            } else {
                let hash = buffer.iter().map(|byte|
                    format!("{:02x}", byte)
                ).collect::<String>();

                if set_vbhash(&hash).is_ok() {
                    log_i(&format!("重置完毕,当前VerifiedBootHash: {:?}", get_vbhash()))
                } else {
                    log_e("重置失败")
                }

                write_persist_hash(&hash);
            }
        }
    };

    if *MAIN_MODULE_IDENTITY == "TrickyStore" && read_version_integer("main_module") >= 245 {
        let persist_hash_file = Path::new(&persist_hash_full_path);
        if persist_hash_file.exists() {
            if let Ok(success) = read_to_string(persist_hash_file) {
                if now_vbhash.as_ref() == Some(&success) {
                    log_i("无需修正");
                } else {
                    if set_vbhash(&success).is_ok() {
                        log_i(&format!("修正完毕,当前VerifiedBootHash: {:?}", get_vbhash()))
                    } else {
                        log_e("修正失败")
                    }
                }
            } else {
                log_w("缓存读取失败,执行完整流程");
                err_apply_random_vbhash()
            }
        } else {
            log_w("缓存文件缺失,执行完整流程");
            err_apply_random_vbhash();
        }
    } else {
        contentapp(false);
    }

    Ok(())
}