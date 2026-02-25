use dialoguer::{Select, theme::ColorfulTheme};
use owo_colors::OwoColorize;

use crate::util::config::Config;
use crate::util::install::install_mpy_cross_version;
use crate::versions::{merge_available_and_installed_versions, mpy_cross_version_exists};

pub fn use_version() {
    println!("Listing installed and available versions...");

    let versions = merge_available_and_installed_versions();

    let items: Vec<String> = versions
        .iter()
        .map(|(version, installed, active)| {
            let mut label = version.to_string();

            if *active {
                label = format!(
                    "{} {}",
                    label.bright_green().bold(),
                    "(active)".bright_green()
                );
            } else if *installed {
                label = format!(
                    "{} {}",
                    label.bright_yellow(),
                    "(installed)".bright_yellow()
                );
            } else {
                label = format!(
                    "{} {}",
                    label.bright_black(),
                    "(not installed)".bright_black()
                );
            }

            label
        })
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a version")
        .items(&items)
        .default(0)
        .interact()
        .expect("Failed to select a version");

    let selected_version = &versions[selection];

    if !mpy_cross_version_exists(&selected_version.0) {
        println!("Selected version does not exist on PyPI!");
        return;
    }

    if !selected_version.1 {
        println!("Installing mpy-cross {}...", selected_version.0);
        install_mpy_cross_version(&selected_version.0).expect("Failed to install mpy-cross");
    }

    println!("Activating mpy-cross {}...", selected_version.0);

    let mut config = Config::load().expect("Failed to load config");
    config.active_version = selected_version.0.clone();
    config.save().expect("Failed to save config");
}
