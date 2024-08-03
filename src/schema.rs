use serde::{Deserialize, Serialize};
use uuid::Uuid; // Adicionado para o uso do tipo Uuid

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTaskSchema {
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateDocumentSchema {
    pub user_id: Uuid,
    pub doc_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTaskSchema {
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDocumentSchema {
    pub user_id: Option<Uuid>, // Tipo deve corresponder ao esquema do banco de dados
    pub doc_type: Option<String>,
}