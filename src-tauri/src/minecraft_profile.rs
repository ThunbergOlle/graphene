use crate::filesystem::minecraft_files::{
    read_launcher_profiles, write_launcher_profiles, LauncherProfile,
};

pub fn new_graphene_profile() -> LauncherProfile {
    LauncherProfile {
        created: "2021-06-01T15:00:00.000Z".to_string(),
        icon: "graphene".to_string(),
        last_used: "2021-06-01T15:00:00.000Z".to_string(),
        last_version_id: "1.17".to_string(),
        name: "Graphene".to_string(),
        profile_type: "custom".to_string(),
    }
}

pub fn add_profile_to_launcher_profiles(minecraft_default_path: &str, profile: LauncherProfile) {
    let mut launcher_profiles = read_launcher_profiles(minecraft_default_path);

    launcher_profiles
        .profiles
        .insert(profile.name.clone(), profile);

    write_launcher_profiles(minecraft_default_path, launcher_profiles);

    println!("Added profile to launcher_profiles.json");
}
