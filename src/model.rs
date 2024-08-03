use chrono::{DateTime, Utc}; // Corrigido para usar DateTime em vez de NaiveDateTime
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TaskModel {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>, // Alinhado com o tipo DateTime<Utc>
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DocumentModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub doc_type: String,
    pub filename: String,
    pub created_at: Option<DateTime<Utc>>, // Ajuste para Option
}