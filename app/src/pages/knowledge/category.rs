use yew::prelude::*;

use crate::components::Svg;
use crate::services::Knowledge;

#[derive(Debug)]
pub struct KnowledgeCategory {
    pub name: String,
    pub knowledge: Vec<Knowledge>,
}

impl KnowledgeCategory {
    pub fn new(name: String) -> Self {
        Self {
            name,
            knowledge: vec![],
        }
    }
}

pub enum Messages {
    Toggle,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
	#[prop_or_default]
    pub name: String,
	#[prop_or_default]
    pub children: Children,
}

pub struct Category {
    props: Props,
    link: ComponentLink<Self>,
    open: bool,
}

impl Component for Category {
    type Message = Messages;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            open: true,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::Toggle => {
                self.open = !self.open;
                true
            },
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let icon_class = match self.open {
            true => "knowledge-category-icon open",
            false => "knowledge-category-icon",
        };

        html! {
            <>
            <div class="knowledge-category" onclick=self.link.callback(move |_| Messages::Toggle)>
                <Svg class=icon_class src="icons/chevron_right.svg" />
                {&self.props.name}
            </div>
            {
                match self.open {
                    true => html! {
                        <>
                        { self.props.children.clone() }
                        </>
                    },
                    false => html! {},
                }
            }
            </>
        }
    }
}