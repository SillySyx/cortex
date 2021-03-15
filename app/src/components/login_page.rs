use yew::prelude::*;

pub enum Views {
    Login,
    Loading,
}

pub enum Messages {
    KeyPressed(KeyboardEvent),
    UpdatePassword(String)
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub unlock_app: Callback<String>,
}

pub struct LoginPage {
    props: Props,
    link: ComponentLink<Self>,
    view: Views,
    password: String,
}

impl Component for LoginPage {
    type Message = Messages;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            view: Views::Login,
            password: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::KeyPressed(key) => {
                if key.key() == String::from("Enter") {
                    self.view = Views::Loading;
                    self.props.unlock_app.emit(self.password.clone());
                    return true;
                }
                false
            },
            Messages::UpdatePassword(value) => {
                self.password = value;
                true
            },
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="login-layout login-background animation-fade">
            <img class="login-logo" src="icons/brain.svg" alt="" />
            {
                match self.view {
                    Views::Login => html! {
                        <input 
                           class="login-box login-input" 
                           type="password" 
                           placeholder="Enter your password"
                           oninput=self.link.callback(|e: InputData| Messages::UpdatePassword(e.value))
                           onkeyup=self.link.callback(|e| Messages::KeyPressed(e)) />
                    },
                    Views::Loading => html! {
                        <img class="login-box login-loader animation-spin" src="icons/loading.svg" alt="" />
                    },
                }
            }
            </div>
        }
    }
}