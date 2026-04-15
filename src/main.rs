mod api;
mod commands;
mod config;
mod models;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rustasana")]
#[command(about = "Rustasana - A blazingly fast Asana CLI client written in Rust", long_about = None)]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Asana configuration. Your settings will be saved in ~/.asana.yml
    #[command(alias = "c")]
    Config,

    /// Get workspaces
    #[command(alias = "w")]
    Workspaces,

    /// Get tasks
    #[command(alias = "ts")]
    Tasks {
        /// Without cache
        #[arg(short = 'n', long)]
        no_cache: bool,

        /// Update cache
        #[arg(short, long)]
        refresh: bool,
    },

    /// Get a task
    #[command(alias = "t")]
    Task {
        /// Task index
        index: Option<usize>,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,

        /// Output as JSON
        #[arg(short, long)]
        json: bool,
    },

    /// Post comment
    #[command(alias = "cm")]
    Comment {
        /// Task index
        index: usize,
    },

    /// Complete task
    Done {
        /// Task index
        index: usize,
    },

    /// Set due date
    Due {
        /// Task index
        index: usize,
        /// Due date (YYYY-MM-DD, 'today', or 'tomorrow')
        date: String,
    },

    /// Open a task in the web browser
    #[command(alias = "b")]
    Browse {
        /// Task index
        index: Option<usize>,
    },

    /// Download attachment from a task
    #[command(alias = "dl")]
    Download {
        /// Task index
        task_index: usize,

        /// Attachment index or GID
        attachment: String,

        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Config => commands::config::run(),
        Commands::Workspaces => commands::workspaces::run(),
        Commands::Tasks { no_cache, refresh } => commands::tasks::run(no_cache, refresh),
        Commands::Task {
            index,
            verbose,
            json,
        } => commands::task::run(index, verbose, json),
        Commands::Comment { index } => commands::comment::run(index),
        Commands::Done { index } => commands::done::run(index),
        Commands::Due { index, date } => commands::due::run(index, &date),
        Commands::Browse { index } => commands::browse::run(index),
        Commands::Download {
            task_index,
            attachment,
            output,
        } => commands::download::run(task_index, attachment, output),
    }
}
