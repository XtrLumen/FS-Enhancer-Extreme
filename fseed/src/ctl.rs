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

use crate::{
    define::{
        FSEEMODDIR,
        MAIN_MODULE_IDENTITY,
        ENV_NORMAL,
        FINAL_NICE_NAME,
        FINAL_MAIN_MODULE_DIR
    },
    util_functions::{
        pidof,
        kill
    }
};

use std::process;

pub fn fs_state() -> anyhow::Result<()> {
    if *ENV_NORMAL {
        if let Some(pid) = pidof(*FINAL_NICE_NAME)? {
            println!("running|{}", pid)
        } else {
            println!("not running")
        }
    } else {
        println!("Environment abnormal")
    }

    Ok(())
}

pub fn fs_stop() -> anyhow::Result<()> {
    if *ENV_NORMAL {
        if *MAIN_MODULE_IDENTITY != "TEESimulatorRS" {
            if let Some(pid) = pidof(*FINAL_NICE_NAME)? {
                kill(pid)?;
                println!("{}|stopped", pid)
            } else {
                println!("not running")
            }
        } else {
            println!("Not support {}", *MAIN_MODULE_IDENTITY)
        }
    } else {
        println!("Environment abnormal")
    }

    Ok(())
}

pub fn fs_start() -> anyhow::Result<()> {
    if *ENV_NORMAL {
        if *MAIN_MODULE_IDENTITY != "TEESimulatorRS" {
            process::Command::new("sh").current_dir(*FINAL_MAIN_MODULE_DIR).arg("./service.sh")
                .stdin(process::Stdio::null())
                .stdout(process::Stdio::null())
                .stderr(process::Stdio::null())
                .spawn()?;
            if let Some(pid) = pidof(*FINAL_NICE_NAME)? {
                println!("success|{}", pid)
            } else {
                println!("failure");
                process::exit(1)
            }
        } else {
            println!("Not support {}", *MAIN_MODULE_IDENTITY)
        }
    } else {
        println!("Environment abnormal")
    }

    Ok(())
}

pub fn fs_restart() -> anyhow::Result<()> {
    if let Some(pid) = pidof(*FINAL_NICE_NAME)? {
        kill(pid)?;
        fs_start()?
    } else {
        println!("not running")
    }

    Ok(())
}

pub fn fsee_state() -> anyhow::Result<()> {
    if let Some(pid) = pidof("fsees")? {
        println!("running|{}", pid)
    } else {
        println!("not running")
    }

    Ok(())
}

pub fn fsee_stop() -> anyhow::Result<()> {
    if let Some(pid) = pidof("fsees")? {
        kill(pid)?;
        println!("{}|stopped", pid)
    } else {
        println!("not running")
    }

    Ok(())
}

pub fn fsee_start() -> anyhow::Result<()> {
    process::Command::new(format!("{}/bin/fsees", FSEEMODDIR))
        .stdin(process::Stdio::null())
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .spawn()?;
    if let Some(pid) = pidof("fsees")? {
        println!("success|{}", pid)
    } else {
        println!("failure");
        process::exit(1)
    }

    Ok(())
}

pub fn fsee_restart() -> anyhow::Result<()> {
    if let Some(pid) = pidof("fsees")? {
        kill(pid)?;
        fsee_start()?
    } else {
        println!("not running")
    }

    Ok(())
}