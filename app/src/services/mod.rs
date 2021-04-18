mod login;
mod passwords;
mod clipboard;
mod knowledge;
pub mod store;
pub mod cryptography;

pub fn generate_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub use {
    login::LoginService,
    passwords::{PasswordService, Password, Category},
    clipboard::ClipboardService,
    knowledge::{Knowledge, KnowledgeData, KnowledgeDataType, KnowledgeService, parse_markdown_to_html},
};