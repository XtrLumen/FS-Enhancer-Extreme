use crate::bridge::log_i;

use std::{
    str,
    process::{
        Stdio,
        Command
    },
    sync::OnceLock,
};

const MAGISK_OFFICIAL_PKG: &str = "com.topjohnwu.magisk";
const MAGISK_THIRD_PARTIES: &[(&str, &str)] = &[
    ("alpha", "io.github.vvb2060.magisk"),
    ("kitsune", "io.github.huskydg.magisk"),
];

/// Lazily detected package name of the installed Magisk variant.
static MAGISK_VARIANT_PKG: OnceLock<&'static str> = OnceLock::new();

/// Helper to execute a command and capture its stdout.
fn run_command(program: &str, args: &[&str]) -> Option<String> {
    Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?
        .wait_with_output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
}

/// Detects the installed Magisk variant (Official, Alpha, Kitsune, etc.).
fn detect_variant() -> &'static str {
    if let Some(version_str) = run_command("magisk", &["-v"]) {
        for (keyword, pkg_name) in MAGISK_THIRD_PARTIES {
            if version_str.contains(keyword) {
                log_i(&format!("Detected Magisk variant: {}", keyword));
                return pkg_name;
            }
        }
    }
    log_i("Detected official Magisk variant.");
    MAGISK_OFFICIAL_PKG
}

/// Detects if Magisk is installed and if its version is supported.
pub fn detect() -> Option<u32> {
    let version_str = run_command("magisk", &["-V"])?;
    let version:u32 = version_str.trim().parse::<u32>().ok()?;

    // As a side effect of successful version detection, cache the variant.
    MAGISK_VARIANT_PKG.get_or_init(detect_variant);

    Some(version)
}