use yew::prelude::*;

pub enum Messages {
}

pub struct KnowledgebasePage {
}

impl Component for KnowledgebasePage {
    type Message = Messages;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
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
            <div class="knowledgebase">
                <h1>{"Knowledgebase"}</h1>
                <p>{"Not implemented yet"}</p>
            </div>
        }
    }
}