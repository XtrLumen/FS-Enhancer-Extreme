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

mod apatch;
mod kernelsu;
mod magisk;

use crate::{
    define::{
        FS_STR,
        FSMODDIR,
        TSMODDIR,
        ROOT_IMPL_ENV_FILE,
        MAIN_MODULE_ENV_FILE
    },
    bridge::log_e,
    util_functions::{
        read_to_string,
        delete_file
    }
};

use std::{
    fs,
    path::Path,
    sync::OnceLock,
};

struct Environment {
    multiple: bool,
    identity: String,
    version_code: String
}

static ROOT_INPL_ENVIRONMENT: OnceLock<Environment> = OnceLock::new();
static MAIN_MODULE_ENVIRONMENT: OnceLock<Environment> = OnceLock::new();

fn root_collect() {
    let mut collect_result: Vec<(&str, u32)> = Vec::new();
    if let Some(version_code) = apatch::detect() {
        collect_result.push(("APatch", version_code));
    }
    if let Some(version_code) = kernelsu::detect() {
        collect_result.push(("KernelSU", version_code));
    }
    if let Some(version_code) = magisk::detect() {
        collect_result.push(("Magisk", version_code));
    }

    let environment = match collect_result.len() {
        0 => Environment {
            multiple: false,
            identity: String::from("Unknown"),
            version_code: String::new()
        },
        1 => {
            let (identity, version_code) = collect_result[0];
            Environment {
                multiple: false,
                identity: String::from(identity),
                version_code: format!("{}", version_code)
            }
        },
        _ => {
            let multiple_type: Vec<String> = collect_result.iter().map(|(identity, version_code)|
                format!("{}({})", identity, version_code)
            ).collect();
            Environment {
                multiple: true,
                identity: format!("{}", multiple_type.join("|")),
                version_code: String::new()
            }
        }
    };

    ROOT_INPL_ENVIRONMENT.set(environment).ok();
}

fn main_module_collect() {
    let mut collect_result: Vec<(&str, String)> = Vec::new();

    for (dir, identity) in [
        (FSMODDIR, FS_STR),
        (TSMODDIR, "TSMaster")
    ] {
        let prop_full_path = format!("{}/module.prop", dir);
        let prop_file = Path::new(&prop_full_path);
        if prop_file.exists() {
            if let Ok(content) = read_to_string(prop_file) {
                if let Some(version_code) = content.lines().find_map(|line|
                    line.strip_prefix("versionCode=").map(|value|
                        value.trim().to_string()
                    )
                ) {
                    collect_result.push((
                        if identity == "TSMaster" {
                            if content.lines().find_map(|line|line.strip_prefix("name=")).is_some_and(|name|
                            name.contains("Tricky") && name.contains("Store") && name.contains("OSS")) {
                                "TrickyStoreOSS"
                            } else if content.lines().find_map(|line|line.strip_prefix("name=")).is_some_and(|name|
                            name.contains("TEESimulator") && name.contains("RS")) {
                                "TEESimulatorRS"
                            } else if content.lines().find_map(|line|line.strip_prefix("name=")).is_some_and(|name|
                            name.contains("TEESimulator")) {
                                "TEESimulator"
                            } else {
                                "TrickyStore"
                            }
                        } else {
                            identity
                        },
                        version_code
                    ));
                }
            }
        }
    }

    let environment = match collect_result.len() {
        0 => Environment {
            multiple: false,
            identity: String::from("Unknown"),
            version_code: String::new()
        },
        1 => {
            let (identity, version_code) = &collect_result[0];
            Environment {
                multiple: false,
                identity: String::from(*identity),
                version_code: format!("{}", version_code)
            }
        },
        _ => {
            let multiple_type: Vec<String> = collect_result.iter().map(|(identity, version_code)|{
                format!("{}({})", identity, version_code)
            }).collect();
            Environment {
                multiple: true,
                identity: format!("{}", multiple_type.join("|")),
                version_code: String::new(),
            }
        }
    };

    MAIN_MODULE_ENVIRONMENT.set(environment).ok();
}

pub fn internal_entry() {
    root_collect();
    main_module_collect();
    let (root_env, main_module_env) = (ROOT_INPL_ENVIRONMENT.get().unwrap(), MAIN_MODULE_ENVIRONMENT.get().unwrap());

    println!("root_impl:\n{}\n{}\n{}", root_env.multiple, root_env.identity, root_env.version_code);
    println!("main_module:\n{}\n{}\n{}", main_module_env.multiple, main_module_env.identity, main_module_env.version_code);

    for (path, env) in [
        (ROOT_IMPL_ENV_FILE, root_env),
        (MAIN_MODULE_ENV_FILE, main_module_env),
    ] {
        if Path::new(path).exists() {
            delete_file(path)
        }
        let data = format!("{}\n{}\n{}", env.multiple, env.identity, env.version_code);
        if let Err(error) = fs::write(path, data) {
            log_e(&format!("写入失败: {}", error));
        }
    }
}

pub fn external_entry() {
    internal_entry()
}