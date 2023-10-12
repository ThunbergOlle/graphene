// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod filesystem;
mod minecraft_profile;

// location: /Users/ollethunberg/Library/Application Support/minecraft/launcher_profiles.json

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn create_minecraft_profile(minecraft_path: &str) {
    let graphene_profile = minecraft_profile::new_graphene_profile();

    minecraft_profile::add_profile_to_launcher_profiles(minecraft_path, graphene_profile);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            create_minecraft_profile,
            filesystem::minecraft_paths::default_path,
            filesystem::minecraft_paths::validate_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
