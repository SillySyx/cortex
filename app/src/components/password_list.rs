use yew::{
    prelude::*, 
    web_sys::HtmlInputElement,
};

use super::{Button, ContextMenu, ContextMenuContent};
use crate::services::{PasswordService, Password, Category};

pub enum Views {
    ListPasswords,
    NewPassword,
    EditPassword,
    NewCategory,
    EditCategory,
    ImportExport,
}

pub enum Messages {
    ChangeView(Views),

    AddCategory(String),
    EditCategory(String, String),
    RemoveCategory(String),

    AddPassword(String, String, String, String),
    CopyPassword(String, String),
    EditPassword(String, String, Option<String>, Option<String>, Option<String>),
    RemovePassword(String, String),

    UpdateSearchText(String),
    SearchKeyPressed(KeyboardEvent),
}

pub struct PasswordList {
    link: ComponentLink<Self>,
    focus_ref: NodeRef,
    search_text: String,
    passwords: Vec<Category>,
    context_menu_open: bool,
    view: Views,
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
            view: Views::ListPasswords,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::ChangeView(view) => {
                self.view = view;
                self.context_menu_open = false;
                true
            },
            Messages::AddCategory(name) => {
                // check that no category with that namn exists
                self.passwords.push(Category {
                    title: name,
                    passwords: vec![],
                });
                PasswordService::save_passwords(&self.passwords);
                self.view = Views::ListPasswords;
                true
            },
            Messages::EditCategory(category_id, new_name) => {
                match self.passwords.iter_mut().find(|c| c.title == category_id) {
                    Some(category) => {
                        category.title = new_name;
                    },
                    None => {},
                };
                PasswordService::save_passwords(&self.passwords);
                self.view = Views::ListPasswords;
                true
            },
            Messages::RemoveCategory(_category_id) => {
                // remove category
                PasswordService::save_passwords(&self.passwords);
                self.view = Views::ListPasswords;
                true
            },
            Messages::AddPassword(category_id, name, description, password) => {
                match self.passwords.iter_mut().find(move |c| c.title == category_id) {
                    Some(category) => {
                        category.passwords.push(Password {
                            name,
                            description,
                            password,
                        });
                    },
                    None => {},
                };
                PasswordService::save_passwords(&self.passwords);
                self.view = Views::ListPasswords;
                true
            }
            Messages::CopyPassword(_category_id, _password_id) => {
                yew::services::ConsoleService::log("Copy password");
                false
            },
            Messages::EditPassword(_category_id, _password_id, _new_name, _new_desc, _new_password) => {
                yew::services::ConsoleService::log("EditPassword");
                PasswordService::save_passwords(&self.passwords);
                self.view = Views::ListPasswords;
                false
            },
            Messages::RemovePassword(_category_id, _password_id) => {
                // remove password from category
                PasswordService::save_passwords(&self.passwords);
                self.view = Views::ListPasswords;
                true
            },
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
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(input) = self.focus_ref.cast::<HtmlInputElement>() {
                let _ = input.focus();
            }
        }
    }

    fn view(&self) -> Html {

        html! {
            <div class="password-list">
                {
                    match self.view {
                        Views::ListPasswords => self.render_password_list(),
                        Views::NewPassword => self.render_new_password(),
                        Views::EditPassword => self.render_edit_password(),
                        Views::NewCategory => self.render_new_category(),
                        Views::EditCategory => self.render_edit_category(),
                        Views::ImportExport => self.render_import_export(),
                    }
                }
            </div>
        }
    }
}

impl PasswordList {
    fn render_password_list(&self) -> Html {
        let categories = filter_categories(&self.passwords, self.search_text.clone());

        html! {
            <div class="animation-fade">
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
                        <Button active=false clicked=self.link.callback(|_| Messages::ChangeView(Views::NewCategory))>
                            {"New category"}
                        </Button>
                        <Button active=false clicked=self.link.callback(|_| Messages::ChangeView(Views::ImportExport))>
                            {"Import/Export"}
                        </Button>
                    </ContextMenuContent>
                </ContextMenu>
            </header>

            { for categories.iter().map(|category| self.render_category(category)) }
            </div>
        }
    }

    fn render_category(&self, category: &Category) -> Html {
        let edit_category = self.link.callback(move |_| Messages::ChangeView(Views::EditCategory));

        let new_category = self.link.callback(move |_| Messages::ChangeView(Views::NewPassword));

        html! {
            <div class="animation-fade">
                <img class="category-icon" src="icons/edit.svg" alt="Edit category" onclick=edit_category />
                <img class="category-icon" src="icons/add.svg" alt="New password" onclick=new_category />
                <h1 class="category-title">{&category.title}</h1>
                <div class="category">
                    { for category.passwords.iter().map(|password| self.render_password(category.title.clone(), password)) }
                </div>
            </div>
        }
    }

    fn render_password(&self, category_id: String, password: &Password) -> Html {
        let category_id = category_id.clone();
        let password_id = password.name.clone();
        let copy_password = self.link.callback(move |_| Messages::CopyPassword(category_id.clone(), password_id.clone()));

        let edit_password = self.link.callback(move |_| Messages::ChangeView(Views::EditPassword));

        html! {
            <div class="password animation-fade">
                <h1 class="password-title">{&password.name}</h1>
                <p class="password-description">{&password.description}</p>
                <div class="password-icons">
                    <img class="password-icon" src="icons/key.svg" alt="Copy password" onclick=copy_password />
                    <img class="password-icon" src="icons/edit.svg" alt="Edit password" onclick=edit_password />
                </div>
            </div>
        }
    }

    fn render_new_password(&self) -> Html {
        let category_id = "".to_string(); 
        let add_password = self.link.callback(move |_| Messages::AddPassword(category_id.clone(), String::from("test1"), String::from("test2"), String::from("test3")));

        html! {
            <div class="animation-fade">
                <div>{"new password"}</div>
                <Button active=false clicked=add_password>
                    {"Add"}
                </Button>
                <Button active=false clicked=self.link.callback(|_| Messages::ChangeView(Views::ListPasswords))>
                    {"Back"}
                </Button>
            </div>
        }
    }

    fn render_edit_password(&self) -> Html {
        let category_id = "".to_string(); 
        let password_id = "".to_string();
        let edit_password = self.link.callback(move |_| Messages::EditPassword(category_id.clone(), password_id.clone(), Some(String::from("new")), None, None));
        
        let category_id = "".to_string(); 
        let password_id = "".to_string();
        let remove_password = self.link.callback(move |_| Messages::RemovePassword(category_id.clone(), password_id.clone()));

        html! {
            <div class="animation-fade">
                <div>{"edit password"}</div>
                <Button active=false clicked=edit_password>
                    {"Change"}
                </Button>
                <Button active=false clicked=remove_password>
                    {"Remove"}
                </Button>
                <Button active=false clicked=self.link.callback(|_| Messages::ChangeView(Views::ListPasswords))>
                    {"Back"}
                </Button>
            </div>
        }
    }

    fn render_new_category(&self) -> Html {
        html! {
            <div class="animation-fade">
                <div>{"new category"}</div>
                <Button active=false clicked=self.link.callback(|_| Messages::AddCategory(String::from("test")))>
                    {"Add"}
                </Button>
                <Button active=false clicked=self.link.callback(|_| Messages::ChangeView(Views::ListPasswords))>
                    {"Back"}
                </Button>
            </div>
        }
    }

    fn render_edit_category(&self) -> Html {
        let category_id = "".to_string();
        let edit_category = self.link.callback(move |_| Messages::EditCategory(category_id.clone(), String::from("new")));

        let category_id = "".to_string();
        let remove_category = self.link.callback(move |_| Messages::RemoveCategory(category_id.clone()));

        html! {
            <div class="animation-fade">
                <div>{"edit category"}</div>
                <Button active=false clicked=edit_category>
                    {"Change"}
                </Button>
                <Button active=false clicked=remove_category>
                    {"Remove"}
                </Button>
                <Button active=false clicked=self.link.callback(|_| Messages::ChangeView(Views::ListPasswords))>
                    {"Back"}
                </Button>
            </div>
        }
    }

    fn render_import_export(&self) -> Html {
        html! {
            <div class="animation-fade">
                <div>{"Import/Export"}</div>
                <Button active=false clicked=self.link.callback(|_| Messages::ChangeView(Views::ListPasswords))>
                    {"Back"}
                </Button>
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