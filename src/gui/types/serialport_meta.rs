use serialport::UsbPortInfo;
#[derive(Clone, Debug)]
pub struct SerialportMeta {
    pub port_type: String,
    pub usb_port_info: Option<UsbPortInfo>,
}
impl SerialportMeta {
    pub fn new(port_type: String, usb_port_info: Option<UsbPortInfo>) -> SerialportMeta {
        SerialportMeta {
            port_type: port_type,
            usb_port_info: usb_port_info,
        }
    }
}
