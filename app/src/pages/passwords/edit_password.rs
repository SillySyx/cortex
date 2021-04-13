use yew::prelude::*;
use yew::services::{TimeoutService, Task};

use std::time::Duration;

use crate::components::{Button, PageHeader, InputBox, Error, Svg};
use crate::services::{PasswordService, Password, ClipboardService};

use super::page::Views;

pub enum Messages {
    UpdateName(String),
    UpdateDescription(String),
    UpdatePassword(String),

    BackClicked,
    RemoveClicked,
    SaveClicked,
    CopyDescription,
    CopyPassword,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub change_view: Callback<(Views, Option<String>)>,
    pub password_id: String,
}

pub struct EditPasswordView {
    props: Props,
    link: ComponentLink<Self>,
    password_id: String,
    name: String,
    name_error: String,
    description: String,
    password: String,
    password_error: String,
    timeout_task: Option<Box<dyn Task>>,
}

impl Component for EditPasswordView {
    type Message = Messages;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let password = match PasswordService::load_password(&props.password_id) {
            Ok(value) => value,
            Err(_) => {
                props.change_view.emit((Views::DecryptError, None));
                Password {
                    description: "".into(),
                    name: "".into(),
                    id: "".into(),
                    password: "".into(),
                }
            },
        };

        let password_id = props.password_id.clone();

        Self {
            props,
            link,
            password_id,
            name: password.name.clone(),
            name_error: String::from(""),
            description: password.description.clone(),
            password: password.password.clone(),
            password_error: String::from(""),
            timeout_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::UpdateName(name) => {
                self.name = name;

                self.name_error = match self.name.is_empty() {
                    true => String::from("No name entered"),
                    false => String::from(""),
                };

                true
            },
            Messages::UpdateDescription(description) => {
                self.description = description;
                true
            },
            Messages::UpdatePassword(password) => {
                self.password = password;

                self.password_error = match self.password.is_empty() {
                    true => String::from("No password entered"),
                    false => String::from(""),
                };

                true
            },
            Messages::BackClicked => {
                self.props.change_view.emit((Views::ListPasswords, None));
                false
            },
            Messages::SaveClicked => {
                if self.name.is_empty() {
                    return false;
                }
                if self.password.is_empty() {
                    return false;
                }

                if let Ok(_) = PasswordService::update_password(&self.password_id, Some(self.name.clone()), Some(self.description.clone()), Some(self.password.clone())) {
                    self.props.change_view.emit((Views::ListPasswords, None));
                }
                
                false
            },
            Messages::RemoveClicked => {
                if let Ok(_) = PasswordService::remove_password(&self.password_id) {
                    self.props.change_view.emit((Views::ListPasswords, None));
                }
                false
            },
            Messages::CopyDescription => {
                ClipboardService::copy_to_clipboard(self.description.clone());
                false
            },
            Messages::CopyPassword => {
                ClipboardService::copy_to_clipboard(self.password.clone());

                let task = TimeoutService::spawn(Duration::from_secs(5), Callback::from(|_| {
                    ClipboardService::copy_to_clipboard("".to_string());
                }));
                self.timeout_task = Some(Box::new(task));

                false
            },
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let disabled = self.name.is_empty() || self.password.is_empty();

        html! {
            <div class="animation-fade">
                <PageHeader title="Edit password" />

                <InputBox
                    label="Name"
                    placeholder="Enter name"
                    mandatory=true
                    value=self.name.clone()
                    error=self.name_error.clone()
                    value_changed=self.link.callback(|value| Messages::UpdateName(value))>
                </InputBox>

                <InputBox
                    label="Description"
                    placeholder="Enter description"
                    value=self.description.clone()
                    value_changed=self.link.callback(|value| Messages::UpdateDescription(value))>
                    <Svg class="input-box-icon animation-grow animation-highlight" src="icons/copy.svg" clicked=self.link.callback(|_| Messages::CopyDescription) />
                </InputBox>

                <InputBox
                    password=true
                    label="Password"
                    placeholder="Enter password"
                    mandatory=true
                    value=self.password.clone()
                    error=self.password_error.clone()
                    value_changed=self.link.callback(|value| Messages::UpdatePassword(value))>
                    <Svg class="input-box-icon animation-grow animation-highlight" src="icons/copy.svg" clicked=self.link.callback(|_| Messages::CopyPassword) />
                </InputBox>

                <div class="button-grid">
                    <Button disabled=disabled clicked=self.link.callback(|_| Messages::SaveClicked)>
                        {"Save"}
                    </Button>
                    <Button clicked=self.link.callback(|_| Messages::BackClicked)>
                        {"Back"}
                    </Button>
                </div>
                
                <Error title="Danger" text="It's not possible to restore the password once it has been removed.">
                    <Button class="error-button" clicked=self.link.callback(|_| Messages::RemoveClicked)>
                        {"Remove"}
                    </Button>
                </Error>
            </div>
        }
    }
}