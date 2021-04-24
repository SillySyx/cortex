use yew::prelude::*;

use super::list::ListView;
use super::add_category::AddCategoryView;
use super::add_password::AddPasswordView;
use super::edit_category::EditCategoryView;
use super::edit_password::EditPasswordView;
use super::import_export::ImportExportView;

pub enum Views {
    ListPasswords,
    NewPassword,
    EditPassword,
    NewCategory,
    EditCategory,
    ImportExport,
}

pub enum Messages {
    ChangeView(Views, Option<String>),
}

pub struct PasswordsPage {
    link: ComponentLink<Self>,
    view: Views,
    id: String,
}

impl Component for PasswordsPage {
    type Message = Messages;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            view: Views::ListPasswords,
            id: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::ChangeView(view, id) => {
                self.view = view;
                self.id = match id {
                    Some(value) => value,
                    None => String::from(""),
                };
                true
            },
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match self.view {
            Views::ListPasswords => html! {
                <ListView change_view=self.link.callback(|(view, id)| Messages::ChangeView(view, id)) />
            },
            Views::NewPassword => html! {
                <AddPasswordView category_id=self.id.clone() change_view=self.link.callback(|(view, id)| Messages::ChangeView(view, id)) />
            },
            Views::EditPassword => html! {
                <EditPasswordView password_id=self.id.clone() change_view=self.link.callback(|(view, id)| Messages::ChangeView(view, id)) />
            },
            Views::NewCategory => html! {
                <AddCategoryView change_view=self.link.callback(|(view, id)| Messages::ChangeView(view, id)) />
            },
            Views::EditCategory => html! {
                <EditCategoryView category_id=self.id.clone() change_view=self.link.callback(|(view, id)| Messages::ChangeView(view, id)) />
            },
            Views::ImportExport => html! {
                <ImportExportView change_view=self.link.callback(|(view, id)| Messages::ChangeView(view, id)) />
            },
        }
    }
}