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

use libloading::Library;

use std::{
    fs,
    mem,
    thread,
    process,
    path::Path,
    ffi::CString,
    sync::{
        mpsc,
        OnceLock
    },
    time::{
        Instant,
        Duration
    }
};

struct Functions {
    verify: unsafe fn(),
    log_i: unsafe fn(&str, &str),
    log_w: unsafe fn(&str, &str),
    log_e: unsafe fn(&str, &str)
}

static FUNCTIONS: OnceLock<Functions> = OnceLock::new();

pub fn load_bridge() {
    let lib_instance = unsafe {Library::new("/data/adb/modules/fs_enhancer_extreme/lib/libutils.so")
        .expect("libutils.so加载失败")
    };
    let expect_msg: &str = "libutils.so符号缺失";
    let void_void_load = |function_name: &str| -> unsafe fn() {unsafe{
        *lib_instance.get::<unsafe fn()>(function_name.as_bytes())
            .expect(expect_msg)
    }};
    let str_str_void_load = |function_name: &str| -> unsafe fn(&str, &str) {unsafe{
        *lib_instance.get::<unsafe fn(&str, &str)>(function_name.as_bytes())
            .expect(expect_msg)
    }};
    let functions = Functions {
        verify: void_void_load("verify_bridge"),
        log_i: str_str_void_load("log_i_bridge"),
        log_w: str_str_void_load("log_w_bridge"),
        log_e: str_str_void_load("log_e_bridge")
    };
    FUNCTIONS.set(functions).ok();
    mem::forget(lib_instance);
}

fn verify() {
    unsafe {(FUNCTIONS.get().unwrap().verify)()}
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

fn watch(path: &str, args: &[&[&str]], events: u32, tx: mpsc::Sender<bool>) {
    if !Path::new(path).exists() {
        log_w(&format!("目录{}不存在,尝试创建", path));
        if let Err(error) = fs::create_dir_all(path) {
            log_e(&format!("目录{}创建失败: {},结束线程", path, error));
            //发送状态
            tx.send(false).ok();
            return;
        }
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
        unsafe {libc::close(instance);}
        return;
    }
    log_i("线程就绪");
    //发送状态
    tx.send(true).ok();
    //创建缓冲区
    let mut buffer = [0u8; 1024];
    //日志速度限制
    let mut last = Instant::now();
    let speed = Duration::from_millis(1000);
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
            last = Instant::now();
        }
        //执行
        for arg in args {
            process::Command::new("/data/adb/modules/fs_enhancer_extreme/bin/fseed").args(*arg)
                .status().ok();
        }
    }
}

fn main() {
    //函数导入
    load_bridge();
    //验证
    verify();
    log_i("开始启动线程");
    //创建通道
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    //启动线程
    thread::spawn(move || {
        watch(
            "/data/adb/modules_update",
            &[
                &["modcheck", "--daemon"]
            ],
            libc::IN_CREATE | libc::IN_ISDIR,
            tx1
        );
    });
    thread::spawn(move || {
        watch(
            "/data/app",
            &[
                &["listupdate"],
                &["appcheck"]
            ],
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