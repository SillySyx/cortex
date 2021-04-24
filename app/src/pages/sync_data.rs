use yew::prelude::*;

use crate::services::{LoginService, PasswordService, KnowledgeService};
use crate::services::webrtc::{WebRtcService, WebRtcChannelMessage};
use crate::components::{PageHeader, Button, Error};

pub enum Messages {
	Connected,
	Disconnected,
	Message(WebRtcChannelMessage),

	ResetData,
}

pub struct SyncDataPage {
    link: ComponentLink<Self>,
	webrtc: WebRtcService,
	connected: bool,
}

impl Component for SyncDataPage {
	type Message = Messages;
	type Properties = ();

	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		let mut webrtc = WebRtcService {
			connected: link.callback(|_| Messages::Connected),
			disconnected: link.callback(|_| Messages::Disconnected),
			message: link.callback(|msg| Messages::Message(msg)),
		};

		let _ = webrtc.connect();

		Self {
			link,
			webrtc,
			connected: false,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
			Messages::Connected => {
				self.connected = true;
				true
			},
			Messages::Disconnected => {
				self.connected = false;
				true
			},
			Messages::Message(message) => {
				yew::services::ConsoleService::log(&format!("{:?}", message));
				false
			},
			Messages::ResetData => {
                PasswordService::reset_data();
                KnowledgeService::reset_data();
                LoginService::reset_data();
                LoginService::logout();
				false
			},
		}
	}

	fn change(&mut self, _: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		html! {
			<div class="sync-data-page animation-fade">
				<PageHeader title={"Sync data"}
							description={"To sync data between device both of them needs to have the same master password and then enter this page."}>
				</PageHeader>

				{ self.render_connection_status() }

                <Error title="Danger" text="Resetting the data of this device will remove all passwords and knowledge.">
                    <Button class="error-button" clicked=self.link.callback(|_| Messages::ResetData)>
                        {"Reset data"}
                    </Button>
                </Error>
			</div>
		}
	}
}

impl SyncDataPage {
	fn render_connection_status(&self) -> Html {
		if self.connected {
			return html! {
				<div class="connection-status">
					<div class="connection-status-icon connected"></div>
					<span>{"Connected"}</span>
				</div>
			};
		}

		html! {
			<div class="connection-status">
				<div class="connection-status-icon not-connected"></div>
				<span>{"Not connected"}</span>
			</div>
		}
	}
}