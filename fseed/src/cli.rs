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
        VERSION_NAME,
        ENV_NORMAL
    },
    util_functions::{
        switch_mnt_namespace
    },
    envcollect,
    conflict,
    ctl,
    description,
    packagelist,
    passprop,
    passvbhash,
    securitypatch,
    webui
};

use std::process;

use clap::{
    Parser,
    Subcommand
};

/// FS Enhancer Extreme cli
#[derive(Parser)]
#[command(version = VERSION_NAME)]
enum Commands {
    /// Operation Forge Store service
    Fsctl {
        #[command(subcommand)]
        command: Ctl,
    },
    /// Operation FS Enhancer Extreme service
    Fseectl {
        #[command(subcommand)]
        command: Ctl,
    },
    /// Check running environment if normal from envcollect cache
    Envcheck,
    /// Check and directly uninstall conflict apps
    Appcheck,
    /// Check and add remove tag or force delete conflict modules
    Modcheck(conflict::Mode),
    /// Through Bootloader unlock related prop detection
    Passprop,
    /// Automatically correct abnormal VerifiedBootHash prop
    Passvbhash,
    /// Launch standalone WebUI app to id fs_enhancer_extreme
    Startwebui,
    /// Sync Security Patch Level from security_patch.txt to prop
    Spsyncprop,
    /// Detect and cache all necessity runtime environments
    Envcollect,
    /// Refresh module decription line from envcollect cache
    Descrefresh(description::Mode),
    /// Refresh Forge Store target.txt from user config
    Listrefresh
}

#[derive(Subcommand)]
enum Ctl {
    /// Restart service process
    Restart,
    /// Start   service process
    Start,
    /// Stop    service process
    Stop,
    /// Get     service running status
    State,
}

pub fn entry() -> anyhow::Result<()> {
    switch_mnt_namespace()?;
    match Commands::parse() {
        Commands::Fsctl {command} => {
            match command {
                Ctl::Restart => ctl::fs_restart(),
                Ctl::Start => ctl::fs_start(),
                Ctl::Stop => ctl::fs_stop(),
                Ctl::State => ctl::fs_state()
            }
        }
        Commands::Fseectl {command} => {
            match command {
                Ctl::Restart => ctl::fsee_restart(),
                Ctl::Start => ctl::fsee_start(),
                Ctl::Stop => ctl::fsee_stop(),
                Ctl::State => ctl::fsee_state()
            }
        }
        Commands::Envcheck => {
            if !*ENV_NORMAL {
                process::exit(1)
            }

            Ok(())
        },
        Commands::Appcheck => conflict::app_process(),
        Commands::Modcheck(conflict_args) => {
            conflict::route(conflict_args);

            Ok(())
        },
        Commands::Passprop => passprop::entry(),
        Commands::Passvbhash => passvbhash::entry(),
        Commands::Startwebui => webui::start(),
        Commands::Spsyncprop => securitypatch::sync(),
        Commands::Envcollect => {
            envcollect::external_entry();

            Ok(())
        },
        Commands::Descrefresh(conflict_args) => description::refresh(conflict_args),
        Commands::Listrefresh => packagelist::refresh()
    }
}