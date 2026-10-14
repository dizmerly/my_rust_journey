use std::io::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs::File;
use toml;


#[derive(Serialize)]
struct Task {
    name: String, 
    description: String,
    status: Option<String>,
}

fn print_task(task: Task) {
    println!("{}", task.name);
    println!("{}", task.description); 
} 

fn main() {
    /*
     * the idea here is to have a cli command that adds tasks to tasks. 
     */

    let tasksPath = "/Users/dizmerly/repos/my_rust_journey/simple_todo_list/rustodo/tasks.csv";
    let taskList = File::open(&tasksPath).expect("could not open file");
    
    let taskReader = Reader::new(&taskList);
    let taskWriter = Writer::new(&taskList); 

    let args = Task::parse();    

    
    
    print_task(args);

    

    
}
