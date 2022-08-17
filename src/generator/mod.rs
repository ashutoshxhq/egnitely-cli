use std::error::Error;
use colored::*;
use self::rust_generator::RustGenerator;

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
        println!("{} egnitely function: name=`{}` language=`{}`","Created".bold().green(), self.name.bold().blue(), self.language.bold().blue());
        Ok(())
    }
}