use serde_json::{json, Value};
use std::{error::Error};

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
        println!("Data: {:?}", data);
        let req = reqwest::Client::new().post("https://dev.api.egnitely.com/auth/login")
        .json(&data);

        let res: Value = req.send().await?.json().await?;

        println!("Login Response: {:#?}", res);
        Ok(json!({}))
    }

    pub fn logout() -> Result<(), Box<dyn Error>> {
        println!("Logout Triggered");
        Ok(())
    }
}
