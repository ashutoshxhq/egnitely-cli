use std::io::prelude::*;
use std::{error::Error, fs, fs::File};

pub struct RustGenerator {
    name: String,
}

impl RustGenerator {
    pub fn new(name: String) -> Self {
        RustGenerator { name }
    }

    pub fn generate_lib(&self) -> Result<(), Box<dyn Error>> {
        let lib_rs = "use egnitely_client::{Json, Error};\nuse serde_json::{json, Value};\n\npub async fn handler(Json(data): Json<Value>) -> Result<Value, Error> {\n\t// TODO: IMPLEMENT FUNCTION\n\t\n\tOk(json!({\n\t\t    \"message\": \"function executed successfully\"\n\t}))\n}\n\n#[cfg(test)]\nmod tests {\n\t use super::*;\n\n\t#[tokio::test]\n\tasync fn trigger_handler() {\n\n\t\tlet resp = handler(\n\t\t\tContext::new(\"test\".to_string(), \"0.1.0\".to_string(), json!({}), json!({})),\n\t\t\tjson!({}),\n\t\t)\n\t\t.await\n\t\t.unwrap();\n\t\t\n\t\tassert_eq!(\"function executed successfully\", resp[\"message\"])\n\t}\n}\n".to_string();

        let mut cargo_toml = "[package] \nname = \"".to_string();
        cargo_toml.push_str(&self.name);
        cargo_toml.push_str("\"\nversion = \"0.1.0\"\ndescription = \"Write your function descrition here\"\nproject = \"project-name\"\nedition = \"2021\"\n\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n\n[dependencies]\nserde = { version = \"1.0.137\", features = [\"derive\"] }\nserde_json = \"1.0.81\"\negnitely_client = \"0.1.2\"\naws-config = \"0.47.0\"\ntokio = { version = \"1\", features = [\"full\"] }\nschemars ={ version = \"0.8.10\", features = [\"chrono\", \"uuid1\"]}");

        let gitignore = "/target\n/Cargo.lock";

        let mut folder_name = self.name.clone();
        folder_name.push_str("/src");

        fs::create_dir(self.name.clone())?;
        fs::create_dir(folder_name.clone())?;

        let mut lib_rs_filepath = folder_name.clone();
        lib_rs_filepath.push_str("/lib.rs");
        let mut lib_rs_file = File::create(lib_rs_filepath)?;
        lib_rs_file.write_all(lib_rs.as_bytes())?;

        let mut cargo_toml_filepath = self.name.clone();
        cargo_toml_filepath.push_str("/Cargo.toml");
        let mut cargo_toml_file = File::create(cargo_toml_filepath)?;
        cargo_toml_file.write_all(cargo_toml.as_bytes())?;

        let mut gitignore_filepath = self.name.clone();
        gitignore_filepath.push_str("/.gitignore");
        let mut gitignore_file = File::create(gitignore_filepath)?;
        gitignore_file.write_all(gitignore.as_bytes())?;

        Ok(())
    }
}
