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

use std::process;

pub fn invoke() -> Option<u32> {
    process::Command::new("apd").arg("-V")
        .output().ok()
        .and_then(|output|
            String::from_utf8(output.stdout).ok()
        ).and_then(|version_str|
            version_str.split_whitespace().nth(1)?.parse().ok()
        )
}