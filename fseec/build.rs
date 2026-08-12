use std::process;

fn main() {
    let git_result = process::Command::new("git").args(["describe", "--tags", "--always"])
        .output().ok()
        .and_then(|output|
            String::from_utf8(output.stdout).ok()
        );
    let git_version: String = if let Some(success) = git_result {
        success
    } else {
        println!("cargo:warning=Failed to get git version, using default");
        String::from("0.0.0")
    };
    println!("cargo:rustc-env=VERSION_NAME={}", git_version);
}