use yew::prelude::*;
use yew::web_sys::HtmlInputElement;
use yew::services::{TimeoutService, Task};

use std::time::Duration;

use crate::components::{Button, ContextMenu, ContextMenuContent, PageHeader, InputBox, Svg};
use crate::services::{PasswordService, Password, Category, ClipboardService};

use super::page::Views;

pub enum Messages {
    ChangeView(Views, Option<String>),

    CopyPassword(String),
    CopyDescription(String),

    UpdateSearchText(String),
    ClearSearchText,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub change_view: Callback<(Views, Option<String>)>,
}

pub struct ListView {
    props: Props,
    link: ComponentLink<Self>,
    search_ref: NodeRef,
    search_text: String,
    context_menu_open: bool,
    categories: Vec<Category>,
    timeout_task: Option<Box<dyn Task>>,
}

impl Component for ListView {
    type Message = Messages;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let categories = match PasswordService::list_categories() {
            Ok(value) => value,
            Err(_) => {
                props.change_view.emit((Views::DecryptError, None));
                vec![]
            },
        };

        Self {
            props,
            link,
            search_ref: NodeRef::default(),
            search_text: String::new(),
            context_menu_open: false,
            categories,
            timeout_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::ChangeView(view, id) => {
                self.context_menu_open = false;
                self.props.change_view.emit((view, id));
                false
            },
            Messages::UpdateSearchText(value) => {
                self.search_text = value;
                true
            },
            Messages::ClearSearchText => {
                self.search_text = String::from("");
                true
            },
            Messages::CopyPassword(password_id) => {
                if let Ok(password) = PasswordService::load_password(&password_id) {
                    ClipboardService::copy_to_clipboard(password.password.clone());

                    let task = TimeoutService::spawn(Duration::from_secs(5), Callback::from(|_| {
                        ClipboardService::copy_to_clipboard("".to_string());
                    }));
                    self.timeout_task = Some(Box::new(task));
                }
                false
            },
            Messages::CopyDescription(password_id) => {
                if let Ok(password) = PasswordService::load_password(&password_id) {
                    ClipboardService::copy_to_clipboard(password.description.clone());
                }
                false
            },
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(input) = self.search_ref.cast::<HtmlInputElement>() {
                let _ = input.focus();
            }
        }
    }

    fn view(&self) -> Html {
        let categories = filter_categories(&self.categories, self.search_text.clone());

        let add_clicked = self.link.callback(|_| Messages::ChangeView(Views::NewCategory, None));
        let import_export_clicked = self.link.callback(|_| Messages::ChangeView(Views::ImportExport, None));

        html! {
            <div class="animation-fade">
                <PageHeader title={"Password manager"} description={"Handle your passwords with ease."}>
                    <ContextMenu open=self.context_menu_open>
                        <Svg class="input-box-icon animation-twist-grow" src="icons/cog.svg" />
                        <ContextMenuContent>
                            <Button clicked=add_clicked>
                                {"Add category"}
                            </Button>
                            <Button clicked=import_export_clicked>
                                {"Import/Export"}
                            </Button>
                        </ContextMenuContent>
                    </ContextMenu>
                </PageHeader>

                <InputBox 
                    class="search-box"
                    value=self.search_text.clone()
                    placeholder="Search for passwords"
                    value_changed=self.link.callback(|value| Messages::UpdateSearchText(value))
                    aborted=self.link.callback(|_| Messages::ClearSearchText)>
                </InputBox>

                { match categories.is_empty() {
                    false => html! {},
                    true => html! {
                        <div class="passwords-empty">
                            <div class="link-button animation-highlight" onclick=self.link.callback(|_| Messages::ChangeView(Views::NewCategory, None))>
                                {"Add category"}
                            </div>
                        </div>
                    },
                }}

                { for categories.iter().map(|category| self.render_category(category)) }
            </div>
        }
    }
}

impl ListView {
    fn render_category(&self, category: &Category) -> Html {
        let category_id = category.id.clone();
        let edit_category = self.link.callback(move |_| Messages::ChangeView(Views::EditCategory, Some(category_id.clone())));

        let category_id = category.id.clone();
        let new_password = self.link.callback(move |_| Messages::ChangeView(Views::NewPassword, Some(category_id.clone())));

        let category_id = category.id.clone();
        let new_password2 = self.link.callback(move |_| Messages::ChangeView(Views::NewPassword, Some(category_id.clone())));

        html! {
            <>
            <div class="category animation-fade">
                <h2 class="category-title">{&category.title}</h2>
                <ContextMenu open=self.context_menu_open>
                    <Svg class="category-icon animation-twist-grow" src="icons/cog.svg" />
                    <ContextMenuContent>
                        <Button clicked=new_password.clone()>
                            {"New password"}
                        </Button>
                        <Button clicked=edit_category>
                            {"Edit category"}
                        </Button>
                    </ContextMenuContent>
                </ContextMenu>
            </div>

            { match category.passwords.is_empty() {
                false => html! {},
                true => html! {
                    <div class="passwords-empty">
                        <div class="link-button animation-highlight" onclick=new_password2>
                            {"Add password"}
                        </div>
                    </div>
                },
            }}

            <div class="category-items animation-fade">
                { for category.passwords.iter().map(|password| self.render_password(password)) }
            </div>
            </>
        }
    }

    fn render_password(&self, password: &Password) -> Html {
        let password_id = password.id.clone();
        let copy_password = self.link.callback(move |_| Messages::CopyPassword(password_id.clone()));

        let password_id = password.id.clone();
        let copy_desc = self.link.callback(move |_| Messages::CopyDescription(password_id.clone()));

        let password_id = password.id.clone();
        let edit_password = self.link.callback(move |_| Messages::ChangeView(Views::EditPassword, Some(password_id.clone())));

        html! {
            <div class="password animation-fade">
                <div>
                    <h3 class="password-title">
                        <span onclick=edit_password>{&password.name}</span>
                    </h3>
                    <p class="password-description animation-highlight">
                        <span onclick=copy_desc>{&password.description}</span>
                    </p>
                </div>
                <div>
                    <Svg class="password-icon animation-grow animation-highlight" src="icons/key.svg" clicked=copy_password />
                </div>
            </div>
        }
    }
}

fn filter_categories(categories: &Vec<Category>, search: String) -> Vec<Category> {
    let mut categories = categories.to_vec();

    if search.is_empty() {
        return categories.to_vec();
    }

    categories.iter_mut().map(|category| {
        let mut category = category.to_owned();

        category.passwords = category.passwords.iter().filter_map(|p| {
            if p.name.to_lowercase().contains(&search.to_lowercase()) {
                return Some(p.to_owned());
            }

            if p.description.to_lowercase().contains(&search.to_lowercase()) {
                return Some(p.to_owned());
            }

            None
        }).collect();

        category
    }).collect()
}