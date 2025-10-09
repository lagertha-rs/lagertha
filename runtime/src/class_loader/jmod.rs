use crate::class_loader::{ClassLoaderErr, ClassSource};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml::Value;
use tracing_log::log::debug;
use zip::ZipArchive;

/// In JDK 24, HotSpot parses the `modules` file located in `$JAVA_HOME/lib/modules` to resolve the runtime image.  
/// However, this file is not documented and there is no stable public API for parsing it directly.
///
/// As a temporary workaround, this project falls back to parsing `.jmod` files from `$JAVA_HOME/jmods` to get the class
/// binary representation.

#[derive(Debug)]
pub struct BootstrapJmodClassLoader {
    index: HashMap<String, ClassSource>,
    //TODO: remove afterwards, insures whatever is loaded is present in fixtures.toml (tested)
    tested_parsing: HashSet<String>,
}

impl BootstrapJmodClassLoader {
    //TODO: remove afterwards
    fn prepare_tested_classes() -> HashSet<String> {
        let fixtures_toml = include_str!("../../../class_file/fixtures.toml");
        let root_value: Value = fixtures_toml.parse().expect("Failed to parse TOML");

        let classes_set: HashSet<String> = root_value
            .get("modules")
            .and_then(|modules| modules.get("java.base"))
            .and_then(|java_base| java_base.get("classes"))
            .and_then(|classes_value| classes_value.as_array())
            .map(|classes_array| {
                classes_array
                    .iter()
                    .filter_map(|val| val.as_str())
                    .map(|v| v.replace('.', "/"))
                    .collect()
            })
            .unwrap();

        classes_set
    }
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
                        index.entry(bin_name).or_insert(ClassSource {
                            jmod_path: path.clone(),
                            entry_name: name,
                        });
                    }
                }
            }
        }

        debug!("Index prepared. Found {} classes.", index.len());

        //TODO: remove afterwards
        let tested_parsing = Self::prepare_tested_classes();

        Ok(Self {
            index,
            tested_parsing,
        })
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
    pub(crate) fn find_class(&self, binary_name: &str) -> Result<Vec<u8>, ClassLoaderErr> {
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

        assert!(
            self.tested_parsing.contains(binary_name),
            "Class \"{}\" is not tested",
            binary_name
        );
        Ok(buf)
    }
}
