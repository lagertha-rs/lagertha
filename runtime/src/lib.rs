use crate::class_loader::ClassLoaderErr;
use crate::method_area::MethodArea;
use crate::rt::class::class::Class;
use crate::rt::class::LoadingError;
use crate::rt::constant_pool::error::RuntimePoolError;
use class_file::error::ClassFileErr;
use common::utils::cursor::CursorError;
use common::InstructionErr;
use std::rc::Rc;
use thiserror::Error;

mod class_loader;
mod heap;
mod method_area;
mod native_registry;
pub mod rt;
mod stack;
mod string_pool;

type HeapAddr = usize;

#[derive(Clone, Debug)]
pub enum Value {
    Null,
    Ref(HeapAddr),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ClassId(pub usize);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct MethodId(pub usize);

#[derive(Debug, Eq, PartialEq)]
pub enum ClassState {
    Loaded,
    Initialized,
}

#[derive(Debug, Error)]
pub enum JvmError {
    #[error(transparent)]
    Loading(#[from] LoadingError),
    #[error(transparent)]
    ClassFile(#[from] ClassFileErr),
    #[error(transparent)]
    Cursor(#[from] CursorError),
    #[error(transparent)]
    RuntimePool(#[from] RuntimePoolError),
    #[error(transparent)]
    ClassLoader(#[from] ClassLoaderErr),
    #[error("")]
    MissingAttributeInConstantPoll,
    #[error("")]
    ConstantNotFoundInRuntimePool,
    #[error("")]
    TrailingBytes,
    #[error("")]
    TypeError,
}

// TODO: returns only class right now, in future not sure
pub fn parse_bin_class(main_class: Vec<u8>) -> Result<Rc<Class>, JvmError> {
    //let class_file = ClassFile::try_from(main_class)?;
    //let main = Rc::new(Class::new(class_file)?);

    //let method_area = MethodArea::with_main(main.clone())?;
    let method_area = MethodArea::new()?;

    let obj_class_name = "java/lang/Object".to_string();
    let obj_class = method_area.get_class(&obj_class_name)?;

    Ok(obj_class)
}
