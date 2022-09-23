pub fn get_server_url() -> String {
    let egnitely_env = std::env::var("EGNITELY_ENV");
    match egnitely_env {
        Ok(egnitely_env) => {
            if egnitely_env == "dev" {
                return "https://dev.api.egnitely.com".to_string();
            }
            "https://api.egnitely.com".to_string()
        }
        Err(_err) => "https://api.egnitely.com".to_string(),
    }
}
