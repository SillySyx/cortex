mod home;
mod knowledge;
mod login;
mod main;
mod passwords;

pub use {
    home::HomePage,
    knowledge::KnowledgePage,
    login::LoginPage,
    main::{MainPage, Views as MainPageViews},
    passwords::PasswordsPage,
};