use serde::{Deserialize, Serialize};


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
