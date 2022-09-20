use pickledb::PickleDb;
use pickledb::PickleDbDumpPolicy;
use pickledb::SerializationMethod;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::iter::Iterator;
use std::path::{Component, Path};
use walkdir::WalkDir;
use zip::write::FileOptions;

pub struct Function {}

impl Function {
    pub fn new() -> Self {
        Function {}
    }

    pub async fn zip_function(&self) -> Result<(), Box<dyn Error>> {
        let path = Path::new("./function.zip");
        let file = File::create(&path)?;

        let walkdir = WalkDir::new("./");
        let it = walkdir.into_iter();

        let mut zip = zip::ZipWriter::new(file);
        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o755);

        let mut buffer = Vec::new();
        for entry in it.filter_map(|e| e.ok()) {
            let path = entry.path();
            let name = path.strip_prefix(Path::new("./"))?;

            if path.is_file() {
                if !(path.file_name() == Path::new("function.zip").file_name())
                    && !(name.components().nth(0) == Some(Component::Normal(OsStr::new("target"))))
                {
                    #[allow(deprecated)]
                    zip.start_file_from_path(name, options)?;
                    let mut f = File::open(path)?;

                    f.read_to_end(&mut buffer)?;
                    zip.write_all(&*buffer)?;
                    buffer.clear();
                }
            } else if !name.as_os_str().is_empty() {
                if !(name.components().nth(0) == Some(Component::Normal(OsStr::new("target")))) {
                    #[allow(deprecated)]
                    zip.add_directory_from_path(name, options)?;
                }
            }
        }

        zip.finish()?;
        Ok(())
    }

    pub async fn upload_function(&self) -> Result<(), Box<dyn Error>> {
        let client = reqwest::blocking::Client::new();
        let form = reqwest::blocking::multipart::Form::new().file("file", "./function.zip")?;
        if let Some(home_dir) = dirs::home_dir() {
            let db = PickleDb::load(
                home_dir.join(".egnitely").join("credentials.db"),
                PickleDbDumpPolicy::DumpUponRequest,
                SerializationMethod::Json,
            )?;
            let access_token = db.get::<String>("access_token");
            if let Some(access_token) = access_token {
                let _response = client
                    .post("http://localhost:8000/functions/0cc8f4d9-2ce9-4a7d-a3d4-17ea26b2626e/upload")
                    .header("Authorization", format!("Bearer {}", access_token))
                    .multipart(form)
                    .send()?;
            } else {
                println!("Please login before pushing the function")
            }
        }

        Ok(())
    }
}
