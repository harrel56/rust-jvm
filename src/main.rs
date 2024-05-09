mod parser;

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

    let parser = ByteParser::new(&bytes);

    let magic = u32::from_be_bytes(as_u32_slice(&bytes, 0, 4));
    if magic != 0xcafebabe {
        return Err(Error::new(ErrorKind::InvalidData, "magic bytes are invalid"));
    }

    let minor_ver = u16::from_be_bytes(as_u16_slice(&bytes, 4, 6));
    let major_ver = u16::from_be_bytes(as_u16_slice(&bytes, 6, 8));

    let class_header = ClassHeader {
        minor_version: minor_ver,
        major_version: major_ver,
    };
    println!("class: {:?}", class_header);
    Ok(())
}

fn as_u16_slice(bytes: &[u8], start: usize, end: usize) -> [u8; 2] {
    bytes[start..end].try_into().expect("Invalid range provided")
}

fn as_u32_slice(bytes: &[u8], start: usize, end: usize) -> [u8; 4] {
    bytes[start..end].try_into().expect("Invalid range provided")
}

