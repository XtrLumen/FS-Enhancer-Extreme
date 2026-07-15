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
    define::IS_ZHCN,
    bridge::log_e
};

use std::{
    fs,
    str,
    env,
    process,
    path::Path,
    os::fd::AsRawFd,
    io::{
        Error,
        Result
    }
};

use anyhow::{
    anyhow,
    ensure
};

pub fn switch_mnt_namespace() -> anyhow::Result<()> {
    let init_proc_mnt = "/proc/1/ns/mnt";
    let file_descriptor = fs::File::open(init_proc_mnt)?;
    let current_dir = env::current_dir();
    let return_type = unsafe {libc::setns(file_descriptor.as_raw_fd(), libc::CLONE_NEWNS)};
    if let Ok(current_dir) = current_dir {
        env::set_current_dir(current_dir)?;
    }
    ensure!(return_type == 0, "switch mnt namespace failed");

    Ok(())
}

pub fn pidof(name: &str) -> anyhow::Result<Option<i32>> {
    for process in fs::read_dir("/proc")?.flatten() {
        let temp_file_name = process.file_name();
        let file_name = temp_file_name.to_str().unwrap();

        let pid: i32 = if let Ok(integer) = file_name.parse() {
            integer
        } else {
            continue
        };

        let cmdline = if let Ok(exist) =  fs::read(process.path().join("cmdline")) {
            exist
        } else {
            continue
        };

        let Some(bytes) = cmdline.split(|&byte| byte == 0).next() else {
            continue;
        };
        let arg_one = if let Ok(normal) = str::from_utf8(bytes) {
            normal
        } else {
            continue
        };
        let basename = arg_one.rsplit('/').next().unwrap();

        if basename == name {
            return Ok(Some(pid));
        }
    }

    Ok(None)
}

pub fn kill(pid: i32) -> anyhow::Result<()> {
    let return_type = unsafe {libc::kill(pid, libc::SIGKILL)};
    if return_type == 0 {
        Ok(())
    } else {
        log_e(&format!("kill调用失败: {}", Error::last_os_error()));
        Err(anyhow!("{}|{}", pid, Error::last_os_error()))
    }
}

fn result_process(result: &process::Output, is_err: bool) -> (i32, String){
    let code = result.status.code().unwrap_or(-1);
    let source = if is_err {
        &result.stderr
    } else {
        &result.stdout 
    };
    let output = String::from_utf8_lossy(source).trim().to_string();
    (code, output)
}

fn intercept_log_err(result: &process::Output, is_err: bool) {
    let (code, output) = result_process(result, is_err);
    log_e(&format!("{}|{}", code, output));
}

fn pass_through_err(result: &process::Output, is_err: bool) -> anyhow::Error {
    let (code, output) = result_process(result, is_err);
    anyhow!("{}|{}", code, output)
}

fn intercept_log_and_pass_through_err(result: Result<process::Output>, command: &str, is_err: bool) -> anyhow::Result<process::Output> {
    match result {
        Ok(success) => {
            if success.status.success() {
                Ok(success)
            } else {
                intercept_log_err(&success, is_err);
                Err(pass_through_err(&success, is_err))
            }
        }
        Err(error) => {
            log_e(&format!("{}执行失败: {}", command, error));
            Err(error.into())
        }
    }
}

pub fn resetprop(args: &[&str]) -> anyhow::Result<()> {
    let result = process::Command::new("resetprop").args(args)
        .output();
    intercept_log_and_pass_through_err(result, "resetprop", true)?;

    Ok(())
}

pub fn getprop(prop: &str) -> anyhow::Result<String> {
    let result = process::Command::new("getprop").arg(prop)
        .output();
    Ok(String::from_utf8(intercept_log_and_pass_through_err(result, "getprop", false)?.stdout).unwrap().trim().to_string())
}

pub fn am_start(args: &[&str]) -> anyhow::Result<()> {
    let am_result = process::Command::new("am").arg("start").arg("-n").args(args)
        .output()?;
    if am_result.status.success() {
        Ok(())
    } else {
        Err(pass_through_err(&am_result, false))
    }
}

fn pm(args: &[&str]) -> Result<process::Output> {
    process::Command::new("pm").args(args).output()
}

pub fn pm_install(arg: &str) -> bool {
    match pm(&["install", arg]) {
        Ok(result) => {
            if result.status.success() {
                true
            } else {
                log_e("安装失败");
                intercept_log_err(&result, false);
                false
            }
        }
        Err(error) => {
            log_e(&format!("安装失败: {}", error));
            false
        }
    }
}

pub fn pm_uninstall(arg: &str) {
    match pm(&["uninstall", arg]) {
        Ok(result) => {
            if !result.status.success() {
                log_e("卸载失败");
                intercept_log_err(&result, false);
            }
        }
        Err(error) => log_e(&format!("卸载失败: {}", error))
    }
}

pub fn pm_list(arg: &str) -> anyhow::Result<String> {
    let result = pm(&["list", "package", arg]);
    Ok(String::from_utf8(intercept_log_and_pass_through_err(result, "pm list", false)?.stdout).unwrap().trim().to_string())
}

pub fn pm_path(arg: &str, crash: bool) -> anyhow::Result<bool> {
    match pm(&["path", arg]) {
        Ok(success) => match success.status.code() {
            Some(0) => Ok(true),
            Some(1) => Ok(false),
            _ => if crash {
                Err(pass_through_err(&success, false))
            } else {
                intercept_log_err(&success, false);
                Ok(false)
            }
        },
        Err(error) => if crash {
            Err(error.into())
        } else {
            log_e(&format!("pm path执行失败: {}", error));
            Ok(false)
        }
    }
}

pub fn read_to_string(path: &Path) -> anyhow::Result<String> {
    fs::read_to_string(path).map_err(|error|{
        log_e(&format!("读取失败: {}", error));
        error.into()
    })
}

pub fn read_multiple_bool(env_file: &str) -> bool {
    read_to_string(Path::new(env_file)).ok().is_some_and(|content|
        content.lines().nth(0).is_some_and(|line|
            line.trim().parse::<bool>().unwrap_or(false)
        )
    )
}
pub fn read_identity_string(env_file: &str) -> String {
    read_to_string(Path::new(env_file)).ok().and_then(|content|
        content.lines().nth(1).map(|line|
            line.trim().to_string()
        )
    ).unwrap_or_default()
}
pub fn read_version_integer(env_file: &str) -> u16 {
    read_to_string(Path::new(env_file)).ok().and_then(|content|
        content.lines().nth(2).map(|line|
            line.trim().parse::<u16>().unwrap_or(32767)
        )
    ).unwrap_or(65535)
}

pub fn override_description(path: &str, description: &str) {
    let full_path = format!("{}/module.prop", path);
    let file = Path::new(&full_path);
    if let Ok(content) = read_to_string(file) {
        if content.contains("description=") {
            let data: String = content.lines().map(|line| {
                if line.starts_with("description=") {
                    description
                } else {
                    line
                }
            }).intersperse("\n").collect();
            if let Err(e) = fs::write(&full_path, data) {
                log_e(&format!("写入失败: {}", e))
            }
        } else {
            log_e("文件损坏")
        }
    }
}

pub fn delete_file(path: &str) {
    fs::remove_file(path).ok();
}

pub fn print_cn(msg: &str) {
    if *IS_ZHCN {
        print!("{}", msg);
    }
}
pub fn print_en(msg: &str) {
    if !*IS_ZHCN {
        print!("{}", msg);
    }
}
pub fn println_cn(msg: &str) {
    if *IS_ZHCN {
        println!("{}", msg);
    }
}
pub fn println_en(msg: &str) {
    if !*IS_ZHCN {
        println!("{}", msg);
    }
}