use yew::{
    prelude::*, 
    web_sys::HtmlInputElement
};

use crate::services::LoginService;

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
    pub unlock_app: Callback<()>,
}

pub struct LoginPage {
    props: Props,
    link: ComponentLink<Self>,
    focus_ref: NodeRef,
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
            focus_ref: NodeRef::default(),
            view: Views::Login,
            password: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::KeyPressed(key) => {
                if key.key() == String::from("Enter") {
                    self.view = Views::Loading;

                    match convert_password_to_key(self.password.clone()) {
                        Some(key) => {
                            LoginService::store_key(key.clone());
                            self.props.unlock_app.emit(());
                            return true;
                        },
                        None => {},
                    };
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

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(input) = self.focus_ref.cast::<HtmlInputElement>() {
                match input.focus() {
                    Ok(_) => {},
                    Err(_) => {},
                };
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="login-layout login-background animation-fade">
            <img class="login-logo" src="icons/brain.svg" alt="" />
            {
                match self.view {
                    Views::Login => html! {
                        <input 
                            ref=self.focus_ref.clone()
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

fn convert_password_to_key(password: String) -> Option<String> {
    let key = match crypto::generate_key_from_seed(&password) {
        Ok(data) => data.to_vec(),
        Err(_) => return None,
    };

    let json = match serde_json::value::to_value(key) {
        Ok(json) => json,
        Err(_) => return None,
    };

    Some(json.to_string())
}