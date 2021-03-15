use yew::prelude::*;

pub enum Message {
    KeyPressed(KeyboardEvent),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub unlock_app: Callback<&'static str>,
}

pub struct LoginPage {
    props: Props,
    link: ComponentLink<Self>,
    view: &'static str,
}

impl Component for LoginPage {
    type Message = Message;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            view: "login",
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::KeyPressed(key) => {
                if key.key() == String::from("Enter") {
                    self.view = "loading";
                    self.props.unlock_app.emit("");
                    return true;
                }
                false
            }
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
                           onkeyup=self.link.callback(|e| Message::KeyPressed(e)) />
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