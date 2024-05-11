mod parser;
mod constant_pool;
mod attribute;

use std::{fs};
use std::io::{Error, ErrorKind};
use crate::constant_pool::{ConstantInfo, read_cp_info};
use crate::parser::ByteParser;

#[derive(Debug)]
struct ClassHeader {
    minor_version: u16,
    major_version: u16,
    constant_pool: Vec<ConstantInfo>
}

fn main() -> Result<(), Error> {
    let bytes = fs::read("./src/Main.class")?;

    let mut parser = ByteParser::new(&bytes);
    let magic = parser.read_u32();

    if magic != 0xcafebabe {
        return Err(Error::new(ErrorKind::InvalidData, "magic bytes are invalid"));
    }

    let minor_version = parser.read_u16();
    let major_version = parser.read_u16();

    let constant_pool_size = parser.read_u16() - 1;
    let mut constant_pool = Vec::with_capacity(constant_pool_size as usize);
    for _ in 0..constant_pool_size {
        constant_pool.push(read_cp_info(&mut parser));
    }

    let class_header = ClassHeader {
        minor_version,
        major_version,
        constant_pool
    };
    println!("class: {:?}", class_header);
    Ok(())
}



