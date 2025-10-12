use danubec_symbol::FileId;
use slotmap::{SecondaryMap, SlotMap};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Fs {
    inner: Inner,
}

#[derive(Debug)]
struct Inner {
    paths: SlotMap<FileId, PathBuf>,
    canonicals: SecondaryMap<FileId, PathBuf>,
    files: SecondaryMap<FileId, String>,
}

impl Fs {
    pub fn new() -> Self {
        Self {
            inner: Inner::new(),
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
            if path.exists() {
                return Some(self.inner.file(path));
            }
        }

        {
            let path = parent.join("mod").with_extension("dnb");
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

impl Inner {
    fn new() -> Self {
        Self {
            paths: SlotMap::with_key(),
            canonicals: SecondaryMap::new(),
            files: SecondaryMap::new(),
        }
    }

    fn file(&mut self, path: PathBuf) -> FileId {
        if let Some(file_id) = self
            .paths
            .iter()
            .find_map(|(id, p)| (p == &path).then_some(id))
        {
            return file_id;
        }

        let canonical = path.canonicalize().ok();
        let canonical = canonical.and_then(|c| if c != path { Some(c) } else { None });
        let file_id = self.paths.insert(path);

        if let Some(canonical) = canonical {
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
