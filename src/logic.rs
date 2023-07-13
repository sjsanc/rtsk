use chrono::{DateTime, Utc};
use clap::Args;
use colored::Colorize;
use redis::Commands;
use serde::{Deserialize, Serialize};
use tabled::{settings::Color, Tabled};

use crate::display::*;
use crate::utils::{get_due_or_default, get_priority_or_default};

#[derive(Serialize, Deserialize, Tabled)]
pub struct Task {
    id: i32,

    #[tabled(display_with("Self::display_text", self))]
    task: String,

    #[tabled(display_with = "display_priority")]
    priority: Priority,

    #[tabled(skip)]
    done: bool,

    #[tabled(display_with = "display_age", rename = "Age")]
    created_at: DateTime<Utc>,

    #[tabled(display_with = "display_due_date")]
    due: Option<DateTime<Utc>>,

    #[tabled(skip)]
    updated_at: DateTime<Utc>,
}

impl Task {
    fn display_text(&self) -> String {
        match self.done {
            true => format!("{}", format!("{}", &self.task.green().strikethrough())),
            false => format!("{}", self.task),
        }
    }
}

#[derive(Args)]
pub struct TaskArgs {
    task: String,

    #[arg(short, long)]
    priority: Option<String>,

    #[arg(short, long)]
    due: Option<String>,
    project: Option<String>,
    tags: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum Priority {
    Now,
    High,
    Low,
}

pub fn create_task(args: &TaskArgs, conn: &mut redis::Connection) -> Result<(), redis::RedisError> {
    let count: isize = redis::cmd("DBSIZE").query(conn).unwrap();

    let priority = get_priority_or_default(&args.priority);

    let due = get_due_or_default(&args.due);

    let new_task = Task {
        id: (count as i32) + 1,
        task: args.task.clone(),
        priority: priority,
        due,
        // project: args.project.clone(),
        // tags: args.tags.clone(),
        done: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let task_json = serde_json::to_string(&new_task).unwrap();

    conn.set(new_task.id, &task_json)?;

    match new_task.priority {
        Priority::Now => println!("{}", "Urgent Task created! Do it now!".blue().bold()),
        Priority::High => println!("{}", "High priority task created!".blue().bold()),
        Priority::Low => println!("{}", "Task created!".blue().bold()),
    }

    print_tasks(&vec![new_task], Some(Color::FG_BRIGHT_BLUE));

    Ok(())
}

pub fn complete_task(id: &i32, conn: &mut redis::Connection) -> Result<(), redis::RedisError> {
    let task_json: String = conn.get(id).unwrap();

    let mut task: Task = serde_json::from_str(&task_json).unwrap();

    task.done = true;

    let task_json = serde_json::to_string(&task).unwrap();

    conn.set(id, &task_json)?;

    println!("{}", "Task completed! Well done!".green().bold());

    print_tasks(&vec![task], Some(Color::FG_BRIGHT_GREEN));

    Ok(())
}

pub fn delete_task(id: &i32, conn: &mut redis::Connection) -> Result<(), redis::RedisError> {
    let task_json: String = conn.get(id).unwrap();

    let task: Task = serde_json::from_str(&task_json).unwrap();

    conn.del(id)?;

    println!("{}", "Task deleted!".red().bold());

    print_tasks(&vec![task], Some(Color::FG_BRIGHT_RED));

    Ok(())
}

pub fn list_tasks(all: &bool, conn: &mut redis::Connection) -> Result<(), redis::RedisError> {
    let keys: Vec<i32> = redis::cmd("KEYS").arg("*").query(conn).unwrap();

    let mut tasks: Vec<Task> = Vec::new();

    for key in keys {
        let task: String = conn.get(key).unwrap();
        let task: Task = serde_json::from_str(&task).unwrap();

        if !all && task.done {
            continue;
        } else {
            tasks.push(task);
        }
    }

    tasks.sort_by_key(|k| k.id);

    print_tasks(&tasks, None);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
