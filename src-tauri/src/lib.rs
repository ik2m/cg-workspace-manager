use std::fs;
use std::path::PathBuf;
use tauri::command;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[command]
fn list_files_in_dir(dir: String) -> Result<Vec<String>, String> {
    let path = PathBuf::from(dir);
    if !path.is_dir() {
        return Err("指定されたパスはディレクトリではありません".into());
    }

    let entries = fs::read_dir(path).map_err(|e| format!("ディレクトリ読み込み失敗: {}", e))?;

    let mut files = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|e| format!("エントリ読み込み失敗: {}", e))?;
        let file_type = entry
            .file_type()
            .map_err(|e| format!("ファイルタイプ取得失敗: {}", e))?;
        files.push(entry.file_name().to_string_lossy().to_string());
    }

    Ok(files)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, list_files_in_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
