use std::io::prelude::*;
use clap::Parser;
use std::fs::File;
use std::io::BufReader; 

#[derive(Parser)]
struct Task {
    name: String, 
    description: String,
}

fn print_task(task: Task) {
    println!("{}", task.name);
    println!("{}", task.description); 
} 



fn main() {
    println!("Hello, world, I am a todo list!");

    

    
}
