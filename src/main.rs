use std::env;

use clap::{Parser, Subcommand};

use crate::commands::local::Local;
use crate::commands::realtime::realtime_command::realtime_command;

mod analyzer;
mod commands;
mod utils;

#[derive(Subcommand)]
enum Commands {
    /// analyse local snapshot
    Local {
        /// snapshot file path
        #[arg(short)]
        file_path: String,
    },

    /// realtime analyse Chromium based browser tab v8 heap memory
    Realtime,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Verbose log
    #[arg(long, short)]
    verbose: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if cli.verbose {
        env::set_var("RUST_LOG", "none,heap_inspector=debug");
    } else {
        env::set_var("RUST_LOG", "none,heap_inspector=info");
    }

    pretty_env_logger::init();

    match &cli.command {
        Some(Commands::Local { file_path, .. }) => Local::new(file_path).start().await,
        Some(Commands::Realtime) => realtime_command(),
        _ => {}
    }
}
