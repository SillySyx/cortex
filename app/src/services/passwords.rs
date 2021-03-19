use yew::services::{StorageService, storage::Area};
use crypto::{encrypt, decrypt, generate_iv_from_seed};
use serde::{Serialize, Deserialize};

use super::LoginService;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    pub title: String,
    pub passwords: Vec<Password>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Password {
    pub name: String,
    pub description: String,
    pub password: String,
}

pub struct PasswordService {
}

impl PasswordService {
    pub fn load_passwords() -> Option<Vec<Category>> {
        let key = match load_key() {
            Some(key) => key,
            None => return None,
        };

        let iv = match generate_iv_from_seed("silly goose") {
            Ok(iv) => iv,
            Err(_) => return None,
        };

        let store = match StorageService::new(Area::Local) {
            Ok(store) => store,
            Err(_) => return None,
        };

        yew::services::ConsoleService::log("store");

        let data = match store.restore("passwords") {
            Ok(data) => data,
            Err(_) => String::new(),
        };
        
        yew::services::ConsoleService::log("restore");

        if data.is_empty() {
            return Some(vec![]);
        }

        let encrypted_bytes = data.as_bytes();

        yew::services::ConsoleService::log("as_bytes");

        let bytes = match decrypt(encrypted_bytes, &key, &iv) {
            Ok(bytes) => bytes,
            Err(_) => return None,
        };

        yew::services::ConsoleService::log("decrypt");

        let data: Vec<Category> = match serde_json::from_slice(&bytes) {
            Ok(data) => data,
            Err(_) => return None,
        };

        yew::services::ConsoleService::log("json");
        
        Some(data)
    }

    pub fn save_passwords(passwords: &Vec<Category>) {
        let key = match load_key() {
            Some(key) => key,
            None => return,
        };

        let iv = match generate_iv_from_seed("silly goose") {
            Ok(iv) => iv,
            Err(_) => return,
        };

        let json = match serde_json::value::to_value(passwords) {
            Ok(json) => json,
            Err(_) => return,
        };

        let data = json.to_string();
        let bytes = data.as_bytes();

        let encrypted_bytes = match encrypt(bytes, &key, &iv) {
            Ok(bytes) => bytes,
            Err(_) => return,
        };

        let data = format!("{:?}", encrypted_bytes);

        let mut store = match StorageService::new(Area::Local) {
            Ok(store) => store,
            Err(_) => return,
        };

        store.store("passwords", Ok(data));
    }
}

fn load_key() -> Option<[u8; 32]> {
    let key = LoginService::key_present_in_storage()?;

    let data: [u8; 32] = match serde_json::from_str(&key) {
        Ok(data) => data,
        Err(_) => return None,
    };

    Some(data)
}