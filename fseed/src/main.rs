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

use std::sync::OnceLock;
use libloading::Library;

struct Functions {
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
    let verify_functions_load = |function_name: &str| -> unsafe fn() -> bool {
        match unsafe {lib_instance.get::<unsafe fn() -> bool>(function_name.as_bytes())} {
            Ok(pointer) => *pointer,
            Err(_) => {
                panic!("加载失败:libutils.so不存在验证函数");
            }
        }
    };
    let functions = Functions {
        verify: verify_functions_load("verify_bridge")
    };
    FUNCTIONS.set(functions).ok();
    std::mem::forget(lib_instance);
}

fn verify() -> bool {
    unsafe {(FUNCTIONS.get().unwrap().verify)()}
}

fn main() {
    //函数导入
    init_lib();
    //验证
    if !verify() {
        eprintln!("拒绝执行:文件被篡改!");
        unsafe {*(0xDEADBEEF as *mut u8) = 0}
    }
    println!("Pass!");
}