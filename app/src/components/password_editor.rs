use yew::prelude::*;

use super::{Button, PageHeader, InputBox, Error};
use crate::services::ClipboardService;

#[derive(PartialEq)]
enum Mode {
    New,
    Edit,
}

pub enum Messages {
    UpdateName(String),
    UpdateDescription(String),
    UpdatePassword(String),

    CopyDescription,
    CopyPassword,

    AddClicked,
    BackClicked,
    SaveClicked,
    RemoveClicked,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
	#[prop_or_default]
    pub id: String,
	#[prop_or_default]
    pub name: String,
	#[prop_or_default]
    pub description: String,
	#[prop_or_default]
    pub password: String,
	#[prop_or(false)]
    pub new_mode: bool,

	#[prop_or_default]
    pub added: Callback<(String, String, String)>,
	#[prop_or_default]
    pub backed: Callback<()>,
	#[prop_or_default]
    pub saved: Callback<(String, String, String, String)>,
	#[prop_or_default]
    pub removed: Callback<String>,
}

pub struct PasswordEditor {
    props: Props,
    link: ComponentLink<Self>,
    id: String,
    name: String,
    name_error: String,
    description: String,
    password: String,
    password_error: String,
    mode: Mode,
}

impl Component for PasswordEditor {
    type Message = Messages;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mode = match props.new_mode {
            true => Mode::New,
            false => Mode::Edit,
        };

        let id = props.id.clone();
        let name = props.name.clone();
        let description = props.description.clone();
        let password = props.password.clone();

        Self {
            props,
            link,
            id,
            name,
            name_error: String::from(""),
            description,
            password,
            password_error: String::from(""),
            mode,
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

                self.props.added.emit((self.name.clone(), self.description.clone(), self.password.clone()));
                false
            },
            Messages::BackClicked => {
                self.props.backed.emit(());
                false
            },
            Messages::SaveClicked => {
                if self.name.is_empty() {
                    return false;
                }
                if self.password.is_empty() {
                    return false;
                }
                
                self.props.saved.emit((self.id.clone(), self.name.clone(), self.description.clone(), self.password.clone()));
                false
            },
            Messages::RemoveClicked => {
                self.props.removed.emit(self.id.clone());
                false
            },
            Messages::CopyDescription => {
                ClipboardService::copy_to_clipboard(self.description.clone());
                false
            },
            Messages::CopyPassword => {
                ClipboardService::copy_to_clipboard(self.password.clone());
                false
            },
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let title = match self.mode {
            Mode::New => "Add password",
            Mode::Edit => "Edit password",
        };

        let copy_desc = match self.mode {
            Mode::New => html! {},
            Mode::Edit => html! {
                <img class="input-box-icon animation-grow" src="icons/copy.svg" alt="Copy description" onclick=self.link.callback(|_| Messages::CopyDescription) />
            },
        };

        let copy_pass = match self.mode {
            Mode::New => html! {},
            Mode::Edit => html! {
                <img class="input-box-icon animation-grow" src="icons/copy.svg" alt="Copy password" onclick=self.link.callback(|_| Messages::CopyPassword) />
            },
        };

        html! {
            <div class="password-editor animation-fade">
                <PageHeader title=title />

                <InputBox
                    focus=true
                    label={"Name"}
                    placeholder={"Enter name"}
                    value=self.name.clone()
                    error=self.name_error.clone()
                    value_changed=self.link.callback(|value| Messages::UpdateName(value))>
                </InputBox>

                <InputBox
                    label={"Description"}
                    placeholder={"Enter description"}
                    value=self.description.clone()
                    value_changed=self.link.callback(|value| Messages::UpdateDescription(value))>
                    { copy_desc }
                </InputBox>

                <InputBox
                    password=true
                    label={"Password"}
                    placeholder={"Enter password"}
                    value=self.password.clone()
                    error=self.password_error.clone()
                    value_changed=self.link.callback(|value| Messages::UpdatePassword(value))>
                    { copy_pass }
                </InputBox>

                { self.render_buttons() }
            </div>
        }
    }
}

impl PasswordEditor {
    fn render_buttons(&self) -> Html {
        let disabled = self.name.is_empty() || self.password.is_empty();

        if self.mode == Mode::New {
            return html! {
                <div class="password-editor-buttons">
                    <Button disabled=disabled clicked=self.link.callback(|_| Messages::AddClicked)>
                        {"Add"}
                    </Button>
                    <Button clicked=self.link.callback(|_| Messages::BackClicked)>
                        {"Back"}
                    </Button>
                </div>
            };
        }

        html! {
            <>
            <div class="password-editor-buttons">
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
            </>
        }
    }
}