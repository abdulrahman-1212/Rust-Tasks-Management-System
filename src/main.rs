use std::fs::File;
use std::io::{self, Write, Read};
use serde_json;
use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveDateTime};
use clap::{Arg, Command};


fn main() {
    let matches = Command::new("Task Manager")
        .version("1.0")
        .author("Your Name")
        .about("Manage your tasks")
        .subcommand(
            Command::new("add")
                .about("Add a new task")
                .arg(Arg::new("title").required(true))
                .arg(Arg::new("description").required(true))
                .arg(Arg::new("due_date").required(true))
                .arg(Arg::new("priority").required(true)),
        )
        .subcommand(Command::new("list").about("List all tasks"))
        .subcommand(Command::new("complete").about("Mark a task as complete"))
        .get_matches();

    let mut tasks = load_tasks();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let title = sub_matches.value_of("title").unwrap();
            let description = sub_matches.value_of("description").unwrap();
            let due_date = NaiveDate::parse_from_str(sub_matches.value_of("due_date").unwrap(), "%Y-%m-%d").unwrap();
            let priority: u8 = sub_matches.value_of("priority").unwrap().parse().unwrap();

            let task = Task::new(title, description, due_date, priority);
            tasks.push(task);
            save_tasks(&tasks).unwrap();
            println!("Task added!");
        }
        Some(("list", _)) => {
            for task in &tasks {
                println!("{:?}", task);
            }
        }
        Some(("complete", sub_matches)) => {
            // Code to mark a task as completed (you can identify a task by index or title)
        }
        _ => {}
    }
}



// ----------------------------------------------------- Task Structure ---------------------------------

#[derive(Serialize, Deserialize, Debug)]

struct Task {
    title: String,
    description: String,
    due_date: NaiveDate,
    priority: u8,
    completed: bool
}

impl Task {
    fn new(title: &str, description: &str, due_date: NaiveDate, priority: u8) -> Task {
        Task {
            title: title.to_string(),
            description: description.to_string(),
            due_date,
            priority,
            completed: false,
        }
    }

    fn mark_completed(&mut self) {
        self.completed = true;
    }
}

// ------------------------ Save & Load ------------------------------------

fn save_tasks(tasks: &Vec<Task>) -> io::Result<()> {
    let mut file = File::create("tasks.json")?;
    let json = serde_json::to_string(tasks).unwrap();
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn load_tasks() -> Vec<Task> {
    let mut file = match File::open("tasks.json") {
        Ok(file) => file,
        Err(_) => return vec![],
    };

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let tasks: Vec<Task> = serde_json::from_str(&content).unwrap_or_else(|_| vec![]);
    tasks
}