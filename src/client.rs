pub struct ButtplugClient {
}

pub trait ButtplugClientBase {
    fn process_message() {
    }

    fn get_device_list() {
    }
}

impl ButtplugClient {
    pub fn new() -> ButtplugClient {
        return ButtplugClient{};
    }
}

impl ButtplugClientBase for ButtplugClient {
}