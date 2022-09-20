use std::error::Error;
use crate::{authn::EgnitelyAuthN, function::Function, generator::EgnitelyGenerator};

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

    pub async fn login(&self) -> Result<(), Box<dyn Error>> {
        let authn = EgnitelyAuthN::new();
        authn.login().await?;
        Ok(())
    }

    pub async fn logout(&self) -> Result<(), Box<dyn Error>> {
        let authn = EgnitelyAuthN::new();
        authn.logout().await?;
        Ok(())
    }

    pub fn trigger_function(&self, file: String) -> Result<(), Box<dyn Error>> {
        println!("Trigger function with json: {}", file);
        Ok(())
    }

    pub async fn push_function(&self) -> Result<(), Box<dyn Error>> {
        println!("Push function to egnitely");
        let function = Function::new();
        function.zip_function().await?;
        function.upload_function().await?;
        Ok(())
    }

    pub fn apply_template(&self, file: String) -> Result<(), Box<dyn Error>> {
        println!("Apply app template to provider: {}", file);
        Ok(())
    }

    pub fn create_function(&self, name: String) -> Result<(), Box<dyn Error>> {
        let generator = EgnitelyGenerator::new(name, "rust".to_string());
        generator.generate_function()?;
        Ok(())
    }

    pub fn get_resource(&self, resource_name: EgnitelyResource) -> Result<(), Box<dyn Error>> {
        println!("Create function named: {:?}", resource_name);
        Ok(())
    }

    pub fn delete_resource(
        &self,
        resource_name: EgnitelyResource,
        _id: String,
    ) -> Result<(), Box<dyn Error>> {
        println!("Create function named: {:?}", resource_name);
        Ok(())
    }
}
