use yew::prelude::*;

use crate::components::{Button, Svg};
use super::{PasswordsPage, KnowledgebasePage, HomePage, LoginPage};
use crate::services::LoginService;

#[derive(PartialEq, Clone)]
pub enum Views {
    Home,
    Passwords,
    Knowledgebase,
}

pub enum Messages {
    ChangeView(Views),
}

pub struct MainPage {
    link: ComponentLink<Self>,
    view: Views,
}

impl Component for MainPage {
    type Message = Messages;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            view: Views::Home,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::ChangeView(view) => {
                self.view = view;
                true
            },
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if !LoginService::is_logged_in() {
            return self.render_login_page();
        }

        self.render_main_page()
    }
}

impl MainPage {
    fn render_login_page(&self) -> Html {
        html! {
            <LoginPage unlock_app=self.link.callback(|_| Messages::ChangeView(Views::Home)) />
        }
    }

    fn render_main_page(&self) -> Html {
        html! {
            <div class="main-layout main-background animation-fade">
                <aside class="main-menu">
                    <div class="menu-items">
                        <Svg class="main-menu-logo" src="icons/brain.svg" clicked=self.link.callback(|_| Messages::ChangeView(Views::Home)) />

                        { self.render_menu_button("icons/password.svg", Views::Passwords) }
                        { self.render_menu_button("icons/knowledge.svg", Views::Knowledgebase) }
                    </div>
                </aside>
                <section class="main-content animation-fade">
                {
                    match self.view {
                        Views::Home => html! {
                            <HomePage change_view={self.link.callback(|view: Views| Messages::ChangeView(view))} />
                        },
                        Views::Passwords => html! {
                            <PasswordsPage />
                        },
                        Views::Knowledgebase => html! {
                            <KnowledgebasePage />
                        },
                    }
                }
                </section>
            </div>
        }
    }

    fn render_menu_button(&self, icon: &'static str, view: Views) -> Html {
        let button_active = self.view == view;

        html! {
            <Button active=button_active clicked=self.link.callback(move |_| Messages::ChangeView(view.clone()))>
                <Svg class="main-button-icon" src=icon />
            </Button>
        }
    }
}