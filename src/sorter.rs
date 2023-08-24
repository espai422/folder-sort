use std::ffi::{OsStr, OsString};
use std::fs::{create_dir_all, rename, DirEntry};
use std::io;
use std::path::{Path, PathBuf};

const DEFAULT_DIR: &str = "def";

pub fn sort(path: &Path) -> io::Result<()> {
    // get directory content
    let content = path.read_dir()?;

    // Create missing folders
    content
        .filter_map(|fs_entity_result| -> Option<PathBuf> {
            let fs_entity = fs_entity_result.ok()?;
            let filetype = fs_entity.file_type().ok()?;

            // Filter for files
            if !filetype.is_file() {
                return None;
            }

            Some(fs_entity.path())
        })
        .for_each(|path| {
            let file_name = path.to_str().unwrap_or_default();
            match move_file(path.as_path()) {
                Ok(_) => println!("Moved File: {file_name}"),
                Err(err) => println!("Could not move file: {file_name}\n{:?}", err),
            };
        });

    Ok(())
}

fn move_file(path: &Path) -> io::Result<()> {
    let folder_name = get_folder_by_extension(path);
    let mut path_segments = split_path(path);

    let new_directory_path_name = path_segments.join("/");
    let mut new_directory_path = PathBuf::from(&new_directory_path_name);

    new_directory_path.pop();
    new_directory_path.push(folder_name);

    if !new_directory_path.try_exists().unwrap_or(true) {
        match create_dir_all(new_directory_path) {
            Ok(_) => println!("Created path: {}", new_directory_path_name),
            Err(_) => println!("Could not create: {}", new_directory_path_name),
        }
    }

    path_segments.insert(path_segments.len() - 1, folder_name);

    let new_path_name = path_segments.join("/");
    let new_path = Path::new(&new_path_name);

    rename(path, new_path)
}

fn split_path(path: &Path) -> Vec<&str> {
    let mut path_segments: Vec<&str> = path
        .iter()
        .map(|segment| segment.to_str().unwrap_or_default())
        .collect();
    path_segments
}

fn get_folder_by_extension(path: &Path) -> &str {
    let folder_name = path
        .extension()
        .unwrap_or(OsStr::new(DEFAULT_DIR))
        .to_str()
        .unwrap_or_default();
    folder_name
}
