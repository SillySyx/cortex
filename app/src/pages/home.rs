use yew::prelude::*;

use crate::components::{PageHeader, ContextMenu, ContextMenuContent, Button, Svg};
use crate::services::LoginService;
use super::MainPageViews;

pub enum Messages {
	Logout,
	ChangeView(MainPageViews),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub change_view: Callback<MainPageViews>,
}

pub struct HomePage {
	props: Props,
    link: ComponentLink<Self>,
}

impl Component for HomePage {
	type Message = Messages;
	type Properties = Props;

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			props,
			link,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
			Messages::Logout => {
				LoginService::logout();
				false
			},
			Messages::ChangeView(view) => {
				self.props.change_view.emit(view);
				false
			}
		}
	}

	fn change(&mut self, _: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		html! {
			<>
			<PageHeader title={"Welcome"}>
				<ContextMenu>
					<Svg class="page-header-icon animation-twist-grow" src="icons/cog.svg" />
					<ContextMenuContent>
						<Button clicked=self.link.callback(|_| Messages::Logout)>
							{"Logout"}
						</Button>
					</ContextMenuContent>
				</ContextMenu>
			</PageHeader>
			<div class="home-page">
				<Svg class="home-page-icon" src="icons/home.svg" />
				<div class="quick-links">
					<div class="quick-link animation-grow" onclick={self.link.callback(|_| Messages::ChangeView(MainPageViews::Passwords))}>
						<Svg class="quick-link-icon" src="icons/password.svg" />
						<div>
							<h3>{"Password manager"}</h3>
							<p class="quick-link-text">{"Save things so that you can forget them!"}</p>
						</div>
					</div>
					<div class="quick-link animation-grow" onclick={self.link.callback(|_| Messages::ChangeView(MainPageViews::Knowledgebase))}>
						<Svg class="quick-link-icon" src="icons/knowledge.svg" />
						<div>
							<h3>{"Knowledgebase"}</h3>
							<p class="quick-link-text">{"Useful if you have a silly brain."}</p>
						</div>
					</div>
				</div>
			</div>
			</>
		}
	}
}