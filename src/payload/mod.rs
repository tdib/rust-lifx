mod set_colour;

use set_colour::{SetColourConfig, SET_COLOUR_CONFIG_SIZE};

#[derive(Debug)]
pub enum Payload {
    SetColour(SetColourConfig<[u8; SET_COLOUR_CONFIG_SIZE]>),
}

impl Payload {
    pub fn get_bytes(&self) -> Vec<u8> {
        match self {
            Self::SetColour(config) => config.0.to_vec(),
        }
    }

    pub fn set_colour(
        hue: u16,
        saturation: u16,
        brightness: u16,
        kelvin: u16,
        duration: u32,
    ) -> Payload {
        Self::SetColour({
            let mut config = SetColourConfig::default();

            config.set_hue(hue);
            config.set_saturation(saturation);
            config.set_brightness(brightness);
            config.set_kelvin(kelvin);
            config.set_duration(duration);

            config
        })
    }

    pub fn get_size(&self) -> usize {
        match self {
            Self::SetColour(config) => SET_COLOUR_CONFIG_SIZE,
        }
    }
}
