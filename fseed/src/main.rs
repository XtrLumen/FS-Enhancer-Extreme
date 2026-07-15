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

#![feature(iter_intersperse)]

mod envcollect;
mod define;
mod bridge;
mod cli;
mod conflict;
mod ctl;
mod description;
mod packagelist;
mod passprop;
mod passvbhash;
mod securitypatch;
mod util_functions;
mod webui;

use crate::{
    bridge::{
        init_bridge,
        verify
    },
    cli::entry
};

fn main() -> anyhow::Result<()> {
    init_bridge();
    verify();
    entry()
}