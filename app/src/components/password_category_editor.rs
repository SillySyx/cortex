use yew::prelude::*;

use super::{Button, PageHeader, InputBox, Error};

#[derive(PartialEq)]
enum Mode {
    New,
    Edit,
}

pub enum Messages {
    UpdateName(String),

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
	#[prop_or(false)]
    pub new_mode: bool,

	#[prop_or_default]
    pub added: Callback<String>,
	#[prop_or_default]
    pub backed: Callback<()>,
	#[prop_or_default]
    pub saved: Callback<(String, String)>,
	#[prop_or_default]
    pub removed: Callback<String>,
}

pub struct PasswordCategoryEditor {
    props: Props,
    link: ComponentLink<Self>,
    id: String,
    name: String,
    error: String,
    mode: Mode,
}

impl Component for PasswordCategoryEditor {
    type Message = Messages;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mode = match props.new_mode {
            true => Mode::New,
            false => Mode::Edit,
        };

        let id = props.id.clone();
        let name = props.name.clone();

        Self {
            props,
            link,
            id,
            name,
            error: String::from(""),
            mode,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::UpdateName(name) => {
                self.name = name;

                self.error = match self.name.is_empty() {
                    true => String::from("No name entered"),
                    false => String::from(""),
                };

                true
            },
            Messages::AddClicked => {
                if self.name.is_empty() {
                    return false;
                }

                self.props.added.emit(self.name.clone());
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
                
                self.props.saved.emit((self.id.clone(), self.name.clone()));
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

    fn view(&self) -> Html {
        let title = match self.mode {
            Mode::New => "New category",
            Mode::Edit => "Edit category",
        };

        html! {
            <div class="password-editor animation-fade">
                <PageHeader title=title 
                            description={"Categories are used to group similar passwords together so that it's easier to overview."} />

                <InputBox
                    focus=true
                    label={"Name"}
                    placeholder={"Enter name"}
                    value=self.name.clone()
                    error=self.error.clone()
                    value_changed=self.link.callback(|value| Messages::UpdateName(value))>
                </InputBox>

                { self.render_buttons() }
            </div>
        }
    }
}

impl PasswordCategoryEditor {
    fn render_buttons(&self) -> Html {
        let disabled = self.name.is_empty();

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
            <Error title="Danger" text="Removing this category will also remove all of its passwords, it's not possible to restore any passwords once they have been removed.">
                <Button class="error-button" clicked=self.link.callback(|_| Messages::RemoveClicked)>
                    {"Remove"}
                </Button>
            </Error>
            </>
        }
    }
}