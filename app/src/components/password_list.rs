use yew::{
    prelude::*, 
    web_sys::HtmlInputElement,
};

use super::{Button, ContextMenu, ContextMenuContent, PasswordEditor, PasswordCategoryEditor};
use crate::services::{PasswordService, Password, Category, ClipboardService};

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
    ChangeViewWithId(Views, Option<String>, Option<String>),

    AddCategory(String),
    EditCategory(String, String),
    RemoveCategory(String),

    AddPassword(String, String, String, String),
    CopyPassword(String, String),
    EditPassword(String, String, String, String, String),
    RemovePassword(String, String),

    UpdateSearchText(String),
    SearchKeyPressed(KeyboardEvent),
}

pub struct PasswordList {
    link: ComponentLink<Self>,
    search_ref: NodeRef,
    search_text: String,
    passwords: Vec<Category>,
    context_menu_open: bool,
    view: Views,
    selected_category_id: String,
    selected_password_id: String,
}

impl Component for PasswordList {
    type Message = Messages;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            search_ref: NodeRef::default(),
            search_text: String::new(),
            passwords: PasswordService::load_passwords(),
            context_menu_open: false,
            view: Views::ListPasswords,
            selected_category_id: String::new(),
            selected_password_id: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::ChangeView(view) => {
                self.view = view;
                self.context_menu_open = false;
                true
            },
            Messages::ChangeViewWithId(view, category_id, password_id) => {
                if let Some(id) = category_id {
                    self.selected_category_id = id;
                }
                if let Some(id) = password_id {
                    self.selected_password_id = id;
                }
                self.view = view;
                self.context_menu_open = false;
                true
            },
            Messages::AddCategory(name) => {
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
            Messages::RemoveCategory(category_id) => {
                if let Some(index) = self.passwords.iter().position(|c| c.title == category_id) {
                    self.passwords.remove(index);
                    PasswordService::save_passwords(&self.passwords);
                }
                self.view = Views::ListPasswords;
                true
            },
            Messages::AddPassword(category_id, name, description, password) => {
                if let Some(category) = self.passwords.iter_mut().find(|c| c.title == category_id) {
                    category.passwords.push(Password {
                        name,
                        description,
                        password,
                    });
                }
                PasswordService::save_passwords(&self.passwords);
                self.view = Views::ListPasswords;
                true
            }
            Messages::CopyPassword(category_id, password_id) => {
                if let Some(category) = self.passwords.iter().find(|c| c.title == category_id) {
                    if let Some(password) = category.passwords.iter().find(|p| p.name == password_id) {
                        ClipboardService::copy_to_clipboard(password.password.clone());
                    }
                }
                false
            },
            Messages::EditPassword(category_id, password_id, name, desc, pass) => {
                if let Some(category) = self.passwords.iter_mut().find(|c| c.title == category_id) {
                    if let Some(password) = category.passwords.iter_mut().find(|p| p.name == password_id) {
                        password.name = name;
                        password.description = desc;
                        password.password = pass;

                        PasswordService::save_passwords(&self.passwords);
                    }
                }
                self.view = Views::ListPasswords;
                true
            },
            Messages::RemovePassword(category_id, password_id) => {
                if let Some(category) = self.passwords.iter_mut().find(|c| c.title == category_id) {
                    if let Some(index) = category.passwords.iter().position(|p| p.name == password_id) {
                        category.passwords.remove(index);
                        PasswordService::save_passwords(&self.passwords);
                    }
                }
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
            if let Some(input) = self.search_ref.cast::<HtmlInputElement>() {
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
                    ref=self.search_ref.clone()
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
        let category_id = category.title.clone();
        let edit_category = self.link.callback(move |_| Messages::ChangeViewWithId(Views::EditCategory, Some(category_id.clone()), None));

        let category_id = category.title.clone();
        let new_password = self.link.callback(move |_| Messages::ChangeViewWithId(Views::NewPassword, Some(category_id.clone()), None));

        html! {
            <div class="animation-fade">
                <img class="category-icon" src="icons/edit.svg" alt="Edit category" onclick=edit_category />
                <img class="category-icon" src="icons/add.svg" alt="New password" onclick=new_password />
                <h1 class="category-title">{&category.title}</h1>
                <div class="category">
                    { for category.passwords.iter().map(|password| self.render_password(category.title.clone(), password)) }
                </div>
            </div>
        }
    }

    fn render_password(&self, category_id: String, password: &Password) -> Html {
        let category_id_clone = category_id.clone();
        let password_id = password.name.clone();
        let copy_password = self.link.callback(move |_| Messages::CopyPassword(category_id_clone.clone(), password_id.clone()));

        let category_id_clone = category_id.clone();
        let password_id = password.name.clone();
        let edit_password = self.link.callback(move |_| Messages::ChangeViewWithId(Views::EditPassword, Some(category_id_clone.clone()), Some(password_id.clone())));

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
        let category_id = self.selected_category_id.clone();

        html! {
            <PasswordEditor
                id="".to_string()
                name="".to_string()
                description="".to_string()
                password="".to_string()
                new_mode=true
                added=self.link.callback(move |(name, desc, pass)| Messages::AddPassword(category_id.clone(), name, desc, pass))
                backed=self.link.callback(|_| Messages::ChangeView(Views::ListPasswords))
                saved=self.link.callback(|_| Messages::ChangeView(Views::ListPasswords))
                removed=self.link.callback(|_| Messages::ChangeView(Views::ListPasswords)) />
        }
    }

    fn render_edit_password(&self) -> Html {
        let category_id = self.selected_category_id.clone();
        let password_id = self.selected_password_id.clone();
        
        let category = match self.passwords.iter().find(|c| c.title == category_id) {
            Some(category) => category,
            None => return self.render_error("failed to find category"),
        };

        let password = match category.passwords.iter().find(|p| p.name == password_id) {
            Some(password) => password,
            None => return self.render_error("failed to find password"),
        };

        let category_id = self.selected_category_id.clone();
        let saved = self.link.callback(move |(id, name, desc, pass)| Messages::EditPassword(category_id.clone(), id, name, desc, pass));

        let category_id = self.selected_category_id.clone();
        let removed = self.link.callback(move |id| Messages::RemovePassword(category_id.clone(), id));

        let id = password.name.clone();
        let name = password.name.clone();
        let desc = password.description.clone();
        let pass = password.password.clone();

        html! {
            <PasswordEditor
                id=id
                name=name
                description=desc
                password=pass
                new_mode=false
                added=self.link.callback(|_| Messages::ChangeView(Views::ListPasswords))
                backed=self.link.callback(|_| Messages::ChangeView(Views::ListPasswords))
                saved=saved
                removed=removed />
        }
    }

    fn render_new_category(&self) -> Html {
        html! {
            <PasswordCategoryEditor
                id="".to_string()
                name="".to_string()
                new_mode=true
                added=self.link.callback(|name| Messages::AddCategory(name))
                backed=self.link.callback(|_| Messages::ChangeView(Views::ListPasswords))
                saved=self.link.callback(|_| Messages::ChangeView(Views::ListPasswords))
                removed=self.link.callback(|_| Messages::ChangeView(Views::ListPasswords)) />
        }
    }

    fn render_edit_category(&self) -> Html {
        let category_id = self.selected_category_id.clone();

        let category = match self.passwords.iter().find(|c| c.title == category_id) {
            Some(category) => category,
            None => return self.render_error("failed to find category"),
        };

        let id = category.title.clone();
        let name = category.title.clone();

        html! {
            <PasswordCategoryEditor
                id=id
                name=name
                new_mode=false
                added=self.link.callback(|_| Messages::ChangeView(Views::ListPasswords))
                backed=self.link.callback(|_| Messages::ChangeView(Views::ListPasswords))
                saved=self.link.callback(|(id, name)| Messages::EditCategory(id, name))
                removed=self.link.callback(|id| Messages::RemoveCategory(id)) />
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

    fn render_error(&self, message: &'static str) -> Html {
        html! {
            <div class="animation-fade error-message">
                <p>{message}</p>
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