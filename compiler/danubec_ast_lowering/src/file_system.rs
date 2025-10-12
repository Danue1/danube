use fxhash::FxHashMap;
use slotmap::{SecondaryMap, SlotMap};
use std::{collections::HashMap, path::PathBuf};

slotmap::new_key_type! {
    pub struct FileId;
}

#[derive(Debug)]
pub struct FileSystem {
    inner: InnerFileSystem,
}

#[derive(Debug)]
struct InnerFileSystem {
    paths: SlotMap<FileId, PathBuf>,
    canonicals: SecondaryMap<FileId, PathBuf>,
    files: SecondaryMap<FileId, String>,

    path_to_id: FxHashMap<PathBuf, FileId>,
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            inner: InnerFileSystem::new(),
        }
    }

    #[inline]
    pub fn krate(&mut self, path: PathBuf) -> FileId {
        self.inner.file(path)
    }

    pub fn module(&mut self, parent_id: FileId, path: &str) -> Option<FileId> {
        let parent = self.inner.path(parent_id)?.parent()?.join(path);

        {
            let path = parent.with_extension("dnb");
            if let Some(file_id) = self.inner.path_to_id(&path) {
                return Some(file_id);
            }
            if path.exists() {
                return Some(self.inner.file(path));
            }
        }

        {
            let path = parent.join("mod").with_extension("dnb");
            if let Some(file_id) = self.inner.path_to_id(&path) {
                return Some(file_id);
            }
            if path.exists() {
                return Some(self.inner.file(path));
            }
        }

        None
    }

    #[inline]
    pub fn source(&mut self, file_id: FileId) -> Option<&str> {
        self.inner.source(file_id)
    }

    #[inline]
    pub fn path(&self, file_id: FileId) -> Option<&PathBuf> {
        self.inner.path(file_id)
    }
}

impl InnerFileSystem {
    fn new() -> Self {
        Self {
            paths: SlotMap::with_key(),
            canonicals: SecondaryMap::new(),
            files: SecondaryMap::new(),

            path_to_id: FxHashMap::default(),
        }
    }

    fn file(&mut self, path: PathBuf) -> FileId {
        use std::collections::hash_map::Entry;

        match self.path_to_id.entry(path.clone()) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                let canonical = path.canonicalize().ok();
                let canonical = canonical.and_then(|c| if c != path { Some(c) } else { None });
                let file_id = self.paths.insert(path);
                entry.insert(file_id);

                if let Some(canonical) = canonical {
                    self.canonicals.insert(file_id, canonical);
                }

                file_id
            }
        }
    }

    fn canonicalized(&self, file_id: FileId) -> Option<&PathBuf> {
        let path = self.canonicals.get(file_id);
        path.or_else(|| self.paths.get(file_id))
    }

    #[inline]
    fn path(&self, file_id: FileId) -> Option<&PathBuf> {
        self.paths.get(file_id)
    }

    #[inline]
    fn path_to_id(&self, path: &PathBuf) -> Option<FileId> {
        self.path_to_id.get(path).copied()
    }

    fn source(&mut self, file_id: FileId) -> Option<&str> {
        if !self.files.contains_key(file_id) {
            let path = self.canonicalized(file_id)?;
            let content = std::fs::read_to_string(path).ok()?;
            self.files.insert(file_id, content);
        }

        self.files.get(file_id).map(|s| s.as_str())
    }
}
