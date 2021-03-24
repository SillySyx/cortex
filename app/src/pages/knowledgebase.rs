use yew::prelude::*;

use crate::components::PageHeader;

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
            <>
            <PageHeader title={"Knowledgebase"}
                        description={"Save things you dont want to forget."}>
            </PageHeader>
            <p>{"Not yet implemented"}</p>
            </>
        }
    }
}