mod commands;
use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "type")]
enum FileNode {
    #[serde(rename = "dir")]
    Dir { name: String, path: String, children: Vec<FileNode> },

    #[serde(rename = "file")]
    File { name: String, path: String },
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::file_tree::list_files_in_dir,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
