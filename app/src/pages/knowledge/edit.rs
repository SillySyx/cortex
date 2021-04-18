use yew::prelude::*;

use crate::components::{PageHeader, InputBox, TextBox, Button, Error};
use crate::services::{KnowledgeService, Knowledge, KnowledgeData, KnowledgeDataType};

pub enum Messages {
    UpdateName(String),
    UpdateDescription(String),
    UpdatePath(String),
    UpdateContent(String),

    SaveClicked,
    BackClicked,
    RemoveClicked,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub change_view: Callback<(String, Option<String>)>,
    pub id: String,
}

pub struct EditView {
    props: Props,
    link: ComponentLink<Self>,
    error: String,
    name: String,
    name_error: String,
    path: String,
    path_error: String,
    description: String,
    content_type: KnowledgeDataType,
    content: String,
}

impl Component for EditView {
    type Message = Messages;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut error = String::new();

        let knowledge = match KnowledgeService::load_knowledge(&props.id) {
            Ok(value) => value,
            Err(_) => {
                error = String::from("Failed to load knowledge");
                Knowledge::default()
            },
        };

        let knowledge_data = match KnowledgeService::load_knowledge_data(&props.id) {
            Ok(value) => value,
            Err(_) => {
                error = String::from("Failed to load knowledge data");
                KnowledgeData::default()
            },
        };

        let content = match std::str::from_utf8(&knowledge_data.data) {
            Ok(value) => value.to_string(),
            Err(_) => {
                error = String::from("Failed to parse knowledge data");
                String::new()
            },
        };

        Self {
            props,
            link,
            error,
            name: knowledge.name.clone(),
            name_error: String::new(),
            description: knowledge.description.clone(),
            path: knowledge.path.clone(),
            path_error: String::new(),
            content_type: knowledge_data.data_type,
            content,
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
            Messages::SaveClicked => {
                match KnowledgeService::update_knowledge(&self.props.id, Some(self.path.clone()), Some(self.name.clone()), Some(self.description.clone())) {
                    Ok(_) => {},
                    Err(_) => {
                        self.error = String::from("Failed to update knowledge");
                        return true;
                    }
                };

                let data = self.content.as_bytes().to_vec();

                match KnowledgeService::update_knowledge_data(&self.props.id, Some(self.content_type.clone()), Some(data)) {
                    Ok(_) => {},
                    Err(_) => {
                        self.error = String::from("Failed to update knowledge data");
                        return true;
                    },
                };

                self.props.change_view.emit((String::from("list"), Some(self.props.id.clone())));
                false
            },
            Messages::BackClicked => {
                self.props.change_view.emit((String::from("list"), Some(self.props.id.clone())));
                false
            },
            Messages::RemoveClicked => {
                match KnowledgeService::remove_knowledge_data(&self.props.id) {
                    Ok(_) => {},
                    Err(_) => {
                        self.error = String::from("Failed to remove knowledge data");
                        return true;
                    },
                };

                match KnowledgeService::remove_knowledge(&self.props.id) {
                    Ok(_) => {},
                    Err(_) => {
                        self.error = String::from("Failed to remove knowledge");
                        return true;
                    },
                };

                self.props.change_view.emit((String::from("list"), Some(self.props.id.clone())));
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
                <PageHeader title="Edit knowledge"
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
                    <Button disabled=disabled clicked=self.link.callback(|_| Messages::SaveClicked)>
                        {"Save"}
                    </Button>
                    <Button clicked=self.link.callback(|_| Messages::BackClicked)>
                        {"Back"}
                    </Button>
                </div>

                <Error title="Danger" text="It's not possible to restore this knowledge once it has been removed.">
                    <Button class="error-button" clicked=self.link.callback(|_| Messages::RemoveClicked)>
                        {"Remove"}
                    </Button>
                </Error>
            </div>
        }
    }
}