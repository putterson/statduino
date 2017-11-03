extern crate toml;
#[macro_use]
extern crate serde_derive;

use std::io;
mod usbserial;

#[derive(Deserialize)]
struct Config {
    //TODO create a deserializer for u16 that takes a hex string
    vendor_id: String,
    product_id: String,
}

fn read_config() -> Result<Config, io::Error> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open("config.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;


    let config : Config = toml::from_str(&contents).unwrap();

    Ok(config)
}

fn main() {
    let config = read_config().expect("Failed to read configuration file");
    let product_id = u16::from_str_radix(&config.product_id, 16).unwrap();
    let vendor_id = u16::from_str_radix(&config.vendor_id, 16).unwrap();

    let usbserial = usbserial::USBSerial::new();
    let connection = usbserial.connect(vendor_id, product_id);

    let build_command = vec![0x42];//'B'
    let failure_command = vec![0x46];//'F'
    let success_command = vec![0x53];//'S'
    let alarm_command = vec![0x41];//'A'
    


    use std::{thread, time};

    let onepointfive = time::Duration::from_millis(1500);

    loop {
        connection.write(&success_command);
        thread::sleep(onepointfive);
        connection.write(&build_command);
        thread::sleep(onepointfive);
        connection.write(&failure_command);
        connection.write(&alarm_command);
        thread::sleep(onepointfive);
        thread::sleep(onepointfive);
        connection.write(&build_command);
        thread::sleep(onepointfive);
    }
}
