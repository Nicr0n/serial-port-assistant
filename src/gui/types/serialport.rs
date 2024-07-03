use serialport::SerialPortInfo;

use super::serialport_meta::SerialportMeta;
#[derive(Clone, Debug)]
pub struct Serialport {
    pub portname: String,
    pub metainfo: SerialportMeta,
}

/**
 * retrun wrapped SerialPort Info
 */
impl Serialport {
    pub fn from_serial_port_info(serialport_info: SerialPortInfo) -> Serialport {
        let serialport_meta: SerialportMeta = match serialport_info.port_type {
            serialport::SerialPortType::UsbPort(usb_port_info) => {
                SerialportMeta::new(String::from("UsbPort"), Some(usb_port_info))
            }
            serialport::SerialPortType::PciPort => {
                SerialportMeta::new(String::from("PciPort"), None)
            }
            serialport::SerialPortType::BluetoothPort => {
                SerialportMeta::new(String::from("BluetoothPort"), None)
            }
            serialport::SerialPortType::Unknown => {
                SerialportMeta::new(String::from("Unknown"), None)
            }
        };
        Serialport {
            portname: serialport_info.port_name,
            metainfo: serialport_meta,
        }
    }
}
