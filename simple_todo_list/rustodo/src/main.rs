use std::fs;
use std::io; 
use std::io::prelude::*; 
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::fs::OpenOptions;
use toml;
use clap::Parser;

const TASK_PATH: &str = "./tasks.toml";


#[derive(Serialize, Deserialize, Parser)]
struct Task {
    command: String,
    name: Option<String>, 
    description: Option<String>,
    status: Option<String>,
}

// Adds to a constant filepath defined at the top of the program
fn add_task(task: Task) {
    let mut task_list_file = OpenOptions::new()
        .append(true)
        .open(TASK_PATH)
        .expect("Could not open file");

    let task = toml::to_string_pretty(&task).expect("Could not convert string");
    task_list_file.write_all(task.as_bytes()).expect("Could not write to file");    
}

fn show_tasks() -> io::Result<()> {
    let task_list_file = fs::read_to_string(TASK_PATH)?;
    println!("Task List\n {}", task_list_file);
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


fn main(){
    // let the program read some sort of arguments and parse them
    let args = Task::parse();  


    // Match command to the appropriate action the user wants to perform
    match args.command.as_str() {
        // Добавить - add 
        "dobavit" => {
            add_task(args);
        }
        // Покажи - show
        "pokazhi" => {
            show_tasks(); 
        }
        // Убери - clean
        "uberi" => {
            clean_tasks().expect("Could not clean file");
        }

        "help" => {
            println!("
                    dobavit - add\n
                    pokazhi - show\n
                    uberi - clean\n
                ");
        }
        "pomagi" => {
            println!("
                    dobavit - add\n
                    pokazhi - show\n
                    uberi - clean\n
                ");
        }
        
        _ => {
            println!("Error parsing command");
        }
    }
}
