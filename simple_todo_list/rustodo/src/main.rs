use clap::Parser;
use gstd;
use serde::{Deserialize, Serialize};
use std::fs;
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
        .write_all(toml_text.as_bytes())
        .expect("Could not write to file");
}


fn show_tasks() -> io::Result<()> {
    let task_list_file = fs::read_to_string(TASK_PATH)?;
    println!("Task List\n {}", task_list_file);
    Ok(())
}

fn finish_task(task_name: Option<String>) -> io::Result<()> {
    let contents = fs::read_to_string(TASK_PATH)?;
    let mut task_list: TaskList = toml::from_str(&contents)
        .expect("Couldn't convert toml file into list");
    for task in &mut task_list.tasks {
        match &task.name {
            name if name == &task_name => {
                task.completion_status = Some(true);
            }
            _ => {
                continue
            }
        }
    }

    
    let updated_contents = toml::to_string_pretty(&task_list)
        .expect("Couldn't serialize updated task list");

    fs::write(TASK_PATH, updated_contents)?;
    Ok(())
}


fn change_task(task_name: Option<String>) -> io::Result<()> { 
    let contents = fs::read_to_string(TASK_PATH)?;
    let mut task_list: TaskList = toml::from_str(&contents)
        .expect("Couldn't convert toml file into list");
    
    for task in &mut task_list.tasks {
        match &task.name {
            name if name == &task_name => {
                println!("Would you like to change the name or description? Enter (n/d)");
                let mut choice = String::new();
                io::stdin().read_line(&mut choice)?;

                loop {
                    match choice.trim() {
                        "n" =>{
                            println!("Enter new name: ");
                            let mut new_name = String::new();
                            io::stdin().read_line(&mut new_name)?;
                            task.name = Some(new_name);
                            break;
                        }
                        
                        "d" => {
                            println!("Enter new description: ");
                            let mut new_description = String::new();
                            io::stdin().read_line(&mut new_description)?;
                            task.name = Some(new_description);
                            break;
                        }
                        _ => {
                            println!("Please enter n or d for either name or description")
                        }
                    }
                }
                
            }
            _ => {
                continue
            }
        }
    }

    let updated_contents = toml::to_string_pretty(&task_list)
        .expect("Couldn't serialize updated task list");

    fs::write(TASK_PATH, updated_contents)?;
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
    let task_list = TaskList { tasks: Vec::new() };

    // Match command to the appropriate action the user wants to perform
    match args.command.as_str() {
        // Добавить - add
        "dobavit" => {
            add_task(args, task_list);
        }
        "dobav" => {
            add_task(args, task_list);
        }
        // Покажи - show
        "pokazhi" => {
            show_tasks().expect("Could not show tasks");
        }
        // Удали - delete
        "udali" => {
            println!("udalil");
        }
        // Измени - change
        "izmeni" => {
            change_task(args.name).expect("Could not change task");
            println!("izmenil");
        }

        // Закончил - finished
        "zakonchil" => {
            finish_task(args.name).expect("Could not finish task");
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
                    izmeni - change\n
                    zakonchil - finished\n
                ");
        }
        // помаги - help
        "pomagi" => {
            println!(
                "
                    dobavit - add\n
                    pokazhi - show\n
                    uberi - clean\n
                    izmeni - change\n
                    zakonchil - finished\n
                ");
        }

        _ => {
            println!("Error parsing command");
        }
    }
}
