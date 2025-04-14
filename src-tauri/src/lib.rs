use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;
use tauri::command;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn list_files_in_dir(dir: String) -> Result<Vec<String>, String> {
    let path = PathBuf::from(dir);
    if !path.is_dir() {
        return Err("指定されたパスはディレクトリではありません".into());
    }

    let files = WalkDir::new(&path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.path().display().to_string())
        .collect();

    Ok(files)
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, list_files_in_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
