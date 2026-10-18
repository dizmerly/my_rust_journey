use clap::Parser;
use gstd;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use toml;

const TASK_PATH: &str = "./tasks.toml";

#[derive(Serialize, Deserialize, Parser)]
struct Task {
    command: String,
    name: Option<String>,
    description: Option<String>,
    completion_status: Option<bool>,
}
#[derive(Serialize, Deserialize)]
struct TaskList {
    tasks: Vec<Task>,
}

// Adds to a constant filepath defined at the top of the program
fn add_task(mut task: Task, mut task_list: TaskList) {
    let mut task_list_file = OpenOptions::new()
        .append(true)
        .open(TASK_PATH)
        .expect("Could not open file");

    if task.completion_status == None {
        task.completion_status = gstd::Some(false);
    }
    task_list.tasks.push(task);
    let toml_text = toml::to_string_pretty(&task_list).expect("Could not serialize task list");

    task_list_file
        .write_all(task_list.as_bytes())
        .expect("Could not write to file");
}

fn show_tasks() -> io::Result<()> {
    let task_list_file = fs::read_to_string(TASK_PATH)?;
    println!("Task List\n {}", task_list_file);
    Ok(())
}

fn finish_task(task_name: String) -> io::Result<()> {
    let mut task_list_file = OpenOptions::new()
        .write(true)
        .read(true)
        .open(TASK_PATH)
        .expect("Could not open file");
<<<<<<< HEAD

=======
>>>>>>> a4ecb2a82ffee041129034fb74130f04756a47ef
    Ok(())
}

fn clean_tasks() -> std::io::Result<()> {
    // Clean the file of any items
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(TASK_PATH)?;

    println!("File truncated to 0 bytes.");
    Ok(())
}

fn main() {
    // let the program read some sort of arguments and parse them
    let args = Task::parse();
    let mut task_list = TaskList { tasks: Vec::new() };

    // Match command to the appropriate action the user wants to perform
    match args.command.as_str() {
        // Добавить - add
        "dobavit" => {
            add_task(args, task_list);
        }
        // Покажи - show
        "pokazhi" => {
            show_tasks();
        }
        // Удали - delete
        "udali" => {
            println!("udalil");
        }
        // Измени - change
        "izmeni" => {
            println!("izmenil");
        }

        // Закончил - finished
        "zakonchil" => {
            println!("zakonchil");
        }

        // Убери - clean
        "uberi" => {
            clean_tasks().expect("Could not clean file");
        }

        "help" => {
            println!(
                "
                    dobavit - add\n
                    pokazhi - show\n
                    uberi - clean\n
                "
            );
        }
        // помаги - help
        "pomagi" => {
            println!(
                "
                    dobavit - add\n
                    pokazhi - show\n
                    uberi - clean\n
                "
            );
        }

        _ => {
            println!("Error parsing command");
        }
    }
}
