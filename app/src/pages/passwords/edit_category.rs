use yew::prelude::*;

use crate::components::{Button, PageHeader, InputBox, Error};
use crate::services::PasswordService;

use super::page::Views;

pub enum Messages {
    UpdateName(String),
    BackClicked,
    SaveClicked,
    RemoveClicked,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub change_view: Callback<(Views, Option<String>)>,
    pub category_id: String,
}

pub struct EditCategoryView {
    props: Props,
    link: ComponentLink<Self>,
    category_id: String,
    error: String,
    name: String,
    name_error: String,
}

impl Component for EditCategoryView {
    type Message = Messages;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut error = String::new();

        let name = match PasswordService::load_category(&props.category_id) {
            Ok(value) => value.title,
            Err(_) => {
                error = String::from("Failed to load category");
                String::from("")
            },
        };

        let category_id = props.category_id.clone();

        Self {
            props,
            link,
            category_id,
            error,
            name,
            name_error: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::UpdateName(name) => {
                self.name = name;

                self.name_error = match self.name.is_empty() {
                    true => String::from("No name entered"),
                    false => String::from(""),
                };

                true
            },
            Messages::BackClicked => {
                self.props.change_view.emit((Views::ListPasswords, None));
                false
            },
            Messages::SaveClicked => {
                if self.name.is_empty() {
                    return false;
                }

                if let Ok(_) = PasswordService::update_category(&self.category_id, Some(self.name.clone())) {
                    self.props.change_view.emit((Views::ListPasswords, None));
                }

                false
            },
            Messages::RemoveClicked => {
                if let Ok(_) = PasswordService::remove_category(&self.category_id) {
                    self.props.change_view.emit((Views::ListPasswords, None));
                }

                false
            },
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if !self.error.is_empty() {
            return html! {
                <>
                    <Error text=&self.error />
                    <div class="button-grid">
                        <Button clicked=self.link.callback(|_| Messages::BackClicked)>
                            {"Back"}
                        </Button>
                    </div>
                </>
            };
        }

        let disabled = self.name.is_empty();

        html! {
            <div class="animation-fade">
                <PageHeader title="Edit category" 
                            description="Categories are used to group similar passwords together so that it's easier to overview." />

                <InputBox
                    label="Name"
                    placeholder="Enter name"
                    mandatory=true
                    value=self.name.clone()
                    error=self.name_error.clone()
                    value_changed=self.link.callback(|value| Messages::UpdateName(value))>
                </InputBox>
                
                <div class="button-grid">
                    <Button disabled=disabled clicked=self.link.callback(|_| Messages::SaveClicked)>
                        {"Save"}
                    </Button>
                    <Button clicked=self.link.callback(|_| Messages::BackClicked)>
                        {"Back"}
                    </Button>
                </div>

                <Error title="Danger" text="Removing this category will also remove all of its passwords, it's not possible to restore any passwords once they have been removed.">
                    <Button class="error-button" clicked=self.link.callback(|_| Messages::RemoveClicked)>
                        {"Remove"}
                    </Button>
                </Error>
            </div>
        }
    }
}