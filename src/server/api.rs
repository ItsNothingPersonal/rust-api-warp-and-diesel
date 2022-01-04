use serde::{Deserialize, Serialize};

use super::model::{BookStatus, CreateBookDTO};

#[derive(Debug, Deserialize, Clone)]
pub struct AddBook {
    pub title: String,
    pub author: String,
    pub status: BookStatus,
}

impl AddBook {
    pub fn to_dto(&self) -> CreateBookDTO {
        CreateBookDTO {
            title: self.title.clone(),
            author: self.author.clone(),
            status: self.status,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateStatus {
    pub status: BookStatus,
}

#[derive(Debug, Serialize, Clone)]
pub struct IdResponse {
    pub id: i64,
}

impl IdResponse {
    pub fn new(id: i64) -> IdResponse {
        IdResponse { id }
    }
}
