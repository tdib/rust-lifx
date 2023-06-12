use bitfield::bitfield;

pub const FRAME_ADDRESS_HEADER_SIZE: usize = 16;
bitfield! {
    #[derive(Debug)]
    pub struct FrameAddressHeader([u8]);
    u64;

    // Bytes 0-7
    u8, target, set_target: 7, 0, 8; // MAC address of target device, or 0 for a broadcast

    // Bytes 8-12
    // Bits 64-111 are reserved

    // Byte 13
    bool, res_required, set_res_required: 112; // True if response is required
    bool, ack_required, set_ack_required: 113; // True if acknowledgement is required
    // Bits 114-119 are reserved

    // Byte 14
    u8, sequence, set_sequence: 127, 120; // Wrap around message sequence number
}

impl Default for FrameAddressHeader<[u8; 16]> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl FrameAddressHeader<[u8; 16]> {
    pub fn new(target: u64, res_required: bool, ack_required: bool, sequence: u8) -> Self {
        let mut frame_address_header = Self::default();

        (target << 16)
            .to_be_bytes()
            .into_iter()
            .enumerate()
            .for_each(|(i, byte)| {
                frame_address_header.set_target(i, byte);
            });
        frame_address_header.set_res_required(res_required);
        frame_address_header.set_ack_required(ack_required);
        frame_address_header.set_sequence(sequence);

        frame_address_header
    }
}
