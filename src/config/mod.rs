
pub fn get_server_url() -> String {
    if std::env::var("EGNITELY_ENV").expect("Unable to get database url") == "dev" {
        return "https://dev.api.egnitely.com".to_string();
    }
    "https://api.egnitely.com".to_string()
}