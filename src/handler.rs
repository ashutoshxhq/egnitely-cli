use serde_json::{json, Value};
use std::error::Error;

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

    pub fn login(&self, email: String, password: String) -> Result<Value, Box<dyn Error>> {
        println!("Login to egnitely account");
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
        println!("Create function named: {}", name);
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
