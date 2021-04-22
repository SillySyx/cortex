use std::error::Error;

use rand::seq::SliceRandom;
use rand::thread_rng;

use serde::{Serialize, Deserialize};

use super::generate_id;
use super::store::{EncryptedData, load_encrypted_data_from_storage, remove_encrypted_data_from_storage, save_encrypted_data_to_storage};
use super::cryptography::CryptographyService;

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
        let categories = load_categories()?;

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
        let encrypted_bytes = load_encrypted_data_from_storage("passwords")?;

        Ok(encrypted_bytes.bytes)
    }

    pub fn import_categories(bytes: &[u8]) -> Result<(), Box<dyn Error>> {
        let mut imported_passwords = decrypt_passwords(bytes)?;
        sort_passwords(&mut imported_passwords);

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
        let _ = remove_encrypted_data_from_storage("passwords");
    }
}

fn load_categories() -> Result<Vec<Category>, Box<dyn Error>> {
    let encrypted_passwords = load_encrypted_data_from_storage("passwords")?;

    if encrypted_passwords.bytes.is_empty() {
        return Ok(vec![]);
    }

    let mut passwords  = decrypt_passwords(&encrypted_passwords.bytes)?;

    sort_passwords(&mut passwords);

    Ok(passwords)
}

fn save_categories(categories: &mut Vec<Category>) -> Result<(), Box<dyn Error>> {
    sort_passwords(categories);

    let encrypted_bytes = encrypt_passwords(categories)?;

    let encrypted_passwords = EncryptedData { 
        bytes: encrypted_bytes,
    };

    save_encrypted_data_to_storage("passwords", &encrypted_passwords)
}

fn encrypt_passwords(passwords: &Vec<Category>) -> Result<Vec<u8>, Box<dyn Error>> {
    let bytes = serde_json::to_vec(passwords)?;

    CryptographyService::encrypt_data(&bytes)
}

fn decrypt_passwords(bytes: &[u8]) -> Result<Vec<Category>, Box<dyn Error>> {
    let bytes = CryptographyService::decrypt_data(bytes)?;

    let data: Vec<Category> = serde_json::from_slice(&bytes)?;

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

pub fn generate_passphrase() -> String {
    let mut rng = thread_rng();

    let mut adjectives = list_adjectives();
    let mut nouns = list_nouns();

    adjectives.shuffle(&mut rng);
    nouns.shuffle(&mut rng);

    format!("{} {} {} {} {} {}", adjectives[0], nouns[0], adjectives[1], nouns[1], adjectives[2], nouns[2])
}

fn list_adjectives() -> Vec<String> {
    let source: &str = include_str!("adjectives");

    source
        .split(|c| c == '\r' || c == '\n')
        .map(|s| s.to_owned())
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>()
}

fn list_nouns() -> Vec<String> {
    let source: &str = include_str!("nouns");

    source
        .split(|c| c == '\r' || c == '\n')
        .map(|s| s.to_owned())
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>()
}