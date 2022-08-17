use std::error::Error;

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
        println!("Creating new function");
        println!("Function Name: {}", self.name);
        println!("Language: {}", self.language);
        
        let rust_gen = RustGenerator::new(self.name.clone());
        rust_gen.generate_lib()?;

        Ok(())
    }
}