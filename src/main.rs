

use log::{info, error};
use simplelog::{TermLogger, LevelFilter, TerminalMode};
use structopt::StructOpt;

use hid_cli::*;

#[derive(StructOpt)]
#[structopt(name = "hidcli", about = "USB HID device helper")]
struct Options {
    #[structopt(subcommand)]
    command: Command,
    
    #[structopt(flatten)]
    filter: Filter,

    #[structopt(long = "log-level", default_value = "info")]
    /// Enable verbose logging
    level: LevelFilter,
}

#[derive(Clone, PartialEq, Debug)]
struct HexData(Vec<u8>);

impl std::str::FromStr for HexData {
    type Err = hex::FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        hex::decode(s).map(HexData)
    }
}

#[derive(StructOpt)]
enum Command {
    /// Read data from an HID report
    Read{
        #[structopt(parse(try_from_str=u8_parse_hex))]
        /// HID report ID (in hex)
        report_id: u8,

        #[structopt(parse(try_from_str=u8_parse_hex))]
        /// HID report length to read
        length: u8,
    },
    /// Write data to an HID report
    Write{
        #[structopt(parse(try_from_str=u8_parse_hex))]
        /// HID report ID (in hex)
        report_id: u8,

        #[structopt()]
        /// Data to write (in hex)
        data: HexData,
    }
}

fn main() {
    // Parse options
    let opts = Options::from_args();

    // Setup logging
    let mut config = simplelog::ConfigBuilder::new();
    config.set_time_level(LevelFilter::Off);

    TermLogger::init(opts.level, config.build(), TerminalMode::Mixed).unwrap();

    // Connect to HID device
    let d = match HidDevice::connect(opts.filter.vid, opts.filter.pid, opts.filter.serial.as_deref()) {
        Ok(d) => d,
        Err(e) => {
            error!("Error connecting to device {:?}: {:?}", opts.filter, e);
            return;
        }
    };

    // Execute command
    match opts.command {
        Command::Read{report_id, length} => {
            let mut buff = vec![0u8; length as usize + 1];
            buff[0] = report_id;

            match d.get_feature_report(&mut buff) {
                Ok(_) => info!("Read from report_id 0x{:02x}, data: {:02x?} ", report_id, buff),
                Err(e) => error!("Read from report_id 0x{:02x}, error: {:?}", report_id, e),
            }
        },
        Command::Write{report_id, data} => {
            let mut buff = vec![0u8; data.0.len() + 1];
            buff[0] = report_id;
            (&mut buff[1..]).copy_from_slice(&data.0);

            match d.write(&buff) {
                Ok(_) => info!("Write to report_id 0x{:02x}, data: ({:02x?}) OK", report_id, data),
                Err(e) => error!("Write to report_id 0x{:02x}, error: {:?}", report_id, e),
            }
        }
    }

}
