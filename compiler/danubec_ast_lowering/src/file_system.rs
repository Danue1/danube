use slotmap::{SecondaryMap, SlotMap};
use std::path::PathBuf;

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
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            inner: InnerFileSystem::new(),
        }
    }

    #[inline]
    pub fn krate(&mut self, path: PathBuf) -> FileId {
        self.file(path)
    }

    pub fn module(&mut self, parent_id: FileId, path: &str) -> Option<FileId> {
        let path = self.inner.path(parent_id)?.parent()?.join(path);

        let path = path.with_extension("dnb");
        if path.exists() {
            return Some(self.file(path));
        }

        let path = path.join("mod").with_extension("dnb");
        dbg!(&path);
        if path.exists() {
            return Some(self.file(path));
        }

        None
    }

    #[inline]
    pub fn file(&mut self, path: PathBuf) -> FileId {
        self.inner.file(path)
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
        }
    }

    fn file(&mut self, path: PathBuf) -> FileId {
        let canonical = path.canonicalize().ok();
        let same = canonical.as_ref().map_or(false, |c| c == &path);
        let file_id = self.paths.insert(path);

        if !same && let Some(canonical) = canonical {
            self.canonicals.insert(file_id, canonical);
        }

        file_id
    }

    fn canonicalized(&self, file_id: FileId) -> Option<&PathBuf> {
        let path = self.canonicals.get(file_id);
        path.or_else(|| self.paths.get(file_id))
    }

    #[inline]
    fn path(&self, file_id: FileId) -> Option<&PathBuf> {
        self.paths.get(file_id)
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
