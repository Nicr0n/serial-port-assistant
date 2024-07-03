use iced::{executor, Application, Command, Theme};
use iced::{widget::Column, Subscription};

use crate::gui::types::assistant::SerialPortAssistant;
use crate::utils::linux::usb_sniffer::load_usb_sniffer;

use crate::message::message::SerialPortAssistantMessage;

use crate::SERIAL_PORT_ASSISTANT_TITLE;

use crate::gui::component::footer::footer;

use super::component::{header::header, page::initial};

impl Application for SerialPortAssistant {
    type Message = SerialPortAssistantMessage;

    type Executor = executor::Default;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: ()) -> (SerialPortAssistant, Command<Self::Message>) {
        let state = SerialPortAssistant::default();
        (state, Command::none())
    }

    fn title(&self) -> String {
        String::from(SERIAL_PORT_ASSISTANT_TITLE)
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            // SerialPortAssistantMessage::Echo(_event) => match get_serialport_list() {
            //     Ok(serialport_list) => self.avaliable_port_list = serialport_list,
            //     Err(e) => println!("{:?}", e),
            // },
            SerialPortAssistantMessage::Echo(event) => {
                println!("{:?}", event)
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let footer = footer();
        let header = header();
        let body = initial::intial_page();

        let contente = Column::new().push(header).push(body).push(footer);

        contente.into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        match load_usb_sniffer() {
            Ok(subscription) => subscription.map(SerialPortAssistantMessage::Echo),
            Err(_) => todo!(),
        }
    }
}
