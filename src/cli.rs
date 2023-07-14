use clap::{Parser, Subcommand};

use crate::logic::{
    complete_task, create_project, create_task, delete_task, list_projects, list_tasks, TaskArgs,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Creates a new task
    New(TaskArgs),
    List {
        #[arg(short, long)]
        all: bool,
    },
    Complete {
        id: i32,
    },
    Delete {
        id: i32,
    },
    Projects {
        #[command(subcommand)]
        command: ProjectsCommands,
    },
}

#[derive(Subcommand)]
pub enum ProjectsCommands {
    New {
        name: String,

        #[arg(short, long)]
        shortcode: String,

        #[arg(short, long)]
        description: Option<String>,
    },
    List {},
}

pub fn execute_command(cli: Cli, conn: &mut redis::Connection) {
    match &cli.command {
        Commands::New(task) => {
            let _ = create_task(task, conn);
        }
        Commands::List { all } => {
            let _ = list_tasks(all, conn);
        }
        Commands::Complete { id } => {
            let _ = complete_task(id, conn);
        }
        Commands::Delete { id } => {
            let _ = delete_task(id, conn);
        }
        Commands::Projects { command } => match command {
            ProjectsCommands::New {
                name,
                description,
                shortcode,
            } => {
                let _ = create_project(name, shortcode, description.clone(), conn);
            }
            ProjectsCommands::List {} => {
                let _ = list_projects(conn);
            }
        },
    }
}
