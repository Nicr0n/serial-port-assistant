use crate::gui::types::serialport::Serialport;

pub fn get_serialport_list() -> Result<Vec<Serialport>, serialport::Error> {
    match serialport::available_ports() {
        Ok(serialport_info_list) => {
            let mut serialport_list: Vec<Serialport> = Vec::new();
            for serialport_info in serialport_info_list {
                println!("{:?}", serialport_info);
                serialport_list.push(Serialport::from_serial_port_info(serialport_info))
            }
            Ok(serialport_list)
        }
        Err(e) => return Err(e),
    }
}
