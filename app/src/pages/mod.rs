mod home;
mod knowledgebase;
mod login;
mod main;
mod passwords;
mod todo;

pub use {
    home::HomePage,
    knowledgebase::KnowledgebasePage,
    login::LoginPage,
    main::{MainPage, Views as MainPageViews},
    passwords::PasswordsPage,
    todo::TodoPage,
};