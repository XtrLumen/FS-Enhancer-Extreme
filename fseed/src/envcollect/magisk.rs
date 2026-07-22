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

fn magisk(arg: &str) -> Option<String> {
    process::Command::new("magisk").arg(arg)
        .output().ok()
        .and_then(|output|
            String::from_utf8(output.stdout).ok()
        )
}

fn detect_variant() -> Option<&'static str> {
    let version_name = magisk("-v")?;

    if version_name.contains("alpha") {
        Some("Alpha")
    } else if version_name.contains("kitsune") {
        Some("Kitsune")
    } else {
        Some("Magisk")
    }
}

pub fn detect() -> Option<(&'static str, u32)> {
    let variant: &str = detect_variant()?;
    let version_code: u32 = magisk("-V")?.trim().parse().ok()?;

    Some((variant, version_code))
}