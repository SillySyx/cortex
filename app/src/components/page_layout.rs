use yew::prelude::*;

use super::{Messages, LoginPage, MainPage};

pub struct PageLayout {
    page: &'static str,
}

impl Component for PageLayout {
    type Message = Messages;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            page: determin_initial_page(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::ChangePage(page) => self.page = page,
            _ => {},
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let change_page = |page| {
            self.send_message(Messages::ChangePage(page));
        };
        match self.page {
            "login" => html! {
                <LoginPage change_page=change_page />
            },
            "main" => html! {
                <MainPage />
            },
            _ => html! {},
        }
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