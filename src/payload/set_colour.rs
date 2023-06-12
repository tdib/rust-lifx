use bitfield::bitfield;

pub const SET_COLOUR_CONFIG_SIZE: usize = 13;

bitfield! {
    #[derive(Debug, Default)]
    pub struct SetColourConfig(MSB0 [u8]);
    u16;

    // Bits 0-7 are reserved

    pub u16, hue, set_hue: 23, 8;
    pub u16, saturation, set_saturation: 39, 24;
    pub u16, brightness, set_brightness: 55, 40;
    pub u16, kelvin, set_kelvin: 71, 56;

    pub u32, duration, set_duration: 103, 72;
}
