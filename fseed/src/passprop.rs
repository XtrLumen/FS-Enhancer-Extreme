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
    define::BLKSSZGET,
    util_functions::{
        resetprop,
        getprop
    }
};

use std::ffi::CString;

pub fn entry() -> anyhow::Result<()> {
    let check_missing_match_prop = |prop: &[&str]| -> anyhow::Result<()> {
        let value: String = getprop(prop[0])?;
        if value.is_empty() || value != prop[1] {
            resetprop(prop)?;
        }

        Ok(())
    };

    let contains_reset_prop = |prop: &[&str]| -> anyhow::Result<()> {
        if getprop(prop[0])?.contains(prop[1]) {
            resetprop(&[prop[0], prop[2]])?;
        }

        Ok(())
    };

    let check_missing_prop = |prop: &[&str]| -> anyhow::Result<()> {
        if getprop(prop[0])?.is_empty() {
            resetprop(prop)?;
        }

        Ok(())
    };

    let check_reset_prop = |prop: &[&str]| -> anyhow::Result<()> {
        let value: String = getprop(prop[0])?;
        if !value.is_empty() && value != prop[1] {
            resetprop(prop)?;
        }

        Ok(())
    };

    // check_missing_match_prop
    for arg in [
        ["ro.boot.vbmeta.device_state", "locked"],
        ["ro.boot.verifiedbootstate", "green"],
        ["ro.boot.veritymode", "enforcing"],
        ["ro.boot.warranty_bit", "0"],
        ["ro.boot.flash.locked", "1"],
    ] {
        check_missing_match_prop(&arg)?;
    }

    // contains_reset_prop
    for arg in [
        ["vendor.boot.bootmode", "recovery", "unknown"],
        ["ro.boot.bootmode", "recovery", "unknown"],
        ["ro.bootmode", "recovery", "unknown"]
    ] {
        contains_reset_prop(&arg)?;
    }

    // check_missing_prop
    let vbmeta_size = {
        let default = String::from("4096");
        let slot = getprop("ro.boot.slot_suffix")?;
        let path = CString::new(format!("/dev/block/by-name/vbmeta{}", slot)).unwrap();
        unsafe {
            let file_descriptor: libc::c_int = libc::open(path.as_ptr(), libc::O_RDONLY);
            if file_descriptor >= 0 {
                let mut size: libc::c_int = 0;
                let return_type: libc::c_int = libc::ioctl(file_descriptor, BLKSSZGET, &mut size);
                libc::close(file_descriptor);
                if return_type == 0 {
                    size.to_string()
                } else {
                    default
                }
            } else {
                default
            }
        }
    };
    #[cfg(debug_assertions)]
    crate::bridge::log_d(&format!("vbmeta_size: {}", vbmeta_size));

    for arg in [
        ["ro.boot.vbmeta.size", &vbmeta_size],
        ["ro.boot.vbmeta.hash_alg", "sha256"],
        ["ro.boot.vbmeta.avb_version", "1.2"]
    ] {
        check_missing_prop(&arg)?;
    }

    // check_reset_prop
    for arg in [
        ["vendor.boot.vbmeta.device_state", "locked"],
        ["vendor.boot.verifiedbootstate", "green"],

        ["sys.oem_unlock_allowed", "0"],

        ["ro.vendor.boot.warranty_bit", "0"],
        ["ro.build.tags", "release-keys"],
        ["ro.crypto.state", "encrypted"],
        ["ro.vendor.warranty_bit", "0"],
        ["ro.force.debuggable", "0"],
        ["ro.build.type", "user"],
        ["ro.warranty_bit", "0"],
        ["ro.debuggable", "0"],
        ["ro.secure", "1"],

        ["ro.secureboot.lockstate", "locked"],
        ["ro.boot.realmebootstate", "green"],
        ["ro.boot.realme.lockstate", "1"]
    ] {
        check_reset_prop(&arg)?;
    }

    Ok(())
}