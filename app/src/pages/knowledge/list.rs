use yew::prelude::*;

use crate::components::{ContextMenu, ContextMenuContent, PageHeader, Svg, Button};
use crate::services::{KnowledgeService, KnowledgeDataType, parse_markdown_to_html};

pub enum Messages {
    SelectKnowledge(String),
    ChangeView(String, Option<String>),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub change_view: Callback<(String, Option<String>)>,
}

pub struct ListView {
    props: Props,
    link: ComponentLink<Self>,
    selected_knowledge: String,
}

impl Component for ListView {
    type Message = Messages;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            selected_knowledge: "root".into(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::SelectKnowledge(id) => {
                self.selected_knowledge = id;
                true
            },
            Messages::ChangeView(view, id) => {
                self.props.change_view.emit((view, id));
                true
            },
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let knowledge = match KnowledgeService::load_knowledge(&self.selected_knowledge) {
            Ok(data) => data,
            Err(error) => return render_error(error.to_string()),
        };

        let add_clicked = self.link.callback(|_| Messages::ChangeView("add".into(), None));

        html! {
            <div class="knowledge animation-fade">
                <PageHeader title=&knowledge.name
                            description=&knowledge.description>
                    <ContextMenu>
                        <Svg class="input-box-icon animation-twist-grow" src="icons/cog.svg" />
                        <ContextMenuContent>
                            <Button clicked=add_clicked>
                                {"Add knowledge"}
                            </Button>
                            { self.render_edit_button() }
                        </ContextMenuContent>
                    </ContextMenu>
                </PageHeader>
                <div class="knowledge-layout">
                    { self.render_menu() }
                    { render_knowledge_data(&knowledge.id) }
                </div>
            </div>
        }
    }
}

impl ListView {
    fn render_menu(&self) -> Html {
        let list = match KnowledgeService::list_knowledge() {
            Ok(data) => data,
            Err(_) => return html! {},
        };

        html! {
            <div class="knowledge-menu">
                { for list.iter().map(|item| {
                    let id = item.id.clone();
                    html! {
                        <div class="knowledge-menuitem" onclick=self.link.callback(move |_| Messages::SelectKnowledge(id.clone()))>
                            {&item.name}
                        </div>
                    }
                }) }
            </div>
        }
    }

    fn render_edit_button(&self) -> Html {
        if self.selected_knowledge == "root" {
            return html! {};
        }

        let id = self.selected_knowledge.clone();
        let edit_clicked = self.link.callback(move |_| Messages::ChangeView("edit".into(), Some(id.clone())));

        html! {
            <Button clicked=edit_clicked>
                {"Edit knowledge"}
            </Button>
        }
    }
}

fn render_error(error: String) -> Html {
    html! {
        <div class="knowledge animation-fade">
            <PageHeader title="Error"
                        description=&error>
            </PageHeader>
        </div>
    }
}

fn render_knowledge_data(id: &str) -> Html {
    let knowledge_data = match KnowledgeService::load_knowledge_data(id) {
        Ok(data) => data,
        Err(error) => return render_error(error.to_string()),
    };

    html! {
        <div class="knowledge-content">
        { match knowledge_data.data_type {
            KnowledgeDataType::Markdown => render_markdown(knowledge_data.data.clone()),
        }}
        </div>
    }
}

fn render_markdown(data: Vec<u8>) -> Html {
    let div = yew::utils::document().create_element("div").unwrap();

    let markdown = match std::str::from_utf8(&data) {
        Ok(data) => data,
        Err(_) => return html! {},
    };

    let html = parse_markdown_to_html(markdown);

    div.set_inner_html(&html);
    div.set_attribute("class", "markdown").unwrap();

    Html::VRef(div.into())
}