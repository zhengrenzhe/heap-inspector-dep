use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::commands::local::local;
use crate::commands::realtime::realtime;
use crate::commands::workbench::workbench;

mod commands;

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
        Some(Commands::Local { file }) => local(file).await,
        Some(Commands::Workbench) => workbench(),
        Some(Commands::Realtime) => realtime().await,
        _ => {}
    }
}
