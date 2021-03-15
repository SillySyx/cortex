use yew::prelude::*;

use super::Button;

pub enum Messages {
    SearchButtonClicked,
}

pub struct PasswordList {
    link: ComponentLink<Self>,
}

impl Component for PasswordList {
    type Message = Messages;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::SearchButtonClicked => {},
        };
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let categories = load_categories();

        html! {
            <div class="password-list">
                <header class="search-box">
                    <input class="main-search-box" placeholder="Search for passwords" />
                    <Button active=false clicked=self.link.callback(|_| Messages::SearchButtonClicked)>
                        <img src="icons/add.svg" alt="" />
                    </Button>
                </header>

                { for categories.iter().map(render_category) }
            </div>
        }
    }
}

struct Category {
    title: String,
    passwords: Vec<Password>,
}

struct Password {
    name: String,
    desc: String,
}

fn load_categories() -> Vec<Category> {
    vec![
        Category {
            title: String::from("test"),
            passwords: vec![
                Password {
                    name: String::from("pass"),
                    desc: String::from("oooo"),
                }
            ]
        }
    ]
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