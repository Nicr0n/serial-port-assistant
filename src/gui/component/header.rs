use iced::widget::{Container, Row, Text};

use crate::message::message::SerialPortAssistantMessage;

pub fn header() -> Container<'static, SerialPortAssistantMessage> {
    let text = Text::new(String::from("header"));
    let footer = Row::new().push(text);
    Container::new(footer)
}
