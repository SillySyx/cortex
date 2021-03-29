use yew::prelude::*;

use crate::services::LoginService;
use crate::components::{InputBox, Svg};

pub enum Messages {
    Unlock(String),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub unlock_app: Callback<()>,
}

pub struct LoginPage {
    props: Props,
    link: ComponentLink<Self>,
    error: String,
}

impl Component for LoginPage {
    type Message = Messages;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            error: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::Unlock(password) => {
                if password.is_empty() {
                    self.error = String::from("No password entered");
                    return true;
                }

                if let Some(key) = convert_password_to_key(password) {
                    let valid = match LoginService::verify_key(&key) {
                        Some(valid) => valid,
                        None => {
                            LoginService::store_verify_key(&key);
                            true
                        },
                    };

                    if !valid {
                        self.error = String::from("Invalid password");
                        return true;
                    }

                    LoginService::store_key(key.clone());
                    self.props.unlock_app.emit(());
                    return true;
                }
                false
            },
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="login-layout login-background animation-fade">
                <div class="login-content">
                    <Svg class="login-logo" src="icons/brain.svg" />
                    <InputBox 
                        class="login-input"
                        password=true
                        error=self.error.clone()
                        placeholder="Enter your password"
                        submitted=self.link.callback(|password| Messages::Unlock(password))>
                    </InputBox>
                </div>
            </div>
        }
    }
}

fn convert_password_to_key(password: String) -> Option<String> {
    let key = match crypto::generate_key_from_seed(&password) {
        Ok(data) => data.to_vec(),
        Err(_) => return None,
    };

    match serde_json::to_string(&key) {
        Ok(data) => Some(data),
        Err(_) => None,
    }
}