use yew::prelude::*;

use super::Messages;

pub struct MainPage {
    link: ComponentLink<Self>,
    view: &'static str,
}

impl Component for MainPage {
    type Message = Messages;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            view: "passwords",
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::MainPageChangeView(view) => self.view = view,
            _ => {},
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="main-layout main-background animation-fade">
                <aside class="main-menu">
                    <img class="main-menu-logo" src="icons/brain.svg" alt="" />
                    <div class="main-button" onclick=self.link.callback(|_| Messages::MainPageChangeView("passwords"))>
                        <img src="icons/password.svg" alt="Passwords" />
                    </div>
                    <div class="main-button" onclick=self.link.callback(|_| Messages::MainPageChangeView("knowledge"))>
                        <img src="icons/knowledge.svg" alt="Knowledge" />
                    </div>
                </aside>
                <section class="main-content">
                {
                    match self.view {
                        "passwords" => html! {
                            <div>{"passy"}</div>
                        },
                        "knowledge" => html! {
                            <div>{"huh?"}</div>
                        },
                        _ => html! {},
                    }
                }
                </section>
            </div>
        }
    }
}