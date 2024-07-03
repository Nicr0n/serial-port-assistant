use std::fmt;

use iced::{futures::channel::mpsc::Sender, subscription, Subscription};
use rusb::{has_hotplug, Context, Device, Hotplug, HotplugBuilder, Registration, UsbContext};

#[derive(Debug, Clone)]
struct HotPlugHandler {
    sender: Sender<Event>,
    init_state: InitState,
}

#[cfg(target_os = "linux")]
impl<T: UsbContext> Hotplug<T> for HotPlugHandler {
    fn device_arrived(&mut self, device: Device<T>) {
        println!("device arrived {:?}", device);
        match self.init_state {
            InitState::Uninitialized => {}
            InitState::Initialized => {
                let _ = self
                    .sender
                    .try_send(Event::OnUsbArrived)
                    .expect("send failed");
            }
        }
    }

    fn device_left(&mut self, device: Device<T>) {
        println!("device left {:?}", device);
        match self.init_state {
            InitState::Uninitialized => {}
            InitState::Initialized => {
                let _ = self.sender.try_send(Event::OnUsbLeft).expect("send failed");
            }
        }
    }
}

impl Drop for HotPlugHandler {
    fn drop(&mut self) {
        println!("HotPlugHandler dropped");
    }
}

pub fn load_usb_sniffer() -> Result<Subscription<Event>, Error> {
    // check usblib hotplug api
    if has_hotplug() {
        struct SnifferWorker;
        let sub = subscription::channel(
            std::any::TypeId::of::<SnifferWorker>(),
            100,
            |mut _output| async move {
                let mut state = State::Starting;
                let mut _init_state = InitState::Uninitialized;
                let context = Context::new().unwrap();
                let handler = HotPlugHandler {
                    sender: _output,
                    init_state: _init_state,
                };
                loop {
                    match &mut state {
                        State::Starting => {
                            println!("register usb hotplug sniffer");

                            match HotplugBuilder::new()
                                .enumerate(true)
                                .register(&context, Box::new(handler.clone()))
                            {
                                Ok(reg) => {
                                    println!("usb sniffer ready");
                                    // pass ownership here,set ready
                                    state = State::Ready(reg);
                                    _init_state = InitState::Initialized
                                }
                                Err(_) => {}
                            };
                        }
                        State::Ready(_reg) => {
                            context.handle_events(None).unwrap();
                            println!("handled");
                        }
                    }
                }
            },
        );
        Ok(sub)
    } else {
        Err(Error)
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    OnUsbArrived,
    OnUsbLeft,
}

enum State {
    Starting,
    Ready(Registration<Context>),
}

// send message when initialized
#[derive(Debug, Clone)]
enum InitState {
    Uninitialized,
    Initialized,
}

pub struct Error;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "libusb hotplug api unsupported")
    }
}
