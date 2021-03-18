use yew::{
    prelude::*, 
    web_sys::HtmlInputElement,
};

use super::{Button, ContextMenu, ContextMenuContent};
use crate::services::{PasswordService, Password, Category};

pub enum Messages {
    NewPasswordClicked,
    NewCategoryClicked,
    ImportClicked,
    ExportClicked,
    UpdateSearchText(String),
    SearchKeyPressed(KeyboardEvent),
    CopyPassword(String),
}

pub struct PasswordList {
    link: ComponentLink<Self>,
    focus_ref: NodeRef,
    search_text: String,
    passwords: Vec<Category>,
    context_menu_open: bool,
}

impl Component for PasswordList {
    type Message = Messages;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            focus_ref: NodeRef::default(),
            search_text: String::new(),
            passwords: PasswordService::load_passwords(),
            context_menu_open: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::UpdateSearchText(value) => {
                self.search_text = value;
                true
            },
            Messages::SearchKeyPressed(e) => {
                if e.key() == String::from("Escape") {
                    self.search_text = String::from("");
                    return true;
                }
                false
            },
            Messages::CopyPassword(password) => {
                yew::services::ConsoleService::log(&format!("Copy password {:?}", password));
                false
            },
            Messages::NewCategoryClicked => {
                yew::services::ConsoleService::log("NewCategoryClicked");
                self.context_menu_open = false;
                true
            },
            Messages::NewPasswordClicked => {
                yew::services::ConsoleService::log("NewPasswordClicked");
                self.context_menu_open = false;
                true
            },
            Messages::ImportClicked => {
                yew::services::ConsoleService::log("ImportClicked");
                self.context_menu_open = false;
                true
            },
            Messages::ExportClicked => {
                yew::services::ConsoleService::log("ExportClicked");
                self.context_menu_open = false;
                true
            },
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(input) = self.focus_ref.cast::<HtmlInputElement>() {
                match input.focus() {
                    Ok(_) => {},
                    Err(_) => {},
                };
            }
        }
    }

    fn view(&self) -> Html {
        let categories = filter_categories(&self.passwords, self.search_text.clone());

        html! {
            <div class="password-list">
                <header class="search-box">
                    <input 
                        ref=self.focus_ref.clone()
                        value=self.search_text
                        class="main-search-box" 
                        placeholder="Search for passwords"
                        oninput=self.link.callback(|e: InputData| Messages::UpdateSearchText(e.value)) 
                        onkeyup=self.link.callback(|e| Messages::SearchKeyPressed(e)) />

                    <ContextMenu open=self.context_menu_open>
                        <img class="search-box-button" src="icons/cog.svg" alt="" />
                        <ContextMenuContent>
                            <Button active=false clicked=self.link.callback(|_| Messages::NewCategoryClicked)>
                                {"New category"}
                            </Button>
                            <Button active=false clicked=self.link.callback(|_| Messages::NewPasswordClicked)>
                                {"New password"}
                            </Button>
                            <Button active=false clicked=self.link.callback(|_| Messages::ImportClicked)>
                                {"Import"}
                            </Button>
                            <Button active=false clicked=self.link.callback(|_| Messages::ExportClicked)>
                                {"Export"}
                            </Button>
                        </ContextMenuContent>
                    </ContextMenu>
                </header>

                { for categories.iter().map(|category| render_category(category, &self.link)) }
            </div>
        }
    }
}

fn filter_categories(passwords: &Vec<Category>, search: String) -> Vec<Category> {
    let mut passwords = passwords.to_vec();

    if search.is_empty() {
        return passwords.to_vec();
    }

    passwords.iter_mut().map(|category| {
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

fn render_category(category: &Category, link: &ComponentLink<PasswordList>) -> Html {
    html! {
        <div>
            <h1 class="category-title">{&category.title}</h1>
            <div class="category">
                { for category.passwords.iter().map(|password| render_password(password, link)) }
            </div>
        </div>
    }
}

fn render_password(password: &Password, link: &ComponentLink<PasswordList>) -> Html {
    let password_clone = password.clone();
    html! {
        <div class="password">
            <h1 class="password-title">{&password.name}</h1>
            <p class="password-description">{&password.description}</p>
            <img class="password-icon" src="icons/key.svg" alt="" onclick=link.callback(move |_| Messages::CopyPassword(password_clone.password.clone())) />
        </div>
    }
}