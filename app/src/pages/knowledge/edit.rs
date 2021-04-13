use yew::prelude::*;

use crate::services::{KnowledgeService, Knowledge, KnowledgeData};

pub enum Messages {
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
            content,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="animation-fade">
                {"edit"}
            </div>
        }
    }
}