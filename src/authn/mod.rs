use colored::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::HashMap, error::Error};
use std::{thread, time};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceCodeResponse {
    user_code: String,
    device_code: String,
    expires_in: u64,
    interval: u64,
    verification_uri: String,
    verification_uri_complete: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenErrorResponse {
    error: String,
    error_description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenSuccessResponse {
    access_token: String,
    id_token: String,
    refresh_token: String,
    scope: String,
    token_type: String,
    expires_in: u64,
}

pub struct EgnitelyAuthN {}

impl EgnitelyAuthN {
    pub fn new() -> Self {
        EgnitelyAuthN {}
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
            .await
            .unwrap()
            .json::<DeviceCodeResponse>()
            .await
            .unwrap();
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
            Ok(()) => {},
            Err(_err) => {
                println!("Unable to open browser, please manualy open the link.")
            },
        }

        let mut token_params = HashMap::new();
        token_params.insert("client_id", "wYIqKC5ffQnzy9lpGGt4Lia98NH5ea4m");
        token_params.insert("grant_type", "urn:ietf:params:oauth:grant-type:device_code");
        token_params.insert("device_code", &device_code_res.device_code);

        loop {
            let token_req = client
                .post("https://dev-qgdysq-r.us.auth0.com/oauth/token")
                .form(&token_params);
            let token_res = token_req
                .send()
                .await
                .unwrap()
                .json::<Value>()
                .await
                .unwrap();
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
                            println!("");
                            println!("{}", "Successfully Logged In".green().bold());
                            break;
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
        println!("Logout Triggered");
        Ok(())
    }
}
