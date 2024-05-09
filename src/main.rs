mod parser;
mod constant_pool;

use std::{fs};
use std::io::{Error, ErrorKind, Read};
use crate::parser::ByteParser;

#[derive(Debug)]
struct ClassHeader {
    minor_version: u16,
    major_version: u16
}

fn main() -> Result<(), Error> {
    let bytes = fs::read("./src/Main.class")?;

    let mut parser = ByteParser::new(&bytes);
    let magic = parser.read_u32();

    if magic != 0xcafebabe {
        return Err(Error::new(ErrorKind::InvalidData, "magic bytes are invalid"));
    }

    let minor_ver = parser.read_u16();
    let major_ver = parser.read_u16();

    let constant_pool_size = parser.read_u16();
    for i in 0..(constant_pool_size - 1) {
        read_cp_info(&mut parser);
    }

    let class_header = ClassHeader {
        minor_version: minor_ver,
        major_version: major_ver,
    };
    println!("class: {:?}", class_header);
    Ok(())
}

fn read_cp_info(parser: &mut ByteParser) {
    let tag = parser.read_u8();
    match tag {
        1 => {
            println!("CONSTANT_Utf8");
            let length = parser.read_u16();
            for i in 0..length {
                parser.read_u8();
            }
        }
        3 => {
            println!("CONSTANT_Integer");
            parser.read_u32();
        }
        4 => {
            println!("CONSTANT_Float");
            parser.read_u32();
        }
        5 => {
            println!("CONSTANT_Long");
            parser.read_u32();
            parser.read_u32();
        }
        6 => {
            println!("CONSTANT_Double");
            parser.read_u32();
            parser.read_u32();
        }
        7 => {
            println!("CONSTANT_Class");
            parser.read_u16();
        }
        8 => {
            println!("CONSTANT_String");
            parser.read_u16();
        }
        9 => {
            println!("CONSTANT_Fieldref");
            parser.read_u16();
            parser.read_u16();
        }
        10 => {
            println!("CONSTANT_Methodref");
            parser.read_u16();
            parser.read_u16();
        }
        11 => {
            println!("CONSTANT_InterfaceMethodref");
            parser.read_u16();
            parser.read_u16();
        }
        12 => {
            println!("CONSTANT_NameAndType");
            parser.read_u16();
            parser.read_u16();
        }
        15 => {
            println!("CONSTANT_MethodHandle");
            parser.read_u8();
            parser.read_u16();
        }
        16 => {
            println!("CONSTANT_MethodType");
            parser.read_u16();
        }
        18 => {
            println!("CONSTANT_InvokeDynamic");
            parser.read_u16();
            parser.read_u16();
        }
        _ => panic!("Invalid cp_info.tag [{}]", tag)
    }
}

