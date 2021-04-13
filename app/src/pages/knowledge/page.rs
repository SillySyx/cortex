use yew::prelude::*;

use super::{
    list::ListView,
    add::AddView,
    edit::EditView
};

pub enum Messages {
    ChangeView(String, Option<String>),
}

pub struct KnowledgePage {
    link: ComponentLink<Self>,
    view: String,
    id: Option<String>,
}

impl Component for KnowledgePage {
    type Message = Messages;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            view: "list".into(),
            id: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::ChangeView(view, id) => {
                self.view = view;
                self.id = id;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let id = match self.id.clone() {
            Some(value) => value,
            None => "root".into(),
        };

        match self.view.as_str() {
            "edit" => html! {
                <EditView id=id change_view=self.link.callback(|(view, id)| Messages::ChangeView(view, id)) />
            },
            "add" => html! {
                <AddView id=id change_view=self.link.callback(|(view, id)| Messages::ChangeView(view, id)) />
            },
            _ => html! {
                <ListView id=id change_view=self.link.callback(|(view, id)| Messages::ChangeView(view, id)) />
            },
        }
    }
}