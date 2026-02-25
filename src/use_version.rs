use dialoguer::{Select, theme::ColorfulTheme};
use owo_colors::OwoColorize;

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
        .interact().expect("Failed to select a version");


    println!("You selected: {}", versions[selection].0);

    
}