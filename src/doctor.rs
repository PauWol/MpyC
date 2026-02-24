use crate::util::install::{install_uv, is_uv_installed, mpy_cross_version_exists};
use crate::util::path::{
    base_dir_exists, config_file_exists, make_base_dir, make_config_file, make_versions_dir,
    versions_dir_exists,
};

pub fn doctor() {
    // 1. Check if uv is installed

    if !is_uv_installed() {
        println!("uv is not installed. Installing...");
        if let Err(e) = install_uv() {
            eprintln!("Failed to install uv: {}", e);
        } else {
            println!("uv installed successfully.");
        }
    }

    // 2. Check for Paths
    if !base_dir_exists() {
        println!(
            "Base directory does not exist. Please run 'mpyc setup' to set up the environment."
        );
        if let Err(e) = make_base_dir() {
            eprintln!("Failed to create base directory: {}", e);
        }
    } else {
        println!("Base directory exists.");
    }

    if !versions_dir_exists() {
        println!(
            "Versions directory does not exist. Please run 'mpyc setup' to set up the environment."
        );
        if let Err(e) = make_versions_dir() {
            eprintln!("Failed to create versions directory: {}", e);
        }
    } else {
        println!("Versions directory exists.");
    }

    if !config_file_exists() {
        println!("Config file does not exist. Please run 'mpyc setup' to set up the environment.");
        if let Err(e) = make_config_file() {
            eprintln!("Failed to create config file: {}", e);
        }
    } else {
        println!("Config file exists.");
    }
}
