use yew::prelude::*;

use crate::components::{Button, PageHeader, InputBox};
use crate::services::PasswordService;

use super::page::Views;

pub enum Messages {
    UpdateName(String),
    AddClicked,
    BackClicked,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub change_view: Callback<(Views, Option<String>)>,
}

pub struct AddCategoryView {
    props: Props,
    link: ComponentLink<Self>,
    name: String,
    name_error: String,
}

impl Component for AddCategoryView {
    type Message = Messages;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            name: String::from(""),
            name_error: String::from(""),
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
            Messages::AddClicked => {
                if self.name.is_empty() {
                    return false;
                }

                if let Ok(_) = PasswordService::create_category(self.name.clone()) {
                    self.props.change_view.emit((Views::ListPasswords, None));
                }

                false
            },
            Messages::BackClicked => {
                self.props.change_view.emit((Views::ListPasswords, None));
                false
            },
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let disabled = self.name.is_empty();

        html! {
            <div class="animation-fade">
                <PageHeader title="New category" 
                            description="Categories are used to group similar passwords together so that it's easier to overview." />

                <InputBox
                    label="Name"
                    placeholder="Enter name"
                    focus=true
                    mandatory=true
                    value=self.name.clone()
                    error=self.name_error.clone()
                    value_changed=self.link.callback(|value| Messages::UpdateName(value))>
                </InputBox>

                <div class="button-grid">
                    <Button disabled=disabled clicked=self.link.callback(|_| Messages::AddClicked)>
                        {"Add"}
                    </Button>
                    <Button clicked=self.link.callback(|_| Messages::BackClicked)>
                        {"Back"}
                    </Button>
                </div>
            </div>
        }
    }
}