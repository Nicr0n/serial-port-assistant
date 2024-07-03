use iced::widget::{Container, Row, Text};

use crate::message::message::SerialPortAssistantMessage;

pub fn footer() -> Container<'static, SerialPortAssistantMessage> {
    let text = Text::new(String::from("footer"));
    let footer = Row::new().push(text);
    Container::new(footer)
}
