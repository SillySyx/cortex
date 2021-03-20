use yew::{
    prelude::*, 
    web_sys::HtmlInputElement,
};

use super::Button;

#[derive(PartialEq)]
enum Mode {
    New,
    Edit,
}

pub enum Messages {
    UpdateName(String),
    UpdateDescription(String),
    UpdatePassword(String),

    AddClicked,
    BackClicked,
    SaveClicked,
    RemoveClicked,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub id: String,
    pub name: String,
    pub description: String,
    pub password: String,
    pub new_mode: bool,

    pub added: Callback<(String, String, String)>,
    pub backed: Callback<()>,
    pub saved: Callback<(String, String, String, String)>,
    pub removed: Callback<String>,
}

pub struct PasswordEditor {
    props: Props,
    link: ComponentLink<Self>,
    focus_ref: NodeRef,
    id: String,
    name: String,
    description: String,
    password: String,
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

        let id = props.name.clone();
        let name = props.name.clone();
        let description = props.description.clone();
        let password = props.password.clone();

        Self {
            props,
            link,
            focus_ref: NodeRef::default(),
            id,
            name,
            description,
            password,
            mode,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::UpdateName(name) => {
                self.name = name;
                true
            },
            Messages::UpdateDescription(description) => {
                self.description = description;
                true
            },
            Messages::UpdatePassword(password) => {
                self.password = password;
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
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(input) = self.focus_ref.cast::<HtmlInputElement>() {
                let _ = input.focus();
            }
        }
    }

    fn view(&self) -> Html {
        let password_type = match self.mode {
            Mode::New => "text",
            Mode::Edit => "password",
        };

        html! {
            <div class="password-editor animation-fade">
                { self.render_header() }

                <lable>{"Name"}</lable>
                <input
                    ref=self.focus_ref.clone()
                    value=self.name 
                    placeholder="Enter name"
                    oninput=self.link.callback(|e: InputData| Messages::UpdateName(e.value)) />

                <lable>{"Description"}</lable>
                <input
                    value=self.description 
                    placeholder="Enter description"
                    oninput=self.link.callback(|e: InputData| Messages::UpdateDescription(e.value)) />

                <lable>{"Password"}</lable>
                <input
                    value=self.password 
                    type=password_type
                    placeholder="Enter password"
                    oninput=self.link.callback(|e: InputData| Messages::UpdatePassword(e.value)) />
                    
                { self.render_buttons() }
            </div>
        }
    }
}

impl PasswordEditor {
    fn render_header(&self) -> Html {
        if self.mode == Mode::New {
            return html! {
                <div class="password-editor-header">
                    <h1>{"Add password"}</h1>
                </div>
            };
        }

        html! {
            <div class="password-editor-header">
                <h1>{"Edit password"}</h1>
            </div>
        }
    }

    fn render_buttons(&self) -> Html {
        if self.mode == Mode::New {
            return html! {
                <div class="password-editor-buttons">
                    <Button active=false clicked=self.link.callback(|_| Messages::AddClicked)>
                        {"Add"}
                    </Button>
                    <Button active=false clicked=self.link.callback(|_| Messages::BackClicked)>
                        {"Back"}
                    </Button>
                </div>
            };
        }

        html! {
            <>
            <div class="password-editor-buttons">
                <Button active=false clicked=self.link.callback(|_| Messages::SaveClicked)>
                    {"Save"}
                </Button>
                <Button active=false clicked=self.link.callback(|_| Messages::BackClicked)>
                    {"Back"}
                </Button>
            </div>
            <div class="password-editor-dangerzone">
                <h1>{"Danger"}</h1>
                <p>{"It's not possible to restore the password once it has been removed."}</p>
                <Button active=false clicked=self.link.callback(|_| Messages::RemoveClicked)>
                    {"Remove"}
                </Button>
            </div>
            </>
        }
    }
}