mod home;
mod knowledge;
mod login;
mod main;
mod passwords;
mod sync_data;

pub use {
    home::HomePage,
    knowledge::KnowledgePage,
    login::LoginPage,
    main::{MainPage, Views as MainPageViews},
    passwords::PasswordsPage,
    sync_data::SyncDataPage,
};