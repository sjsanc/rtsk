use clap::Parser;

extern crate redis;

mod cli;
mod db;
mod display;
mod logic;
mod utils;

fn main() {
    let args = cli::Cli::parse();
    let mut conn = db::connect_to_db().unwrap();

    cli::execute_command(args, &mut conn);
}

// rtsk new "This is a task" --priority now|high|low|cold --due 2021-01-01  --project "rust-task" --tags "rust,task,cli"
// -> Created a new task with ID 1

// rtsk projects new "rust-task" // create a new project
// rtks projects list // list all projects

// rtsk tags list

// rtsk list // list all tasks
// rtsk list --project "rust-task" // list all tasks by project
// rtsk list --tags "rust" // list all tasks with the tag "rust"
// rtsk list --priority high // list all tasks with the priority "high"
// rstk list --priority > high // list all tasks with the priority greater than "high"

// Tasks can be produced under a project
// Projects provide a new id of the type <shortcode>-<id>
// Everything is kept in the same Redis db

// The issue is that Tasks need an easy way to be selected
// We want to give them an easy ID, but also a UUID
// Projects need the same thing. But they can't have the same IDs, so IDs need to be more than just I32
