use serde_json::{json, Value};
use std::error::Error;
use std::io::Write;

use crate::{generator::EgnitelyGenerator, authn::EgnitelyAuthN};

#[derive(Debug)]
pub enum EgnitelyResource {
    Function,
    Provider,
    AppTemplate,
}

pub struct EgnitelyHandler {}

impl EgnitelyHandler {
    pub fn new() -> Self {
        EgnitelyHandler {}
    }

    pub async fn login(&self) -> Result<Value, Box<dyn Error>> {
        println!("Login to your Egnitely Account");
        println!("");

        print!("Enter your email: ");
        std::io::stdout().flush().unwrap();
        let mut email = String::new();
        let _b1 = std::io::stdin().read_line(&mut email).unwrap();

        print!("Enter your password: ");
        std::io::stdout().flush().unwrap();
        let password = rpassword::read_password().unwrap();
        println!("");

        let authn = EgnitelyAuthN::new();
        email.truncate(email.clone().len() - 2);
        authn.login(email, password).await?;
        Ok(json!({}))
    }

    pub fn logout(&self) -> Result<Value, Box<dyn Error>> {
        println!("Logout of egnitely");
        Ok(json!({}))
    }

    pub fn trigger_function(&self, file: String) -> Result<Value, Box<dyn Error>> {
        println!("Trigger function with json: {}", file);
        Ok(json!({}))
    }

    pub fn push_function(&self) -> Result<Value, Box<dyn Error>> {
        println!("Push function to egnitely");
        Ok(json!({}))
    }

    pub fn apply_template(&self, file: String) -> Result<Value, Box<dyn Error>> {
        println!("Apply app template to provider: {}", file);
        Ok(json!({}))
    }

    pub fn create_function(&self, name: String) -> Result<Value, Box<dyn Error>> {
        let generator = EgnitelyGenerator::new(name, "rust".to_string());
        generator.generate_function()?;
        Ok(json!({}))
    }

    pub fn get_resource(&self, resource_name: EgnitelyResource) -> Result<Value, Box<dyn Error>> {
        println!("Create function named: {:?}", resource_name);
        Ok(json!({}))
    }

    pub fn delete_resource(
        &self,
        resource_name: EgnitelyResource,
        id: String,
    ) -> Result<Value, Box<dyn Error>> {
        println!("Create function named: {:?}", resource_name);
        Ok(json!({}))
    }
}
