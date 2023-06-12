use bitfield::bitfield;

pub const PROTOCOL_HEADER_SIZE: usize = 12;
bitfield! {
    #[derive(Debug)]
    pub struct ProtocolHeader([u8]);
    u64;

    // Bytes 0-8
    reserved, _: 63, 0;

    // Bytes 9-10
    u16, protocol_type, set_protocol_type: 79, 64; // Protocol type, determines payload layout

    // Bytes 11-12
    reserved_2, _: 96, 80;
}

impl Default for ProtocolHeader<[u8; 12]> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl ProtocolHeader<[u8; 12]> {
    pub fn new(protocol_type: u16) -> Self {
        let mut protocol_header = Self::default();

        protocol_header.set_protocol_type(protocol_type);

        protocol_header
    }
}
