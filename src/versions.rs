use pep440::Version;
use reqwest::blocking::get;
use serde_json::Value;

use crate::util::config::Config;
use crate::util::path::get_versions_dir;

const MPY_CROSS_PYPI_URL: &str = "https://pypi.org/pypi/mpy-cross/json";

/// Checks if a specific version of mpy-cross exists on PyPI
pub fn mpy_cross_version_exists(version: &str) -> bool {
    let response = get(MPY_CROSS_PYPI_URL).expect("Failed to fetch PyPI mpy-cross JSON");

    let json: Value = response.json().expect("Failed to parse JSON from PyPI");

    if let Some(releases) = json.get("releases") {
        releases.get(version).is_some()
    } else {
        false
    }
}

/// Gets a list of all available versions of mpy-cross from PyPI
fn get_all_mpy_cross_versions() -> Vec<String> {
    let response = get(MPY_CROSS_PYPI_URL).expect("Failed to fetch PyPI mpy-cross JSON");

    let json: Value = response.json().expect("Failed to parse JSON from PyPI");

    if let Some(releases) = json.get("releases") {
        releases.as_object().unwrap().keys().cloned().collect()
    } else {
        vec![]
    }
}

/// Gets a list of all installed versions of mpy-crosss
fn get_installed_mpy_cross_versions() -> Vec<String> {
    let mut versions = Vec::new();
    let versions_dir = get_versions_dir();
    for entry in std::fs::read_dir(versions_dir).unwrap() {
        let version = entry.unwrap().file_name().to_str().unwrap().to_string();
        if mpy_cross_version_exists(&version) {
            versions.push(version);
        }
    }
    versions
}

fn get_active_mpy_cross_version() -> Option<String> {
    // This function should return the currently active mpy-cross version
    // For now, it returns None as a placeholder
    let config = Config::load().expect("Failed to load config");
    if config.is_valid() {
        Some(config.active_version)
    } else {
        None
    }
}

/// Merges the list of available versions with the list of installed versions and the active version
/// Returns a vector of tuples containing the version, whether it's installed, and whether it's active
pub fn merge_available_and_installed_versions() -> Vec<(String, bool, bool)> {
    let available_versions = get_all_mpy_cross_versions();
    let installed_versions = get_installed_mpy_cross_versions();
    let active_version = get_active_mpy_cross_version();

    let mut merged: Vec<(String, bool, bool, Version)> = available_versions
        .into_iter()
        .filter_map(|v| {
            let parsed = Version::parse(&v).expect("");

            // 👇 Only show stable versions by default
            if parsed.pre.is_some() || parsed.dev.is_some() {
                return None;
            }

            let installed = installed_versions.contains(&v);
            let active = active_version.as_ref() == Some(&v);

            Some((v, installed, active, parsed))
        })
        .collect();

    merged.sort_by(|a, b| b.2.cmp(&a.2).then(b.1.cmp(&a.1)).then(b.3.cmp(&a.3)));

    merged.into_iter().map(|(v, i, a, _)| (v, i, a)).collect()
}
