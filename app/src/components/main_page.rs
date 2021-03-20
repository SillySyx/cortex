use yew::prelude::*;

use super::{Button, PasswordList, Knowledgebase};

#[derive(PartialEq)]
pub enum Views {
    PasswordList,
    Knowledge,
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
            view: Views::PasswordList,
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
        let password_active = self.view == Views::PasswordList;
        let knowledge_active = self.view == Views::Knowledge;

        html! {
            <div class="main-layout main-background animation-fade">
                <aside class="main-menu">
                    <img class="main-menu-logo" src="icons/brain.svg" alt="" />

                    <Button active=password_active clicked=self.link.callback(|_| Messages::ChangeView(Views::PasswordList))>
                        <img src="icons/password.svg" alt="Passwords" />
                    </Button>

                    <Button active=knowledge_active clicked=self.link.callback(|_| Messages::ChangeView(Views::Knowledge))>
                        <img src="icons/knowledge.svg" alt="Knowledge" />
                    </Button>
                </aside>
                <section class="main-content">
                {
                    match self.view {
                        Views::PasswordList => html! {
                            <PasswordList />
                        },
                        Views::Knowledge => html! {
                            <Knowledgebase />
                        },
                    }
                }
                </section>
            </div>
        }
    }
}