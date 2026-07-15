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

use crate::define::LOG_TAG;

use libloading::Library;

use std::{
    mem,
    sync::OnceLock
};

struct Functions {
    verify: unsafe fn(),
    log_i: unsafe fn(&str, &str),
    log_w: unsafe fn(&str, &str),
    log_e: unsafe fn(&str, &str),
    log_d: unsafe fn(&str, &str),
    log_raw: unsafe fn(&str)
}

static FUNCTIONS: OnceLock<Functions> = OnceLock::new();

pub fn init_bridge() {
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
    let str_void_load = |function_name: &str| -> unsafe fn(&str) {unsafe{
        *lib_instance.get::<unsafe fn(&str)>(function_name.as_bytes())
            .expect(expect_msg)
    }};
    let functions = Functions {
        verify: void_void_load("verify_bridge"),
        log_i: str_str_void_load("log_i_bridge"),
        log_w: str_str_void_load("log_w_bridge"),
        log_e: str_str_void_load("log_e_bridge"),
        log_d: str_str_void_load("log_d_bridge"),
        log_raw: str_void_load("log_raw_bridge")
    };
    FUNCTIONS.set(functions).ok();
    mem::forget(lib_instance);
}

pub fn verify() {
    unsafe {(FUNCTIONS.get().unwrap().verify)()}
}

pub fn log_i(msg: &str) {
    unsafe {(FUNCTIONS.get().unwrap().log_i)(LOG_TAG, msg)}
}
pub fn log_w(msg: &str) {
    unsafe {(FUNCTIONS.get().unwrap().log_w)(LOG_TAG, msg)}
}
pub fn log_e(msg: &str) {
    unsafe {(FUNCTIONS.get().unwrap().log_e)(LOG_TAG, msg)}
}
#[allow(unused)]
#[inline(always)]
pub fn log_d(msg: &str) {
    unsafe {(FUNCTIONS.get().unwrap().log_d)(LOG_TAG, msg)}
}
pub fn log_raw(msg: &str) {
    unsafe {(FUNCTIONS.get().unwrap().log_raw)(msg)}
}