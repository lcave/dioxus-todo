use uuid::Uuid;

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub content: String,
}
