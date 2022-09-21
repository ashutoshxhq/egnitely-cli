use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct FunctionResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub repository_url: Option<String>,
    pub branch: Option<String>,
    pub sub_directory: Option<String>,
    pub input_schema: Option<Value>,
    pub output_schema: Option<Value>,
    pub project_id: Uuid,
    pub team_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub team_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerErrorResponse {
    pub error: String,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EgnitelyResponse<T> {
    pub data: T,
    pub status: String,
}
