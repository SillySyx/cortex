use yew::prelude::*;

use crate::components::{PageHeader, ContextMenu, ContextMenuContent, Svg};

#[derive(Debug, Clone)]
struct Knowledge {
    id: String,
    title: String,
    description: String,
    content: String,
    children: Vec<Self>,
}

pub enum Messages {
    SelectKnowledge(String),
}

pub struct KnowledgePage {
    link: ComponentLink<Self>,
    root: Knowledge,
    selected_knowledge: String,
    selected_parent: String,
}

impl Component for KnowledgePage {
    type Message = Messages;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let child = Knowledge {
            id: "1".into(),
            title: "test".into(),
            description: "".into(),
            content: "<p>oooo</p>".into(),
            children: vec![],
        };

        let knowledge = Knowledge {
            id: "root".into(),
            title: "Knowledge".into(),
            description: "Useful if you have a silly brain.".into(),
            content: "<p>☣ Work in progress! ☣</p>".into(),
            children: vec![child],
        };

        Self {
            link,
            root: knowledge,
            selected_knowledge: "root".into(),
            selected_parent: "".into(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::SelectKnowledge(id) => {
                self.selected_knowledge = id;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let _parent = self.select_knowledge(self.selected_parent.clone());

        let knowledge = match self.select_knowledge(self.selected_knowledge.clone()) {
            Some(value) => value,
            None => return html! {
                <p>{"Oh no!"}</p>
            },
        };

        html! {
            <div class="animation-fade">
                <PageHeader title=knowledge.title.clone()
                            description=knowledge.description.clone()>
                    <ContextMenu>
                        <Svg class="input-box-icon animation-twist-grow" src="icons/cog.svg" />
                        <ContextMenuContent>
                            <button>{"Edit knowledge"}</button>
                        </ContextMenuContent>
                    </ContextMenu>
                </PageHeader>
                { self.render_content(knowledge) }
                { self.render_children(knowledge) }
            </div>
        }
    }
}

impl KnowledgePage {
    fn select_knowledge(&self, id: String) -> Option<&Knowledge> {
        if id == String::from("root") {
            return Some(&self.root);
        }

        find_child_knowledge(&self.root, id)
    }

    fn render_content(&self, knowledge: &Knowledge) -> Html {
        if knowledge.content.is_empty() {
            return html! {};
        }
    
        let div = yew::utils::document().create_element("div").unwrap();
        div.set_inner_html(&knowledge.content);
        div.set_attribute("class", "knowledge-content").unwrap();
    
        Html::VRef(div.into())
    }

    fn render_children(&self, knowledge: &Knowledge) -> Html {
        if knowledge.children.is_empty() {
            return html! {};
        }
    
        html! {
            <div class="knowledge-children">
            { for knowledge.children.iter().map(move |child| self.render_child(child)) }
            </div>
        }
    }
    
    fn render_child(&self, child: &Knowledge) -> Html {
        let id = child.id.clone();
    
        html! {
            <div class="knowledge-child" onclick=self.link.callback(move |_| Messages::SelectKnowledge(id.clone()))>
                <h4>{&child.title}</h4>
            </div>
        }
    }
}

fn find_child_knowledge(knowledge: &Knowledge, id: String) -> Option<&Knowledge> {
    for child in knowledge.children.iter() {
        if child.id == id {
            return Some(child);
        }

        if child.children.is_empty() {
            return None;
        }

        return find_child_knowledge(child, id);
    }

    None
}