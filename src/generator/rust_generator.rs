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
        let lib_rs = "use egnitely_client::Context;\nuse std::error::Error;\nuse serde_json::{json, Value};\n\npub async fn handler(_ctx: Context, input:Value) -> Result<Value, Box<dyn Error>> {\n\n\tprintln!(\"Input Data: {:?}\", input);\n\t//TODO: Implement your function\n\n\tOk(json!({\n\t\t\"message\": \"function executed successfully\"\n\t}))\n}\n".to_string();

        let mut cargo_toml = "[package] \nname = \"".to_string();
        cargo_toml.push_str(&self.name);
        cargo_toml.push_str("\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n\n[dependencies]\nserde = { version = \"1.0.137\", features = [\"derive\"] }\nserde_json = \"1.0.81\"\negnitely-client = { git=\"https://github.com/egnitely/egnitely-client-lib.git\", version=\"0.1.0\" }\n");

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
