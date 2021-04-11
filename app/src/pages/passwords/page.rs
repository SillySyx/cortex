use yew::prelude::*;

use crate::components::{Button, Error};
use crate::services::{LoginService, PasswordService};

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
    DecryptError,
}

pub enum Messages {
    ChangeView(Views, Option<String>),
    Logout,
    ResetData,
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
            Messages::Logout => {
                LoginService::logout();
                false
            },
            Messages::ResetData => {
                PasswordService::reset_data();
                LoginService::logout();
                false
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
            Views::DecryptError => self.render_decrypt_error(),
        }
    }
}

impl PasswordsPage {
    fn render_decrypt_error(&self) -> Html {
        html! {
            <div class="animation-fade">
                <Error title="Invalid password specified" icon="icons/error.svg">
                    <Button clicked=self.link.callback(|_| Messages::Logout)>
                        {"Reenter password"}
                    </Button>
                    <Button class="error-button" clicked=self.link.callback(|_| Messages::ResetData)>
                        {"Reset application data"}
                    </Button>
                </Error>
            </div>
        }
    }
}