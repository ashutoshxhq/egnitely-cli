use std::error::Error;

pub fn login() -> Result<(), Box<dyn Error>> {
    println!("Logging In to your account...");
    Ok(())
}