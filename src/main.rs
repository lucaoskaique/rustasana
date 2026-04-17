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
#[command(version = "0.6.0")]
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

    /// List projects
    #[command(alias = "p")]
    Projects,

    /// Get tasks
    #[command(alias = "ts")]
    Tasks {
        /// Without cache
        #[arg(short = 'n', long)]
        no_cache: bool,

        /// Update cache
        #[arg(short, long)]
        refresh: bool,

        /// Filter by assignee (user GID)
        #[arg(long, conflicts_with = "project")]
        assignee: Option<String>,

        /// Fetch all tasks from a specific project (project GID)
        #[arg(short, long, conflicts_with = "assignee")]
        project: Option<String>,

        /// Include completed tasks (default: incomplete only)
        #[arg(short = 'a', long)]
        all: bool,
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

        /// Project context (project GID)
        #[arg(long, conflicts_with = "assignee")]
        project: Option<String>,

        /// Assignee context (user GID)
        #[arg(long, conflicts_with = "project")]
        assignee: Option<String>,
    },

    /// Post comment
    #[command(alias = "cm")]
    Comment {
        /// Task index
        index: usize,

        /// Project context (project GID)
        #[arg(short, long, conflicts_with = "assignee")]
        project: Option<String>,

        /// Assignee context (user GID)
        #[arg(short = 'a', long, conflicts_with = "project")]
        assignee: Option<String>,
    },

    /// Complete task
    Done {
        /// Task index
        index: usize,

        /// Project context (project GID)
        #[arg(short, long, conflicts_with = "assignee")]
        project: Option<String>,

        /// Assignee context (user GID)
        #[arg(short = 'a', long, conflicts_with = "project")]
        assignee: Option<String>,
    },

    /// Assign task to a user
    Assign {
        /// Task index
        index: usize,
        /// User GID, 'me', 'null', or 'unassigned' (to unassign)
        assignee: String,

        /// Project context (project GID)
        #[arg(short, long, conflicts_with = "assignee_context")]
        project: Option<String>,

        /// Assignee context (user GID) - which cache to read from
        #[arg(long, conflicts_with = "project")]
        assignee_context: Option<String>,
    },

    /// Set due date
    Due {
        /// Task index
        index: usize,
        /// Due date (YYYY-MM-DD, 'today', or 'tomorrow')
        date: String,

        /// Project context (project GID)
        #[arg(short, long, conflicts_with = "assignee")]
        project: Option<String>,

        /// Assignee context (user GID)
        #[arg(short = 'a', long, conflicts_with = "project")]
        assignee: Option<String>,
    },

    /// Open a task in the web browser
    #[command(alias = "b")]
    Browse {
        /// Task index
        index: Option<usize>,

        /// Project context (project GID)
        #[arg(short, long, conflicts_with = "assignee")]
        project: Option<String>,

        /// Assignee context (user GID)
        #[arg(short = 'a', long, conflicts_with = "project")]
        assignee: Option<String>,
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

        /// Project context (project GID)
        #[arg(short, long, conflicts_with = "assignee")]
        project: Option<String>,

        /// Assignee context (user GID)
        #[arg(short = 'a', long, conflicts_with = "project")]
        assignee: Option<String>,
    },

    /// Install AI agent skill globally for OpenCode, Claude, Cursor, etc.
    InstallSkill {
        /// Force overwrite existing skill
        #[arg(short, long)]
        force: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Config => commands::config::run(),
        Commands::Workspaces => commands::workspaces::run(),
        Commands::Projects => commands::projects::run(),
        Commands::Tasks {
            no_cache,
            refresh,
            assignee,
            project,
            all,
        } => commands::tasks::run(no_cache, refresh, assignee, project, all),
        Commands::Task {
            index,
            verbose,
            json,
            project,
            assignee,
        } => commands::task::run(index, verbose, json, project, assignee),
        Commands::Comment {
            index,
            project,
            assignee,
        } => commands::comment::run(index, project, assignee),
        Commands::Done {
            index,
            project,
            assignee,
        } => commands::done::run(index, project, assignee),
        Commands::Assign {
            index,
            assignee,
            project,
            assignee_context,
        } => commands::assign::run(index, assignee, project, assignee_context),
        Commands::Due {
            index,
            date,
            project,
            assignee,
        } => commands::due::run(index, &date, project, assignee),
        Commands::Browse {
            index,
            project,
            assignee,
        } => commands::browse::run(index, project, assignee),
        Commands::Download {
            task_index,
            attachment,
            output,
            project,
            assignee,
        } => commands::download::run(task_index, attachment, output, project, assignee),
        Commands::InstallSkill { force } => commands::install_skill::run(force),
    }
}
