use std::error::Error;

use super::generate_id;

#[derive(Debug, Clone)]
pub struct Knowledge {
    pub id: String,
    pub path: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub enum KnowledgeDataType {
    Markdown,
}

#[derive(Debug, Clone)]
pub struct KnowledgeData {
    pub data_type: KnowledgeDataType,
    pub data: Vec<u8>,
}

pub struct KnowledgeService {
}

impl KnowledgeService {
    pub fn list_knowledge() -> Result<Vec<Knowledge>, Box<dyn Error>> {
        Ok(vec![
            Knowledge {
                id: generate_id(),
                path: "/".into(),
                name: "Recepies".into(),
                description: "".into(),
            }
        ])
    }
    
    pub fn load_knowledge(id: &str) -> Result<Knowledge, Box<dyn Error>> {
        if id == "root" {
            return Ok(Knowledge {
                id: "root".into(),
                path: "/".into(),
                name: "Knowledge".into(),
                description: "Useful if you have a silly brain".into(),
            });
        }

        Ok(Knowledge {
            id: generate_id(),
            path: "/".into(),
            name: "Recepies".into(),
            description: "".into(),
        })
    }

    pub fn create_knowledge(path: String, name: String, description: String) -> Result<Knowledge, Box<dyn Error>> {
        Ok(Knowledge {
            id: generate_id(),
            path,
            name,
            description,
        })
    }

    pub fn update_knowledge(id: &str, path: Option<String>, name: Option<String>, description: Option<String>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn remove_knowledge(id: &str) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn load_knowledge_data(id: &str) -> Result<KnowledgeData, Box<dyn Error>> {
        if id == "root" {
            return Ok(KnowledgeData {
                data_type: KnowledgeDataType::Markdown,
                data: b"<svg class=\"knowledge-icon\"><use href=\"icons/list_knowledge.svg#src\"></svg>".to_vec(),
            });
        }

        Ok(KnowledgeData {
            data_type: KnowledgeDataType::Markdown,
            data: vec![],
        })
    }

    pub fn create_knowledge_data(id: &str, data_type: KnowledgeDataType, data: Vec<u8>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn update_knowledge_data(id: &str, data_type: Option<KnowledgeDataType>, data: Option<Vec<u8>>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn remove_knowledge_data(id: &str) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

pub fn parse_markdown_to_html(markdown: &str) -> String {
    let mut options = pulldown_cmark::Options::empty();
    options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);

    let parser = pulldown_cmark::Parser::new_ext(&markdown, options);

    let mut html: String = String::with_capacity(markdown.len() * 3 / 2);

    pulldown_cmark::html::push_html(&mut html, parser);

    html
}