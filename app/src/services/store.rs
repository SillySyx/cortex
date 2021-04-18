use std::error::Error;
use yew::services::{StorageService, storage::Area};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncryptedData {
    pub bytes: Vec<u8>,
}

pub fn load_encrypted_data_from_storage(name: &str) -> Result<EncryptedData, Box<dyn Error>> {
    let store = StorageService::new(Area::Local)?;

    let data = match store.restore(name) {
        Ok(data) => data,
        Err(_) => String::new(),
    };
    
    if data.is_empty() {
        return Ok(EncryptedData { bytes: vec![] });
    }

    let encrypted_passwords: EncryptedData = serde_json::from_str(&data)?;

    Ok(encrypted_passwords)
}

pub fn save_encrypted_data_to_storage(name: &str, encrypted_passwords: &EncryptedData) -> Result<(), Box<dyn Error>> {
    let data = serde_json::to_string(encrypted_passwords)?;

    let mut store = StorageService::new(Area::Local)?;

    store.store(name, Ok(data));

    Ok(())
}

pub fn remove_encrypted_data_from_storage(name: &str) -> Result<(), Box<dyn Error>> {
    let mut store = StorageService::new(Area::Local)?;

    store.remove(name);

    Ok(())
}