use yew::prelude::*;
use yew::services::ConsoleService;

use super::{Messages, LoginPage, MainPage};

pub struct App {
    page: &'static str,
}

impl Component for App {
    type Message = Messages;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            page: determin_initial_page(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::ChangePage(page) => {
                self.page = page;
                true
            },
            Messages::UnlockApp(password) => {
                self.unlock_app(password)
            },
            _ => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match self.page {
            "login" => html! {
                <LoginPage />
            },
            "main" => html! {
                <MainPage />
            },
            _ => html! {},
        }
    }
}

impl App {
    fn unlock_app(&self, _password: &str) -> bool {
        ConsoleService::log("unlock app");
        true
    }
}

fn determin_initial_page() -> &'static str {
    match is_logged_in() {
        true => "main",
        false => "login",
    }
}

fn is_logged_in() -> bool {
    false
}