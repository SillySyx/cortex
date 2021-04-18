use yew::prelude::*;

use crate::components::{PageHeader, InputBox, TextBox, Button, Error};
use crate::services::{KnowledgeService, KnowledgeDataType};

pub enum Messages {
    UpdateName(String),
    UpdateDescription(String),
    UpdatePath(String),
    UpdateContent(String),

    AddClicked,
    BackClicked,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub change_view: Callback<(String, Option<String>)>,
    pub id: String,
}

pub struct AddView {
    props: Props,
    link: ComponentLink<Self>,
    parent_id: String,
    error: String,
    name: String,
    name_error: String,
    path: String,
    path_error: String,
    description: String,
    content: String,
}

impl Component for AddView {
    type Message = Messages;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let parent_id = props.id.clone();

        Self {
            props,
            link,
            parent_id,
            error: String::new(),
            name: String::new(),
            name_error: String::new(),
            description: String::new(),
            path: String::new(),
            path_error: String::new(),
            content: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::UpdateName(value) => {
                self.name = value;

                self.name_error = match self.name.is_empty() {
                    true => String::from("No name entered"),
                    false => String::from(""),
                };

                true
            },
            Messages::UpdateDescription(value) => {
                self.description = value;
                true
            },
            Messages::UpdatePath(value) => {
                self.path = value;

                self.path_error = match self.path.is_empty() {
                    true => String::from("No path entered"),
                    false => String::from(""),
                };

                true
            },
            Messages::UpdateContent(value) => {
                self.content = value;
                true
            },
            Messages::AddClicked => {
                let knowledge = match KnowledgeService::create_knowledge(self.path.clone(), self.name.clone(), self.description.clone()) {
                    Ok(value) => value,
                    Err(_) => {
                        self.error = String::from("Failed to create knowledge");
                        return true;
                    }
                };

                let data = self.content.as_bytes().to_vec();

                match KnowledgeService::create_knowledge_data(&knowledge.id, KnowledgeDataType::Markdown, data) {
                    Ok(_) => {},
                    Err(_) => {
                        self.error = String::from("Failed to create knowledge data");
                        return true;
                    },
                };

                self.props.change_view.emit((String::from("list"), Some(self.parent_id.clone())));
                false
            },
            Messages::BackClicked => {
                self.props.change_view.emit((String::from("list"), Some(self.parent_id.clone())));
                false
            },
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if !self.error.is_empty() {
            return html! {
                <>
                    <Error text=&self.error />
                    <div class="button-grid">
                        <Button clicked=self.link.callback(|_| Messages::BackClicked)>
                            {"Back"}
                        </Button>
                    </div>
                </>
            };
        }

        let disabled = self.name.is_empty() || self.path.is_empty();

        html! {
            <div class="animation-fade">
                <PageHeader title="Add knowledge"
                            description="It's possible to place multiple knowledge entries under the same path, this will generate the navigation three.">
                </PageHeader>

                <InputBox
                    label="Name"
                    placeholder="Enter name"
                    focus=true
                    mandatory=true
                    value=self.name.clone()
                    error=self.name_error.clone()
                    value_changed=self.link.callback(|value| Messages::UpdateName(value))>
                </InputBox>

                <InputBox
                    label="Description"
                    placeholder="Enter decription"
                    value=self.description.clone()
                    value_changed=self.link.callback(|value| Messages::UpdateDescription(value))>
                </InputBox>

                <InputBox
                    label="Path"
                    placeholder="Enter path"
                    mandatory=true
                    value=self.path.clone()
                    error=self.path_error.clone()
                    value_changed=self.link.callback(|value| Messages::UpdatePath(value))>
                </InputBox>

                <TextBox
                    label="Content"
                    placeholder="Enter content"
                    value=self.content.clone()
                    value_changed=self.link.callback(|value| Messages::UpdateContent(value))>
                </TextBox>

                <div class="button-grid">
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