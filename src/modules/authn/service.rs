use std::{error::Error, collections::HashMap, path::Path, fs, thread, time};
use colored::*;
use pickledb::{PickleDbDumpPolicy, SerializationMethod, PickleDb};
use serde_json::{Value, json};

use crate::modules::authn::entities::{DeviceCodeResponse, TokenErrorResponse, TokenSuccessResponse};


pub struct AuthNService {}

impl AuthNService {
    pub fn new() -> Self {
        AuthNService {}
    }

    pub async fn login(&self) -> Result<Value, Box<dyn Error>> {
        let mut params = HashMap::new();
        params.insert("client_id", "wYIqKC5ffQnzy9lpGGt4Lia98NH5ea4m");
        params.insert("scope", "openid offline_access profile");
        params.insert("audience", "https://dev.api.egnitely.com");

        let client = reqwest::Client::new();

        let device_code_req = client
            .post("https://dev-qgdysq-r.us.auth0.com/oauth/device/code")
            .form(&params);
        let device_code_res = device_code_req
            .send()
            .await?
            .json::<DeviceCodeResponse>()
            .await?;
        println!("");
        println!("");
        println!(
            "Please open this url in your browser to login: {}",
            device_code_res.verification_uri_complete.blue().bold()
        );
        println!(
            "Please enter this code if prompted: {}",
            device_code_res.user_code.blue().bold()
        );
        println!("");
        match open::that(device_code_res.verification_uri_complete) {
            Ok(()) => {}
            Err(_err) => {
                println!("Unable to open browser, please manualy open the link.")
            }
        }

        let mut token_params = HashMap::new();
        token_params.insert("client_id", "wYIqKC5ffQnzy9lpGGt4Lia98NH5ea4m");
        token_params.insert("grant_type", "urn:ietf:params:oauth:grant-type:device_code");
        token_params.insert("device_code", &device_code_res.device_code);

        loop {
            let token_req = client
                .post("https://dev-qgdysq-r.us.auth0.com/oauth/token")
                .form(&token_params);
            let token_res = token_req.send().await?.json::<Value>().await?;
            let error = serde_json::from_value::<TokenErrorResponse>(token_res.clone());
            match error {
                Ok(err) => {
                    if err.error == "authorization_pending".to_string() {
                        println!("CLI will wait while you login to Egnitely in your browser, you can close it by pressing CTRL+C");
                    }
                }
                Err(_err) => {
                    let token_data = serde_json::from_value::<TokenSuccessResponse>(token_res);
                    match token_data {
                        Ok(_token_data) => {
                            if let Some(home_dir) = dirs::home_dir() {
                                if !(Path::new(&home_dir.join(".egnitely")).is_dir()) {
                                    fs::create_dir(home_dir.join(".egnitely"))?;
                                    let mut db = PickleDb::new(
                                        home_dir.join(".egnitely").join("credentials"),
                                        PickleDbDumpPolicy::AutoDump,
                                        SerializationMethod::Json,
                                    );
                                    db.set("access_token", &_token_data.access_token)?;
                                    db.set("refresh_token", &_token_data.refresh_token)?;
                                    db.set("id_token", &_token_data.id_token)?;
                                } else {
                                    if !(Path::new(&home_dir.join(".egnitely").join("credentials")).is_file()) {
                                        let mut db = PickleDb::new(
                                            home_dir.join(".egnitely").join("credentials"),
                                            PickleDbDumpPolicy::AutoDump,
                                            SerializationMethod::Json,
                                        );
                                        db.set("access_token", &_token_data.access_token)?;
                                        db.set("refresh_token", &_token_data.refresh_token)?;
                                        db.set("id_token", &_token_data.id_token)?;
                                    } else{
                                        let mut db = PickleDb::load(
                                            home_dir.join(".egnitely").join("credentials"),
                                            PickleDbDumpPolicy::AutoDump,
                                            SerializationMethod::Json,
                                        )?;
                                        db.set("access_token", &_token_data.access_token)?;
                                        db.set("refresh_token", &_token_data.refresh_token)?;
                                        db.set("id_token", &_token_data.id_token)?;
                                    }
                                }
                                println!("");
                                println!("{}", "Successfully Logged In".green().bold());
                                break;
                            }
                        }
                        Err(err) => {
                            println!("Something went wrong, Error Decoding: {:?}", err);
                            break;
                        }
                    }
                }
            }
            thread::sleep(time::Duration::from_secs(device_code_res.interval));
        }

        Ok(json!({}))
    }

    pub async fn logout(&self) -> Result<(), Box<dyn Error>> {
        if let Some(home_dir) = dirs::home_dir() {
            if Path::new(&home_dir.join(".egnitely")).is_dir() {
                let mut db = PickleDb::load(
                    home_dir.join(".egnitely").join("credentials"),
                    PickleDbDumpPolicy::DumpUponRequest,
                    SerializationMethod::Json,
                )?;
                db.rem("access_token")?;
                db.rem("refresh_token")?;
                db.rem("id_token")?;
            }
        }
        println!("P{} Logged Out", "Successfully".green().bold());
        Ok(())
    }
}
