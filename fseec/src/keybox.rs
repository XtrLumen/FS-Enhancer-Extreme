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

use crate::define::{
    BUILTIN_KEYBOX,
    FINAL_MAIN_MODULE_CONFIG
};

use std::{
    fs,
    path::Path
};

fn backup(path: &Path) -> anyhow::Result<()> {
    if path.join("keybox.xml").exists() {
        fs::rename(path.join("keybox.xml"), path.join("keybox.xml.bak"))?;
    }

    Ok(())
}

fn restore(path: &Path) -> anyhow::Result<()> {
    if path.join("keybox.xml.bak").exists() {
        fs::rename(path.join("keybox.xml.bak"), path.join("keybox.xml"))?;
    }

    Ok(())
}

pub fn extract() -> anyhow::Result<()> {
    let base_path = Path::new(*FINAL_MAIN_MODULE_CONFIG);

    backup(base_path)?;
    if let Err(error) = fs::write(base_path.join("keybox.xml"), BUILTIN_KEYBOX) {
        restore(base_path)?;
        return Err(error.into())
    }

    Ok(())
}

pub fn transfer(keybox_path: String) -> anyhow::Result<()> {
    let base_path = Path::new(*FINAL_MAIN_MODULE_CONFIG);

    let data = fs::read(&keybox_path)?;

    backup(base_path)?;
    if let Err(error) = fs::write(base_path.join("keybox.xml"), data) {
        restore(base_path)?;
        return Err(error.into())
    }

    Ok(())
}