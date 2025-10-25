use crate::class_loader::{ClassLoaderErr, ClassSource};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use tracing_log::log::debug;
use walkdir::WalkDir;

#[derive(Debug)]
pub(super) struct SystemClassLoader {
    index: HashMap<String, ClassSource>,
}

impl SystemClassLoader {
    pub fn new(path: &Vec<String>) -> Result<Self, ClassLoaderErr> {
        debug!("Creating SystemClassLoader from classpath entries: {path:?}");
        let mut index = HashMap::new();

        for entry in path {
            let files_and_folders = WalkDir::new(entry);
            let files_and_folders = files_and_folders.into_iter().collect::<Vec<_>>();
            let java_classes: Vec<_> = files_and_folders
                .into_iter()
                .filter_map(Result::ok)
                .map(|e| e.into_path())
                .filter(|path| {
                    path.is_file() && path.extension().map(|ext| ext == "class").unwrap_or(false)
                })
                .collect();
            for class in java_classes {
                let rel = class.strip_prefix(entry).unwrap_or(&class);
                let rel_str = Self::path_to_forward_slash(rel);
                if let Some(key) = Self::binary_name_from_rel(&rel_str) {
                    index.entry(key).or_insert_with(|| ClassSource {
                        jmod_path: PathBuf::from(entry),
                        entry_name: rel.to_string_lossy().into_owned(),
                    });
                }
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
