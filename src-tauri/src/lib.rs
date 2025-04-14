use tauri::command;
use std::fs;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use serde::Serialize;

#[derive(Serialize)]
struct FileNode {
    name: String,
    path: String,
    children: Option<Vec<FileNode>>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn list_files_in_dir(dir: String) -> Result<FileNode, String> {
    let path = PathBuf::from(dir);
    if !path.is_dir() {
        return Err("指定されたパスはディレクトリではありません".into());
    }

    Ok(build_file_tree(&path))
}

fn build_file_tree(root_path: &Path) -> FileNode {
    let mut root = FileNode {
        name: root_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| root_path.display().to_string()),
        path: root_path.display().to_string(),
        children: Some(Vec::new()),
    };

    let mut nodes: HashMap<PathBuf, Vec<FileNode>> = HashMap::new();
    nodes.insert(root_path.to_path_buf(), Vec::new());

    for entry in WalkDir::new(root_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path() != root_path)
    {
        let path = entry.path().to_path_buf();
        let parent_path = path.parent().unwrap_or(root_path).to_path_buf();

        let node = FileNode {
            name: path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| path.display().to_string()),
            path: path.display().to_string(),
            children: if path.is_dir() { Some(Vec::new()) } else { None },
        };

        nodes.entry(parent_path.clone()).or_default().push(node);

        if path.is_dir() {
            nodes.entry(path).or_default();
        }
    }

    fn attach_children(node: &mut FileNode, nodes: &mut HashMap<PathBuf, Vec<FileNode>>) {
        if let Some(children) = nodes.remove(&PathBuf::from(&node.path)) {
            node.children = Some(
                children
                    .into_iter()
                    .map(|mut child| {
                        attach_children(&mut child, nodes);
                        child
                    })
                    .collect(),
            );
        }
    }

    attach_children(&mut root, &mut nodes);
    root
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
