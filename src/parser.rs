struct ByteParser<'a> {
    pos: usize,
    bytes: &'a [u8]
}
impl ByteParser {
    fn new(bytes: &[u8]) -> ByteParser {
        ByteParser { pos: 0, bytes }
    }
}