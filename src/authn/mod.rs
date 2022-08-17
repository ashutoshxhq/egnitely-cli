use serde_json::{json, Value};
use std::{error::Error};
use colored::*;

pub struct EgnitelyAuthN {}

impl EgnitelyAuthN {
    pub fn new() -> Self {
        EgnitelyAuthN {}
    }

    pub async fn login(&self, email: String, password: String) -> Result<Value, Box<dyn Error>> {
        
        let data = json!({
            "email": email.clone(),
            "password": password.clone()
        });
        let req = reqwest::Client::new().post("https://dev.api.egnitely.com/auth/login")
        .json(&data);
        let res: Value = req.send().await?.json().await?;

        println!("{}", "Successfully Logged In".green().bold());
        Ok(json!({}))
    }

    pub fn logout() -> Result<(), Box<dyn Error>> {
        println!("Logout Triggered");
        Ok(())
    }
}
