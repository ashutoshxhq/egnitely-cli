mod authn;
mod authz;
mod extras;
mod function;
mod generator;
mod handler;
mod project;
mod provider;
mod template;
mod config;
extern crate dirs;
use clap::{Parser, Subcommand};
use colored::*;
use handler::{EgnitelyHandler, EgnitelyResource};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Push the function to Egnitely
    Push {
        #[clap(short, long)]
        project: Option<String>,
    },
    /// Create a new Egnitely Function
    Create {
        #[clap(value_parser)]
        name: Option<String>,
    },
    /// Create a new Egnitely Function
    New {
        #[clap(value_parser)]
        name: Option<String>,
    },
    /// Apply a function template to a cloud environment
    Apply {
        #[clap(value_parser)]
        file: Option<String>,
    },
    /// List functions, templates or providers
    Get {
        #[clap(subcommand)]
        command: Option<GetCommands>,
    },
    /// Login to your Egnitely Account
    Login,
    /// Logout of your Egnitely Account
    Logout,
}

#[derive(Subcommand)]
enum GetCommands {
    /// Get list of functions
    Functions,
    /// Get list of projects
    Projects,
    /// Get list of providers
    Providers,
    /// Get list of templates
    Templates,
}


#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let egnitely = EgnitelyHandler::new();
    match &cli.command {
        Some(Commands::Create { name }) => {
            if let Some(_name) = name {
                let res = egnitely.create_function(_name.clone());
                match res {
                    Ok(_res) => {}
                    Err(err) => {
                        println!(
                            "{}",
                            format!("Error: {}", err).red().bold()
                            
                        );
                    }
                }
            }
        }
        Some(Commands::New { name }) => {
            if let Some(_name) = name {
                let res = egnitely.create_function(_name.clone());
                match res {
                    Ok(_res) => {}
                    Err(err) => {
                        println!(
                            "{}",
                            format!("Error: {}", err).red().bold()
                            
                        );
                    }
                }
            }
        }
        Some(Commands::Push { project }) => {
            if let Some(project) = project {
                let res = egnitely.push_function(project.clone()).await;
                match res {
                    Ok(_res) => {}
                    Err(err) => {
                        println!(
                            "{}",
                            format!("Error: {}", err).red().bold()
                            
                        );
                    }
                }
            }
        }
        Some(Commands::Get { command }) => {
            if let Some(command) = command {
                match command {
                    GetCommands::Functions => {
                        let res = egnitely.get_resource(EgnitelyResource::Function).await;
                        match res {
                            Ok(_res) => {}
                            Err(err) => {
                                println!(
                                    "{}",
                                    format!("Error: {}", err).red().bold()
                                    
                                );
                            }
                        }
                    }
                    GetCommands::Projects => {
                        let res = egnitely.get_resource(EgnitelyResource::Project).await;
                        match res {
                            Ok(_res) => {}
                            Err(err) => {
                                println!(
                                    "{}",
                                    format!("Error: {}", err).red().bold()
                                    
                                );
                            }
                        }
                    }
                    GetCommands::Providers => {
                        let _res = egnitely.get_resource(EgnitelyResource::Provider);
                    }
                    GetCommands::Templates => {
                        let _res = egnitely.get_resource(EgnitelyResource::AppTemplate);
                    }
                }
            }
        }
        Some(Commands::Apply { file }) => {
            if let Some(_file) = file {
                let _res = egnitely.apply_template(_file.clone());
            }
        }
        Some(Commands::Login) => {
            let res = egnitely.login().await;
            match res {
                Ok(_res) => {}
                Err(err) => {
                    println!(
                        "{}",
                        format!("{}: {}", "Error", err)
                        
                    );
                }
            }
        }
        Some(Commands::Logout) => {
            let res = egnitely.logout().await;
            match res {
                Ok(_res) => {}
                Err(err) => {
                    println!(
                        "{}",
                        format!("Error: {}", err).red().bold()
                        
                    );
                }
            }
        }
        None => {
            println!("Please type `egnite --help` to see list of commands")
        }
    }
}
