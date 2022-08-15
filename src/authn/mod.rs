use std::error::Error;
use serde_json::{Value, json};

pub struct EgnitelyAuthN {}

impl EgnitelyAuthN {
    pub fn login(email: String, password: String) -> Result<Value, Box<dyn Error>> {
        println!("Login Triggered with Email: {}, Password: {}", email, password);
        Ok(json!({}))
    }

    pub fn logout() -> Result<(), Box<dyn Error>> {
        println!("Logout Triggered");
        Ok(())
    }
}