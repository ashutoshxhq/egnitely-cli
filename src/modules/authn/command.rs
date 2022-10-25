use std::{error::Error};

use super::service::AuthNService;

pub async fn login() -> Result<(), Box<dyn Error>> {
    let authn = AuthNService::new();
    authn.login().await?;
    Ok(())
}

pub async fn logout() -> Result<(), Box<dyn Error>> {
    let authn = AuthNService::new();
    authn.logout().await?;
    Ok(())
}