use yew::prelude::*;

use super::Button;

pub enum Messages {
    UpdateName(String),
    UpdateDescription(String),
    UpdatePassword(String),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub name: String,
    pub description: String,
    pub password: String,
    pub new_mode: bool,
}

pub struct PasswordEditor {
    link: ComponentLink<Self>,
    id: String,
    name: String,
    description: String,
    password: String,
    new_mode: bool,
}

impl Component for PasswordEditor {
    type Message = Messages;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            id: props.name.clone(),
            name: props.name,
            description: props.description,
            password: props.password,
            new_mode: props.new_mode,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::UpdateName(name) => {
                self.name = name;
                true
            },
            Messages::UpdateDescription(desc) => {
                self.description = desc;
                true
            },
            Messages::UpdatePassword(pass) => {
                self.password = pass;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="category-editor animation-fade">
                <h1>
                { match self.new_mode {
                    true => "New category",
                    false => "Edit category",
                }}
                </h1>

                <input
                    value=self.name 
                    placeholder="Enter name"
                    oninput=self.link.callback(|e: InputData| Messages::UpdateName(e.value)) />
                    
                <input
                    value=self.description 
                    placeholder="Enter description"
                    oninput=self.link.callback(|e: InputData| Messages::UpdateDescription(e.value)) />
                    
                <input
                    value=self.password 
                    placeholder="Enter password"
                    oninput=self.link.callback(|e: InputData| Messages::UpdatePassword(e.value)) />
            </div>
        }
    }
}

// <Button active=false clicked=self.link.callback(|_| Messages::AddCategory(String::from("test")))>
// {"Add"}
// </Button>
// <Button active=false clicked=self.link.callback(|_| Messages::ChangeView(Views::ListPasswords))>
// {"Back"}
// </Button>