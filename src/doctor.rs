use dialoguer::{Confirm, theme::ColorfulTheme};
use owo_colors::OwoColorize;

use crate::util::install::{install_uv, is_uv_installed};
use crate::util::path::{
    base_dir_exists, config_file_exists, make_base_dir, make_config_file, make_versions_dir,
    versions_dir_exists,
};

pub fn doctor() {
    println!();
    println!("{}", "⚡ MpyC Doctor".bright_yellow().bold());
    println!("{}", "Running system diagnostics...\n".bright_black());

    let mut issues = 0;

    // --- UV CHECK ---
    if is_uv_installed() {
        println!("{} uv is installed", "✔".bright_green());
    } else {
        println!("{} uv is NOT installed", "✖".bright_red());
        issues += 1;
    }

    // --- BASE DIR ---
    if base_dir_exists() {
        println!("{} Base directory", "✔".bright_green());
    } else {
        println!("{} Base directory missing", "✖".bright_red());
        issues += 1;
    }

    // --- VERSIONS DIR ---
    if versions_dir_exists() {
        println!("{} Versions directory", "✔".bright_green());
    } else {
        println!("{} Versions directory missing", "✖".bright_red());
        issues += 1;
    }

    // --- CONFIG FILE ---
    if config_file_exists() {
        println!("{} Config file", "✔".bright_green());
    } else {
        println!("{} Config file missing", "✖".bright_red());
        issues += 1;
    }

    println!();

    if issues == 0 {
        println!("{}", "✔ All checks passed!".bright_green().bold());
        return;
    }

    println!(
        "{} Found {} issue(s).",
        "⚠".bright_yellow(),
        issues.to_string().bright_yellow()
    );

    let fix = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Attempt automatic repair?")
        .default(true)
        .interact()
        .unwrap();

    if !fix {
        println!("{}", "Aborted.".bright_yellow());
        return;
    }

    println!();
    println!("{}", "Applying fixes...\n".bright_blue());

    if !is_uv_installed() {
        if let Err(e) = install_uv() {
            eprintln!("{} Failed to install uv: {}", "✖".bright_red(), e);
        } else {
            println!("{} uv installed", "✔".bright_green());
        }
    }

    if !base_dir_exists() {
        if let Err(e) = make_base_dir() {
            eprintln!(
                "{} Failed to create base directory: {}",
                "✖".bright_red(),
                e
            );
        } else {
            println!("{} Base directory created", "✔".bright_green());
        }
    }

    if !versions_dir_exists() {
        if let Err(e) = make_versions_dir() {
            eprintln!(
                "{} Failed to create versions directory: {}",
                "✖".bright_red(),
                e
            );
        } else {
            println!("{} Versions directory created", "✔".bright_green());
        }
    }

    if !config_file_exists() {
        if let Err(e) = make_config_file() {
            eprintln!("{} Failed to create config file: {}", "✖".bright_red(), e);
        } else {
            println!("{} Config file created", "✔".bright_green());
        }
    }

    println!();
    println!("{}", "✔ Doctor finished.".bright_green().bold());
}
