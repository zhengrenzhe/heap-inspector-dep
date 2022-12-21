use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::commands::local::local_command::LC;
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
        file_path: PathBuf,
    },

    /// realtime analyse Chromium based browser tab v8 heap memory
    Realtime,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Local { file_path }) => {
            (LC {
                file_path: file_path.clone(),
            })
            .start()
            .await
        }
        Some(Commands::Realtime) => realtime_command(),
        _ => {}
    }
}
