mod authz;
mod config;
mod extras;
mod modules;
extern crate dirs;
use clap::{Parser, Subcommand};
use colored::*;
use modules::{authn, function, global, project};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List functions, templates or providers
    Get {
        #[clap(subcommand)]
        command: Option<GetCommands>,
    },
    /// Create a new Egnitely Function
    Generate {
        #[clap(subcommand)]
        command: Option<GenerateCommands>,
    },
    /// Apply a file to
    Apply {
        #[clap(value_parser)]
        file: Option<String>,
    },
    /// Push the function to Egnitely
    Push,
    /// Login to your Egnitely Account
    Login,
    /// Logout of your Egnitely Account
    Logout,
}

#[derive(Subcommand)]
enum GetCommands {
    /// Get list of functions
    Functions {
        #[clap(short, long)]
        project: Option<String>,
    },
    /// Get list of projects
    Projects,
    /// Get list of providers
    Applications {
        #[clap(short, long)]
        project: Option<String>,
    },
}

#[derive(Subcommand)]
enum GenerateCommands {
    /// Generate function
    Function {
        #[clap(value_parser)]
        name: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Generate { command }) => {
            if let Some(command) = command {
                match command {
                    GenerateCommands::Function { name } => {
                        if let Some(name) = name {
                            let res = function::command::create_function(name.clone());
                            match res {
                                Ok(_res) => {}
                                Err(err) => {
                                    println!("{}", format!("Error: {}", err).red().bold());
                                }
                            }
                        }
                    }
                }
            }
        }
        Some(Commands::Push) => {
            let res = function::command::push_function().await;
            match res {
                Ok(_res) => {}
                Err(err) => {
                    println!("{}", format!("Error: {}", err).red().bold());
                }
            }
        }
        Some(Commands::Get { command }) => {
            if let Some(command) = command {
                match command {
                    GetCommands::Functions { project } => {
                        if let Some(project) = project {
                            let res = function::command::get_function(project.to_string()).await;
                            match res {
                                Ok(_res) => {}
                                Err(err) => {
                                    println!("{}", format!("Error: {}", err).red().bold());
                                }
                            }
                        } else{
                            println!("{}: Plese provide a project name using the -p flag", "Error".red().bold())
                        }
                    }
                    GetCommands::Projects => {
                        let res = project::command::get_projects().await;
                        match res {
                            Ok(_res) => {}
                            Err(err) => {
                                println!("{}", format!("Error: {}", err).red().bold());
                            }
                        }
                    }
                    GetCommands::Applications { project } => {
                        if let Some(_project) = project {
                            // let _res = egnitely.get_resource(EgnitelyResource::Application);
                        }
                    }
                }
            }
        }
        Some(Commands::Apply { file }) => {
            if let Some(_file) = file {
                let _res = global::command::apply_resource(_file.clone()).await;
            }
        }
        Some(Commands::Login) => {
            let res = authn::command::login().await;
            match res {
                Ok(_res) => {}
                Err(err) => {
                    println!("{}", format!("{}: {}", "Error", err));
                }
            }
        }
        Some(Commands::Logout) => {
            let res = authn::command::logout().await;
            match res {
                Ok(_res) => {}
                Err(err) => {
                    println!("{}", format!("Error: {}", err).red().bold());
                }
            }
        }
        None => {
            println!("Lost? type `egnitely --help` to see list of commands")
        }
    }
}
