use std::vec;

use yew::{
    prelude::*, 
    web_sys::HtmlInputElement,
    services::reader::{File, FileData, ReaderService, ReaderTask},
};

use crate::components::{Button, ContextMenu, ContextMenuContent, PasswordEditor, PasswordCategoryEditor, PageHeader, InputBox};
use crate::services::{Category, ClipboardService, LoginService, Password, PasswordService};

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
    ChangeView(Views),
    ChangeViewWithId(Views, Option<String>, Option<String>),
    Logout,
    ResetData,

    AddCategory(String),
    EditCategory(String, String),
    RemoveCategory(String),

    AddPassword(String, String, String, String),
    CopyPassword(String, String),
    EditPassword(String, String, String, String, String),
    RemovePassword(String, String),

    UpdateSearchText(String),
    ClearSearchText,

    ImportClicked,
    ImportFile(File),
    ImportData(Vec<u8>),
}

pub struct PasswordsPage {
    link: ComponentLink<Self>,
    reader_service: ReaderService,
    reader_task: Option<ReaderTask>,
    upload_ref: NodeRef,
    search_ref: NodeRef,
    search_text: String,
    passwords: Vec<Category>,
    context_menu_open: bool,
    view: Views,
    selected_category_id: String,
    selected_password_id: String,
}

impl Component for PasswordsPage {
    type Message = Messages;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut view = Views::ListPasswords;

        let passwords = match PasswordService::load_passwords() {
            Some(passwords) => passwords,
            None => {
                view = Views::DecryptError;
                vec![]
            },
        };

        Self {
            link,
            reader_service: ReaderService::new(),
            reader_task: None,
            upload_ref: NodeRef::default(),
            search_ref: NodeRef::default(),
            search_text: String::new(),
            passwords,
            context_menu_open: false,
            view,
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
            Messages::Logout => {
                LoginService::logout();
                false
            },
            Messages::ResetData => {
                PasswordService::reset_data();
                LoginService::logout();
                false
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
            Messages::ClearSearchText => {
                self.search_text = String::from("");
                true
            },
            Messages::ImportClicked => {
                if let Some(input) = self.upload_ref.cast::<HtmlInputElement>() {
                    let _ = input.click();
                }
                false
            },
            Messages::ImportFile(file) => {
                let callback = self.link.callback(|data: FileData| Messages::ImportData(data.content));
                let task = self.reader_service.read_file(file, callback).unwrap();
                self.reader_task = Some(task);
                false
            },
            Messages::ImportData(data) => {
                let bytes: Vec<u8> = match serde_json::from_slice(&data) {
                    Ok(bytes) => bytes,
                    Err(_) => {
                        // show import error?!?
                        return false;
                    },
                };

                let mut passwords = match PasswordService::import_bytes(bytes) {
                    Some(passwords) => passwords,
                    None => {
                        // show import error?!?
                        return false;
                    },
                };

                self.passwords = PasswordService::combine_passwords(&self.passwords, &mut passwords);
                PasswordService::save_passwords(&self.passwords);

                self.view = Views::ListPasswords;
                true
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
        match self.view {
            Views::ListPasswords => self.render_password_list(),
            Views::NewPassword => self.render_new_password(),
            Views::EditPassword => self.render_edit_password(),
            Views::NewCategory => self.render_new_category(),
            Views::EditCategory => self.render_edit_category(),
            Views::ImportExport => self.render_import_export(),
            Views::DecryptError => self.render_decrypt_error(),
        }
    }
}

impl PasswordsPage {
    fn render_password_list(&self) -> Html {
        let categories = filter_categories(&self.passwords, self.search_text.clone());

        html! {
            <>
                <PageHeader title={"Password manager"} description={"Handle your passwords with ease."}>
                </PageHeader>

                <InputBox 
                    class="search-box"
                    value=self.search_text.clone()
                    focus=true
                    placeholder="Search for passwords"
                    value_changed=self.link.callback(|value| Messages::UpdateSearchText(value))
                    aborted=self.link.callback(|_| Messages::ClearSearchText)>
                    <ContextMenu open=self.context_menu_open>
                        <img class="search-box-button animation-grow" src="icons/cog.svg" alt="" />
                        <ContextMenuContent>
                            <Button clicked=self.link.callback(|_| Messages::ChangeView(Views::NewCategory))>
                                {"New category"}
                            </Button>
                            <Button clicked=self.link.callback(|_| Messages::ChangeView(Views::ImportExport))>
                                {"Import/Export"}
                            </Button>
                        </ContextMenuContent>
                    </ContextMenu>
                </InputBox>

                { for categories.iter().map(|category| self.render_category(category)) }
            </>
        }
    }

    fn render_category(&self, category: &Category) -> Html {
        let category_id = category.title.clone();
        let edit_category = self.link.callback(move |_| Messages::ChangeViewWithId(Views::EditCategory, Some(category_id.clone()), None));

        let category_id = category.title.clone();
        let new_password = self.link.callback(move |_| Messages::ChangeViewWithId(Views::NewPassword, Some(category_id.clone()), None));

        html! {
            <div class="animation-fade">
                <img class="category-icon animation-grow" src="icons/edit.svg" alt="Edit category" onclick=edit_category />
                <img class="category-icon animation-grow" src="icons/add.svg" alt="New password" onclick=new_password />
                <h2 class="category-title">{&category.title}</h2>
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
                <h3 class="password-title" onclick=edit_password.clone()>{&password.name}</h3>
                <p class="password-description">{&password.description}</p>
                <div class="password-icons">
                    <img class="password-icon animation-grow" src="icons/key.svg" alt="Copy password" onclick=copy_password />
                    <img class="password-icon animation-grow" src="icons/edit.svg" alt="Edit password" onclick=edit_password />
                </div>
            </div>
        }
    }

    fn render_new_password(&self) -> Html {
        let category_id = self.selected_category_id.clone();

        html! {
            <PasswordEditor
                new_mode=true
                added=self.link.callback(move |(name, desc, pass)| Messages::AddPassword(category_id.clone(), name, desc, pass))
                backed=self.link.callback(|_| Messages::ChangeView(Views::ListPasswords)) />
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
                backed=self.link.callback(|_| Messages::ChangeView(Views::ListPasswords))
                saved=saved
                removed=removed />
        }
    }

    fn render_new_category(&self) -> Html {
        html! {
            <PasswordCategoryEditor
                new_mode=true
                added=self.link.callback(|name| Messages::AddCategory(name))
                backed=self.link.callback(|_| Messages::ChangeView(Views::ListPasswords)) />
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
                backed=self.link.callback(|_| Messages::ChangeView(Views::ListPasswords))
                saved=self.link.callback(|(id, name)| Messages::EditCategory(id, name))
                removed=self.link.callback(|id| Messages::RemoveCategory(id)) />
        }
    }

    fn render_import_export(&self) -> Html {
        let encrypted_bytes = PasswordService::export_bytes();
        let href = format!("data:text/plain;charset=utf-8,{:?}", encrypted_bytes);

        let file_uploaded = self.link.callback(|event: ChangeData| {
            if let ChangeData::Files(files) = event {
                if let Some(file) = files.get(0) {
                    return Messages::ImportFile(file);
                }
            }
            Messages::ImportData(vec![])
        });

        html! {
            <div class="import-export animation-fade">
                <PageHeader 
                    title={"Import/Export"} 
                    description={"Move passwords between devices by export and import, both devices needs the same master password."}>
                </PageHeader>
                <div class="import-export-buttons">
                    <input ref=self.upload_ref.clone() type="file" onchange=file_uploaded />
                    <Button clicked=self.link.callback(|_| Messages::ImportClicked)>
                        {"Import"}
                    </Button>
                    <a class="main-button animation-grow" href=href download="passwords.cortex">
                        {"Export"}
                    </a>
                    <Button clicked=self.link.callback(|_| Messages::ChangeView(Views::ListPasswords))>
                        {"Back"}
                    </Button>
                </div>
            </div>
        }
    }

    fn render_error(&self, message: &'static str) -> Html {
        html! {
            <div class="animation-fade error-message">
                <img class="error-icon" src="icons/error.svg" alt="" />
                <h1 class="error-text">{message}</h1>
                <Button clicked=self.link.callback(|_| Messages::ChangeView(Views::ListPasswords))>
                    {"Back"}
                </Button>
            </div>
        }
    }

    fn render_decrypt_error(&self) -> Html {
        html! {
            <div class="animation-fade error-message">
                <img class="error-icon" src="icons/error.svg" alt="" />
                <h1 class="error-text">{"Invalid password specified"}</h1>
                <Button clicked=self.link.callback(|_| Messages::Logout)>
                    {"Reenter password"}
                </Button>
                <div class="password-editor-dangerzone">
                    <Button clicked=self.link.callback(|_| Messages::ResetData)>
                        {"Reset application data"}
                    </Button>
                </div>
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