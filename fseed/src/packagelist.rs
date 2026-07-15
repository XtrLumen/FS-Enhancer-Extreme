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
        FSEECONFIG,
        ENV_NORMAL,
        FINAL_MAIN_MODULE_CONFIG
    },
    util_functions::{
        pm_list,
        read_to_string
    }
};

use std::{
    fs,
    path::Path,
    collections::HashSet
};

pub fn refresh() -> anyhow::Result<()> {
    if *ENV_NORMAL {
        let result = pm_list("-3")?;
    
        let packages: HashSet<&str> = result.lines().filter_map(|line|
            line.strip_prefix("package:")
        ).collect();
    
        let usr_full_path = format!("{}/usr.txt", FSEECONFIG);
        let usr_file = Path::new(&usr_full_path);
        let usr: HashSet<String> = if let Ok(file) = read_to_string(usr_file) {
            file.lines().map(|content|
                content.to_string()
            ).collect()
        } else {
            HashSet::new()
        };
    
        let mut output = String::new();
        for pkg in packages.iter().filter(|package|
            !usr.contains(**package)
        ) {
            output.push_str(pkg);
            output.push('\n');
        }
    
        let sys_full_path = format!("{}/sys.txt", FSEECONFIG);
        let sys_file = Path::new(&sys_full_path);
        if let Ok(sys) = read_to_string(sys_file) {
            output.push_str(&sys);
        }
    
        let target = format!("{}/target.txt", *FINAL_MAIN_MODULE_CONFIG);
        fs::write(&target, output)?;
    } else {
        println!("Environment abnormal")
    }

    Ok(())
}