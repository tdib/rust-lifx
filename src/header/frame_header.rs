use bitfield::bitfield;

pub const FRAME_HEADER_SIZE: usize = 8;
bitfield! {
    pub struct FrameHeader([u8]);
    impl Debug;
    u8;
    // Bytes 0 and 1
    pub u16, size, set_size: 15, 0;// 36 bytes + payload

    // Bytes 2 and 3
    pub u16, protocol, set_protocol: 27, 16; // Always 1024
    pub addressable, set_addressable: 28; // Always true
    pub tagged, set_tagged: 29; // True if we are broadcasting to multiple lights
    pub u8, origin, set_origin: 31, 30; // Always 0

    // Byte 4-7
    pub u32, source, set_source: 63, 32;// Unique value set by client for responses
}

impl Default for FrameHeader<[u8; 8]> {
    fn default() -> Self {
        let mut frame_header = Self(Default::default());
        // Hard set values
        frame_header.set_origin(0);
        frame_header.set_addressable(true);
        frame_header.set_protocol(1024);

        frame_header
    }
}

impl FrameHeader<[u8; 8]> {
    pub fn new(tagged: bool, size: u16, source: u32) -> Self {
        let mut frame_header = Self::default();

        frame_header.set_tagged(tagged);
        frame_header.set_size(size);
        frame_header.set_source(source);

        frame_header
    }
}
