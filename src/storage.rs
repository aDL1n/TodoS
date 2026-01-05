use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use colored::Colorize;
use crate::Task;

pub fn get_todo_path(todos_path: &PathBuf, todo_file_name: &String)
                 -> Result<PathBuf, Box<dyn std::error::Error>>
{
    let todo_file_path = todos_path.join(format!("{}.json", todo_file_name));

    if !todo_file_path.exists() {
        return Err("TODO with this name not exist!".red().bold().into());
    }

    Ok(todo_file_path)
}

pub fn read_todo_file(todo_file_path: &PathBuf) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let todo_file = OpenOptions::new().read(true).open(&todo_file_path)?;

    let reader = BufReader::new(&todo_file);
    let tasks: Vec<Task> = serde_json::from_reader(reader)
        .unwrap_or_else(|_| Vec::new());

    Ok(tasks)
}

pub fn rewrite_todo_file(todo_file_path: PathBuf, new_tasks: Vec<Task>)
                     -> Result<(), Box<dyn std::error::Error>>
{
    let todo_file = OpenOptions::new().write(true).truncate(true).open(&todo_file_path)?;

    let writer = BufWriter::new(todo_file);
    serde_json::to_writer_pretty(writer, &new_tasks)?;

    Ok(())
}