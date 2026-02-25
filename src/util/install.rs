use crate::util::path::get_versions_dir;

use std::process::Command;

const WINDOWS_UV_INSTALL: &str =
    "powershell -ExecutionPolicy ByPass -c 'irm https://astral.sh/uv/install.ps1 | iex'";
const LINUX_MACOS_UV_INSTALL: &str = "curl -LsSf https://astral.sh/uv/install.sh | sh";

fn get_uv_install_command() -> String {
    if cfg!(target_os = "windows") {
        WINDOWS_UV_INSTALL.to_string()
    } else if cfg!(target_os = "macos") {
        LINUX_MACOS_UV_INSTALL.to_string()
    } else {
        LINUX_MACOS_UV_INSTALL.to_string()
    }
}

pub fn is_uv_installed() -> bool {
    which::which("uv").is_ok()
}

pub fn install_uv() -> Result<(), Box<dyn std::error::Error>> {
    let uv_install_command = get_uv_install_command();
    let output = Command::new("bash")
        .arg("-c")
        .arg(&uv_install_command)
        .output()
        .expect("failed to execute process");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

fn create_new_virtualenv(version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("uv")
        .arg("venv")
        .arg(version)
        .current_dir(get_versions_dir())
        .output()
        .expect("failed to execute process");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

fn install_mpy_cross(version: &str) -> Result<(), Box<dyn std::error::Error>> {
    // `uv install <version> --venv <path>` (example)
    let output = Command::new("uv")
        .arg("pip")
        .arg("install")
        .arg(format!("mpy-cross=={}", version))
        .arg("-t")
        .arg(get_versions_dir().join(version))
        .output()?;

    println!("{}", String::from_utf8_lossy(&output.stdout));
    eprintln!("{}", String::from_utf8_lossy(&output.stderr));

    if !output.status.success() {
        return Err("Failed to install mpy-cross with uv".into());
    }

    Ok(())
}

pub fn install_mpy_cross_version(version: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !is_uv_installed() {
        println!("uv is not installed! run `mpyc doctor` to fix this issue.");
        return Err("uv is not installed".into());
    }

    println!("Creating virtual environment for mpy-cross {}...", version);
    create_new_virtualenv(version)?;

    println!("Installing mpy-cross {}...", version);
    install_mpy_cross(version)?;

    Ok(())
}
