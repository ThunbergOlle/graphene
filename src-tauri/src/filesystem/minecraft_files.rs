use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct LauncherProfiles {
    pub profiles: HashMap<String, LauncherProfile>,
    pub settings: LauncherSettings,
    pub version: u8,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LauncherProfile {
    pub created: String,
    pub icon: String,
    pub last_used: String,
    pub last_version_id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub profile_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LauncherSettings {
    pub enable_advanced: bool,
    pub enable_analytics: bool,
    pub enable_historical: bool,
    pub enable_releases: bool,
    pub enable_snapshots: bool,
    pub keep_launcher_open: bool,
    pub profile_sorting: String,
    pub show_game_log: bool,
    pub show_menu: bool,
    pub sound_on: bool,
}

pub fn read_launcher_profiles(minecraft_default_path: &str) -> LauncherProfiles {
    let launcher_profiles_path =
        std::path::Path::new(minecraft_default_path).join("launcher_profiles.json");

    let file = std::fs::File::open(launcher_profiles_path).unwrap();
    let reader = std::io::BufReader::new(file);

    let launcher_profiles: LauncherProfiles = serde_json::from_reader(reader).unwrap();

    launcher_profiles
}

pub fn write_launcher_profiles(minecraft_default_path: &str, launcher_profiles: LauncherProfiles) {
    let launcher_profiles_path =
        std::path::Path::new(minecraft_default_path).join("launcher_profiles.json");

    let file = std::fs::File::create(launcher_profiles_path).unwrap();
    let writer = std::io::BufWriter::new(file);

    serde_json::to_writer_pretty(writer, &launcher_profiles).unwrap()
}
