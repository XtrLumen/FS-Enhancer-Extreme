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

use crate::util_functions::{
    am_start,
    pm_path,
    print_cn,
    print_en,
    println_cn,
    println_en
};

pub fn start() -> anyhow::Result<()> {
    let launch = |app: &str, arg: &[&str]| -> anyhow::Result<()> {
        print_cn(&format!("- {}已安装,启动...", app));
        print_en(&format!("- {} installed, Launch...", app));
        am_start(arg)?;
        println_cn("完毕");
        println_en("Complete");

        Ok(())
    };

    if pm_path("com.dergoogler.mmrl.wx", true)? {
        launch("WebUI-X", &["com.dergoogler.mmrl.wx/.ui.activity.webui.WebUIActivity", "-e", "MOD_ID", "fs_enhancer_extreme"])?;
    } else if pm_path("io.github.a13e300.ksuwebui", true)? {
        launch("WebUI", &["io.github.a13e300.ksuwebui/.WebUIActivity", "-e", "id", "fs_enhancer_extreme"])?;
    } else {
        println_cn("- WebUI未安装,启动失败");
        println_en("- WebUI not installed, Start failed");
    }

    Ok(())
}