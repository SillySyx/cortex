use yew::prelude::*;

pub enum Messages {
}

pub struct TodoPage {
}

impl Component for TodoPage {
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
            <div class="todo">
                <h1>{"Todo"}</h1>
                <p>{"Not implemented yet"}</p>
            </div>
        }
    }
}