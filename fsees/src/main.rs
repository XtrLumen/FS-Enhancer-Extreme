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
 * Copyright (C) 2025-2026 XtrLumen
 */

use std::thread;
use std::process;
use std::path::Path;
use std::ffi::CString;
use std::sync::OnceLock;
use libloading::Library;

struct Functions {
    log_i: unsafe fn(&str, &str),
    log_w: unsafe fn(&str, &str),
    log_e: unsafe fn(&str, &str),
    log_d: unsafe fn(&str, &str),
    verify: unsafe fn() -> bool
}

static FUNCTIONS: OnceLock<Functions> = OnceLock::new();

fn init_lib() {
    let lib_instance = match unsafe {Library::new("/data/adb/modules/fs_enhancer_extreme/lib/libutils.so")} {
        Ok(success) => success,
        Err(_) => {
            panic!("加载libutils.so失败");
        }
    };
    let log_functions_load = |function_name: &str| -> unsafe fn(&str, &str) {
        match unsafe {lib_instance.get::<unsafe fn(&str, &str)>(function_name.as_bytes())} {
            Ok(pointer) => *pointer,
            Err(_) => {
                panic!("加载失败:libutils.so不存在日志函数");
            }
        }
    };
    let verify_functions_load = |function_name: &str| -> unsafe fn() -> bool {
        match unsafe {lib_instance.get::<unsafe fn() -> bool>(function_name.as_bytes())} {
            Ok(pointer) => *pointer,
            Err(_) => {
                panic!("加载失败:libutils.so不存在验证函数");
            }
        }
    };
    let functions = Functions {
        log_i: log_functions_load("log_i_bridge"),
        log_w: log_functions_load("log_w_bridge"),
        log_e: log_functions_load("log_e_bridge"),
        log_d: log_functions_load("log_d_bridge"),
        verify: verify_functions_load("verify_bridge")
    };
    FUNCTIONS.set(functions).ok();
    std::mem::forget(lib_instance);
}


const TAG: &str = "daemon";
fn log_i(msg: &str) {
    unsafe {(FUNCTIONS.get().unwrap().log_i)(TAG, msg)}
}
fn log_w(msg: &str) {
    unsafe {(FUNCTIONS.get().unwrap().log_w)(TAG, msg)}
}
fn log_e(msg: &str) {
    unsafe {(FUNCTIONS.get().unwrap().log_e)(TAG, msg)}
}
#[allow(unused)]
fn log_d(msg: &str) {
    unsafe {(FUNCTIONS.get().unwrap().log_d)(TAG, msg)}
}

fn verify() -> bool {
    unsafe {(FUNCTIONS.get().unwrap().verify)()}
}

fn watch(path: &str, args: &[&[&str]], events: u32, tx: std::sync::mpsc::Sender<bool>) {
    if !Path::new(path).exists() {
        log_e(&format!("目录{}不存在,结束线程", path));
        //发送状态
        tx.send(false).ok();
        return;
    }
    //创建实例
    let instance = unsafe {libc::inotify_init()};
    if instance < 0 {
        log_e("实例创建失败");
        //发送状态
        tx.send(false).ok();
        return;
    }
    //添加监听
    let watch = unsafe {
        let cstring = CString::new(path).unwrap();
        libc::inotify_add_watch(instance, cstring.as_ptr(), events)
    };
    if watch < 0 {
        log_e("监听添加失败");
        //发送状态
        tx.send(false).ok();
        unsafe { libc::close(instance); }
        return;
    }
    log_i("线程就绪");
    //发送状态
    tx.send(true).ok();
    //创建缓冲区
    let mut buffer = [0u8; 1024];
    //日志速度限制
    let mut last = std::time::Instant::now();
    let speed = std::time::Duration::from_millis(1000);
    //循环开始等待
    loop {
        //阻塞
        unsafe {libc::read(
            instance, buffer.as_mut_ptr() as *mut libc::c_void, buffer.len()
        )};
        if last.elapsed() >= speed {
            let all_args: Vec<String> = args.iter().map(|content|
                content.join(" ")
            ).collect();
            log_i(&format!("触发执行fseed {}", all_args.join(" | ")));
            last = std::time::Instant::now();
        }
        //执行
        for arg in args {
            process::Command::new("/data/adb/modules/fs_enhancer_extreme/bin/fseed").args(*arg).status().ok();
        }
    }
}

fn main() {
    //函数导入
    init_lib();
    //验证
    if verify() {
        log_i("开始启动线程");
    } else {
        log_e("拒绝启动:文件被篡改!");
        unsafe {*(0xDEADBEEF as *mut u8) = 0}
    }
    //创建通道
    let (tx1, rx1) = std::sync::mpsc::channel();
    let (tx2, rx2) = std::sync::mpsc::channel();
    //启动线程
    thread::spawn(move || {
        watch(
            "/data/adb/modules_update",
            &[&["--conflictmodcheck", "-s"]],
            libc::IN_CREATE | libc::IN_ISDIR,
            tx1
        );
    });
    thread::spawn(move || {
        watch(
            "/data/app",
            &[&["--conflictappcheck"],
            &["--packagelistupdate"]],
            libc::IN_CREATE | libc::IN_DELETE,
            tx2
        );
    });
    //接收状态
    let res1 = rx1.recv().unwrap();
    let res2 = rx2.recv().unwrap();
    if res1 && res2 {
        log_i("成功启动服务");
    } else if res1 || res2 {
        log_w("线程部分就绪");
    } else {
        log_e("服务启动失败");
        process::abort();
    }
    //挂起
    thread::park();
}