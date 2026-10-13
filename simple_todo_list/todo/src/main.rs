use std::io::prelude::*;
use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;

#[derive(Parser)]
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

    let tasksPath = "/Users/dizmerly/repos/my_rust_journey/simple_todo_list/todo";
    let taskList = File::open(&tasksPath).expect("could not open file");
    
    let taskReader = BufReader::new(&taskList);
    let taskWriter = BufWriter::new(&taskList); 


    let args = Task::parse(); 
    
    print_task(args);

    

    
}
