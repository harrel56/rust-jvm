const U8_SIZE: usize = 1;
const U16_SIZE: usize = 2;
const U32_SIZE: usize = 4;
const U64_SIZE: usize = 8;

pub struct ByteParser<'a> {
    pos: usize,
    bytes: &'a [u8]
}
impl ByteParser<'_> {
    pub fn new(bytes: &[u8]) -> ByteParser {
        ByteParser { pos: 0, bytes }
    }

    pub fn skip(&mut self, count: usize) {
        self.pos += count;
    }

    pub fn read_u8(&mut self) -> u8 {
        self.pos += U8_SIZE;
        self.bytes[self.pos - U8_SIZE]
    }

    pub fn read_u16(&mut self) -> u16 {
        self.pos += U16_SIZE;
        let mut slice = [0u8; U16_SIZE];
        slice.copy_from_slice(&self.bytes[(self.pos - U16_SIZE)..self.pos]);
        u16::from_be_bytes(slice)
    }

    pub fn read_u32(&mut self) -> u32 {
        self.pos += U32_SIZE;
        let mut slice = [0u8; U32_SIZE];
        slice.copy_from_slice(&self.bytes[(self.pos - U32_SIZE)..self.pos]);
        u32::from_be_bytes(slice)
    }

    pub fn read_u64(&mut self) -> u64 {
        self.pos += U64_SIZE;
        let mut slice = [0u8; U64_SIZE];
        slice.copy_from_slice(&self.bytes[(self.pos - U64_SIZE)..self.pos]);
        u64::from_be_bytes(slice)
    }
}