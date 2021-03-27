use yew::prelude::*;

use crate::components::{PageHeader, ContextMenu, ContextMenuContent, Button};
use crate::services::LoginService;
use super::{MainPageViews};

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
					<img class="page-header-icon animation-grow" src="icons/cog.svg" alt="" />
					<ContextMenuContent>
						<Button clicked=self.link.callback(|_| Messages::Logout)>
							{"Logout"}
						</Button>
					</ContextMenuContent>
				</ContextMenu>
			</PageHeader>
			<div class="home-page">
				<img src="icons/home.svg" alt="Home" />
				<div class="quick-links">
					<div class="quick-link animation-grow" onclick={self.link.callback(|_| Messages::ChangeView(MainPageViews::Passwords))}>
						<img src="icons/password.svg" alt="" />
						<div>
							<h3>{"Password manager"}</h3>
							<p>{"Handle your passwords with ease."}</p>
						</div>
					</div>
					<div class="quick-link animation-grow" onclick={self.link.callback(|_| Messages::ChangeView(MainPageViews::Knowledgebase))}>
						<img src="icons/knowledge.svg" alt="" />
						<div>
							<h3>{"Knowledgebase"}</h3>
							<p>{"Save things you dont want to forget."}</p>
						</div>
					</div>
					<div class="quick-link animation-grow" onclick={self.link.callback(|_| Messages::ChangeView(MainPageViews::Todo))}>
						<img src="icons/lists.svg" alt="" />
						<div>
							<h3>{"Todo lists"}</h3>
							<p>{"Useful if you have a silly brain."}</p>
						</div>
					</div>
				</div>
			</div>
			</>
		}
	}
}