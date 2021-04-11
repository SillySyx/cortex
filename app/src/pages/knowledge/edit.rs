use yew::prelude::*;

pub enum Messages {
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub change_view: Callback<(String, Option<String>)>,
}

pub struct EditView {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for EditView {
    type Message = Messages;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
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