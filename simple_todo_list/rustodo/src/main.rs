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

fn add_task(task: Task) {
    let mut task_list_file = OpenOptions::new()
        .append(true)
        .open(TASK_PATH)
        .expect("Could not open file");

    let task = toml::to_string_pretty(&task).expect("Could not convert string");
    task_list_file.write_all(task.as_bytes()).expect("Could not write to file");
    
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
    /*
     * the idea here is to have a cli command that adds tasks to tasks. 
     */
    // let the program read some sort of arguments and parse them
    let args = Task::parse();  


    // Match command to the appropriate action the user wants to perform
    match args.command.as_str() {
        "add" => {
            add_task(args);
        }
        "clean" => {
            clean_tasks().expect("Could not clean file");
        }
        
        _ => {
            println!("Error parsing command");
        }
    }
}
