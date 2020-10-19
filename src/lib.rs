

pub use hidapi::{HidApi, HidDevice};

use structopt::StructOpt;

#[derive(Clone, PartialEq, Debug, StructOpt)]
#[structopt(name = "hidpal", about = "USB HID device helper")]
pub struct Filter {
    #[structopt(long, default_value="1209", parse(try_from_str=u16_parse_hex), env="USB_VID")]
    /// USB Device Vendor ID (VID) in hex
    pub vid: u16,

    #[structopt(long, default_value="fff0", parse(try_from_str=u16_parse_hex), env="USB_PID")]
    /// USB Device Product ID (PID) in hex
    pub pid: u16,

    #[structopt(long, env = "USB_SERIAL")]
    /// USB Device Serial
    pub serial: Option<String>,
}


pub fn u16_parse_hex(s: &str) -> Result<u16, std::num::ParseIntError> {
    u16::from_str_radix(s, 16)
}

pub fn u8_parse_hex(s: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(s, 16)
}

#[derive(Clone, PartialEq, Debug)]
pub struct Info {
    manufacturer: Option<String>,
    product: Option<String>,
    serial: Option<String>,
}

pub trait Device {
    fn connect(vid: u16, pid: u16, serial: Option<&str>) -> Result<Self, anyhow::Error> where Self: Sized;
    fn info(&mut self) -> Result<Info, anyhow::Error>;
}

impl Device for HidDevice {
    /// Connect to an HID device using vid/pid(/serial)
    fn connect(vid: u16, pid: u16, serial: Option<&str>) -> Result<Self, anyhow::Error> {
        // Create new HID API instance
        let api = HidApi::new()?;

        // Attempt to connect to device
        let hid_device = match &serial {
            Some(s) => api.open_serial(vid, pid, s),
            None => api.open(vid, pid),
        }?;

        Ok(hid_device)
    }

    /// Fetch information for the connected device
    fn info(&mut self) -> Result<Info, anyhow::Error> {
        let manufacturer = self.get_manufacturer_string()?;
        let product = self.get_product_string()?;
        let serial = self.get_serial_number_string()?;

        Ok(Info{ manufacturer, product, serial })
    }
}