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

use std::{
    fs,
    mem,
    path::Path,
    fs::{
        File,
        OpenOptions
    },
    io::{
        Read,
        Write
    }
};

use blake3::Hasher;
use ed25519_compact::{
    PublicKey,
    Signature
};

const FSEELOG: &str = "/data/adb/fs_enhancer_extreme/log/log.log";

fn verify() -> Option<bool> {
    let base_path = Path::new("/data/adb/modules/fs_enhancer_extreme");

    let misty_bytes: Vec<u8> = if let Ok(data) = fs::read(base_path.join("mistylake")) {
        if data.is_empty() {
            return None
        } else {
            data
        }
    } else {
        return Some(false)
    };

    let action: &str = if base_path.join(".action.sh").exists() {
        ".action.sh"
    } else {
        "action.sh"
    };

    let files = [
        "bin/fseed",
        "bin/fsees",
        "lib/libutils.so",
        "script/state.sh",
        "script/util_functions.sh",
        action,
        "module.base",
        "post-fs-data.sh",
        "provider.apk",
        "service.sh",
        "uninstall.sh"
    ];

    let mut rebuild_checksum: String = String::with_capacity(files.len() * 64);

    for file in files {
        let mut file = if let Ok(exists_continue) = File::open(base_path.join(file)) {
            exists_continue
        } else {
            return Some(false)
        };

        let mut hasher = Hasher::new();
        let mut buffer = [0u8; 4096];
        loop {
            let size = match file.read(&mut buffer) {
                Ok(0) => break,
                Ok(success) => success,
                _ => return Some(false)
            };
            hasher.update(&buffer[..size]);
        }

        rebuild_checksum.push_str(
            &hasher.finalize().to_hex().to_string()
        )
    }

    let mut public_key_bytes = [0u8; 32];
    public_key_bytes[0..16].copy_from_slice(&misty_bytes[16..32]);
    public_key_bytes[16..32].copy_from_slice(&misty_bytes[64..80]);

    let mut sign_bytes = [0u8; 64];
    sign_bytes[0..16].copy_from_slice(&misty_bytes[0..16]);
    sign_bytes[16..48].copy_from_slice(&misty_bytes[32..64]);
    sign_bytes[48..64].copy_from_slice(&misty_bytes[80..96]);

    let confirmed_sign = Signature::new(sign_bytes);

    Some(PublicKey::new(public_key_bytes).verify(rebuild_checksum, &confirmed_sign).is_ok())
}

fn log(level: char, tag: &str, msg: &str) {
    let (timestamp, pid, tid) = unsafe {
        let mut ts: libc::timespec = mem::zeroed();
        libc::clock_gettime(libc::CLOCK_REALTIME, &mut ts);

        let mut tm: libc::tm = mem::zeroed();
        libc::localtime_r(&ts.tv_sec, &mut tm);

        let finaltime = format!("{:02}-{:02} {:02}:{:02}:{:02}.{:03}", tm.tm_mon + 1, tm.tm_mday, tm.tm_hour, tm.tm_min, tm.tm_sec, ts.tv_nsec / 1_000_000);
        (finaltime, libc::getpid(), libc::gettid())
    };
    OpenOptions::new().create(true).append(true).open(FSEELOG).and_then(|mut content|
        content.write_all(
            format!("{}  {}  {} {} [FSEE]  : <{}> {}\n", timestamp, pid, tid, level, tag, msg).as_bytes()
        )
    ).ok();
}

fn log_raw(raw: &str) {
    OpenOptions::new().create(true).append(true).open(FSEELOG).and_then(|mut content|
        content.write_all(
            format!("{}\n", raw).as_bytes()
        )
    ).ok();
}

#[unsafe(no_mangle)]
pub fn verify_bridge() -> Option<bool> {
    verify()
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
pub fn log_raw_bridge(msg: &str) {
    log_raw(msg);
}