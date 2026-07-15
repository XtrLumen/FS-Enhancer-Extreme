use std::process::Command;

fn get_git_version() -> Result<String, std::io::Error> {
    let version_name = String::from_utf8(
        Command::new("git")
            .args(["describe", "--tags", "--always"])
            .output()?
            .stdout
    ).map_err(|_|
        std::io::Error::other("Failed to read git describe stdout")
    )?;
    let version_name = version_name.trim_start_matches('v').to_string();
    Ok(version_name)
}

fn main() {
    let name = get_git_version().unwrap_or_else(|_|{
        // show warning if git is not installed
        println!("cargo:warning=Failed to get git version, using 0.0.0");
        "0.0.0".to_string()
    });
    println!("cargo:rustc-env=VERSION_NAME={name}");
}
