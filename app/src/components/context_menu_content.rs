use yew::prelude::*;

pub enum Messages {
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

pub struct ContextMenuContent {
    props: Props,
}

impl Component for ContextMenuContent {
    type Message = Messages;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            props,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="context-menu-content">
                { self.props.children.clone() }
            </div>
        }
    }
}