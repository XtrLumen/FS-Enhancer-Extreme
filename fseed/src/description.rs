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
        FS_STR,
        FSEEMODDIR,
        ROOT_IMPL_ENV_FILE,
        MAIN_MODULE_ENV_FILE,
        MAIN_MODULE_IDENTITY,
        ENV_NORMAL,
        DESC_BASE,
        DESC_SERVICE_SUCCESS,
        DESC_SERVICE_FAILURE,
        DESC_SERVICE_NOT_START,
        DESC_MULTIPLE,
        DESC_MAIN_MODULE_NOT_INSTALL,
        DESC_DISABLE,
        DESC_ROOT_IMPL,
        DESC_MAIN_MODULE
    },
    util_functions::{
        pidof,
        read_multiple_bool,
        read_identity_string,
        read_version_integer,
        override_description
    },
    envcollect::internal_entry
};

use std::path::Path;

use clap::Args;

#[derive(Args)]
pub struct Mode {
    /// Force recollect environment
    #[arg(short, long)]
    force: bool
}

pub fn refresh(mode: Mode) -> anyhow::Result<()> {
    if mode.force {
        internal_entry()
    }

    let full_environment: String = if !Path::new(&format!("{}/disable", FSEEMODDIR)).exists() {
        let root_impl_identity = read_identity_string(ROOT_IMPL_ENV_FILE);
        let (root_impl_prefix, root_impl_environment) = if read_multiple_bool(ROOT_IMPL_ENV_FILE) {
            (*DESC_MULTIPLE, root_impl_identity)
        } else {
            if root_impl_identity == "Unknown" {
                ("⚠️", root_impl_identity)
            } else {
                ("✅", format!("{}({})", root_impl_identity, read_version_integer(ROOT_IMPL_ENV_FILE)))
            }
        };

        let main_module_identity = read_identity_string(MAIN_MODULE_ENV_FILE);
        let (main_module_prefix, main_module_environment) = if read_multiple_bool(MAIN_MODULE_ENV_FILE) {
            if *MAIN_MODULE_IDENTITY == "MULTIPLE" {
                (*DESC_MULTIPLE, main_module_identity)
            } else {
                if *MAIN_MODULE_IDENTITY == "OFF" {
                    ("❌", DESC_DISABLE.to_string())
                } else {
                    if *MAIN_MODULE_IDENTITY == FS_STR {
                        ("✅", main_module_identity
                            .split('|').next().unwrap()
                            .to_string())
                    } else {
                        ("✅", main_module_identity
                            .rsplit('|').next().unwrap()
                            .to_string())
                    }
                }
            }
        } else {
            if *MAIN_MODULE_IDENTITY == "Unknown" {
                ("❌", DESC_MAIN_MODULE_NOT_INSTALL.to_string())
            } else {
                ("✅", format!("{}({})", main_module_identity, read_version_integer(MAIN_MODULE_ENV_FILE)))
            }
        };

        let service_state = if *ENV_NORMAL {
            if let Some(_) = pidof("fsees")? {
                *DESC_SERVICE_SUCCESS
            } else {
                *DESC_SERVICE_FAILURE
            }
        } else {
            *DESC_SERVICE_NOT_START
        };

        format!("{}{}{}, {}{}{}, {}", *DESC_ROOT_IMPL, root_impl_prefix, root_impl_environment, *DESC_MAIN_MODULE, main_module_prefix, main_module_environment, service_state)
    } else {
        format!("❌{}", DESC_DISABLE.to_string())
    };

    let final_description = format!("description=[{}]\\n{}", full_environment, *DESC_BASE);
    override_description(FSEEMODDIR, &final_description);
    println!("{}", final_description);

    Ok(())
}