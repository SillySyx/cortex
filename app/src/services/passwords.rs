#[derive(Clone)]
pub struct Category {
    pub title: String,
    pub passwords: Vec<Password>,
}

#[derive(Clone)]
pub struct Password {
    pub name: String,
    pub description: String,
    pub password: String,
}

pub struct PasswordService {
}

impl PasswordService {
    pub fn load_passwords() -> Vec<Category> {
        // read localStorage
        // decrypt bytes
        // parse bytes into Vec<Category>

        vec![]
    }
}