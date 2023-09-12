use std::ffi::{OsStr, OsString};
use std::fs::{create_dir_all, rename, DirEntry};
use std::io;
use std::path::{Path, PathBuf};

const DEFAULT_DIR: &str = "def";

pub fn sort(path: &Path) -> io::Result<()> {
    // get directory content
    let content = path.read_dir()?;

    // Create missing folders
    content.filter_map(dir_entry_to_file).for_each(|file| {
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
    if !fs_entity.file_type().ok()?.is_file() {
        return None;
    }

    Some(fs_entity)
}

fn move_file(file: &DirEntry) -> io::Result<()> {
    let path = file.path();

    let folder_name = get_extension(&path);
    let mut filename = file.file_name();
    let str_filename = filename.to_str().unwrap_or_default();
    let source_path = path.clone();

    let mut path = file.path();

    path.pop();
    path.push(folder_name);

    create_dir_if_not_exists(&path);

    let files_with_same_name = get_file_number(&path, str_filename);
    match files_with_same_name {
        None => {}
        Some(num) => {filename = OsString::from(format!("({}){}",num, str_filename));}
    }

    path.push(&filename);

    rename(&source_path, &path)
}

fn get_file_number(path: &PathBuf, filename: &str) -> Option<usize> {
    let matching_entities = get_matching_entities(path, filename)?;
    let number = matching_entities.len();

    let mut flag = false;
    for i in matching_entities {
        if i.file_name().to_str().unwrap_or_default().eq(filename) {
            flag = true;
            break;
        }
    }

    if flag {
        return Some(number);
    }

    None
}

fn get_matching_entities(path: &PathBuf, filename: &str) -> Option<Vec<DirEntry>> {
    let vec = path
        .read_dir().ok()?
        .filter_map(|fs_entity| {
            let fs_entity = fs_entity.ok()?;
            let is_valid_entity = fs_entity
                .file_name()
                .to_str()
                .unwrap_or_default()
                .contains(filename);

            if is_valid_entity {
                return Some(fs_entity);
            }

            None
        }).collect();

    Some(vec)
}

fn create_dir_if_not_exists(path: &PathBuf) {
    if !path.try_exists().unwrap_or(true) {
        match create_dir_all(&path) {
            Ok(_) => println!("Created directory: {:?}", path),
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::fs::{remove_dir_all, File};

    #[test]
    fn sort_directory() -> Result<(), Box<dyn Error>> {
        let test_directory = "./testing_dir";
        let mut path = PathBuf::from(test_directory);

        let _ = remove_dir_all(&path);
        create_dir_all(&path)?;

        // Create sample files to sort
        let files = vec!["test.txt", "main.py", "no_extension"];

        for file in &files {
            path.push(file);
            File::create(&path)?;
            path.pop();
        }

        // Sort
        sort(&path)?;

        // Read the directory
        let directories = vec!["txt", "py", DEFAULT_DIR];

        for (dir, file) in directories.iter().zip(files.iter()) {
            path.push(dir);

            let entries: Vec<PathBuf> = path
                .read_dir()?
                .filter_map(|entry| entry.ok().map(|e| e.path()))
                .collect();

            assert!(
                entries.iter().all(|p| p.to_str().unwrap().ends_with(file)),
                "File not found: {:?}",
                file
            );

            path.pop();
        }

        // Check if the directory is successfully sorted
        Ok(())
    }
}
