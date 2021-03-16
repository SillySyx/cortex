use yew::prelude::*;

use super::{LoginPage, MainPage};
use crate::services::LoginService;

pub enum Pages {
    Login,
    Main,
}

pub enum Messages {
    UnlockApp,
}

pub struct App {
    link: ComponentLink<Self>,
    page: Pages,
}

impl Component for App {
    type Message = Messages;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            page: determin_initial_page(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::UnlockApp => self.unlock_app(),
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match self.page {
            Pages::Login => html! {
                <LoginPage unlock_app=self.link.callback(|_| Messages::UnlockApp) />
            },
            Pages::Main => html! {
                <MainPage />
            },
        }
    }
}

impl App {
    fn unlock_app(&mut self) -> ShouldRender {
        self.page = Pages::Main;
        return true;
    }
}

fn determin_initial_page() -> Pages {
    match LoginService::is_logged_in() {
        true => Pages::Main,
        false => Pages::Login,
    }
}