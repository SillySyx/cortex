use std::error::Error;

use super::LoginService;

pub struct CryptographyService {
}

impl CryptographyService {
    pub fn encrypt_data(bytes: &Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
        let key = load_key_from_storage()?;
        let iv = crypto::generate_iv_from_seed("silly goose")?;
    
        crypto::encrypt(&bytes, &key, &iv)
    }
    
    pub fn decrypt_data(bytes: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let key = load_key_from_storage()?;
    
        let iv = crypto::generate_iv_from_seed("silly goose")?;
    
        crypto::decrypt(bytes, &key, &iv)
    }
}

fn load_key_from_storage() -> Result<[u8; 32], Box<dyn Error>> {
    let key = match LoginService::key_present_in_storage() {
        Some(value) => value,
        None => return Err(Box::from("Failed to load key")),
    };

    let data: [u8; 32] = serde_json::from_str(&key)?;

    Ok(data)
}