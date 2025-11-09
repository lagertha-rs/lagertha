use crate::{ClassId, FieldKey, MethodKey, Symbol};
use common::error::JvmError;
use common::jtype::PrimitiveType;
use lasso::ThreadedRodeo;
use std::cell::OnceCell;

pub struct BootstrapRegistry {
    // Common method keys
    pub clinit_mk: MethodKey,
    pub init_mk: MethodKey,
    pub main_mk: MethodKey,

    // Common field keys
    pub class_name_fk: FieldKey,
    pub throwable_backtrace_fk: FieldKey,
    pub throwable_depth_fk: FieldKey,

    // Common class names (interned)
    pub java_lang_object_sym: Symbol,
    pub java_lang_class_sym: Symbol,
    pub java_lang_throwable_sym: Symbol,
    pub java_lang_string_sym: Symbol,
    pub java_lang_system_sym: Symbol,

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

    // Common descriptors (interned)
    pub desc_void_sym: Symbol,         // ()V
    pub desc_string_sym: Symbol,       // Ljava/lang/String;
    pub desc_object_sym: Symbol,       // Ljava/lang/Object;
    pub desc_class_sym: Symbol,        // Ljava/lang/Class;
    pub desc_string_array_sym: Symbol, // [Ljava/lang/String;
    pub desc_char_array_sym: Symbol,   // [C
    pub desc_int_array_sym: Symbol,    // [I

    // core classes IDs
    java_lang_class_id: OnceCell<ClassId>,
    java_lang_object_id: OnceCell<ClassId>,
    java_lang_throwable_id: OnceCell<ClassId>,
}

impl BootstrapRegistry {
    pub fn new(interner: &ThreadedRodeo) -> Self {
        // Method names
        let clinit_sym = interner.get_or_intern("<clinit>");
        let init_sym = interner.get_or_intern("<init>");
        let main_sym = interner.get_or_intern("main");

        // Common descriptors
        let desc_void_sym = interner.get_or_intern("()V");
        let desc_string_sym = interner.get_or_intern("Ljava/lang/String;");
        let desc_object_sym = interner.get_or_intern("Ljava/lang/Object;");
        let desc_class_sym = interner.get_or_intern("Ljava/lang/Class;");
        let desc_string_array_sym = interner.get_or_intern("[Ljava/lang/String;");
        let desc_char_array_sym = interner.get_or_intern("[C");

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
                desc: desc_void_sym,
            },
            init_mk: MethodKey {
                name: init_sym,
                desc: desc_void_sym,
            },
            main_mk: MethodKey {
                name: main_sym,
                desc: interner.get_or_intern("([Ljava/lang/String;)V"),
            },

            // Field keys
            class_name_fk: FieldKey {
                name: name_field,
                desc: desc_string_sym,
            },
            throwable_backtrace_fk: FieldKey {
                name: interner.get_or_intern("backtrace"),
                desc: desc_object_sym,
            },
            throwable_depth_fk: FieldKey {
                name: interner.get_or_intern("depth"),
                desc: interner.get_or_intern("I"),
            },

            // Class names
            java_lang_object_sym: interner.get_or_intern("java/lang/Object"),
            java_lang_class_sym: interner.get_or_intern("java/lang/Class"),
            java_lang_throwable_sym: interner.get_or_intern("java/lang/Throwable"),
            java_lang_string_sym: interner.get_or_intern("java/lang/String"),
            java_lang_system_sym: interner.get_or_intern("java/lang/System"),
            init_sym,
            clinit_sym,
            main_sym,

            // Descriptors
            desc_void_sym,
            desc_string_sym,
            desc_object_sym,
            desc_class_sym,
            desc_string_array_sym,
            desc_char_array_sym,
            desc_int_array_sym: interner.get_or_intern("[I"),

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
            PrimitiveType::Void => self.void_sym,
        }
    }
}
