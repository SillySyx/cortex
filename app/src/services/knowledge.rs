use std::error::Error;

use serde::{Serialize, Deserialize};

use super::generate_id;
use super::store::{EncryptedData, load_encrypted_data_from_storage, remove_encrypted_data_from_storage, save_encrypted_data_to_storage};
use super::cryptography::CryptographyService;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Knowledge {
    pub id: String,
    pub category: String,
    pub name: String,
    pub description: String,
}

impl Knowledge {
    pub fn default() -> Self {
        Self {
            description: "Useful if you have a silly brain.".into(),
            id: "root".into(),
            name: "Knowledge".into(),
            category: "".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum KnowledgeDataType {
    Markdown,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct KnowledgeData {
    pub data_type: KnowledgeDataType,
    pub data: Vec<u8>,
}

impl KnowledgeData {
    pub fn default() -> Self {
        Self {
            data: vec![],
            data_type: KnowledgeDataType::Markdown,
        }
    }
}

pub struct KnowledgeService {
}

impl KnowledgeService {
    pub fn list_knowledge() -> Result<Vec<Knowledge>, Box<dyn Error>> {
        let encrypted_data = load_encrypted_data_from_storage("knowledge")?;

        if encrypted_data.bytes.is_empty() {
            return Ok(vec![]);
        }

        let bytes = CryptographyService::decrypt_data(&encrypted_data.bytes)?;

        let knowledge: Vec<Knowledge> = serde_json::from_slice(&bytes)?;

        // sort knowledge

        Ok(knowledge)
    }
    
    pub fn load_knowledge(id: &str) -> Result<Knowledge, Box<dyn Error>> {
        if id == "root" {
            return Ok(Knowledge::default());
        }

        let list = Self::list_knowledge()?;
        
        match list.iter().find(|knowledge| knowledge.id == id) {
            Some(value) => Ok(value.to_owned()),
            None => Err(Box::from("Knowledge not found")),
        }
    }

    pub fn create_knowledge(category: String, name: String, description: String) -> Result<Knowledge, Box<dyn Error>> {
        let mut list = Self::list_knowledge()?;

        let knowledge = Knowledge {
            id: generate_id(),
            category,
            name,
            description,
        };

        list.push(knowledge.clone());

        save_knowledge(&list)?;

        Ok(knowledge)
    }

    pub fn update_knowledge(id: &str, category: Option<String>, name: Option<String>, description: Option<String>) -> Result<(), Box<dyn Error>> {
        let mut list = Self::list_knowledge()?;

        let knowledge = match list.iter_mut().find(|knowledge| knowledge.id == id) {
            Some(value) => value,
            None => return Err(Box::from("Failed to find knowledge")),
        };

        if let Some(category) = category {
            knowledge.category = category;
        }

        if let Some(name) = name {
            knowledge.name = name;
        }

        if let Some(description) = description {
            knowledge.description = description;
        }

        save_knowledge(&list)?;

        Ok(())
    }

    pub fn remove_knowledge(id: &str) -> Result<(), Box<dyn Error>> {
        let mut list = Self::list_knowledge()?;

        let index = match list.iter().position(|knowledge| knowledge.id == id) {
            Some(value) => value,
            None => return Err(Box::from("Failed to find knowledge")),
        };

        list.remove(index);

        save_knowledge(&list)?;

        Ok(())
    }

    pub fn load_knowledge_data(id: &str) -> Result<KnowledgeData, Box<dyn Error>> {
        if id == "root" {
            return Ok(KnowledgeData::default());
        }

        let id = format_knowledge_data_id(id);

        let encrypted_data = load_encrypted_data_from_storage(&id)?;

        if encrypted_data.bytes.is_empty() {
            return Err(Box::from("Failed to load knowledge data"));
        }

        let bytes = CryptographyService::decrypt_data(&encrypted_data.bytes)?;

        let knowledge: KnowledgeData = serde_json::from_slice(&bytes)?;

        Ok(knowledge)
    }

    pub fn create_knowledge_data(id: &str, data_type: KnowledgeDataType, data: Vec<u8>) -> Result<(), Box<dyn Error>> {
        let knowledge = KnowledgeData {
            data,
            data_type,
        };

        save_knowledge_data(id, &knowledge)
    }

    pub fn update_knowledge_data(id: &str, data_type: Option<KnowledgeDataType>, data: Option<Vec<u8>>) -> Result<(), Box<dyn Error>> {
        let mut knowledge = Self::load_knowledge_data(id)?;

        if let Some(data_type) = data_type {
            knowledge.data_type = data_type;
        }

        if let Some(data) = data {
            knowledge.data = data;
        }

        save_knowledge_data(id, &knowledge)
    }

    pub fn remove_knowledge_data(id: &str) -> Result<(), Box<dyn Error>> {
        let id = format_knowledge_data_id(id);
        remove_encrypted_data_from_storage(&id)
    }

    pub fn reset_data() {
        let _ = remove_encrypted_data_from_storage("knowledge");
    }
}

pub fn parse_markdown_to_html(markdown: &str) -> String {
    let mut options = pulldown_cmark::Options::empty();
    options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
    options.insert(pulldown_cmark::Options::ENABLE_TABLES);
    options.insert(pulldown_cmark::Options::ENABLE_TASKLISTS);
    options.insert(pulldown_cmark::Options::ENABLE_FOOTNOTES);
    options.insert(pulldown_cmark::Options::ENABLE_SMART_PUNCTUATION);

    let parser = pulldown_cmark::Parser::new_ext(&markdown, options);

    let mut html: String = String::with_capacity(markdown.len() * 3 / 2);

    pulldown_cmark::html::push_html(&mut html, parser);

    html
}

fn save_knowledge(knowledge: &Vec<Knowledge>) -> Result<(), Box<dyn Error>> {
    let bytes = serde_json::to_vec(knowledge)?;
    let bytes = CryptographyService::encrypt_data(&bytes)?;
    let data = EncryptedData { bytes };

    save_encrypted_data_to_storage("knowledge", &data)
}

fn save_knowledge_data(id: &str, knowledge: &KnowledgeData) -> Result<(), Box<dyn Error>> {
    let bytes = serde_json::to_vec(knowledge)?;
    let bytes = CryptographyService::encrypt_data(&bytes)?;
    let data = EncryptedData { bytes };

    let id = format_knowledge_data_id(id);

    save_encrypted_data_to_storage(&id, &data)
}

fn format_knowledge_data_id(id: &str) -> String {
    format!("knowledge_data_{}", id)
}