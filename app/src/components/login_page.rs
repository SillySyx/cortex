use yew::prelude::*;
use yew::services::ConsoleService;

use super::Messages;

pub struct LoginPage {
    link: ComponentLink<Self>,
    view: &'static str,
}

#[derive(Properties, Clone, PartialEq)]
struct Props {
    change_page: Fn<&str>,
}

impl Component for LoginPage {
    type Message = Messages;
    type Properties = Props;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            view: "login",
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::LoginPageChangeView(view) => {
                self.view = view;
                true
            },
            Messages::LoginPageKeyPressed(key) => {
                if key.key() == String::from("Enter") {
                    ConsoleService::log("huh");
                    self.link.props.change_page("main");
                    return true;
                }
                false
            },
            _ => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="login-layout login-background animation-fade">
            <img class="login-logo" src="icons/brain.svg" alt="" />
            {
                match self.view {
                    "login" => html! {
                        <input 
                           class="login-box login-input" 
                           type="password" 
                           placeholder="Enter your password"
                           onkeyup=self.link.callback(|e| Messages::LoginPageKeyPressed(e)) />
                    },
                    "loading" => html! {
                        <img class="login-box login-loader animation-spin" src="icons/loading.svg" alt="" />
                    },
                    _ => html! {},
                }
            }
            </div>
        }
    }
}