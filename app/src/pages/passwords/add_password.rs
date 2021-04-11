use yew::prelude::*;

use crate::components::{Button, PageHeader, InputBox};
use crate::services::PasswordService;

use super::page::Views;

pub enum Messages {
    UpdateName(String),
    UpdateDescription(String),
    UpdatePassword(String),

    AddClicked,
    BackClicked,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub change_view: Callback<(Views, Option<String>)>,
    pub category_id: String,
}

pub struct AddPasswordView {
    props: Props,
    link: ComponentLink<Self>,
    category_id: String,
    name: String,
    name_error: String,
    description: String,
    password: String,
    password_error: String,
}

impl Component for AddPasswordView {
    type Message = Messages;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let category_id = props.category_id.clone();

        Self {
            props,
            link,
            category_id,
            name: String::from(""),
            name_error: String::from(""),
            description: String::from(""),
            password: String::from(""),
            password_error: String::from(""),
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
            Messages::AddClicked => {
                if self.name.is_empty() {
                    return false;
                }
                if self.password.is_empty() {
                    return false;
                }

                if let Ok(_) = PasswordService::create_password(&self.category_id, self.name.clone(), self.description.clone(), self.password.clone()) {
                    self.props.change_view.emit((Views::ListPasswords, None));
                }

                false
            },
            Messages::BackClicked => {
                self.props.change_view.emit((Views::ListPasswords, None));
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
            <div class="password-editor animation-fade">
                <PageHeader title="Add password" />

                <InputBox
                    label="Name"
                    placeholder="Enter name"
                    focus=true
                    value=self.name.clone()
                    error=self.name_error.clone()
                    value_changed=self.link.callback(|value| Messages::UpdateName(value))>
                </InputBox>

                <InputBox
                    label="Description"
                    placeholder="Enter description"
                    value=self.description.clone()
                    value_changed=self.link.callback(|value| Messages::UpdateDescription(value))>
                </InputBox>

                <InputBox
                    label="Password"
                    placeholder="Enter password"
                    value=self.password.clone()
                    error=self.password_error.clone()
                    value_changed=self.link.callback(|value| Messages::UpdatePassword(value))>
                </InputBox>

                <div class="password-editor-buttons">
                    <Button disabled=disabled clicked=self.link.callback(|_| Messages::AddClicked)>
                        {"Add"}
                    </Button>
                    <Button clicked=self.link.callback(|_| Messages::BackClicked)>
                        {"Back"}
                    </Button>
                </div>
            </div>
        }
    }
}