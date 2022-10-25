use super::{service::ProjectService};
use std::error::Error;

pub async fn get_projects() -> Result<(), Box<dyn Error>> {
    let project = ProjectService::new("".to_string());
    project.get_projects().await?;
    Ok(())
}
