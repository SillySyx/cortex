use yew::prelude::*;

use crate::components::PageHeader;

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
            <>
            <PageHeader title={"Todo"}
                        description={"Not implemented yet"}>
            </PageHeader>
            </>
        }
    }
}