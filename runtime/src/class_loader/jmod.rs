use crate::class_loader::ClassLoaderErr;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use tracing_log::log::debug;
use zip::ZipArchive;

/// In JDK 24, HotSpot parses the `modules` file located in `$JAVA_HOME/lib/modules` to resolve the runtime image.  
/// However, this file is not documented and there is no stable public API for parsing it directly.
///
/// As a temporary workaround, this project falls back to parsing `.jmod` files from `$JAVA_HOME/jmods` to get the class
/// binary representation.
#[derive(Debug, Clone)]
struct JmodSource {
    jmod_path: PathBuf,
    entry_name: String,
}

#[derive(Debug)]
pub struct BootstrapJmodLoader {
    index: HashMap<String, JmodSource>,
}

impl BootstrapJmodLoader {
    pub fn from_jmods_dir<P: AsRef<Path>>(jmods_dir: P) -> Result<Self, ClassLoaderErr> {
        debug!(
            "Creating BootstrapJmodLoader from jmods dir \"{}\"...",
            jmods_dir.as_ref().display()
        );
        debug!("Preparing index of classes in jmod files...");
        let mut index = HashMap::new();

        for entry in std::fs::read_dir(jmods_dir).map_err(|_| ClassLoaderErr::CanNotAccessSource)? {
            let path = entry
                .map_err(|_| ClassLoaderErr::CanNotAccessSource)?
                .path();
            if path.extension().map(|e| e == "jmod").unwrap_or(false) {
                let file = File::open(&path).map_err(|_| ClassLoaderErr::CanNotAccessSource)?;
                let mut zip = ZipArchive::new(file).map_err(|_| ClassLoaderErr::ArchiveErr)?;
                for i in 0..zip.len() {
                    let name = {
                        let zf = zip
                            .by_index_raw(i)
                            .map_err(|_| ClassLoaderErr::ArchiveErr)?;
                        zf.name().to_owned()
                    };

                    if !name.starts_with("classes/") || !name.ends_with(".class") {
                        continue;
                    }

                    if let Some(bin_name) = Self::binary_name(&name) {
                        index.entry(bin_name).or_insert(JmodSource {
                            jmod_path: path.clone(),
                            entry_name: name,
                        });
                    }
                }
            }
        }

        debug!("Index prepared. Found {} classes.", index.len());

        Ok(Self { index })
    }

    fn binary_name(entry: &str) -> Option<String> {
        const PREFIX: &str = "classes/";
        const SUFFIX: &str = ".class";
        if !entry.starts_with(PREFIX) || !entry.ends_with(SUFFIX) {
            return None;
        }
        let inner = &entry[PREFIX.len()..entry.len() - SUFFIX.len()];
        Some(inner.to_string())
    }

    pub fn find_class(&self, binary_name: &str) -> Result<Vec<u8>, ClassLoaderErr> {
        let src = self
            .index
            .get(binary_name)
            .ok_or_else(|| ClassLoaderErr::ClassNotFound(binary_name.to_string()))?;

        let file = File::open(&src.jmod_path).map_err(|_| ClassLoaderErr::CanNotAccessSource)?;
        let mut zip = ZipArchive::new(file).map_err(|_| ClassLoaderErr::ArchiveErr)?;
        let mut zf = zip
            .by_name(&src.entry_name)
            .map_err(|_| ClassLoaderErr::ArchiveErr)?;
        let mut buf = Vec::with_capacity(zf.size() as usize);
        zf.read_to_end(&mut buf)
            .map_err(|_| ClassLoaderErr::ArchiveErr)?;
        Ok(buf)
    }
}
