use yew::prelude::*;
use std::error::Error;

pub struct WebRtcService {
    pub connected: Callback<()>,
    pub disconnected: Callback<()>,
    pub message: Callback<WebRtcChannelMessage>,
}

pub struct WebRtcChannel {
}

#[derive(Debug)]
pub struct WebRtcChannelMessage {
    pub r#type: String,
    pub data: String,
}

impl WebRtcService {
    pub fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        let id = "123";

        // connect websocket

        let connect_message = WebRtcChannelMessage {
            r#type: "".into(),
            data: id.into(),
        };

        self.send_message(&connect_message)?;

        Ok(())
    }

    pub fn send_message(&self, message: &WebRtcChannelMessage) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn setup_websocket(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

impl WebRtcChannel {
}