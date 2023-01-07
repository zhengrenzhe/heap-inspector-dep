extern crate core;

use std::env;

use clap::{Parser, Subcommand};
use spinach::term;

use crate::commands::local::local::Local;
use crate::commands::realtime::realtime_command::realtime_command;
use crate::utils::logger::init_logger;

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

        /// serve port, default port is 9999
        #[arg(short)]
        port: Option<u16>,
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
    ctrlc::set_handler(|| {
        term::show_cursor();
        std::process::exit(0);
    })
    .expect("ctrl-c error");

    let cli = Cli::parse();

    if cli.verbose {
        env::set_var("RUST_LOG", "none,heap_inspector=debug");
    } else {
        env::set_var("RUST_LOG", "none,heap_inspector=info");
    }

    init_logger();

    match &cli.command {
        Some(Commands::Local { file_path, port }) => Local::new(file_path, port).start().await,
        Some(Commands::Realtime) => realtime_command(),
        _ => {}
    }
}
