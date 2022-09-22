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
    /// Trigger Egnitely Function with test data
    Trigger {
        #[clap(value_parser)]
        file: Option<String>,
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
    /// Delete a function, template or provider
    Delete {
        #[clap(subcommand)]
        command: Option<DeleteCommand>,
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

#[derive(Subcommand)]
enum DeleteCommand {
    /// Delete a function with the given id
    Function {
        #[clap(value_parser)]
        id: Option<String>,
    },
    /// Delete a provider with the given id
    Provider {
        #[clap(value_parser)]
        id: Option<String>,
    },
    /// Delete a template with the given id
    Template {
        #[clap(value_parser)]
        id: Option<String>,
    },
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
                            "{}: Something went wrong, Error: {:?}",
                            "Error:".red().bold(),
                            err
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
                            "{}: Something went wrong, Error: {:?}",
                            "Error:".red().bold(),
                            err
                        );
                    }
                }
            }
        }
        Some(Commands::Trigger { file }) => {
            if let Some(_file) = file {
                let _res = egnitely.trigger_function(_file.clone());
            }
        }
        Some(Commands::Push { project }) => {
            if let Some(project) = project {
                let res = egnitely.push_function(project.clone()).await;
                match res {
                    Ok(_res) => {}
                    Err(err) => {
                        println!(
                            "{}: Something went wrong, Error: {:?}",
                            "Error:".red().bold(),
                            err
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
                                    "{}: Something went wrong, Error: {:?}",
                                    "Error:".red().bold(),
                                    err
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
                                    "{}: Something went wrong, Error: {:?}",
                                    "Error:".red().bold(),
                                    err
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
        Some(Commands::Delete { command }) => {
            if let Some(command) = command {
                match command {
                    DeleteCommand::Function { id } => {
                        if let Some(id) = id {
                            let _res =
                                egnitely.delete_resource(EgnitelyResource::Function, id.clone());
                        }
                    }
                    DeleteCommand::Provider { id } => {
                        if let Some(id) = id {
                            let _res =
                                egnitely.delete_resource(EgnitelyResource::Provider, id.clone());
                        }
                    }
                    DeleteCommand::Template { id } => {
                        if let Some(id) = id {
                            let _res =
                                egnitely.delete_resource(EgnitelyResource::AppTemplate, id.clone());
                        }
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
                        "{}: Something went wrong, Error: {:?}",
                        "Error:".red().bold(),
                        err
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
                        "{}: Something went wrong, Error: {:?}",
                        "Error:".red().bold(),
                        err
                    );
                }
            }
        }
        None => {
            println!("Version: 0.1.0")
        }
    }
}
