use std::io::prelude::*; 
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write; 
use toml;
use clap::Parser;

const TASK_PATH: &str = "./tasks.toml";


#[derive(Serialize, Deserialize, Parser)]
struct Task {
    command: String,
    name: String, 
    description: String,
    status: Option<String>,
}

fn print_task(task: Task) {
    println!("{}", task.name);
    println!("{}", task.description); 
} 

fn main(){
    /*
     * the idea here is to have a cli command that adds tasks to tasks. 
     */
    let mut task_list_file = OpenOptions::new()
        .append(true)
        .open(TASK_PATH)
        .expect("Could not open file");

    // let the program read some sort of arguments and parse them
    let args = Task::parse();
    
    // format this into text that can be stored in the task file
    let task = toml::to_string_pretty(&args).expect("Could not convert string");  
    // write the task to the fzile
    task_list_file.write_all(task.as_bytes()).expect("Could not write to file");

}
