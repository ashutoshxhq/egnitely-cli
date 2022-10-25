use super::{service::ProjectService};
use std::error::Error;

pub async fn get_projects() -> Result<(), Box<dyn Error>> {
    let project = ProjectService::new();
    project.get_projects().await?;
    Ok(())
}
