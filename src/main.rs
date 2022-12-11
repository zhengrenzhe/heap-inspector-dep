use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::commands::local::local_command::local_command;
use crate::commands::realtime::realtime_command::realtime_command;
use crate::commands::workbench::workbench_command;

mod commands;
pub mod utils;

#[derive(Subcommand)]
enum Commands {
    /// analyse local snapshot
    Local {
        /// snapshot file path
        #[arg(short)]
        file: PathBuf,
    },

    /// start web workbench
    Workbench,

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
        Some(Commands::Local { file }) => local_command(file).await,
        Some(Commands::Workbench) => workbench_command(),
        Some(Commands::Realtime) => realtime_command().await,
        _ => {}
    }
}
