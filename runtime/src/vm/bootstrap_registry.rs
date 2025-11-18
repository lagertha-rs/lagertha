use crate::{ClassId, FieldKey, MethodKey, Symbol};
use common::error::JvmError;
use common::jtype::PrimitiveType;
use lasso::ThreadedRodeo;
use std::cell::OnceCell;

pub struct BootstrapRegistry {
    // Common method keys
    pub clinit_mk: MethodKey,
    pub no_arg_constructor_mk: MethodKey,
    pub main_mk: MethodKey,
    pub system_init_phase1_mk: MethodKey,
    pub print_stack_trace_mk: MethodKey,
    pub thread_group_parent_and_name_constructor_mk: MethodKey,
    pub thread_thread_group_and_name_constructor_mk: MethodKey,
    pub thread_group_uncaught_exception_mk: MethodKey,
    pub thread_get_thread_group_mk: MethodKey,

    // Common field keys
    pub class_name_fk: FieldKey,
    pub system_out_fk: FieldKey,
    pub system_err_fk: FieldKey,
    pub file_output_stream_fd_fk: FieldKey,
    pub fd_fd_fk: FieldKey,
    pub throwable_backtrace_fk: FieldKey,
    pub throwable_depth_fk: FieldKey,
    pub stack_trace_declaring_class_fk: FieldKey,
    pub stack_trace_method_name_fk: FieldKey,
    pub stack_trace_file_name_fk: FieldKey,
    pub stack_trace_line_number_fk: FieldKey,
    pub stack_trace_declaring_class_name_fk: FieldKey,

    // Common class names (interned)
    pub java_lang_object_sym: Symbol,
    pub java_lang_class_sym: Symbol,
    pub java_lang_throwable_sym: Symbol,
    pub java_lang_string_sym: Symbol,
    pub java_lang_system_sym: Symbol,
    pub java_lang_thread_sym: Symbol,
    pub java_lang_thread_group_sym: Symbol,

    // Primitive name symbols
    pub int_sym: Symbol,
    pub byte_sym: Symbol,
    pub short_sym: Symbol,
    pub long_sym: Symbol,
    pub float_sym: Symbol,
    pub double_sym: Symbol,
    pub char_sym: Symbol,
    pub boolean_sym: Symbol,
    pub void_sym: Symbol,

    // Common method names (interned)
    pub init_sym: Symbol,
    pub clinit_sym: Symbol,
    pub main_sym: Symbol,
    pub arraycopy_sym: Symbol,

    // Common descriptors (interned)
    pub void_desc: Symbol,         // ()V
    pub string_desc: Symbol,       // Ljava/lang/String;
    pub object_desc: Symbol,       // Ljava/lang/Object;
    pub class_desc: Symbol,        // Ljava/lang/Class;
    pub string_array_desc: Symbol, // [Ljava/lang/String;
    pub char_array_desc: Symbol,   // [C
    pub int_array_desc: Symbol,    // [I
    pub int_desc: Symbol,          // I

    // core classes IDs
    java_lang_class_id: OnceCell<ClassId>,
    java_lang_object_id: OnceCell<ClassId>,
    java_lang_throwable_id: OnceCell<ClassId>,
    java_lang_thread_group_id: OnceCell<ClassId>,
    java_lang_thread_id: OnceCell<ClassId>,
    java_lang_string_id: OnceCell<ClassId>,
    char_array_class_id: OnceCell<ClassId>,
}

impl BootstrapRegistry {
    pub fn new(interner: &ThreadedRodeo) -> Self {
        // Method names
        let clinit_sym = interner.get_or_intern("<clinit>");
        let init_sym = interner.get_or_intern("<init>");
        let main_sym = interner.get_or_intern("main");

        // Common descriptors
        let void_desc = interner.get_or_intern("()V");
        let string_desc = interner.get_or_intern("Ljava/lang/String;");
        let object_desc = interner.get_or_intern("Ljava/lang/Object;");
        let class_desc = interner.get_or_intern("Ljava/lang/Class;");
        let string_array_desc = interner.get_or_intern("[Ljava/lang/String;");
        let char_array_desc = interner.get_or_intern("[C");
        let int_desc = interner.get_or_intern("I");
        let desc_print_stream_sym = interner.get_or_intern("Ljava/io/PrintStream;");

        // Primitive type names
        let int_sym = interner.get_or_intern("int");
        let byte_sym = interner.get_or_intern("byte");
        let short_sym = interner.get_or_intern("short");
        let long_sym = interner.get_or_intern("long");
        let float_sym = interner.get_or_intern("float");
        let double_sym = interner.get_or_intern("double");
        let char_sym = interner.get_or_intern("char");
        let boolean_sym = interner.get_or_intern("boolean");
        let void_sym = interner.get_or_intern("void");

        // Field names
        let name_field = interner.get_or_intern("name");

        Self {
            // Method keys
            clinit_mk: MethodKey {
                name: clinit_sym,
                desc: void_desc,
            },
            no_arg_constructor_mk: MethodKey {
                name: init_sym,
                desc: void_desc,
            },
            main_mk: MethodKey {
                name: main_sym,
                desc: interner.get_or_intern("([Ljava/lang/String;)V"),
            },
            system_init_phase1_mk: MethodKey {
                name: interner.get_or_intern("initPhase1"),
                desc: void_desc,
            },
            print_stack_trace_mk: MethodKey {
                name: interner.get_or_intern("printStackTrace"),
                desc: void_desc,
            },
            thread_group_parent_and_name_constructor_mk: MethodKey {
                name: init_sym,
                desc: interner.get_or_intern("(Ljava/lang/ThreadGroup;Ljava/lang/String;)V"),
            },
            thread_thread_group_and_name_constructor_mk: MethodKey {
                name: init_sym,
                desc: interner.get_or_intern("(Ljava/lang/ThreadGroup;Ljava/lang/String;)V"),
            },
            thread_group_uncaught_exception_mk: MethodKey {
                name: interner.get_or_intern("uncaughtException"),
                desc: interner.get_or_intern("(Ljava/lang/Thread;Ljava/lang/Throwable;)V"),
            },
            thread_get_thread_group_mk: MethodKey {
                name: interner.get_or_intern("getThreadGroup"),
                desc: interner.get_or_intern("()Ljava/lang/ThreadGroup;"),
            },

            // Field keys
            class_name_fk: FieldKey {
                name: name_field,
                desc: string_desc,
            },
            throwable_backtrace_fk: FieldKey {
                name: interner.get_or_intern("backtrace"),
                desc: object_desc,
            },
            throwable_depth_fk: FieldKey {
                name: interner.get_or_intern("depth"),
                desc: int_desc,
            },
            system_out_fk: FieldKey {
                name: interner.get_or_intern("out"),
                desc: desc_print_stream_sym,
            },
            system_err_fk: FieldKey {
                name: interner.get_or_intern("err"),
                desc: desc_print_stream_sym,
            },
            file_output_stream_fd_fk: FieldKey {
                name: interner.get_or_intern("fd"),
                desc: interner.get_or_intern("Ljava/io/FileDescriptor;"),
            },
            fd_fd_fk: FieldKey {
                name: interner.get_or_intern("fd"),
                desc: int_desc,
            },
            stack_trace_declaring_class_fk: FieldKey {
                name: interner.get_or_intern("declaringClassObject"),
                desc: class_desc,
            },
            stack_trace_method_name_fk: FieldKey {
                name: interner.get_or_intern("methodName"),
                desc: string_desc,
            },
            stack_trace_file_name_fk: FieldKey {
                name: interner.get_or_intern("fileName"),
                desc: string_desc,
            },
            stack_trace_line_number_fk: FieldKey {
                name: interner.get_or_intern("lineNumber"),
                desc: int_desc,
            },
            stack_trace_declaring_class_name_fk: FieldKey {
                name: interner.get_or_intern("declaringClass"),
                desc: string_desc,
            },

            // Class names
            java_lang_object_sym: interner.get_or_intern("java/lang/Object"),
            java_lang_class_sym: interner.get_or_intern("java/lang/Class"),
            java_lang_throwable_sym: interner.get_or_intern("java/lang/Throwable"),
            java_lang_string_sym: interner.get_or_intern("java/lang/String"),
            java_lang_system_sym: interner.get_or_intern("java/lang/System"),
            java_lang_thread_sym: interner.get_or_intern("java/lang/Thread"),
            java_lang_thread_group_sym: interner.get_or_intern("java/lang/ThreadGroup"),

            // Method names
            init_sym,
            clinit_sym,
            main_sym,
            arraycopy_sym: interner.get_or_intern("arraycopy"),

            // Descriptors
            void_desc,
            string_desc,
            object_desc,
            class_desc,
            string_array_desc,
            char_array_desc,
            int_desc,
            int_array_desc: interner.get_or_intern("[I"),

            // Primitive names
            int_sym,
            byte_sym,
            short_sym,
            long_sym,
            float_sym,
            double_sym,
            char_sym,
            boolean_sym,
            void_sym,

            // Class IDs
            java_lang_class_id: OnceCell::new(),
            java_lang_object_id: OnceCell::new(),
            java_lang_throwable_id: OnceCell::new(),
            java_lang_thread_group_id: OnceCell::new(),
            java_lang_thread_id: OnceCell::new(),
            java_lang_string_id: OnceCell::new(),
            char_array_class_id: OnceCell::new(),
        }
    }

    pub fn set_java_lang_class_id(&self, class_id: ClassId) -> Result<(), JvmError> {
        self.java_lang_class_id
            .set(class_id)
            .map_err(|_| JvmError::Todo("java/lang/Class ID already set".to_string()))
    }

    pub fn set_java_lang_object_id(&self, class_id: ClassId) -> Result<(), JvmError> {
        self.java_lang_object_id
            .set(class_id)
            .map_err(|_| JvmError::Todo("java/lang/Object ID already set".to_string()))
    }

    pub fn set_java_lang_throwable_id(&self, class_id: ClassId) -> Result<(), JvmError> {
        self.java_lang_throwable_id
            .set(class_id)
            .map_err(|_| JvmError::Todo("java/lang/Throwable ID already set".to_string()))
    }

    pub fn set_java_lang_thread_group_id(&self, class_id: ClassId) -> Result<(), JvmError> {
        self.java_lang_thread_group_id
            .set(class_id)
            .map_err(|_| JvmError::Todo("java/lang/ThreadGroup ID already set".to_string()))
    }

    pub fn set_java_lang_thread_id(&self, class_id: ClassId) -> Result<(), JvmError> {
        self.java_lang_thread_id
            .set(class_id)
            .map_err(|_| JvmError::Todo("java/lang/Thread ID already set".to_string()))
    }

    pub fn set_java_lang_string_id(&self, class_id: ClassId) -> Result<(), JvmError> {
        self.java_lang_string_id
            .set(class_id)
            .map_err(|_| JvmError::Todo("java/lang/String ID already set".to_string()))
    }

    pub fn set_char_array_class_id(&self, class_id: ClassId) -> Result<(), JvmError> {
        self.char_array_class_id
            .set(class_id)
            .map_err(|_| JvmError::Todo("[C class ID already set".to_string()))
    }

    pub fn get_java_lang_string_id(&self) -> Result<ClassId, JvmError> {
        self.java_lang_string_id
            .get()
            .copied()
            .ok_or_else(|| JvmError::Todo("java/lang/String".to_string()))
    }

    pub fn get_char_array_class_id(&self) -> Result<ClassId, JvmError> {
        self.char_array_class_id
            .get()
            .copied()
            .ok_or_else(|| JvmError::Todo("[C".to_string()))
    }

    pub fn get_java_lang_thread_group_id(&self) -> Result<ClassId, JvmError> {
        self.java_lang_thread_group_id
            .get()
            .copied()
            .ok_or_else(|| JvmError::Todo("java/lang/ThreadGroup".to_string()))
    }

    pub fn get_java_lang_thread_id(&self) -> Result<ClassId, JvmError> {
        self.java_lang_thread_id
            .get()
            .copied()
            .ok_or_else(|| JvmError::Todo("java/lang/Thread".to_string()))
    }

    pub fn get_java_lang_throwable_id(&self) -> Result<ClassId, JvmError> {
        self.java_lang_throwable_id
            .get()
            .copied()
            .ok_or_else(|| JvmError::Todo("java/lang/Throwable".to_string()))
    }

    pub fn get_java_lang_class_id(&self) -> Result<ClassId, JvmError> {
        self.java_lang_class_id
            .get()
            .copied()
            .ok_or_else(|| JvmError::Todo("java/lang/Class".to_string()))
    }

    pub fn get_java_lang_object_id(&self) -> Result<ClassId, JvmError> {
        self.java_lang_object_id
            .get()
            .copied()
            .ok_or_else(|| JvmError::Todo("java/lang/Object".to_string()))
    }

    pub fn get_primitive_sym(&self, primitive: &PrimitiveType) -> Symbol {
        match primitive {
            PrimitiveType::Int => self.int_sym,
            PrimitiveType::Byte => self.byte_sym,
            PrimitiveType::Short => self.short_sym,
            PrimitiveType::Long => self.long_sym,
            PrimitiveType::Float => self.float_sym,
            PrimitiveType::Double => self.double_sym,
            PrimitiveType::Char => self.char_sym,
            PrimitiveType::Boolean => self.boolean_sym,
        }
    }
}
