use yew::prelude::*;

pub enum Messages {
}

pub struct HomePage {
}

impl Component for HomePage {
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
            <div class="home-page">
                <h1>{"Home"}</h1>
                <p>{"Not implemented yet"}</p>
                <img src="icons/home.svg" alt="Home" />
            </div>
        }
    }
}