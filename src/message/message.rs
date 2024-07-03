use crate::utils::linux::usb_sniffer;

#[derive(Debug, Clone)]
pub enum SerialPortAssistantMessage {
    Echo(usb_sniffer::Event),
}
