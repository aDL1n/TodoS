mod cli;
mod storage;

use crate::cli::{Cli, Commands};
use chrono::Utc;
use clap::{Parser};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::string::String;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    name: String,
    value: String,
    timestamp: i64,
    completed: bool
}

fn main() {
    let home_path = dirs::home_dir().unwrap();
    let path = home_path.join(".todos");

    if !path.exists() {
        fs::create_dir(&path).expect("Unable to create todo directory");
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::Create { todo_name } => {
            match create_todo(&path, &todo_name) {
                Ok(_) => println!("New todo with name {} created!", todo_name.yellow()),
                Err(error) =>
                    eprintln!("{} {:?}", "Unable to create file!".red().bold(), error)
            }
        }
        Commands::Add { todo_name, task_name, value } => {
            match add_to_todo(&path, &todo_name, &task_name, value) {
                Ok(_) => match task_name {
                    Some(task_name) => {
                        println!("{} {} {} {}",
                                 "New task added to".green(),
                                 todo_name.yellow().bold(),
                                 "with name".green(),
                                 task_name.yellow().bold()
                        )
                    }
                    None =>
                        println!("{} {}", "New task added to".green(), todo_name.yellow().bold())
                },
                Err(error) => eprintln!("{}", error)
            }
        }
        Commands::Complete { todo_name, task_name} => {
            match task_name {
                Some(task_name) => match complete_task(&path, &todo_name, &task_name) {
                    Ok(_) => println!("Task {} completed successfully!", task_name.yellow()),
                    Err(error) => eprintln!("{}", error)
                }
                None => match complete_all_tasks(&path, &todo_name) {
                    Ok(_) => println!("{} {}", "Completed all tasks in".green(), todo_name.yellow()),
                    Err(error) => eprintln!("{}", error)
                }
            }
        }
        Commands::Remove { todo_name, task_name } => {
            match task_name {
                Some(task_name) => match remove_task(&path, &todo_name, &task_name) {
                    Ok(_) => println!("Task {} removed successfully!", &task_name.yellow()),
                    Err(error) => eprintln!("{}", error)
                }
                None => match remove_todo(&path, &todo_name) {
                    Ok(_) => println!("TODO {} removed successfully!", todo_name.yellow()),
                    Err(error) => eprintln!("{}", error)
                }
            }
        }
        Commands::List { todo_name, task_name } => {
            match task_name {
                Some(task_name) => match print_task(&path, &todo_name, &task_name) {
                    Ok(_) => (),
                    Err(error) => eprintln!("{}", error)
                }
                None => match print_todo(&path, &todo_name) {
                    Ok(_) => (),
                    Err(error) => eprintln!("{}", error)
                }
            }

        }
    }
}

fn create_todo(todos_path: &PathBuf, todo_name: &String) -> Result<(), Box<dyn std::error::Error>> {
    let todo_file = todos_path.join(format!("{}.json", todo_name));

    if todo_file.exists() {
        return Err("This TODO already exists!".red().bold().into())
    }

    File::create(&todo_file)?;

    Ok(())
}

fn add_to_todo(todos_path: &PathBuf, todo_name: &String, task_name: &Option<String>, value: String)
    -> Result<(), Box<dyn std::error::Error>>
{
    let todo_file_path = storage::get_todo_path(todos_path, &todo_name)?;

    let mut tasks: Vec<Task> = storage::read_todo_file(&todo_file_path)?;

    tasks.push(Task {
        name: task_name.clone().unwrap_or(tasks.len().to_string()),
        value,
        timestamp: Utc::now().timestamp(),
        completed: false
    });

    storage::rewrite_todo_file(todo_file_path, tasks)?;

    Ok(())
}

fn complete_all_tasks(todos_path: &PathBuf, todo_name: &String)
    -> Result<(), Box<dyn std::error::Error>>
{
    let todo_file_path = storage::get_todo_path(todos_path, todo_name)?;
    let mut tasks: Vec<Task> = storage::read_todo_file(&todo_file_path)?;

    tasks.iter_mut().for_each(|todo_item| {
        todo_item.completed = true;
    });

    storage::rewrite_todo_file(todo_file_path, tasks)?;

    Ok(())
}

fn complete_task(todos_path: &PathBuf, todo_name: &String, task_name: &String)
    -> Result<(), Box<dyn std::error::Error>>
{
    let todo_file_path = storage::get_todo_path(todos_path, todo_name)?;
    let mut tasks: Vec<Task> = storage::read_todo_file(&todo_file_path)?;

    match tasks.iter_mut()
        .find(|todo_item| {todo_item.name.eq(task_name)})
    {
        Some(todo_item) => {
            todo_item.completed = true;
        }
        None => return Err(format!(
            "Task with name {} in {} not found!",
            task_name.yellow(),
            todo_name.yellow()
        ).red().bold().into())
    }

    storage::rewrite_todo_file(todo_file_path, tasks)?;

    Ok(())
}

fn remove_todo(todos_path: &PathBuf, todo_name: &String) -> Result<(), Box<dyn std::error::Error>> {
    let todo_file_path = storage::get_todo_path(todos_path, todo_name)?;

    fs::remove_file(&todo_file_path)?;

    Ok(())
}

fn remove_task(todos_path: &PathBuf, todo_name: &String, task_name: &String)
    -> Result<(), Box<dyn std::error::Error>>
{
    let todo_file_path = storage::get_todo_path(todos_path, todo_name)?;
    let mut tasks: Vec<Task> = storage::read_todo_file(&todo_file_path)?;

    match find_task(&tasks, &task_name) {
        Some(task_index) => {
            tasks.remove(task_index);
        }
        None => {
            Err(format!("Task with name {} not found!", task_name.yellow()).red())?;
        }
    }

    storage::rewrite_todo_file(todo_file_path, tasks)?;

    Ok(())
}

fn print_todo(todos_path: &PathBuf, todo_name: &String) -> Result<(), Box<dyn std::error::Error>> {
    let todo_file_path = storage::get_todo_path(todos_path, todo_name)?;
    let tasks: Vec<Task> = storage::read_todo_file(&todo_file_path)?;

    println!("{} {}", "TODO".green().bold(), todo_name.yellow());

    if tasks.is_empty() {
        println!("Empty...");
        return Ok(())
    }

    for task in tasks {
        println!("Task {}", task.name);
        println!("{}", task.value);
        println!("Completed={} - {}", task.completed, chrono::DateTime::from_timestamp(task.timestamp, 0)
            .unwrap_or_default()
            .format("%d.%m.%Y %H:%M:%S").to_string());
        println!("\n");
    }

    Ok(())
}

fn print_task(todos_path: &PathBuf, todo_name: &String, task_name: &String)
    -> Result<(), Box<dyn std::error::Error>>
{
    let todo_file_path = storage::get_todo_path(todos_path, todo_name)?;
    let tasks: Vec<Task> = storage::read_todo_file(&todo_file_path)?;

    match find_task(&tasks, &task_name) {
        Some(task_index) => {
            println!("Task {} from {}", task_name.yellow(), todo_name.yellow());
            println!("{}", tasks[task_index].value);
            println!("Completed={} - {}",
                     tasks[task_index].completed,
                     chrono::DateTime::from_timestamp(tasks[task_index].timestamp, 0)
                         .unwrap_or_default()
                         .format("%d.%m.%Y %H:%M:%S").to_string()
            );
        }
        None => return Err(format!("Task with name {} not found!", task_name.yellow()).red().into())
    }
    Ok(())
}

fn find_task(tasks: &Vec<Task>, task_name: &String) -> Option<usize> {
    for index in 0..tasks.len() {
        let task = &tasks[index];

        if task.name.eq(task_name) {
            return Some(index);
        }
    }
    None
}