use std::error::Error;

pub fn create_function(name: String) -> Result<(), Box<dyn Error>> {
    println!("Creating function with name: {}", name);
    Ok(())
}