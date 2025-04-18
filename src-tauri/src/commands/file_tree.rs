use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use crate::{FileNode};

#[tauri::command]
pub fn list_files_in_dir(dir: String) -> Result<FileNode, String> {
    let path = PathBuf::from(dir);
    if !path.is_dir() {
        return Err("指定されたパスはディレクトリではありません".into());
    }

    Ok(build_file_tree(&path))
}

fn build_file_tree(root_path: &Path) -> FileNode {
    // 中間ノード用: path -> 子ノードたち
    let mut nodes: HashMap<PathBuf, Vec<FileNode>> = HashMap::new();

    // すべてのエントリを収集
    for entry in WalkDir::new(root_path)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path().to_path_buf();
        let parent_path = path.parent().unwrap_or(root_path).to_path_buf();
        let file_name = path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.display().to_string());

        let node = if path.is_dir() {
            // 空ディレクトリも `children: []` で作る
            FileNode::Dir {
                name: file_name,
                path: path.display().to_string(),
                children: Vec::new(),
            }
        } else {
            FileNode::File {
                name: file_name,
                path: path.display().to_string(),
            }
        };

        nodes.entry(parent_path).or_default().push(node);

        // ディレクトリ自身もキーにしておく（空であっても）
        if path.is_dir() {
            nodes.entry(path).or_default();
        }
    }

    fn attach_children(path: &PathBuf, nodes: &mut HashMap<PathBuf, Vec<FileNode>>) -> FileNode {
        let name = path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.display().to_string());

        let children = nodes.remove(path).unwrap_or_default()
            .into_iter()
            .map(|mut node| match &mut node {
                FileNode::Dir { path, .. } => attach_children(&PathBuf::from(path.clone()), nodes),
                _ => node,
            })
            .collect::<Vec<_>>();

        FileNode::Dir {
            name,
            path: path.display().to_string(),
            children,
        }
    }

    attach_children(&root_path.to_path_buf(), &mut nodes)
}