use yew::services::{StorageService, storage::Area};
use serde::{Serialize, Deserialize};

use super::LoginService;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EncryptedPasswords {
    bytes: Vec<u8>,
}

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
        let encrypted_passwords = load_encrypted_passwords_from_storage()?;

        if encrypted_passwords.bytes.is_empty() {
            return Some(vec![]);
        }

        decrypt_passwords(&encrypted_passwords.bytes)
    }

    pub fn save_passwords(passwords: &Vec<Category>) {
        let encrypted_bytes = match encrypt_passwords(passwords) {
            Some(data) => data,
            None => return,
        };

        let encrypted_passwords = EncryptedPasswords { 
            bytes: encrypted_bytes,
        };

        save_encrypted_passwords_to_storage(&encrypted_passwords);
    }

    pub fn reset_data() {
        let mut storage = match StorageService::new(Area::Local) {
            Ok(store) => store,
            Err(_) => return,
        };

        storage.remove("passwords");
    }

    pub fn export_bytes() -> Vec<u8> {
        let data = match load_encrypted_passwords_from_storage() {
            Some(data) => data,
            None => return vec![],
        };

        data.bytes
    }

    pub fn import_bytes(bytes: Vec<u8>) -> Option<Vec<Category>> {
        decrypt_passwords(&bytes)
    }

    pub fn combine_passwords(categories: &Vec<Category>, new_categories: &Vec<Category>) -> Vec<Category> {
        let mut categories = categories.clone();

        let combine_passwords = |category: &mut Category, new_category: &Category| {
            for new_password in new_category.passwords.clone() {
                match category.passwords.iter_mut().find(|p| p.name == new_password.name) {
                    Some(password) => {
                        password.name = new_password.name;
                        password.description = new_password.description;
                        password.password = new_password.password;
                    },
                    None => {
                        category.passwords.push(new_password.clone());
                    },
                };
            }
        };

        for new_category in new_categories {
            match categories.iter_mut().find(|c| c.title == new_category.title) {
                Some(category) => {
                    combine_passwords(category, new_category);
                },
                None => {
                    categories.push(new_category.clone());
                },
            };
        }

        categories
    }
}

fn load_key_from_storage() -> Option<[u8; 32]> {
    let key = LoginService::key_present_in_storage()?;

    let data: [u8; 32] = match serde_json::from_str(&key) {
        Ok(data) => data,
        Err(_) => return None,
    };

    Some(data)
}

fn save_encrypted_passwords_to_storage(encrypted_passwords: &EncryptedPasswords) {
    let data = match serde_json::to_string(encrypted_passwords) {
        Ok(data) => data,
        Err(_) => return,
    };

    let mut store = match StorageService::new(Area::Local) {
        Ok(store) => store,
        Err(_) => return,
    };

    store.store("passwords", Ok(data));
}

fn load_encrypted_passwords_from_storage() -> Option<EncryptedPasswords> {
    let store = match StorageService::new(Area::Local) {
        Ok(store) => store,
        Err(_) => return None,
    };

    let data = match store.restore("passwords") {
        Ok(data) => data,
        Err(_) => String::new(),
    };
    
    if data.is_empty() {
        return Some(EncryptedPasswords { bytes: vec![] });
    }

    let encrypted_passwords: EncryptedPasswords = match serde_json::from_str(&data) {
        Ok(data) => data,
        Err(_) => return None,
    };

    Some(encrypted_passwords)
}

fn encrypt_passwords(passwords: &Vec<Category>) -> Option<Vec<u8>> {
    let bytes = match serde_json::to_vec(passwords) {
        Ok(bytes) => bytes,
        Err(_) => return None,
    };

    let key = load_key_from_storage()?;

    let iv = match crypto::generate_iv_from_seed("silly goose") {
        Ok(iv) => iv,
        Err(_) => return None,
    };

    match crypto::encrypt(&bytes, &key, &iv) {
        Ok(bytes) => Some(bytes),
        Err(_) => None,
    }
}

fn decrypt_passwords(bytes: &[u8]) -> Option<Vec<Category>> {
    let key = load_key_from_storage()?;

    let iv = match crypto::generate_iv_from_seed("silly goose") {
        Ok(iv) => iv,
        Err(_) => return None,
    };

    let bytes = match crypto::decrypt(bytes, &key, &iv) {
        Ok(bytes) => bytes,
        Err(_) => return None,
    };

    let data: Vec<Category> = match serde_json::from_slice(&bytes) {
        Ok(data) => data,
        Err(_) => return None,
    };

    Some(data)
}