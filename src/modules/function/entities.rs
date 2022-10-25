use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct FunctionResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub latest_version: Option<String>,
    pub repository_url: Option<String>,
    pub branch: Option<String>,
    pub sub_directory: Option<String>,
    pub input_schema: Option<Value>,
    pub output_schema: Option<Value>,
    pub project_id: Uuid,
    pub team_id: Uuid,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub team_id: Uuid,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Clone)]
pub struct CargoTomlSchema {
    pub package: Package,
}

#[derive(Deserialize, Clone)]
pub struct Package {
    pub name: String,
    pub description: Option<String>,
    pub version: String,
    pub project: Option<String>,
}

