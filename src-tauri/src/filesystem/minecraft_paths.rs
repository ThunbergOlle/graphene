use std::env;

#[tauri::command]
pub fn default_path() -> String {
    let path = match env::consts::OS {
        "macos" => {
            let home = env::var("HOME").unwrap();
            format!("{}/Library/Application Support/minecraft", home)
        }
        "windows" => {
            let home = env::var("APPDATA").unwrap();
            format!("{}/.minecraft", home)
        }
        "linux" => {
            let home = env::var("HOME").unwrap();
            format!("{}/.minecraft", home)
        }
        _ => {
            let home = env::var("HOME").unwrap();
            format!("{}/.minecraft", home)
        }
    };

    println!("path: {}", path);

    path
}

#[tauri::command]
pub fn validate_path(path: &str) -> bool {
    if path.is_empty() {
        return false;
    }

    let path = std::path::Path::new(path);
    let folder_exists = path.exists();

    // does folder have a launcher_profiles.json file?
    let launcher_profiles: std::path::PathBuf = path.join("launcher_profiles.json");

    let file_exists = launcher_profiles.exists();

    folder_exists && file_exists
}
