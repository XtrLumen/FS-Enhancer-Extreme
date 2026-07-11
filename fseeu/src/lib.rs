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

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::fs::OpenOptions;
use ed25519_compact::{PublicKey, Signature};

fn verify() -> bool {
    let pwd = Path::new("/data/adb/modules/fs_enhancer_extreme");
    
    //创建文件列表
    let action = if pwd.join(".action.sh").exists() {
        ".action.sh"
    } else {
        "action.sh"
    };
    let lists = [
        "bin/cmd",
        "bin/fseed",
        "bin/fsees",
        "lib/libutils.so",
        "script/state.sh",
        "script/util_functions.sh",
        "banner.png",
        "post-fs-data.sh",
        "provider.apk",
        "service.sh",
        "uninstall.sh",
        action
    ];

    //计算哈希拼接
    let mut blake3hash = String::new();
    for file in lists.iter() {
        let path = pwd.join(file);

        let mut hasher = blake3::Hasher::new();
        let mut file = match File::open(&path) {
            Ok(f) => f,
            Err(_) => return false,
        };
        let mut buffer = [0u8; 4096];

        loop {
            let size = match file.read(&mut buffer) {
                Ok(s) => s,
                Err(_) => return false,
            };
            if size == 0 {
                break;
            }
            hasher.update(&buffer[..size]);
        }

        blake3hash.push_str(&hex::encode(hasher.finalize().as_bytes()));
    }

    //读取集合文件
    let ml_bytes = match std::fs::read(pwd.join("mistylake")) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };

    //拼接签名
    let mut sg_bytes = [0u8; 64];
    sg_bytes[0..16].copy_from_slice(&ml_bytes[0..16]);
    sg_bytes[16..48].copy_from_slice(&ml_bytes[32..64]);
    sg_bytes[48..64].copy_from_slice(&ml_bytes[80..96]);

    //拼接公钥
    let mut pb_bytes = [0u8; 32];
    pb_bytes[0..16].copy_from_slice(&ml_bytes[16..32]);
    pb_bytes[16..32].copy_from_slice(&ml_bytes[64..80]);

    //使用签名和公钥验证与拼接哈希是否匹配
    PublicKey::new(pb_bytes).verify(&blake3hash, &Signature::new(sg_bytes)).is_ok()
}

fn log(level: char, tag: &str, msg: &str) {
    let (timestamp, pid, tid) = unsafe {
        //创建时间结构体
        let mut ts: libc::timespec = std::mem::zeroed();
        let mut tm: libc::tm = std::mem::zeroed();
        //赋值时间结构体
        libc::clock_gettime(libc::CLOCK_REALTIME, &mut ts);
        //时间格式转换
        libc::localtime_r(&ts.tv_sec, &mut tm);
        //时间格式分割
        let finaltime = format!("{:02}-{:02} {:02}:{:02}:{:02}.{:03}", tm.tm_mon + 1, tm.tm_mday, tm.tm_hour, tm.tm_min, tm.tm_sec, ts.tv_nsec / 1_000_000);
        (finaltime, libc::getpid(), libc::gettid())
    };
    OpenOptions::new().create(true).append(true).open("/data/adb/fs_enhancer_extreme/log/log.log").and_then(|mut content|
        content.write_all(
            format!("{}  {}  {} {} [FSEE]: <{}> {}\n", timestamp, pid, tid, level, tag, msg).as_bytes()
        )
    ).ok();
}

#[unsafe(no_mangle)]
pub fn log_i_bridge(tag: &str, msg: &str) {
    log('I', tag, msg)
}
#[unsafe(no_mangle)]
pub fn log_w_bridge(tag: &str, msg: &str) {
    log('W', tag, msg)
}
#[unsafe(no_mangle)]
pub fn log_e_bridge(tag: &str, msg: &str) {
    log('E', tag, msg)
}
#[unsafe(no_mangle)]
pub fn log_d_bridge(tag: &str, msg: &str) {
    log('D', tag, msg)
}
#[unsafe(no_mangle)]
pub fn verify_bridge() -> bool {
    verify()
}