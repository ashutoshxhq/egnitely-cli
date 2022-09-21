use std::fs::create_dir_all;
use std::io::prelude::*;
use std::{error::Error, fs, fs::File};
use convert_case::{Case, Casing};

pub struct RustGenerator {
    name: String,
}

impl RustGenerator {
    pub fn new(name: String) -> Self {
        RustGenerator { name }
    }

    pub fn generate_lib(&self) -> Result<(), Box<dyn Error>> {
        let lib_rs = "use egnitely_client::{Context, Error};\nuse serde_json::{json, Value};\n\npub async fn handler(mut _ctx: Context, _input: Value) -> Result<Value, Error> {\n\t// TODO: IMPLEMENT FUNCTION\n\t\n\tOk(json!({\n\t\t    \"message\": \"function executed successfully\"\n\t}))\n}\n\n#[cfg(test)]\nmod tests {\n\t use super::*;\n\n\t#[tokio::test]\n\tasync fn trigger_handler() {\n\n\t\tlet resp = handler(\n\t\t\tContext::new(\"test\".to_string(), \"0.1.0\".to_string(), json!({}), json!({})),\n\t\t\tjson!({}),\n\t\t)\n\t\t.await\n\t\t.unwrap();\n\t\t\n\t\tassert_eq!(\"function executed successfully\", resp[\"message\"])\n\t}\n}\n".to_string();

        let mut cargo_toml = "[package] \nname = \"".to_string();
        cargo_toml.push_str(&self.name);
        cargo_toml.push_str("\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n\n[dependencies]\nserde = { version = \"1.0.137\", features = [\"derive\"] }\nserde_json = \"1.0.81\"\negnitely_client = \"0.1.2\"\naws-config = \"0.47.0\"\ntokio = { version = \"1\", features = [\"full\"] }\nschemars ={ version = \"0.8.10\", features = [\"chrono\", \"uuid1\"]}");

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

    pub fn generate_application(&self) -> Result<(), Box<dyn Error>> {
        let main_rs = format!("use {}::handler;\nuse schemars::{{schema::RootSchema, schema_for, JsonSchema}};\nuse std::error::Error;\nuse std::fs::File;\nuse std::future::Future;\nuse std::io::prelude::*;\n\nfn main() {{\n    \tlet res = get_function_schema(handler);\n \tmatch res {{\n\t\tOk(_res) => {{\n \t\t\tprintln!(\"Created schema files\");\n\t\t}}\n\t\tErr(err) => {{\n  \t\t\tprintln!(\"Error: Something went wrong\");\n    \t\t\tprintln!(\"Error: {{}}\", err);\n\t\t}}\n \t}}\n}}\npub fn get_function_schema<I, J: JsonSchema, F, R, O: JsonSchema>(\n   \t_handler: F,\n) -> Result<Vec<RootSchema>, Box<dyn Error>>\nwhere\n  \tF: Fn(I, J) -> R,\n \tR: Future<Output = Result<O, Box<dyn std::error::Error + Send + Sync + 'static>>>,\n{{\n   \tlet input_schema = schema_for!(J);\n \tlet output_schema = schema_for!(O);\n\n \tlet mut file = File::create(\"input_schema.json\")?;\n\n  \tfile.write_all(serde_json::to_string_pretty(&input_schema)?.as_bytes())?;\n \tlet mut file = File::create(\"output_schema.json\")?;\n\n  \tfile.write_all(serde_json::to_string_pretty(&output_schema)?.as_bytes())?;\n\n  \tOk(vec![input_schema, output_schema])\n}}\n", self.name.to_case(Case::Snake));
        let cargo_toml = format!("[package] \nname = \"application\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n\n[dependencies]\nserde = {{ version = \"1.0\", features = [\"derive\"] }}\nserde_json = \"1.0\"\nschemars ={{ version = \"0.8.10\", features = [\"chrono\", \"uuid1\"]}}\n{} ={{ path = \"../../\"}}", self.name);
        let folder_name = "./temp/application/src".to_string();
        create_dir_all(folder_name.clone())?;

        let mut main_rs_filepath = folder_name.clone();
        main_rs_filepath.push_str("/main.rs");
        let mut main_rs_file = File::create(main_rs_filepath)?;
        main_rs_file.write_all(main_rs.as_bytes())?;

        let mut cargo_toml_filepath = "./temp/application".to_string();
        cargo_toml_filepath.push_str("/Cargo.toml");
        let mut cargo_toml_file = File::create(cargo_toml_filepath)?;
        cargo_toml_file.write_all(cargo_toml.as_bytes())?;

        Ok(())
    }
}
