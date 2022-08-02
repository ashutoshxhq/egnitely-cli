use clap::{Parser, Subcommand};
use create::create_function;
use login::login;
use logout::logout;

use crate::publish::publish_function;
mod login;
mod logout;
mod create;
mod publish;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Publish the function to Egnitely
    Publish,
    /// Push the function to Egnitely
    Push,
    /// Create a new Egnitely Function
    New {
        #[clap(value_parser)]
        name: Option<String>,
    },
    /// Create a new Egnitely Function
    Create {
        #[clap(value_parser)]
        name: Option<String>,
    },
    /// Login to your Egnitely Account
    Login,
    /// Logout of your Egnitely Account
    Logout,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Publish) => {
            let _res = publish_function();
        }
        Some(Commands::Push) => {
            let _res = publish_function();
        }
        Some(Commands::New { name }) => {
            if let Some(name) = name {
                let _res = create_function(name.clone());
            }
        }
        Some(Commands::Create { name }) => {
            if let Some(name) = name {
                let _res = create_function(name.clone());
            }
        }
        Some(Commands::Login) => {
            let _res = login();
        }
        Some(Commands::Logout) => {
            let _res = logout();
        }
        None => {
            println!("Version: 0.1.0")
        }
    }
}
