pub struct Device {
    mac_address: String,
    ip_address: Option<String>,
}

impl Device {
    pub fn new(mac_address: &str, ip_address: Option<String>) -> Self {
        Self {
            mac_address: mac_address.to_string(),
            ip_address,
        }
    }

    pub fn get_info(&self) {
        println!("{}, {:?}", self.mac_address, self.ip_address);
    }
}
