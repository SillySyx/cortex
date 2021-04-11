use std::error::Error;

use yew::services::{StorageService, storage::Area};
use serde::{Serialize, Deserialize};

use super::{LoginService, generate_id};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EncryptedPasswords {
    bytes: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Category {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub passwords: Vec<Password>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Password {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub password: String,
}

pub struct PasswordService {
}

impl PasswordService {
    pub fn list_categories() -> Result<Vec<Category>, Box<dyn Error>> {
        let mut categories = load_categories()?;

        categories.sort_by_key(|category| category.title.to_lowercase());

        Ok(categories)
    }

    pub fn load_category(id: &str) -> Result<Category, Box<dyn Error>> {
        let categories = load_categories()?;

        match categories.iter().find(|category| category.id == id) {
            Some(category) => Ok(category.clone()),
            None => Err(Box::from("Failed to find category")),
        }
    }

    pub fn create_category(title: String) -> Result<Category, Box<dyn Error>> {
        let mut categories = load_categories()?;

        let category = Category {
            id: generate_id(),
            title,
            passwords: vec![],
        };

        categories.push(category.clone());

        save_categories(&mut categories)?;

        Ok(category)
    }

    pub fn update_category(id: &str, title: Option<String>) -> Result<(), Box<dyn Error>> {
        let mut categories = load_categories()?;

        let category = match categories.iter_mut().find(|category| category.id == id) {
            Some(category) => category,
            None => return Err(Box::from("Failed to find category")),
        };

        if let Some(title) = title {
            category.title = title;
        }

        save_categories(&mut categories)
    }

    pub fn remove_category(id: &str) -> Result<(), Box<dyn Error>> {
        let mut categories = load_categories()?;

        let position = match categories.iter().position(|category| category.id == id) {
            Some(value) => value,
            None => return Err(Box::from("Failed to find category")),
        };

        categories.remove(position);

        save_categories(&mut categories)
    }

    pub fn export_categories() -> Result<Vec<u8>, Box<dyn Error>> {
        let encrypted_bytes = load_encrypted_passwords_from_storage()?;

        Ok(encrypted_bytes.bytes)
    }

    pub fn import_categories(bytes: &[u8]) -> Result<(), Box<dyn Error>> {
        let imported_passwords = decrypt_passwords(bytes)?;
        let stored_passwords = load_categories()?;

        let mut merged_passwords = combine_passwords(&stored_passwords, &imported_passwords);

        save_categories(&mut merged_passwords)
    }

    pub fn load_password(password_id: &str) -> Result<Password, Box<dyn Error>> {
        let categories = load_categories()?;

        for category in categories {
            if let Some(password) = category.passwords.iter().find(|password| password.id == password_id) {
                return Ok(password.clone());
            }
        }

        Err(Box::from("Failed to find password"))
    }

    pub fn create_password(category_id: &str, name: String, description: String, password: String) -> Result<Password, Box<dyn Error>> {
        let mut categories = load_categories()?;

        let category = match categories.iter_mut().find(|category| category.id == category_id) {
            Some(value) => value,
            None => return Err(Box::from("Failed to find category")),
        };

        let password = Password {
            id: generate_id(),
            name: name.clone(),
            description: description.clone(),
            password: password.clone(),
        };

        category.passwords.push(password.clone());

        save_categories(&mut categories)?;

        Ok(password)
    }

    pub fn update_password(password_id: &str, name: Option<String>, description: Option<String>, password: Option<String>) -> Result<(), Box<dyn Error>> {
        let mut categories = load_categories()?;

        for category in categories.iter_mut() {
            if let Some(entry) = category.passwords.iter_mut().find(|password| password.id == password_id) {
                if let Some(name) = name {
                    entry.name = name;
                }

                if let Some(description) = description {
                    entry.description = description;
                }

                if let Some(password) = password {
                    entry.password = password;
                }

                save_categories(&mut categories)?;

                return Ok(());
            }
        }

        Err(Box::from("Failed to find password"))
    }

    pub fn remove_password(password_id: &str) -> Result<(), Box<dyn Error>> {
        let mut categories = load_categories()?;

        for category in categories.iter_mut() {
            if let Some(position) = category.passwords.iter().position(|password| password.id == password_id) {
                category.passwords.remove(position);

                save_categories(&mut categories)?;

                return Ok(());
            }
        }

        Err(Box::from("Failed to find password"))
    }

    pub fn reset_data() {
        let mut storage = match StorageService::new(Area::Local) {
            Ok(store) => store,
            Err(_) => return,
        };

        storage.remove("passwords");
    }
}

fn load_encrypted_passwords_from_storage() -> Result<EncryptedPasswords, Box<dyn Error>> {
    let store = StorageService::new(Area::Local)?;

    let data = match store.restore("passwords") {
        Ok(data) => data,
        Err(_) => String::new(),
    };
    
    if data.is_empty() {
        return Ok(EncryptedPasswords { bytes: vec![] });
    }

    let encrypted_passwords: EncryptedPasswords = serde_json::from_str(&data)?;

    Ok(encrypted_passwords)
}

fn save_encrypted_passwords_to_storage(encrypted_passwords: &EncryptedPasswords) -> Result<(), Box<dyn Error>> {
    let data = serde_json::to_string(encrypted_passwords)?;

    let mut store = StorageService::new(Area::Local)?;

    store.store("passwords", Ok(data));

    Ok(())
}

fn load_categories() -> Result<Vec<Category>, Box<dyn Error>> {
    let encrypted_passwords = load_encrypted_passwords_from_storage()?;

    if encrypted_passwords.bytes.is_empty() {
        return Ok(vec![]);
    }

    decrypt_passwords(&encrypted_passwords.bytes)
}

fn save_categories(categories: &mut Vec<Category>) -> Result<(), Box<dyn Error>> {
    sort_passwords(categories);

    let encrypted_bytes = encrypt_passwords(categories)?;

    let encrypted_passwords = EncryptedPasswords { 
        bytes: encrypted_bytes,
    };

    save_encrypted_passwords_to_storage(&encrypted_passwords)
}

fn load_key_from_storage() -> Result<[u8; 32], Box<dyn Error>> {
    let key = match LoginService::key_present_in_storage() {
        Some(value) => value,
        None => return Err(Box::from("Failed to load key")),
    };

    let data: [u8; 32] = serde_json::from_str(&key)?;

    Ok(data)
}

fn encrypt_passwords(passwords: &Vec<Category>) -> Result<Vec<u8>, Box<dyn Error>> {
    let bytes = serde_json::to_vec(passwords)?;

    let key = load_key_from_storage()?;
    let iv = crypto::generate_iv_from_seed("silly goose")?;

    crypto::encrypt(&bytes, &key, &iv)
}

fn decrypt_passwords(bytes: &[u8]) -> Result<Vec<Category>, Box<dyn Error>> {
    let key = load_key_from_storage()?;

    let iv = crypto::generate_iv_from_seed("silly goose")?;

    let bytes = crypto::decrypt(bytes, &key, &iv)?;

    let mut data: Vec<Category> = serde_json::from_slice(&bytes)?;

    for category in data.iter_mut() {
        if category.id.is_empty() {
            category.id = generate_id();
        }

        for password in category.passwords.iter_mut() {
            if password.id.is_empty() {
                password.id = generate_id();
            }
        }
    }

    sort_passwords(&mut data);

    Ok(data)
}

fn combine_passwords(categories: &Vec<Category>, new_categories: &Vec<Category>) -> Vec<Category> {
    let mut categories = categories.clone();

    let combine_passwords = |category: &mut Category, new_category: &Category| {
        for new_password in new_category.passwords.clone() {
            match category.passwords.iter_mut().find(|p| p.id == new_password.id) {
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
        match categories.iter_mut().find(|c| c.id == new_category.id) {
            Some(category) => {
                category.title = new_category.title.clone();
                combine_passwords(category, new_category);
            },
            None => {
                categories.push(new_category.clone());
            },
        };
    }

    categories
}

fn sort_passwords(categories: &mut Vec<Category>) {
    categories.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
    
    for category in categories {
        category.passwords.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    }
}