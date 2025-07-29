use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use serde_json;

const FILE_PATH: &str = "tasks.json";

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    task: String,
    status: String,
}

fn load_tasks() -> Vec<Task> {

    //if file did not exist in the provided path
    //then this command below will create one json file there
    if !Path::new(FILE_PATH).exists() {
        File::create(FILE_PATH)
                .and_then(|mut f| f.write_all(b"[]"))
                .expect("failed to find or create file");
    }

    //this is a read only command File:: cannot use this to write in a file
    let mut my_file = File::open(FILE_PATH).expect("i could not find your file");
    let mut content = String :: new();

    //this line below will simply convert the json file into json strin
    //eg : "s[task:"create file", ...]"
    my_file.read_to_string(&mut content).expect("i was unable to read the tasks");

    let list : Vec<Task> = serde_json::from_str(&content).unwrap_or_else(|_| vec![]);

    return list;

}

fn write_tasks_to_list(list:&Vec<Task>){

    //converting the list into a serialised json friendly string
    let serialized_list = match serde_json::to_string_pretty(&list){
        Ok(json) => json,
        Err(e)=>{
            eprintln!("Failed to serialize list : {}", e);
            return;
        }
    };

    //we used this approach to open the file for writing purpose
    let mut my_file = match OpenOptions::new()
                            .write(true)
                            .create(true)
                            .truncate(true)
                            .open(FILE_PATH){
                                Ok(f) => f, 
                                Err(e) => {
                                    eprintln!("couldn not open file while writing : {}", e);
                                    return;
                                }
                            };
    
    if let Err(e) = my_file.write_all(serialized_list.as_bytes()){
        eprintln!("failed to write tasks in the file : {}", e);
    }
}

fn add_task_to_list(new_task: &str) {
    let mut list = load_tasks();

    let new_id = list.len() + 1;

    let new_work = Task{
        id : new_id.try_into().unwrap(),
        task : new_task.to_string(),
        status : "not yet started".to_string(),
    };

    list.push(new_work);
    write_tasks_to_list(&list);

    println!("Task added successfully!!");
}


#[derive(Parser)]
#[command(name = "Task Manager")]
#[command(about = "A simple CLI task manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add {
        task: String,
    },


    Delete {
        id: u32,
    },

    Show {},
    
    Helpme{},

    #[command(name = "exit")]
    Exit {},
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { task } => {
            println!("Adding task: {}", task);
            add_task_to_list(&task);
            println!("task {} added to the list",task);
        }
        Commands::Delete { id } => {
            
            println!("Deleting task with id: {}", id);
        }
        Commands::Helpme {} => {
            println!("Commands available:");
            println!("1. Add a new task:       task_manager add <task>");
            println!("2. Update a task:        task_manager update <id> <task>");
            println!("3. Delete a task:        task_manager delete <id>");
            println!("4. Update task progress: task_manager progress <status> <id>");
            println!("5. Show this menu:       task_manager show");
            println!("6. Exit the tool:        task_manager exit");
        }
        Commands::Show {} => {
            let tasks = load_tasks();
            for task in tasks.iter() {
                println!(
                    "Task ID: {}, Task: {}, Status: {}",
                    task.id, task.task, task.status
                );
            }
        }
        Commands::Exit {} => {
            println!("Exiting the Task Manager. Goodbye!");
        }
    }
}
