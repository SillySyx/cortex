use yew::prelude::*;

use super::Button;

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
    pub id: String,
    pub name: String,
    pub new_mode: bool,

    pub added: Callback<String>,
    pub backed: Callback<()>,
    pub saved: Callback<(String, String)>,
    pub removed: Callback<String>,
}

pub struct PasswordCategoryEditor {
    props: Props,
    link: ComponentLink<Self>,
    id: String,
    name: String,
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

        let id = props.name.clone();
        let name = props.name.clone();

        Self {
            props,
            link,
            id,
            name,
            mode,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::UpdateName(name) => {
                self.name = name;
                true
            },
            Messages::AddClicked => {
                self.props.added.emit(self.name.clone());
                false
            },
            Messages::BackClicked => {
                self.props.backed.emit(());
                false
            },
            Messages::SaveClicked => {
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
        html! {
            <div class="category-editor animation-fade">
                { self.render_header() }

                <input
                    value=self.name 
                    placeholder="Enter name"
                    oninput=self.link.callback(|e: InputData| Messages::UpdateName(e.value)) />
                    
                { self.render_buttons() }
            </div>
        }
    }
}

impl PasswordCategoryEditor {
    fn render_header(&self) -> Html {
        if self.mode == Mode::New {
            return html! {
                <>
                <h1>{"New category"}</h1>
                <p>{"Categories are used to group similar passwords together so that it's easier to overview."}</p>
                </>
            };
        }

        html! {
            <>
            <h1>{"Edit category"}</h1>
            <p>{"Categories are used to group similar passwords together so that it's easier to overview."}</p>
            </>
        }
    }

    fn render_buttons(&self) -> Html {
        if self.mode == Mode::New {
            return html! {
                <>
                <Button active=false clicked=self.link.callback(|_| Messages::AddClicked)>
                    {"Add"}
                </Button>
                <Button active=false clicked=self.link.callback(|_| Messages::BackClicked)>
                    {"Back"}
                </Button>
                </>
            };
        }

        html! {
            <>
            <Button active=false clicked=self.link.callback(|_| Messages::SaveClicked)>
                {"Save"}
            </Button>
            <Button active=false clicked=self.link.callback(|_| Messages::RemoveClicked)>
                {"Remove"}
            </Button>
            <Button active=false clicked=self.link.callback(|_| Messages::BackClicked)>
                {"Back"}
            </Button>
            </>
        }
    }
}