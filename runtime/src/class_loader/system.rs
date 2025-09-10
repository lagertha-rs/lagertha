use crate::class_loader::{ClassLoaderErr, ClassSource};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use tracing_log::log::debug;

#[derive(Debug)]
pub(super) struct SystemClassLoader {
    index: HashMap<String, ClassSource>,
}

impl SystemClassLoader {
    pub fn new(path: &Vec<String>) -> Result<Self, ClassLoaderErr> {
        debug!("Creating SystemClassLoader from classpath entries: {path:?}");
        let mut index = HashMap::new();

        for entry in path {
            let base = PathBuf::from(entry);
            if !base.exists() {
                continue;
            }
            if base.is_dir() {
                debug!("Indexing classes under {}", base.display());
                Self::index_dir(&base, &mut index)?;
            } else {
                debug!(
                    "Classpath entry is not a directory, skipping: {}",
                    base.display()
                );
            }
        }

        debug!(
            "System classpath index prepared. Found {} classes.",
            index.len()
        );
        Ok(Self { index })
    }

    pub(crate) fn find_class(&self, name: &str) -> Result<Vec<u8>, ClassLoaderErr> {
        let key = Self::normalize_key(name);
        let src = self
            .index
            .get(&key)
            .ok_or_else(|| ClassLoaderErr::ClassNotFound(name.to_string()))?;

        let abs_path = src.jmod_path.join(&src.entry_name);
        let mut file = File::open(&abs_path).map_err(|_| ClassLoaderErr::CanNotAccessSource)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)
            .map_err(|_| ClassLoaderErr::CanNotAccessSource)?;
        Ok(buf)
    }

    fn index_dir(
        base_dir: &Path,
        index: &mut HashMap<String, ClassSource>,
    ) -> Result<(), ClassLoaderErr> {
        let mut stack = vec![base_dir.to_path_buf()];
        while let Some(dir) = stack.pop() {
            let rd = fs::read_dir(&dir).map_err(|_| ClassLoaderErr::CanNotAccessSource)?;
            for entry in rd {
                let entry = entry.map_err(|_| ClassLoaderErr::CanNotAccessSource)?;
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.is_file() {
                    if path.extension().map(|e| e == "class").unwrap_or(false) {
                        let rel = path.strip_prefix(base_dir).unwrap_or(&path);
                        let rel_str = Self::path_to_forward_slash(rel);
                        if let Some(key) = Self::binary_name_from_rel(&rel_str) {
                            index.entry(key).or_insert_with(|| ClassSource {
                                jmod_path: base_dir.to_path_buf(),
                                entry_name: rel.to_string_lossy().into_owned(), // keep OS separators for joining
                            });
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn path_to_forward_slash(p: &Path) -> String {
        p.components()
            .map(|c| c.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .join("/")
    }

    fn binary_name_from_rel(rel: &str) -> Option<String> {
        rel.strip_suffix(".class").map(|s| s.to_string())
    }

    fn normalize_key(name: &str) -> String {
        let s = name.replace('.', "/");
        if let Some(stripped) = s.strip_suffix(".class") {
            stripped.to_string()
        } else {
            s
        }
    }
}
