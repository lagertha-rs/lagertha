use crate::class_loader::ClassLoaderErr;
use crate::interpreter::Interpreter;
use crate::method_area::MethodArea;
use crate::rt::class::LinkageError;
use crate::rt::constant_pool::error::RuntimePoolError;
use common::utils::cursor::CursorError;
use std::sync::Arc;
use thiserror::Error;
use tracing_log::log::debug;

mod class_loader;
mod heap;
mod interpreter;
mod method_area;
mod native;
pub mod rt;
pub mod stack;
mod string_pool;

//TODO: avoid string allocations here
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct MethodKey {
    pub class: String,
    pub name: String,
    pub desc: String,
}

impl MethodKey {
    pub fn new(class: String, name: String, desc: String) -> Self {
        Self { class, name, desc }
    }
}

#[derive(Debug, Error)]
pub enum JvmError {
    #[error("LinkageError: {0}")]
    Linkage(#[from] LinkageError),
    #[error(transparent)]
    Cursor(#[from] CursorError),
    #[error("RuntimeConstantPoolError: {0}")]
    RuntimePool(#[from] RuntimePoolError),
    #[error(transparent)]
    ClassLoader(#[from] ClassLoaderErr),
    #[error("MissingAttributeInConstantPoll")]
    MissingAttributeInConstantPoll,
    #[error("ConstantNotFoundInRuntimePool")]
    ConstantNotFoundInRuntimePool,
    #[error("TrailingBytes")]
    TrailingBytes,
    #[error("ClassNotFoundException: {0}")]
    ClassNotFound(String),
    #[error("stack overflow")]
    StackOverflow,
    #[error("Frame stack is empty")]
    FrameStackIsEmpty,
    #[error("Operand stack is empty")]
    OperandStackIsEmpty,
    #[error("OutOfMemory")]
    OutOfMemory,
    #[error("Could not find or load main class {0}")]
    NoMainClassFound(String),
    #[error("NoSuchMethod: {0}")]
    NoSuchMethod(String),
    #[error("NoSuchField: {0}")]
    FieldNotFound(String),
    #[error("LocalVariableNotFound: {0}")]
    LocalVariableNotFound(u8),
    #[error("LocalVariableNotInitialized: {0}")]
    LocalVariableNotInitialized(u8),
    #[error("TypeDescriptorErr: {0}")]
    TypeDescriptorErr(#[from] common::TypeDescriptorErr),
    #[error("NullPointerException")]
    NullPointerException,
    #[error("InstructionErr: {0}")]
    InstructionErr(#[from] common::InstructionErr),
    #[error("ClassMirrorIsAlreadyCreated")]
    ClassMirrorIsAlreadyCreated,
    #[error("NegativeArraySizeException")]
    NegativeArraySizeException,
    #[error("ArrayIndexOutOfBoundsException")]
    ArrayIndexOutOfBoundsException,
    #[error("Method is not expecting to be abstract `{0}`")]
    MethodIsAbstract(String),
}

#[derive(Debug)]
pub struct VmConfig {
    pub home: String,
    pub version: String,
    pub class_path: Vec<String>,
    pub initial_heap_size: usize,
    pub max_heap_size: usize,
    pub frame_stack_size: usize,
    pub operand_stack_size: usize,
}

//TODO: make it better
impl VmConfig {
    pub fn validate(&self) {
        if self.version != "24.0.2" {
            panic!(
                "Unsupported Java version: {}. Only 24.0.2 is supported.",
                self.version
            );
        }
    }
}

pub fn start(main_class: Vec<u8>, config: VmConfig) -> Result<(), JvmError> {
    debug!("Starting VM with config: {:?}", config);
    config.validate();

    let vm_config = Arc::new(config);
    let method_area = MethodArea::new(vm_config.clone())?;

    let mut interpreter = Interpreter::new(&vm_config, method_area);
    interpreter.start(main_class)
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::with_settings;
    use rstest::rstest;
    use std::fs;
    use std::path::{Path, PathBuf};

    const DISPLAY_SNAPSHOT_PATH: &str = "../snapshots";

    fn to_snapshot_name(path: &Path) -> String {
        let mut iter = path.iter().map(|s| s.to_string_lossy().to_string());
        for seg in iter.by_ref() {
            if seg == "test-classes" {
                break;
            }
        }
        let tail: Vec<String> = iter.collect();
        tail.join("-")
    }

    fn vm_config() -> VmConfig {
        let java_home = std::env::var("JAVA_HOME").expect("JAVA_HOME must be set for tests");

        VmConfig {
            home: java_home,
            version: "24.0.2".to_string(),
            class_path: vec![],
            initial_heap_size: 16 * 1024 * 1024,
            max_heap_size: 64 * 1024 * 1024,
            frame_stack_size: 256,
            operand_stack_size: 256,
        }
    }

    fn main_base_package_dir(path: &PathBuf) -> String {
        let mut dir = path.parent().expect("file should have a parent dir");

        dir = dir.parent().expect("should have parent dir");
        dir = dir.parent().expect("should have parent dir");

        dir.to_string_lossy().to_string()
    }

    #[rstest]
    #[trace]
    // Requires `testdata/compile-fixtures.sh` to be run to generate the .class files
    fn test_display(
        #[base_dir = "../target/test-classes/custom"]
        #[files("**/*Main.class")]
        path: PathBuf,
    ) {
        // Given
        let bytes = fs::read(&path).unwrap_or_else(|_| panic!("Can't read file {:?}", path));
        let mut base_config = vm_config();
        base_config.class_path.push(main_base_package_dir(&path));
        let vm_config = Arc::new(base_config);
        let method_area = MethodArea::new(vm_config.clone()).expect("Failed to create MethodArea");
        let mut interpreter = Interpreter::new(&vm_config, method_area);

        // When
        interpreter
            .start(bytes)
            .unwrap_or_else(|e| panic!("Interpreter failed for {:?} with error: {:?}", path, e));

        // Then
        with_settings!(
            {
                snapshot_path => DISPLAY_SNAPSHOT_PATH,
                prepend_module_to_snapshot => false,
            },
            {
                insta::assert_yaml_snapshot!(to_snapshot_name(&path), &interpreter);
            }
        );
    }
}
