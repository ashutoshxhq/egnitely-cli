use std::error::Error;

pub fn logout() -> Result<(), Box<dyn Error>> {
    println!("Logging Out of your account...");
    Ok(())
}