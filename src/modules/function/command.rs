use crate::extras::error::CLIError;
use crate::modules::function::entities::CargoTomlSchema;
use crate::modules::function::generator::EgnitelyGenerator;
use crate::modules::function::service::FunctionService;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::error::Error;
use std::fs;
use std::time::Duration;

pub async fn push_function() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("Cargo.toml")?;
    let data: CargoTomlSchema = toml::from_str(&contents)?;

    println!(
        "{} {} to generate input and output schema (might take a minute or two)",
        "Compiling".blue().bold(),
        data.package.name.clone(),
    );

    if let Some(description) = data.package.description {
        if let Some(project) = data.package.project {
            let pb = ProgressBar::new_spinner();
            pb.enable_steady_tick(Duration::from_millis(120));
            pb.set_style(
                ProgressStyle::with_template("{msg:.green} {spinner:.green}")
                    .unwrap()
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

            let function = FunctionService::new(
                data.package.name.clone(),
                data.package.version.clone(),
                description,
            );
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
                    "{}: No project name found in Cargo.toml file",
                    "Error:".red().bold(),
                ),
            ));
        }
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

pub fn create_function(name: String) -> Result<(), Box<dyn Error>> {
    let generator = EgnitelyGenerator::new(name, "rust".to_string());
    generator.generate_function()?;
    Ok(())
}

pub async fn get_function() -> Result<(), Box<dyn Error>> {
    let function = FunctionService::new("".to_string(), "".to_string(), "".to_string());
    function.get_functions().await?;
    Ok(())
}
