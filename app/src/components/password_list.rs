use yew::{
    prelude::*, 
    web_sys::HtmlInputElement,
};

use super::Button;

pub enum Messages {
    SearchButtonClicked,
    UpdateSearchText(String),
    SearchKeyPressed(KeyboardEvent),
}

pub struct PasswordList {
    link: ComponentLink<Self>,
    focus_ref: NodeRef,
    search_text: String,
}

impl Component for PasswordList {
    type Message = Messages;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            focus_ref: NodeRef::default(),
            search_text: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::SearchButtonClicked => {
                false
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
                match input.focus() {
                    Ok(_) => {},
                    Err(_) => {},
                };
            }
        }
    }

    fn view(&self) -> Html {
        let categories = load_categories(self.search_text.clone());

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

                    <Button active=false clicked=self.link.callback(|_| Messages::SearchButtonClicked)>
                        <img src="icons/add.svg" alt="" />
                    </Button>
                </header>

                { for categories.iter().map(render_category) }
            </div>
        }
    }
}

#[derive(Clone)]
struct Category {
    title: String,
    passwords: Vec<Password>,
}

#[derive(Clone)]
struct Password {
    name: String,
    desc: String,
}

fn load_categories(search: String) -> Vec<Category> {
    let mut data = vec![
        Category {
            title: String::from("Games"),
            passwords: vec![
                Password {
                    name: String::from("Black desert"),
                    desc: String::from(""),
                },
            ]
        },
        Category {
            title: String::from("Work"),
            passwords: vec![
                Password {
                    name: String::from("atea.com"),
                    desc: String::from("kristoffer.hagelstam@atea.com"),
                },
            ]
        },
    ];

    if search.is_empty() {
        return data;
    }

    data.iter_mut().map(|category| {
        let mut category = category.to_owned();

        category.passwords = category.passwords.iter().filter_map(|p| {
            if p.name.to_lowercase().contains(&search.to_lowercase()) {
                return Some(p.to_owned());
            }

            if p.desc.to_lowercase().contains(&search.to_lowercase()) {
                return Some(p.to_owned());
            }

            None
        }).collect();

        category
    }).collect()
}

fn render_category(category: &Category) -> Html {
    html! {
        <div>
            <h1 class="category-title">{&category.title}</h1>
            <div class="category">
                { for category.passwords.iter().map(render_password) }
            </div>
        </div>
    }
}

fn render_password(password: &Password) -> Html {
    html! {
        <div class="password">
            <h1 class="password-title">{&password.name}</h1>
            <p class="password-description">{&password.desc}</p>
            <img class="main-button password-icon" src="icons/add.svg" alt="" />
        </div>
    }
}