use yew::prelude::*;

use super::{LoginPage, MainPage};

pub enum Pages {
    Login,
    Main
}

pub enum Messages {
    UnlockApp(String),
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
            Messages::UnlockApp(password) => self.unlock_app(password),
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match self.page {
            Pages::Login => html! {
                <LoginPage unlock_app=self.link.callback(|password| Messages::UnlockApp(password)) />
            },
            Pages::Main => html! {
                <MainPage />
            },
        }
    }
}

impl App {
    fn unlock_app(&mut self, password: String) -> bool {
        if password == "123" {
            // convert password to key
            // verify key matches what has been used before
            // set self.key

            self.page = Pages::Main;
            return true;
        }
        false
    }
}

fn determin_initial_page() -> Pages {
    match is_logged_in() {
        true => Pages::Main,
        false => Pages::Login,
    }
}

fn is_logged_in() -> bool {
    true
}