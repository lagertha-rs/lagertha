use crate::error::JvmError;
use crate::heap::Heap;
use crate::heap::method_area::MethodArea;
use crate::interpreter::Interpreter;
use crate::native::NativeRegistry;
use crate::rt::class::Class;
use crate::stack::FrameStack;
use common::jtype::{HeapAddr, Value};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use tracing_log::log::debug;

mod class_loader;
pub mod error;
mod heap;
mod interpreter;
mod native;
pub mod rt;
pub mod stack;

pub type ClassId = usize;
pub type MethodId = usize;

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

pub struct VirtualMachine {
    config: VmConfig,
    heap: Rc<RefCell<Heap>>,
    native_registry: NativeRegistry,
    frame_stack: FrameStack,
}

impl VirtualMachine {
    pub fn new(config: VmConfig) -> Result<Self, JvmError> {
        config.validate();
        let method_area = MethodArea::new(&config)?;
        let heap = Rc::new(RefCell::new(Heap::new(method_area)));

        let native_registry = NativeRegistry::new();
        Ok(Self {
            frame_stack: FrameStack::new(&config),
            config,
            heap,
            native_registry,
        })
    }

    pub fn heap(&self) -> Rc<RefCell<Heap>> {
        self.heap.clone()
    }

    pub fn get_mirror_addr_by_name(&mut self, name: &str) -> Result<HeapAddr, JvmError> {
        self.method_area.get_mirror_addr_by_name(name, &self.heap)
    }

    pub fn get_mirror_addr_by_class(
        &mut self,
        target_class: &Arc<Class>,
    ) -> Result<HeapAddr, JvmError> {
        self.method_area
            .get_mirror_addr_by_class(target_class, &self.heap)
    }

    pub fn get_primitive_mirror_addr(&mut self, name: &HeapAddr) -> HeapAddr {
        self.method_area.get_primitive_mirror_addr(name, &self.heap)
    }
}

pub fn start(main_class: Vec<u8>, config: VmConfig) -> Result<(), JvmError> {
    debug!("Starting VM with config: {:?}", config);

    let vm = VirtualMachine::new(config)?;

    let mut interpreter = Interpreter::new(vm);
    match interpreter.start(main_class) {
        Ok(_) => {
            debug!("VM execution finished successfully");
            Ok(())
        }
        Err(e) => match e {
            JvmError::JavaExceptionThrown(addr) => {
                let heap = interpreter.vm().heap.borrow();
                let exception = heap.get_instance(&addr);

                let print_stack_trace_method = exception
                    .class()
                    .get_virtual_method("printStackTrace", "()V")
                    .expect("Exception class should have printStackTrace method")
                    .clone();
                let param = vec![Value::Ref(addr)];
                drop(heap);
                interpreter.run_instance_method(&print_stack_trace_method, param)?;
                Err(e)
            }
            _ => {
                eprintln!("VM execution failed with error: {:?}", e);
                Err(e)
            }
        },
    }
}

#[cfg(test)]
impl serde::Serialize for VirtualMachine {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        use std::ops::Deref;

        let mut state = serializer.serialize_struct("VirtualMachine", 3)?;
        state.serialize_field("heap", self.heap.borrow().deref())?;
        state.end()
    }
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
        #[base_dir = "../target/test-classes/runtime"]
        #[files("**/*Main.class")]
        path: PathBuf,
    ) {
        // Given
        let bytes = fs::read(&path).unwrap_or_else(|_| panic!("Can't read file {:?}", path));
        let mut base_config = vm_config();
        base_config.class_path.push(main_base_package_dir(&path));
        let vm = VirtualMachine::new(base_config).unwrap();
        let mut interpreter = Interpreter::new(vm);

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
