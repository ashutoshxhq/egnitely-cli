mod entities;
use pickledb::PickleDb;
use pickledb::PickleDbDumpPolicy;
use pickledb::SerializationMethod;
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

use crate::function::entities::EgnitelyResponse;
use crate::function::entities::FunctionResponse;
use crate::function::entities::ServerErrorResponse;

use self::entities::ProjectResponse;

pub struct Function {
    pub name: String,
    pub version: String,
}

impl Function {
    pub fn new(name: String, version: String) -> Self {
        Function { name, version }
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
                    .get(format!("http://localhost:8000/projects?name={}", project))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .send()?;
                if get_project_response.status().is_success() {
                    let get_project: EgnitelyResponse<ProjectResponse> =
                        get_project_response.json()?;

                    let get_function_response = client
                        .get(format!(
                            "http://localhost:8000/functions?name={}",
                            self.name.clone()
                        ))
                        .header("Authorization", format!("Bearer {}", access_token))
                        .send()?;

                    if get_function_response.status().is_success() {
                        let get_function: EgnitelyResponse<FunctionResponse> =
                            get_function_response.json()?;

                        let _upload_response = client
                            .post(format!(
                                "http://localhost:8000/functions/{}/upload",
                                get_function.data.id
                            ))
                            .query(&[("version", self.version.clone())])
                            .header("Authorization", format!("Bearer {}", access_token))
                            .multipart(form)
                            .send()?;

                        let _update_response = client
                            .patch(format!(
                                "http://localhost:8000/functions/{}",
                                get_project.data.id
                            ))
                            .header("Authorization", format!("Bearer {}", access_token))
                            .json(&json! {{
                                "name": self.name.clone(),
                                "input_schema": input_schema,
                                "output_schema": output_schema
                            }})
                            .send()?;

                        fs::remove_file("./temp/function.zip")?;
                        fs::remove_dir_all("./temp")?;
                    } else {
                        let create_function_response = client
                            .post("http://localhost:8000/functions")
                            .header("Authorization", format!("Bearer {}", access_token))
                            .json(&json! {{
                                "name": self.name.clone(),
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
                                    "http://localhost:8000/functions/{}/upload",
                                    create_function.data.id
                                ))
                                .query(&[("version", self.version.clone())])
                                .header("Authorization", format!("Bearer {}", access_token))
                                .multipart(form)
                                .send()?;
                        } else {
                            println!(
                                "CREATE_FUNCTION: Something went wrong. Status: {:?}",
                                create_function_response.status()
                            );
                            let _error: ServerErrorResponse = create_function_response.json()?;
                        }
                    }
                } else {
                    println!(
                        "GET_PROJECT: Something went wrong. Status: {:?}",
                        get_project_response.status()
                    );
                    let _error: ServerErrorResponse = get_project_response.json()?;
                }
            } else {
                println!("Please login before pushing the function")
            }
        }

        Ok(())
    }
}
