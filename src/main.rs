use std::time::Duration;

use leaprs::*;
use throbber::Throbber;

fn main() {
    let mut connection =
        Connection::create(ConnectionConfig::default()).expect("Failed to create connection");
    connection.open().expect("Failed to open the connection");

    connection.wait_for("Connecting to the service...".to_string(), |e| match e {
        Event::Connection(e) => {
            let flags = e.flags();
            Msg::Success(format!("Connected. Service state: {:?}", flags))
        }
        _ => Msg::None,
    });

    connection.wait_for("Waiting for a device...".to_string(), |e| match e {
        Event::Device(e) => {
            let device_info = e
                .device()
                .open()
                .expect("Failed to open the device")
                .get_info()
                .expect("Failed to get device info");

            let serial = device_info
                .serial()
                .expect("Failed to get the device serial");

            let pid = device_info.pid();

            Msg::Success(format!("Got the device {} ({})", serial, pid))
        }
        _ => Msg::None,
    });

    // Set the tracking mode to unknown.
    #[cfg(feature = "gemini")]
    {
        connection
            .get_tracking_mode()
            .expect("Failed to request for tracking mode");

        connection.wait_for(
            "Waiting for the tracking mode message...".to_string(),
            |e| match e {
                Event::TrackingMode(e) => {
                    Msg::Success(format!("Tracking mode: {:#?}", e.current_tracking_mode()))
                }
                _ => Msg::None,
            },
        );
    }

    connection.wait_for("Waiting for a hand...".to_string(), |e| match e {
        Event::Tracking(e) => {
            if !e.hands().is_empty() {
                Msg::Success("Got a hand".to_string())
            } else {
                Msg::None
            }
        }
        _ => Msg::None,
    });

    connection.wait_for("Close the hand".to_string(), |e| match e {
        Event::Tracking(e) => {
            if let Some(hand) = e.hands().first() {
                let grab_strength = hand.grab_strength();
                if grab_strength >= 1.0 {
                    Msg::Success("The hand is closed".to_string())
                } else {
                    Msg::Progress(format!("Close the hand {:.0}%", grab_strength * 100.0))
                }
            } else {
                Msg::Progress("Close the hand".to_string())
            }
        }
        _ => Msg::None,
    });

    connection.wait_for("Open the hand".to_string(), |e| match e {
        Event::Tracking(e) => {
            if let Some(hand) = e.hands().first() {
                let ungrab_strength = 1.0 - hand.grab_strength();
                if ungrab_strength >= 0.999 {
                    Msg::Success("The hand is opened".to_string())
                } else {
                    Msg::Progress(format!("Open the hand {:.0}%", ungrab_strength * 100.0))
                }
            } else {
                Msg::Progress("Open the hand".to_string())
            }
        }
        _ => Msg::None,
    });

    connection
        .set_policy_flags(PolicyFlags::IMAGES, PolicyFlags::empty())
        .expect("Failed to set policy flags");

    connection.wait_for("Reading image".to_string(), |e| match e {
        Event::Image(e) => {
            let w = e.images()[0].properties().width();
            let h = e.images()[0].properties().height();
            let images = e.images();
            let image_data = images[0].data();
            image::save_buffer("image.png", image_data, w, h, image::ColorType::L8)
                .expect("failed to save buffer");
            Msg::Success("Saved image.png".to_string())
        }
        _ => Msg::None,
    });

    connection
        .set_policy_flags(PolicyFlags::empty(), PolicyFlags::IMAGES)
        .expect("Failed to set policy flags");
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Msg {
    None,
    Success(String),
    Progress(String),
    Failure(String),
}

trait WaitFor {
    fn wait_for<F>(&mut self, message: String, condition: F)
    where
        F: Fn(&Event) -> Msg;
}

impl WaitFor for Connection {
    fn wait_for<F>(&mut self, message: String, condition: F)
    where
        F: Fn(&Event) -> Msg,
    {
        let mut throbber = Throbber::new().interval(Duration::from_millis(100));

        throbber.start_with_msg(message);

        loop {
            if let Ok(message) = self.poll(100) {
                match condition(&message.event()) {
                    Msg::None => {}
                    Msg::Success(message) => {
                        throbber.success(message);
                        break;
                    }
                    Msg::Progress(message) => {
                        throbber.change_message(message);
                    }
                    Msg::Failure(message) => {
                        throbber.fail(message);
                        break;
                    }
                }
            }
        }
        throbber.end();
    }
}