pub enum Attribute {
    ConstantValue {
        attribute_name_index: u16,
        constantvalue_index: u16
    },
    Code {
        attribute_name_index: u16,
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
        exceptions: Vec<Exception>,
        attributes: Vec<Attribute>
    },
    Exceptions {
        attribute_name_index: u16,
    },
    SourceFile,
    LineNumberTable,
    LocalVariableTable,
    InnerClasses,
    Synthetic,
    Deprecated,
    EnclosingMethod,
    Signature,
    SourceDebugExtension,
    LocalVariableTypeTable,
    RuntimeVisibleAnnotations,
    RuntimeInvisibleAnnotations,
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    AnnotationDefault,
    StackMapTable,
    BootstrapMethods,
    RuntimeVisibleTypeAnnotations,
    RuntimeInvisibleTypeAnnotations,
    MethodParameters,
    Module,
    ModulePackages,
    ModuleMainClass,
    NestHost,
    NestMembers,
    Record,
    PermittedSubclasses
}

pub struct Exception {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16
}
