use std::error::Error;

pub fn publish_function() -> Result<(), Box<dyn Error>> {
    println!("Publishing function to Egnitely");
    Ok(())
}