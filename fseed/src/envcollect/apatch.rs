use std::{
    process,
    process::Stdio
};

pub fn detect() -> Option<u32> {
    process::Command::new("apd")
        .arg("-V")
        .stdout(Stdio::piped())
        .spawn()
        .ok()
        .and_then(|child| child.wait_with_output().ok())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .and_then(|version_str| version_str.split_whitespace().nth(1)?.parse::<u32>().ok())
}