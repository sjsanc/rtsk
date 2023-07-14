use chrono::{DateTime, Utc};
use clap::Args;
use colored::Colorize;
use redis::Commands;
use redis::RedisResult;
use serde::{Deserialize, Serialize};
use tabled::{settings::Color, Tabled};
use uuid::Uuid;

use crate::db::get_struct;
use crate::db::get_struct_by_property;
use crate::db::store_struct;
use crate::display::*;
use crate::utils::*;

trait HasUuid {
    fn get_uuid(&self) -> Uuid;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Project {
    uuid: Uuid,
    name: String,
    shortcode: String,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl HasUuid for Project {
    fn get_uuid(&self) -> Uuid {
        self.uuid
    }
}

#[derive(Serialize, Deserialize, Tabled, Clone)]
pub struct Task {
    id: i32,

    #[tabled(skip)]
    uuid: Uuid,

    #[tabled(display_with("Self::display_text", self))]
    task: String,

    #[tabled(display_with = "display_priority")]
    priority: Priority,

    #[tabled(skip)]
    done: bool,

    #[tabled(display_with = "display_age", rename = "age")]
    created_at: DateTime<Utc>,

    #[tabled(display_with = "display_tags")]
    tags: Vec<String>,

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

impl HasUuid for Task {
    fn get_uuid(&self) -> Uuid {
        self.uuid
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

    #[arg(short, long)]
    tags: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Priority {
    Now,
    High,
    Low,
}

pub fn create_task(
    args: &TaskArgs,
    conn: &mut redis::Connection,
) -> Result<Task, redis::RedisError> {
    let count: isize = redis::cmd("DBSIZE").query(conn).unwrap();

    let new_task = Task {
        id: (count as i32) + 1,
        uuid: Uuid::new_v4(),
        task: args.task.clone(),
        priority: get_priority_or_default(&args.priority),
        due: get_due_or_default(&args.due),
        tags: get_tags_or_default(&args.tags),
        done: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    store_struct("task", &new_task.uuid.to_string(), &new_task, conn)?;

    match new_task.priority {
        Priority::Now => println!("{}", "Urgent Task created! Do it now!".blue().bold()),
        Priority::High => println!("{}", "High priority task created!".blue().bold()),
        Priority::Low => println!("{}", "Task created!".blue().bold()),
    }

    print_tasks(&vec![new_task.clone()], Some(Color::FG_BRIGHT_BLUE));

    Ok(new_task)
}

pub fn complete_task(id: &i32, conn: &mut redis::Connection) -> Result<(), redis::RedisError> {
    let mut task = get_struct_by_property::<Task, i32>("task", "id", *id, conn)?
        .ok_or_else(|| {
            println!("{}", "Task not found!".red().bold());
        })
        .unwrap();

    task.done = true;

    store_struct("task", &task.uuid.to_string(), &task, conn)?;

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
    let keys: RedisResult<Vec<String>> = conn.keys("*task*");

    let mut tasks: Vec<Task> = Vec::new();

    for key in keys.unwrap() {
        let task: Task = get_struct(&key, conn).unwrap();

        if *all {
            tasks.push(task);
        } else if !task.done {
            tasks.push(task);
        }
    }

    tasks.sort_by_key(|k| k.id);

    print_tasks(&tasks, None);

    Ok(())
}

pub fn create_project(
    name: &str,
    shortcode: &str,
    description: Option<String>,
    conn: &mut redis::Connection,
) -> Result<Project, redis::RedisError> {
    let new_project = Project {
        uuid: Uuid::new_v4(),
        name: name.to_string(),
        shortcode: shortcode.to_string(),
        description,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let project_json = serde_json::to_string(&new_project).unwrap();

    conn.set(new_project.uuid.to_string(), &project_json)?;

    println!("{}", "Project created!".blue().bold());

    Ok(new_project)
}

pub fn list_projects(conn: &mut redis::Connection) -> Result<(), redis::RedisError> {
    let keys: Vec<String> = redis::cmd("KEYS").arg("*").query(conn).unwrap();

    let mut projects: Vec<Project> = Vec::new();

    for key in keys {
        let project: String = conn.get(key).unwrap();
        let project: Project = serde_json::from_str(&project).unwrap();

        projects.push(project);
    }

    projects.sort_by_key(|k| k.name.clone());

    // print_projects(&projects, None);

    Ok(())
}

// =================================================================================
// TESTS
// =================================================================================

#[cfg(test)]
mod tests {
    use redis::Commands;

    use crate::db;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_create_task() {
        let mut conn = db::connect_to_db().unwrap();
        let count: usize = redis::cmd("DBSIZE").query(&mut conn).unwrap();

        let args = super::TaskArgs {
            task: "Test task".to_string(),
            priority: None,
            due: None,
            project: None,
            tags: Some("TEST".to_string()),
        };

        let test_task = super::create_task(&args, &mut conn).unwrap();

        let keys: Vec<String> = redis::cmd("KEYS").arg("*").query(&mut conn).unwrap();

        assert_eq!(keys.len(), count + 1);

        for key in keys {
            let task_json: String = conn.get(key).unwrap();
            let task: super::Task = serde_json::from_str(&task_json).unwrap();

            if task.uuid == test_task.uuid {
                assert_eq!(task.task, test_task.task);

                let _: () = conn.del(task.id).unwrap();
            }
        }
    }
}
