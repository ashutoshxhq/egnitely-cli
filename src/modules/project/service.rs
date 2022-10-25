use std::error::Error;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use prettytable::{Table, row};
use crate::{extras::response::EgnitelyResponse, config::get_server_url, modules::project::entities::ProjectResponse};

pub struct ProjectService {
    pub name: String,
}

impl ProjectService {
    pub fn new(name: String) -> Self {
        ProjectService { name }
    }

    pub async fn get_projects(&self) -> Result<(), Box<dyn Error>> {
        println!("");
        let client = reqwest::blocking::Client::new();
        if let Some(home_dir) = dirs::home_dir() {
            let db = PickleDb::load(
                home_dir.join(".egnitely").join("credentials"),
                PickleDbDumpPolicy::DumpUponRequest,
                SerializationMethod::Json,
            )?;
            let access_token = db.get::<String>("access_token");
            if let Some(access_token) = access_token {
                let get_functions_response = client
                    .get(format!("{}/projects", get_server_url()))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .send()?;

                if get_functions_response.status().is_success() {
                    let get_functions: EgnitelyResponse<Vec<ProjectResponse>> =
                        get_functions_response.json()?;

                    let mut table = Table::new();

                    // Add a row per time
                    table.add_row(row!["ID", "NAME", "CREATED AT"]);
                    for function in get_functions.data {
                        table.add_row(row![function.id, function.name, function.created_at]);
                    }
                    table.printstd();
                } else{
                    println!("No projects found, please check if you are logged in");
                }
            }
        }
        println!("");
        Ok(())
    }
}