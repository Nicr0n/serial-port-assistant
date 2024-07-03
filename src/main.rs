use gui::types::assistant::SerialPortAssistant;
use iced::{Application, Settings};

pub const SERIAL_PORT_ASSISTANT_TITLE: &str = "SPAssistant";

mod gui;
mod message;
mod utils;
fn main() -> iced::Result {
    SerialPortAssistant::run(Settings::default())
}
