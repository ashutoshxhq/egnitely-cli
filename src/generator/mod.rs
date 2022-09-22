use self::rust_generator::RustGenerator;
use colored::*;

use std::{error::Error, fs, process::Command};

mod rust_generator;

pub struct EgnitelyGenerator {
    name: String,
    language: String,
}

impl EgnitelyGenerator {
    pub fn new(name: String, language: String) -> Self {
        EgnitelyGenerator { name, language }
    }

    pub fn generate_function(&self) -> Result<(), Box<dyn Error>> {
        let rust_gen = RustGenerator::new(self.name.clone());
        rust_gen.generate_lib()?;
        println!(
            "{} egnitely function: name=`{}` language=`{}`",
            "Created".bold().green(),
            self.name.bold().blue(),
            self.language.bold().blue()
        );
        Ok(())
    }

    pub fn generate_application(&self) -> Result<(), Box<dyn Error>> {
        let rust_gen = RustGenerator::new(self.name.clone());
        rust_gen.generate_application().unwrap();

        let _output = Command::new("cargo")
            .current_dir("./temp/application")
            .args(["run"])
            .output()
            .expect("failed to generate input and output schema");

        fs::copy(
            "./temp/application/input_schema.json",
            "./input_schema.json",
        ).unwrap();
        fs::copy(
            "./temp/application/output_schema.json",
            "./output_schema.json",
        ).unwrap();

        fs::remove_dir_all("./temp/application").unwrap();

        Ok(())
    }
}
