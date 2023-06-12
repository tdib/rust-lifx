use crate::header::{
    FrameAddressHeader, FrameHeader, ProtocolHeader, FRAME_ADDRESS_HEADER_SIZE, FRAME_HEADER_SIZE,
    PROTOCOL_HEADER_SIZE,
};
use crate::payload::Payload;

#[derive(Debug)]
pub struct Packet {
    frame_header: FrameHeader<[u8; FRAME_HEADER_SIZE]>,
    frame_address_header: FrameAddressHeader<[u8; FRAME_ADDRESS_HEADER_SIZE]>,
    protocol_header: ProtocolHeader<[u8; PROTOCOL_HEADER_SIZE]>,
    payload: Option<Payload>,
}

impl From<Packet> for Vec<u8> {
    fn from(packet: Packet) -> Self {
        [
            packet.frame_header.0.as_ref(),
            packet.frame_address_header.0.as_ref(),
            packet.protocol_header.0.as_ref(),
            packet.payload.unwrap().get_bytes().as_ref(),
        ]
        .concat()
    }
}

impl Packet {
    // tagged, size, source
    // target, res_required, ack_required, sequence
    // device MAC, res_required, ack_required, sequence, protocol_type
    pub fn new(device_mac: Option<u64>, protocol: u16, payload: Option<Payload>) -> Self {
        let tagged = device_mac.is_none();

        let mut size = FRAME_HEADER_SIZE + FRAME_ADDRESS_HEADER_SIZE + PROTOCOL_HEADER_SIZE;
        if let Some(payload) = &payload {
            size += payload.get_size();
        }
        Self {
            // TODO: allow passing in source
            frame_header: FrameHeader::new(tagged, size as u16, 0),
            // TODO: allow passing in res, ack, and sequence
            frame_address_header: FrameAddressHeader::new(device_mac.unwrap_or(0), false, true, 0),
            protocol_header: ProtocolHeader::new(protocol),
            payload,
        }
    }
}
