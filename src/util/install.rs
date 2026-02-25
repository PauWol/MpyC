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
    let output = std::process::Command::new("bash")
        .arg("-c")
        .arg(&uv_install_command)
        .output()
        .expect("failed to execute process");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

pub fn install_mpy_cross(version: &str) {}
