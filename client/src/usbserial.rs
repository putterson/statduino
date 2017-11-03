extern crate libusb;

use self::libusb::Context;
use self::libusb::DeviceHandle;
extern crate owning_ref;
use std;

pub struct USBSerial {
    context : Context,
}

pub struct USBSerialConnection<'a> {
    handle : DeviceHandle<'a>,
}

impl USBSerial {
    pub fn new() -> USBSerial {
        let context = libusb::Context::new().unwrap();
        list_devices(&context);
        return USBSerial {
            context,
        }
    }

    pub fn connect(&self, vid : u16, pid : u16) -> USBSerialConnection {
        println!("Opening {:04x}:{:04x}", vid, pid);
        let mut handle = self.context.open_device_with_vid_pid(vid, pid).unwrap();
        let interface : u8 = 1;

        if handle.kernel_driver_active(interface).unwrap() {
            println!("Detaching kernel driver");
            handle.detach_kernel_driver(interface).unwrap();
        }
        println!("Claiming interface {}", interface);
        handle.claim_interface(interface).unwrap();

        //controlTransfer(int requestType, int request, int value, int index, byte[] buffer, int offset, int length, int timeout) 

        //request type 0x40
        // let init_req_type = libusb::request_type(libusb::Direction::Out,libusb::RequestType::Vendor,libusb::Recipient::Device);
 
        return USBSerialConnection {
            handle
        }
    }
}

impl <'a> USBSerialConnection<'a> {
     pub fn write(&self, data : &[u8]) {
        let mut written = 0;
        let size = data.len();
        while written < size {
            let res = self.handle.write_bulk(0x02, &data[written..size], std::time::Duration::new(100,0)).expect("Failed to write to endpoint");
            written += res;
        }
     }
}

fn list_devices(context : &libusb::Context) {
    for device in context.devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        println!("Bus {:03} Device {:03} ID {:04x}:{:04x}",
            device.bus_number(),
            device.address(),
            device_desc.vendor_id(),
            device_desc.product_id());
    }
}