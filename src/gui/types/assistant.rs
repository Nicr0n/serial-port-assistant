use super::serialport::Serialport;

#[derive(Debug, Clone, Default)]
pub struct SerialPortAssistant {
    pub avaliable_port_list: Vec<Serialport>,
}
