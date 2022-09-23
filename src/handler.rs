use crate::extras::error::CLIError;
use crate::project::Project;
use crate::{authn::EgnitelyAuthN, function::Function, generator::EgnitelyGenerator};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use serde::Deserialize;
use std::fs;
use std::{error::Error, time::Duration};

#[derive(Debug, PartialEq)]
pub enum EgnitelyResource {
    Function,
    Project,
    Provider,
    AppTemplate,
}

#[derive(Deserialize, Clone)]
struct CargoTomlSchema {
    package: Package,
}

#[derive(Deserialize, Clone)]
struct Package {
    name: String,
    description: Option<String>,
    version: String,
}

pub struct EgnitelyHandler {}

impl EgnitelyHandler {
    pub fn new() -> Self {
        EgnitelyHandler {}
    }

    pub async fn login(&self) -> Result<(), Box<dyn Error>> {
        let authn = EgnitelyAuthN::new();
        authn.login().await?;
        Ok(())
    }

    pub async fn logout(&self) -> Result<(), Box<dyn Error>> {
        let authn = EgnitelyAuthN::new();
        authn.logout().await?;
        Ok(())
    }

    pub async fn push_function(&self, project: String) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string("Cargo.toml")?;
        let data: CargoTomlSchema = toml::from_str(&contents)?;

        println!(
            "{} {} to generate input and output schema (might take a minute or two)",
            "Compiling".blue().bold(),
            data.package.name.clone(),
        );

        if let Some(description) = data.package.description {
            let pb = ProgressBar::new_spinner();
            pb.enable_steady_tick(Duration::from_millis(120));
            pb.set_style(
                ProgressStyle::with_template("{msg:.green} {spinner:.green}")
                    .unwrap()
                    // For more spinners check out the cli-spinners project:
                    // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
                    .tick_strings(&[
                        "▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱",
                        "▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱",
                        "▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱",
                        "▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱",
                        "▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱",
                        "▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱",
                        "▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱",
                        "▰▰▰▰▰▰▰▰▱▱▱▱▱▱▱",
                        "▰▰▰▰▰▰▰▰▰▱▱▱▱▱▱",
                        "▰▰▰▰▰▰▰▰▰▰▱▱▱▱▱",
                        "▰▰▰▰▰▰▰▰▰▰▰▱▱▱▱",
                        "▰▰▰▰▰▰▰▰▰▰▰▰▱▱▱",
                        "▰▰▰▰▰▰▰▰▰▰▰▰▰▱▱",
                        "▰▰▰▰▰▰▰▰▰▰▰▰▰▰▱",
                        "▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰",
                        "▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱",
                    ]),
            );
            pb.set_message("Compiling");

            let generator = EgnitelyGenerator::new(data.package.name.clone(), "rust".to_string());
            generator.generate_application()?;

            println!(
                "{} {} {} to Egnitely",
                "Uploading".blue().bold(),
                data.package.name.clone(),
                data.package.version.clone()
            );
            pb.set_message("Uploading");

            let function = Function::new(data.package.name.clone(), data.package.version.clone(), description);
            function.zip_function().await?;
            function.upload_function(project).await?;

            pb.finish_and_clear();
            println!(
                "{} {} {} to Egnitely",
                "Pushed".green().bold(),
                data.package.name,
                data.package.version
            );

            Ok(())
        } else {
            return Err(CLIError::new(
                "DATA_ERROR".to_string(),
                format!(
                    "{}: No description found in Cargo.toml file",
                    "Error:".red().bold(),
                ),
            ));
        }
    }

    pub fn apply_template(&self, file: String) -> Result<(), Box<dyn Error>> {
        println!("Apply app template to provider: {}", file);
        Ok(())
    }

    pub fn create_function(&self, name: String) -> Result<(), Box<dyn Error>> {
        let generator = EgnitelyGenerator::new(name, "rust".to_string());
        generator.generate_function()?;
        Ok(())
    }

    pub async fn get_resource(
        &self,
        resource_name: EgnitelyResource,
    ) -> Result<(), Box<dyn Error>> {
        if resource_name == EgnitelyResource::Function {
            let function = Function::new("".to_string(), "".to_string(), "".to_string());
            function.get_functions().await?;
        } else if resource_name == EgnitelyResource::Project {
            let project = Project::new("".to_string());
            project.get_projects().await?;
        }
        Ok(())
    }
}
