use clap::{Parser, Subcommand};

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
            println!("Publish Called")
        }
        Some(Commands::Push) => {
            println!("Push Called")
        }
        Some(Commands::New { name }) => {
            println!("New Called");
            if let Some(name) = name {
                println!("Value: {}", name);
            }
        }
        Some(Commands::Create { name }) => {
            println!("Create Called");
            if let Some(name) = name {
                println!("Value: {}", name);
            }
        }
        Some(Commands::Login) => {
            println!("Login Called")
        }
        Some(Commands::Logout) => {
            println!("Logout Called")
        }
        None => {
            println!("Version: 0.1.0")
        }
    }
}
