const DIR_NAME: &str = ".mpyc";
const VERSIONS_DIR_NAME: &str = "versions";

const CONFIG_FILE_NAME: &str = "config.toml";

fn get_base_dir() -> std::path::PathBuf {
    dirs::home_dir().unwrap().join(DIR_NAME)
}

pub fn get_config_file() -> std::path::PathBuf {
    get_base_dir().join(CONFIG_FILE_NAME)
}

// Gets the directory where all versions of mpy-cross are stored
pub fn get_versions_dir() -> std::path::PathBuf {
    get_base_dir().join(VERSIONS_DIR_NAME)
}

/// Checks if the base .mpyc directory exists
pub fn base_dir_exists() -> bool {
    get_base_dir().exists()
}

pub fn config_file_exists() -> bool {
    get_config_file().exists()
}

pub fn versions_dir_exists() -> bool {
    get_versions_dir().exists()
}

pub fn make_base_dir() -> std::io::Result<()> {
    std::fs::create_dir_all(get_base_dir())
}

pub fn make_versions_dir() -> std::io::Result<()> {
    std::fs::create_dir_all(get_versions_dir())
}

pub fn make_config_file() -> std::io::Result<()> {
    std::fs::File::create(get_config_file())?;
    Ok(())
}
