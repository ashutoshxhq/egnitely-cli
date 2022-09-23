pub mod entities;
use colored::Colorize;
use pickledb::PickleDb;
use pickledb::PickleDbDumpPolicy;
use pickledb::SerializationMethod;
use prettytable::row;
use prettytable::Table;
use semver::Version;
use serde_json::json;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::iter::Iterator;
use std::path::{Component, Path};
use walkdir::WalkDir;
use zip::write::FileOptions;

use crate::config::get_server_url;
use crate::extras::error::CLIError;
use crate::extras::response::EgnitelyResponse;
use crate::function::entities::FunctionResponse;

use self::entities::ProjectResponse;

pub struct Function {
    pub name: String,
    pub version: String,
    pub description: String,
}

impl Function {
    pub fn new(name: String, version: String, description: String) -> Self {
        Function {
            name,
            version,
            description,
        }
    }

    pub async fn get_functions(&self) -> Result<(), Box<dyn Error>> {
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
                    .get(format!("{}/functions", get_server_url()))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .send()?;

                if get_functions_response.status().is_success() {
                    let get_functions: EgnitelyResponse<Vec<FunctionResponse>> =
                        get_functions_response.json()?;

                    let mut table = Table::new();

                    // Add a row per time
                    table.add_row(row!["ID", "NAME", "CREATED AT"]);
                    for function in get_functions.data {
                        table.add_row(row![function.id, function.name, function.created_at]);
                    }
                    table.printstd();
                } else {
                    println!("No functions found, please check if you are logged in");
                }
            }
        }
        println!("");
        Ok(())
    }

    pub async fn zip_function(&self) -> Result<(), Box<dyn Error>> {
        let path = Path::new("./temp/function.zip");
        let file = File::create(&path)?;

        let walkdir = WalkDir::new("./");
        let it = walkdir.into_iter();

        let mut zip = zip::ZipWriter::new(file);
        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o755);

        let mut buffer = Vec::new();
        for entry in it.filter_map(|e| e.ok()) {
            let path = entry.path();
            let name = path.strip_prefix(Path::new("./"))?;

            if path.is_file() {
                if !(path.file_name() == Path::new("./temp/function.zip").file_name())
                    && !(name.components().nth(0) == Some(Component::Normal(OsStr::new("target"))))
                {
                    #[allow(deprecated)]
                    zip.start_file_from_path(name, options)?;
                    let mut f = File::open(path)?;

                    f.read_to_end(&mut buffer)?;
                    zip.write_all(&*buffer)?;
                    buffer.clear();
                }
            } else if !name.as_os_str().is_empty() {
                if !(name.components().nth(0) == Some(Component::Normal(OsStr::new("target")))) {
                    #[allow(deprecated)]
                    zip.add_directory_from_path(name, options)?;
                }
            }
        }

        zip.finish()?;
        Ok(())
    }

    pub async fn upload_function(&self, project: String) -> Result<(), Box<dyn Error>> {
        let input_schema = fs::read_to_string("./input_schema.json")?;
        let output_schema = fs::read_to_string("./output_schema.json")?;
        let client = reqwest::blocking::Client::new();
        let form = reqwest::blocking::multipart::Form::new().file("file", "./temp/function.zip")?;
        if let Some(home_dir) = dirs::home_dir() {
            let db = PickleDb::load(
                home_dir.join(".egnitely").join("credentials"),
                PickleDbDumpPolicy::DumpUponRequest,
                SerializationMethod::Json,
            )?;
            let access_token = db.get::<String>("access_token");
            if let Some(access_token) = access_token {
                let get_project_response = client
                    .get(format!("{}/projects?name={}", get_server_url(), project))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .send()?;
                if get_project_response.status().is_success() {
                    let get_project: EgnitelyResponse<ProjectResponse> =
                        get_project_response.json()?;

                    let get_function_response = client
                        .get(format!(
                            "{}/functions?name={}",
                            get_server_url(),
                            self.name.clone()
                        ))
                        .header("Authorization", format!("Bearer {}", access_token))
                        .send()?;

                    if get_function_response.status().is_success() {
                        let get_function: EgnitelyResponse<FunctionResponse> =
                            get_function_response.json()?;
                        if let Some(latest_version) = get_function.data.latest_version {
                            let current_version = Version::parse(&self.version.clone())?;
                            let latest_version = Version::parse(&latest_version)?;
                            if current_version.le(&latest_version) {
                                return Err(CLIError::new(
                                    "VERSION_ERROR".to_string(),
                                    format!("Function's version is less than or equal to last deployed version, please bump up the version before pushing it"),
                                ));
                            }
                        }

                        let _upload_response = client
                            .post(format!(
                                "{}/functions/{}/upload",
                                get_server_url(),
                                get_function.data.id
                            ))
                            .query(&[
                                ("version", self.version.clone()),
                                ("project_id", get_project.data.id.to_string()),
                            ])
                            .header("Authorization", format!("Bearer {}", access_token))
                            .multipart(form)
                            .send()?;

                        if !_upload_response.status().is_success() {
                            return Err(CLIError::new(
                                "UPLOAD_ERROR".to_string(),
                                format!(
                                    "Unable to upload a function, Error: {:?}",
                                    _upload_response.text()?
                                ),
                            ));
                        }

                        let _update_response = client
                            .patch(format!(
                                "{}/functions/{}",
                                get_server_url(),
                                get_function.data.id
                            ))
                            .header("Authorization", format!("Bearer {}", access_token))
                            .json(&json! {{
                                "name": self.name.clone(),
                                "description": self.description.clone(),
                                "latest_version": self.version.clone(),
                                "input_schema": input_schema,
                                "output_schema": output_schema
                            }})
                            .send()?;

                        if !_update_response.status().is_success() {
                            return Err(CLIError::new(
                                "UPDATE_FUNCTION_ERROR".to_string(),
                                format!(
                                    "Unable to upload a function, Error: {:?}",
                                    _update_response.text()?
                                ),
                            ));
                        }

                        fs::remove_file("./temp/function.zip")?;
                        fs::remove_dir_all("./temp")?;
                    } else {
                        let create_function_response = client
                            .post(format!("{}/functions", get_server_url()))
                            .header("Authorization", format!("Bearer {}", access_token))
                            .json(&json! {{
                                "name": self.name.clone(),
                                "description": self.description.clone(),
                                "latest_version": self.version.clone(),
                                "input_schema": input_schema,
                                "output_schema": output_schema,
                                "project_id": get_project.data.id,
                                "team_id": get_project.data.team_id,
                            }})
                            .send()?;

                        if create_function_response.status().is_success() {
                            let create_function: EgnitelyResponse<FunctionResponse> =
                                create_function_response.json()?;
                            let _upload_response = client
                                .post(format!(
                                    "{}/functions/{}/upload",
                                    get_server_url(),
                                    create_function.data.id
                                ))
                                .query(&[
                                    ("version", self.version.clone()),
                                    ("project_id", get_project.data.id.to_string()),
                                ])
                                .header("Authorization", format!("Bearer {}", access_token))
                                .multipart(form)
                                .send()?;

                            if !_upload_response.status().is_success() {
                                return Err(CLIError::new(
                                    "UPLOAD_ERROR".to_string(),
                                    format!(
                                        "Unable to upload a function, Error: {:?}",
                                        _upload_response.text()?
                                    ),
                                ));
                            }
                        } else {
                            return Err(CLIError::new(
                                "CREATE_FUNCTION_ERROR".to_string(),
                                format!(
                                    "Unable to create function `{}`, Status: {:?}, Error: {:?}",
                                    self.name,
                                    create_function_response.status(),
                                    create_function_response.text()?
                                ),
                            ));
                        }
                    }
                } else {
                    return Err(CLIError::new(
                        "BAD_PROJECT_NAME".to_string(),
                        format!(
                            "Unable to find project `{}`, Status: {:?}, Error: {:?}",
                            project,
                            get_project_response.status(),
                            get_project_response.text()?
                        ),
                    ));
                }
            } else {
                println!(
                    "{} Please login before pushing the function",
                    "Error:".red().bold()
                );
                return Err(CLIError::new(
                    "AUTH_ERROR".to_string(),
                    format!("Please login before pushing the function"),
                ));
            }
        }

        Ok(())
    }
}
