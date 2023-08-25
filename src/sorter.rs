use std::ffi::{OsStr};
use std::fs::{create_dir_all, rename, DirEntry};
use std::io;
use std::path::{Path, PathBuf};

const DEFAULT_DIR: &str = "def";

pub fn sort(path: &Path) -> io::Result<()> {
    // get directory content
    let content = path.read_dir()?;

    // Create missing folders
    content
        .filter_map(|dir_entry| dir_entry_to_file(dir_entry))
        .for_each(|file| {
            let file_name = String::from(file.file_name().to_str().unwrap_or_default());
            match move_file(&file) {
                Ok(_) => println!("Moved File: {file_name}"),
                Err(err) => println!("Could not move file: {file_name}\n{:?}", err),
            };
        });

    Ok(())
}

fn dir_entry_to_file(fs_entity_result: io::Result<DirEntry>) -> Option<DirEntry> {
    let fs_entity = fs_entity_result.ok()?;
    if fs_entity.file_type().ok()?.is_file() {return None}

    Some(fs_entity)
}

fn move_file(file: &DirEntry) -> io::Result<()> {
    let path = file.path();

    let folder_name = get_extension(&path);
    let filename = file.file_name();
    let source_path = path.clone();

    let mut path = file.path();

    path.pop();
    path.push(folder_name);

    create_path_if_not_exists(&path);

    path.push(filename);

    rename(&source_path, &path)
}

fn create_path_if_not_exists(path: &PathBuf) {
    if !path.try_exists().unwrap_or(true) {
        match create_dir_all(&path) {
            Ok(_) => println!("Created path: {:?}", path),
            Err(_) => println!("Could not create: {:?}", path),
        }
    }
}

fn get_extension(path: &PathBuf) -> String {
    let folder_name = path
        .extension()
        .unwrap_or(OsStr::new(DEFAULT_DIR))
        .to_str()
        .unwrap_or_default();
    String::from(folder_name)
}
