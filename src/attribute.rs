use crate::parser::ByteParser;

#[derive(Debug)]
pub struct Attribute {
    attribute_name_index: u16,
    value: AttributeValue
}

#[derive(Debug)]
pub enum AttributeValue {
    ConstantValue {
        constantvalue_index: u16
    },
    Code {
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
        exceptions: Vec<Exception>,
        attributes: Vec<Attribute>
    },
    Exceptions {
        exception_index_table: Vec<u16>,
    },
    SourceFile {

    },
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

#[derive(Debug)]
pub struct Exception {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16
}

pub fn read_attributes(parser: &mut ByteParser) -> Vec<Attribute> {
    let attributes_count = parser.read_u16();
    for _ in 0..attributes_count {
        let attribute_name_index = parser.read_u16();
        let attribute_length = parser.read_u32();
        parser.skip(attribute_length as usize);
    }
    vec!()
}
