use std::env;
//! SigmaOS-window-manager - SigmaOS Component CLI
//!
//! Command-line interface for SigmaOS-window-manager component.

use anyhow::Result;
use clap::{Parser, Subcommand};
use log::info;
use window-manager::{Component, ComponentStatus, Config, window-manager};

#[derive(Parser)]
#[command(name = "window-manager")]
#[command(about = "SigmaOS SigmaOS-window-manager component", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the component
    Init {
        /// Enable debug mode
        #[arg(short, long)]
        debug: bool,
    },
    /// Start the component
    Start,
    /// Stop the component
    Stop,
    /// Get component status
    Status,
    /// Run the component
    Run {
        /// Enable debug mode
        #[arg(short, long)]
        debug: bool,
    },
}

fn main() -> Result<()> { -> Result<()> {
    let cli = Cli::parse();
    
    let config = Config::default();
    let mut component = window-manager::new(config);

    match cli.command {
        Commands::Init { debug } => {
            if debug {
                println!("Initializing SigmaOS-window-manager in debug mode...");
            } else {
                println!("Initializing SigmaOS-window-manager...");
            }
            component.initialize()?;
            println!("SigmaOS-window-manager initialized successfully");
        }
        Commands::Start => {
            component.initialize()?;
            component.start()?;
            println!("SigmaOS-window-manager started successfully");
        }
        Commands::Stop => {
            component.stop()?;
            println!("SigmaOS-window-manager stopped successfully");
        }
        Commands::Status => {
            let status = component.status();
            println!("SigmaOS-window-manager status: {:?}", status);
        }
        Commands::Run { debug } => {
            if debug {
                println!("Running SigmaOS-window-manager in debug mode...");
            } else {
                println!("Running SigmaOS-window-manager...");
            }
            component.initialize()?;
            component.start()?;
            println!("SigmaOS-window-manager is running. Press Ctrl+C to stop.");
            
            // Keep running
            tokio::runtime::Runtime::new()?.block_on(async {
                tokio::signal::ctrl_c().await.unwrap();
                info!("Received Ctrl+C, shutting down...");
            });
            
            component.stop()?;
            println!("SigmaOS-window-manager stopped");
        }
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct Config {
    verbose: bool,
    dry_run: bool,
}

impl Config {
    fn from_env() -> Self {
        Config {
            verbose: env::var("VERBOSE").is_ok(),
            dry_run: env::var("DRY_RUN").is_ok(),
        }
    }
}
