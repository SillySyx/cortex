use yew::services::{StorageService, storage::Area};

pub struct LoginService {
}

impl LoginService {
    pub fn logout() {
        let mut storage = match StorageService::new(Area::Session) {
            Ok(store) => store,
            Err(_) => return,
        };

        storage.remove("key");

        let window = match web_sys::window() {
            Some(value) => value,
            None => return,
        };


        let location = window.location();

        let _ = location.reload();
    }

    pub fn store_key(value: String) {
        let mut storage = match StorageService::new(Area::Session) {
            Ok(store) => store,
            Err(_) => return,
        };

        storage.store("key", Ok(value));
    }

    pub fn key_present_in_storage() -> Option<String> {
        let storage = match StorageService::new(Area::Session) {
            Ok(store) => store,
            Err(_) => return None,
        };

        match storage.restore("key") {
            Ok(key) => Some(key),
            Err(_) => None,
        }
    }

    pub fn is_logged_in() -> bool {
        let key = match Self::key_present_in_storage() {
            Some(value) => value,
            None => return false,
        };

        !key.is_empty()
    }

    pub fn verify_key(key: &String) -> Option<bool> {
        let storage = match StorageService::new(Area::Local) {
            Ok(store) => store,
            Err(_) => return Some(false),
        };

        let data = match storage.restore("verification") {
            Ok(verification) => verification,
            Err(_) => return None,
        };

        let bytes: Vec<u8> = match serde_json::from_str(&data) {
            Ok(bytes) => bytes,
            Err(_) => return Some(false),
        };

        let key: Vec<u8> = match serde_json::from_str(&key) {
            Ok(key) => key,
            Err(_) => return Some(false),
        };

        let iv = match crypto::generate_iv_from_seed("silly goose") {
            Ok(iv) => iv,
            Err(_) => return Some(false),
        };

        let bytes = match crypto::decrypt(&bytes, &key, &iv) {
            Ok(data) => data,
            Err(_) => return Some(false),
        };

        Some(bytes == vec![1,2,3,4,5])
    }

    pub fn store_verify_key(key: &String) {
        let iv = match crypto::generate_iv_from_seed("silly goose") {
            Ok(iv) => iv,
            Err(_) => return,
        };

        let key: Vec<u8> = match serde_json::from_str(&key) {
            Ok(key) => key,
            Err(_) => return,
        };

        let bytes = vec![1,2,3,4,5];

        let data = match crypto::decrypt(&bytes, &key, &iv) {
            Ok(data) => data,
            Err(_) => return,
        };

        let data = match serde_json::to_string(&data) {
            Ok(data) => data,
            Err(_) => return,
        };

        let mut storage = match StorageService::new(Area::Local) {
            Ok(store) => store,
            Err(_) => return,
        };

        storage.store("verification", Ok(data));
    }
}