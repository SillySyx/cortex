use yew::services::{StorageService,storage::Area};

pub struct LoginService {
}

impl LoginService {
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
}