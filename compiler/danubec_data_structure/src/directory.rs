use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
pub struct Directory<T> {
    files: HashMap<String, T>,
    directories: HashMap<String, Directory<T>>,
}

#[derive(Debug)]
pub enum DirectoryError {
    InvalidDirectory,
    InvalidPath,
}

impl<T> Directory<T> {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            directories: HashMap::new(),
        }
    }

    pub fn insert(&mut self, path: &PathBuf, value: T) -> Result<(), DirectoryError> {
        let components: Vec<_> = path.iter().filter_map(|c| c.to_str()).collect();
        if let Some((_, directories)) = components.split_last() {
            let mut current = self;
            for directory in directories {
                if directory.is_empty() {
                    return Err(DirectoryError::InvalidDirectory);
                }
                current = current
                    .directories
                    .entry(directory.to_string())
                    .or_insert_with(Directory::new);
            }
            let file = path
                .file_stem()
                .expect("failed to get file stem")
                .to_string_lossy()
                .to_string();
            current.files.insert(file, value);

            Ok(())
        } else {
            Err(DirectoryError::InvalidPath)
        }
    }

    #[inline]
    pub fn file(&self, path: &str) -> Option<&T> {
        self.files.get(path)
    }

    #[inline]
    pub fn directory(&self, path: &str) -> Option<&Directory<T>> {
        self.directories.get(path)
    }

    pub fn directory_recursive(&self, path: &str) -> Option<&Directory<T>> {
        let mut current = self;
        for directory in path.split('/') {
            if directory.is_empty() {
                return None;
            }
            current = current.directory(directory)?;
        }
        Some(current)
    }
}
