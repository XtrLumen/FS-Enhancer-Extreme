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

use crate::define::FSEELOG;

use std::{
    mem,
    io::Write,
    fs::OpenOptions
};

pub fn log(level: char, tag: &str, msg: &str) {
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

pub fn log_raw(raw: &str) {
    OpenOptions::new().create(true).append(true).open(FSEELOG).and_then(|mut content|
        content.write_all(
            format!("{}\n", raw).as_bytes()
        )
    ).ok();
}