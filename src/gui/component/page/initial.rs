use iced::{
    widget::{Column, Container},
    Length, Theme,
};

use crate::message::message::SerialPortAssistantMessage;

pub fn intial_page<'a>() -> Container<'a, SerialPortAssistantMessage, Theme> {
    let body = Column::new();

    Container::new(body).height(Length::Fill)
}
