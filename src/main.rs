use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Commands {
    /// analyse local snapshot
    Local {
        /// lists test values
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

fn main() {
    let cli = Cli::parse();
}
